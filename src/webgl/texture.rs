use super::{
    Colour,
    Error,
    Result
};

use web_sys::{
    WebGl2RenderingContext,
    WebGlTexture,
    WebGlProgram,
    WebGlUniformLocation
};

pub struct Texture {
    context: WebGl2RenderingContext,
    texture: WebGlTexture
}

pub struct Sampler {
    context: WebGl2RenderingContext,
    uniform: WebGlUniformLocation
}

pub struct BoundTexture<'a> {
    texture: &'a Texture
}

impl Texture {
    pub fn new(context: WebGl2RenderingContext) -> Result<Texture> {
        let texture = context.create_texture().ok_or(Error::TextureCreationFailure)?;

        let texture = Texture{
            context,
            texture
        };

        texture.with(
            |texture| {
                texture.texture.context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
                texture.texture.context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
            });

        Ok(texture)
    }

    pub fn update<T: AsRef<[Colour]>>(&self, width: usize, data: T) -> Result<()> {
        self.with(|texture| texture.update(width, data))
    }

    pub fn with<U, F: FnOnce(BoundTexture) -> U>(&self, f: F) -> U {
        self.context.active_texture(WebGl2RenderingContext::TEXTURE0);

        self.context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.texture));
        let result = f(BoundTexture { texture: &self });
        self.context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);
        result
    }
}

impl Sampler {
    pub fn new(context: WebGl2RenderingContext, program: &WebGlProgram, location: &str) -> Result<Sampler> {
        let uniform = context.get_uniform_location(program, location).ok_or(Error::SamplerCreationFailure)?;

        Ok(Sampler {
            context,
            uniform
        })
    }

    pub fn update(&self, _: &BoundTexture<'_>) {
        self.context.uniform1i(Some(&self.uniform), 0);
    }
}

impl<'a> BoundTexture<'a> {
    pub fn update<T: AsRef<[Colour]>>(&self, width: usize, data: T) -> Result<()> {
        let data = data.as_ref();

        let height = data.len() / width;

        unsafe {
            let u8_slice = std::slice::from_raw_parts(&data[0].r, data.len() * 4);

            self.texture.context
                .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                    WebGl2RenderingContext::TEXTURE_2D,
                    0,
                    WebGl2RenderingContext::RGBA as i32,
                    width as i32,
                    height as i32,
                    0,
                    WebGl2RenderingContext::RGBA,
                    WebGl2RenderingContext::UNSIGNED_BYTE,
                    Some(u8_slice))
                .map_err(|e| Error::TextureImageFailure(e.as_string().unwrap_or_else(|| "Unknown error".to_string())))?;
        }

        self.texture.context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);

        Ok(())
    }
}
