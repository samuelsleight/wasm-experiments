mod utils;
mod shaders;

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

    let program = shaders::compile_and_link_program(&context)?;
    context.use_program(Some(&program));

    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(0);

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(
        WebGlRenderingContext::TRIANGLES,
        0,
        (vertices.len() / 3) as i32,
    );

    Ok(())
}
