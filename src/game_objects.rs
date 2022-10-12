use bevy::prelude::{
    shape::Cube, Assets, Commands, Handle, Mesh, PbrBundle, ResMut, StandardMaterial,
    Transform, Vec3, Color,
};

use crate::{
    components::{PositionComponent, ShootComponent, MoveComponent, HPComponent},
    resources::Game, materials::get_color_material,
};

// pub fn monster_spawner() {

// }

fn get_red_color(materials: &mut ResMut<Assets<StandardMaterial>>) -> Handle<StandardMaterial> {
    get_color_material(materials, Color::RED)
}

pub fn spawn_enemy(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    game: &mut Game,
    commands: &mut Commands,
    x: i32,
    y: i32,
) {



    let cube = Cube::new(0.1);
    let cube_mesh = meshes.add(cube.into());

    let coord = hex2d::Coordinate::new(x, y);
    let (x_pixel, z_pixel) = coord.to_pixel(game.hex_spacing);

    // let material = get_color_material(materials, Color::RED);

    let material = get_red_color(materials);

    //let handle = Handle::<StandardMaterial> { }

    let unit = commands.spawn_bundle(
        PbrBundle {
          mesh: cube_mesh, // does only the handle get cloned here ? so we reuse the mesh ?
          material: material,
          transform: Transform {
              translation: Vec3::new(x_pixel, 0.3, z_pixel),
              ..Default::default()
          },
          ..Default::default()
      })
      .insert(MoveComponent {
          ticks_to_move: 100,
          ticks_passed: 0,
      })
      .insert(PositionComponent { x: 0, y: 0 })
      .insert(HPComponent::new(100.))
      .id();
  
  game.set_entity(0, 0, unit);

  

}


pub fn spawn_tower(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    game: &mut Game,
    commands: &mut Commands,
    x: i32,
    y: i32,
) {
    let cube = Cube::new(0.1);
    let cube_mesh = meshes.add(cube.into());

    let c = hex2d::Coordinate::new(x, y);
    let (x_pixel, z_pixel) = c.to_pixel(game.hex_spacing);

    let material = get_color_material(materials, Color::AQUAMARINE);

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
