#include <cassert>
#include <fstream>
#include <iostream>
#include <iterator>
#include <vector>

#include <fpdf_edit.h>
#include <fpdf_save.h>
#include <fpdf_signature.h>
#include <fpdfview.h>

int main(int argc, char* argv[])
{
    FPDF_LIBRARY_CONFIG config;
    config.version = 2;
    config.m_pUserFontPaths = nullptr;
    config.m_pIsolate = nullptr;
    config.m_v8EmbedderSlot = 0;
    FPDF_InitLibraryWithConfig(&config);

    std::ifstream testFile(argv[1], std::ios::binary);
    std::vector<char> fileContents((std::istreambuf_iterator<char>(testFile)),
                                   std::istreambuf_iterator<char>());
    FPDF_DOCUMENT document = FPDF_LoadMemDocument(
        fileContents.data(), fileContents.size(), /*password=*/nullptr);
    assert(document);

    int signatureCount = FPDF_GetSignatureCount(document);
    for (int i = 0; i < signatureCount; ++i)
    {
        std::cerr << "Signature #" << i << ":" << std::endl;
        std::cerr << "debug, signature is "
                  << FPDF_GetSignatureObject(document, i) << std::endl;
    }

    FPDF_CloseDocument(document);

    FPDF_DestroyLibrary();
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
