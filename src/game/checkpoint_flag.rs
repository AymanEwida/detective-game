use crate::renderer::{error::Result, render::Render, styles::Size, vertice::Position};

#[derive(Debug)]
pub struct CheckPointFlag {
    position: Position,
    size: Size,
    spawn_position: Position,
}

impl CheckPointFlag {
    pub fn new(position: Position, spawn_position: Position, size: Size) -> Self {
        Self {
            position,
            size,
            spawn_position,
        }
    }
}

impl<'a> CheckPointFlag {
    pub fn draw(&self, render: &mut Render<'a>) -> Result<()> {
        render.load_image(
            "assets/game/checkpoint_flag.png",
            self.position,
            self.size,
            false,
            None,
            None,
            None,
            None,
        )?;

        Ok(())
    }

    pub fn get_postion(&self) -> Position {
        self.position
    }

    pub fn get_size(&self) -> Size {
        self.size
    }

    pub fn get_spawn_postion(&self) -> Position {
        self.spawn_position
    }

    pub fn set_spawn_postion(&mut self, new_position: Position) {
        self.spawn_position = new_position;
    }
}
