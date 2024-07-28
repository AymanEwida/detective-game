use std::{path::Path, ptr};

use gl::types::GLenum;

use crate::{library::{constants::TWICE_PI, utils::{calc_control_point, convert_coordinates, convert_size, length_of_line}}, set_attribute};

use super::{buffer::Buffer, color::{self, Color, ColorType}, error::Result, program::Program, shader::Shader, source_code::{TEXTURE_FRAGMENT_SHADER_SOURCE, TEXTURE_VERTEX_SHADER_SOURCE, VERTICES_FRAGMENT_SHADER_SOURCE, VERTICES_VERTEX_SHADER_SOURCE}, texture::{self, Texture}, vertex_array::{self, VertexArray}, vertice::{Position, Vertice, _TextureVerticeData, _VerticeData}};

#[derive(Debug, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32
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

#[derive(Debug)]
struct Image {
    vertex_array: VertexArray,
    texture: Texture,
}

impl Image {
    fn new(vertex_array: VertexArray, texture: Texture) -> Self {
        Self {
            vertex_array,
            texture
        }
    }
}

#[derive(Debug)]
pub struct Background {
    color: ColorType,
    image: Option<Image>
}

impl Default for Background {
    fn default() -> Self {
        Self {
            color: Color::Black.get_color_in_f32(),
            image: None
        }
    }
}

pub struct Render {
    size: Size,
    vertices_program: Program,
    texture_program: Program,
    background: Background,
    objects: Vec<Object>
}

impl Render {
    pub fn new(size: Size) -> Result<Self> {
        unsafe {
            let vertices_vertex_shader = Shader::new(VERTICES_VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER)?;
            let vertices_fragment_shader = Shader::new(VERTICES_FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
            let vertices_program = Program::new(&[vertices_vertex_shader, vertices_fragment_shader])?;

            let texture_vertex_shader = Shader::new(TEXTURE_VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER)?;
            let texture_fragment_shader = Shader::new(TEXTURE_FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
            let texture_program = Program::new(&[texture_vertex_shader, texture_fragment_shader])?;

            Ok(Self {
                size,
                vertices_program,
                texture_program,
                background: Background::default(),
                objects: Vec::new(),
            })
        }
    }
}

impl Render {
    pub fn resize(&mut self, new_size: Size) {
        self.size = new_size;

        unsafe {
            gl::Viewport(0, 0, self.size.width as i32, self.size.height as i32);
        }
    }

    pub fn fill_with_color(&self, color: Color) -> Background {
        let colot_data = color.get_color_in_f32();
        
        Background { color: colot_data, image: None }
    }

    pub fn fill_with_image(&self, image_path: &str) -> Result<Background> {
        let vertices: [_TextureVerticeData; 4] = [
            _TextureVerticeData([-1.0, 1.0], [0.0, 0.0]),
            _TextureVerticeData([1.0, 1.0], [1.0, 0.0]),
            _TextureVerticeData([1.0, -1.0], [1.0, 1.0]),
            _TextureVerticeData([-1.0, -1.0], [0.0, 1.0]),
        ];

        unsafe {
            let vertex_array = VertexArray::new();
            vertex_array.bind();

            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            vertex_buffer.set_data(&vertices, gl::STATIC_DRAW);

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

            Ok(Background { color: Color::Black.get_color_in_f32(), image: Some(Image::new(vertex_array, texture)) })
        }
    }

    pub fn draw_triangle(&self, key: String, vertices: [Vertice; 3]) -> Result<Object> {
        unsafe {
            let vertex_array = VertexArray::new();
            vertex_array.bind();

            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            vertex_buffer.set_data(&vertices.map(| vertice | vertice.get_vertice_data(&self.size)), gl::STATIC_DRAW);

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
        let position = convert_coordinates(position, &self.size);
        let size = convert_size(size, &self.size);

        let vertices: [_VerticeData; 4] = [
            _VerticeData(position.get_vertice_position(None), color.get_vertices_color_in_f32()),
            _VerticeData(position.get_vertice_position(Some(&Size { width: size.width, height: 0.0 })), color.get_vertices_color_in_f32()),
            _VerticeData(position.get_vertice_position(Some(&size)), color.get_vertices_color_in_f32()),
            _VerticeData(position.get_vertice_position(Some(&Size { width: 0.0, height: size.height })), color.get_vertices_color_in_f32()),
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

        let center = convert_coordinates(center, &self.size);
        
        let radius_x = (radius * 2.0) / self.size.width;
        let radius_y = (radius * 2.0) / self.size.height; 

        for num in 0..=num_segments {
            let theta = TWICE_PI * (num as f32) / (num_segments as f32);
            
            let x = theta.cos() * radius_x + center.x;
            let y = theta.sin() * radius_y + center.y;

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

        let start = convert_coordinates(start, &self.size);
        let end = convert_coordinates(end, &self.size);

        let control_point = calc_control_point(&start, &end);

        for num in 0..=num_segments {
            let t = num as f32 / num_segments as f32;

            let x = (1.0 - t).powi(2) * start.x + 2.0 * (1.0 - t) * t * control_point.x + t.powi(2) * end.x;
            let y = (1.0 - t).powi(2) * start.y + 2.0 * (1.0 - t) * t * control_point.y + t.powi(2) * end.y;

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
        let start = convert_coordinates(start, &self.size);
        let end = convert_coordinates(end, &self.size);
        
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
        let position = convert_coordinates(position, &self.size);
        let size = convert_size(size, &self.size);

        let texture_vertices: [_TextureVerticeData; 4] = [
            _TextureVerticeData(position.get_vertice_position(None), [0.0, 0.0]),
            _TextureVerticeData(position.get_vertice_position(Some(&Size { width: size.width, height: 0.0 })), [1.0, 0.0]),
            _TextureVerticeData(position.get_vertice_position(Some(&size)), [1.0, 1.0]),
            _TextureVerticeData(position.get_vertice_position(Some(&Size { width: 0.0, height: size.height })), [0.0, 1.0]),
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

    pub fn update(&mut self, background: Option<Background>, objects: Vec<Object>) {
        print!("{:?}\n", background);

        if background.is_some() {
            self.background = background.unwrap_or(Background::default());
        }
        
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

    pub fn render(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
            if let Some(background_image) = &self.background.image {
                print!("here1\n");
                self.texture_program.apply();
                background_image.texture.bind();
                background_image.vertex_array.bind();

                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null()); 
            } else {
                let (red, green, blue, alpha) = self.background.color;
    
                gl::ClearColor(red, green, blue, alpha);
            }
            
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
