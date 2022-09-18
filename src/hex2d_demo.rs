use std::default::Default;

use bevy::{prelude::{App, Commands, ResMut, Assets, Mesh, Image, StandardMaterial, PbrBundle, Transform, Vec3, Res, Input, MouseButton, shape::{self, Quad}, Color, PointLightBundle, PointLight, Component, Query, KeyCode, Vec2, AssetServer, Handle, AlphaMode}, window::{WindowDescriptor, Windows}, render::{texture::ImageSettings, render_resource::{Extent3d, TextureDimension, TextureFormat}}, DefaultPlugins, log, sprite::TextureAtlas};

use crate::{hexagon::Hexagon3D, Shape};
use bevy_flycam::PlayerPlugin;

const X_EXTENT: f32 = 2.;
const Y_EXTENT: f32 = 2.;

struct Game {
  width: i32,
  height: i32
}

#[derive(Component, Debug)]
struct GroundTile {
  x: i32,
  y: i32
}


// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
  const TEXTURE_SIZE: usize = 8;

  let mut palette: [u8; 32] = [
      255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
      198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
  ];

  

  let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
  for y in 0..TEXTURE_SIZE {
      let offset = TEXTURE_SIZE * y * 4;
      texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
      palette.rotate_right(4);
  }

  Image::new_fill(
      Extent3d {
          width: TEXTURE_SIZE as u32,
          height: TEXTURE_SIZE as u32,
          depth_or_array_layers: 1,
      },
      TextureDimension::D2,
      &texture_data,
      TextureFormat::Rgba8UnormSrgb,
  )
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut images: ResMut<Assets<Image>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  asset_server: Res<AssetServer>,
) {
  let debug_material = materials.add(StandardMaterial {
      base_color_texture: Some(images.add(uv_debug_texture())),
      ..Default::default()
  });


  
  let texture_handle: Handle<Image>  = asset_server.load("terrain.png");

  // let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(128.0, 128.0), 4, 3);
  
  let texture_material = materials.add(
    StandardMaterial {
        base_color_texture: Some(texture_handle), 
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
     }
  );
  // let texture = texture_atlas.textures[0];

  let game = Game { width: 10, height: 10};

 

  //let quat = bevy::math::Quat{size: Vec2 {x: -1.0, y: 0.0}, flip: false};
  let quat = bevy::math::Quat::from_rotation_x(std::f32::consts::PI * 1.50);

  let hex_mesh = meshes.add(Hexagon3D::default().into());

  

// for (i, shape) in shapes.into_iter().enumerate() {
  for x in 0..game.width {
    for y in 0..game.height {
      let c = hex2d::Coordinate::new(x, y);
      let (x_pixel, y_pixel) = c.to_pixel(hex2d::Spacing::FlatTop(0.51));
      commands
          .spawn_bundle(PbrBundle {
              mesh: hex_mesh.clone(), // does only the handle get cloned here ? so we reuse the mesh ?
              material: texture_material.clone(),
              transform: Transform {
                  translation: Vec3::new(
                      x_pixel,
                      0.01,
                      y_pixel,
                  ),
                  rotation: quat.clone(),
                  
                  ..Default::default()
              },
              ..Default::default()
          })
          .insert(GroundTile {x , y});
          // .insert(Shape);
    } 
  }
   
  
  //     info!(i);
  //     commands
  //         .spawn_bundle(PbrBundle {
  //             mesh: shape,
  //             material: debug_material.clone(),
  //             transform: Transform {
  //                 translation: Vec3::new(
  //                     -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
  //                     2.0,
  //                     0.0,
  //                 ),
  //                 rotation: quat.clone(),
                  
  //                 ..Default::default()
  //             },
  //             ..Default::default()
  //         })
  //         .insert(GroundTile {x: i as i32 , y: 1} );
  //         // .insert(Shape);
  // }

  commands.spawn_bundle(PointLightBundle {
      point_light: PointLight {
          intensity: 9000.0,
          range: 10000.,
          shadows_enabled: false,
          ..Default::default()
      },
      transform: Transform::from_xyz(8.0, 16.0, 8.0),
      ..Default::default()
  });

  // ground plane
  commands.spawn_bundle(PbrBundle {
      mesh: meshes.add(shape::Plane { size: 50. }.into()),
      material: materials.add(Color::SILVER.into()),
      ..Default::default()
  });

  commands.insert_resource(game);

  // commands.spawn_bundle(PerspectiveCameraBundle {
  //     transform: Transform::from_xyz(0.0, 0.0, 64.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
  //     ..Default::default()
  // });


  // let cam = Camera3dBundle {
  //     transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
  //     ..Default::default()
  // };
  
  //log::info!("{:?}", cam.transform);

  // commands.spawn_bundle(cam);
  
}


fn mouse_button_input(
    buttons: Res<Input<MouseButton>> ,
    windows: Res<Windows>,
    // tile_storage_query: Query<(Entity, &TileStorage)>,
    // tile_storage: Res<TileStorage>,
) {
    // log::info!("mouse_button_input");
    if buttons.just_pressed(MouseButton::Left) {

        // Left button was pressed
        let window = windows.get_primary().unwrap();
        
        if let Some(pos) = window.cursor_position() {
            log::info!("Left Mouse clicked: {:?}", pos);
            //todo: get tile from click.
        }

    }
    if buttons.just_released(MouseButton::Left) {
        // Left Button was released
    }
    if buttons.pressed(MouseButton::Right) {
        // Right Button is being held down
    }
    
    // we can check multiple at once with `.any_*`
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        // Either the left or the right button was just pressed
    }
}

#[allow(dead_code)]
fn rotate_hexes(mut query: Query<(&mut GroundTile, &mut Transform)>, keys: Res<Input<KeyCode>>) {

  // if (keys.just_pressed(KeyCode::F)) {
  //   query.for_each_mut(|(&mut muttile, &mut transform)| {
  //      transform.rotate_local_x(std::f32::consts::PI / 360.0);
  //      info!("GroundTile: {:?} rotation: {:?}", tile, transform.rotation);
  //   });
  // }

  for mut i in query.iter_mut() {
    i.1.rotate_local_x(std::f32::consts::PI / 360.0);
  }
  
}


pub fn run_hex2d_demo() {

  App::new()
  .insert_resource(WindowDescriptor {
      width: 1270.0,
      height: 720.0,
      title: String::from("Hexagon Column Example"),
      ..Default::default()
  })
  .insert_resource(ImageSettings::default_nearest())
  .add_plugins(DefaultPlugins)
  .add_plugin(PlayerPlugin)
  .add_startup_system(setup)
  // .add_system(crate::examples::movement)
  .add_system(mouse_button_input)
  
  // .add_system(rotate_hexes)
  .run();

}
