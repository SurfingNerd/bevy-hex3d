use bevy::prelude::Component;



#[derive(Debug, Component)]
pub struct MoveComponent {

  pub ticks_passed: i32,
  pub ticks_to_move: i32,
}


#[derive(Debug, Component)]
pub struct HPComponent {
  pub hp_current: i32,
  pub hp_max: i32
}

#[derive(Debug, Component)]
pub struct ShootComponent {

  pub range: i32,
  pub damage: f32,

  pub ticks_to_fire: i32,
  pub ticks_passed: i32
}


#[derive(Debug, Component)]
pub struct Tower {

  pub range: i32,
  pub damage: f32,

  pub ticks_to_fire: i32,
  pub ticks_passed: i32
}

#[derive(Debug, Component)]
pub struct PositionComponent {

  pub x: i32,
  pub y: i32,
}
