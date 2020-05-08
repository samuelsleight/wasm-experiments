mod utils;
mod shaders;
mod renderer;

use wasm_bindgen::{
    JsCast,
    prelude::*
};

use web_sys::{
    HtmlCanvasElement,
    WebGlRenderingContext
};

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    utils::set_panic_hook();

    let document = web_sys::window().unwrap().document().ok_or("Window does not contain document")?;
    let canvas = document.get_element_by_id("webgl").ok_or("Canvas does not exist")?;
    let canvas: HtmlCanvasElement = canvas.dyn_into().map_err(|_| "Canvas was not canvas")?;

    let context: WebGlRenderingContext = canvas
        .get_context("webgl")
        .map_err(|_| "Get context failed")?
        .ok_or("webgl context not supported")?
        .dyn_into()
        .map_err(|_| "webgl context conversion failed")?;

    let renderer = renderer::Renderer::new(context)?;
    renderer.render();

    Ok(())
}
