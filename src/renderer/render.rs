use std::{collections::HashMap, path::Path, ptr};

use gl::types::GLenum;
use glam::{Mat4, Vec3};
use glfw::{Action, MouseButton};

use crate::{game::character::Direction, library::{constants::TWICE_PI, utils::{absolute_f32, calc_control_point, calc_equidistant_points, calc_mid_point, calc_mid_point_position_of_quadrilateral_shape, calc_mid_point_position_of_triangle, convert_angle_to_radians, convert_coordinates, convert_size, create_translate, is_cursor_in_button, length_of_line}}, set_attribute};

use super::{buffer::Buffer, button::{Button, ButtonAction, OnHoverStyles}, color::{Color, ColorType}, error::Result, program::Program, shader::Shader, source_code::{TEXTURE_FRAGMENT_SHADER_SOURCE, TEXTURE_VERTEX_SHADER_SOURCE, VERTICES_FRAGMENT_SHADER_SOURCE, VERTICES_VERTEX_SHADER_SOURCE}, styles::{Padding, Size}, text::{calculate_text_size, calculate_word_width, generated_characters_bitmap, Character}, texture::Texture, vertex_array::VertexArray, vertice::{Position, Vertice, _TextureVerticeData, _VerticeData}};

#[derive(Debug, PartialEq)]
struct Object<'a> {
    vertices: Vec<_VerticeData>,
    indices: Option<Vec<u32>>,
    texture_image_path: Option<&'a str>,
    texture_opacity: f32,
    mode: GLenum,
    transform_matrix: Mat4
}

impl<'a> Object<'a> {
    fn new(vertices: Vec<_VerticeData>, indices: Option<Vec<u32>>, texture_image_path: Option<&'a str>, texture_opacity: Option<f32>, mode: GLenum) -> Self {
        Self {
            vertices,
            indices,
            mode,
            texture_image_path,
            texture_opacity: texture_opacity.unwrap_or(1.0),
            transform_matrix: Mat4::IDENTITY
        }
    }
}

impl Object<'_> {
    pub fn scale(&mut self, scale: Vec3) {
        let scaling_matrix = Mat4::from_scale(scale);

        self.transform_matrix = self.transform_matrix * scaling_matrix;
    }

    pub fn rotate(&mut self, angle: f32, rotation_point: Position) {
        let translate_to_origin = Mat4::from_translation(-glam::vec3(rotation_point.x, rotation_point.y, 0.0));

        let rotation_matrix = Mat4::from_axis_angle(glam::vec3(0.0, 0.0, 1.0), convert_angle_to_radians(angle));

        let translate_back = Mat4::from_translation(glam::vec3(rotation_point.x, rotation_point.y, 0.0));

        self.transform_matrix = self.transform_matrix * translate_back * rotation_matrix * translate_to_origin;
    }

    pub fn translate(&mut self, translate: Vec3) {
        let translation_matrix = Mat4::from_translation(translate);

        self.transform_matrix = self.transform_matrix * translation_matrix;
    }
}

struct RenderableCharacter {
    character: char,
    vertices: [_VerticeData; 4],
    indices: [u32; 6],
    color: (f32, f32, f32)
}

impl RenderableCharacter {
    pub fn new(character: char, vertices: [_VerticeData; 4], color: (f32, f32, f32)) -> Self {
        Self {
            character,
            vertices,
            indices: [0, 1, 2, 2, 3, 0],
            color
        }
    }
}

#[derive(Debug)]
struct Image {
    vertex_array: VertexArray,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    texture: Texture,
    count: i32
}

impl Image {
    fn new(vertex_array: VertexArray, vertex_buffer: Buffer, index_buffer: Buffer, texture: Texture, count: i32) -> Self {
        Self {
            vertex_array,
            vertex_buffer,
            index_buffer,
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

pub struct MouseInteraction {
    pub cursor_position: Position,
    pub mouse_button: MouseButton,
    pub action: Action
}

impl MouseInteraction {
    pub fn new(cursor_position: Position, mouse_button: MouseButton, action: Action) -> Self {
        Self {
            cursor_position,
            mouse_button,
            action
        }
    }
}

pub struct ButtonProps<'a> {
    pub position: Position,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub padding: Padding,
    pub bg_color: Color,
    pub text: String,
    pub text_scale: f32,
    pub text_color: Color,
    pub on_hover_styles: OnHoverStyles,
    pub click_action: ButtonAction,
    pub on_hover: Box<dyn FnMut() + 'a>,
    pub on_hover_release: Box<dyn FnMut() + 'a>,
    pub on_click: Box<dyn FnMut() + 'a>
}

pub struct Render<'a> {
    size: Size,
    vertices_program: Program,
    texture_program: Program,
    vertex_array: VertexArray,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    background: Background,
    characters: HashMap<char, Character>,
    images: HashMap<&'a str, Texture>,
    objects: Vec<Object<'a>>,
    renderable_characters: Vec<RenderableCharacter>,
    buttons: Vec<Button<'a>>,
    mouse_interaction: Option<MouseInteraction>,
    was_hovering_on_button: (bool, Option<usize>),
    button_click_action: ButtonAction
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

            let vertex_array = VertexArray::new();
            
            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            vertex_buffer.set_empty(0, gl::DYNAMIC_DRAW);

            let index_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
            index_buffer.set_empty(0, gl::DYNAMIC_DRAW);

            let pos_attrib = vertices_program.get_attrib_location("position")?;
            set_attribute!(vertex_array, pos_attrib, _VerticeData::0);
            
            let color_attrib = vertices_program.get_attrib_location("color")?;
            set_attribute!(vertex_array, color_attrib, _VerticeData::1);

            let characters = generated_characters_bitmap(None)?;

            vertex_buffer.unbind();
            index_buffer.unbind();
            Texture::unbind_all();
            VertexArray::unbind();

            Ok(Self {
                size,
                vertices_program,
                texture_program,
                vertex_array,
                vertex_buffer,
                index_buffer,
                background: Background::default(),
                characters,
                images: HashMap::new(),
                objects: Vec::new(),
                renderable_characters: Vec::new(),
                buttons: Vec::new(),
                mouse_interaction: None,
                was_hovering_on_button: (false, None),
                button_click_action: ButtonAction::None,
            })
        }
    }
}

impl<'a> Render<'a> {
    pub fn get_size(&self) -> Size {
        self.size
    }

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
                texture.load_image(&Path::new(image_path))?;
    
                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                gl::Enable(gl::BLEND);
                
                vertex_buffer.unbind();
                index_buffer.unbind();
                texture.unbind();
                VertexArray::unbind();
                
                self.background.image = Some(Image::new(vertex_array, vertex_buffer, index_buffer, texture, background_indices.len() as i32));
            }
        }

        Ok(())
    }

    pub fn set_mouse_interaction(&mut self, interaction: Option<MouseInteraction>) {
        self.mouse_interaction = interaction;
    }

    pub fn draw_triangle(&mut self, first_point: Vertice, second_point: Vertice, third_point: Vertice, scale: Option<f32>, translate: Option<Position>, rotate: Option<f32>) {
        let vertices_data = vec![first_point.get_vertice_data(&self.size), second_point.get_vertice_data(&self.size), third_point.get_vertice_data(&self.size)];
        let indices = vec![0, 1, 2];

        let mut object = Object::new(vertices_data, Some(indices), None, None, gl::TRIANGLES);

        if let Some(translate) = translate {
            let translate = create_translate(translate, &self.size);

            object.translate(glam::vec3(translate.x, translate.y, 0.0));
        }

        if let Some(rotate) = rotate {
            object.rotate(rotate, convert_coordinates(calc_mid_point_position_of_triangle(first_point.0, second_point.0, third_point.0), &self.size));
        }

        if let Some(scale) = scale {
            assert!(scale > 0.0, "scale must be a positive number");

            object.scale(glam::vec3(scale, scale, 1.0));
        }

        self.objects.push(object);
    }

    pub fn draw_rectangle(&mut self, position: Position, size: Size, color: Color, scale: Option<f32>, translate: Option<Position>, rotate: Option<f32>) {
        let position = convert_coordinates(position, &self.size);
        let size = convert_size(size, &self.size);

        let vertices_data = vec![
            _VerticeData(position.to_position_array(), color.get_vertices_color_in_f32()),
            _VerticeData(position.get_position_from_size(&Size { width: size.width, height: 0.0 }).to_position_array(), color.get_vertices_color_in_f32()),
            _VerticeData(position.get_position_from_size(&size).to_position_array(), color.get_vertices_color_in_f32()),
            _VerticeData(position.get_position_from_size(&Size { width: 0.0, height: size.height }).to_position_array(), color.get_vertices_color_in_f32()),
        ];
        let indices = vec![0, 1, 2, 2, 3, 0];

        let mut object = Object::new(vertices_data, Some(indices), None, None, gl::TRIANGLES);

        if let Some(translate) = translate {
            let translate = create_translate(translate, &self.size);

            object.translate(glam::vec3(translate.x, translate.y, 0.0));
        }

        if let Some(rotate) = rotate {
            object.rotate(rotate, calc_mid_point_position_of_quadrilateral_shape(&position, &size));
        }

        if let Some(scale) = scale {
            assert!(scale > 0.0, "scale must be a positive number");

            object.scale(glam::vec3(scale, scale, 1.0));
        }

        self.objects.push(object);
    }

    pub fn draw_geometric_object(&mut self, center: Position, radius: f32, color: Color, num_segments: Option<u32>, scale: Option<f32>, translate: Option<Position>, rotate: Option<f32>) {
        assert!(radius > 0.0, "radius must be positive number");
        
        let num_segments = num_segments.unwrap_or(360);

        assert!(num_segments > 0, "num_segments must be a positive number");

        let center = convert_coordinates(center, &self.size);

        let mut vertices_data = Vec::with_capacity(num_segments as usize);
        let mut indices = Vec::with_capacity(num_segments as usize * 3);
        
        let radius_x = (radius * 2.0) / self.size.width;
        let radius_y = (radius * 2.0) / self.size.height; 

        for num in 0..=num_segments {
            let theta = TWICE_PI * (num as f32) / (num_segments as f32);
            
            let x = theta.cos() * radius_x + center.x;
            let y = theta.sin() * radius_y + center.y;

            vertices_data.push(_VerticeData([x, y], color.get_vertices_color_in_f32()));

            indices.extend_from_slice(&[0, num, num+1]);
        }

        let mut object = Object::new(vertices_data, Some(indices), None, None, gl::TRIANGLE_FAN);

        if let Some(translate) = translate {
            let translate = create_translate(translate, &self.size);

            object.translate(glam::vec3(translate.x, translate.y, 0.0));
        }

        if let Some(rotate) = rotate {
            object.rotate(rotate, center);
        }

        if let Some(scale) = scale {
            assert!(scale > 0.0, "scale must be a positive number");

            object.scale(glam::vec3(scale, scale, 1.0));
        }

        self.objects.push(object);
    }

    pub fn draw_curved_line(&mut self, start: Position, end: Position, color: Color, num_segments: Option<u32>, scale: Option<f32>, translate: Option<Position>, rotate: Option<f32>) {
        let num_segments = num_segments.unwrap_or(length_of_line(&start, &end) as u32);

        let start = convert_coordinates(start, &self.size);
        let end = convert_coordinates(end, &self.size);

        let mut vertices_data = Vec::with_capacity(num_segments as usize);
        let mut indices = Vec::with_capacity(num_segments as usize);
        
        let control_point = calc_control_point(&start, &end);

        for num in 0..=num_segments {
            let t = num as f32 / num_segments as f32;

            let x = (1.0 - t).powi(2) * start.x + 2.0 * (1.0 - t) * t * control_point.x + t.powi(2) * end.x;
            let y = (1.0 - t).powi(2) * start.y + 2.0 * (1.0 - t) * t * control_point.y + t.powi(2) * end.y;

            vertices_data.push(_VerticeData([x, y], color.get_vertices_color_in_f32()));

            indices.push(num);
        }

        let mut object = Object::new(vertices_data, Some(indices), None, None, gl::LINE_STRIP);

        if let Some(translate) = translate {
            let translate = create_translate(translate, &self.size);

            object.translate(glam::vec3(translate.x, translate.y, 0.0));
        }

        if let Some(rotate) = rotate {
            object.rotate(rotate, calc_mid_point(&start, &end));
        }

        if let Some(scale) = scale {
            assert!(scale > 0.0, "scale must be a positive number");

            object.scale(glam::vec3(scale, scale, 1.0));
        }

        self.objects.push(object);
    }

    pub fn draw_line(&mut self, start: Position, end: Position, color: Color, scale: Option<f32>, translate: Option<Position>, rotate: Option<f32>) {    
        let start = convert_coordinates(start, &self.size);
        let end = convert_coordinates(end, &self.size);
        
        let vertices_data = vec![
            _VerticeData(start.to_position_array(), color.get_vertices_color_in_f32()),
            _VerticeData(end.to_position_array(), color.get_vertices_color_in_f32()),
        ];
        let indices = vec![0, 1];

        let mut object = Object::new(vertices_data, Some(indices), None, None, gl::LINE_STRIP);

        if let Some(translate) = translate {
            let translate = create_translate(translate, &self.size);

            object.translate(glam::vec3(translate.x, translate.y, 0.0));
        }

        if let Some(rotate) = rotate {
            object.rotate(rotate, calc_mid_point(&start, &end));
        }

        if let Some(scale) = scale {
            assert!(scale > 0.0, "scale must be a positive number");

            object.scale(glam::vec3(scale, scale, 1.0));
        }

        self.objects.push(object);
    }

    pub fn load_image(&mut self, image_path: &'a str, position: Position, size: Size, flip: bool, opacity: Option<f32>, scale: Option<f32>, translate: Option<Position>, rotate: Option<f32>) -> Result<()> {
        let position = convert_coordinates(position, &self.size);
        let size = convert_size(size, &self.size);

        let flip = if flip {  1.0 } else { 0.0 };

        let vertices_data = vec![
            _VerticeData(position.to_position_array(), [absolute_f32(flip - 0.0), 0.0, 0.0, 0.0]),
            _VerticeData(position.get_position_from_size(&Size { width: size.width, height: 0.0 }).to_position_array(), [absolute_f32(flip - 1.0), 0.0, 0.0, 0.0]),
            _VerticeData(position.get_position_from_size(&size).to_position_array(), [absolute_f32(flip - 1.0), 1.0, 0.0, 0.0]),
            _VerticeData(position.get_position_from_size(&Size { width: 0.0, height: size.height }).to_position_array(), [absolute_f32(flip - 0.0), 1.0, 0.0, 0.0]),
        ];
        let indices = vec![0, 1, 2, 2, 3, 0];

        let found_texture = self.images.get(image_path);

        if found_texture.is_some() {
            let mut object = Object::new(vertices_data, Some(indices), Some(image_path), opacity, gl::TRIANGLES);

            if let Some(translate) = translate {
                let translate = create_translate(translate, &self.size);

                object.translate(glam::vec3(translate.x, translate.y, 0.0));
            }

            if let Some(rotate) = rotate {
                object.rotate(rotate, calc_mid_point_position_of_quadrilateral_shape(&position, &size));
            }

            if let Some(scale) = scale {
                assert!(scale > 0.0, "scale must be a positive number");

                object.scale(glam::vec3(scale, scale, 1.0));
            }

            self.objects.push(object);

            return Ok(());
        }
        
        unsafe {
            let texture = Texture::new();
            texture.set_wrapping(gl::REPEAT);
            texture.set_filtering(gl::LINEAR);
            texture.load_image(&Path::new(image_path))?;

            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::BLEND);

            self.images.insert(image_path, texture);
            
            let mut object = Object::new(vertices_data, Some(indices), Some(image_path), opacity, gl::TRIANGLES);

            if let Some(translate) = translate {
                let translate = create_translate(translate, &self.size);

                object.translate(glam::vec3(translate.x, translate.y, 0.0));
            }

            if let Some(rotate) = rotate {
                object.rotate(rotate, calc_mid_point_position_of_quadrilateral_shape(&position, &size));
            }

            if let Some(scale) = scale {
                assert!(scale > 0.0, "scale must be a positive number");

                object.scale(glam::vec3(scale, scale, 1.0));
            }

            self.objects.push(object);
        }

        Ok(())
    }

    pub fn display_text(&mut self, text: &str, start_position: Position, scale: f32, text_max_width: Option<f32>, color: Color) -> Result<Size> {
        assert!(scale > 0.0, "scale must be a positive number");
        
        let mut min_y = start_position.y;

        let start_position = convert_coordinates(start_position, &self.size);

        let mut text_size = Size { width: 0.0, height: 0.0 };
        if let Some(width) = text_max_width {
            text_size.width = width;
        }

        let max_width = (text_max_width.unwrap_or(0.0) * 2.0) / self.size.width;

        let (r, g, b, ..) = color.get_color_in_f32();

        let lines: Vec<&str> = text.split('\n').collect();

        let mut line_height = 0.0;

        for line in lines {
            let mut line_width = 0.0;
            let mut width_offset = 0.0;
            let mut max_height = 0.0;
            let mut prev_height = 0.0;
            let mut height_offset = 0.0;
            let mut is_new_word = false;

            for (idx, ch) in line.chars().enumerate() {
                if ch == ' ' {
                    width_offset += 0.03 * scale;
                    line_width += (0.03 * self.size.width * scale) / 2.0;
                    is_new_word = true;
                    
                    continue;
                }

                if max_width > 0.0 && is_new_word {
                    if let Some(found_index) = line[idx..].find(' ') {
                        let found_index = found_index + idx;

                        let word_width = calculate_word_width(&self.characters, &line[idx..found_index], scale, self.size.width);
                        
                        if start_position.x + width_offset + word_width >= start_position.x + max_width {
                            line_height += max_height + 0.09;
                            width_offset = 0.0;
                            max_height = 0.0;
                            prev_height = 0.0;
                            height_offset = 0.0;
                            is_new_word = false;
                        }
                    } else {
                        let word_width = calculate_word_width(&self.characters, &line[idx..], scale, self.size.width);

                        if start_position.x + width_offset + word_width >= start_position.x + max_width {
                            line_height += max_height + 0.09;
                            width_offset = 0.0;
                            max_height = 0.0;
                            prev_height = 0.0;
                            height_offset = 0.0;
                            is_new_word = false;
                        }
                    }
                }

                assert!(self.characters.get(&ch) != None, "character must exist, provided: {}", ch);

                let character = self.characters.get(&ch).unwrap();

                let character_size = convert_size(Size { width: character.size.width * scale, height: character.size.height * scale }, &self.size);

                let offset_y = ((character.size.height - character.offset.y) * scale * 2.0) / self.size.height;
                
                if max_height == 0.0 {
                    max_height = character_size.height;
                } else {
                    if (character.size.height - character.offset.y) > 0.0 {
                        if max_height < (character_size.height - offset_y) {
                            max_height = character_size.height - offset_y;
                        }
                    } else {
                        if max_height < character_size.height {
                            max_height = character_size.height;
                        }
                    }
                }

                if prev_height == 0.0 {
                    prev_height = character_size.height;
                }
                
                if is_new_word {
                    if max_height == character_size.height {
                        height_offset += prev_height - max_height;
                    } else {
                        height_offset += prev_height - character_size.height;   
                    }
                } else {
                    height_offset += prev_height - character_size.height;
                }

                let character_start_position = Position { x: start_position.x + width_offset, y: start_position.y - line_height - height_offset - offset_y };

                let characher_start_y = (character_start_position.y * self.size.height) / 2.0;
                if min_y > characher_start_y {
                    min_y = characher_start_y;
                }

                let character_end_y = characher_start_y + character.size.height;
                if text_size.height < (character_end_y - min_y) {
                    text_size.height = character_end_y - min_y;
                }

                let vertices_data = [
                    _VerticeData(character_start_position.to_position_array(), [0.0, 0.0, 0.0, 0.0]),
                    _VerticeData(character_start_position.get_position_from_size(&Size { width: character_size.width, height: 0.0 }).to_position_array(), [1.0, 0.0, 0.0, 0.0]),
                    _VerticeData(character_start_position.get_position_from_size(&character_size).to_position_array(), [1.0, 1.0, 0.0, 0.0]),
                    _VerticeData(character_start_position.get_position_from_size(&Size { width: 0.0, height: character_size.height }).to_position_array(), [0.0, 1.0, 0.0, 0.0]),
                ];

                self.renderable_characters.push(RenderableCharacter::new(ch, vertices_data, (r, g, b)));

                width_offset += ((character.size.width as f32 + character.offset.x) * scale * 2.0) / self.size.width;
                prev_height = character_size.height;

                line_width += (character.size.width + character.offset.x) * scale;
                
                if is_new_word {
                    is_new_word = false;
                }
            }

            if text_max_width.is_none() && text_size.width < line_width {
                text_size.width = line_width;
            }

            line_height += max_height + 0.08;
        }

        Ok(text_size)
    }

    pub fn draw_equidistant_from_angle_and_length(&mut self, apex: Position, angle: f32, line_length: f32, angle_direction: Direction, color: Color) {
        let (first_point, second_point, apex) = calc_equidistant_points(apex, angle, line_length, angle_direction);

        self.draw_line(apex, first_point, color, None, None, None);
        self.draw_line(apex, second_point, color, None, None, None);
        self.draw_line(first_point, second_point, color, None, None, None); 
    }

    pub fn display_button(&mut self, button_props: ButtonProps<'a>) {
        let mut button = Button::new(
            self.buttons.len() + 1,
            button_props.position,
            button_props.width,
            button_props.height,
            calculate_text_size(&self.characters, &button_props.text, button_props.position, button_props.width, button_props.text_scale, self.size),
            button_props.padding,
            button_props.bg_color,
            button_props.text,
            button_props.text_scale,
            button_props.text_color,
            button_props.on_hover_styles,
            button_props.click_action
        );
        button.on_hover(button_props.on_hover);
        button.on_hover_release(button_props.on_hover_release);
        button.on_click(button_props.on_click);
        
        self.buttons.push(button);
    }

    pub fn handle_buttons_events(&mut self, real_cursor_position: Position) -> Result<()> {
        for button in self.buttons.iter_mut() {
            if is_cursor_in_button(button.get_position_with_padding(), button.get_size(), real_cursor_position) {
                self.was_hovering_on_button = (true, Some(button.get_id()));
                button.set_is_hovering(true);
                
                button.hover_call();

                if let Some(mouse_interaction) = &self.mouse_interaction {
                    if mouse_interaction.mouse_button == MouseButton::Button1 {
                        if mouse_interaction.action == Action::Release {
                            self.button_click_action = button.get_click_action();

                            button.click_call();
                        }
                    }
                }
            } else {
                if self.was_hovering_on_button.0 && self.was_hovering_on_button.1 == Some(button.get_id()) {
                    button.on_hover_release_call();

                    self.was_hovering_on_button = (false, None);
                }

                button.set_is_hovering(false);
            }
        }
 
       for i in 0..self.buttons.len() {
            let button_ptr = &mut self.buttons[i] as *const Button;

            unsafe {
                let button = &*button_ptr;

                button.draw(self)?;
            }
        }

        Ok(())
    }

    pub fn get_button_click_action(&self) -> ButtonAction {
        self.button_click_action
    }

    pub fn render(&mut self) -> Result<()> {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            if let Some(background_image) = &self.background.image {
                self.texture_program.apply();
                background_image.vertex_array.bind();
                background_image.vertex_buffer.bind();
                background_image.index_buffer.bind();

                self.texture_program.set_transform_matrix_uniform(Mat4::IDENTITY)?;
                self.texture_program.set_opacity_uniform_to_texture(1.0)?;
                self.texture_program.set_int_uniform("texture0", 0)?;
                background_image.texture.activate(gl::TEXTURE0);

                gl::DrawElements(gl::TRIANGLES, background_image.count, gl::UNSIGNED_INT, ptr::null()); 
            }

            let (red, green, blue, alpha) = self.background.color;

            gl::ClearColor(red, green, blue, alpha);

            for object in self.objects.iter() {
                self.vertex_array.bind();

                self.vertex_buffer.set_data(&object.vertices, gl::DYNAMIC_DRAW);

                if let Some(texture_image_path) = object.texture_image_path {
                    assert!(self.images.get(texture_image_path) != None, "texture must exist");

                    let texture = self.images.get(texture_image_path).unwrap();

                    self.texture_program.apply();
                    self.texture_program.set_transform_matrix_uniform(object.transform_matrix)?;
                    self.texture_program.set_bool_uniform("isText", 0)?;
                    self.texture_program.set_opacity_uniform_to_texture(object.texture_opacity)?;
                    self.texture_program.set_int_uniform("texture0", 0)?;
                    texture.activate(gl::TEXTURE0);
                } else {
                    self.vertices_program.apply();
                    self.vertices_program.set_transform_matrix_uniform(object.transform_matrix)?;
                }

                if let Some(indices) = &object.indices {
                    self.index_buffer.set_data(indices, gl::DYNAMIC_DRAW);
                    
                    gl::DrawElements(object.mode, indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
                } else {
                    gl::DrawArrays(object.mode, 0, object.vertices.len() as i32);
                }

                self.vertex_buffer.unbind();
                self.index_buffer.unbind();
                Texture::unbind_all();
                VertexArray::unbind();
            }

            for randerable_character in self.renderable_characters.iter() {
                assert!(self.characters.get(&randerable_character.character) != None, "character must exist");

                self.texture_program.apply();
                self.texture_program.set_transform_matrix_uniform(Mat4::IDENTITY)?;
                self.texture_program.set_bool_uniform("isText", 1)?;
                self.texture_program.set_color_data_uniform("textColor", randerable_character.color)?;

                self.vertex_array.bind();

                self.vertex_buffer.set_data(&randerable_character.vertices, gl::DYNAMIC_DRAW);
                self.index_buffer.set_data(&randerable_character.indices, gl::DYNAMIC_DRAW);

                let character = self.characters.get(&randerable_character.character).unwrap();

                self.texture_program.set_int_uniform("texture0", 0)?;
                character.texture.activate(gl::TEXTURE0);

                gl::DrawElements(gl::TRIANGLES, randerable_character.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());

                self.texture_program.set_bool_uniform("isText", 0)?;
                
                self.vertex_buffer.unbind();
                self.index_buffer.unbind();
                Texture::unbind_all();
                VertexArray::unbind();
            }
        }

        self.objects.clear();
        self.renderable_characters.clear();
        self.buttons.clear();
        self.button_click_action = ButtonAction::None;
        
        Ok(())
    }    
}
