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
    bool m_bInVisitVarDecl;
    std::string m_aDeclRefName;

  public:
    Visitor(Context& rContext, clang::ASTContext& rASTContext)
        : m_rContext(rContext), m_bInVisitVarDecl(false)
    {
        m_rContext.setASTContext(rASTContext);
    }

    bool VisitDeclRefExpr(clang::DeclRefExpr* pExpr)
    {
        m_aDeclRefName = pExpr->getFoundDecl()->getQualifiedNameAsString();
        return true;
    }

    bool VisitVarDecl(clang::VarDecl* pDecl)
    {
        if (m_rContext.ignoreLocation(pDecl->getLocation()) ||
            m_bInVisitVarDecl)
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
            m_bInVisitVarDecl = true;
            m_aDeclRefName.clear();
            bool bRet = RecursiveASTVisitor::TraverseVarDecl(pDecl);
            m_bInVisitVarDecl = false;
            if (!bRet)
                return false;

            if (m_aDeclRefName.find("make_shared") != std::string::npos)
                // Have the type info spelled out already.
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
