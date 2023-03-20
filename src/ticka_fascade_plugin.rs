use bevy::prelude::{Plugin, ResMut};
use bevy_ticka::ticka_plugin::{TickaPlugin, TickaRes};
use ticka::real_time_ticka_fascade::RealTimeTickaFascade;



struct TickaFascadePlugin {

}

fn ticka_system(real_time_ticka_fascade: ResMut<TickaRes> ) {

    //real_time_ticka_fascade.
}

impl Plugin for TickaFascadePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {

        let plugin = TickaPlugin::new();

        plugin.build(app);
        
        app.insert_resource(resource);
    }
}