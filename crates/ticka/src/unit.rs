use std::f32::consts::E;

use sn_rust::indexed_field2d_location::IndexedField2DLocation;

use crate::ticka_context::TickaContext;


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

pub trait PlanAction where Self: Clone + Sized {

    /// executes the actions, changing values on the context
    fn execute(&self, unit: &Unit,  context: &mut TickaContext);
    
    /// text description of planned Action
    fn description(&self, unit: &Unit,  context: &TickaContext) -> String;

    /// returns Some, if the action does a field movement.
    fn move_to_field(&self, unit: &Unit) -> Option<IndexedField2DLocation> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct MovePlanAction {
    direction: hex2d::Direction
}


impl MovePlanAction {
    pub fn from_single_step(direction: hex2d::Direction) -> Self {
        MovePlanAction { direction }
    }

    pub fn get_target_location(&self, unit: &Unit,context:  &TickaContext) -> IndexedField2DLocation {
        IndexedField2DLocation::new(0,0)
    }
}

impl PlanAction for MovePlanAction {
    fn execute(&self, unit: &Unit, context:  &mut TickaContext) {
        print!("moving: {:?} to {:?}", unit, self.direction)
    }

    fn description(&self, unit: &Unit, context: &TickaContext) -> String {

        let l = self.get_target_location(unit, context);
        format!("Moves unit {} to {}-{}",unit.id, l.x(), l.y() )
    }
}



#[derive(Debug, Clone)]
pub enum UnitPlanEnum {
    Idle,
    Move(MovePlanAction),
}

impl UnitPlanEnum {

    // fn get_trait(&self) -> &dyn PlanAction{

    //     match self {
    //         UnitPlanEnum::Idle => todo!(),
    //         UnitPlanEnum::Move(a) => &dyn a,
    //     }
    // }
}

// a plan what the unit is planning to do.
// this can target a field (like a Move or n Area of Effect attack), a unit on a field, including itself.
// 
#[derive(Debug, Clone)]
pub struct UnitPlan {
    unit: Unit,
    plan: UnitPlanEnum,
}


impl UnitPlan {

    pub fn new(unit: Unit, plan: UnitPlanEnum) -> Self {
        UnitPlan { unit, plan }
    }

    pub fn execute(&self,  context: &mut TickaContext) {

        match &self.plan {
            UnitPlanEnum::Idle => {

            },
            UnitPlanEnum::Move(move_plan_action) => {
                move_plan_action.execute(&self.unit, context);
            },
        }
    }

    // fn get_trait(&self) -> &dyn PlanAction{

    //     match self {
    //         UnitPlanEnum::Idle => todo!(),
    //         UnitPlanEnum::Move(a) => &a,
    //     }
    // }


    pub fn description(&self, context: &TickaContext) -> String {
        //self.plan.get_trait()
        //self.plan.description(&self.unit)

        match &self.plan {
            UnitPlanEnum::Idle => {
                return "Idle".to_string();
            },
            UnitPlanEnum::Move(move_action) => {
                return move_action.description(&self.unit, context);
            },
        }
    }

    pub fn move_to_field(&self) -> Option<IndexedField2DLocation> {

        match &self.plan {
            UnitPlanEnum::Idle => {
                None
            },
            UnitPlanEnum::Move(move_action) => {
                return move_action.move_to_field(&self.unit);
            },
        }
    }

    pub fn unit(&self) -> &Unit {
        &self.unit
    }

    pub fn plan(&self) -> &UnitPlanEnum {
        &self.plan
    }
}


