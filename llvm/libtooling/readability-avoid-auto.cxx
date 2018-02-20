#include <iostream>

#include <clang/ASTMatchers/ASTMatchFinder.h>
#include <clang/ASTMatchers/ASTMatchers.h>
#include <clang/Frontend/FrontendActions.h>
#include <clang/Tooling/CommonOptionsParser.h>
#include <clang/Tooling/Refactoring.h>
#include <llvm/Support/Signals.h>

namespace
{
class AvoidAutoCallback : public clang::ast_matchers::MatchFinder::MatchCallback
{
  public:
    AvoidAutoCallback(clang::tooling::Replacements* /*pReplace*/) {}

    void
    run(const clang::ast_matchers::MatchFinder::MatchResult& rResult) override
    {
        if (const auto pDecl = rResult.Nodes.getNodeAs<clang::VarDecl>("decl"))
        {
            clang::QualType aType = pDecl->getType();
            report(rResult.Context,
                   "avoid auto, consider spelling out %0 instead",
                   pDecl->getLocation())
                << pDecl->getSourceRange() << aType;
        }
    }

  private:
    clang::DiagnosticBuilder report(clang::ASTContext* pContext,
                                    llvm::StringRef aString,
                                    clang::SourceLocation aLocation) const
    {
        clang::DiagnosticsEngine& rEngine = pContext->getDiagnostics();
        return rEngine.Report(aLocation,
                              rEngine.getDiagnosticIDs()->getCustomDiagID(
                                  clang::DiagnosticIDs::Level::Error, aString));
    }
};

clang::ast_matchers::StatementMatcher makeMatcher()
{
    using namespace clang::ast_matchers;
    return declStmt(has(varDecl(hasType(autoType())).bind("decl")));
}

llvm::cl::extrahelp
    aCommonHelp(clang::tooling::CommonOptionsParser::HelpMessage);
llvm::cl::OptionCategory aAvoidAutoCategory("avoid-auto options");
}

int main(int argc, const char** argv)
{
    llvm::sys::PrintStackTraceOnErrorSignal();
    clang::tooling::CommonOptionsParser aOptionsParser(argc, argv,
                                                       aAvoidAutoCategory);
    clang::tooling::RefactoringTool aTool(aOptionsParser.getCompilations(),
                                          aOptionsParser.getSourcePathList());
    clang::ast_matchers::MatchFinder aFinder;
    AvoidAutoCallback aCallback(&aTool.getReplacements());
    aFinder.addMatcher(makeMatcher(), &aCallback);
    return aTool.run(clang::tooling::newFrontendActionFactory(&aFinder).get());
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
