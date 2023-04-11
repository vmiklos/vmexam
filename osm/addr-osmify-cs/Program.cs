/*
 * Copyright 2020 Miklos Vajna
 *
 * SPDX-License-Identifier: MIT
 */

using System.Linq;
using System.Net;
using System.Text.Json.Serialization;
using System.Text.Json;
using System.Threading;
using System.Web;
using System;

namespace addr_osmify_cs
{
// NominatimResult represents one element in the result array from Nominatim.
class NominatimResult
{
    [JsonPropertyName("class")] public string Class
    {
        get;
        set;
    }
    [JsonPropertyName("lat")] public string Lat
    {
        get;
        set;
    }
    [JsonPropertyName("lon")] public string Lon
    {
        get;
        set;
    }
    [JsonPropertyName("osm_type")] public string OsmType
    {
        get;
        set;
    }
    [JsonPropertyName("osm_id")] public long OsmId
    {
        get;
        set;
    }
}

// TurboTags contains various tags about one Overpass element.
class TurboTags
{
    [JsonPropertyName("addr:city")] public string City
    {
        get;
        set;
    }
    [JsonPropertyName("addr:housenumber")] public string HouseNumber
    {
        get;
        set;
    }
    [JsonPropertyName("addr:postcode")] public string PostCode
    {
        get;
        set;
    }
    [JsonPropertyName("addr:street")] public string Street
    {
        get;
        set;
    }
}

// TurboElement represents one result from Overpass.
class TurboElement
{
    [JsonPropertyName("tags")] public TurboTags Tags
    {
        get;
        set;
    }
}

// TurboResult is the result from Overpass.
class TurboResult
{
    [JsonPropertyName("elements")] public TurboElement[] Elements
    {
        get;
        set;
    }
}

// Stores input / output of osmify().
class Context
{
  public
    string Input
    {
        get;
        set;
    }
  public
    string Out
    {
        get;
        set;
    }
}

class Program
{
    // Send query to nominatim.
    static string QueryNominatim(string query)
    {
        string url = "http://nominatim.openstreetmap.org/search.php?";
        url += String.Format("q={0}&format=json", HttpUtility.UrlEncode(query));
        var client = new WebClient();
        client.Headers.Add("user-agent", "Mozilla/4.0 (compatible; MSIE 6.0; " +
                                         "Windows NT 5.2; .NET CLR 1.0.3705;)");
        return client.DownloadString(url);
    }

    // Send query to overpass turbo.
    static string QueryTurbo(string query)
    {
        string url = "http://overpass-api.de/api/interpreter";
        var client = new WebClient();
        byte[] buf = client.UploadData(
            url, "POST", System.Text.Encoding.UTF8.GetBytes(query));
        return System.Text.Encoding.UTF8.GetString(buf);
    }

    // Turn query into a coordinate + address string.
    static string Osmify(string query)
    {
        // Use nominatim to get the coordinates and the osm type/id.
        var elements = JsonSerializer.Deserialize<NominatimResult[]>(
            QueryNominatim(query));
        if (elements.Length < 1)
        {
            return "No results from nominatim";
        }

        if (elements.Length > 1)
        {
            // There are multiple elements, prefer buildings if possible.
            // Example where this is useful: 'Karinthy Frigyes út 18, Budapest'.
            elements = elements.Where(element => element.Class == "building")
                           .ToArray();
        }

        NominatimResult element = elements[0];
        string lat = element.Lat;
        string lon = element.Lon;
        string objectType = element.OsmType;
        long objectId = element.OsmId;

        // Use overpass to get the properties of the object.
            string overpassQuery = String.Format(@"[out:json];
                (
                    {0}({1});
                );
                out body;", objectType, objectId);
            var turboResult = JsonSerializer.Deserialize<TurboResult>(QueryTurbo(overpassQuery));
            TurboElement[] turboElements = turboResult.Elements;
            if (turboElements.Length < 1)
            {
            return "No results from overpass";
            }

            TurboElement turboElement = turboElements[0];
            string city = turboElement.Tags.City;
            string housenumber = turboElement.Tags.HouseNumber;
            string postcode = turboElement.Tags.PostCode;
            string street = turboElement.Tags.Street;
            string addr = String.Format("{0} {1}, {2} {3}", postcode, city, street, housenumber);

            // Print the result.
            return String.Format("{0},{1} ({2})", lat, lon, addr);
    }

    // Invokes osmify() on a thread.
    static void Worker(object data)
    {
        var context = data as Context;
        if (context == null)
        {
            return;
        }

        context.Out = Osmify(context.Input);
    }

    // Shows a spinner while osmify() is in progress.
    static void Spinner(Context context, Thread thread)
    {
        char[] spinCharacters = "\\|/-".ToCharArray();
        int spinIndex = 0;
        while (true)
        {
            if (thread.Join(100))
            {
                Console.Write("\r");
                Console.WriteLine(context.Out);
                break;
            }

            Console.Write("\r [" + spinCharacters[spinIndex] + "] ");
            spinIndex = (spinIndex + 1) % spinCharacters.Length;
        }
    }

    static void Main(string[] args)
    {
        if (args.Length > 0)
        {
            var context = new Context();
            context.Input = args[0];
            var thread = new Thread(new ParameterizedThreadStart(Worker));
            thread.Start(context);
            Spinner(context, thread);
        }
        else
        {
            Console.WriteLine("usage: addr-osmify <query>");
            Console.WriteLine("");
            Console.WriteLine(
                "e.g. addr-osmify 'Mészáros utca 58/a, Budapest'");
        }
    }
}
} // namespace addr_osmify_cs

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
