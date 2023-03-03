use bevy::{prelude::{Plugin, Res, Resource}, time::Time};
use ticka::ticka::Ticka;


// Idea: create a RealTimeTicka in ticka ?
// so we can generalize real time ticka systems without bevy dependencies.


#[derive(Resource)]
pub struct TickaPlugin {

   
    // how many ticks per second should occur ?
    ticks_per_second: f64,
}

#[derive(Resource)]
pub struct TickaRes {
    
    ticka: Ticka,

    // ticke length in seconds.
    tick_length: f64,

    // how many ticks per second should occur ?
    ticks_per_second: f64,

    last_processed_tick: f64,
    // since game loops fluctuate in their tick time,
    // we need to store the offset to the last tick.
    last_tick_adjustement: f64
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


fn ticka_system(time: Res<Time>, ticka: Res<TickaRes>) {

    time.elapsed_seconds_f64();
    time.delta_seconds_f64();
    //bevy::time::Time::delta_seconds_f64(&self)
}

impl Plugin for TickaPlugin {

    fn build(&self, app: &mut bevy::prelude::App) {

        let tick_length = 1.0 / self.ticks_per_second;

        app.insert_resource(TickaRes { ticks_per_second: self.ticks_per_second, ticka: Ticka {}, tick_length, last_processed_tick: 0.0, last_tick_adjustement: 0.0 });
        app.add_system(ticka_system);
    }
}