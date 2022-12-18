use std::fmt::{Display, Formatter};

use gdnative::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Actor;

impl Actor {
    pub fn new(_base: &KinematicBody2D) -> Self {
        Actor
    }
}

#[methods]
impl Actor {}

impl Display for Actor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Actor")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Actor>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Actor>();
    }

    #[test]
    fn trait_unpin() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Actor>();
    }
}
