#include <iostream>
#include <sstream>

#include <clang/ASTMatchers/ASTMatchFinder.h>
#include <clang/ASTMatchers/ASTMatchers.h>
#include <clang/Frontend/FrontendActions.h>
#include <clang/Tooling/CommonOptionsParser.h>
#include <clang/Tooling/Refactoring.h>
#include <clang/Tooling/ReplacementsYaml.h>
#include <llvm/Support/Signals.h>
#include <llvm/Support/YAMLTraits.h>

namespace
{
bool startsWith(const std::string& s, const std::string& t)
{
    return s.length() >= t.length() &&
           memcmp(s.c_str(), t.c_str(), t.length()) == 0;
}

class Callback : public clang::ast_matchers::MatchFinder::MatchCallback
{
  public:
    void
    run(const clang::ast_matchers::MatchFinder::MatchResult& rResult) override
    {
        if (const auto pExpr = rResult.Nodes.getNodeAs<clang::CallExpr>("expr"))
        {
            const clang::FunctionDecl* pFunction = pExpr->getDirectCallee();
            if (!pFunction)
                return;

            if (pFunction->getNameAsString() != "xmlSecError")
            {
                m_aLastCalledFunction = pFunction->getNameAsString();
                return;
            }

            if (pExpr->getNumArgs() < 6)
                return;

            auto pLiteral = llvm::dyn_cast<clang::StringLiteral>(
                pExpr->getArg(4)->IgnoreParenImpCasts());
            if (!pLiteral)
                return;

            std::string aFunctionName = pLiteral->getString();
            if (startsWith(aFunctionName, m_aLastCalledFunction))
                return;

            auto pReason =
                llvm::dyn_cast<clang::IntegerLiteral>(pExpr->getArg(5));
            if (!pReason)
                return;

            // XMLSEC_ERRORS_R_XMLSEC_FAILED is 1
            if (pReason->getValue().getLimitedValue() != 1)
                return;

            // We now found a call to xmlSecError() with a non-matching error
            // function name.
            std::stringstream ss;
            ss << "errorFunction argument of xmlSecError() should be probably '"
               << m_aLastCalledFunction << "'";
            report(rResult.Context, ss.str(), pExpr->getLocStart());
        }
    }

  private:
    clang::DiagnosticBuilder report(clang::ASTContext* pContext,
                                    llvm::StringRef aString,
                                    clang::SourceLocation aLocation) const
    {
        clang::DiagnosticsEngine& rEngine = pContext->getDiagnostics();
        clang::DiagnosticIDs::Level eLevel =
            clang::DiagnosticIDs::Level::Warning;
        if (rEngine.getWarningsAsErrors())
            eLevel = clang::DiagnosticIDs::Level::Error;
        return rEngine.Report(
            aLocation,
            rEngine.getDiagnosticIDs()->getCustomDiagID(eLevel, aString));
    }

    std::string m_aLastCalledFunction;
};

clang::ast_matchers::StatementMatcher makeMatcher()
{
    using namespace clang::ast_matchers;
    return callExpr(unless(hasAncestor(callExpr()))).bind("expr");
}

llvm::cl::extrahelp
    aCommonHelp(clang::tooling::CommonOptionsParser::HelpMessage);
llvm::cl::OptionCategory aCategory("suspicious-xmlsec options");
}

int main(int argc, const char** argv)
{
    llvm::sys::PrintStackTraceOnErrorSignal();
    clang::tooling::CommonOptionsParser aOptionsParser(argc, argv, aCategory);
    clang::tooling::RefactoringTool aTool(aOptionsParser.getCompilations(),
                                          aOptionsParser.getSourcePathList());
    clang::ast_matchers::MatchFinder aFinder;
    Callback aCallback;
    aFinder.addMatcher(makeMatcher(), &aCallback);
    int nRet =
        aTool.run(clang::tooling::newFrontendActionFactory(&aFinder).get());

    return nRet;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
