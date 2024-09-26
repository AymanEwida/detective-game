use detective_game::{game::{character::Character, level::{GameObject, ObjectLevel, ObjectLevelType}, player::Player}, renderer::{error::Result, render::Render}};

pub struct Simulator<'a> {
    objects: Vec<ObjectLevel<'a>>
}

impl<'a> From<Vec<ObjectLevel<'a>>> for Simulator<'a> {
    fn from(value: Vec<ObjectLevel<'a>>) -> Self {
        Self {
            objects: value
        }
    }
}

impl Simulator<'_> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }
}

impl<'a> Simulator<'a> {
    pub fn draw(&self, player: &mut Player<'a>, render: &mut Render<'a>) -> Result<()> {
        render.fill_with_image("assets/game/background.jpg")?;
        
        for object in self.objects.iter() {
            match object.get_type() {
                ObjectLevelType::Wall => {
                    if player.collide(object) {
                        player.move_to_prev_position();
                    }
                },

                _ => ()
            }
            

            object.draw(render)?;
        }

        player.draw(render)?;

        // render.draw_equidistant_from_angle(Position { x: 400.0, y: 300.0 }, 30.0, 300.0, Direction::Up);

        Ok(())
    }

    pub fn clear_objects(&mut self) {
        self.objects.clear();
    }

    pub fn insert_objects(&mut self, new_objects: &[ObjectLevel<'a>]) {
        self.objects.extend_from_slice(new_objects);
    }
}
