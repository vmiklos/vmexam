#include <cassert>
#include <fstream>
#include <iostream>
#include <iterator>
#include <vector>

#include <fpdf_edit.h>
#include <fpdfview.h>

#include "core/fpdfapi/page/cpdf_page.h"
#include "core/fpdfapi/page/cpdf_pageobject.h"
#include "core/fpdfapi/page/cpdf_pathobject.h"
#include "core/fpdfapi/parser/cpdf_dictionary.h"
#include "core/fpdfapi/parser/cpdf_object.h"

void testTdf106059()
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

    // Start of internal API.
    auto pageInternal = static_cast<CPDF_Page*>(page);
    CPDF_Object* resources = pageInternal->GetPageAttr("Resources");
    assert(resources);
    CPDF_Dictionary* xobject =
        resources->GetDict()->GetObjectFor("XObject")->AsDictionary();
    assert(xobject);
    // The page has one image.
    assert(xobject->GetCount() == 1);

    CPDF_Object* referenceXObject =
        xobject->GetObjectFor(xobject->begin()->first);
    assert(referenceXObject);
    // The image is a reference XObject.
    assert(referenceXObject->GetDict()->GetObjectFor("Ref"));
    // End of internal API.

    FPDF_ClosePage(page);

    FPDF_CloseDocument(document);

    FPDF_DestroyLibrary();
}

void testTdf105461()
{
    FPDF_LIBRARY_CONFIG config;
    config.version = 2;
    config.m_pUserFontPaths = nullptr;
    config.m_pIsolate = nullptr;
    config.m_v8EmbedderSlot = 0;
    FPDF_InitLibraryWithConfig(&config);

    std::ifstream testFile("tdf105461.pdf", std::ios::binary);
    std::vector<char> fileContents((std::istreambuf_iterator<char>(testFile)),
                                   std::istreambuf_iterator<char>());
    FPDF_DOCUMENT document = FPDF_LoadMemDocument(
        fileContents.data(), fileContents.size(), /*password=*/nullptr);
    assert(document);

    // The document has one page.
    assert(FPDF_GetPageCount(document) == 1);
    FPDF_PAGE page = FPDF_LoadPage(document, /*page_index=*/0);
    assert(page);

    int pageObjectCount = FPDFPage_CountObject(page);
    int yellowPathcount = 0;
    for (int i = 0; i < pageObjectCount; ++i)
    {
        FPDF_PAGEOBJECT pageObject = FPDFPage_GetObject(page, i);
        if (FPDFPageObj_GetType(pageObject) != FPDF_PAGEOBJ_PATH)
            continue;

        // Start of internal API.
        auto pageObjectInternal = static_cast<CPDF_PageObject*>(pageObject);
        if (pageObjectInternal->m_ColorState.GetFillRGB() != 0xffff)
            continue;
        // End of internal API.

        ++yellowPathcount;
    }
    assert(yellowPathcount == 1);

    FPDF_ClosePage(page);

    FPDF_CloseDocument(document);

    FPDF_DestroyLibrary();
}

int main()
{
    // testTdf106059();
    testTdf105461();
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
