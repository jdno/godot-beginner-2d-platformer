use std::fmt::Display;

use gdnative::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Enemy {}

impl Enemy {
    fn new(_owner: &KinematicBody2D) -> Self {
        Enemy {}
    }
}

#[methods]
impl Enemy {}

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
