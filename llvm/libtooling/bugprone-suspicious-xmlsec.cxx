/*
 * This is free software; see Copyright file in the xmlsec source distribution
 * for preciese wording.
 */

#include <fstream>
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
bool startsWith(const std::string& str, const std::string& prefix)
{
    return str.size() >= prefix.size() &&
           str.compare(0, prefix.size(), prefix) == 0;
}

bool endsWith(const std::string& str, const std::string& suffix)
{
    return str.size() >= suffix.size() &&
           str.compare(str.size() - suffix.size(), suffix.size(), suffix) == 0;
}

bool readLines(const std::string& rPath, std::vector<std::string>& rLines)
{
    std::ifstream aStream(rPath);
    if (!aStream.is_open())
    {
        std::cerr << "parseLines: failed to open " << rPath << std::endl;
        return false;
    }

    std::string aLine;
    while (std::getline(aStream, aLine))
        rLines.push_back(aLine);

    return true;
}

class Callback : public clang::ast_matchers::MatchFinder::MatchCallback
{
  public:
    Callback(const std::vector<std::string>& rWhitelist)
        : m_rWhitelist(rWhitelist)
    {
    }

    void
    run(const clang::ast_matchers::MatchFinder::MatchResult& rResult) override
    {
        if (const auto pExpr = rResult.Nodes.getNodeAs<clang::CallExpr>("expr"))
        {
            clang::SourceLocation aExpansionLocation =
                rResult.Context->getSourceManager().getExpansionLoc(
                    pExpr->getLocStart());
            std::string aFilename = rResult.Context->getSourceManager()
                                        .getPresumedLoc(aExpansionLocation)
                                        .getFilename();
            if (whitelist(aFilename))
                return;

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

    bool whitelist(const std::string& rFilename)
    {
        for (const auto& rItem : m_rWhitelist)
            if (endsWith(rFilename, rItem))
                return true;

        return false;
    }

    std::string m_aLastCalledFunction;
    const std::vector<std::string>& m_rWhitelist;
};

clang::ast_matchers::StatementMatcher makeMatcher()
{
    using namespace clang::ast_matchers;
    return callExpr(unless(hasAncestor(callExpr()))).bind("expr");
}

llvm::cl::extrahelp
    aCommonHelp(clang::tooling::CommonOptionsParser::HelpMessage);
llvm::cl::OptionCategory aCategory("suspicious-xmlsec options");

// Stubs for clang-tidy compatibility.
llvm::cl::opt<bool> aListChecks("list-checks", llvm::cl::desc("ignored"),
                                llvm::cl::init(false),
                                llvm::cl::cat(aCategory));
llvm::cl::opt<std::string> aHeaderFilter("header-filter",
                                         llvm::cl::desc("ignored"),
                                         llvm::cl::init(""),
                                         llvm::cl::cat(aCategory));
}

int main(int argc, const char** argv)
{
    llvm::sys::PrintStackTraceOnErrorSignal();
    clang::tooling::CommonOptionsParser aOptionsParser(argc, argv, aCategory);
    if (aListChecks)
        return 0;

    clang::tooling::RefactoringTool aTool(aOptionsParser.getCompilations(),
                                          aOptionsParser.getSourcePathList());
    clang::ast_matchers::MatchFinder aFinder;
    std::vector<std::string> aWhitelist;
    if (const char* pWhitelist = getenv("WHITELIST_FILE"))
    {
        if (!readLines(pWhitelist, aWhitelist))
            return 1;
    }

    Callback aCallback(aWhitelist);
    aFinder.addMatcher(makeMatcher(), &aCallback);
    int nRet =
        aTool.run(clang::tooling::newFrontendActionFactory(&aFinder).get());

    return nRet;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
