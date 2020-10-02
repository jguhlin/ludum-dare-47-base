use gdnative::prelude::*;
use hecs::World;
use once_cell::sync::OnceCell;
use rand::prelude::*;
use rand_xoshiro::rand_core::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

// Is a dialog taking priority over movement / etc?
pub static DIALOG_PRIORITY: OnceCell<Arc<AtomicBool>> = OnceCell::new();

// Flexible gamestate
pub static GAMESTATE: OnceCell<Arc<RwLock<HashMap<String, StateProperty>>>> = OnceCell::new();

// HECS WORLD
pub static WORLD: OnceCell<Arc<RwLock<World>>> = OnceCell::new();

// Initialize the game only once...
pub static GAME_INIT: OnceCell<Arc<AtomicBool>> = OnceCell::new();

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Pos {
    pub x: i16,
    pub y: i16,
}

impl Pos {
    pub fn new(x: i16, y: i16) -> Self {
        Pos { x, y }
    }
}

impl Into<Vector2> for Pos {
    fn into(self) -> Vector2 {
        Vector2::new(self.x as f32, self.y as f32)
    }
}

#[derive(PartialEq, Clone)]
pub struct Posf32 {
    pub x: f32,
    pub y: f32,
}

impl Posf32 {
    pub fn new(x: f32, y: f32) -> Self {
        Posf32 { x, y }
    }
}

impl Into<Vector2> for Posf32 {
    fn into(self) -> Vector2 {
        Vector2::new(self.x as f32, self.y as f32)
    }
}

#[derive(Clone, Debug)]
pub enum StateProperty {
    String(String),
    Numeric(i64),
    Empty,
}

impl StateProperty {
    pub fn numeric_get_value(self) -> Result<i64, &'static str> {
        match self {
            StateProperty::Numeric(x) => Ok(x),
            _ => Err("Not a Numeric Type"),
        }
    }
}

pub fn init_game_state() {
    if GAME_INIT.get().is_some() {
        return;
    }

    match WORLD.set(Arc::new(RwLock::new(World::new()))) {
        Err(_x) => panic!("Unable to set world..."),
        Ok(()) => (),
    };
    //        .expect("Unable to set World");

    DIALOG_PRIORITY
        .set(Arc::new(AtomicBool::new(false)))
        .expect("Unable to set DIALOG_PRIORITY");

    GAME_INIT
        .set(Arc::new(AtomicBool::new(true)))
        .expect("Unable to set DIALOG_PRIORITY");

    GAMESTATE
        .set(Arc::new(RwLock::new(HashMap::new())))
        .expect("Unable to initialize game state");

}

// This is for a BRAND NEW game rather than just loading up a game...
// Will need a matching fn for loading...
pub fn new_game() {
    set_game_state("gold", StateProperty::Numeric(1000));
   // .expect("Unable to create the world...");
}

pub fn read_game_state(key: &str) -> StateProperty {
    GAMESTATE
        .get()
        .unwrap()
        .read()
        .unwrap()
        .get(key)
        .unwrap_or(&StateProperty::Empty)
        .clone()
}

pub fn set_game_state(key: &str, val: StateProperty) -> StateProperty {
    GAMESTATE
        .get()
        .unwrap()
        .write()
        .unwrap()
        .insert(key.to_string(), val)
        .unwrap_or(StateProperty::Empty)
}