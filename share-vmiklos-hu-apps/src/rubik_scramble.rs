#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

use anyhow::Context as _;

#[derive(serde::Serialize)]
struct RubikResult {
    ok: String,
    error: String,
}

const TABLE: &[u8] = include_bytes!("../bin/table.bin");

pub fn our_app(request: &rouille::Request) -> anyhow::Result<String> {
    let lang = request
        .get_param("lang")
        .context("missing GET param: lang")?;
    let wide = request.get_param("wide").is_some();
    let megaminx = request.get_param("megaminx").is_some();

    if let Some(state) = request.get_param("state") {
        if state == "f2l-solved" || state == "oll-solved" {
            // Generate a scramble that allows practicing solving the last layer.
            let table = kewb::fs::decode_table(TABLE)?;
            let mut solver = kewb::Solver::new(&table, 25, None);
            let mut states = Vec::new();

            let state = if state == "f2l-solved" {
                kewb::generators::generate_state_f2l_solved()
            } else {
                kewb::generators::generate_state_oll_solved()
            };
            let scramble = kewb::scramble::scramble_from_state(state, &mut solver)?;

            states.push(state);
            solver.clear();

            let stringified = scramble
                .iter()
                .map(|m| m.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            return Ok(stringified);
        }
    }

    let mut colors: Vec<String> = Vec::new();
    if let Some(value) = request.get_param("colors") {
        colors = value.split(",").map(|i| i.to_string()).collect();
    };
    rubik::shuffle(&lang, wide, megaminx, &colors)
}

pub fn app(request: &rouille::Request) -> rouille::Response {
    let result = match our_app(request) {
        Ok(ok) => RubikResult {
            ok,
            error: "".to_string(),
        },
        Err(err) => RubikResult {
            ok: "".to_string(),
            error: format!("{err:?}"),
        },
    };
    rouille::Response::json(&result)
}
