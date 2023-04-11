/*
 * Copyright 2019 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

package hu.vmiklos.addr_osmify;

import static org.junit.Assert.assertEquals;

import java.io.ByteArrayOutputStream;
import java.nio.charset.StandardCharsets;
import org.junit.Test;

public class AppTest
{
    @Test public void testHappy() throws Exception
    {
        MockUrlopen mu = new MockUrlopen("-happy");
        try
        {
            String[] args = {"Mészáros utca 58/a, Budapest"};
            ByteArrayOutputStream baos = new ByteArrayOutputStream();
            new App(args, baos);
            String out = new String(baos.toByteArray(), StandardCharsets.UTF_8);
            String expected =
                "47.490592,19.030662 (1016 Budapest, Mészáros utca 58/a)\n";
            assertEquals(expected, out);
        }
        catch (Exception e)
        {
            throw e;
        }
        finally
        {
            mu.destruct();
        }
    }
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
