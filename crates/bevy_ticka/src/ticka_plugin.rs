use std::marker::PhantomData;

use bevy::{prelude::{Plugin, Res, Resource}, time::Time};
use ticka::ticka::Ticka;


// Idea: create a RealTimeTicka in ticka ?
// so we can generalize real time ticka systems without bevy dependencies.


#[derive(Resource)]
pub struct TickaPlugin<TUnit : Send, TUnitPlan: Send> {

    // phantomUnit: PhantomData<&TUnit>,
    // phantomPlan: PhantomData<&TUnitPlan>,

   
    // how many ticks per second should occur ?
    ticks_per_second: f64,
}

#[derive(Resource)]
pub struct TickaRes<TUnit: Send, TUnitPlan: Send> {
    
    ticka: Ticka<TUnit, TUnitPlan>,

    // ticke length in seconds.
    tick_length: f64,

    // how many ticks per second should occur ?
    ticks_per_second: f64,

    last_processed_tick: f64,
    // since game loops fluctuate in their tick time,
    // we need to store the offset to the last tick.
    last_tick_adjustement: f64
}

impl<TUnit, TUnitPlan> Default for TickaPlugin<TUnit, TUnitPlan> {
    fn default() -> Self {
        Self { ticks_per_second: 1.0 }
    }
}

impl<TUnit, TUnitPlan> TickaPlugin<TUnit, TUnitPlan> {

    pub fn new() -> Self {
        TickaPlugin::default()
    }

}


fn ticka_system<TUnit: Send, TUnitPlan: Send>(time: Res<Time>, ticka: Res<TickaRes<TUnit, TUnitPlan>>) {

    time.elapsed_seconds_f64();
    time.delta_seconds_f64();
    //bevy::time::Time::delta_seconds_f64(&self)
}

impl<TUnit: Send, TUnitPlan: Send> Plugin for TickaPlugin<TUnit, TUnitPlan> {

    fn build(&self, app: &mut bevy::prelude::App) {

        let tick_length = 1.0 / self.ticks_per_second;

        const res = TickaRes<TUnit, TUnitPlan> { ticks_per_second: self.ticks_per_second, 
            
        app.insert_resource(res);
        app.add_system(ticka_system);
    }
}