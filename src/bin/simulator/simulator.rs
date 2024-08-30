use detective_game::{game::{character::Character, level::{GameObject, ObjectLevel, ObjectLevelType}, player::Player}, renderer::{color::Color, error::Result, render::{Render, Size}, vertice::{Position, Vertice}}};

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

        render.load_image("assets/test/test.jpg", Position { x: 350.0, y: 250.0 }, Size { width: 100.0, height: 100.0 }, Some(90.0))?;
        render.draw_rectangle(Position { x: 150.0, y: 250.0 }, Size { width: 100.0, height: 100.0 }, Color::Blue, Some(45.0));
        render.draw_geometric_object(Position { x: 650.0, y: 100.0 }, 50.0, Color::Blue, None, Some(180.0));
        render.draw_triangle(Vertice(Position { x: 600.0, y: 350.0 }, Color::Red), Vertice(Position { x: 700.0, y: 350.0 }, Color::Red), Vertice(Position { x: 650.0, y: 250.0 }, Color::Red), Some(90.0));
        render.draw_line(Position { x: 250.0, y: 450.0 }, Position { x: 450.0, y: 450.0 }, Color::Red, Some(90.0));
        render.draw_curved_line(Position { x: 150.0, y: 550.0 }, Position { x: 300.0, y: 550.0 }, Color::Blue, None, Some(180.0));

        player.draw(render)?;

        Ok(())
    }

    pub fn clear_objects(&mut self) {
        self.objects.clear();
    }

    pub fn insert_objects(&mut self, new_objects: &[ObjectLevel<'a>]) {
        self.objects.extend_from_slice(new_objects);
    }
}
