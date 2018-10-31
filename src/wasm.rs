use super::Polygon;
use wasm_bindgen::JsValue;

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test_poly_poly(a: String, b: String) -> JsValue
{
    let poly_a: Polygon = serde_json::from_str(&a).unwrap();
    let poly_b: Polygon = serde_json::from_str(&b).unwrap();

    let response = poly_a.test(&poly_b);

    JsValue::from_serde(&response).unwrap()
}
