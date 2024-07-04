#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

use anyhow::Context as _;

#[derive(serde::Serialize)]
struct RubikResult {
    ok: String,
    error: String,
}

pub fn our_app(request: &rouille::Request) -> anyhow::Result<String> {
    let lang = request
        .get_param("lang")
        .context("missing GET param: lang")?;
    let wide = request.get_param("wide").is_some();
    rubik::shuffle(&lang, wide)
}

pub fn app(request: &rouille::Request) -> rouille::Response {
    let result = match our_app(request) {
        Ok(ok) => RubikResult {
            ok,
            error: "".to_string(),
        },
        Err(err) => RubikResult {
            ok: "".to_string(),
            error: format!("{:?}", err),
        },
    };
    rouille::Response::json(&result)
}
