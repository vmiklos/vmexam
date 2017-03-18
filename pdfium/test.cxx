#include <cassert>
#include <fstream>
#include <iostream>
#include <iterator>
#include <vector>

#include <fpdf_edit.h>
#include <fpdfview.h>

int main()
{
    FPDF_LIBRARY_CONFIG config;
    config.version = 2;
    config.m_pUserFontPaths = nullptr;
    config.m_pIsolate = nullptr;
    config.m_v8EmbedderSlot = 0;
    FPDF_InitLibraryWithConfig(&config);

    std::ifstream testFile("tdf106059.pdf", std::ios::binary);
    std::vector<char> fileContents((std::istreambuf_iterator<char>(testFile)),
                                   std::istreambuf_iterator<char>());
    FPDF_DOCUMENT document = FPDF_LoadMemDocument(
        fileContents.data(), fileContents.size(), /*password=*/nullptr);
    assert(document);

    // The document has one page.
    assert(FPDF_GetPageCount(document) == 1);
    FPDF_PAGE page = FPDF_LoadPage(document, /*page_index=*/0);
    assert(page);

    int objectCount = FPDFPage_CountObject(page);
    std::cerr << "FPDFPage_CountObject() is " << objectCount << std::endl;

    FPDF_PAGEOBJECT pageObject = FPDFPage_GetObject(page, 0);
    assert(pageObject);

    FPDF_ClosePage(page);

    FPDF_CloseDocument(document);

    FPDF_DestroyLibrary();
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
