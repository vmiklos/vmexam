#[derive(serde::Serialize)]
struct HyphenResult {
    hyphenated: String,
    error: String,
}

pub fn app(request: &rouille::Request) -> rouille::Response {
    let word = match request.get_param("word") {
        Some(value) => value,
        None => {
            return rouille::Response::json(&HyphenResult {
                hyphenated: "".to_owned(),
                error: "missing GET param: word".to_owned(),
            });
        }
    };
    let dict = match hyphen::HyphenDict::new("/usr/share/hyphen/hyph_hu_HU.dic") {
        Ok(value) => value,
        Err(err) => {
            return rouille::Response::json(&HyphenResult {
                hyphenated: "".to_owned(),
                error: format!("failed to load the dict: {:?}", err),
            });
        }
    };
    let hyphenated = match dict.hyphenate(&word) {
        Ok(value) => value,
        Err(err) => {
            return rouille::Response::json(&HyphenResult {
                hyphenated: "".to_owned(),
                error: format!("failed to hyphenate the word: {:?}", err),
            });
        }
    };
    let response = rouille::Response::json(&HyphenResult {
        hyphenated,
        error: "".to_owned(),
    });
    response
}
