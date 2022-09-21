//! This example demonstrates the built-in 3d shapes in Bevy.
//! The scene includes a patterned texture and a rotation for visualizing the normals and UVs.

mod components;
mod glow_line;
mod hex2d_demo;
mod hexagon;
mod resources;
mod debug_systems;
// use crate::map::TiledMapPlugin;

use hex2d_demo::run_hex2d_demo;


fn main() {
    run_hex2d_demo();
}
