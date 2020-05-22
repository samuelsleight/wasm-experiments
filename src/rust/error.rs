use crate::{
    dom,
    webgl
};

macro_rules! impl_error {
    ($path:path) => {
        impl From<$path> for wasm_bindgen::JsValue {
            fn from(err: $path) -> wasm_bindgen::JsValue {
                js_sys::Error::from(err).into()
            }
        }

        impl From<$path> for js_sys::Error {
            fn from(err: $path) -> js_sys::Error {
                js_sys::Error::new(&format!("{}", err))
            }
        }
    };
}

impl_error!{dom::Error}
impl_error!{webgl::Error}
