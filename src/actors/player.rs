use std::f64::consts::FRAC_PI_4;
use std::fmt::Display;

use gdnative::prelude::*;

use crate::actors::Actor;

const GRAVITY: f32 = 1000.0;

#[derive(Copy, Clone, PartialEq, Debug, Default, NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {
    #[property]
    speed: f32,
    velocity: Vector2,
}

impl Player {
    pub fn new(_base: &KinematicBody2D) -> Self {
        Self {
            speed: 800.0,
            velocity: Vector2::ZERO,
        }
    }

    fn calculate_direction(&self) -> Vector2 {
        let input = Input::godot_singleton();

        let move_right_input = input.get_action_strength("move_right", false) as f32;
        let move_left_input = input.get_action_strength("move_left", false) as f32;

        Vector2::new(move_right_input - move_left_input, 0.0)
    }

    fn calculate_gravity(&self, delta: f32) -> Vector2 {
        let gravity = self.velocity.y + GRAVITY * delta;
        Vector2::new(0.0, gravity)
    }
}

#[methods]
impl Player {
    #[method]
    fn _physics_process(&mut self, #[base] owner: &KinematicBody2D, delta: f32) {
        let gravity = self.calculate_gravity(delta);
        let direction = self.calculate_direction();

        let velocity = gravity + direction * self.speed;
        self.velocity = velocity;

        owner.move_and_slide(velocity, Vector2::ZERO, false, 4, FRAC_PI_4, true);
    }
}

impl Actor for Player {}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Player>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Player>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Player>();
    }
}
