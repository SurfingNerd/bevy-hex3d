use sn_rust::indexed_field2d_location::IndexedField2DLocation;


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


pub enum UnitPlanAction {
    Move(MovePlanAction)
}

pub trait PlanAction {

    fn execute(&self, unit: &Unit);
    fn description(&self, unit: &Unit) -> String;

    fn move_to_field(&self, unit: &Unit) -> Option<IndexedField2DLocation> {
        None
    }
}

pub struct MovePlanAction {
    direction: hex2d::Direction
}

impl MovePlanAction {
    pub fn from_single_step(direction: hex2d::Direction) -> Self {
        MovePlanAction { direction }
    }
}

impl PlanAction for MovePlanAction {

    fn execute(&self, unit: &Unit) {
        print!("moving: {:?} to {:?}", unit, self.direction)
    }

    fn description(&self, unit: &Unit) -> String {
        format!("moving: {:?} to {:?}", unit, self.direction)
    }
}


pub enum UnitPlanEnum {
    Idle,
    Move(MovePlanAction),
}

// a plan what the unit is planning to do.
// this can target a field (like a Move or n Area of Effect attack), a unit on a field, including itself.
// 
pub struct UnitPlan {
    unit: Unit,
    plan: Box<dyn PlanAction>
}


impl UnitPlan {

    pub fn new(unit: Unit, plan: Box<dyn PlanAction>) -> Self {
        UnitPlan { unit, plan }
    }

    pub fn execute(&self) {
       self.plan.execute(&self.unit)
    }

    pub fn description(&self) -> String {
        self.plan.description(&self.unit)
    }

    pub fn move_to_field(&self) -> Option<IndexedField2DLocation> {
        self.plan.move_to_field(&self.unit)
    }

    pub fn unit(&self) -> &Unit {
        &self.unit
    }
}


