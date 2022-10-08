use std::default::{Default, self};
use bevy_inspector_egui::{WorldInspectorPlugin};


use bevy::{
    log,
    prelude::*,
    render::{
        texture::ImageSettings, primitives::Frustum,
    },
    window::{WindowDescriptor, Windows},
    DefaultPlugins, diagnostic::*, time::Time,
};

use crate::{components::*, resources::Game, glow_line::{GlowLine, glow_line_system}, debug_systems::debug_resources_system, game_objects::{spawn_tower, spawn_enemy}, textures::uv_debug_texture, pools::MaterialRegistry, playground::PlaygroundPlugin};
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
    let mut game = Game::new(500,500);
    let max_tower_x = 20;
    let max_tower_y = 20;

    movement_settings.speed = 30.;

    
    
    // materials
    let texture_handle: Handle<Image> = asset_server.load("wood_pointy_top.png");

    //let into : Image = Image.
    // let sprite = Sprite
    // let texture_atlas = TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(128.0, 128.0), 4, 3);

    // info!("atlas {:?}", texture_atlas.texture_handles);

    // let texture_material = materials.add(StandardMaterial {
    //     base_color_texture: Some(texture_handle),
    //     alpha_mode: AlphaMode::Opaque,
    //     ..Default::default()
    // });
 
    // let heighmap = images.get(&heighmap);  
    
    // .insert(GroundTile {x , y});
    // .insert(Shape);
    let color_sun = Color::rgb(0.976, 0.685, 0.04);
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color: color_sun,
            intensity: 400000.0,
            range: 1000.,
            shadows_enabled: false,
            ..Default::default()
        },
        transform: Transform::from_xyz(160.0, 64.0, -222.0),
        ..Default::default()
    });

    

    // Snow Top
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color: Color::rgb(0.80,0.8,0.8),
            intensity: 40000.0,
            range: 10000.,
            shadows_enabled: false,
            ..Default::default()
        },
        transform: Transform::from_xyz(236.0, 137.0, -180.0),
        ..Default::default()
    });

    const HALF_SIZE: f32 = 1000.0;
    //let skyblue = Color::rgb(0.5294, 0.878, 0.9216);
    let skyblue_light = Color::rgb(0.95, 0.96, 0.99916);
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: skyblue_light,
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: false,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });
    
    
    
    for x in 1..std::cmp::min(max_tower_x, game.height) - 1 {
        for y in 1..std::cmp::min(max_tower_y, game.width) - 1 {
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

    let game = Game::new(700, 700);


    App::new()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("Hexagon hex2d demo"),
            ..Default::default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(MaterialRegistry::new())
        .insert_resource(game)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(PlaygroundPlugin::new())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())   
        // .add_plugin(bevy_screen_diags::ScreenDiagsPlugin)
        //.add_startup_system(load_heighmap_startup_system)
        .add_startup_system(setup)
        .add_system(mouse_button_input)
        .add_system(move_entites)
        .add_system(tower_shoot)
        .add_system(glow_line_system)
        .add_system(enemy_spawner)
         // .add_system(debug_resources_system)
        .run();
}
