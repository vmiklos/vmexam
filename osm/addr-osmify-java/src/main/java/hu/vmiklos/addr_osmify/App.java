/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

package hu.vmiklos.addr_osmify;

import com.google.gson.Gson;
import com.google.gson.reflect.TypeToken;
import java.lang.reflect.Type;
import java.net.URLEncoder;
import java.util.ArrayList;
import java.util.List;
import java.util.Collection;
import java.io.OutputStream;
import java.nio.charset.Charset;

public final class App
{
    public static Urlopener urlopener = new DefaultUrlopener();

    /**
     * Send query to overpass turbo.
     */
    private static String queryTurbo(String query) throws Exception
    {
        return App.urlopener.urlopen("http://overpass-api.de/api/interpreter",
                                     query);
    }

    /**
     * Send query to nominatim.
     */
    private static String queryNominatim(String query) throws Exception
    {
        String urlString = "https://nominatim.openstreetmap.org/search.php";
        urlString += "?q=" + URLEncoder.encode(query, "UTF-8");
        urlString += "&format=json";

        return App.urlopener.urlopen(urlString, "");
    }

    /**
     * Turn query into a coordinate + address string.
     */
    public static String osmify(String query) throws Exception
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
            return "No results from nominatim";
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
            return "No results from overpass";
        }

        TurboElement turboElement = turboElements.get(0);
        String city = turboElement.tags.city;
        String houseNumber = turboElement.tags.houseNumber;
        String postCode = turboElement.tags.postCode;
        String street = turboElement.tags.street;
        String addr = postCode + " " + city + ", " + street + " " + houseNumber;

        // Print the result.
        return "geo:" + lat + "," + lon + " (" + addr + ")";
    }

    /**
     * Shows a spinner while osmify() is in progress.
     */
    private void spinner(Context context, Thread thread, OutputStream out)
        throws Exception
    {
        char[] spinCharacters = "\\|/-".toCharArray();
        int spinIndex = 0;
        while (true)
        {
            thread.join(100, 0);
            if (!thread.isAlive())
            {
                System.err.print("\r");
                System.err.flush();
                out.write(
                    (context.out + "\n").getBytes(Charset.forName("UTF-8")));
                break;
            }

            System.err.print("\r [" + spinCharacters[spinIndex] + "] ");
            System.err.flush();
            spinIndex = (spinIndex + 1) % spinCharacters.length;
        }
    }

    public App(String[] args, OutputStream out) throws Exception
    {
        if (args.length > 0)
        {
            Context context = new Context();
            context.in = args[0];
            Thread thread = new Thread(new Worker(context));
            thread.start();
            spinner(context, thread, out);
        }
        else
        {
            System.out.println("usage: addr-osmify <query>");
            System.out.println();
            System.out.println(
                "e.g. addr-osmify 'Mészáros utca 58/a, Budapest'");
        }
    }

    public static void main(String[] args)
    {
        try
        {
            new App(args, System.out);
        }
        catch (Exception e)
        {
            e.printStackTrace();
        }
    }
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
