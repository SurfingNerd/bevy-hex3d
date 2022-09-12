// use bevy::{
//     prelude::{AssetServer, Commands, Res, OrthographicCameraBundle, Handle, Transform},
//     sprite::ColorMaterial, log,
// };
// use bevy_ecs_tilemap::Map;

// use super::tiled_map::{TiledMapBundle, TiledMap};

// // pub fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {


// //     let handle: Handle<TiledMap> = asset_server.load("res/map_tiles.png");
// //     //let material_handle = materials.add(ColorMaterial::texture(texture_handle));

// //     let map_entity = commands.spawn().id();

// //     commands.entity(map_entity).insert_bundle(TiledMapBundle {
// //         tiled_map: handle,
// //         map: Map::new(0u16, map_entity),
// //         transform: Transform::from_xyz(0.0, 0.0, 0.0),
// //         ..Default::default()
// //     });
// // }

// pub fn map_startup(mut commands: Commands, asset_server: Res<AssetServer>) {

//     let handle: Handle<TiledMap> = asset_server.load("map.tmx");

//     log::info!("map started up!!");
//     let map_entity = commands.spawn().id();

//     commands.entity(map_entity).insert_bundle(TiledMapBundle {
//         tiled_map: handle,
//         map: Map::new(0u16, map_entity),
//         transform: Transform::from_xyz(0.0, 0.0, 0.0),
//         ..Default::default()
//     });

// }
