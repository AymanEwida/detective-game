use std::{collections::VecDeque, path::Path, ptr};

use gl::types::GLenum;

use crate::{library::{constants::TWICE_PI, utils::{calc_control_point, convert_coordinates, convert_size, length_of_line}}, set_attribute};

use super::{buffer::Buffer, color::{Color, ColorType}, error::Result, program::Program, shader::Shader, source_code::{TEXTURE_FRAGMENT_SHADER_SOURCE, TEXTURE_VERTEX_SHADER_SOURCE, VERTICES_FRAGMENT_SHADER_SOURCE, VERTICES_VERTEX_SHADER_SOURCE}, texture::Texture, vertex_array::VertexArray, vertice::{Position, Vertice, _TextureVerticeData, _VerticeData}};

// static mut OBJECTS: Vec<Object> = Vec::new();

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Size {
    pub width: f32,
    pub height: f32
}

#[derive(Debug, PartialEq)]
enum DrawType {
    INDEX,
    ARRAY
}

impl Default for DrawType {
    fn default() -> Self {
        Self::INDEX
    }
}

#[derive(Debug, PartialEq)]
struct Object<'a> {
    vertices_data: Vec<_VerticeData>,
    indices_data: Option<&'a [i32]>,
    count: Option<i32>,
    texture: Option<Texture>,
    draw_type: DrawType,
    mode: GLenum
}

impl<'a> Object<'a> {
    fn new(vertices_data: Vec<_VerticeData>, indices_data: Option<&'a [i32]>, count: Option<i32>, texture: Option<Texture>, draw_type: DrawType, mode: GLenum) -> Self {
        Self {
            vertices_data,
            indices_data,
            count,
            draw_type,
            mode,
            texture
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

#[derive(Debug)]
struct RenderableObject<'a> {
    count: i32,
    texture: &'a Option<Texture>,
    draw_type: &'a DrawType,
    mode: GLenum
}

impl<'a> RenderableObject<'a> {
    fn new(count: i32, texture: &'a Option<Texture>, draw_type: &'a DrawType, mode: GLenum) -> Self {
        Self {
            count,
            texture,
            draw_type,
            mode
        }
    }
}

pub struct Render<'a> {
    size: Size,
    vertices_program: Program,
    texture_program: Program,
    vertex_array: VertexArray,
    background: Background,
    objects: Vec<Object<'a>>,
    renderable_objects: VecDeque<RenderableObject<'a>>
}

impl Render<'_> {
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
                vertex_array: VertexArray::new(),
                background: Background::default(),
                objects: Vec::new(),
                renderable_objects: VecDeque::new(),
            })
        }
    }
}


// TODO: find a better way to update objects and render them this bad!!
impl Render<'_> {
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

    pub fn draw_triangle(&mut self, vertices: [Vertice; 3]) {
        let indices = &[0, 1, 2];

        self.objects.push(Object::new(vertices.map(| vertice | vertice.get_vertice_data(&self.size)).to_vec(), Some(indices), None, None, DrawType::default(), gl::TRIANGLES));
    }

    pub fn draw_rectangle(&mut self, position: Position, size: Size, color: Color) {
        let position = convert_coordinates(position, &self.size);
        let size = convert_size(size, &self.size);

        let vertices = vec![
            _VerticeData(position.get_vertice_position(None), color.get_vertices_color_in_f32()),
            _VerticeData(position.get_vertice_position(Some(&Size { width: size.width, height: 0.0 })), color.get_vertices_color_in_f32()),
            _VerticeData(position.get_vertice_position(Some(&size)), color.get_vertices_color_in_f32()),
            _VerticeData(position.get_vertice_position(Some(&Size { width: 0.0, height: size.height })), color.get_vertices_color_in_f32()),
        ];
        let indices = &[0, 1, 2, 2, 3, 0];

        self.objects.push(Object::new(vertices, Some(indices), None, None, DrawType::default(), gl::TRIANGLES));
    }

    pub fn draw_circle(&mut self, center: Position, radius: f32, color: Color, num_segments: Option<u32>) {
        let num_segments = num_segments.unwrap_or(360);

        let mut vertices: Vec<_VerticeData> = Vec::with_capacity(num_segments as usize);

        let center = convert_coordinates(center, &self.size);
        
        let radius_x = (radius * 2.0) / self.size.width;
        let radius_y = (radius * 2.0) / self.size.height; 

        for num in 0..=num_segments {
            let theta = TWICE_PI * (num as f32) / (num_segments as f32);
            
            let x = theta.cos() * radius_x + center.x;
            let y = theta.sin() * radius_y + center.y;

            vertices.push(_VerticeData([x, y], color.get_vertices_color_in_f32()));
        }

        let count = vertices.len() as i32;

        self.objects.push(Object::new(vertices, None, Some(count), None, DrawType::ARRAY, gl::TRIANGLE_FAN));
    }

    pub fn draw_curved_line(&mut self, start: Position, end: Position, color: Color, num_segments: Option<u32>) {
        let num_segments = num_segments.unwrap_or(length_of_line(&start, &end) as u32);

        let mut vertices: Vec<_VerticeData> = Vec::with_capacity(num_segments as usize);

        let start = convert_coordinates(start, &self.size);
        let end = convert_coordinates(end, &self.size);

        let control_point = calc_control_point(&start, &end);

        for num in 0..=num_segments {
            let t = num as f32 / num_segments as f32;

            let x = (1.0 - t).powi(2) * start.x + 2.0 * (1.0 - t) * t * control_point.x + t.powi(2) * end.x;
            let y = (1.0 - t).powi(2) * start.y + 2.0 * (1.0 - t) * t * control_point.y + t.powi(2) * end.y;

            vertices.push(_VerticeData([x, y], color.get_vertices_color_in_f32()));
        }

        let count = vertices.len() as i32;

        self.objects.push(Object::new(vertices, None, Some(count), None, DrawType::ARRAY, gl::LINE_STRIP));
    }

    pub fn draw_line(&mut self, start: Position, end: Position, color: Color) {    
        let start = convert_coordinates(start, &self.size);
        let end = convert_coordinates(end, &self.size);
        
        let vertices = vec![
            _VerticeData(start.get_vertice_position(None), color.get_vertices_color_in_f32()),
            _VerticeData(end.get_vertice_position(None), color.get_vertices_color_in_f32()),
        ];

        let count = vertices.len() as i32;

        self.objects.push(Object::new(vertices, None, Some(count), None, DrawType::ARRAY, gl::LINE_STRIP));
    }

    pub fn load_image(&mut self, image_path: &str, position: Position, size: Size) -> Result<()> {
        let position = convert_coordinates(position, &self.size);
        let size = convert_size(size, &self.size);

        let texture_vertices = vec![
            _VerticeData(position.get_vertice_position(None), [0.0, 0.0, 0.0, 0.0]),
            _VerticeData(position.get_vertice_position(Some(&Size { width: size.width, height: 0.0 })), [1.0, 0.0, 0.0, 0.0]),
            _VerticeData(position.get_vertice_position(Some(&size)), [1.0, 1.0, 0.0, 0.0]),
            _VerticeData(position.get_vertice_position(Some(&Size { width: 0.0, height: size.height })), [0.0, 1.0, 0.0, 0.0]),
        ];
        let indices = &[0, 1, 2, 2, 3, 0];

        unsafe {
            let texture = Texture::new();
            texture.set_wrapping(gl::REPEAT);
            texture.set_filtering(gl::LINEAR);
            texture.load(&Path::new(image_path))?;

            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::BLEND);

            self.objects.push(Object::new(texture_vertices, Some(indices), None, Some(texture), DrawType::default(), gl::TRIANGLES));
        }
        
        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        self.objects.sort_by(| first, second | second.vertices_data.len().partial_cmp(&first.vertices_data.len()).unwrap());

        let mut vertex_buffer_data: Vec<&[_VerticeData]> = Vec::new();
        let mut index_buffer_data = Vec::new();

        for _ in 0..self.objects.len() {
            let object = self.objects.pop();

            if let Some(object) = &object {
                vertex_buffer_data.push(&object.vertices_data);
    
                if let Some(indices_data) = object.indices_data {
                    index_buffer_data.push(indices_data);
                    
                    self.renderable_objects.push_back(RenderableObject::new(indices_data.len() as i32, &object.texture, &object.draw_type, object.mode));
                } 
                
                if let Some(count) = object.count {
                    self.renderable_objects.push_back(RenderableObject::new(count, &object.texture, &object.draw_type, object.mode));    
                }
            }
        }

        unsafe {
            let vertex_array = &self.vertex_array;
    
            vertex_array.bind();
    
            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            vertex_buffer.set_data(&vertex_buffer_data, gl::STATIC_DRAW);
    
            let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
            index_buffer.set_data(&index_buffer_data, gl::STATIC_DRAW);
    
            let pos_attrib = self.vertices_program.get_attrib_location("position")?;
            set_attribute!(vertex_array, pos_attrib, _VerticeData::0);
            
            let color_attrib = self.vertices_program.get_attrib_location("color")?;
            set_attribute!(vertex_array, color_attrib, _VerticeData::1);
    
            VertexArray::unbind();
        }

        self.render();

        Ok(())
    }

    fn render(&mut self) {
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

            print!("before render: {:?}\n\n", self.objects);

            for _ in 0..self.renderable_objects.len() {
                let object = self.renderable_objects.pop_front();

                print!("during render: {:?}, object: {:?}\n\n", self.objects, object);

                if let Some(object) = object {
                    match object.draw_type {
                        DrawType::INDEX => {
                            if let Some(texture) = &object.texture {
                                self.texture_program.apply();
                                texture.bind();
                            } else {
                                self.vertices_program.apply();
                            }
    
                            self.vertex_array.bind();
    
                            gl::DrawElements(object.mode, object.count, gl::UNSIGNED_INT, ptr::null());
                        },
                        DrawType::ARRAY => {
                            if let Some(texture) = &object.texture {
                                self.texture_program.apply();
                                texture.bind();
                            } else {
                                self.vertices_program.apply();
                            }
    
                            self.vertex_array.bind();
    
                            gl::DrawArrays(object.mode, 0, object.count);
                        }
                    }
    
                    VertexArray::unbind()
                }

                print!("remain: {:?}\n\n", self.objects);
            }

            print!("after render: {:?}\n\n", self.objects);
        }
    }
}
