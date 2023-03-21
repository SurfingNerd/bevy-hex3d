//! This example demonstrates the built-in 3d shapes in Bevy.
//! The scene includes a patterned texture and a rotation for visualizing the normals and UVs.

mod components;
mod glow_line;
mod hex2d_demo;
mod hexagon;
mod game;
mod debug_systems;
mod game_objects;
mod textures;
mod materials;
mod playground;
mod cam_movement;
mod ticka_fascade_plugin;

// use crate::map::TiledMapPlugin;

use hex2d_demo::run_hex2d_demo;


fn main() {
    run_hex2d_demo();
}
