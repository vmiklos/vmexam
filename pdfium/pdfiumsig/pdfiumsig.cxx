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

#include <cert.h>
#include <cms.h>
#include <nss.h>
#include <sechash.h>

#include "core/fpdfapi/parser/cpdf_array.h"
#include "core/fpdfapi/parser/cpdf_dictionary.h"
#include "fpdfsdk/cpdfsdk_helpers.h"

namespace std
{
template <> struct default_delete<HASHContext>
{
    void operator()(HASHContext* ptr) { HASH_Destroy(ptr); }
};
template <> struct default_delete<CERTCertificate>
{
    void operator()(CERTCertificate* ptr) { CERT_DestroyCertificate(ptr); }
};
} // namespace std

class CryptoNss
{
  public:
    CryptoNss();
    ~CryptoNss();
};

CryptoNss::CryptoNss()
{
    SECStatus ret = NSS_NoDB_Init(nullptr);
    if (ret != SECSuccess)
    {
        std::cerr << "warning, NSS_NoDB_Init() failed" << std::endl;
    }
}

CryptoNss::~CryptoNss()
{
    SECStatus ret = NSS_Shutdown();
    if (ret != SECSuccess)
    {
        std::cerr << "warning, NSS_Shutdown() failed" << std::endl;
    }
}

struct ByteRange
{
    size_t offset;
    size_t length;
};

/**
 * Validates if `signature` is a proper signature of `bytes`. This only focuses
 * on if the digest matches or not, ignoring cert validation. Not specific to
 * PDF in any way.
 *
 * The flow is: message -> contentInfo -> signedData -> signerInfo
 */
void validateBytes(const std::vector<unsigned char>& bytes,
                   const std::vector<unsigned char>& signature)
{
    SECItem signatureItem;
    signatureItem.data = const_cast<unsigned char*>(signature.data());
    signatureItem.len = signature.size();

    NSSCMSMessage* message =
        NSS_CMSMessage_CreateFromDER(&signatureItem,
                                     /*cb=*/nullptr,
                                     /*cb_arg=*/nullptr,
                                     /*pwfn=*/nullptr,
                                     /*pwfn_arg=*/nullptr,
                                     /*decrypt_key_cb=*/nullptr,
                                     /*decrypt_key_cb_arg=*/nullptr);
    if (!NSS_CMSMessage_IsSigned(message))
    {
        std::cerr << "warning, NSS_CMSMessage_IsSigned() failed" << std::endl;
        return;
    }

    NSSCMSContentInfo* contentInfo =
        NSS_CMSMessage_ContentLevel(message, /*n=*/0);
    if (!contentInfo)
    {
        std::cerr << "warning, NSS_CMSMessage_ContentLevel() failed"
                  << std::endl;
        return;
    }

    auto signedData = static_cast<NSSCMSSignedData*>(
        NSS_CMSContentInfo_GetContent(contentInfo));
    if (!signedData)
    {
        std::cerr << "warning, NSS_CMSContentInfo_GetContent() failed"
                  << std::endl;
        return;
    }

    std::vector<std::unique_ptr<CERTCertificate>> messageCertificates;
    for (size_t i = 0; signedData->rawCerts[i]; ++i)
    {
        std::unique_ptr<CERTCertificate> certificate(CERT_NewTempCertificate(
            CERT_GetDefaultCertDB(), signedData->rawCerts[i],
            /*nickname=*/nullptr, /*isperm=*/0, /*copyDER=*/0));
        messageCertificates.push_back(std::move(certificate));
    }

    SECItem algItem = NSS_CMSSignedData_GetDigestAlgs(signedData)[0]->algorithm;
    SECOidTag algOid = SECOID_FindOIDTag(&algItem);
    HASH_HashType hashType = HASH_GetHashTypeByOidTag(algOid);
    std::unique_ptr<HASHContext> hashContext(HASH_Create(hashType));
    if (!hashContext)
    {
        std::cerr << "warning, HASH_Create() failed" << std::endl;
        return;
    }

    HASH_Update(hashContext.get(), bytes.data(), bytes.size());

    size_t hashLength = 0;
    switch (algOid)
    {
    case SEC_OID_SHA1:
        hashLength = 20;
        break;
    default:
        std::cerr << "warning, unhandled hash alg" << std::endl;
        return;
    }

    NSSCMSSignerInfo* signerInfo =
        NSS_CMSSignedData_GetSignerInfo(signedData, 0);
    if (!signerInfo)
    {
        std::cerr << "warning, NSS_CMSSignedData_GetSignerInfo() failed"
                  << std::endl;
        return;
    }

    std::vector<unsigned char> hash(hashLength);
    unsigned int actualHashLength;
    HASH_End(hashContext.get(), hash.data(), &actualHashLength, hashLength);
    // Need to call this manually, so that signerinfo->cert gets set. Otherwise
    // NSS_CMSSignerInfo_Verify() will call
    // NSS_CMSSignerInfo_GetSigningCertificate() with certdb=nullptr, which
    // won't find the certificate.
    std::unique_ptr<CERTCertificate> certificate(
        NSS_CMSSignerInfo_GetSigningCertificate(signerInfo,
                                                CERT_GetDefaultCertDB()));

    SECItem hashItem;
    hashItem.data = hash.data();
    hashItem.len = actualHashLength;
    SECStatus ret = NSS_CMSSignerInfo_Verify(signerInfo, &hashItem,
                                             /*contentType=*/nullptr);
    if (ret != SECSuccess)
    {
        std::cerr << "  - Signature Verification: digest does not match"
                  << std::endl;
        return;
    }
    std::cerr << "  - Signature Verification: digest matches" << std::endl;
}

void validateByteRanges(const std::vector<unsigned char>& bytes,
                        const std::vector<ByteRange>& byteRanges,
                        const std::vector<unsigned char>& signature)
{
    CryptoNss crypto;
    std::vector<unsigned char> buffer;
    for (const auto& byteRange : byteRanges)
    {
        size_t bufferSize = buffer.size();
        buffer.resize(bufferSize + byteRange.length);
        memcpy(buffer.data() + bufferSize, bytes.data() + byteRange.offset,
               byteRange.length);
    }

    validateBytes(buffer, signature);
}

void validateSignature(const std::vector<unsigned char>& bytes,
                       FPDF_SIGNATURE signature, int signatureIndex)
{
    std::cerr << "Signature #" << signatureIndex << ":" << std::endl;
    int contentsLen = FPDFSignatureObj_GetContents(signature, nullptr, 0);
    std::vector<unsigned char> contents(contentsLen);
    FPDFSignatureObj_GetContents(signature, contents.data(), contents.size());

    CPDF_Dictionary* signatureDict = CPDFDictionaryFromFPDFSignature(signature);
    CPDF_Dictionary* valueDict = signatureDict->GetDictFor("V");
    const CPDF_Array* byteRangeArray = valueDict->GetArrayFor("ByteRange");

    std::vector<ByteRange> byteRanges;
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
        byteRanges.push_back({byteRangeOffset, byteRangeLength});
    }

    // Sanity checks.
    if (byteRanges.size() < 2)
    {
        std::cerr << "warning, expected 2 byte ranges" << std::endl;
        return;
    }

    if (byteRanges[0].offset != 0)
    {
        std::cerr << "warning, first range start is not 0" << std::endl;
        return;
    }

    // Binary vs hex dump and 2 is the leading "<" and the trailing ">" around
    // the hex string.
    size_t signatureLength = contents.size() * 2 + 2;
    if (byteRanges[1].offset != (byteRanges[0].length + signatureLength))
    {
        std::cerr
            << "warning, second range start is not the end of the signature"
            << std::endl;
        return;
    }

    validateByteRanges(bytes, byteRanges, contents);
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
    std::vector<unsigned char> fileContents(
        (std::istreambuf_iterator<char>(testFile)),
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
