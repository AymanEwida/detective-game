use super::{error::Result, program::Program, shader::Shader};

const VERTEX_SHADER_SOURCE: &str = r#"
#version 330
in vec2 position;
in vec3 color;
out vec3 vertexColor;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    vertexColor = color;
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330
out vec4 FragColor;
in vec3 vertexColor;
void main() {
    FragColor = vec4(vertexColor, 1.0);
}
"#;

pub struct Render {
    program: Program
}

impl Render {
    pub fn new() -> Result<Self> {
        unsafe {
            let vertex_shader = Shader::new(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
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
