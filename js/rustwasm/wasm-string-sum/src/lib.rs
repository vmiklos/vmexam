use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sum_as_string(a: usize, b: usize) -> String {
    (a + b).to_string()
}
