#include <iostream>

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
class Callback : public clang::ast_matchers::MatchFinder::MatchCallback
{
  public:
    Callback(clang::tooling::Replacements* pReplacements)
        : m_pReplacements(pReplacements)
    {
    }

    void
    run(const clang::ast_matchers::MatchFinder::MatchResult& rResult) override
    {
        if (const auto pDecl = rResult.Nodes.getNodeAs<clang::VarDecl>("decl"))
        {
            clang::SourceRange aRange(
                pDecl->getTypeSourceInfo()->getTypeLoc().getSourceRange());
            clang::QualType aType = pDecl->getType();
            clang::FixItHint aFixIt =
                fixit(rResult.Context, aRange, aType.getAsString());
            report(rResult.Context, "avoid auto", aRange.getBegin()) << aRange
                                                                     << aFixIt;
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

    clang::FixItHint fixit(clang::ASTContext* pContext,
                           const clang::SourceRange& rRange,
                           const std::string& rString)
    {
        clang::FixItHint aFixIt =
            clang::FixItHint::CreateReplacement(rRange, rString);
        m_pReplacements->insert(clang::tooling::Replacement(
            pContext->getSourceManager(), aFixIt.RemoveRange,
            aFixIt.CodeToInsert));
        return aFixIt;
    }

    clang::tooling::Replacements* m_pReplacements;
};

clang::ast_matchers::StatementMatcher makeMatcher()
{
    using namespace clang::ast_matchers;
    return declStmt(has(varDecl(hasType(autoType())).bind("decl")));
}

llvm::cl::extrahelp
    aCommonHelp(clang::tooling::CommonOptionsParser::HelpMessage);
llvm::cl::OptionCategory aCategory("avoid-auto options");
llvm::cl::opt<std::string>
    aExportFixes("export-fixes",
                 llvm::cl::desc("YAML file to store suggested fixes in."),
                 llvm::cl::value_desc("filename"), llvm::cl::cat(aCategory));
}

int main(int argc, const char** argv)
{
    llvm::sys::PrintStackTraceOnErrorSignal();
    clang::tooling::CommonOptionsParser aOptionsParser(argc, argv, aCategory);
    clang::tooling::RefactoringTool aTool(aOptionsParser.getCompilations(),
                                          aOptionsParser.getSourcePathList());
    clang::ast_matchers::MatchFinder aFinder;
    Callback aCallback(&aTool.getReplacements());
    aFinder.addMatcher(makeMatcher(), &aCallback);
    int nRet =
        aTool.run(clang::tooling::newFrontendActionFactory(&aFinder).get());

    if (!aExportFixes.empty())
    {
        std::error_code aEC;
        llvm::raw_fd_ostream aOS(aExportFixes, aEC, llvm::sys::fs::F_None);
        if (aEC)
        {
            llvm::errs() << "Error opening output file: " << aEC.message()
                         << '\n';
            return 1;
        }

        clang::tooling::TranslationUnitReplacements aTUR;
        const clang::tooling::Replacements& rReplacements =
            aTool.getReplacements();
        aTUR.Replacements.insert(aTUR.Replacements.end(), rReplacements.begin(),
                                 rReplacements.end());
        llvm::yaml::Output aYAML(aOS);
        aYAML << aTUR;
        aOS.close();
    }

    return nRet;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
