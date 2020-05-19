use super::error::{
    Error,
    Result
};

use web_sys::{
    WebGl2RenderingContext,
    WebGlProgram
};

use boolinator::Boolinator;

pub struct Attribute {
    context: WebGl2RenderingContext,
    location: u32
}

pub struct ActiveAttribute<'a> {
    attribute: &'a Attribute
}

impl Attribute {
    pub fn new(context: WebGl2RenderingContext, program: &WebGlProgram, name: &str) -> Result<Attribute> {
        let location = context.get_attrib_location(&program, name);

        (location != -1)
            .as_result_from(
                || Attribute {
                    context,
                    location: location as u32
                },
                || Error::AttributeNotFound(name.to_string()))
    }

    pub fn with<U, F: FnOnce(ActiveAttribute) -> U>(&self, f: F) -> U {
        self.context.enable_vertex_attrib_array(self.location);
        let result = f(ActiveAttribute { attribute: self });
        self.context.disable_vertex_attrib_array(self.location);
        result
    }
}

impl<'a> ActiveAttribute<'a> {
    pub fn vertex_attrib_pointer(&self, stride: i32, offset: i32) {
        self.attribute.context.vertex_attrib_pointer_with_i32(self.attribute.location, 2, WebGl2RenderingContext::FLOAT, false, stride * 8, offset * 4);
    }
}
