/*
 * Copyright 2018 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

#include <iostream>

#include <xmlsec/transforms.h>
#include <xmlsec/xmlsec.h>

int main()
{
    // Test base64 encode of 'foo': expect 'Zm9v'.
    xmlSecTransformCtx transformCtx;
    if (xmlSecTransformCtxInitialize(&transformCtx) < 0)
    {
        std::cerr << "xmlSecTransformCtxInitialize() failed" << std::endl;
        return 1;
    }

    xmlSecTransformPtr base64Encode = xmlSecTransformCtxCreateAndAppend(
        &transformCtx, xmlSecTransformBase64Id);
    if (!base64Encode)
    {
        std::cerr << "xmlSecTransformCtxCreateAndAppend() failed" << std::endl;
        xmlSecTransformCtxFinalize(&transformCtx);
        return 1;
    }

    base64Encode->operation = xmlSecTransformOperationEncode;
    std::string in("foo");
    if (xmlSecTransformCtxBinaryExecute(
            &transformCtx, reinterpret_cast<const unsigned char*>(in.c_str()),
            in.size()) < 0)
    {
        std::cerr << "xmlSecTransformCtxCreateAndAppend() failed" << std::endl;
        xmlSecTransformCtxFinalize(&transformCtx);
        return 1;
    }

    std::cerr << "transform result is of " << transformCtx.result->size
              << " bytes" << std::endl;
    std::cerr << "transform result is '" << transformCtx.result->data << "'"
              << std::endl;

    xmlSecTransformCtxFinalize(&transformCtx);

    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
