use bevy::prelude::{info, Plugin, App};


struct HelloPlugin {
  people: u32,
  counter: u32
}

impl HelloPlugin {
  fn add_people(&mut self) {
    self.people = 10;
  }

  fn hello_world(&mut self) {

    info!("Hello {} people {}", self.people, self.counter);
  }
}

impl Plugin for HelloPlugin {

  fn build(&self, app: &mut App) {
    
      // app.add_startup_system(add_people)
      //     .add_system(hello_world)

  }
}