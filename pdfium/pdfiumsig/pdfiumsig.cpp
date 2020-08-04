// Copyright 2020 PDFium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include <cassert>
#include <codecvt>
#include <fstream>
#include <iostream>
#include <iterator>
#include <locale>
#include <vector>

#include <fpdf_signature.h>
#include <fpdfview.h>

#include "public/cpp/fpdf_scopers.h"

#include <cert.h>
#include <cms.h>
#include <nss.h>
#include <sechash.h>

struct HASHContextDeleter {
  void operator()(HASHContext* ptr) { HASH_Destroy(ptr); }
};

using ScopedHASHContext = std::unique_ptr<HASHContext, HASHContextDeleter>;

struct CERTCertificateDeleter {
  void operator()(CERTCertificate* ptr) { CERT_DestroyCertificate(ptr); }
};

using ScopedCERTCertificate =
    std::unique_ptr<CERTCertificate, CERTCertificateDeleter>;

class Crypto {
 public:
  enum class ValidationStatus {
    SUCCESS,
    FAILURE,
  };

  Crypto();
  ~Crypto();
  /**
   * Validates if `signature` is a proper signature of `bytes`. This only
   * focuses on if the digest matches or not, ignoring cert validation. Not
   * specific to PDF in any way.
   *
   * The flow is: message -> content_info -> signed_data -> signer_info
   */
  bool validateBytes(const std::vector<unsigned char>& bytes,
                     const std::vector<unsigned char>& signature,
                     ValidationStatus& status);
};

Crypto::Crypto() {
  SECStatus ret = NSS_NoDB_Init(nullptr);
  if (ret != SECSuccess) {
    std::cerr << "warning, NSS_NoDB_Init() failed" << std::endl;
  }
}

Crypto::~Crypto() {
  SECStatus ret = NSS_Shutdown();
  if (ret != SECSuccess) {
    std::cerr << "warning, NSS_Shutdown() failed" << std::endl;
  }
}

bool Crypto::validateBytes(const std::vector<unsigned char>& bytes,
                           const std::vector<unsigned char>& signature,
                           ValidationStatus& status) {
  SECItem signature_item;
  signature_item.data = const_cast<unsigned char*>(signature.data());
  signature_item.len = signature.size();

  NSSCMSMessage* message =
      NSS_CMSMessage_CreateFromDER(&signature_item,
                                   /*cb=*/nullptr,
                                   /*cb_arg=*/nullptr,
                                   /*pwfn=*/nullptr,
                                   /*pwfn_arg=*/nullptr,
                                   /*decrypt_key_cb=*/nullptr,
                                   /*decrypt_key_cb_arg=*/nullptr);
  if (!NSS_CMSMessage_IsSigned(message)) {
    std::cerr << "warning, NSS_CMSMessage_IsSigned() failed" << std::endl;
    return false;
  }

  NSSCMSContentInfo* content_info =
      NSS_CMSMessage_ContentLevel(message, /*n=*/0);
  if (!content_info) {
    std::cerr << "warning, NSS_CMSMessage_ContentLevel() failed" << std::endl;
    return false;
  }

  auto signed_data = static_cast<NSSCMSSignedData*>(
      NSS_CMSContentInfo_GetContent(content_info));
  if (!signed_data) {
    std::cerr << "warning, NSS_CMSContentInfo_GetContent() failed" << std::endl;
    return false;
  }

  std::vector<ScopedCERTCertificate> message_certificates;
  for (size_t i = 0; signed_data->rawCerts[i]; ++i) {
    ScopedCERTCertificate certificate(CERT_NewTempCertificate(
        CERT_GetDefaultCertDB(), signed_data->rawCerts[i],
        /*nickname=*/nullptr, /*isperm=*/0, /*copyDER=*/0));
    message_certificates.push_back(std::move(certificate));
  }

  SECItem alg_item = NSS_CMSSignedData_GetDigestAlgs(signed_data)[0]->algorithm;
  SECOidTag alg_oid = SECOID_FindOIDTag(&alg_item);
  HASH_HashType hash_type = HASH_GetHashTypeByOidTag(alg_oid);
  ScopedHASHContext hash_context(HASH_Create(hash_type));
  if (!hash_context) {
    std::cerr << "warning, HASH_Create() failed" << std::endl;
    return false;
  }

  HASH_Update(hash_context.get(), bytes.data(), bytes.size());

  NSSCMSSignerInfo* signer_info =
      NSS_CMSSignedData_GetSignerInfo(signed_data, 0);
  if (!signer_info) {
    std::cerr << "warning, NSS_CMSSignedData_GetSignerInfo() failed"
              << std::endl;
    return false;
  }

  std::vector<unsigned char> hash(HASH_ResultLenContext(hash_context.get()));
  unsigned int actual_hash_length;
  HASH_End(hash_context.get(), hash.data(), &actual_hash_length,
           HASH_ResultLenContext(hash_context.get()));
  // Need to call this manually, so that signerinfo->cert gets set. Otherwise
  // NSS_CMSSignerInfo_Verify() will call
  // NSS_CMSSignerInfo_GetSigningCertificate() with certdb=nullptr, which
  // won't find the certificate.
  ScopedCERTCertificate certificate(NSS_CMSSignerInfo_GetSigningCertificate(
      signer_info, CERT_GetDefaultCertDB()));

  SECItem hash_item;
  hash_item.data = hash.data();
  hash_item.len = actual_hash_length;
  SECStatus ret = NSS_CMSSignerInfo_Verify(signer_info, &hash_item,
                                           /*contentType=*/nullptr);
  if (ret != SECSuccess) {
    status = ValidationStatus::FAILURE;
    return true;
  }
  status = ValidationStatus::SUCCESS;
  return true;
}

struct ByteRange {
  size_t offset;
  size_t length;
};

void validateByteRanges(const std::vector<unsigned char>& bytes,
                        const std::vector<ByteRange>& byte_ranges,
                        const std::vector<unsigned char>& signature) {
  Crypto crypto;
  std::vector<unsigned char> buffer;
  for (const auto& byte_range : byte_ranges) {
    size_t buffer_size = buffer.size();
    buffer.resize(buffer_size + byte_range.length);
    memcpy(buffer.data() + buffer_size, bytes.data() + byte_range.offset,
           byte_range.length);
  }

  Crypto::ValidationStatus status{};
  if (!crypto.validateBytes(buffer, signature, status)) {
    std::cerr << "warning, failed to determine digest match" << std::endl;
    return;
  }

  if (status == Crypto::ValidationStatus::FAILURE) {
    std::cerr << "  - Signature Verification: digest does not match"
              << std::endl;
    return;
  }

  std::cerr << "  - Signature Verification: digest matches" << std::endl;
}

void validateSignature(const std::vector<unsigned char>& bytes,
                       FPDF_SIGNATURE signature,
                       int signature_index) {
  std::cerr << "Signature #" << signature_index << ":" << std::endl;
  int contents_len = FPDFSignatureObj_GetContents(signature, nullptr, 0);
  std::vector<unsigned char> contents(contents_len);
  FPDFSignatureObj_GetContents(signature, contents.data(), contents.size());

  int byte_range_len = FPDFSignatureObj_GetByteRange(signature, nullptr, 0);
  std::vector<int> byte_range(byte_range_len);
  FPDFSignatureObj_GetByteRange(signature, byte_range.data(),
                                byte_range.size());

  std::vector<ByteRange> byte_ranges;
  size_t byte_range_offset = 0;
  for (size_t i = 0; i < byte_range.size(); ++i) {
    if (i % 2 == 0) {
      byte_range_offset = byte_range[i];
      continue;
    }

    size_t byte_range_len = byte_range[i];
    byte_ranges.push_back({byte_range_offset, byte_range_len});
  }

  int sub_filter_len = FPDFSignatureObj_GetSubFilter(signature, nullptr, 0);
  std::vector<char> sub_filter_buf(sub_filter_len);
  FPDFSignatureObj_GetSubFilter(signature, sub_filter_buf.data(),
                                sub_filter_buf.size());
  // Buffer is NUL-terminated.
  std::string sub_filter(sub_filter_buf.data(), sub_filter_buf.size() - 1);

  // Sanity checks.
  if (sub_filter != "adbe.pkcs7.detached" &&
      sub_filter != "ETSI.CAdES.detached") {
    std::cerr << "warning, unexpected sub-filter: '" << sub_filter << "'"
              << std::endl;
    return;
  }

  if (byte_ranges.size() < 2) {
    std::cerr << "warning, expected 2 byte ranges" << std::endl;
    return;
  }

  if (byte_ranges[0].offset != 0) {
    std::cerr << "warning, first range start is not 0" << std::endl;
    return;
  }

  // Binary vs hex dump and 2 is the leading "<" and the trailing ">" around
  // the hex string.
  size_t signature_length = contents.size() * 2 + 2;
  if (byte_ranges[1].offset != (byte_ranges[0].length + signature_length)) {
    std::cerr << "warning, second range start is not the end of the signature"
              << std::endl;
    return;
  }

  int reason_len = FPDFSignatureObj_GetReason(signature, nullptr, 0);
  if (reason_len > 0) {
    std::vector<char16_t> reason_buf(reason_len);
    FPDFSignatureObj_GetReason(signature, reason_buf.data(), reason_buf.size());
    std::wstring_convert<std::codecvt_utf8_utf16<char16_t>, char16_t>
        conversion;
    std::string reason = conversion.to_bytes(reason_buf.data());
    std::cerr << "  - Signature Reason: " << reason << std::endl;
  }

  int time_len = FPDFSignatureObj_GetTime(signature, nullptr, 0);
  if (time_len > 0) {
    std::vector<char> time_buf(time_len);
    FPDFSignatureObj_GetTime(signature, time_buf.data(), time_buf.size());
    std::cerr << "  - Signature Time: " << time_buf.data() << std::endl;
  }

  validateByteRanges(bytes, byte_ranges, contents);
}

int main(int argc, char* argv[]) {
  FPDF_LIBRARY_CONFIG config;
  config.version = 2;
  config.m_pUserFontPaths = nullptr;
  config.m_pIsolate = nullptr;
  config.m_v8EmbedderSlot = 0;
  FPDF_InitLibraryWithConfig(&config);

  std::ifstream file_stream(argv[1], std::ios::binary);
  std::vector<unsigned char> file_contents(
      (std::istreambuf_iterator<char>(file_stream)),
      std::istreambuf_iterator<char>());
  {
    ScopedFPDFDocument document(FPDF_LoadMemDocument(
        file_contents.data(), file_contents.size(), /*password=*/nullptr));
    assert(document);

    int signature_count = FPDF_GetSignatureCount(document.get());
    for (int i = 0; i < signature_count; ++i) {
      FPDF_SIGNATURE signature = FPDF_GetSignatureObject(document.get(), i);
      validateSignature(file_contents, signature, i);
    }
  }

  FPDF_DestroyLibrary();
}
