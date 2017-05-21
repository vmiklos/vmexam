#include <fstream>
#include <iostream>
#include <set>
#include <sstream>
#include <unistd.h>

#include <clang/AST/ASTConsumer.h>
#include <clang/AST/ASTContext.h>
#include <clang/AST/RecursiveASTVisitor.h>
#include <clang/Rewrite/Core/Rewriter.h>
#include <clang/Tooling/CommonOptionsParser.h>
#include <clang/Tooling/Tooling.h>

class Context
{
    std::set<std::string> m_aPaths;
    clang::ASTContext* m_pContext;

  public:
    Context(const std::set<std::string>& rPaths)
        : m_aPaths(rPaths), m_pContext(nullptr)
    {
    }

    void setASTContext(clang::ASTContext& rContext) { m_pContext = &rContext; }

    bool ignoreLocation(const clang::SourceLocation& rLocation)
    {
        bool bRet = false;

        clang::SourceLocation aLocation =
            m_pContext->getSourceManager().getExpansionLoc(rLocation);
        if (m_pContext->getSourceManager().isInSystemHeader(aLocation))
            bRet = true;
        else if (m_aPaths.empty())
        {
            bRet = false;
        }
        else
        {
            const char* pName = m_pContext->getSourceManager()
                                    .getPresumedLoc(aLocation)
                                    .getFilename();
            bRet = m_aPaths.find(pName) == m_aPaths.end();
        }

        return bRet;
    }

    clang::DiagnosticBuilder report(llvm::StringRef aString,
                                    clang::SourceLocation aLocation) const
    {
        clang::DiagnosticsEngine& rEngine = m_pContext->getDiagnostics();
        return rEngine.Report(
            aLocation, rEngine.getDiagnosticIDs()->getCustomDiagID(
                           clang::DiagnosticIDs::Level::Warning, aString));
    }
};

/// Finds C++ member functions which could be const but are not.
class Visitor : public clang::RecursiveASTVisitor<Visitor>
{
    Context& m_rContext;
    bool m_bConstCandidate = false;

  public:
    Visitor(Context& rContext, clang::ASTContext& rASTContext)
        : m_rContext(rContext)
    {
        m_rContext.setASTContext(rASTContext);
    }

    /*
     * m_nX = nX;
     *      ^ Handles this when it's a primitive type.
     */
    bool VisitBinaryOperator(const clang::BinaryOperator* pOperator)
    {
        if (pOperator->getOpcode() != clang::BO_Assign)
            return true;

        auto pMemberExpr =
            llvm::dyn_cast<clang::MemberExpr>(pOperator->getLHS());
        if (pMemberExpr)
            m_bConstCandidate = false;

        return true;
    }

    bool TraverseCXXMethodDecl(const clang::CXXMethodDecl* pDecl)
    {
        if (m_rContext.ignoreLocation(pDecl->getLocation()))
            return true;

        if (pDecl->isConst())
            return true;

        if (pDecl->isStatic())
            return true;

        if (pDecl->isVirtual())
            return true;

        m_bConstCandidate = true;
        TraverseStmt(pDecl->getBody());

        if (m_bConstCandidate)
        {
            m_rContext.report("this member function can be declared const",
                              pDecl->getCanonicalDecl()->getLocation())
                << pDecl->getCanonicalDecl()->getSourceRange();
            return true;
        }

        return true;
    }
};

class ASTConsumer : public clang::ASTConsumer
{
    Context& m_rContext;

  public:
    ASTConsumer(Context& rContext) : m_rContext(rContext) {}

    virtual void HandleTranslationUnit(clang::ASTContext& rContext)
    {
        if (rContext.getDiagnostics().hasErrorOccurred())
            return;

        Visitor aVisitor(m_rContext, rContext);
        aVisitor.TraverseDecl(rContext.getTranslationUnitDecl());
    }
};

class FrontendAction
{
    Context& m_rContext;

  public:
    FrontendAction(Context& rContext) : m_rContext(rContext) {}

    std::unique_ptr<clang::ASTConsumer> newASTConsumer()
    {
        return llvm::make_unique<ASTConsumer>(m_rContext);
    }
};

/// Parses rPathsFile and puts the first two column of it into rNameMap.
static bool parsePathsFile(const std::string& rPathsFile,
                           std::set<std::string>& rPaths)
{
    std::ifstream aStream(rPathsFile);
    if (!aStream.is_open())
    {
        std::cerr << "parsePathsFile: failed to open " << rPathsFile
                  << std::endl;
        return false;
    }

    std::string aLine;
    char pCwd[PATH_MAX];
    getcwd(pCwd, PATH_MAX);
    const std::string aCwd(pCwd);
    while (std::getline(aStream, aLine))
    {
        if (!aLine.empty() && aLine[0] != '/')
            aLine = aCwd + "/" + aLine;
        rPaths.insert(aLine);
    }

    aStream.close();
    return true;
}

int main(int argc, const char** argv)
{
    llvm::cl::OptionCategory aCategory("find-nonconst-methods options");
    llvm::cl::opt<std::string> aPathsFile(
        "paths-file",
        llvm::cl::desc(
            "If not empty, ignore all source code "
            "paths not mentioned in the paths file (one full path / line)."),
        llvm::cl::cat(aCategory));
    clang::tooling::CommonOptionsParser aParser(argc, argv, aCategory);

    clang::tooling::ClangTool aTool(aParser.getCompilations(),
                                    aParser.getSourcePathList());

    std::set<std::string> aPaths;
    if (!aPathsFile.empty())
    {
        if (!parsePathsFile(aPathsFile, aPaths))
            return 1;
    }

    Context aContext(aPaths);
    FrontendAction aAction(aContext);
    std::unique_ptr<clang::tooling::FrontendActionFactory> pFactory =
        clang::tooling::newFrontendActionFactory(&aAction);
    return aTool.run(pFactory.get());
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
