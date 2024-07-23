use std::ptr;

use crate::set_attribute;

use super::{buffer::Buffer, color::Color, error::Result, program::Program, shader::Shader, source_code::{SourceCode, SourceCodeType, VERTICES_FRAGMENT_SHADER_SOURCE, VERTICES_VERTEX_SHADER_SOURCE}, vertex_array::VertexArray, vertice::{Position, Vertice, _VerticeData}};

#[derive(Debug)]
pub struct Size {
    pub width: usize,
    pub height: usize
}

#[derive(Debug)]
pub struct Object {
    key: String,
    vertex_array: VertexArray,
    count: i32
}

pub struct Render {
    size: Size,
    program: Program,
    objects: Vec<Object>
}

impl Render {
    pub fn new(size: Size) -> Result<Self> {
        unsafe {
            let vertex_shader = Shader::new(SourceCode::new(SourceCodeType::Vertex, VERTICES_VERTEX_SHADER_SOURCE, Some(size.height)).get_source_code(), gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(SourceCode::new(SourceCodeType::Fragment, VERTICES_FRAGMENT_SHADER_SOURCE, None).get_source_code(), gl::FRAGMENT_SHADER)?;
            let program = Program::new(&[vertex_shader, fragment_shader])?;

            Ok(Self {
                size,
                program,
                objects: Vec::new(),
            })
        }
    }
}

impl<'a> Render {
    pub fn resize(&mut self, new_size: Size) -> Result<()> {
        self.size = new_size;

        unsafe {
            let vertex_shader = Shader::new(SourceCode::new(SourceCodeType::Vertex, VERTICES_VERTEX_SHADER_SOURCE, Some(self.size.height)).get_source_code(), gl::VERTEX_SHADER)?;
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

    pub fn draw_triangle(&self, key: String, vertices: [Vertice; 3]) -> Result<Object> {
        unsafe {
            let vertex_array = VertexArray::new();
            vertex_array.bind();

            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            vertex_buffer.set_data(&vertices.map(| vertice | vertice.get_vertices_data()), gl::STATIC_DRAW);

            let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
            index_buffer.set_data(&[0, 1, 2], gl::STATIC_DRAW);

            let pos_attrib = self.program.get_attrib_location("position")?;
            set_attribute!(vertex_array, pos_attrib, _VerticeData::0);
            
            let color_attrib = self.program.get_attrib_location("color")?;
            set_attribute!(vertex_array, color_attrib, _VerticeData::1);

            VertexArray::unbind();

            Ok(Object { key, vertex_array, count: 3 })
        }
    }

    pub fn draw_rectangle(&self, key: String, position: Position, size: Size, color: Color) -> Result<Object> {
        unsafe {
            let vertex_array = VertexArray::new();
            vertex_array.bind();

            let vertices: [_VerticeData; 4] = [
                _VerticeData(position.get_vertice_position(None), color.get_vertices_color_in_f32()),
                _VerticeData(position.get_vertice_position(Some(&Size { width: size.width, height: 0 })), color.get_vertices_color_in_f32()),
                _VerticeData(position.get_vertice_position(Some(&size)), color.get_vertices_color_in_f32()),
                _VerticeData(position.get_vertice_position(Some(&Size { width: 0, height: size.height })), color.get_vertices_color_in_f32()),
            ];

            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            vertex_buffer.set_data(&vertices, gl::STATIC_DRAW);

            let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
            index_buffer.set_data(&[0, 1, 2, 2, 3, 0], gl::STATIC_DRAW);

            let pos_attrib = self.program.get_attrib_location("position")?;
            set_attribute!(vertex_array, pos_attrib, _VerticeData::0);
            
            let color_attrib = self.program.get_attrib_location("color")?;
            set_attribute!(vertex_array, color_attrib, _VerticeData::1);

            VertexArray::unbind();

            Ok(Object { key, vertex_array, count: 6 })
        }
    }

    pub fn update(&mut self, objects: Vec<Object>) {
        for object in objects {
            let mut flag = false;

            for i in 0..self.objects.len() {
                if self.objects[i].key == object.key {
                    flag = true;
                    
                    break;
                }
            }

            if !flag {
                self.objects.push(object);
            }
        }
    }

    pub fn draw(&self) {
        unsafe {
            self.program.apply();
            
            gl::Clear(gl::COLOR_BUFFER_BIT);

            print!("{:?}\n", self.objects);

            for object in self.objects.iter() {
                object.vertex_array.bind();
                gl::DrawElements(gl::TRIANGLES, object.count, gl::UNSIGNED_INT, ptr::null());
            }

        }
    }
}
