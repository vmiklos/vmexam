#include <iostream>

#include <clang/AST/ASTConsumer.h>
#include <clang/AST/ASTContext.h>
#include <clang/AST/RecursiveASTVisitor.h>
#include <clang/Rewrite/Core/Rewriter.h>
#include <clang/Tooling/CommonOptionsParser.h>
#include <clang/Tooling/Tooling.h>

enum class RenameType
{
    Function,
    Field
};

class RenameRewriter : public clang::Rewriter
{
    std::string maOldName;
    std::string maNewName;
    RenameType meType;

public:
    RenameRewriter(const std::string& rOldName, const std::string& rNewName, const std::string& rType)
        : maOldName(rOldName),
        maNewName(rNewName)
    {
        if (rType == "function")
            meType = RenameType::Function;
        else if (rType == "field")
            meType = RenameType::Field;
    }

    const std::string& getOldName()
    {
        return maOldName;
    }

    const std::string& getNewName()
    {
        return maNewName;
    }

    RenameType getType()
    {
        return meType;
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

    bool VisitFunctionDecl(clang::FunctionDecl* pDecl)
    {
        if (mrRewriter.getType() == RenameType::Function)
        {
            std::string aName = pDecl->getNameInfo().getName().getAsString();
            if (aName == mrRewriter.getOldName())
                mrRewriter.ReplaceText(pDecl->getLocation(), aName.length(), mrRewriter.getNewName());
        }
        return true;
    }

    bool VisitCallExpr(clang::CallExpr* pExpr)
    {
        if (mrRewriter.getType() == RenameType::Function)
        {
            const clang::FunctionDecl* pDecl = pExpr->getDirectCallee();
            if (!pDecl)
                return true;
            std::string aName = pDecl->getNameInfo().getName().getAsString();
            if (aName == mrRewriter.getOldName())
                mrRewriter.ReplaceText(pExpr->getLocStart(), aName.length(), mrRewriter.getNewName());
        }
        return true;
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
        if (mrRewriter.getType() == RenameType::Field)
        {
            // Qualified name includes "C::" as a prefix, normal name does not.
            std::string aName = pDecl->getQualifiedNameAsString();
            if (aName == mrRewriter.getOldName())
                mrRewriter.ReplaceText(pDecl->getLocation(), pDecl->getNameAsString().length(), mrRewriter.getNewName());
        }
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
        if (mrRewriter.getType() == RenameType::Field)
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
        }
        return true;
    }
};

class RenameASTConsumer : public clang::ASTConsumer
{
    RenameRewriter& mrRewriter;

public:
    RenameASTConsumer(RenameRewriter& rRewriter)
        : mrRewriter(rRewriter)
    {
    }

    virtual void HandleTranslationUnit(clang::ASTContext& rContext)
    {
        RenameVisitor aVisitor(mrRewriter);
        mrRewriter.setSourceMgr(rContext.getSourceManager(), rContext.getLangOpts());
        aVisitor.TraverseDecl(rContext.getTranslationUnitDecl());
        mrRewriter.getEditBuffer(rContext.getSourceManager().getMainFileID()).write(llvm::errs());
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
                                        llvm::cl::desc("Old name"),
                                        llvm::cl::cat(aCategory));
    llvm::cl::opt<std::string> aNewName("new-name",
                                        llvm::cl::desc("New name"),
                                        llvm::cl::cat(aCategory));
    llvm::cl::opt<std::string> aType("type",
                                     llvm::cl::desc("Type (default: function, other possible value: field"),
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

    RenameRewriter aRewriter(aOldName, aNewName, aType);
    RenameFrontendAction aAction(aRewriter);
    std::unique_ptr<clang::tooling::FrontendActionFactory> pFactory = clang::tooling::newFrontendActionFactory(&aAction);
    return aTool.run(pFactory.get());
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
