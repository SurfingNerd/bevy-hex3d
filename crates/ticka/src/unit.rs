use std::f32::consts::E;

use sn_rust::indexed_field2d_location::IndexedField2DLocation;

use crate::{ticka_context::TickaContext, unit_move_action::MovePlanAction, unit_plan_action::PlanAction};


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

impl Unit {
    pub fn id(&self) -> &UnitsIntegerType  {
        self.id()
    }
}

pub struct UnitPlanner{


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

        let unit_moves = Vec::<MovePlanAction>::new();

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


