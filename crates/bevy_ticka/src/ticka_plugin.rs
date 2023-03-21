use bevy::{prelude::{Plugin, Res, Resource, ResMut}, time::Time};
use ticka::{ticka::Ticka, real_time_ticka_fascade::RealTimeTickaFascade, unit::{Unit, UnitPlan, UnitPlanEnum}, unit_move_action::MovePlanAction};
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

fn unit_plan(unit: &Unit) -> UnitPlan {
    return UnitPlan::new( unit.clone(), UnitPlanEnum::Move(MovePlanAction::from_single_step(hex2d::Direction::XZ)));
}

impl Plugin for TickaPlugin {

    fn build(&self, app: &mut bevy::prelude::App) {
        

        let movement_reader = MovementReader::new();
        let sender = movement_reader.create_sender();
        let ticka = Ticka::new(1000, 1000, 1,  unit_plan, sender);
        let real_time_ticka = RealTimeTickaFascade::from_ticka(ticka, 1.0);
        let res = TickaRes{ real_time_ticka }; 
            
        app.insert_resource(res);
        app.add_system(ticka_system);
    } 
}
