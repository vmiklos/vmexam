/*
 * Copyright 2019 Miklos Vajna. All rights reserved.
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

#import <Foundation/Foundation.h>

NSData* queryTurbo(NSString* query)
{
    NSString* url = @"http://overpass-api.de/api/interpreter";

    NSMutableURLRequest* urlRequest = [[NSMutableURLRequest alloc] initWithURL:[NSURL URLWithString:url]];
    [urlRequest setHTTPMethod:@"POST"];
    NSData* data = [query dataUsingEncoding:NSUTF8StringEncoding];
    [urlRequest setHTTPBody:data];
    CFRunLoopRef runloop = CFRunLoopGetCurrent();
    NSURLSession* session = [NSURLSession sharedSession];
    __block NSData* ret = nil;
    NSURLSessionDataTask* dataTask = [session dataTaskWithRequest:urlRequest completionHandler:^(NSData* data, NSURLResponse* response, NSError* error) {
        NSHTTPURLResponse* httpResponse = (NSHTTPURLResponse *)response;
        if (!error && httpResponse.statusCode == 200)
        {
            ret = data;
        }
        CFRunLoopStop(runloop);
    }];
    [dataTask resume];
    CFRunLoopRun();

    return ret;
}
NSData* queryNominatim(NSString* query)
{
    NSString* urlPrefix = @"http://nominatim.openstreetmap.org/search.php?";
    NSString* queryEncoded = [query stringByAddingPercentEncodingWithAllowedCharacters:[NSCharacterSet URLHostAllowedCharacterSet]];
    NSString* url = [urlPrefix stringByAppendingFormat:@"q=%@&format=json", queryEncoded];

    NSMutableURLRequest* urlRequest = [[NSMutableURLRequest alloc] initWithURL:[NSURL URLWithString:url]];
    [urlRequest setHTTPMethod:@"GET"];
    CFRunLoopRef runloop = CFRunLoopGetCurrent();
    NSURLSession* session = [NSURLSession sharedSession];
    __block NSData* ret = nil;
    NSURLSessionDataTask* dataTask = [session dataTaskWithRequest:urlRequest completionHandler:^(NSData* data, NSURLResponse* response, NSError* error) {
        NSHTTPURLResponse* httpResponse = (NSHTTPURLResponse *)response;
        if (!error && httpResponse.statusCode == 200)
        {
            ret = data;
        }
        CFRunLoopStop(runloop);
    }];
    [dataTask resume];
    CFRunLoopRun();

    return ret;
}

void osmify(NSString* query)
{
    // Use nominatim to get the coordinates and the osm type/id.
    NSError* parseError = nil;
    NSData* data = queryNominatim(query);
    if (!data)
    {
        printf("No response from nominatim.\n");
        return;
    }

    NSArray* elements = [NSJSONSerialization JSONObjectWithData:data options:NSJSONReadingMutableContainers error:&parseError];
    if ([elements count] < 1)
    {
        printf("No results from nominatim.\n");
        return;
    }

    if ([elements count] > 1)
    {
        // There are multiple elements, prefer buildings if possible.
        // TODO
    }
    NSDictionary* element = elements[0];
    NSString* lat = [element objectForKey:@"lat"];
    NSString* lon = [element objectForKey:@"lon"];
    NSString* objectType = [element objectForKey:@"osm_type"];
    NSString* objectId = [element objectForKey:@"osm_id"];

    // Use overpass to get the properties of the object.
    NSString* overpassQuery = [NSString stringWithFormat:@"[out:json];\n"
        "(\n"
        "%@(%@);\n"
        ");\n"
        "out body;", objectType, objectId];
    data = queryTurbo(overpassQuery);
    if (!data)
    {
        printf("No response from overpass.\n");
        return;
    }

    NSDictionary* j = [NSJSONSerialization JSONObjectWithData:data options:0 error:&parseError];
    elements = [j objectForKey:@"elements"];
    if ([elements count] < 1) {
        printf("No results from overpass.\n");
        return;
    }

    element = elements[0];
    NSString* city = [[element objectForKey:@"tags"] objectForKey:@"addr:city"];
    NSString* housenumber = [[element objectForKey:@"tags"] objectForKey:@"addr:housenumber"];
    NSString* postcode = [[element objectForKey:@"tags"] objectForKey:@"addr:postcode"];
    NSString* street = [[element objectForKey:@"tags"] objectForKey:@"addr:street"];
    NSString* addr = [NSString stringWithFormat:@"%@ %@, %@ %@", postcode, city, street, housenumber];

    // Print the result.
    printf("%s\n", [[NSString stringWithFormat:@"geo:%@,%@ (%@)", lat, lon, addr] UTF8String]);
}

int main(int argc, const char* argv[])
{
    @autoreleasepool
    {
        if (argc >= 2)
        {
            osmify([[NSString alloc] initWithUTF8String:argv[1]]);
        }
        else
        {
            printf("%s\n", [@"usage: addr-osmify <query>" cStringUsingEncoding:NSUTF8StringEncoding]);
            printf("\n");
            printf("%s\n", [@"e.g. addr-osmify 'Mészáros utca 58/a, Budapest'" cStringUsingEncoding:NSUTF8StringEncoding]);
        }
    }
    return 0;
}

/* vim:set shiftwidth=4 softtabstop=4 expandtab: */
