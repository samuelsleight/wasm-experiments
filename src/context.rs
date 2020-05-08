use crate::renderer::Renderer;

use wasm_bindgen::{
    JsCast,
    prelude::*
};

use web_sys::{
    HtmlCanvasElement,
    WebGl2RenderingContext
};


#[wasm_bindgen]
pub struct Context {
    renderer: Renderer
}

#[wasm_bindgen]
impl Context {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Context, JsValue> {
        crate::utils::set_panic_hook();

        let canvas: HtmlCanvasElement = canvas()?;

        let context: WebGl2RenderingContext = canvas
            .get_context("webgl2")
            .map_err(|_| "Get context failed")?
            .ok_or("webgl context not supported")?
            .dyn_into()
            .map_err(|_| "webgl context conversion failed")?;

        Ok(Context{
            renderer: Renderer::new(context)?
        })
    }

    #[wasm_bindgen]
    pub fn resize_viewport(&self, width: u32, height: u32) {
        self.renderer.resize_viewport(width, height);
    }

    #[wasm_bindgen]
    pub fn render(&self, time: f32) {
        self.renderer.render(time);
    }
}

fn window() -> Result<web_sys::Window, String> {
    web_sys::window().ok_or("no global `window` exists".to_string())
}

fn document() -> Result<web_sys::Document, String> {
    window()?.document().ok_or("Window does not contain document".to_string())
}

fn canvas() -> Result<HtmlCanvasElement, String> {
    document()?
        .get_element_by_id("webgl")
        .ok_or("Canvas element does not exist".to_string())?
        .dyn_into()
        .map_err(|_| "Canvas element was not a canvas".into())
}

