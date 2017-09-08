#include <cassert>
#include <fstream>
#include <iostream>
#include <iterator>
#include <vector>

#include <fpdf_edit.h>
#include <fpdf_save.h>
#include <fpdfview.h>

#if 0
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
#endif

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

        unsigned int red, green, blue, alpha;
        FPDFPath_GetFillColor(pageObject, &red, &green, &blue, &alpha);
        if (((red << 16) | (green << 8) | blue) != 0xffff00)
            continue;

        ++yellowPathcount;
    }
    assert(yellowPathcount == 1);

    FPDF_ClosePage(page);

    FPDF_CloseDocument(document);

    FPDF_DestroyLibrary();
}

#if 0
static std::string buf;

static int WriteBlockCallback(FPDF_FILEWRITE* /*pFileWrite*/, const void* data,
                              unsigned long size)
{
    buf.append(static_cast<const char*>(data), size);
    return 1;
}

void testRoundtrip()
{
    // Demo code to show how to remove all but the first page of a document and
    // save it.
    FPDF_LIBRARY_CONFIG config;
    config.version = 2;
    config.m_pUserFontPaths = nullptr;
    config.m_pIsolate = nullptr;
    config.m_v8EmbedderSlot = 0;
    FPDF_InitLibraryWithConfig(&config);

    std::ifstream testFile("test.pdf", std::ios::binary);
    std::vector<char> fileContents((std::istreambuf_iterator<char>(testFile)),
                                   std::istreambuf_iterator<char>());
    FPDF_DOCUMENT document = FPDF_LoadMemDocument(
        fileContents.data(), fileContents.size(), /*password=*/nullptr);
    assert(document);

#if 0
    for (int i = FPDF_GetPageCount(document) - 1; i > 0; --i)
    {
        FPDFPage_Delete(document, i);
    }

    FPDF_PAGE page = FPDF_LoadPage(document, /*page_index=*/0);
    assert(page);
    //FPDFPage_GenerateContent(page);
#endif

    FPDF_FILEWRITE fileWrite;
    fileWrite.version = 1;
    fileWrite.WriteBlock = WriteBlockCallback;
    assert(FPDF_SaveWithVersion(document, &fileWrite, 0, 14));
    std::ofstream testOutFile("test.out.pdf", std::ios::binary);
    std::copy(buf.begin(), buf.end(),
              std::ostreambuf_iterator<char>(testOutFile));

    FPDF_CloseDocument(document);

    FPDF_DestroyLibrary();
}
#endif

int main()
{
#if 0
    testTdf106059();
#endif
    testTdf105461();
#if 0
    testRoundtrip();
#endif
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
