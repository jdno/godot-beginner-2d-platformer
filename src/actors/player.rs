use std::f64::consts::FRAC_PI_4;
use std::fmt::Display;

use gdnative::prelude::*;

use crate::actors::Actor;

const FLOOR_NORMAL: Vector2 = Vector2::UP;

#[derive(Copy, Clone, PartialEq, Debug, Default, NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {
    #[property]
    gravity: f32,

    #[property]
    speed: f32,

    velocity: Vector2,
}

impl Player {
    pub fn new(_base: &KinematicBody2D) -> Self {
        Self {
            gravity: 1000.0,
            speed: 800.0,
            velocity: Vector2::ZERO,
        }
    }

    fn calculate_direction(&self, owner: &KinematicBody2D, input: &Input) -> Vector2 {
        let move_right_input = input.get_action_strength("move_right", false) as f32;
        let move_left_input = input.get_action_strength("move_left", false) as f32;

        let gravity = if input.is_action_just_pressed("jump", false) && owner.is_on_floor() {
            -1.0
        } else {
            1.0
        };

        Vector2::new(move_right_input - move_left_input, gravity)
    }

    fn calculate_velocity(
        &mut self,
        linear_velocity: Vector2,
        direction: Vector2,
        speed: Vector2,
        input: &Input,
        delta: f32,
    ) -> Vector2 {
        let x = speed.x * direction.x;
        let mut y = linear_velocity.y + self.gravity * delta;

        // Jumping
        if direction.y < 0.0 {
            y = speed.y * direction.y
        }

        // Interrupting the jump
        if input.is_action_just_released("jump", false) && y < 0.0 {
            y = 0.0;
        }

        Vector2::new(x, y)
    }
}

#[methods]
impl Player {
    #[method]
    fn _physics_process(&mut self, #[base] owner: &KinematicBody2D, delta: f32) {
        let input = Input::godot_singleton();

        let direction = self.calculate_direction(owner, input);
        let velocity = self.calculate_velocity(
            self.velocity,
            direction,
            Vector2::new(self.speed, self.speed),
            input,
            delta,
        );

        self.velocity = velocity;

        owner.move_and_slide(velocity, FLOOR_NORMAL, false, 4, FRAC_PI_4, true);
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
