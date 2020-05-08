use web_sys::{
    WebGlRenderingContext,
    WebGlShader,
    WebGlProgram
};

use boolinator::Boolinator;

fn compile_shader(context: &WebGlRenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| "Unable to create shader object".to_string())?;

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
        .ok_or_else(|| context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".to_string()))
        .map(|()| shader)
}

fn vertex_shader(context: &WebGlRenderingContext) -> Result<WebGlShader, String> {
    compile_shader(
        context,
        WebGlRenderingContext::VERTEX_SHADER,
        include_str!("vertex.glsl"))
}

fn fragment_shader(context: &WebGlRenderingContext) -> Result<WebGlShader, String> {
    compile_shader(
        context,
        WebGlRenderingContext::FRAGMENT_SHADER,
        include_str!("fragment.glsl"))
}

pub fn compile_and_link_program(context: &WebGlRenderingContext) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| "Unable to create shader program object".to_string())?;

    context.attach_shader(&program, &vertex_shader(context)?);
    context.attach_shader(&program, &fragment_shader(context)?);
    context.link_program(&program);

    context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
        .ok_or_else(|| context
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error creating program".to_string()))
        .map(|()| program)
}
