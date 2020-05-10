use crate::dom;

impl From<dom::Error> for wasm_bindgen::JsValue {
    fn from(err: dom::Error) -> wasm_bindgen::JsValue {
        js_sys::Error::from(err).into()
    }
}

impl From<dom::Error> for js_sys::Error {
    fn from(err: dom::Error) -> js_sys::Error {
        js_sys::Error::new(&format!("{}", err))
    }
}
