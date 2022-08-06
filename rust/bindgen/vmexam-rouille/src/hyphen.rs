use anyhow::Context;

#[derive(serde::Serialize)]
struct HyphenResult {
    hyphenated: String,
    error: String,
}

pub fn our_app(request: &rouille::Request) -> anyhow::Result<String> {
    let word = request
        .get_param("word")
        .context("missing GET param: word")?
        .to_string();
    let dict = hyphen::HyphenDict::new("/usr/share/hyphen/hyph_hu_HU.dic")
        .context("failed to load the dictionary")?;
    Ok(dict.hyphenate(&word).context("failed to hyphenate")?)
}

pub fn app(request: &rouille::Request) -> rouille::Response {
    let result = match our_app(request) {
        Ok(hyphenated) => HyphenResult {
            hyphenated,
            error: "".to_string(),
        },
        Err(err) => HyphenResult {
            hyphenated: "".to_string(),
            error: format!("{:?}", err),
        },
    };
    rouille::Response::json(&result)
}
