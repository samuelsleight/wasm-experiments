mod utils;
mod shaders;
mod renderer;

use std::{
    cell::RefCell,
    rc::Rc
};

use wasm_bindgen::{
    JsCast,
    prelude::*
};

use web_sys::{
    HtmlCanvasElement,
    WebGl2RenderingContext
};

fn window() -> Result<web_sys::Window, String> {
    web_sys::window().ok_or("no global `window` exists".to_string())
}

fn request_animation_frame(f: &Closure<dyn FnMut(f32)>) -> Result<(), String> {
    window()?
        .request_animation_frame(f.as_ref().unchecked_ref())
        .map_err(|_| "failed registering animation frame".to_string())
        .map(|_| ())
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    utils::set_panic_hook();

    let document = window()?.document().ok_or("Window does not contain document")?;
    let canvas = document.get_element_by_id("webgl").ok_or("Canvas does not exist")?;
    let canvas: HtmlCanvasElement = canvas.dyn_into().map_err(|_| "Canvas was not canvas")?;

    let context: WebGl2RenderingContext = canvas
        .get_context("webgl2")
        .map_err(|_| "Get context failed")?
        .ok_or("webgl context not supported")?
        .dyn_into()
        .map_err(|_| "webgl context conversion failed")?;

    let renderer = renderer::Renderer::new(context)?;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f32| {
        renderer.render(time);
        request_animation_frame(f.borrow().as_ref().unwrap()).unwrap();
    }) as Box<dyn FnMut(f32)>));

    let result = request_animation_frame(g.borrow().as_ref().unwrap()).map_err(|ref s| JsValue::from_str(s));
    result
}
