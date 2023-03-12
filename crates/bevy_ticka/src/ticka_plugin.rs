use std::marker::PhantomData;

use bevy::{prelude::{Plugin, Res, Resource, ResMut}, time::Time};
use ticka::{ticka::Ticka, real_time_ticka_fascade::RealTimeTickaFascade, layer_2D::Layer2D, unit::{Unit, UnitPlan}};


// Idea: create a RealTimeTicka in ticka ?
// so we can generalize real time ticka systems without bevy dependencies.


#[derive(Resource)]
pub struct TickaPlugin {

    // phantomUnit: PhantomData<&TUnit>,
    // phantomPlan: PhantomData<&TUnitPlan>,

   
    // how many ticks per second should occur ?
    ticks_per_second: f64,
}

#[derive(Resource)]
pub struct TickaRes {
    
    real_time_ticka: RealTimeTickaFascade,

 
}

impl Default for TickaPlugin {
    fn default() -> Self {
        Self { ticks_per_second: 1.0 }
    }
}

impl TickaPlugin {

    pub fn new() -> Self {
        TickaPlugin::default()
    }

}


fn ticka_system(time: Res<Time>,mut ticka: ResMut<TickaRes>) {

    let time = time.elapsed_seconds_f64();

    ticka.real_time_ticka.tick_if_time_has_come(time);
    
    //bevy::time::Time::delta_seconds_f64(&self)
}

fn unit_plan(unit: &Unit) -> UnitPlan {
    return UnitPlan { };
}

impl Plugin for TickaPlugin {

    fn build(&self, app: &mut bevy::prelude::App) {

        let tick_length = 1.0 / self.ticks_per_second;

        //let units = Field2D::<Unit>::new();
        let ticka = Ticka::new(1000, 1000, unit_plan);

        let real_time_ticka = RealTimeTickaFascade::from_ticka(ticka, 1.0);

        let res = TickaRes{ real_time_ticka }; 
            
        app.insert_resource(res);
        app.add_system(ticka_system);
    }
}
