/*
 * Copyright 2019 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 *
 * Code completion prototype.
 */

#include <iostream>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <vector>

#include <clang-c/Index.h>

int main(int argc, char** argv)
{
    if (argc < 5)
    {
        std::cerr << "usage: " << argv[0]
                  << " <file> <linenum> <colnum> <compiler args...>"
                  << std::endl;
        return 1;
    }

    CXIndex pIndex =
        clang_createIndex(/*excludeDeclsFromPCH=*/1, /*displayDiagnostics=*/0);

    std::string aFile = argv[1];
    std::vector<std::string> aArgs;
    for (int i = 4; i < argc; ++i)
        aArgs.push_back(argv[i]);
    std::vector<const char*> aArgPtrs(aArgs.size());
    for (size_t i = 0; i < aArgs.size(); ++i)
        aArgPtrs[i] = aArgs[i].c_str();
    CXTranslationUnit pUnit = clang_parseTranslationUnit(
        pIndex, aFile.c_str(), aArgPtrs.data(), aArgPtrs.size(), nullptr, 0,
        CXTranslationUnit_Incomplete);

    if (pUnit)
    {
        unsigned nLine = std::stoi(argv[2]);
        unsigned nColumn = std::stoi(argv[3]);
        CXCodeCompleteResults* pResults =
            clang_codeCompleteAt(pUnit, aFile.c_str(), nLine, nColumn, nullptr,
                                 0, clang_defaultCodeCompleteOptions());
        std::set<std::string> aSet;
        if (pResults)
        {
            for (unsigned i = 0; i < pResults->NumResults; ++i)
            {
                const CXCompletionString& rCompletionString =
                    pResults->Results[i].CompletionString;
                std::stringstream ss;
                for (unsigned j = 0;
                     j < clang_getNumCompletionChunks(rCompletionString); ++j)
                {
                    if (clang_getCompletionChunkKind(rCompletionString, j) !=
                        CXCompletionChunk_TypedText)
                        continue;

                    const CXString& rString =
                        clang_getCompletionChunkText(rCompletionString, j);
                    ss << clang_getCString(rString);
                }
                aSet.insert(ss.str());
            }
            clang_disposeCodeCompleteResults(pResults);
        }
        for (const std::string& rString : aSet)
            std::cerr << rString << std::endl;
    }

    clang_disposeTranslationUnit(pUnit);
    clang_disposeIndex(pIndex);
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
