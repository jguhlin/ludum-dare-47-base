#![allow(dead_code)]
#![allow(unused_imports)]

extern crate once_cell;
extern crate strum;
#[macro_use]
extern crate strum_macros;
#[macro_use]
extern crate num_derive;
extern crate hecs;
extern crate num_traits;

use gdnative::prelude::*;
use once_cell::sync::OnceCell;
use rand::prelude::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

mod extensions;
mod state;
mod gameworld;
// mod worldcamera;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<gameworld::GameWorld>();
    //    handle.add_class::<WorldCamera::WorldCamera>();
}

godot_init!(init);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
