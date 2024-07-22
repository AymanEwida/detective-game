use std::ptr;

use crate::set_attribute;

use super::{buffer::Buffer, color::Color, error::Result, program::Program, shader::Shader, source_code::{SourceCode, SourceCodeType, VERTICES_FRAGMENT_SHADER_SOURCE, VERTICES_VERTEX_SHADER_SOURCE}, vertex_array::VertexArray, vertice::{Position, Vertice, _VerticeData}};

pub type Size = (usize, usize);

pub struct Render {
    size: Size,
    program: Program,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    vertex_array: VertexArray,
}

impl Render {
    pub fn new(size: Size) -> Result<Self> {
        unsafe {
            let vertex_shader = Shader::new(SourceCode::new(SourceCodeType::Vertex, VERTICES_VERTEX_SHADER_SOURCE, Some(size)).get_source_code(), gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(SourceCode::new(SourceCodeType::Fragment, VERTICES_FRAGMENT_SHADER_SOURCE, None).get_source_code(), gl::FRAGMENT_SHADER)?;
            let program = Program::new(&[vertex_shader, fragment_shader])?;

            let vertex_array = VertexArray::new();
            vertex_array.bind();

            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);

            Ok(Self {
                size,
                program,
                vertex_buffer,
                index_buffer,
                vertex_array
            })
        }
    }
}

impl Render {
    pub fn resize(&mut self, new_size: Size) -> Result<()> {
        self.size = new_size;

        unsafe {
            let vertex_shader = Shader::new(SourceCode::new(SourceCodeType::Vertex, VERTICES_VERTEX_SHADER_SOURCE, Some(new_size)).get_source_code(), gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(SourceCode::new(SourceCodeType::Fragment, VERTICES_FRAGMENT_SHADER_SOURCE, None).get_source_code(), gl::FRAGMENT_SHADER)?;
            let program = Program::new(&[vertex_shader, fragment_shader])?;

            self.program = program;
        }

        Ok(())
    }

    pub fn fill_with_color(&self, color: Color) {
        unsafe {
            let (red, green, blue, alpha) = color.get_color_in_f32();

            gl::ClearColor(red, green, blue, alpha);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw_triangle(&self, vertices: [Vertice; 3]) {
        unsafe {
            self.vertex_buffer.set_data(&vertices.map(| vertice | vertice.get_vertices_data()), gl::STATIC_DRAW);
            self.index_buffer.set_data(&[0, 1, 2, 2, 0], gl::STATIC_DRAW);

            let vertex_array = &self.vertex_array;

            let pos_attrib = self.program.get_attrib_location("position").expect("Unable to get attribute location");
            set_attribute!(vertex_array, pos_attrib, _VerticeData::0);
            
            let color_attrib = self.program.get_attrib_location("color").expect("Unable to get attribute location");
            set_attribute!(vertex_array, color_attrib, _VerticeData::1);

            self.program.apply();
            self.vertex_array.bind();
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, 5, gl::UNSIGNED_INT, ptr::null());
        }
    }

    pub fn draw_rectangle(&self, position: Position, size: Size, color: Color) {
        unsafe {
            let vertices: [Vertice; 4] = [
                Vertice(position, color),
                Vertice([position[0]+size.0 as f32, position[1]], Color::Green),
                Vertice([position[0]+size.0 as f32, position[1]-size.1 as f32], Color::Blue),
                Vertice([position[0], position[1]-size.1 as f32], Color::White),
            ];

            self.vertex_buffer.set_data(&vertices.map(| vertice | vertice.get_vertices_data()), gl::STATIC_DRAW);
            self.index_buffer.set_data(&[0, 1, 2, 2, 3, 0], gl::STATIC_DRAW);

            let vertex_array = &self.vertex_array;

            let pos_attrib = self.program.get_attrib_location("position").expect("Unable to get attribute location");
            set_attribute!(vertex_array, pos_attrib, _VerticeData::0);
            
            let color_attrib = self.program.get_attrib_location("color").expect("Unable to get attribute location");
            set_attribute!(vertex_array, color_attrib, _VerticeData::1);

            self.program.apply();
            self.vertex_array.bind();
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
    }
}
