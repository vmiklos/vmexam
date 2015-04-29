#include <iostream>
#include <sstream>

#include <clang/AST/ASTConsumer.h>
#include <clang/AST/ASTContext.h>
#include <clang/AST/RecursiveASTVisitor.h>
#include <clang/Rewrite/Core/Rewriter.h>
#include <clang/Tooling/CommonOptionsParser.h>
#include <clang/Tooling/Tooling.h>

class RenameRewriter : public clang::Rewriter
{
    std::string maOldName;
    std::string maNewName;
    bool mbDump;

public:
    RenameRewriter(const std::string& rOldName, const std::string& rNewName, bool bDump)
        : maOldName(rOldName),
        maNewName(rNewName),
        mbDump(bDump)
    {
    }

    const std::string& getOldName()
    {
        return maOldName;
    }

    const std::string& getNewName()
    {
        return maNewName;
    }

    bool getDump()
    {
        return mbDump;
    }
};

class RenameVisitor : public clang::RecursiveASTVisitor<RenameVisitor>
{
    RenameRewriter& mrRewriter;

public:
    explicit RenameVisitor(RenameRewriter& rRewriter)
        : mrRewriter(rRewriter)
    {
    }

    /*
     * class C
     * {
     * public:
     *     int nX; <- Handles this declaration.
     * };
     */
    bool VisitFieldDecl(clang::FieldDecl* pDecl)
    {
        // Qualified name includes "C::" as a prefix, normal name does not.
        std::string aName = pDecl->getQualifiedNameAsString();
        if (aName == mrRewriter.getOldName())
            mrRewriter.ReplaceText(pDecl->getLocation(), pDecl->getNameAsString().length(), mrRewriter.getNewName());
        return true;
    }

    /*
     * C::C()
     *     : nX(0) <- Handles this initializer.
     * {
     * }
     */
    bool VisitCXXConstructorDecl(clang::CXXConstructorDecl* pDecl)
    {
        for (clang::CXXConstructorDecl::init_const_iterator it = pDecl->init_begin(); it != pDecl->init_end(); ++it)
        {
            const clang::CXXCtorInitializer* pInitializer = *it;
            if (const clang::FieldDecl* pFieldDecl = pInitializer->getAnyMember())
            {
                std::string aName = pFieldDecl->getQualifiedNameAsString();
                if (aName == mrRewriter.getOldName())
                    mrRewriter.ReplaceText(pInitializer->getSourceLocation(), pFieldDecl->getNameAsString().length(), mrRewriter.getNewName());
            }
        }
        return true;
    }

    /*
     * C aC;
     * aC.nX = 1; <- Handles e.g. this...
     * int y = aC.nX; <- ...and this.
     */
    bool VisitMemberExpr(clang::MemberExpr* pExpr)
    {
        if (clang::ValueDecl* pDecl = pExpr->getMemberDecl())
        {
            std::string aName = pDecl->getQualifiedNameAsString();
            if (aName == mrRewriter.getOldName())
                mrRewriter.ReplaceText(pExpr->getMemberLoc(), pDecl->getNameAsString().length(), mrRewriter.getNewName());
        }
        return true;
    }
};

class RenameASTConsumer : public clang::ASTConsumer
{
    RenameRewriter& mrRewriter;

    std::string getNewName(const clang::FileEntry& rEntry)
    {
        std::stringstream ss;
        ss << rEntry.getName();
        ss << ".new";
        return ss.str();
    }

public:
    RenameASTConsumer(RenameRewriter& rRewriter)
        : mrRewriter(rRewriter)
    {
    }

    virtual void HandleTranslationUnit(clang::ASTContext& rContext)
    {
        if (rContext.getDiagnostics().hasErrorOccurred())
            return;

        RenameVisitor aVisitor(mrRewriter);
        mrRewriter.setSourceMgr(rContext.getSourceManager(), rContext.getLangOpts());
        aVisitor.TraverseDecl(rContext.getTranslationUnitDecl());

        for (clang::Rewriter::buffer_iterator it = mrRewriter.buffer_begin(); it != mrRewriter.buffer_end(); ++it)
        {
            if (mrRewriter.getDump())
                it->second.write(llvm::errs());
            else
            {
                const clang::FileEntry* pEntry = rContext.getSourceManager().getFileEntryForID(it->first);
                if (!pEntry)
                    continue;
                std::string aNewName = getNewName(*pEntry);
                std::string aError;
                std::unique_ptr<llvm::raw_fd_ostream> pStream(new llvm::raw_fd_ostream(aNewName.c_str(), aError, llvm::sys::fs::F_None));
                if (aError.empty())
                    it->second.write(*pStream);
            }
        }
    }
};

class RenameFrontendAction
{
    RenameRewriter& mrRewriter;

public:
    RenameFrontendAction(RenameRewriter& rRewriter)
        : mrRewriter(rRewriter)
    {
    }

    clang::ASTConsumer* newASTConsumer()
    {
        return new RenameASTConsumer(mrRewriter);
    }
};

int main(int argc, const char** argv)
{
    llvm::cl::OptionCategory aCategory("rename options");
    llvm::cl::opt<std::string> aOldName("old-name",
                                        llvm::cl::desc("Old, qualified name (Class::member)."),
                                        llvm::cl::cat(aCategory));
    llvm::cl::opt<std::string> aNewName("new-name",
                                        llvm::cl::desc("New, non-qualified name (without Class::)."),
                                        llvm::cl::cat(aCategory));
    llvm::cl::opt<bool> bDump("dump",
                              llvm::cl::desc("Dump output on the console instead of writing to .new files."),
                              llvm::cl::cat(aCategory));
    clang::tooling::CommonOptionsParser aParser(argc, argv, aCategory);
    if (aOldName.empty())
    {
        std::cerr << "no old name provided." << std::endl;
        return 1;
    }
    else if (aNewName.empty())
    {
        std::cerr << "no new name provided." << std::endl;
        return 1;
    }

    clang::tooling::ClangTool aTool(aParser.getCompilations(), aParser.getSourcePathList());

    RenameRewriter aRewriter(aOldName, aNewName, bDump);
    RenameFrontendAction aAction(aRewriter);
    std::unique_ptr<clang::tooling::FrontendActionFactory> pFactory = clang::tooling::newFrontendActionFactory(&aAction);
    return aTool.run(pFactory.get());
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
