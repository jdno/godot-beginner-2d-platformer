use std::fmt::Display;

use gdnative::prelude::*;

use crate::actors::Actor;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player;

impl Player {
    pub fn new(_base: &KinematicBody2D) -> Self {
        Player
    }
}

#[methods]
impl Player {
    #[method]
    fn _physics_process(&mut self, #[base] owner: &KinematicBody2D, delta: f32) {
        self.physics_process(owner, delta);
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
