#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]

use anyhow::Context as _;

#[derive(serde::Serialize)]
struct RubikResult {
    solution: String,
    error: String,
}

const TABLE: &[u8] = include_bytes!("../bin/table.bin");

pub fn our_app(request: &rouille::Request) -> anyhow::Result<String> {
    let facelet = request
        .get_param("facelet")
        .context("missing GET param: facelet")?;
    let face_cube = kewb::FaceCube::try_from(facelet.as_str())?;
    // Invoke my Kociemba facelet to cubie converter.
    let state = kewb::CubieCube::try_from(&face_cube)?;
    let table = kewb::fs::decode_table(TABLE)?;
    let max: u8 = 23;
    let timeout: Option<f32> = None;
    let mut solver = kewb::Solver::new(&table, max, timeout);
    // Invoke Luckas' Kociemba solver.
    let solution = solver.solve(state).context("no solution")?;
    Ok(solution.to_string())
}

pub fn app(request: &rouille::Request) -> rouille::Response {
    let result = match our_app(request) {
        Ok(solution) => RubikResult {
            solution,
            error: "".to_string(),
        },
        Err(err) => RubikResult {
            solution: "".to_string(),
            error: format!("{:?}", err),
        },
    };
    rouille::Response::json(&result)
}
