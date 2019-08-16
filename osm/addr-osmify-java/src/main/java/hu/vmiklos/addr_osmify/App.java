/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

package hu.vmiklos.addr_osmify;

import com.google.gson.Gson;
import com.google.gson.annotations.SerializedName;
import com.google.gson.reflect.TypeToken;
import java.io.OutputStream;
import java.io.StringReader;
import java.io.StringWriter;
import java.lang.reflect.Type;
import java.net.HttpURLConnection;
import java.net.URL;
import java.net.URLEncoder;
import java.util.ArrayList;
import java.util.List;
import java.util.Collection;
import org.apache.commons.io.IOUtils;

public class App
{
    /**
     * NominatimResult represents one element in the result array from
     * Nominatim.
     */
    class NominatimResult
    {
        @SerializedName("class") public String clazz;
        public String lat;
        public String lon;
        @SerializedName("osm_type") public String osmType;
        @SerializedName("osm_id") public String osmId;
    }

    /**
     * TurboTags contains various tags about one Overpass element.
     */
    class TurboTags
    {
        @SerializedName("addr:city") public String city;
        @SerializedName("addr:housenumber") public String houseNumber;
        @SerializedName("addr:postcode") public String postCode;
        @SerializedName("addr:street") public String street;
    }

    /**
     * TurboElement represents one result from Overpass.
     */
    class TurboElement
    {
        public TurboTags tags;
    }

    /**
     * TurboResult is the result from Overpass.
     */
    class TurboResult
    {
        public List<TurboElement> elements;
    }

    /**
     * Send query to overpass turbo.
     */
    private static String queryTurbo(String query) throws Exception
    {
        StringReader reader = new StringReader(query);
        OutputStream outputStream = null;
        try
        {
            URL url = new URL("http://overpass-api.de/api/interpreter");
            HttpURLConnection connection =
                (HttpURLConnection)url.openConnection();
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

    /**
     * Send query to nominatim.
     */
    private static String queryNominatim(String query) throws Exception
    {
        String urlString = "https://nominatim.openstreetmap.org/search.php";
        urlString += "?q=" + URLEncoder.encode(query, "UTF-8");
        urlString += "&format=json";

        URL url = new URL(urlString);
        HttpURLConnection connection = (HttpURLConnection)url.openConnection();
        StringWriter writer = new StringWriter();
        IOUtils.copy(connection.getInputStream(), writer);
        return writer.toString();
    }

    /**
     * Turn query into a coordinate + address string.
     */
    private static void osmify(String query) throws Exception
    {
        // Use nominatim to get the coordinates and the osm type/id.
        String nominatim = queryNominatim(query);
        Gson gson = new Gson();
        Type collectionType =
            new TypeToken<Collection<NominatimResult>>() {}.getType();
        Collection<NominatimResult> elements =
            gson.fromJson(nominatim, collectionType);
        if (elements.isEmpty())
        {
            System.out.println("No results from nominatim");
            return;
        }

        if (elements.size() > 1)
        {
            // There are multiple elements, prefer buildings if possible.
            // Example where this is useful: 'Karinthy Frigyes út 18, Budapest'.
            ArrayList<NominatimResult> buildings =
                new ArrayList<NominatimResult>();
            for (NominatimResult element : elements)
            {
                if ("building".equals(element.clazz))
                {
                    buildings.add(element);
                }
            }

            if (!buildings.isEmpty())
            {
                elements = buildings;
            }
        }

        NominatimResult element = elements.iterator().next();
        String lat = element.lat;
        String lon = element.lon;
        String objectType = element.osmType;
        String objectId = element.osmId;

        // Use overpass to get the properties of the object.
        String overpassQuery = "[out:json];\n";
        overpassQuery += "(";
        overpassQuery += objectType + "(" + objectId + ");";
        overpassQuery += ");";
        overpassQuery += "out body;";
        String turbo = queryTurbo(overpassQuery);
        TurboResult turboResult = gson.fromJson(turbo, TurboResult.class);
        List<TurboElement> turboElements = turboResult.elements;
        if (turboElements.isEmpty())
        {
            System.out.println("No results from overpass");
            return;
        }

        TurboElement turboElement = turboElements.get(0);
        String city = turboElement.tags.city;
        String houseNumber = turboElement.tags.houseNumber;
        String postCode = turboElement.tags.postCode;
        String street = turboElement.tags.street;
        String addr = postCode + " " + city + ", " + street + " " + houseNumber;

        // Print the result.
        System.out.println("geo:" + lat + "," + lon + " (" + addr + ")");
    }

    private App()
    {
        // This is a utility class.
    }

    public static void main(String[] args)
    {
        if (args.length > 0)
        {
            try
            {
                osmify(args[0]);
            }
            catch (Exception e)
            {
                e.printStackTrace();
            }
        }
        else
        {
            System.out.println("usage: addr-osmify <query>");
            System.out.println();
            System.out.println(
                "e.g. addr-osmify 'Mészáros utca 58/a, Budapest'");
        }
    }
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
