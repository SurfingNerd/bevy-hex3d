
// Note: 
// depending on the type of game, 
// this type can be larger (u64) to support super large scale games or smaller(u16) in order to save some memory
// Todo: Add rust features that change the size of this type in an #if statement. ?!
type UnitsIntegerType = u32;


#[derive(Debug, Clone)]
pub struct Unit {
  // id 0 does not exist an means 0.
  id: UnitsIntegerType
}

pub struct UnitPlanner{


}

pub enum HexDirection {
    
}

pub enum UnitPlanAction {
    Move(MovePlanAction)
}

pub trait PlanAction {

    fn execute(&self, unit: &Unit);
    fn description(&self, unit: &Unit) -> String;
}

pub struct MovePlanAction {
    direction: hex2d::Direction
}

impl PlanAction for MovePlanAction {

    fn execute(&self, unit: &Unit) {
        print!("moving: {:?} to {:?}", unit, self.direction)
    }

    fn description(&self, unit: &Unit) -> String {
        format!("moving: {:?} to {:?}", unit, self.direction)
    }
}

pub struct UnitPlan {

}


