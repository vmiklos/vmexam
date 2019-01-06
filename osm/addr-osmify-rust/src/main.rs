extern crate reqwest;
extern crate serde_json;
extern crate url;

fn query_turbo(query: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = "http://overpass-api.de/api/interpreter";

    let client = reqwest::Client::new();
    let body = String::from(query);
    let mut buf = client.post(url)
        .body(body)
        .send()?;
    match buf.text() {
        Ok(value) => Ok(value),
        Err(error) => Err(Box::new(error)),
    }
}

fn query_nominatim(query: &str) -> Result<String, Box<dyn std::error::Error>> {
    let prefix = "http://nominatim.openstreetmap.org/search.php?";
    let encoded: String = url::form_urlencoded::Serializer::new(String::new())
        .append_pair("q", query)
        .append_pair("format", "json")
        .finish();
    let url = format!("{}{}", prefix, encoded);

    let mut buf = reqwest::get(url.as_str())?;

    match buf.text() {
        Ok(value) => Ok(value),
        Err(error) => Err(Box::new(error)),
    }
}

fn osmify(query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let nominatim = query_nominatim(query)?;
    let json: serde_json::Value = serde_json::from_str(&nominatim)?;
    let mut elements = json.as_array().ok_or("option::NoneError")?.clone();
    if elements.is_empty() {
        println!("No results from nominatim");
        return Ok(());
    }

    if elements.len() > 1 {
        // There are multiple elements, prefer buildings if possible.
        let buildings: Vec<serde_json::Value> = elements.iter().filter(|i| {
            let i = match i.as_object() {
                Some(value) => value,
                None => return false,
            };
            let class = match i.get("class") {
                Some(value) => value.as_str(),
                None => return false,
            };
            match class {
                Some(value) => value == "building",
                None => false,
            }
        } ).map(|i| i.clone()).collect();

        if !buildings.is_empty() {
            elements = buildings;
        }
    }

    let element = elements[0].as_object().ok_or("option::NoneError")?;
    let lat = element["lat"].as_str().ok_or("option::NoneError")?;
    let lon = element["lon"].as_str().ok_or("option::NoneError")?;
    let object_type = element["osm_type"].as_str().ok_or("option::NoneError")?;
    let object_id = element["osm_id"].as_str().ok_or("option::NoneError")?;

    // Use overpass to get the properties of the object.
    let overpass_query = format!(r#"[out:json];
(
    {}({});
);
out body;"#, object_type, object_id);
    let turbo = query_turbo(&overpass_query)?;
    let json: serde_json::Value = serde_json::from_str(&turbo)?;
    let json = json.as_object().ok_or("option::NoneError")?;
    let elements = &json["elements"].as_array().ok_or("option::NoneError")?;
    if elements.is_empty() {
        println!("No results from overpass");
        return Ok(());
    }

    let element = &elements[0];
    let tags = element["tags"].as_object().ok_or("option::NoneError")?;
    let city = tags["addr:city"].as_str().ok_or("option::NoneError")?;
    let housenumber = tags["addr:housenumber"].as_str().ok_or("option::NoneError")?;
    let postcode = tags["addr:postcode"].as_str().ok_or("option::NoneError")?;
    let street = tags["addr:street"].as_str().ok_or("option::NoneError")?;
    let addr = format!("{} {}, {} {}", postcode, city, street, housenumber);

    // Print the result.
    println!("geo:{},{} ({})", lat, lon, addr);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        osmify(&args[1])?;
    } else {
        println!("usage: addr-osmify <query>");
        println!("");
        println!("e.g. addr-osmify 'Mészáros utca 58/a, Budapest'");
    }

    Ok(())
}
