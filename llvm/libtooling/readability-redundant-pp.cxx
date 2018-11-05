#include <iostream>

#include <clang/Frontend/CompilerInstance.h>
#include <clang/Frontend/FrontendActions.h>
#include <clang/Tooling/CommonOptionsParser.h>
#include <clang/Tooling/Refactoring.h>
#include <llvm/Support/Signals.h>

namespace
{

class RedundantPPCallbacks : public clang::PPCallbacks
{
  public:
    RedundantPPCallbacks(clang::Preprocessor& rPP);
    void Ifndef(clang::SourceLocation aLoc, const clang::Token& rMacroNameTok,
                const clang::MacroDefinition& rMacroDefinition) override;
    void Endif(clang::SourceLocation aLoc,
               clang::SourceLocation aIfLoc) override;

  private:
    clang::Preprocessor& m_rPP;
};

RedundantPPCallbacks::RedundantPPCallbacks(clang::Preprocessor& rPP)
    : m_rPP(rPP)
{
}

void RedundantPPCallbacks::Ifndef(
    clang::SourceLocation aLoc, const clang::Token& rMacroNameTok,
    const clang::MacroDefinition& /*rMacroDefinition*/)
{
    std::cerr << "debug, RedundantPPCallbacks::Ifndef: aLoc is ";
    aLoc.dump(m_rPP.getSourceManager());
    std::cerr << ", rMacroNameTok is '" << m_rPP.getSpelling(rMacroNameTok)
              << "'" << std::endl;
}

void RedundantPPCallbacks::Endif(clang::SourceLocation aLoc,
                                 clang::SourceLocation aIfLoc)
{
    std::cerr << "debug, RedundantPPCallbacks::Endif: aLoc is ";
    aLoc.dump(m_rPP.getSourceManager());
    std::cerr << ", IfLoc is ";
    aIfLoc.dump(m_rPP.getSourceManager());
    std::cerr << std::endl;
}

class RedundantPPConsumer : public clang::ASTConsumer
{
  public:
    RedundantPPConsumer(clang::Preprocessor& rPP)
    {
        rPP.addPPCallbacks(llvm::make_unique<RedundantPPCallbacks>(rPP));
    }
};

class RedundantPPAction : public clang::SyntaxOnlyAction
{
  public:
    RedundantPPAction() {}

  protected:
    std::unique_ptr<clang::ASTConsumer>
    CreateASTConsumer(clang::CompilerInstance& rInstance,
                      StringRef /*aFile*/) override
    {
        return llvm::make_unique<RedundantPPConsumer>(
            rInstance.getPreprocessor());
    }
};

class RedundantPPFrontendActionFactory
    : public clang::tooling::FrontendActionFactory
{
  public:
    RedundantPPFrontendActionFactory() {}

    RedundantPPAction* create() override { return new RedundantPPAction(); }
};

llvm::cl::extrahelp
    aCommonHelp(clang::tooling::CommonOptionsParser::HelpMessage);
llvm::cl::OptionCategory aCategory("redundant-pp options");
}

int main(int argc, const char** argv)
{
    llvm::sys::PrintStackTraceOnErrorSignal(argv[0]);
    clang::tooling::CommonOptionsParser aOptionsParser(argc, argv, aCategory);
    clang::tooling::RefactoringTool aTool(aOptionsParser.getCompilations(),
                                          aOptionsParser.getSourcePathList());
    RedundantPPFrontendActionFactory aFactory;
    return aTool.run(&aFactory);
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
