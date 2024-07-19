use super::{error::Result, program::Program, shader::Shader, source_code::{VERTICES_FRAGMENT_SHADER_SOURCE, VERTICES_VERTEX_SHADER_SOURCE}};

pub type Size = (usize, usize);

pub struct Render {
    program: Program
}

impl Render {
    pub fn new() -> Result<Self> {
        unsafe {
            let vertex_shader = Shader::new(VERTICES_VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(VERTICES_FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
            let program = Program::new(&[vertex_shader, fragment_shader])?;

            Ok(Self {
                program
            })
        }
    }
}

impl Render {
    pub fn draw(&self) {
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.program.apply();
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}
