use derivative::Derivative;

use crate::library::utils::calc_button_info_with_padding;

use super::{color::Color, error::Result, render::Render, styles::{Padding, Size}, vertice::Position};

#[derive(Debug, Clone, Copy)]
pub enum ButtonAction {
    RetryLevel,
    Exit,
    NextLevel,
    BuyStoreItem(usize),
    None,
}

#[derive(Debug)]
pub struct OnHoverStyles {
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub padding: Option<Padding>,
    pub bg_color: Option<Color>,
    pub text_scale: Option<f32>,
    pub text_color: Option<Color>,
}

impl Default for OnHoverStyles {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            padding: None,
            bg_color: None,
            text_scale: None,
            text_color: None
        }
    }
}

pub struct OnHoverStylesBuilder {
    inner: OnHoverStyles,
}

impl OnHoverStylesBuilder {
    pub fn new() -> Self {
        Self {
            inner: OnHoverStyles::default()
        }
    }
}

impl OnHoverStylesBuilder {
    pub fn width(mut self, width: f32) -> Self {
        self.inner.width = Some(width);

        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.inner.height = Some(height);

        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.inner.padding = Some(padding);

        self
    }

    pub fn bg_color(mut self, bg_color: Color) -> Self {
        self.inner.bg_color = Some(bg_color);

        self
    }

    pub fn text_scale(mut self, text_scale: f32) -> Self {
        self.inner.text_scale = Some(text_scale);

        self
    }

    pub fn text_color(mut self, text_color: Color) -> Self {
        self.inner.text_color = Some(text_color);

        self
    }

    pub fn build(self) -> OnHoverStyles {
        self.inner
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Button<'a> {
    id: usize,
    position: Position,
    text_size: Size,
    width: Option<f32>,
    height: Option<f32>,
    padding: Padding,
    bg_color: Color,
    text: String,
    text_scale: f32,
    text_color: Color,
    on_hover_styles: OnHoverStyles,
    is_hovering: bool,
    click_action: ButtonAction,

    #[derivative(Debug="ignore")]
    on_hover: Box<dyn FnMut() + 'a>,

    #[derivative(Debug="ignore")]
    on_hover_release: Box<dyn FnMut() + 'a>,

    #[derivative(Debug="ignore")]
    on_click: Box<dyn FnMut() + 'a>
}

impl Button<'_> {
    pub fn new(id: usize, position: Position, width: Option<f32>, height: Option<f32>, text_size: Size, padding: Padding, bg_color: Color, text: String, text_scale: f32, text_color: Color, on_hover_styles: OnHoverStyles, click_action: ButtonAction) -> Self {
        Self {
            id,
            position,
            text_size,
            width,
            height,
            padding,
            bg_color,
            text,
            text_color,
            text_scale,
            on_hover_styles,
            is_hovering: false,
            click_action,
            on_hover: Box::new(|| {}),
            on_hover_release: Box::new(|| {}),
            on_click: Box::new(|| {}) 
        }
    }
}

impl<'a> Button<'a> {
    pub fn draw(&self, render: &mut Render<'_>) -> Result<()> {
        let mut size = Size { width: 0.0, height: 0.0 };

        let style_size = self.get_style_size();
        if let Some(style_width) = style_size.width {
            size.width = style_width;
        }
        if let Some(style_height) = style_size.height {
            size.height = style_height;
        }

        let text_size = render.display_text(self.get_text(), self.get_position(), self.get_text_scale(), style_size.width, self.get_text_color())?;
        
        if style_size.width.is_none() { 
            size.width = text_size.width;
        }

        if style_size.height.is_none() {
            size.height = text_size.height;
        }

        let (button_position, button_size) = calc_button_info_with_padding(&self.get_position(), &size, &self.get_padding());
        render.draw_rectangle(button_position, button_size, self.get_bg_color(), None, None, None);

        Ok(())
    }
    
    pub fn on_hover<F>(&mut self, func: F)
        where F: FnMut() + 'a
    {
        self.on_hover = Box::new(func);
    }

    pub fn on_hover_func(&self) -> &dyn FnMut() {
        &*self.on_hover
    }

    pub fn hover_call(&mut self) {
        (self.on_hover)();
    }

    pub fn on_hover_release<F>(&mut self, func: F)
        where F: FnMut() + 'a
    {
        self.on_hover_release = Box::new(func);
    }

    pub fn on_hover_release_func(&self) -> &dyn FnMut() {
        &*self.on_hover_release
    }

    pub fn on_hover_release_call(&mut self) {
        (self.on_hover_release)();
    }

    pub fn on_click<F>(&mut self, func: F)
        where F: FnMut() + 'a
    {
        self.on_click = Box::new(func);
    }

    pub fn on_click_func(&self) -> &dyn FnMut() {
        &*self.on_click
    }

    pub fn click_call(&mut self) {
        (self.on_click)();
    }

    pub fn get_click_action(&self) -> ButtonAction {
        self.click_action
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn get_position_with_padding(&self) -> Position {
        let size = self.get_size_without_padding();
        
        let (button_position, ..) = calc_button_info_with_padding(&self.get_position(), &size, &self.get_padding()); 
        button_position
    }

    pub fn set_position(&mut self, new_position: Position) {
        self.position = new_position;
    }

    pub fn get_style_size(&self) -> Size<Option<f32>> {
        if self.get_is_hovering() {
            return Size { width: self.on_hover_styles.width, height: self.on_hover_styles.height };
        }

        Size { width: self.width, height: self.height }
    }

    pub fn get_size_without_padding(&self) -> Size {
        let style_size = self.get_style_size();

        let mut size = Size { width: style_size.width.unwrap_or(0.0), height: style_size.height.unwrap_or(0.0) };
        if style_size.width.is_none() { 
            size.width = self.text_size.width;
        }

        if style_size.height.is_none() {
            size.height = self.text_size.height;
        }

        size
    }

    pub fn get_size(&self) -> Size {
        let style_size = self.get_style_size();

        let mut size = Size { width: style_size.width.unwrap_or(0.0), height: style_size.height.unwrap_or(0.0) };
        if style_size.width.is_none() { 
            size.width = self.text_size.width;
        }

        if style_size.height.is_none() {
            size.height = self.text_size.height;
        }

        let (_, button_size) = calc_button_info_with_padding(&self.get_position(), &size, &self.get_padding());

        button_size
    }

    pub fn get_padding(&self) -> Padding {
        if self.get_is_hovering() {
            if let Some(hover_padding) = self.on_hover_styles.padding {
                return hover_padding;
            }
        } 

        self.padding
    }

    pub fn set_padding(&mut self, new_padding: Padding) {
        self.padding = new_padding;
    }

    pub fn get_bg_color(&self) -> Color {
        if self.get_is_hovering() {
            if let Some(hover_bg_color) = self.on_hover_styles.bg_color {
                return hover_bg_color;
            }
        }

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
        if self.get_is_hovering() {
            if let Some(hover_text_color) = self.on_hover_styles.text_color {
                return hover_text_color;
            }
        }

        self.text_color
    }

    pub fn set_text_color(&mut self, new_color: Color) {
        self.text_color = new_color;
    }

    pub fn get_text_scale(&self) -> f32 {
        if self.get_is_hovering() {
            if let Some(hover_text_scale) = self.on_hover_styles.text_scale {
                return hover_text_scale;
            }
        }

        self.text_scale
    }

    pub fn set_text_scale(&mut self, new_scale: f32) {
        self.text_scale = new_scale;
    }

    pub fn get_is_hovering(&self) -> bool {
        self.is_hovering
    }

    pub fn set_is_hovering(&mut self, new_val: bool) {
        self.is_hovering = new_val;
    }
}

