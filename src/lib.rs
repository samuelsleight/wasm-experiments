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

use renderer::Renderer;

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

fn request_animation_frame(f: &Closure<dyn FnMut(f32)>) -> Result<(), String> {
    window()?
        .request_animation_frame(f.as_ref().unchecked_ref())
        .map_err(|_| "failed registering animation frame".to_string())
        .map(|_| ())
}

fn resize_canvas(renderer: &Renderer) -> Result<(), String> {
    let window = window().unwrap();
    let canvas = canvas().unwrap();

    let width = window.inner_width().map_err(|_| "Error getting window width".to_string())?.as_f64().unwrap() as u32;
    let height = window.inner_height().map_err(|_| "Error getting window height".to_string())?.as_f64().unwrap() as u32;

    canvas.set_width(width);
    canvas.set_height(height);
    renderer.resize_viewport(width, height);

    Ok(())
}

fn set_resize_callback(renderer: Rc<RefCell<Renderer>>) -> Result<(), String> {
    let closure = Closure::wrap(Box::new(move || {
        resize_canvas(&renderer.borrow()).unwrap();
    }) as Box<dyn FnMut()>);

    let window = window()?;
    window.set_onresize(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    Ok(())
}

fn start_animation_frame_callback(renderer: Rc<RefCell<Renderer>>) -> Result<(), String> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |time: f32| {
        renderer.borrow().render(time);
        request_animation_frame(f.borrow().as_ref().unwrap()).unwrap();
    }) as Box<dyn FnMut(f32)>));

    let result = request_animation_frame(g.borrow().as_ref().unwrap());
    result
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    utils::set_panic_hook();

    let canvas: HtmlCanvasElement = canvas()?;

    let context: WebGl2RenderingContext = canvas
        .get_context("webgl2")
        .map_err(|_| "Get context failed")?
        .ok_or("webgl context not supported")?
        .dyn_into()
        .map_err(|_| "webgl context conversion failed")?;

    let renderer = Rc::new(RefCell::new(Renderer::new(context)?));
    resize_canvas(&renderer.borrow())?;

    set_resize_callback(renderer.clone())?;
    start_animation_frame_callback(renderer.clone())?;

    Ok(())
}
