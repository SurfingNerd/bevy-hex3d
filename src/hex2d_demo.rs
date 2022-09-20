use std::default::Default;

use bevy::{
    log,
    prelude::{
        info,
        shape::{self, Cube, Quad},
        AlphaMode, App, AssetServer, Assets, Color, Commands, Component, Entity, Handle, Image,
        Input, KeyCode, Mesh, MouseButton, PbrBundle, PointLight, PointLightBundle, Query, Res,
        ResMut, StandardMaterial, Transform, Vec2, Vec3, World,
    },
    render::{
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::ImageSettings,
    },
    sprite::{Sprite, TextureAtlas},
    window::{WindowDescriptor, Windows},
    DefaultPlugins, diagnostic::FrameTimeDiagnosticsPlugin,
};

use crate::{components::*, resources::Game, glow_line::GlowLine};
use crate::{hexagon::Hexagon3D};
use bevy_flycam::{MovementSettings, PlayerPlugin};

const X_EXTENT: f32 = 2.;
const Y_EXTENT: f32 = 2.;


#[derive(Component, Debug)]
struct GroundTile {
    x: i32,
    y: i32,
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
    mut movement_settings: ResMut<MovementSettings>,
    asset_server: Res<AssetServer>,
) {
    let mut game = Game::new(10,10);

    //info!("movement speed: {}", movement_settings.speed);
    movement_settings.speed = 3.;

    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..Default::default()
    });

    let texture_handle: Handle<Image> = asset_server.load("wood_pointy_top.png");

    //let into : Image = Image.

    // let sprite = Sprite

    // let texture_atlas = TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(128.0, 128.0), 4, 3);

    // info!("atlas {:?}", texture_atlas.texture_handles);

    let texture_material = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        alpha_mode: AlphaMode::Opaque,
        ..Default::default()
    });
    // let texture = texture_atlas.textures[0];

    //let quat = bevy::math::Quat{size: Vec2 {x: -1.0, y: 0.0}, flip: false};
    // let quat = bevy::math::Quat::from_rotation_x(std::f32::consts::PI * 1.50);

    // let hex_mesh = meshes.add(Hexagon3D::default().into());

    let mut hexes: Vec<Hexagon3D> = vec![];

    let cube = Cube::new(0.1);
    let cube_mesh = meshes.add(cube.into());

    // for (i, shape) in shapes.into_iter().enumerate() {
    for x in 0..game.width {
        for y in 0..game.height {
            let c = hex2d::Coordinate::new(x, y);
            let (x_pixel, y_pixel) = c.to_pixel(hex2d::Spacing::FlatTop(0.51));
            // info!("pixel x {} y {} ", x_pixel, y_pixel);
            let hex = Hexagon3D {
                diameter: 1.,
                height: 0.,
                x: x_pixel,
                y: 0.,
                z: y_pixel,
            };

            hexes.push(hex);
        }
    }

    let mesh = Hexagon3D::create_mesh_for_hexes(&hexes);
    let mesh_handle = meshes.add(mesh);
    

    commands.spawn_bundle(PbrBundle {
        mesh: mesh_handle,
        material: texture_material.clone(),
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
            // rotation: quat.clone(),
            ..Default::default()
        },
        ..Default::default()
    });
    // .insert(GroundTile {x , y});
    // .insert(Shape);

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 4000.0,
            range: 10000.,
            shadows_enabled: false,
            ..Default::default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..Default::default()
    });

    let mut spawn_tower = |x: i32, y: i32| {
        let c = hex2d::Coordinate::new(x, y);
        let (x_pixel, z_pixel) = c.to_pixel(hex2d::Spacing::FlatTop(0.51));
        // tower:
        let tower_id = commands
            .spawn_bundle(PbrBundle {
                mesh: cube_mesh.clone(), // does only the handle get cloned here ? so we reuse the mesh ?
                material: debug_material.clone(),
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
                ticks_passed: 1000,
                ticks_to_fire: 1000,
            }).id();

         game.set_entity(x, y, tower_id);
    };

    spawn_tower(1, 1);

    // ground plane
    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(shape::Plane { size: 50. }.into()),
    //     material: materials.add(Color::SILVER.into()),
    //     ..Default::default()
    // });

    let unit = commands.spawn_bundle(
          PbrBundle {
            mesh: cube_mesh, // does only the handle get cloned here ? so we reuse the mesh ?
            material: texture_material.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.3, 0.0),
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
    buttons: Res<Input<MouseButton>>,
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

fn move_entites(
    mut query: Query<(&mut PositionComponent, &mut MoveComponent, &mut Transform)>,
    mut game: ResMut<Game>,
) {
    for (mut position, mut movement, mut transform) in query.iter_mut() {
        movement.ticks_passed += 1;
        let old_x = position.x;
        let old_y = position.y;
        if movement.ticks_passed >= movement.ticks_to_move {
            if position.x < position.y && game.get_entity(position.x + 1, position.y) == None {
                position.x += 1;
            } else if game.get_entity(position.x, position.y + 1) == None {
                position.y += 1;
            } else {
              info!("nowhere to go ?!");
            }
            
            let entity = game.delete_entity(old_x, old_y);
            game.set_entity(position.x, position.y, entity);

            movement.ticks_passed = 0;

            // update the UI Pos.
            let c = hex2d::Coordinate::new(position.x, position.y);
            let (x_pixel, y_pixel) = c.to_pixel(hex2d::Spacing::FlatTop(0.51));
            transform.translation.x = x_pixel;
            transform.translation.z = y_pixel;
            // transform.translation = Vec3:: { x_pixel, 0.01, y_pixel };

            info!(
                "Updated Position to {:?} to {:?}",
                position, transform.translation
            );
        }
    }
}

fn tower_shoot(
    mut query: Query<(&mut PositionComponent, &mut ShootComponent, &Transform)>,
    mut query_hp: Query<(&mut HPComponent, &Transform)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game: Res<Game>,
    
) {
    
    for (position, mut shoot, transform) in query.iter_mut() {

      if !shoot.can_shoot_tick() {
        continue;
      }
      
      // find posible targets.
      let coord = hex2d::Coordinate::new(position.x, position.y);
      
      for neighbor in coord.neighbors() {
        
        if let Some(entity) = game.get_entity(neighbor.x, neighbor.y) {
          // info!("trying to shoot at {:?}", neighbor);
          if let Ok((mut hp, transform_target)) = query_hp.get_mut(entity) {
            
            info!("shooting at {:?} hp: {}", entity, hp.hp_current);
            GlowLine::create(&mut commands, &mut meshes, &mut materials,transform.translation.clone(), transform_target.translation.clone(), 0.1);
            shoot.notify_shoot();

            // 
            if hp.damage_is_dead(shoot.damage) {
              info!("Entity is dead! {:?}", entity);
            }
          }

          // we have a neighbor!
          // now let's get the HPComponent, but how?...
          // this is the point in time where i question if i just should store HP not in an Bevy Component,
          // and in the Game.
        }
      }
    }
}

pub fn run_hex2d_demo() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("Hexagon hex2d demo"),
            ..Default::default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(mouse_button_input)
        .add_system(move_entites)
        .add_system(tower_shoot)
        // .add_system(rotate_hexes)
        .run();
}
