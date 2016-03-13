/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * libclang version of
 * <http://vim.wikia.com/wiki/Show_current_function_name_in_C_programs>, i.e.
 * show the fully qualified name of the function, if the cursor is inside a
 * function definition.
 */

#include <iostream>
#include <sstream>
#include <stack>
#include <string>
#include <vector>

#include <clang-c/Index.h>

/// Function or member function.
bool isFunction(CXCursorKind eKind)
{
    return eKind == CXCursor_FunctionDecl || eKind == CXCursor_CXXMethod;
}

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

    if (pUnit)
    {
        const CXFile pFile = clang_getFile(pUnit, aFile.c_str());
        CXSourceLocation aLocation = clang_getLocation(pUnit, pFile, /*line=*/std::stoi(argv[2]), /*column=*/std::stoi(argv[3]));
        CXCursor aCursor = clang_getCursor(pUnit, aLocation);

        CXCursorKind eKind = clang_getCursorKind(aCursor);
        while (true)
        {
            if (isFunction(eKind) || eKind == CXCursor_TranslationUnit)
                break;
            aCursor = clang_getCursorSemanticParent(aCursor);
            eKind = clang_getCursorKind(aCursor);
        }

        if (eKind != CXCursor_TranslationUnit)
        {
            std::stack<std::string> aStack;
            while (true)
            {
                CXString pString = clang_getCursorSpelling(aCursor);
                aStack.push(clang_getCString(pString));
                clang_disposeString(pString);

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
    }

    clang_disposeTranslationUnit(pUnit);
    clang_disposeIndex(pIndex);
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
