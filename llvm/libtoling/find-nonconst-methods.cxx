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

class Context
{
    std::string m_aPathPrefix;
    clang::ASTContext* m_pContext;

  public:
    Context(const std::string& rPathPrefix)
        : m_aPathPrefix(rPathPrefix), m_pContext(nullptr)
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
        else if (m_aPathPrefix.empty())
        {
            bRet = false;
        }
        else
        {
            const char* pName = m_pContext->getSourceManager()
                                    .getPresumedLoc(aLocation)
                                    .getFilename();
            bRet = std::string(pName).find(m_aPathPrefix) != 0;
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

int main(int argc, const char** argv)
{
    llvm::cl::OptionCategory aCategory("find-nonconst-methods options");
    llvm::cl::opt<std::string> aPathPrefix(
        "path-prefix", llvm::cl::desc("If not empty, ignore all source code "
                                      "paths not matching this prefix."),
        llvm::cl::cat(aCategory));
    clang::tooling::CommonOptionsParser aParser(argc, argv, aCategory);

    clang::tooling::ClangTool aTool(aParser.getCompilations(),
                                    aParser.getSourcePathList());

    Context aContext(aPathPrefix);
    FrontendAction aAction(aContext);
    std::unique_ptr<clang::tooling::FrontendActionFactory> pFactory =
        clang::tooling::newFrontendActionFactory(&aAction);
    return aTool.run(pFactory.get());
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
