#include <fstream>
#include <iostream>
#include <set>
#include <sstream>
#include <unistd.h>

#include <clang/AST/ASTConsumer.h>
#include <clang/AST/ASTContext.h>
#include <clang/AST/DeclCXX.h>
#include <clang/AST/RecursiveASTVisitor.h>
#include <clang/AST/Type.h>
#include <clang/Rewrite/Core/Rewriter.h>
#include <clang/Tooling/CommonOptionsParser.h>
#include <clang/Tooling/Tooling.h>

class Context
{
    clang::ASTContext* m_pContext;

  public:
    Context() : m_pContext(nullptr) {}

    void setASTContext(clang::ASTContext& rContext) { m_pContext = &rContext; }

    clang::ASTContext* getASTContext() const { return m_pContext; }

    bool ignoreLocation(const clang::SourceLocation& rLocation)
    {
        clang::SourceLocation aLocation =
            m_pContext->getSourceManager().getExpansionLoc(rLocation);
        return m_pContext->getSourceManager().isInSystemHeader(aLocation);
    }

    clang::DiagnosticBuilder report(llvm::StringRef aString,
                                    clang::SourceLocation aLocation) const
    {
        clang::DiagnosticsEngine& rEngine = m_pContext->getDiagnostics();
        return rEngine.Report(aLocation,
                              rEngine.getDiagnosticIDs()->getCustomDiagID(
                                  clang::DiagnosticIDs::Level::Error, aString));
    }
};

/**
 * Finds auto usage which is not readable (type info not in the same line, nor
 * an iterator).
 */
class Visitor : public clang::RecursiveASTVisitor<Visitor>
{
    Context& m_rContext;
    std::string m_aDeclRefName;

  public:
    Visitor(Context& rContext, clang::ASTContext& rASTContext)
        : m_rContext(rContext)
    {
        m_rContext.setASTContext(rASTContext);
    }

    bool hasTemplateArguments(clang::Expr* pExpr)
    {
        if (!pExpr)
            return false;

        auto pCallExpr = clang::dyn_cast<clang::CallExpr>(pExpr);
        if (!pCallExpr)
        {
            auto pExprWithCleanups =
                clang::dyn_cast<clang::ExprWithCleanups>(pExpr);
            if (!pExprWithCleanups)
                return false;

            auto pCXXConstructExpr = clang::dyn_cast<clang::CXXConstructExpr>(
                pExprWithCleanups->getSubExpr());
            if (!pCXXConstructExpr || pCXXConstructExpr->getNumArgs() < 1)
                return false;

            auto pMaterializeTemporaryExpr =
                clang::dyn_cast<clang::MaterializeTemporaryExpr>(
                    pCXXConstructExpr->getArg(0));
            if (!pMaterializeTemporaryExpr)
                return false;

            auto pCXXBindTemporaryExpr =
                clang::dyn_cast<clang::CXXBindTemporaryExpr>(
                    pMaterializeTemporaryExpr->GetTemporaryExpr());
            if (!pCXXBindTemporaryExpr)
                return false;

            pCallExpr = clang::dyn_cast<clang::CallExpr>(
                pCXXBindTemporaryExpr->getSubExpr());
        }

        if (!pCallExpr)
            return false;

        auto pFunctionDecl =
            clang::dyn_cast<clang::FunctionDecl>(pCallExpr->getCalleeDecl());
        if (!pFunctionDecl)
            return false;

        return pFunctionDecl->getTemplateSpecializationArgs() != nullptr;
    }

    bool VisitVarDecl(clang::VarDecl* pDecl)
    {
        if (m_rContext.ignoreLocation(pDecl->getLocation()))
            return true;

        clang::QualType aType = pDecl->getType();
        std::string aTypeName = aType.getAsString();
        if (aTypeName.find("iterator") != std::string::npos)
            // Ignore iterators.
            return true;

        if (aTypeName.find("std::chrono::duration") != std::string::npos)
            // Unclear what to do here.
            return true;

        if (pDecl->hasInit())
        {
            if (hasTemplateArguments(pDecl->getInit()))
                /*
                 * Allow e.g.
                 * auto pFoo = std::make_shared<Foo>();
                 */
                return true;
        }

        if (clang::isa<clang::AutoType>(aType.getTypePtr()))
            m_rContext.report("harmful auto, consider spelling out %0 instead",
                              pDecl->getLocation())
                << pDecl->getSourceRange() << aType;
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
    llvm::cl::OptionCategory aCategory("find-harmful-auto options");
    clang::tooling::CommonOptionsParser aParser(argc, argv, aCategory);

    clang::tooling::ClangTool aTool(aParser.getCompilations(),
                                    aParser.getSourcePathList());

    Context aContext;
    FrontendAction aAction(aContext);
    std::unique_ptr<clang::tooling::FrontendActionFactory> pFactory =
        clang::tooling::newFrontendActionFactory(&aAction);
    return aTool.run(pFactory.get());
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
