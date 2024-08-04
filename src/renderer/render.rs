use std::{collections::HashMap, path::Path, ptr};

use gl::types::GLenum;

use crate::{library::{constants::TWICE_PI, utils::{calc_control_point, convert_coordinates, convert_size, length_of_line}}, set_attribute};

use super::{buffer::Buffer, color::{Color, ColorType}, error::Result, program::Program, shader::Shader, source_code::{TEXTURE_FRAGMENT_SHADER_SOURCE, TEXTURE_VERTEX_SHADER_SOURCE, VERTICES_FRAGMENT_SHADER_SOURCE, VERTICES_VERTEX_SHADER_SOURCE}, texture::Texture, vertex_array::VertexArray, vertice::{Position, Vertice, _TextureVerticeData, _VerticeData}};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Size {
    pub width: f32,
    pub height: f32
}

// #[derive(Debug, PartialEq)]
// struct Object {
//     vertex_offset: usize,
//     vertex_count: usize,
//     index_offset: usize,
//     index_count: i32,
//     texture: Option<Texture>,
//     mode: GLenum
// }

// impl Object {
//     fn new(vertex_offset: usize, vertex_count: usize, index_offset: usize, index_count: i32, texture: Option<Texture>, mode: GLenum) -> Self {
//         Self {
//             vertex_offset,
//             vertex_count,
//             index_offset,
//             index_count,
//             mode,
//             texture
//         }
//     }
// }

#[derive(Debug, PartialEq)]
struct Object {
    vertices: Vec<_VerticeData>,
    indices: Option<Vec<u32>>,
    texture_image_path: Option<String>,
    mode: GLenum
}

impl Object {
    fn new(vertices: Vec<_VerticeData>, indices: Option<Vec<u32>>, texture_image_path: Option<String>, mode: GLenum) -> Self {
        Self {
            vertices,
            indices,
            mode,
            texture_image_path
        }
    }
}

#[derive(Debug)]
struct Image {
    vertex_array: VertexArray,
    texture: Texture,
    count: i32
}

impl Image {
    fn new(vertex_array: VertexArray, texture: Texture, count: i32) -> Self {
        Self {
            vertex_array,
            texture,
            count
        }
    }
}

#[derive(Debug)]
struct Background {
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
    vertex_array: VertexArray,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    background: Background,
    textures: HashMap<String, Texture>,
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

            let vertex_array = VertexArray::new();
            
            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            vertex_buffer.set_empty(0, gl::DYNAMIC_DRAW);

            let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
            index_buffer.set_empty(0, gl::DYNAMIC_DRAW);

            let pos_attrib = vertices_program.get_attrib_location("position")?;
            set_attribute!(vertex_array, pos_attrib, _VerticeData::0);
            
            let color_attrib = vertices_program.get_attrib_location("color")?;
            set_attribute!(vertex_array, color_attrib, _VerticeData::1);

            VertexArray::unbind();

            Ok(Self {
                size,
                vertices_program,
                texture_program,
                vertex_array,
                vertex_buffer,
                index_buffer,
                background: Background::default(),
                textures: HashMap::new(),
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

    pub fn fill_with_color(&mut self, color: Color) {
        let color_data = color.get_color_in_f32();
        
        self.background.color = color_data;
    }

    pub fn fill_with_image(&mut self, image_path: &str) -> Result<()> {
        let background_image_vertices: [_TextureVerticeData; 4] = [
            _TextureVerticeData([-1.0, 1.0], [0.0, 0.0]),
            _TextureVerticeData([1.0, 1.0], [1.0, 0.0]),
            _TextureVerticeData([1.0, -1.0], [1.0, 1.0]),
            _TextureVerticeData([-1.0, -1.0], [0.0, 1.0]),
        ];
        let background_indices: [i32; 6] = [0, 1, 2, 2, 3, 0];

        if self.background.image.is_none() {
            unsafe {
                let vertex_array = VertexArray::new();
                vertex_array.bind();
    
                let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
                vertex_buffer.set_data(&background_image_vertices, gl::STATIC_DRAW);
    
                let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
                index_buffer.set_data(&background_indices, gl::STATIC_DRAW);
    
                let pos_attrib = self.texture_program.get_attrib_location("position")?;
                set_attribute!(vertex_array, pos_attrib, _TextureVerticeData::0);
                
                let color_attrib = self.texture_program.get_attrib_location("vertexTexCoord")?;
                set_attribute!(vertex_array, color_attrib, _TextureVerticeData::1);
    
                let texture = Texture::new();
                texture.set_wrapping(gl::REPEAT);
                texture.set_filtering(gl::LINEAR);
                texture.load(&Path::new(image_path))?;
    
                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                gl::Enable(gl::BLEND);
    
                VertexArray::unbind();
                
                self.background.image = Some(Image::new(vertex_array, texture, background_indices.len() as i32));
            }
        }

        Ok(())
    }

    pub fn draw_triangle(&mut self, bottom_left: Vertice, bottom_right: Vertice, other_point: Vertice) {
        let vertices_data = vec![bottom_left.get_vertice_data(&self.size), bottom_right.get_vertice_data(&self.size), other_point.get_vertice_data(&self.size)];
        let indices = vec![0, 1, 2];

        self.objects.push(Object::new(vertices_data, Some(indices), None, gl::TRIANGLES));
    }

    pub fn draw_rectangle(&mut self, position: Position, size: Size, color: Color) {
        let position = convert_coordinates(position, &self.size);
        let size = convert_size(size, &self.size);

        let vertices_data = vec![
            _VerticeData(position.get_vertice_position(None), color.get_vertices_color_in_f32()),
            _VerticeData(position.get_vertice_position(Some(&Size { width: size.width, height: 0.0 })), color.get_vertices_color_in_f32()),
            _VerticeData(position.get_vertice_position(Some(&size)), color.get_vertices_color_in_f32()),
            _VerticeData(position.get_vertice_position(Some(&Size { width: 0.0, height: size.height })), color.get_vertices_color_in_f32()),
        ];
        let indices = vec![0, 1, 2, 2, 3, 0];

        self.objects.push(Object::new(vertices_data, Some(indices), None, gl::TRIANGLES));
    }

    pub fn draw_circle(&mut self, center: Position, radius: f32, color: Color, num_segments: Option<u32>) {
        let num_segments = num_segments.unwrap_or(360);

        let mut vertices_data = Vec::with_capacity(num_segments as usize);
        let mut indices = Vec::with_capacity(num_segments as usize * 3);

        let center = convert_coordinates(center, &self.size);
        
        let radius_x = (radius * 2.0) / self.size.width;
        let radius_y = (radius * 2.0) / self.size.height; 

        for num in 0..=num_segments {
            let theta = TWICE_PI * (num as f32) / (num_segments as f32);
            
            let x = theta.cos() * radius_x + center.x;
            let y = theta.sin() * radius_y + center.y;

            vertices_data.push(_VerticeData([x, y], color.get_vertices_color_in_f32()));

            indices.extend_from_slice(&[0, num, num+1]);
        }

        self.objects.push(Object::new(vertices_data, Some(indices), None, gl::TRIANGLE_FAN));
    }

    pub fn draw_curved_line(&mut self, start: Position, end: Position, color: Color, num_segments: Option<u32>) {
        let num_segments = num_segments.unwrap_or(length_of_line(&start, &end) as u32);

        let mut vertices_data = Vec::with_capacity(num_segments as usize);
        let mut indices = Vec::with_capacity(num_segments as usize);
        
        let start = convert_coordinates(start, &self.size);
        let end = convert_coordinates(end, &self.size);
        
        let control_point = calc_control_point(&start, &end);

        for num in 0..=num_segments {
            let t = num as f32 / num_segments as f32;

            let x = (1.0 - t).powi(2) * start.x + 2.0 * (1.0 - t) * t * control_point.x + t.powi(2) * end.x;
            let y = (1.0 - t).powi(2) * start.y + 2.0 * (1.0 - t) * t * control_point.y + t.powi(2) * end.y;

            vertices_data.push(_VerticeData([x, y], color.get_vertices_color_in_f32()));

            indices.push(num);
        }

        self.objects.push(Object::new(vertices_data, Some(indices), None, gl::LINE_STRIP));
    }

    pub fn draw_line(&mut self, start: Position, end: Position, color: Color) {    
        let start = convert_coordinates(start, &self.size);
        let end = convert_coordinates(end, &self.size);
        
        let vertices_data = vec![
            _VerticeData(start.get_vertice_position(None), color.get_vertices_color_in_f32()),
            _VerticeData(end.get_vertice_position(None), color.get_vertices_color_in_f32()),
        ];
        let indices = vec![0, 1];

        self.objects.push(Object::new(vertices_data, Some(indices), None, gl::LINE_STRIP));
    }

    pub fn load_image(&mut self, image_path: &str, position: Position, size: Size) -> Result<()> {
        let position = convert_coordinates(position, &self.size);
        let size = convert_size(size, &self.size);

        let vertices_data = vec![
            _VerticeData(position.get_vertice_position(None), [0.0, 0.0, 0.0, 0.0]),
            _VerticeData(position.get_vertice_position(Some(&Size { width: size.width, height: 0.0 })), [1.0, 0.0, 0.0, 0.0]),
            _VerticeData(position.get_vertice_position(Some(&size)), [1.0, 1.0, 0.0, 0.0]),
            _VerticeData(position.get_vertice_position(Some(&Size { width: 0.0, height: size.height })), [0.0, 1.0, 0.0, 0.0]),
        ];
        let indices = vec![0, 1, 2, 2, 3, 0];

        let found_texture = self.textures.get(image_path);

        if found_texture.is_some() {
            self.objects.push(Object::new(vertices_data, Some(indices), Some(image_path.to_string()), gl::TRIANGLES));

            return Ok(());
        }
        
        unsafe {
            let texture = Texture::new();
            texture.set_wrapping(gl::REPEAT);
            texture.set_filtering(gl::LINEAR);
            texture.load(&Path::new(image_path))?;

            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::BLEND);

            self.textures.insert(image_path.to_string(), texture);
            
            self.objects.push(Object::new(vertices_data, Some(indices), Some(image_path.to_string()), gl::TRIANGLES));
        }
        
        Ok(())
    }

    pub fn render(&mut self) -> Result<()> {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
            if let Some(background_image) = &self.background.image {
                self.texture_program.apply();
                background_image.texture.bind();
                background_image.vertex_array.bind();

                gl::DrawElements(gl::TRIANGLES, background_image.count, gl::UNSIGNED_INT, ptr::null()); 
            }

            let (red, green, blue, alpha) = self.background.color;

            gl::ClearColor(red, green, blue, alpha);

            for object in self.objects.iter() {
                self.vertex_array.bind();

                self.vertex_buffer.set_data(&object.vertices, gl::DYNAMIC_DRAW);

                if let Some(texture_image_path) = &object.texture_image_path {
                    assert!(self.textures.get(texture_image_path) != None, "texture must exist");

                    let texture = self.textures.get(texture_image_path).unwrap();

                    self.texture_program.apply();
                    texture.bind();
                } else {
                    self.vertices_program.apply();
                }

                if let Some(indices) = &object.indices {
                    self.index_buffer.set_data(indices, gl::DYNAMIC_DRAW);
                    
                    gl::DrawElements(object.mode, indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
                } else {
                    gl::DrawArrays(object.mode, 0, object.vertices.len() as i32);
                }

                VertexArray::unbind();
            }

            self.objects = Vec::new();
        }

        Ok(())
    }    
}
