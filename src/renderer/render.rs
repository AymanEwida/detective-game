use super::{color::Color, error::Result, program::Program, shader::Shader, source_code::{SourceCode, SourceCodeType, VERTICES_FRAGMENT_SHADER_SOURCE, VERTICES_VERTEX_SHADER_SOURCE}};

pub type Size = (usize, usize);

pub struct Render {
    _size: Size,
    program: Program
}

impl Render {
    pub fn new(size: Size) -> Result<Self> {
        unsafe {
            let vertex_shader = Shader::new(SourceCode::new(SourceCodeType::Vertex, VERTICES_VERTEX_SHADER_SOURCE, Some(size)).get_source_code(), gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(SourceCode::new(SourceCodeType::Fragment, VERTICES_FRAGMENT_SHADER_SOURCE, None).get_source_code(), gl::FRAGMENT_SHADER)?;
            let program = Program::new(&[vertex_shader, fragment_shader])?;

            Ok(Self {
                _size: size,
                program
            })
        }
    }
}

impl Render {
    pub fn fill_with_color(&self, color: Color) {
        unsafe {
            let (red, green, blue, alpha) = color.get_color_in_f32();

            gl::ClearColor(red, green, blue, alpha);
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.program.apply();
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}
