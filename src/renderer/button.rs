use derivative::Derivative;

use crate::library::utils::calc_button_text_info_with_padding;

use super::{color::Color, error::Result, render::Render, styles::{Padding, Size}, vertice::Position};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Button {
    position: Position,
    size: Size,
    padding: Padding,
    bg_color: Color,
    text: String,
    text_start_position: Position,
    text_max_width: f32,
    text_scale: f32,
    text_color: Color,

    #[derivative(Debug="ignore")]
    on_hover: Box<dyn Fn()>,

    #[derivative(Debug="ignore")]
    on_click: Box<dyn Fn()>
}

impl Button {
    pub fn new(position: Position, size: Size, padding: Padding, bg_color: Color, text: String, text_scale: f32, text_color: Color) -> Self {
        let (text_start_position, text_max_width) = calc_button_text_info_with_padding(&position, &size, &padding);

        Self {
            position,
            size,
            padding,
            bg_color,
            text,
            text_color,
            text_scale,
            text_max_width,
            text_start_position,
            on_hover: Box::new(|| {}),
            on_click: Box::new(|| { print!("here working!!\n") }) 
        }
    }
}

impl Button {
    pub fn draw(&self, render: &mut Render<'_>) -> Result<()> {
        render.draw_rectangle(self.get_position(), self.get_size(), self.get_bg_color(), None, None, None);
        render.display_text(self.get_text(), self.text_start_position, self.get_text_scale(), Some(self.text_max_width), self.get_text_color())?;

        Ok(())
    }

    pub fn on_hover<F>(&mut self, func: F)
        where F: Fn() + 'static
    {
        self.on_hover = Box::new(func);
    }

    pub fn on_hover_func(&self) -> &dyn Fn() {
        &*self.on_hover
    }

    pub fn hover_call(&self) {
        (self.on_hover)();
    }

    pub fn on_click<F>(&mut self, func: F)
        where F: Fn() + 'static
    {
        self.on_click = Box::new(func);
    }

    pub fn on_click_func(&self) -> &dyn Fn() {
        &*self.on_click
    }

    pub fn click_call(&self) {
        (self.on_click)();
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn set_position(&mut self, new_position: Position) {
        self.position = new_position;

        let (new_text_start_position, new_max_width) = calc_button_text_info_with_padding(&new_position, &self.size, &self.padding);
        self.text_start_position = new_text_start_position;
        self.text_max_width = new_max_width;
    }

    pub fn get_size(&self) -> Size {
        self.size
    }

    pub fn get_padding(&self) -> Padding {
        self.padding
    }

    pub fn set_padding(&mut self, new_padding: Padding) {
        self.padding = new_padding;

        let (new_text_start_position, new_max_width) = calc_button_text_info_with_padding(&self.get_position(), &self.size, &self.padding);
        self.text_start_position = new_text_start_position;
        self.text_max_width = new_max_width;
    }

    pub fn get_bg_color(&self) -> Color {
        self.bg_color
    }

    pub fn set_bg_color(&mut self, new_color: Color) {
        self.bg_color = new_color;
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }

    pub fn set_text(&mut self, new_text: String) {
        self.text = new_text;
    }

    pub fn get_text_color(&self) -> Color {
        self.text_color
    }

    pub fn set_text_color(&mut self, new_color: Color) {
        self.text_color = new_color;
    }

    pub fn get_text_scale(&self) -> f32 {
        self.text_scale
    }

    pub fn set_text_scale(&mut self, new_scale: f32) {
        self.text_scale = new_scale;
    }

    pub fn get_text_info(&self) -> (Position, f32) {
        (self.text_start_position, self.text_max_width)
    }
}

