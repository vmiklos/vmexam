#include <fstream>
#include <iostream>
#include <set>
#include <sstream>

#include <clang/AST/ASTConsumer.h>
#include <clang/AST/ASTContext.h>
#include <clang/AST/RecursiveASTVisitor.h>
#include <clang/Rewrite/Core/Rewriter.h>
#include <clang/Tooling/CommonOptionsParser.h>
#include <clang/Tooling/Tooling.h>

class RenameRewriter : public clang::Rewriter
{
    /// Old names -> new names map.
    std::map<std::string, std::string> maNameMap;
    bool mbDump;

public:
    RenameRewriter(const std::map<std::string, std::string>& rNameMap, bool bDump)
        : maNameMap(rNameMap),
        mbDump(bDump)
    {
    }

    const std::map<std::string, std::string>& getNameMap()
    {
        return maNameMap;
    }

    bool getDump()
    {
        return mbDump;
    }
};

class RenameVisitor : public clang::RecursiveASTVisitor<RenameVisitor>
{
    RenameRewriter& mrRewriter;
    // A set of handled locations, so in case a location would be handled
    // multiple times due to macro usage, we only do the rewrite once.
    // Otherwise an A -> BA replacement would be done twice.
    std::set<clang::SourceLocation> maHandledLocations;

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
        const std::map<std::string, std::string>::const_iterator it = mrRewriter.getNameMap().find(aName);
        if (it != mrRewriter.getNameMap().end())
            mrRewriter.ReplaceText(pDecl->getLocation(), pDecl->getNameAsString().length(), it->second);
        return true;
    }

    /*
     * class C
     * {
     * public:
     *     static const int aS[]; <- Handles e.g. this declaration;
     * };
     */
    bool VisitVarDecl(clang::VarDecl* pDecl)
    {
        std::string aName = pDecl->getQualifiedNameAsString();
        const std::map<std::string, std::string>::const_iterator it = mrRewriter.getNameMap().find(aName);
        if (it != mrRewriter.getNameMap().end())
            mrRewriter.ReplaceText(pDecl->getLocation(), pDecl->getNameAsString().length(), it->second);
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
        for (clang::CXXConstructorDecl::init_const_iterator itInit = pDecl->init_begin(); itInit != pDecl->init_end(); ++itInit)
        {
            const clang::CXXCtorInitializer* pInitializer = *itInit;
            if (const clang::FieldDecl* pFieldDecl = pInitializer->getAnyMember())
            {
                std::string aName = pFieldDecl->getQualifiedNameAsString();
                const std::map<std::string, std::string>::const_iterator it = mrRewriter.getNameMap().find(aName);
                if (it != mrRewriter.getNameMap().end())
                    mrRewriter.ReplaceText(pInitializer->getSourceLocation(), pFieldDecl->getNameAsString().length(), it->second);
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
            const std::map<std::string, std::string>::const_iterator it = mrRewriter.getNameMap().find(aName);
            if (it != mrRewriter.getNameMap().end())
            {
                clang::SourceLocation aLocation = pExpr->getMemberLoc();
                if (pExpr->getMemberLoc().isMacroID())
                    /*
                     * int foo(int x);
                     * #define FOO(a) foo(a)
                     * FOO(aC.nX); <- Handles this.
                     */
                    aLocation = mrRewriter.getSourceMgr().getSpellingLoc(aLocation);
                if (maHandledLocations.find(aLocation) == maHandledLocations.end())
                {
                    mrRewriter.ReplaceText(aLocation, pDecl->getNameAsString().length(), it->second);
                    maHandledLocations.insert(aLocation);
                }
            }
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

/// Parses rCsv and puts the first two column of it into rNameMap.
static void parseCsv(const std::string& rCsv, std::map<std::string, std::string>& rNameMap)
{
    std::ifstream aStream(rCsv);
    if (!aStream.is_open())
    {
        std::cerr << "parseCsv: failed to open " << rCsv << std::endl;
        return;
    }

    std::string aLine;
    while (std::getline(aStream, aLine))
    {
        std::stringstream ss(aLine);
        std::string aOldName;
        if (!std::getline(ss, aOldName, ','))
        {
            std::cerr << "parseCsv: first std::getline() failed for line '" << aLine << "'" << std::endl;
            return;
        }
        std::string aNewName;
        if (!std::getline(ss, aNewName, ','))
        {
            std::cerr << "parseCsv: second std::getline() failed for line '" << aLine << "'" << std::endl;
            return;
        }
        rNameMap[aOldName] = aNewName;
    }

    aStream.close();
}

int main(int argc, const char** argv)
{
    llvm::cl::OptionCategory aCategory("rename options");
    llvm::cl::opt<std::string> aOldName("old-name",
                                        llvm::cl::desc("Old, qualified name (Class::member)."),
                                        llvm::cl::cat(aCategory));
    llvm::cl::opt<std::string> aNewName("new-name",
                                        llvm::cl::desc("New, non-qualified name (without Class::)."),
                                        llvm::cl::cat(aCategory));
    llvm::cl::opt<std::string> aCsv("csv",
                                    llvm::cl::desc("Path to a CSV file, containing multiple renames -- seprator must be a comma (,)."),
                                    llvm::cl::cat(aCategory));
    llvm::cl::opt<bool> bDump("dump",
                              llvm::cl::desc("Dump output on the console instead of writing to .new files."),
                              llvm::cl::cat(aCategory));
    clang::tooling::CommonOptionsParser aParser(argc, argv, aCategory);

    std::map<std::string, std::string> aNameMap;
    if (!aOldName.empty() && !aNewName.empty())
        aNameMap[aOldName] = aNewName;
    else if (!aCsv.empty())
        parseCsv(aCsv, aNameMap);
    else
    {
        std::cerr << "either -old-name + -new-name or -csv is required." << std::endl;
        return 1;
    }

    clang::tooling::ClangTool aTool(aParser.getCompilations(), aParser.getSourcePathList());

    RenameRewriter aRewriter(aNameMap, bDump);
    RenameFrontendAction aAction(aRewriter);
    std::unique_ptr<clang::tooling::FrontendActionFactory> pFactory = clang::tooling::newFrontendActionFactory(&aAction);
    return aTool.run(pFactory.get());
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
