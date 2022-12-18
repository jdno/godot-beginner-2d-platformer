use std::f64::consts::FRAC_PI_4;
use std::fmt::Display;

use gdnative::prelude::*;

use crate::actors::FLOOR_NORMAL;

#[derive(Copy, Clone, PartialEq, Debug, Default, NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Enemy {
    #[property]
    gravity: f32,

    #[property]
    speed: f32,

    velocity: Vector2,
}

impl Enemy {
    fn new(_owner: &KinematicBody2D) -> Self {
        Enemy {
            gravity: 1000.0,
            speed: 400.0,
            velocity: Vector2::ZERO,
        }
    }
}

#[methods]
impl Enemy {
    #[method]
    fn _ready(&mut self, #[base] _base: &KinematicBody2D) {
        self.velocity.x = -self.speed
    }

    #[method]
    fn _physics_process(&mut self, #[base] owner: &KinematicBody2D, delta: f32) {
        self.velocity.y += self.gravity * delta;

        if owner.is_on_wall() {
            self.velocity.x *= -1.0;
        }

        self.velocity.y = owner
            .move_and_slide(self.velocity, FLOOR_NORMAL, false, 4, FRAC_PI_4, true)
            .y;
    }
}

impl Display for Enemy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Enemy")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Enemy>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Enemy>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Enemy>();
    }
}
