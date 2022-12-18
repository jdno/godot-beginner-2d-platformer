use std::f64::consts::FRAC_PI_4;

use gdnative::prelude::*;

const GRAVITY: f32 = 3000.0;

pub trait Actor {
    fn velocity(&self) -> &Vector2;
    fn set_velocity(&mut self, velocity: Vector2);

    fn physics_process(&mut self, owner: &KinematicBody2D, delta: f32) {
        let gravity = self.velocity().y + GRAVITY * delta;

        let velocity = Vector2::new(self.velocity().x, gravity);
        self.set_velocity(velocity);

        owner.move_and_slide(velocity, Vector2::ZERO, false, 4, FRAC_PI_4, true);
    }
}
