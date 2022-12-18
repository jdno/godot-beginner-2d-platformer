use std::f64::consts::FRAC_PI_4;

use gdnative::prelude::*;

pub trait Actor {
    fn physics_process(&mut self, owner: &KinematicBody2D, _delta: f32) {
        let horizontal_movement: Vector2 = Vector2::new(300.0, 0.0);
        let vertical_movement: Vector2 = Vector2::ZERO;

        owner.move_and_slide(
            horizontal_movement,
            vertical_movement,
            false,
            4,
            FRAC_PI_4,
            false,
        );
    }
}
