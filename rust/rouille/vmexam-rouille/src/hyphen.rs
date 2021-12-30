#[derive(serde::Serialize)]
struct HyphenResult {
    hyphenated: String,
    error: String,
}

pub fn app(_request: &rouille::Request) -> rouille::Response {
    let response = rouille::Response::json(&HyphenResult {
        hyphenated: "asz=szony=nyal".to_owned(),
        error: "".to_owned(),
    });
    response
}
