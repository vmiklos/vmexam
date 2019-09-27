/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

package hu.vmiklos.addr_osmify;

import java.net.URLEncoder;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.nio.charset.StandardCharsets;
import java.nio.charset.Charset;

/**
 * Test urlopen(), using mocking.
 */
public class MockUrlopener implements Urlopener
{
    String urlopenSuffix;

    static String readFile(String path, Charset encoding) throws Exception
    {
        byte[] encoded = Files.readAllBytes(Paths.get(path));
        return new String(encoded, encoding);
    }

    public String urlopen(String url, String data) throws Exception
    {
        if (!data.isEmpty())
        {
            String path = URLEncoder.encode(url, "UTF-8");
            path = "mock/" + path + urlopenSuffix + ".expected-data";
            String content = readFile(path, StandardCharsets.UTF_8);
            if (!data.equals(content))
            {
                throw new IllegalArgumentException(
                    "data vs content mismatch: data is '" + data +
                    "', content is '" + content + "'");
            }
        }

        String path = URLEncoder.encode(url, "UTF-8");
        path = "mock/" + path + urlopenSuffix;
        return readFile(path, StandardCharsets.UTF_8);
    }

    MockUrlopener(String urlopenSuffix) { this.urlopenSuffix = urlopenSuffix; }
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
