

pub struct UnitStats {

  // mulitplier for the unit's movement speed on the x and z axes..
  pub movement_multiplier: f32,

  pub climping_up_multiplier: f32,
  pub climping_down_multiplier: f32,
  pub swimming_multiplier: f32,

}


impl UnitStats {

  pub fn new() -> Self {
    Self {
      movement_multiplier: 1.0,
      climping_up_multiplier: 1.5,
      climping_down_multiplier: 1.2,
      swimming_multiplier: 2.0,
    }
  }

}