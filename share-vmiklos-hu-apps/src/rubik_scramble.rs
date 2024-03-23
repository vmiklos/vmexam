#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

#[derive(serde::Serialize)]
struct RubikResult {
    ok: String,
    error: String,
}

pub fn app() -> rouille::Response {
    let result = match rubik::shuffle() {
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
