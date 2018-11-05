#include <iostream>
#include <stack>

#include <clang/Frontend/CompilerInstance.h>
#include <clang/Frontend/FrontendActions.h>
#include <clang/Tooling/CommonOptionsParser.h>
#include <clang/Tooling/Refactoring.h>
#include <llvm/Support/Signals.h>

namespace
{

struct Entry
{
    clang::SourceLocation m_aLoc;
    std::string m_aMacroName;
};

class RedundantPPCallbacks : public clang::PPCallbacks
{
  public:
    RedundantPPCallbacks(clang::Preprocessor& rPP);
    void Ifndef(clang::SourceLocation aLoc, const clang::Token& rMacroNameTok,
                const clang::MacroDefinition& rMacroDefinition) override;
    void Endif(clang::SourceLocation aLoc,
               clang::SourceLocation aIfLoc) override;
    ~RedundantPPCallbacks() override;

  private:
    clang::DiagnosticBuilder reportWarning(llvm::StringRef aString,
                                           clang::SourceLocation aLocation);
    clang::DiagnosticBuilder report(clang::DiagnosticIDs::Level eLevel,
                                    llvm::StringRef aString,
                                    clang::SourceLocation aLocation);

    clang::Preprocessor& m_rPP;
    std::vector<Entry> m_aStack;
};

RedundantPPCallbacks::RedundantPPCallbacks(clang::Preprocessor& rPP)
    : m_rPP(rPP)
{
}

RedundantPPCallbacks::~RedundantPPCallbacks() {}

void RedundantPPCallbacks::Ifndef(
    clang::SourceLocation aLoc, const clang::Token& rMacroNameTok,
    const clang::MacroDefinition& /*rMacroDefinition*/)
{
    if (m_rPP.getSourceManager().isInMainFile(aLoc))
    {
        std::string aMacroName = m_rPP.getSpelling(rMacroNameTok);
        for (const auto& rEntry : m_aStack)
        {
            if (rEntry.m_aMacroName == aMacroName)
            {
                reportWarning("nested ifdef", aLoc);
                report(clang::DiagnosticIDs::Note, "previous ifdef",
                       rEntry.m_aLoc);
            }
        }
    }

    Entry aEntry;
    aEntry.m_aLoc = aLoc;
    aEntry.m_aMacroName = m_rPP.getSpelling(rMacroNameTok);
    m_aStack.push_back(aEntry);
}

void RedundantPPCallbacks::Endif(clang::SourceLocation /*aLoc*/,
                                 clang::SourceLocation aIfLoc)
{
    if (m_aStack.empty())
        return;

    if (aIfLoc == m_aStack.back().m_aLoc)
        m_aStack.pop_back();
}

clang::DiagnosticBuilder
RedundantPPCallbacks::reportWarning(llvm::StringRef aString,
                                    clang::SourceLocation aLocation)
{
    clang::DiagnosticsEngine& rEngine = m_rPP.getDiagnostics();
    clang::DiagnosticIDs::Level eLevel = clang::DiagnosticIDs::Level::Warning;
    if (rEngine.getWarningsAsErrors())
        eLevel = clang::DiagnosticIDs::Level::Error;
    return report(eLevel, aString, aLocation);
}

clang::DiagnosticBuilder
RedundantPPCallbacks::report(clang::DiagnosticIDs::Level eLevel,
                             llvm::StringRef aString,
                             clang::SourceLocation aLocation)
{
    clang::DiagnosticsEngine& rEngine = m_rPP.getDiagnostics();
    return rEngine.Report(
        aLocation,
        rEngine.getDiagnosticIDs()->getCustomDiagID(eLevel, aString));
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
