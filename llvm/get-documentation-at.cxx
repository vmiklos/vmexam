/*
 * Copyright 2019 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
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

    if (!pUnit)
        return 1;

    const CXFile pFile = clang_getFile(pUnit, aFile.c_str());
    int nLine = std::stoi(argv[2]);
    int nColumn = std::stoi(argv[3]);
    CXSourceLocation aLocation =
        clang_getLocation(pUnit, pFile, nLine, nColumn);
    CXCursor aCursor = clang_getCursor(pUnit, aLocation);
    if (clang_Cursor_isNull(aCursor) ||
        clang_isInvalid(clang_getCursorKind(aCursor)))
        return 1;

    CXCursor aReferencedCursor = clang_getCursorReferenced(aCursor);
    if (!clang_Cursor_isNull(aReferencedCursor) &&
        !clang_isInvalid(clang_getCursorKind(aReferencedCursor)))
        aCursor = aReferencedCursor;

    CXCursor aCanonicalCursor = clang_getCanonicalCursor(aCursor);
    if (clang_Cursor_isNull(aCanonicalCursor) ||
        clang_isInvalid(clang_getCursorKind(aCanonicalCursor)))
        return 1;

    CXString aString = clang_Cursor_getBriefCommentText(aCanonicalCursor);
    std::cerr << "'" << clang_getCString(aString) << "'" << std::endl;
    clang_disposeString(aString);

    clang_disposeTranslationUnit(pUnit);
    clang_disposeIndex(pIndex);
    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
