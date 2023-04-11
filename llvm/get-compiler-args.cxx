/*
 * Copyright 2019 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 *
 * Code completion prototype.
 */

#include <fstream>
#include <iostream>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <vector>

#include <clang-c/CXCompilationDatabase.h>

int main(int argc, char** argv)
{
    if (argc < 2)
    {
        std::cerr << "usage: " << argv[0] << " <file>" << std::endl;
        return 1;
    }

    std::string aFile = argv[1];
    std::size_t nFound = aFile.find_last_of("/\\");
    std::string aDirectory = aFile.substr(0, nFound);
    while (true)
    {
        std::string aJSON =
            aDirectory + aFile.substr(nFound, 1) + "compile_commands.json";
        std::ifstream aStream(aJSON.c_str());
        if (aStream.good())
            break;

        nFound = aDirectory.find_last_of("/\\");
        if (nFound == std::string::npos)
            break;

        aDirectory = aDirectory.substr(0, nFound);
    }

    if (aDirectory.empty())
    {
        // Fall back to our default as no JSON was found.
        std::cout << "-std=c++1y" << std::endl;
        return 0;
    }

    CXCompilationDatabase_Error eError;
    CXCompilationDatabase pDatabase =
        clang_CompilationDatabase_fromDirectory(aDirectory.c_str(), &eError);
    if (eError == CXCompilationDatabase_NoError)
    {
        CXCompileCommands pCommands =
            clang_CompilationDatabase_getCompileCommands(pDatabase,
                                                         aFile.c_str());
        unsigned nCommandsSize = clang_CompileCommands_getSize(pCommands);
        if (nCommandsSize >= 1)
        {
            CXCompileCommand pCommand =
                clang_CompileCommands_getCommand(pCommands, 0);
            unsigned nArgs = clang_CompileCommand_getNumArgs(pCommand);
            std::stringstream ss;
            for (unsigned i = 0; i < nArgs; ++i)
            {
                CXString aArg = clang_CompileCommand_getArg(pCommand, i);
                if (aFile == clang_getCString(aArg))
                    continue;

                if (i)
                    ss << " ";
                ss << clang_getCString(aArg);
                clang_disposeString(aArg);
            }
            std::cout << ss.str() << std::endl;
        }
        else
            std::cerr << "clang_CompileCommands_getSize() returned "
                      << nCommandsSize << std::endl;
        clang_CompileCommands_dispose(pCommands);
    }
    else
        std::cerr << "clang_CompilationDatabase_fromDirectory() returned "
                  << eError << std::endl;
    clang_CompilationDatabase_dispose(pDatabase);
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
