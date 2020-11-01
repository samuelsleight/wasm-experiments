use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type WorldgenCallback;

    #[wasm_bindgen(method)]
    pub async fn does_this_work(this: &WorldgenCallback, num: f64) -> JsValue;
}

#[wasm_bindgen]
pub async fn does_this_extra_work(num: f64) -> f64 {
    num + 7.0
}
