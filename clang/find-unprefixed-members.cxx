#include <fstream>
#include <iostream>
#include <set>
#include <sstream>

#include <clang/AST/ASTConsumer.h>
#include <clang/AST/ASTContext.h>
#include <clang/AST/RecursiveASTVisitor.h>
#include <clang/Rewrite/Core/Rewriter.h>
#include <clang/Tooling/CommonOptionsParser.h>
#include <clang/Tooling/Tooling.h>

class Context
{
    std::string m_aClassName;
    std::string m_aClassPrefix;

public:
    Context(const std::string& rClassName, const std::string& rClassPrefix)
        : m_aClassName(rClassName),
          m_aClassPrefix(rClassPrefix)
    {
    }

    bool match(const std::string& rName) const
    {
        if (m_aClassName == "")
            return rName.find(m_aClassPrefix) == 0;
        else
            return rName == m_aClassName;
    }
};

class Visitor : public clang::RecursiveASTVisitor<Visitor>
{
    const Context m_rContext;
    bool m_bFound;

public:
    Visitor(const Context& rContext)
        : m_rContext(rContext),
          m_bFound(false)
    {
    }

    bool getFound()
    {
        return m_bFound;
    }

    /*
     * class C
     * {
     * public:
     *     int nX; <- Handles this declaration.
     * };
     */
    bool VisitFieldDecl(clang::FieldDecl* pDecl)
    {
        clang::RecordDecl* pRecord = pDecl->getParent();

        if (m_rContext.match(pRecord->getQualifiedNameAsString()))
        {
            std::string aName = pDecl->getNameAsString();
            if (aName.find("m") != 0)
            {
                aName.insert(0, "m_");
                std::cout << pRecord->getQualifiedNameAsString() << "::" << pDecl->getNameAsString() << "," << aName << std::endl;
                m_bFound = true;
            }
        }

        return true;
    }

    /*
     * class C
     * {
     * public:
     *     static const int aS[]; <- Handles e.g. this declaration;
     * };
     */
    bool VisitVarDecl(clang::VarDecl* pDecl)
    {
        if (!pDecl->getQualifier())
            return true;

        clang::RecordDecl* pRecord = pDecl->getQualifier()->getAsType()->getAsCXXRecordDecl();

        if (m_rContext.match(pRecord->getQualifiedNameAsString()))
        {
            std::string aName = pDecl->getNameAsString();
            if (aName.find("m") != 0)
            {
                aName.insert(0, "m_");
                std::cout << pRecord->getQualifiedNameAsString() << "::" << pDecl->getNameAsString() << "," << aName << std::endl;
                m_bFound = true;
            }
        }

        return true;
    }
};

class ASTConsumer : public clang::ASTConsumer
{
    const Context& m_rContext;

public:
    ASTConsumer(const Context& rContext)
        : m_rContext(rContext)
    {
    }

    virtual void HandleTranslationUnit(clang::ASTContext& rContext)
    {
        if (rContext.getDiagnostics().hasErrorOccurred())
            return;

        Visitor aVisitor(m_rContext);
        aVisitor.TraverseDecl(rContext.getTranslationUnitDecl());
        if (aVisitor.getFound())
            exit(1);
    }
};

class FrontendAction
{
    const Context& m_rContext;

public:
    FrontendAction(const Context& rContext)
        : m_rContext(rContext)
    {
    }

    clang::ASTConsumer* newASTConsumer()
    {
        return new ASTConsumer(m_rContext);
    }
};

int main(int argc, const char** argv)
{
    llvm::cl::OptionCategory aCategory("find-unprefixed-members options");
    llvm::cl::opt<std::string> aClassName("class-name",
                                          llvm::cl::desc("Qualified name (namespace::Class)."),
                                          llvm::cl::cat(aCategory));
    llvm::cl::opt<std::string> aClassPrefix("class-prefix",
                                            llvm::cl::desc("Qualified name prefix (e.g. namespace::Cl)."),
                                            llvm::cl::cat(aCategory));
    clang::tooling::CommonOptionsParser aParser(argc, argv, aCategory);

    if (aClassName.empty() && aClassPrefix.empty())
    {
        std::cerr << "-class-name or -class-prefix is required." << std::endl;
        return 1;
    }

    clang::tooling::ClangTool aTool(aParser.getCompilations(), aParser.getSourcePathList());

    Context aContext(aClassName, aClassPrefix);
    FrontendAction aAction(aContext);
    std::unique_ptr<clang::tooling::FrontendActionFactory> pFactory = clang::tooling::newFrontendActionFactory(&aAction);
    return aTool.run(pFactory.get());
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
