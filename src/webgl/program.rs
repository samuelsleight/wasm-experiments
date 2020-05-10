use std::marker::PhantomData;

use super::error::{
    Error,
    Result
};

use web_sys::{
    WebGl2RenderingContext,
    WebGlProgram,
    WebGlShader,
};

use boolinator::Boolinator;

pub trait NeedsVertexShader {
    type WithVertexShader;
}

pub trait NeedsFragmentShader {
    type WithFragmentShader;
}

pub struct HasVertexShader;
pub struct HasFragmentShader;
pub struct HasBothShaders;

impl NeedsVertexShader for () {
    type WithVertexShader = HasVertexShader;
}

impl NeedsVertexShader for HasFragmentShader {
    type WithVertexShader = HasBothShaders;
}

impl NeedsFragmentShader for () {
    type WithFragmentShader = HasFragmentShader;
}

impl NeedsFragmentShader for HasVertexShader {
    type WithFragmentShader = HasBothShaders;
}

pub struct Program {
    context: WebGl2RenderingContext,
    program: WebGlProgram,
}

pub struct ProgramBuilder<Shaders> {
    program: Program,
    _phantom: PhantomData<Shaders>
}

impl<> ProgramBuilder<()> {
    pub fn new(context: WebGl2RenderingContext) -> Result<Self> {
        let program = context
            .create_program()
            .ok_or(Error::ProgramCreationFailure)?;

        Ok(ProgramBuilder {
            program: Program {
                context,
                program,
            },

            _phantom: PhantomData
        })
    }
}

impl<Shaders: NeedsVertexShader> ProgramBuilder<Shaders> {
    pub fn vertex_shader(self, src: &str) -> Result<ProgramBuilder<Shaders::WithVertexShader>> {
        self.program.context.attach_shader(
            &self.program.program,
            &compile_shader(&self.program.context, WebGl2RenderingContext::VERTEX_SHADER, src)?);

        Ok(ProgramBuilder {
            program: self.program,
            _phantom: PhantomData,
        })
    }
}

impl<Shaders: NeedsFragmentShader> ProgramBuilder<Shaders> {
    pub fn fragment_shader(self, src: &str) -> Result<ProgramBuilder<Shaders::WithFragmentShader>> {
        self.program.context.attach_shader(
            &self.program.program,
            &compile_shader(&self.program.context, WebGl2RenderingContext::FRAGMENT_SHADER, src)?);

        Ok(ProgramBuilder {
            program: self.program,
            _phantom: PhantomData,
        })
    }
}

impl<> ProgramBuilder<HasBothShaders> {
    pub fn link(self) -> Result<Program> {
        self.program.context.link_program(&self.program.program);

        self.program.context
            .get_program_parameter(&self.program.program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
            .ok_or_else(|| self.program.context
                .get_program_info_log(&self.program.program)
                .unwrap_or_else(|| "Unknown error linking shader program".to_string()))
            .map_err(Error::ProgramLinkFailure)
            .map(|()| self.program)
    }
}

impl Program {
    // Temporary - until all functionality is implemented
    pub fn into_program(self) -> WebGlProgram {
        self.program
    }
}

fn compile_shader(context: &WebGl2RenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader> {
    let shader = context
        .create_shader(shader_type)
        .ok_or(Error::ShaderCreationFailure)?;

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
        .ok_or_else(|| context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error compiling shader".to_string()))
        .map_err(Error::ShaderCompilationFailure)
        .map(|()| shader)
}
