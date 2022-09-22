use std::default::{Default, self};

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
    DefaultPlugins, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, ecs::world, time::Time,
};

use crate::{components::*, resources::Game, glow_line::{GlowLine, glow_line_system}, debug_systems::debug_resources_system, game_objects::{spawn_tower, spawn_enemy}, textures::uv_debug_texture, pools::MaterialRegistry};
use crate::{hexagon::Hexagon3D};
use bevy_flycam::{MovementSettings, PlayerPlugin};

const X_EXTENT: f32 = 2.;
const Y_EXTENT: f32 = 2.;


#[derive(Component, Debug)]
struct GroundTile {
    x: i32,
    y: i32,
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut materials_pool: ResMut<MaterialRegistry>,
    mut movement_settings: ResMut<MovementSettings>,
    asset_server: Res<AssetServer>,
) {
    let mut game = Game::new(50,50);

    //info!("movement speed: {}", movement_settings.speed);
    movement_settings.speed = 3.;


    // materials
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
    
    let mat2 = materials.add(
        StandardMaterial {
            base_color: Color::rgb(123.0 / 255., 130. / 255., 78. / 255.),
            metallic: 0.8,
            reflectance: 0.95,
            perceptual_roughness: 0.9,
            
            ..Default::default()
        }
    );

    commands.spawn_bundle(PbrBundle {
        mesh: mesh_handle,
        material: mat2.clone(),
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
    
    for x in 1..game.height - 1 {
        for y in 1..game.width - 1 {
            if x % 2 == 0 && y % 2 == 0 {
                spawn_tower(&mut meshes, &mut materials, &mut game, &mut commands,  x, y);
            }
        }
    }

    spawn_enemy(&mut meshes, &mut materials, &mut materials_pool, &mut images, &mut game, &mut commands, 0, 0);

    commands.insert_resource(game);

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

            // info!(
            //     "Updated Position to {:?} to {:?}",
            //     position, transform.translation
            // );
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
            
            // info!("shooting at {:?} hp: {}", entity, hp.hp_current);
            GlowLine::create(&mut commands, &mut meshes, &mut materials,transform.translation.clone(), transform_target.translation.clone(), 0.1);
            shoot.notify_shoot();

            // 
            if hp.damage_is_dead(shoot.damage) {
            //  info!("Entity is dead! {:?}", entity);
            }

            break; // we have  been shooting, can not shoot a second time...
          }

          // we have a neighbor!
          // now let's get the HPComponent, but how?...
          // this is the point in time where i question if i just should store HP not in an Bevy Component,
          // and in the Game.
        }
      }
    }
}


fn enemy_spawner(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes:ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut game: ResMut<Game>,
    mut material_pool: ResMut<MaterialRegistry>
) {
    if game.get_entity(0, 0) == None {
        spawn_enemy(&mut meshes, &mut materials, &mut material_pool,  &mut images, &mut game, &mut commands, 0, 0);
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
        .insert_resource(MaterialRegistry::new())
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(bevy_screen_diags::ScreenDiagsPlugin)
        .add_startup_system(setup)
        .add_system(mouse_button_input)
        .add_system(move_entites)
        .add_system(tower_shoot)
        .add_system(glow_line_system)
        .add_system(enemy_spawner)
        // .add_system(debug_resources_system)
        .run();
}
