use bevy::prelude::{
    shape::Cube, Assets, Commands, Handle, Material, Mesh, PbrBundle, ResMut, StandardMaterial,
    Transform, Vec3,
};

use crate::{
    components::{PositionComponent, ShootComponent},
    resources::Game,
};

// pub fn monster_spawner() {

// }

pub fn spawn_tower(
    meshes: &mut ResMut<Assets<Mesh>>,
    material: &Handle<StandardMaterial>,
    game: &mut Game,
    commands: &mut Commands,
    x: i32,
    y: i32,
) {
    let cube = Cube::new(0.1);
    let cube_mesh = meshes.add(cube.into());

    let c = hex2d::Coordinate::new(x, y);
    let (x_pixel, z_pixel) = c.to_pixel(hex2d::Spacing::FlatTop(0.51));
    // tower:
    let tower_id = commands
        .spawn_bundle(PbrBundle {
            mesh: cube_mesh.clone(), // does only the handle get cloned here ? so we reuse the mesh ?
            material: material.clone(),
            transform: Transform {
                translation: Vec3::new(x_pixel, 0.3, z_pixel),
                scale: Vec3::new(1., 3., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(PositionComponent { x, y })
        .insert(ShootComponent {
            damage: 1000.,
            range: 3,
            ticks_passed: 0,
            ticks_to_fire: 10,
        })
        .id();

    game.set_entity(x, y, tower_id);
}
