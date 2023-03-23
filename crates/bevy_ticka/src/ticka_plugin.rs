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

fn unit_plan(unit: &Unit, ticka_context: &ticka::ticka_context::TickaContext) -> UnitPlan {

    if unit.id() < &100 {
        return UnitPlan::new( unit.clone(), UnitPlanEnum::Move(MovePlanAction::from_single_step(hex2d::Direction::XZ)));
    }

    return UnitPlan::new(unit.clone(), UnitPlanEnum::Idle(IdlePlanAction::new()));
    
}

impl Plugin for TickaPlugin {

    fn build(&self, app: &mut bevy::prelude::App) {
        
        info!("building Ticka base plugin");
        //let movement_reader = MovementReader::new();
        //let sender = movement_reader.create_sender();
        let ticka = Ticka::new(100, 100, 1,  unit_plan, None);
        let real_time_ticka = RealTimeTickaFascade::from_ticka(ticka, 1.0);
        let res = TickaRes{ real_time_ticka }; 
            
        app.insert_resource(res);
        app.add_system(ticka_system);

        info!("building Ticka base plugin - done");
    } 
}
