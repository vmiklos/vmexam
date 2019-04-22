/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

#include <iostream>
#include <sstream>
#include <stack>
#include <string>
#include <vector>

#include <clang-c/Index.h>

int main(int argc, char** argv)
{
    if (argc < 3)
    {
        std::cerr << "usage: " << argv[0] << " <file> <compiler args...>"
                  << std::endl;
        return 1;
    }

    CXIndex pIndex =
        clang_createIndex(/*excludeDeclsFromPCH=*/1, /*displayDiagnostics=*/0);

    std::string aFile = argv[1];
    std::vector<std::string> aArgs;
    for (int i = 2; i < argc; ++i)
        aArgs.push_back(argv[i]);
    std::vector<const char*> aArgPtrs(aArgs.size());
    for (size_t i = 0; i < aArgs.size(); ++i)
        aArgPtrs[i] = aArgs[i].c_str();
    CXTranslationUnit pUnit = clang_parseTranslationUnit(
        pIndex, aFile.c_str(), aArgPtrs.data(), aArgPtrs.size(), nullptr, 0,
        CXTranslationUnit_Incomplete);

    if (pUnit)
    {
        unsigned nNumDiagnostics = clang_getNumDiagnostics(pUnit);
        std::cerr << "nNumDiagnostics is " << nNumDiagnostics << std::endl;
        for (unsigned i = 0; i < nNumDiagnostics; ++i)
        {
            CXDiagnostic pDiagnostic = clang_getDiagnostic(pUnit, i);
            if (pDiagnostic)
            {
                switch (clang_getDiagnosticSeverity(pDiagnostic))
                {
                case CXDiagnostic_Warning:
                    std::cerr << "warning, " << std::endl;
                    break;
                case CXDiagnostic_Error:
                    std::cerr << "error, " << std::endl;
                    break;
                default:
                    break;
                }

                CXSourceLocation aDiagnosticLocation =
                    clang_getDiagnosticLocation(pDiagnostic);
                CXFile pDiagnosticFile;
                unsigned nDiagnosticLine;
                unsigned nDiagnosticCol;
                clang_getExpansionLocation(aDiagnosticLocation,
                                           &pDiagnosticFile, &nDiagnosticLine,
                                           &nDiagnosticCol, nullptr);
                CXString aDiagnosticFileName =
                    clang_getFileName(pDiagnosticFile);
                CXString aFormattedDiagnistic = clang_formatDiagnostic(
                    pDiagnostic, clang_defaultDiagnosticDisplayOptions());
                std::cerr << clang_getCString(aDiagnosticFileName) << ":"
                          << nDiagnosticLine << ":" << nDiagnosticCol << ": "
                          << clang_getCString(aFormattedDiagnistic)
                          << std::endl;
                clang_disposeString(aFormattedDiagnistic);
                clang_disposeString(aDiagnosticFileName);
            }
            clang_disposeDiagnostic(pDiagnostic);
        }
    }

    clang_disposeTranslationUnit(pUnit);
    clang_disposeIndex(pIndex);
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
