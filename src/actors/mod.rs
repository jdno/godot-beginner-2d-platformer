use gdnative::prelude::*;

pub use self::actor::*;
pub use self::enemy::*;
pub use self::player::*;

mod actor;
mod enemy;
mod player;

const FLOOR_NORMAL: Vector2 = Vector2::UP;
