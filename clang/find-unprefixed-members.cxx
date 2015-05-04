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

class Visitor : public clang::RecursiveASTVisitor<Visitor>
{
    std::string m_aClassName;
    bool m_bFound;

public:
    Visitor(const std::string& rClassName)
        : m_aClassName(rClassName),
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

        if (pRecord->getQualifiedNameAsString() == m_aClassName)
        {
            std::string aName = pDecl->getNameAsString();
            if (aName.find("m_") != 0)
            {
                if (aName.find("m") == 0)
                    aName.insert(1, "_");
                else
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
    std::string m_aClassName;

public:
    ASTConsumer(const std::string& rClassName)
        : m_aClassName(rClassName)
    {
    }

    virtual void HandleTranslationUnit(clang::ASTContext& rContext)
    {
        if (rContext.getDiagnostics().hasErrorOccurred())
            return;

        Visitor aVisitor(m_aClassName);
        aVisitor.TraverseDecl(rContext.getTranslationUnitDecl());
        if (aVisitor.getFound())
            exit(1);
    }
};

class FrontendAction
{
    std::string m_aClassName;

public:
    FrontendAction(const std::string& rClassName)
        : m_aClassName(rClassName)
    {
    }

    clang::ASTConsumer* newASTConsumer()
    {
        return new ASTConsumer(m_aClassName);
    }
};

int main(int argc, const char** argv)
{
    llvm::cl::OptionCategory aCategory("find-unprefixed-members options");
    llvm::cl::opt<std::string> aClassName("class-name",
                                          llvm::cl::desc("Qualified name (namespace::Class)."),
                                          llvm::cl::cat(aCategory));
    clang::tooling::CommonOptionsParser aParser(argc, argv, aCategory);

    if (aClassName.empty())
    {
        std::cerr << "-class-name is required." << std::endl;
        return 1;
    }

    clang::tooling::ClangTool aTool(aParser.getCompilations(), aParser.getSourcePathList());

    FrontendAction aAction(aClassName);
    std::unique_ptr<clang::tooling::FrontendActionFactory> pFactory = clang::tooling::newFrontendActionFactory(&aAction);
    return aTool.run(pFactory.get());
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
