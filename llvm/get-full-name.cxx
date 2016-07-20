/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#include <iostream>
#include <sstream>
#include <stack>
#include <string>
#include <vector>

#include <clang-c/Index.h>

int main(int argc, char** argv)
{
    if (argc < 5)
    {
        std::cerr << "usage: " << argv[0] << " <file> <linenum> <colnum> <compiler args...>" << std::endl;
        return 1;
    }

    CXIndex pIndex = clang_createIndex(/*excludeDeclsFromPCH=*/1, /*displayDiagnostics=*/0);

    std::string aFile = argv[1];
    std::vector<std::string> aArgs;
    for (int i = 4; i < argc; ++i)
        aArgs.push_back(argv[i]);
    std::vector<const char*> aArgPtrs(aArgs.size());
    for (size_t i = 0; i < aArgs.size(); ++i)
        aArgPtrs[i] = aArgs[i].c_str();
    CXTranslationUnit pUnit = clang_parseTranslationUnit(pIndex, aFile.c_str(), aArgPtrs.data(), aArgPtrs.size(), nullptr, 0, CXTranslationUnit_Incomplete);

    if (!pUnit)
        return 1;

    const CXFile pFile = clang_getFile(pUnit, aFile.c_str());
    int nLine = std::stoi(argv[2]);
    int nColumn = std::stoi(argv[3]);
    CXSourceLocation aLocation = clang_getLocation(pUnit, pFile, nLine, nColumn);
    CXCursor aCursor = clang_getCursor(pUnit, aLocation);
    if (clang_Cursor_isNull(aCursor) || clang_isInvalid(clang_getCursorKind(aCursor)))
        return 1;

    aCursor = clang_getCursorReferenced(aCursor);
    if (clang_Cursor_isNull(aCursor) || clang_isInvalid(clang_getCursorKind(aCursor)))
        return 1;

    CXCursorKind eKind = clang_getCursorKind(aCursor);
    if (eKind != CXCursor_TranslationUnit)
    {
        std::stack<std::string> aStack;
        while (true)
        {
            CXString aString = clang_getCursorSpelling(aCursor);
            aStack.push(clang_getCString(aString));
            clang_disposeString(aString);

            aCursor = clang_getCursorSemanticParent(aCursor);
            if (clang_getCursorKind(aCursor) == CXCursor_TranslationUnit)
                break;
        }

        bool bFirst = true;
        while (!aStack.empty())
        {
            if (bFirst)
                bFirst = false;
            else
                std::cout << "::";
            std::cout << aStack.top();
            aStack.pop();
        }
        std::cout << std::endl;
    }

    clang_disposeTranslationUnit(pUnit);
    clang_disposeIndex(pIndex);
    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
