use bevy::prelude::{Plugin, ResMut};
use bevy_ticka::{ticka_plugin::{TickaPlugin, TickaRes}, movement_reader::MovementReader};
use ticka::real_time_ticka_fascade::RealTimeTickaFascade;



struct TickaFascadePlugin {

}

fn ticka_system(mut ticka: ResMut<TickaRes> ) {

    //real_time_ticka_fascade.
    // ticka.as_mut().real_time_ticka()
}

impl Plugin for TickaFascadePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {


        //let movement_reader = MovementReader::create_with_sender();

        

        let plugin = TickaPlugin::new();

        plugin.build(app);
        
        app.add_system(ticka_system);
        
        // app.insert_resource(resource);
    }
}