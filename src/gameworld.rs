use crate::state::*;
use gdnative::prelude::*;
use hecs::World;
use std::ops::Deref;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct GameWorld {}

#[methods]
impl GameWorld {
    pub fn new(_owner: &Node2D) -> Self {
        // GLOBAL STATE GAME INITIALIZATION
        init_game_state();
        new_game();

        Self {}
    }

    #[export]
    pub fn _process(&self, owner: &Node2D, delta: f64) {}

    // Skipping _physics_process for now
}

pub fn with_world<F>(mut f: F)
where
    F: FnMut(&World),
{
    let world = WORLD.get().unwrap().try_read().unwrap();
    let _result = f(&world);
}

pub fn with_world_mut<F>(mut f: F)
where
    F: FnMut(&mut World),
{
    let mut world = WORLD.get().unwrap().try_write().unwrap();
    let _result = f(&mut world);
}
