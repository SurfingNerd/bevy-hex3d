use bevy::{prelude::{Plugin, Res, Resource, ResMut, info}, time::Time};
use ticka::{ticka::Ticka, real_time_ticka_fascade::RealTimeTickaFascade, unit::{Unit, UnitPlan, UnitPlanEnum}, unit_move_action::MovePlanAction, unit_idle_plan_action::IdlePlanAction};
use derive_getters::Getters;

use crate::movement_reader::MovementReader;

// Idea: create a RealTimeTicka in ticka ?
// so we can generalize real time ticka systems without bevy dependencies.


#[derive(Resource)]
pub struct TickaPlugin {

    // phantomUnit: PhantomData<&TUnit>,
    // phantomPlan: PhantomData<&TUnitPlan>,

   
    // how many ticks per second should occur ?
    ticks_per_second: f64,
}

#[derive(Resource, Getters)]
pub struct TickaRes {
    real_time_ticka: RealTimeTickaFascade,
    
}

impl TickaRes {
    pub fn real_time_ticka_mut(&mut self) -> &mut RealTimeTickaFascade {
        &mut self.real_time_ticka
    }

    // pub fn ticka_mut(&mut self) -> &mut Ticka {
    //     &mut self.real_time_ticka.ticka_mut()
    // }
}

impl Default for TickaPlugin {
    fn default() -> Self {
        Self { ticks_per_second: 1.0 }
    }
}

impl TickaPlugin {

    pub fn new() -> Self {
        
        let result = TickaPlugin::default();
        return result;
    }

}


fn ticka_system(time: Res<Time>,mut ticka: ResMut<TickaRes>) {

    let time = time.elapsed_seconds_f64();

    ticka.real_time_ticka.tick_if_time_has_come(time);
    
    //bevy::time::Time::delta_seconds_f64(&self)
}

fn unit_plan(unit: &Unit, context: &ticka::ticka_context::TickaContext) -> UnitPlan {

    if unit.id() < &100 {
        // move towwards XZ.

        // let original_plan = UnitPlanEnum::Move();
        let current_location = context.unit_locations().get_entity_location(unit);

        // using all directions here, could easily result into a loop where a unit tacks 1 step forward, and then 1 step backwards.
        let directions = [hex2d::Direction::XZ, hex2d::Direction::XY, hex2d::Direction::YZ, hex2d::Direction::YX, hex2d::Direction::ZY, hex2d::Direction::ZX];

        for direction in directions.iter() {
            let current_location_coord = hex2d::Coordinate::new(current_location.x() as i32, current_location.y() as i32);
            let target_location = current_location_coord + *direction;
            let target_field = context.unit_locations().field().get(target_location.x as usize, target_location.y as usize);
            if target_field.is_none() {
                return UnitPlan::new( unit.clone(), UnitPlanEnum::Move(MovePlanAction::from_single_step(*direction)));
            }
        }
        

        return UnitPlan::new( unit.clone(), UnitPlanEnum::Idle(IdlePlanAction::new()));
    }

    return UnitPlan::new(unit.clone(), UnitPlanEnum::Idle(IdlePlanAction::new()));
    
}

impl Plugin for TickaPlugin {

    fn build(&self, app: &mut bevy::prelude::App) {
        
        
        info!("building Ticka base plugin");
        //let movement_reader = MovementReader::new();
        //let sender = movement_reader.create_sender();
        
        // todo: afture further refacturing there should be only one source of truth wich spacing is used.
        let spacing =  hex2d::Spacing::FlatTop(0.50);

        let ticka = Ticka::new(100, 100, 1,  unit_plan, None, spacing);
        let real_time_ticka = RealTimeTickaFascade::from_ticka(ticka, 0.05);
        let res = TickaRes{ real_time_ticka }; 
            
        app.insert_resource(res);
        app.add_system(ticka_system);

        info!("building Ticka base plugin - done");
    } 
}
