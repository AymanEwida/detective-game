use std::{path::Path, ptr};

use gl::types::GLenum;

use crate::{library::{constants::TWICE_PI, utils::{calc_mid_point, length_of_line}}, renderer::{texture::Texture, vertice::_TextureVerticeData}, set_attribute};

use super::{buffer::Buffer, color::Color, error::Result, program::Program, shader::Shader, source_code::{SourceCode, SourceCodeType, TEXTURE_FRAGMENT_SHADER_SOURCE, TEXTURE_VERTEX_SHADER_SOURCE, VERTICES_FRAGMENT_SHADER_SOURCE, VERTICES_VERTEX_SHADER_SOURCE}, vertex_array::VertexArray, vertice::{Position, Vertice, _VerticeData}};

#[derive(Debug)]
pub struct Size {
    pub width: usize,
    pub height: usize
}

#[derive(Debug)]
enum DrawType {
    INDEX,
    ARRAY
}

impl Default for DrawType {
    fn default() -> Self {
        Self::INDEX
    }
}

#[derive(Debug)]
pub struct Object {
    key: String,
    vertex_array: VertexArray,
    texture: Option<Texture>,
    count: i32,
    draw_type: DrawType,
    mode: GLenum
}

pub struct Render {
    size: Size,
    vertices_program: Program,
    texture_program: Program,
    objects: Vec<Object>
}

impl Render {
    pub fn new(size: Size) -> Result<Self> {
        unsafe {
            let vertices_vertex_shader = Shader::new(SourceCode::new(SourceCodeType::Vertex, VERTICES_VERTEX_SHADER_SOURCE, Some(size.height)).get_source_code(), gl::VERTEX_SHADER)?;
            let vertices_fragment_shader = Shader::new(SourceCode::new(SourceCodeType::Fragment, VERTICES_FRAGMENT_SHADER_SOURCE, None).get_source_code(), gl::FRAGMENT_SHADER)?;
            let vertices_program = Program::new(&[vertices_vertex_shader, vertices_fragment_shader])?;

            let texture_vertex_shader = Shader::new(SourceCode::new(SourceCodeType::Vertex, TEXTURE_VERTEX_SHADER_SOURCE, Some(size.height)).get_source_code(), gl::VERTEX_SHADER)?;
            let texture_fragment_shader = Shader::new(SourceCode::new(SourceCodeType::Fragment, TEXTURE_FRAGMENT_SHADER_SOURCE, None).get_source_code(), gl::FRAGMENT_SHADER)?;
            let texture_program = Program::new(&[texture_vertex_shader, texture_fragment_shader])?;

            Ok(Self {
                size,
                vertices_program,
                texture_program,
                objects: Vec::new(),
            })
        }
    }
}

impl Render {
    pub fn resize(&mut self, new_size: Size) -> Result<()> {
        self.size = new_size;

        unsafe {
            let vertices_vertex_shader = Shader::new(SourceCode::new(SourceCodeType::Vertex, VERTICES_VERTEX_SHADER_SOURCE, Some(self.size.height)).get_source_code(), gl::VERTEX_SHADER)?;
            let vertices_fragment_shader = Shader::new(SourceCode::new(SourceCodeType::Fragment, VERTICES_FRAGMENT_SHADER_SOURCE, None).get_source_code(), gl::FRAGMENT_SHADER)?;
            let vertices_program = Program::new(&[vertices_vertex_shader, vertices_fragment_shader])?;

            let texture_vertex_shader = Shader::new(SourceCode::new(SourceCodeType::Vertex, TEXTURE_VERTEX_SHADER_SOURCE, Some(self.size.height)).get_source_code(), gl::VERTEX_SHADER)?;
            let texture_fragment_shader = Shader::new(SourceCode::new(SourceCodeType::Fragment, TEXTURE_FRAGMENT_SHADER_SOURCE, None).get_source_code(), gl::FRAGMENT_SHADER)?;
            let texture_program = Program::new(&[texture_vertex_shader, texture_fragment_shader])?;

            self.vertices_program = vertices_program;
            self.texture_program = texture_program;
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

            let pos_attrib = self.vertices_program.get_attrib_location("position")?;
            set_attribute!(vertex_array, pos_attrib, _VerticeData::0);
            
            let color_attrib = self.vertices_program.get_attrib_location("color")?;
            set_attribute!(vertex_array, color_attrib, _VerticeData::1);

            VertexArray::unbind();

            Ok(Object { key, vertex_array, texture: None, count: 3, draw_type: DrawType::default(), mode: gl::TRIANGLES })
        }
    }

    pub fn draw_rectangle(&self, key: String, position: Position, size: Size, color: Color) -> Result<Object> {
        let vertices: [_VerticeData; 4] = [
            _VerticeData(position.get_vertice_position(None), color.get_vertices_color_in_f32()),
            _VerticeData(position.get_vertice_position(Some(&Size { width: size.width, height: 0 })), color.get_vertices_color_in_f32()),
            _VerticeData(position.get_vertice_position(Some(&size)), color.get_vertices_color_in_f32()),
            _VerticeData(position.get_vertice_position(Some(&Size { width: 0, height: size.height })), color.get_vertices_color_in_f32()),
        ];

        unsafe {
            let vertex_array = VertexArray::new();
            vertex_array.bind();

            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            vertex_buffer.set_data(&vertices, gl::STATIC_DRAW);

            let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
            index_buffer.set_data(&[0, 1, 2, 2, 3, 0], gl::STATIC_DRAW);

            let pos_attrib = self.vertices_program.get_attrib_location("position")?;
            set_attribute!(vertex_array, pos_attrib, _VerticeData::0);
            
            let color_attrib = self.vertices_program.get_attrib_location("color")?;
            set_attribute!(vertex_array, color_attrib, _VerticeData::1);

            VertexArray::unbind();

            Ok(Object { key, vertex_array, texture: None, count: 6, draw_type: DrawType::default(), mode: gl::TRIANGLES })
        }
    }

    pub fn draw_circle(&self, key: String, center: Position, radius: f32, color: Color, num_segments: Option<u32>) -> Result<Object> {
        let num_segments = num_segments.unwrap_or(360);

        let mut vertices: Vec<_VerticeData> = Vec::new();

        for num in 0..=num_segments {
            let theta = TWICE_PI * (num as f32) / (num_segments as f32);
            
            let x = theta.cos() * radius + center.x;
            let y = theta.sin() * radius + center.y;

            vertices.push(_VerticeData([x, y], color.get_vertices_color_in_f32()));
        }

        unsafe {
            let vertex_array = VertexArray::new();
            vertex_array.bind();

            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            vertex_buffer.set_data(&vertices, gl::STATIC_DRAW);

            let pos_attrib = self.vertices_program.get_attrib_location("position")?;
            set_attribute!(vertex_array, pos_attrib, _VerticeData::0);
            
            let color_attrib = self.vertices_program.get_attrib_location("color")?;
            set_attribute!(vertex_array, color_attrib, _VerticeData::1);

            VertexArray::unbind();

            Ok(Object { key, vertex_array, texture: None, count: vertices.len() as i32, draw_type: DrawType::ARRAY, mode: gl::TRIANGLE_FAN })
        }
    }

    pub fn draw_curved_line(&self, key: String, start: Position, end: Position, color: Color, num_segments: Option<u32>) -> Result<Object> {
        let num_segments = num_segments.unwrap_or(length_of_line(&start, &end) as u32);

        let mut vertices: Vec<_VerticeData> = Vec::new();

        let mid_point = calc_mid_point(&start, &end);

        for num in 0..=num_segments {
            let t = num as f32 / num_segments as f32;

            let x = (1.0 - t).powi(2) * start.x + 2.0 * (1.0 - t) * t * mid_point.x + t.powi(2) * end.x;
            let y = (1.0 - t).powi(2) * start.y + 2.0 * (1.0 - t) * t * (mid_point.y * 2.0) + t.powi(2) * end.y;

            vertices.push(_VerticeData([x, y], color.get_vertices_color_in_f32()));
        }
        
        unsafe {
            let vertex_array = VertexArray::new();
            vertex_array.bind();

            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            vertex_buffer.set_data(&vertices, gl::STATIC_DRAW);

            let pos_attrib = self.vertices_program.get_attrib_location("position")?;
            set_attribute!(vertex_array, pos_attrib, _VerticeData::0);
            
            let color_attrib = self.vertices_program.get_attrib_location("color")?;
            set_attribute!(vertex_array, color_attrib, _VerticeData::1);

            VertexArray::unbind();

            Ok(Object { key, vertex_array, texture: None, count: vertices.len() as i32, draw_type: DrawType::ARRAY, mode: gl::LINE_STRIP })
        }
    }

    pub fn draw_line(&self, key: String, start: Position, end: Position, color: Color) -> Result<Object> {

        let vertices: [_VerticeData; 2] = [
            _VerticeData(start.get_vertice_position(None), color.get_vertices_color_in_f32()),
            _VerticeData(end.get_vertice_position(None), color.get_vertices_color_in_f32()),
        ];
        
        unsafe {
            let vertex_array = VertexArray::new();
            vertex_array.bind();

            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            vertex_buffer.set_data(&vertices, gl::STATIC_DRAW);

            let pos_attrib = self.vertices_program.get_attrib_location("position")?;
            set_attribute!(vertex_array, pos_attrib, _VerticeData::0);
            
            let color_attrib = self.vertices_program.get_attrib_location("color")?;
            set_attribute!(vertex_array, color_attrib, _VerticeData::1);

            VertexArray::unbind();

            Ok(Object { key, vertex_array, texture: None, count: vertices.len() as i32, draw_type: DrawType::ARRAY, mode: gl::LINE_STRIP })
        }
    }

    pub fn load_image(&self, key: String, image_path: &str, position: Position, size: Size) -> Result<Object> {
        let texture_vertices: [_TextureVerticeData; 4] = [
            _TextureVerticeData(position.get_vertice_position(None), [0.0, 0.0]),
            _TextureVerticeData(position.get_vertice_position(Some(&Size { width: size.width, height: 0 })), [1.0, 0.0]),
            _TextureVerticeData(position.get_vertice_position(Some(&size)), [1.0, 1.0]),
            _TextureVerticeData(position.get_vertice_position(Some(&Size { width: 0, height: size.height })), [0.0, 1.0]),
        ];

        unsafe {
            let vertex_array = VertexArray::new();
            vertex_array.bind();

            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            vertex_buffer.set_data(&texture_vertices, gl::STATIC_DRAW);

            let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
            index_buffer.set_data(&[0, 1, 2, 2, 3, 0], gl::STATIC_DRAW);

            let pos_attrib = self.texture_program.get_attrib_location("position")?;
            set_attribute!(vertex_array, pos_attrib, _TextureVerticeData::0);
            
            let color_attrib = self.texture_program.get_attrib_location("vertexTexCoord")?;
            set_attribute!(vertex_array, color_attrib, _TextureVerticeData::1);

            let texture = Texture::new();
            texture.load(&Path::new(image_path))?;

            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::BLEND);

            VertexArray::unbind();

            Ok(Object { key, vertex_array, texture: Some(texture), count: 6, draw_type: DrawType::default(), mode: gl::TRIANGLES })
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
            gl::Clear(gl::COLOR_BUFFER_BIT);

            for object in self.objects.iter() {
                match object.draw_type {
                    DrawType::INDEX => {
                        if let Some(texture) = &object.texture {
                            self.texture_program.apply();
                            texture.bind();
                        } else {
                            self.vertices_program.apply();
                        }

                        object.vertex_array.bind();

                        gl::DrawElements(object.mode, object.count, gl::UNSIGNED_INT, ptr::null());
                    },
                    DrawType::ARRAY => {
                        if let Some(texture) = &object.texture {
                            self.texture_program.apply();
                            texture.bind();
                        } else {
                            self.vertices_program.apply();
                        }

                        object.vertex_array.bind();

                        gl::DrawArrays(object.mode, 0, object.count);
                    }
                }

                VertexArray::unbind()
            }
        }
    }
}
