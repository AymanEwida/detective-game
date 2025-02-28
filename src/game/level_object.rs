use super::{collectable::CollectableType, door::DoorType, level::GameObject};

pub trait LevelObject<'a>: GameObject<'a> {
    fn get_type(&self) -> ObjectType;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ObjectType {
    Wall,
    Door(DoorType),
    Collectable(CollectableType),
    HidePlace,
    Camera,
}
