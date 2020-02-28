#include <iostream>

#include <clang/ASTMatchers/ASTMatchFinder.h>
#include <clang/ASTMatchers/ASTMatchers.h>
#include <clang/Frontend/FrontendActions.h>
#include <clang/Tooling/CommonOptionsParser.h>
#include <clang/Tooling/Refactoring.h>
#include <llvm/Support/Signals.h>

namespace
{
class Callback : public clang::ast_matchers::MatchFinder::MatchCallback
{
  public:
    void
    run(const clang::ast_matchers::MatchFinder::MatchResult& result) override
    {
        if (const auto expr =
                result.Nodes.getNodeAs<clang::CXXOperatorCallExpr>("expr"))
        {
            clang::SourceRange range(expr->getExprLoc());
            report(result.Context, "ast-matcher", range.getBegin()) << range;
        }
    }

  private:
    clang::DiagnosticBuilder report(clang::ASTContext* context,
                                    llvm::StringRef string,
                                    clang::SourceLocation location) const
    {
        clang::DiagnosticsEngine& engine = context->getDiagnostics();
        clang::DiagnosticIDs::Level level =
            clang::DiagnosticIDs::Level::Warning;
        if (engine.getWarningsAsErrors())
            level = clang::DiagnosticIDs::Level::Error;
        return engine.Report(
            location,
            engine.getDiagnosticIDs()->getCustomDiagID(level, string));
    }
};

clang::ast_matchers::StatementMatcher makeMatcher()
{
    using namespace clang::ast_matchers;
    // Finds cases like:
    // v[0] == "foo"
    // i.e. operator ==() has an argument, which is a StringVector::operator
    // []() result.
    return cxxOperatorCallExpr(
               hasDescendant(
                   declRefExpr(to(functionDecl(hasName("operator=="))))),
               hasDescendant(cxxOperatorCallExpr(
                   hasDescendant(
                       declRefExpr(to(cxxMethodDecl(hasName("operator[]"))))),
                   hasDescendant(declRefExpr(to(varDecl(
                       hasType(cxxRecordDecl(hasName("StringVector"))))))))))
        .bind("expr");
}

llvm::cl::OptionCategory category("ast-matcher options");
} // namespace

int main(int argc, const char** argv)
{
    llvm::sys::PrintStackTraceOnErrorSignal(argv[0]);
    clang::tooling::CommonOptionsParser optionsParser(argc, argv, category);
    clang::tooling::RefactoringTool tool(optionsParser.getCompilations(),
                                         optionsParser.getSourcePathList());
    clang::ast_matchers::MatchFinder finder;
    Callback callback;
    finder.addMatcher(makeMatcher(), &callback);
    return tool.run(clang::tooling::newFrontendActionFactory(&finder).get());
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
