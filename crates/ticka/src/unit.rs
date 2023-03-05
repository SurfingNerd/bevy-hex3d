


#[derive(Debug)]
pub struct Unit {

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


