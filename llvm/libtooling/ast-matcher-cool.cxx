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
        const auto functionDecl =
            result.Nodes.getNodeAs<clang::FunctionDecl>("functionDecl");
        if (!functionDecl)
        {
            return;
        }

        std::set<const clang::ParmVarDecl*> functionParams;
        for (const clang::ParmVarDecl* functionParm :
             functionDecl->parameters())
        {
            functionParams.insert(functionParm);
        }

        const auto lambdaExpr =
            result.Nodes.getNodeAs<clang::LambdaExpr>("lambdaExpr");
        if (!lambdaExpr)
        {
            return;
        }

        for (auto captureIt = lambdaExpr->capture_begin();
             captureIt != lambdaExpr->capture_end(); ++captureIt)
        {
            const clang::LambdaCapture& capture = *captureIt;
            if (!capture.capturesVariable())
            {
                continue;
            }

            if (capture.getCaptureKind() != clang::LCK_ByRef)
            {
                continue;
            }

            auto lambdaParm =
                llvm::dyn_cast<clang::ParmVarDecl>(capture.getCapturedVar());
            if (!lambdaParm)
            {
                continue;
            }

            llvm::StringRef lambdaParmName = lambdaParm->getName();
            if (lambdaParmName == "poll")
            {
                continue;
            }

            auto it = functionParams.find(lambdaParm);
            if (it == functionParams.end())
            {
                continue;
            }

            clang::SourceManager& sourceManager =
                result.Context->getSourceManager();
            if (sourceManager.isInSystemHeader(capture.getLocation()))
            {
                continue;
            }

            clang::SourceRange range(capture.getLocation());
            clang::SourceLocation location(range.getBegin());
            report(result.Context, "ast-matcher", location) << range;
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
    return lambdaExpr(hasAncestor(functionDecl().bind("functionDecl")),
                      hasParent(varDecl()))
        .bind("lambdaExpr");
}

llvm::cl::OptionCategory category("ast-matcher options");
} // namespace

int main(int argc, const char** argv)
{
    llvm::sys::PrintStackTraceOnErrorSignal(argv[0]);
    llvm::Expected<clang::tooling::CommonOptionsParser> optionsParser =
        clang::tooling::CommonOptionsParser::create(argc, argv, category);
    clang::tooling::RefactoringTool tool(optionsParser->getCompilations(),
                                         optionsParser->getSourcePathList());
    clang::ast_matchers::MatchFinder finder;
    Callback callback;
    finder.addMatcher(makeMatcher(), &callback);
    return tool.run(clang::tooling::newFrontendActionFactory(&finder).get());
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
