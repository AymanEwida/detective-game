use std::{path::Path, ptr};

use gl::types::GLenum;

use crate::{library::{constants::TWICE_PI, utils::{calc_control_point, convert_coordinates, convert_size, length_of_line}}, set_attribute};

use super::{buffer::Buffer, color::{Color, ColorType}, error::Result, program::Program, shader::Shader, source_code::{TEXTURE_FRAGMENT_SHADER_SOURCE, TEXTURE_VERTEX_SHADER_SOURCE, VERTICES_FRAGMENT_SHADER_SOURCE, VERTICES_VERTEX_SHADER_SOURCE}, texture::Texture, vertex_array::VertexArray, vertice::{Position, Vertice, _TextureVerticeData, _VerticeData}};

static mut OBJECTS: Vec<Object> = Vec::new();

#[derive(Debug, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum DrawType {
    INDEX,
    ARRAY
}

impl Default for DrawType {
    fn default() -> Self {
        Self::INDEX
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Object {
    key: String,
    vertex_array: VertexArray,
    texture: Option<Texture>,
    count: i32,
    draw_type: DrawType,
    mode: GLenum
}

impl Object {
    fn new(key: String, vertex_array: VertexArray, texture: Option<Texture>, count: i32, draw_type: DrawType, mode: GLenum) -> Self {
        Self {
            key,
            vertex_array,
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

    pub fn draw_triangle(&self, key: String, vertices: [Vertice; 3]) -> Result<()> {
        let indices: [i32; 3] = [0, 1, 2];

        let found_object = self.objects.binary_search_by_key(&&key, | object | &object.key);

        match found_object {
            Ok(index) => {
                unsafe {
                    let vertex_array = &self.objects[index].vertex_array;

                    vertex_array.bind();
        
                    let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
                    vertex_buffer.set_data(&vertices.map(| vertice | vertice.get_vertice_data(&self.size)), gl::STATIC_DRAW);
        
                    let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
                    index_buffer.set_data(&indices, gl::STATIC_DRAW);
        
                    let pos_attrib = self.vertices_program.get_attrib_location("position")?;
                    set_attribute!(vertex_array, pos_attrib, _VerticeData::0);
                    
                    let color_attrib = self.vertices_program.get_attrib_location("color")?;
                    set_attribute!(vertex_array, color_attrib, _VerticeData::1);
        
                    VertexArray::unbind();

                    Ok(())
                }
            },
            Err(_) => {
                unsafe {
                    let vertex_array = VertexArray::new();
                    vertex_array.bind();
        
                    let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
                    vertex_buffer.set_data(&vertices.map(| vertice | vertice.get_vertice_data(&self.size)), gl::STATIC_DRAW);
        
                    let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
                    index_buffer.set_data(&indices, gl::STATIC_DRAW);
        
                    let pos_attrib = self.vertices_program.get_attrib_location("position")?;
                    set_attribute!(vertex_array, pos_attrib, _VerticeData::0);
                    
                    let color_attrib = self.vertices_program.get_attrib_location("color")?;
                    set_attribute!(vertex_array, color_attrib, _VerticeData::1);
        
                    VertexArray::unbind();

                    OBJECTS.push(Object::new(key, vertex_array, None, indices.len() as i32, DrawType::default(), gl::TRIANGLES));
                    
                    Ok(())
                }
            }

        }
    }

    pub fn draw_rectangle(&self, key: String, position: Position, size: Size, color: Color) -> Result<()> {
        let position = convert_coordinates(position, &self.size);
        let size = convert_size(size, &self.size);

        let vertices: [_VerticeData; 4] = [
            _VerticeData(position.get_vertice_position(None), color.get_vertices_color_in_f32()),
            _VerticeData(position.get_vertice_position(Some(&Size { width: size.width, height: 0.0 })), color.get_vertices_color_in_f32()),
            _VerticeData(position.get_vertice_position(Some(&size)), color.get_vertices_color_in_f32()),
            _VerticeData(position.get_vertice_position(Some(&Size { width: 0.0, height: size.height })), color.get_vertices_color_in_f32()),
        ];
        let indices: [i32; 6] = [0, 1, 2, 2, 3, 0];

        let found_object = self.objects.binary_search_by_key(&&key, | object | &object.key);

        match found_object {
            Ok(index) => {
                unsafe {
                    let vertex_array = &self.objects[index].vertex_array;

                    vertex_array.bind();
        
                    let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
                    vertex_buffer.set_data(&vertices, gl::STATIC_DRAW);
        
                    let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
                    index_buffer.set_data(&indices, gl::STATIC_DRAW);
        
                    let pos_attrib = self.vertices_program.get_attrib_location("position")?;
                    set_attribute!(vertex_array, pos_attrib, _VerticeData::0);
                    
                    let color_attrib = self.vertices_program.get_attrib_location("color")?;
                    set_attribute!(vertex_array, color_attrib, _VerticeData::1);
        
                    VertexArray::unbind();
        
                    Ok(())
                }      
            },
            Err(_) => {
                unsafe {
                    let vertex_array = VertexArray::new();
                    vertex_array.bind();
        
                    let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
                    vertex_buffer.set_data(&vertices, gl::STATIC_DRAW);
        
                    let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
                    index_buffer.set_data(&indices, gl::STATIC_DRAW);
        
                    let pos_attrib = self.vertices_program.get_attrib_location("position")?;
                    set_attribute!(vertex_array, pos_attrib, _VerticeData::0);
                    
                    let color_attrib = self.vertices_program.get_attrib_location("color")?;
                    set_attribute!(vertex_array, color_attrib, _VerticeData::1);
        
                    VertexArray::unbind();
                    
                    OBJECTS.push(Object::new(key, vertex_array, None, indices.len() as i32, DrawType::default(), gl::TRIANGLES));

                    Ok(())
                }
            }
        }
    }

    pub fn draw_circle(&self, key: String, center: Position, radius: f32, color: Color, num_segments: Option<u32>) -> Result<()> {
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

        let found_object = self.objects.binary_search_by_key(&&key, | object | &object.key);

        match found_object {
            Ok(index) => {
                unsafe {
                    let vertex_array = &self.objects[index].vertex_array;

                    vertex_array.bind();

                    let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
                    vertex_buffer.set_data(&vertices, gl::STATIC_DRAW);
        
                    let pos_attrib = self.vertices_program.get_attrib_location("position")?;
                    set_attribute!(vertex_array, pos_attrib, _VerticeData::0);
                    
                    let color_attrib = self.vertices_program.get_attrib_location("color")?;
                    set_attribute!(vertex_array, color_attrib, _VerticeData::1);
        
                    VertexArray::unbind();
        
                    Ok(())
                }
            },
            Err(_) => {
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

                    OBJECTS.push(Object::new(key, vertex_array, None, vertices.len() as i32, DrawType::ARRAY, gl::TRIANGLE_FAN));

                    Ok(())
                }
            }
        }
    }

    pub fn draw_curved_line(&self, key: String, start: Position, end: Position, color: Color, num_segments: Option<u32>) -> Result<()> {
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

        let found_object = self.objects.binary_search_by_key(&&key, | object | &object.key);

        match found_object {
            Ok(index) => {
                unsafe {
                    let vertex_array = &self.objects[index].vertex_array;

                    vertex_array.bind();

                    let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
                    vertex_buffer.set_data(&vertices, gl::STATIC_DRAW);
        
                    let pos_attrib = self.vertices_program.get_attrib_location("position")?;
                    set_attribute!(vertex_array, pos_attrib, _VerticeData::0);
                    
                    let color_attrib = self.vertices_program.get_attrib_location("color")?;
                    set_attribute!(vertex_array, color_attrib, _VerticeData::1);
        
                    VertexArray::unbind();
        
                    Ok(())
                }
            },
            Err(_) => {
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
                    
                    OBJECTS.push(Object::new(key, vertex_array, None, vertices.len() as i32, DrawType::ARRAY, gl::LINE_STRIP));

                    Ok(())
                }
            }
        }
    }

    pub fn draw_line(&self, key: String, start: Position, end: Position, color: Color) -> Result<()> {    
        let start = convert_coordinates(start, &self.size);
        let end = convert_coordinates(end, &self.size);
        
        let vertices: [_VerticeData; 2] = [
            _VerticeData(start.get_vertice_position(None), color.get_vertices_color_in_f32()),
            _VerticeData(end.get_vertice_position(None), color.get_vertices_color_in_f32()),
        ];

        let found_object = self.objects.binary_search_by_key(&&key, | object | &object.key);

        match found_object {
            Ok(index) => {
                unsafe {
                    let vertex_array = &self.objects[index].vertex_array;

                    vertex_array.bind();
                    
                    let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
                    vertex_buffer.set_data(&vertices, gl::STATIC_DRAW);
        
                    let pos_attrib = self.vertices_program.get_attrib_location("position")?;
                    set_attribute!(vertex_array, pos_attrib, _VerticeData::0);
                    
                    let color_attrib = self.vertices_program.get_attrib_location("color")?;
                    set_attribute!(vertex_array, color_attrib, _VerticeData::1);
        
                    VertexArray::unbind();
        
                    Ok(())
                }
            },
            Err(_) => {
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

                    OBJECTS.push(Object::new(key, vertex_array, None, vertices.len() as i32, DrawType::ARRAY, gl::LINE_STRIP));

                    Ok(())
                }
            }
        }
    }

    pub fn load_image(&self, key: String, image_path: &str, position: Position, size: Size) -> Result<()> {
        let position = convert_coordinates(position, &self.size);
        let size = convert_size(size, &self.size);

        let texture_vertices: [_TextureVerticeData; 4] = [
            _TextureVerticeData(position.get_vertice_position(None), [0.0, 0.0]),
            _TextureVerticeData(position.get_vertice_position(Some(&Size { width: size.width, height: 0.0 })), [1.0, 0.0]),
            _TextureVerticeData(position.get_vertice_position(Some(&size)), [1.0, 1.0]),
            _TextureVerticeData(position.get_vertice_position(Some(&Size { width: 0.0, height: size.height })), [0.0, 1.0]),
        ];
        let indices: [i32; 6] = [0, 1, 2, 2, 3, 0];

        let found_object = self.objects.binary_search_by_key(&&key, | object | &object.key);

        match found_object {
            Ok(index) => {
                unsafe {
                    let vertex_array = &self.objects[index].vertex_array;

                    vertex_array.bind();

                    let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
                    vertex_buffer.set_data(&texture_vertices, gl::STATIC_DRAW);
        
                    let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
                    index_buffer.set_data(&indices, gl::STATIC_DRAW);

                    let pos_attrib = self.texture_program.get_attrib_location("position")?;
                    set_attribute!(vertex_array, pos_attrib, _TextureVerticeData::0);
                    
                    let color_attrib = self.texture_program.get_attrib_location("vertexTexCoord")?;
                    set_attribute!(vertex_array, color_attrib, _TextureVerticeData::1);
        
                    VertexArray::unbind();
        
                    Ok(())
                }
            },
            Err(_) => {
                unsafe {
                    let vertex_array = VertexArray::new();
                    vertex_array.bind();
        
                    let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
                    vertex_buffer.set_data(&texture_vertices, gl::STATIC_DRAW);
        
                    let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
                    index_buffer.set_data(&indices, gl::STATIC_DRAW);
        
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

                    OBJECTS.push(Object::new(key, vertex_array, Some(texture), indices.len() as i32, DrawType::default(), gl::TRIANGLES));
        
                    Ok(())
                }
            }
        }
    }

    pub fn update(&mut self) {
        unsafe {
            for _ in 0..OBJECTS.len() {
                let object = OBJECTS.pop();

                if let Some(object) = object {
                    self.objects.push(object);
                }
            }
        }
        

        self.objects.sort_by(| first, second | first.key.partial_cmp(&second.key).unwrap());
    }

    pub fn render(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
            if let Some(background_image) = &self.background.image {
                self.texture_program.apply();
                background_image.texture.bind();
                background_image.vertex_array.bind();

                gl::DrawElements(gl::TRIANGLES, background_image.count, gl::UNSIGNED_INT, ptr::null()); 
            } else {
                let (red, green, blue, alpha) = self.background.color;
    
                gl::ClearColor(red, green, blue, alpha);
            }

            dbg!(&self.objects);
            
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
