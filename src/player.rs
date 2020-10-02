use crate::extensions::NodeExt;
use crate::state::*;
use crate::worldmap::{Direction, WorldMap};
use gdnative::api::{Camera2D, KinematicBody2D, KinematicCollision2D, Sprite, TileMap};
use gdnative::prelude::*;
use rand::prelude::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
pub struct Player {
    #[property(default = 200.0)]
    base_speed: f32,
    screen_size: Vector2,
    velocity: Vector2,
    direction: Direction,
}

#[methods]
impl Player {
    fn new(_owner: &KinematicBody2D) -> Self {
        Player {
            base_speed: 200.0,
            screen_size: Vector2::zero(),
            velocity: Vector2::zero(),
            direction: Direction::Still,
        }
    }

    #[export]
    fn _ready(&mut self, owner: &KinematicBody2D) {
        let viewport = unsafe { owner.get_viewport().unwrap().assume_safe() };
        self.screen_size = viewport.size();
    }

    pub fn get_input(&mut self) {
        let mut velocity = self.velocity;
        let input = Input::godot_singleton();

        if Input::is_action_pressed(&input, "ui_right") {
            velocity.x = 1.0
        }
        if Input::is_action_pressed(&input, "ui_left") {
            velocity.x = -1.0
        }
        if Input::is_action_pressed(&input, "ui_down") {
            velocity.y = 1.0
        }
        if Input::is_action_pressed(&input, "ui_up") {
            velocity.y = -1.0
        }

        if velocity.length() > 0.0 {
            velocity = velocity.normalize()
        }

        self.velocity = velocity;
    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody2D, delta: f32) {
        self.get_input();

        let speed = self.velocity * self.base_speed * delta;

        if DIALOG_PRIORITY.get().unwrap().load(Ordering::Relaxed) {
            return;
        }

        let collision = owner.move_and_collide(speed, false, false, false);
        if collision.is_some() {
            let scene = ResourceLoader::godot_singleton()
                .load("res://Confirm.tscn", "PackedScene", false)
                .unwrap();

            let root = unsafe { owner.get_typed_node::<Node, _>("/root/WorldMap/HUD") };
            let scene = unsafe { scene.assume_safe().cast::<PackedScene>().unwrap() };

            root.add_child(scene.instance(0).unwrap(), false);

            DIALOG_PRIORITY
                .get()
                .unwrap()
                .store(true, Ordering::Relaxed);

            let btnyes = unsafe {
                owner.get_typed_node::<Button, _>(
                    "/root/WorldMap/HUD/Confirm/ColorRect/VBoxContainer/HBoxContainer/BtnYes",
                )
            };
            let btnno = unsafe {
                owner.get_typed_node::<Button, _>(
                    "/root/WorldMap/HUD/Confirm/ColorRect/VBoxContainer/HBoxContainer/BtnNo",
                )
            };

            let tilemap =
                unsafe { owner.get_typed_node::<TileMap, _>("/root/WorldMap/CanvasLayer/TileMap") };

            let va = VariantArray::new();
            let pos = unsafe { collision.unwrap().assume_safe().position() };
            let pos = tilemap.world_to_map(pos);
            va.push(pos);

            let va = va.into_shared();

            btnyes.connect("button_up", unsafe { owner.assume_shared() }, "dock", va, 0);

            btnno.connect(
                "button_up",
                unsafe { owner.assume_shared() },
                "cancel_dock",
                VariantArray::new_shared(),
                0,
            );

            //let viewport = unsafe { root.get_viewport().unwrap().assume_safe() };
            //viewport.add_child(scene.instance(0).unwrap(), false);
        }

        if self.velocity.x != 0.0 {
            let sprite: &Sprite = unsafe {
                &owner
                    .get_node("Sprite")
                    .unwrap()
                    .assume_safe()
                    .cast()
                    .unwrap()
            };
            sprite.set_flip_h(self.velocity.x < 0.0);
        }

        let world_camera: &Camera2D = unsafe {
            &owner
                .get_node("/root/WorldMap/CanvasLayer/WorldCamera")
                .unwrap()
                .assume_safe()
                .cast()
                .unwrap()
        };
        world_camera.set_position(owner.global_position());
    }

    #[export]
    fn resize(&mut self, _owner: &KinematicBody2D, new_screen_size: Vector2) {
        self.screen_size = new_screen_size;
    }

    #[export]
    fn dock(&mut self, owner: &KinematicBody2D, pos: Vector2) {
        let mut current_town_write = CURRENT_TOWN.get().unwrap().write().unwrap();
        *current_town_write = Some(Pos {
            x: pos.x as i16,
            y: pos.y as i16,
        });
        let scene = ResourceLoader::godot_singleton()
            .load("res://TownMap.tscn", "PackedScene", false)
            .unwrap();

        // In case another thread needs it asap.
        drop(current_town_write);

        let root = unsafe { owner.get_typed_node::<Node, _>("/root") };
        let worldmap = unsafe { owner.get_typed_node::<Node, _>("/root/WorldMap") };
        root.remove_child(worldmap);
        worldmap.queue_free();

        let scene = unsafe { scene.assume_safe().cast::<PackedScene>().unwrap() };

        root.add_child(scene.instance(0).unwrap(), false);
        DIALOG_PRIORITY
            .get()
            .unwrap()
            .store(false, Ordering::Relaxed);
    }

    #[export]
    fn cancel_dock(&mut self, owner: &KinematicBody2D) {
        DIALOG_PRIORITY
            .get()
            .unwrap()
            .store(false, Ordering::Relaxed);

        let root = unsafe { owner.get_typed_node::<Node, _>("/root/WorldMap/HUD") };
        let conf = unsafe { owner.get_typed_node::<Node, _>("/root/WorldMap/HUD/Confirm") };

        root.remove_child(conf);
        conf.queue_free();
    }
}
