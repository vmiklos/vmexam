/*
 * Copyright 2019 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

package hu.vmiklos.addr_osmify;

import java.io.OutputStream;
import java.io.StringReader;
import java.io.StringWriter;
import java.net.HttpURLConnection;
import java.net.URL;
import org.apache.commons.io.IOUtils;

/**
 * Default urlopen(), using HttpURLConnection.
 */
public class DefaultUrlopener implements Urlopener
{
    public String urlopen(String urlString, String data) throws Exception
    {
        URL url = new URL(urlString);
        HttpURLConnection connection = (HttpURLConnection)url.openConnection();

        if (data.isEmpty())
        {
            StringWriter writer = new StringWriter();
            IOUtils.copy(connection.getInputStream(), writer);
            return writer.toString();
        }

        StringReader reader = new StringReader(data);
        OutputStream outputStream = null;
        try
        {
            connection.setRequestMethod("POST");
            connection.setDoOutput(true);
            outputStream = connection.getOutputStream();
            IOUtils.copy(reader, outputStream);

            StringWriter writer = new StringWriter();
            IOUtils.copy(connection.getInputStream(), writer);
            return writer.toString();
        }
        catch (Exception e)
        {
            throw e;
        }
        finally
        {
            reader.close();
            if (outputStream != null)
            {
                outputStream.close();
            }
        }
    }
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
