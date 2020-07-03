#include <cassert>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <iterator>
#include <vector>

#include <fpdf_edit.h>
#include <fpdf_save.h>
#include <fpdf_signature.h>
#include <fpdfview.h>

#include "core/fpdfapi/parser/cpdf_array.h"
#include "core/fpdfapi/parser/cpdf_dictionary.h"
#include "fpdfsdk/cpdfsdk_helpers.h"

void validateBytes(const std::vector<char>& bytes,
                   const std::vector<std::pair<size_t, size_t>>& byteRanges,
                   const std::vector<char>& contents)
{
}

void validateSignature(const std::vector<char>& bytes, FPDF_SIGNATURE signature,
                       int signatureIndex)
{
    std::cerr << "Signature #" << signatureIndex << ":" << std::endl;
    int contentsLen = FPDFSignatureObj_GetContents(signature, nullptr, 0);
    std::vector<char> contents(contentsLen);
    FPDFSignatureObj_GetContents(signature, contents.data(), contents.size());

    CPDF_Dictionary* signatureDict = CPDFDictionaryFromFPDFSignature(signature);
    CPDF_Dictionary* valueDict = signatureDict->GetDictFor("V");
    const CPDF_Array* byteRangeArray = valueDict->GetArrayFor("ByteRange");
    ByteString subFilter = valueDict->GetNameFor("SubFilter");

    std::vector<std::pair<size_t, size_t>> byteRanges;
    size_t byteRangeOffset = 0;
    for (size_t i = 0; i < byteRangeArray->size(); ++i)
    {
        float number = byteRangeArray->GetNumberAt(i);
        if (i % 2 == 0)
        {
            byteRangeOffset = number;
            continue;
        }

        size_t byteRangeLength = number;
        byteRanges.emplace_back(byteRangeOffset, byteRangeLength);
    }

    // Sanity checks.
    if (byteRanges.size() < 2)
    {
        std::cerr << "warning, expected 2 byte ranges" << std::endl;
        return;
    }

    if (byteRanges[0].first != 0)
    {
        std::cerr << "warning, first range start is not 0" << std::endl;
        return;
    }

    // Binary vs hex dump and 2 is the leading "<" and the trailing ">" around
    // the hex string.
    size_t signatureLength = contents.size() * 2 + 2;
    if (byteRanges[1].first != (byteRanges[0].second + signatureLength))
    {
        std::cerr
            << "warning, second range start is not the end of the signature"
            << std::endl;
        return;
    }

    validateBytes(bytes, byteRanges, contents);
}

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
        FPDF_SIGNATURE signature = FPDF_GetSignatureObject(document, i);
        validateSignature(fileContents, signature, i);
    }

    FPDF_CloseDocument(document);

    FPDF_DestroyLibrary();
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
