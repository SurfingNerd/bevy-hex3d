


pub struct UnitMovementPlugin {
  
}

impl Plugin for PlaygroundPlugin {
  fn build(&self, app: &mut App) {
      app
          .add_system(move_entites_system);
  }
}