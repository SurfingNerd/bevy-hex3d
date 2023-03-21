use bevy::{prelude::{Plugin, ResMut, Commands, Component, Query, Transform, Entity, Assets, StandardMaterial, shape::Cube, Color, PbrBundle, Vec3, Handle, Mesh}, sprite::ColorMaterial, reflect::Reflect};
use bevy_ticka::{ticka_plugin::{TickaPlugin, TickaRes}, movement_reader::MovementReader};
use derive_getters::Getters;
use ticka::{real_time_ticka_fascade::RealTimeTickaFascade, ticka::Ticka};

use crate::{components::PositionComponent, materials::get_color_material, game::Game};



#[derive(Component, Getters, Clone)]
pub struct TickaEntityComponent {
    ticka_storage_id: u32,
    bevy_entity_id: Entity
}

pub struct TickaFascadePlugin;


pub fn spawn_enemy(
    game: &mut Game,
    ticka: &mut Ticka,
    color_material: &Handle<bevy::prelude::StandardMaterial>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
 //   game: &mut Game,
    commands: &mut Commands,
    x: i32,
    y: i32,
) -> TickaEntityComponent {



    let cube = Cube::new(0.1);
    let cube_mesh = meshes.add(cube.into());

    let coord = hex2d::Coordinate::new(x, y);
    let (x_pixel, z_pixel) = coord.to_pixel(game.hex_spacing);

    // let material = get_color_material(materials, Color::RED);

    let ticka_unit = ticka.units_mut().spawn_entity(x as u32, y as u32);
  
    //let handle = Handle::<StandardMaterial> { }
   
    let entity = commands.spawn(
        PbrBundle {
          mesh: cube_mesh, // does only the handle get cloned here ? so we reuse the mesh ?
          material: color_material.clone(),
          transform: Transform {
              translation: Vec3::new(x_pixel, 0.3, z_pixel),
              ..Default::default()
          },
          ..Default::default()
      })
      .id();

    let mut entity_component = TickaEntityComponent {
        bevy_entity_id: entity.clone(),
        ticka_storage_id: *ticka_unit.id()
    };
    


    commands.entity(entity).insert(entity_component.clone());
      

    
    return entity_component;
         
    // game.set_entity(0, 0, unit);

  

}


fn startup_ticka(mut commands: Commands, mut game: ResMut<Game>,mut  meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
 mut ticka_res: ResMut<TickaRes>) {

    bevy::log::debug!("startup");

    // let cube = Cube::new(0.1);
    // let cube_mesh = meshes.add(cube.into());

    // let coord = hex2d::Coordinate::new(x, y);
    // let (x_pixel, z_pixel) = coord.to_pixel( hex2d::Spacing::FlatTop(0.50));

    let color_material = get_color_material(&mut materials, Color::BLUE);

    // let material = get_color_material(materials, Color::ALICE_BLUE); // get_blue_color(materials);


    for x in 0..10 {
        for y in 0..10 {
            let ticka = ticka_res.as_mut().real_time_ticka_mut().ticka_mut(); 
            // let spawned = ticka.units_mut().spawn_entity(x, y);
            
            let spawned = spawn_enemy(game.as_mut(), ticka, &color_material, &mut meshes, &mut materials, &mut commands, x, y);
            //commands.spawn(bundle)
            
            // game.set_entity(0, 0, unit);
            //let ticka_entity = TickaEntityComponent { ticka_storage_id: *spawned.id() };


        }
    }


    bevy::log::debug!("startup (wpawning end)");

    //ticka.real_time_ticka().units().spawn_entity(x, y)
}

fn ticka_system(mut commands: Commands, mut ticka: ResMut<TickaRes>, mut game: ResMut<Game>, mut query: Query<(&mut TickaEntityComponent, &mut Transform)>, ) {

    bevy::log::debug!("tick_system");
    //ticka.real_time_ticka().units()
    //real_time_ticka_fascade.
    // ticka.as_mut().real_time_ticka()

    let ticka = ticka.as_mut().real_time_ticka_mut().ticka_mut();

    let units = ticka.units_mut();

    for (mut ticka_entity_component,mut transform) in query.iter_mut() {

        
        let location = &units.unit_locations()[ticka_entity_component.ticka_storage_id as usize];

        let x = location.x();
        let y = location.y();

        // Move entity
        let coord = hex2d::Coordinate::new(x as i32, y as i32);
        let (x_pixel, z_pixel) = coord.to_pixel(game.hex_spacing);
        
        transform.translation = Vec3::new(x_pixel, 0.0, z_pixel);
    }


}

impl Plugin for TickaFascadePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {

        bevy::log::info!("building TickaFascadePlugin...");
        //let movement_reader = MovementReader::create_with_sender();

        let plugin = TickaPlugin::new();
        // registeres TickaRes
        plugin.build(app);

        app.add_startup_system(startup_ticka);
        app.add_system(ticka_system);

        bevy::log::info!("did built TickaFascadePlugin...");
        // app.insert_resource(resource);
    }
}