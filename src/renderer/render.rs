use crate::{renderer::vertex_array::VertexArray, set_attribute};

use super::{buffer::Buffer, color::Color, error::Result, program::Program, shader::Shader, source_code::{SourceCode, SourceCodeType, VERTICES_FRAGMENT_SHADER_SOURCE, VERTICES_VERTEX_SHADER_SOURCE}};

pub type Size = (usize, usize);
type Position = [f32; 2];

#[repr(C, packed)]
struct Vertices(Position, [f32; 3]);

pub struct Render {
    _size: Size,
    program: Program,
    _vertex_buffer: Buffer,
    vertex_array: VertexArray,
}

impl Render {
    pub fn new(size: Size) -> Result<Self> {
        unsafe {
            let vertex_shader = Shader::new(SourceCode::new(SourceCodeType::Vertex, VERTICES_VERTEX_SHADER_SOURCE, Some(size)).get_source_code(), gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(SourceCode::new(SourceCodeType::Fragment, VERTICES_FRAGMENT_SHADER_SOURCE, None).get_source_code(), gl::FRAGMENT_SHADER)?;
            let program = Program::new(&[vertex_shader, fragment_shader])?;

            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            let vertices: [Vertices; 3] = [
                Vertices([-200.0, -300.0], Color::Red.get_vertices_color_in_f32()),
                Vertices([200.0, -300.0], Color::Green.get_vertices_color_in_f32()),
                Vertices([0.0, 300.0], Color::Blue.get_vertices_color_in_f32())
            ];
            vertex_buffer.set_data(&vertices, gl::STATIC_DRAW);

            let vertex_array = VertexArray::new();
            
            let pos_attrib = program.get_attrib_location("position")?;
            set_attribute!(vertex_array, pos_attrib, Vertices::0);
            
            let color_attrib = program.get_attrib_location("color")?;
            set_attribute!(vertex_array, color_attrib, Vertices::1);

            Ok(Self {
                _size: size,
                program,
                _vertex_buffer: vertex_buffer,
                vertex_array
            })
        }
    }
}

impl Render {
    pub fn fill_with_color(&self, color: Color) {
        unsafe {
            let (red, green, blue, alpha) = color.get_color_in_f32();

            gl::ClearColor(red, green, blue, alpha);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.program.apply();
            self.vertex_array.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}
