use std::time::{Duration, Instant};

use crate::renderer::{error::Result, render::{Render, Size}, vertice::Position};

pub const DEFAULT_SIZE_FOR_CAMERA: Size = Size { width: 30.0, height: 30.0 };
pub const DEFAULT_REPEAT_INTERVAL: Duration = Duration::from_millis(3000);

#[derive(Debug, Clone)]
pub struct Camera<'a> {
    position: Position,
    size: Size,
    image: &'a str,
    flip: bool,
    scale: Option<f32>,
    rotate: Option<f32>,
    repeat: bool,
    last_move_time: Instant,
    repeat_interval: Option<Duration> 
}

impl Camera<'_> {
    pub fn new_without_repeat(position: Position, flip: bool, scale: Option<f32>, rotate: Option<f32>) -> Self {
        Self {
            position,
            size: DEFAULT_SIZE_FOR_CAMERA,
            image: "assets/game/camera.png",
            flip,
            scale,
            rotate,
            repeat: false,
            last_move_time: Instant::now(),
            repeat_interval: None
        }
    }

    pub fn new_with_repeat(position: Position, flip: bool, scale: Option<f32>, rotate: Option<f32>, repeat_time: Option<u64>) -> Self {
        let repeat_time = repeat_time.unwrap_or(0);
        
        let interval = if repeat_time <= 0 { DEFAULT_REPEAT_INTERVAL } else { Duration::from_millis(repeat_time) };

        Self {
            position,
            size: DEFAULT_SIZE_FOR_CAMERA,
            image: "assets/game/camera.png",
            flip,
            scale,
            rotate,
            repeat: true,
            last_move_time: Instant::now(),
            repeat_interval: Some(interval)
        }
    }
}

impl<'a> Camera<'a> {
    pub fn draw(&mut self, render: &mut Render<'a>) -> Result<()> {
        if self.repeat && self.last_move_time.elapsed() >= self.repeat_interval.unwrap() {
            self.flip = !self.flip;

            let filp_factor = if self.flip { 1.0 } else { -1.0 };

            self.position = Position { x: self.position.x + (filp_factor * 12.0), y: self.position.y };

            self.last_move_time = Instant::now();
        }

        render.load_image(self.image, self.position, self.size, self.flip, None, self.scale, None, self.rotate)?;

        Ok(())
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn set_position(&mut self, new_position: Position) {
        self.position = new_position;
    }
}
