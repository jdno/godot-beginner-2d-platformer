use std::f64::consts::FRAC_PI_4;
use std::fmt::Display;

use gdnative::api::{CollisionShape2D, PhysicsBody2D};
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
    fn _ready(&mut self, #[base] base: &KinematicBody2D) {
        // Do not move enemy until it is visible
        base.set_physics_process(false);

        // Move the enemy leftwards towards the player
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

    #[method]
    fn on_stomp_detector_body_entered(&mut self, #[base] owner: &KinematicBody2D, body: Variant) {
        let body = unsafe { body.try_to_object::<PhysicsBody2D>().unwrap().assume_safe() };

        if body.global_position().y > owner.global_position().y {
            return;
        }

        let collision_shape = unsafe {
            owner
                .get_node("CollisionShape2D")
                .unwrap()
                .assume_safe()
                .cast::<CollisionShape2D>()
                .unwrap()
        };
        collision_shape.set_deferred("disabled", true);

        owner.queue_free();
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
