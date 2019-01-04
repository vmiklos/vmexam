extern crate reqwest;
extern crate serde_json;
extern crate url;

fn query_turbo(query: &str) -> String {
    let url = "http://overpass-api.de/api/interpreter";

    let client = reqwest::Client::new();
    let body = String::from(query);
    let mut buf = client.post(url)
        .body(body)
        .send().unwrap();
    return buf.text().unwrap();
}

fn query_nominatim(query: &str) -> String {
    let prefix = "http://nominatim.openstreetmap.org/search.php?";
    let encoded: String = url::form_urlencoded::Serializer::new(String::new())
        .append_pair("q", query)
        .append_pair("format", "json")
        .finish();
    let url = format!("{}{}", prefix, encoded);

    let mut buf = reqwest::get(url.as_str()).unwrap();

    return buf.text().unwrap();
}

fn osmify(query: &str) {
    let json: serde_json::Value = serde_json::from_str(&query_nominatim(query)).unwrap();
    let mut elements = json.as_array().unwrap().clone();
    if elements.is_empty() {
        println!("No results from nominatim");
        return;
    }

    if elements.len() > 1 {
        // There are multiple elements, prefer buildings if possible.
        let buildings: Vec<serde_json::Value> = elements.iter().filter(|i| {
            let i = i.as_object().unwrap();
            match i.get("class") {
                Some(value) => value.as_str().unwrap() == "building",
                None => false,
            }
        } ).map(|i| i.clone()).collect();

        if !buildings.is_empty() {
            elements = buildings;
        }
    }

    let element = elements[0].as_object().unwrap();
    let lat = element["lat"].as_str().unwrap();
    let lon = element["lon"].as_str().unwrap();
    let object_type = element["osm_type"].as_str().unwrap();
    let object_id = element["osm_id"].as_str().unwrap();

    // Use overpass to get the properties of the object.
    let overpass_query = format!(r#"[out:json];
(
    {}({});
);
out body;"#, object_type, object_id);
    let json: serde_json::Value = serde_json::from_str(&query_turbo(&overpass_query)).unwrap();
    let json = json.as_object().unwrap();
    let elements = &json["elements"].as_array().unwrap();
    if elements.is_empty() {
        println!("No results from overpass");
        return;
    }

    let element = &elements[0];
    let tags = element["tags"].as_object().unwrap();
    let city = tags["addr:city"].as_str().unwrap();
    let housenumber = tags["addr:housenumber"].as_str().unwrap();
    let postcode = tags["addr:postcode"].as_str().unwrap();
    let street = tags["addr:street"].as_str().unwrap();
    let addr = format!("{} {}, {} {}", postcode, city, street, housenumber);

    // Print the result.
    println!("geo:{},{} ({})", lat, lon, addr);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        osmify(&args[1]);
    } else {
        println!("usage: addr-osmify <query>");
        println!("");
        println!("e.g. addr-osmify 'Mészáros utca 58/a, Budapest'");
    }
}
