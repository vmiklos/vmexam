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
    Callback() {}

    void
    run(const clang::ast_matchers::MatchFinder::MatchResult& result) override
    {
        const auto decl = result.Nodes.getNodeAs<clang::FunctionDecl>("decl");
        if (!decl)
        {
            return;
        }

        clang::SourceLocation location = decl->getLocation();
        if (!result.Context->getSourceManager().isInMainFile(location))
        {
            return;
        }

        std::cerr << "debug, found function decl" << std::endl;
    }
};

clang::ast_matchers::DeclarationMatcher makeMatcher()
{
    using namespace clang::ast_matchers;
    return functionDecl().bind("decl");
}

llvm::cl::extrahelp help(clang::tooling::CommonOptionsParser::HelpMessage);
llvm::cl::OptionCategory category("test options");
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
