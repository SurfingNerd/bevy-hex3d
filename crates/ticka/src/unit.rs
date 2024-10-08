use std::f32::consts::E;

use sn_rust::{indexed_field2d_location::IndexedField2DLocation, mobile_entity_field_2_d::StorageLocationProvider};

use crate::{ticka_context::TickaContext, unit_move_action::MovePlanAction, unit_plan_action::PlanAction, unit_idle_plan_action::{IdlePlanAction, self}};


// Note: 
// depending on the type of game, 
// this type can be larger (u64) to support super large scale games or smaller(u16) in order to save some memory
// Todo: Add rust features that change the size of this type in an #if statement. ?!
type UnitsIntegerType = u32;

/// Unit 0 is NULL or can used as prototype creational pattern.
#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
  // id 0 does not exist an means NULL.
  id: UnitsIntegerType
}

impl Unit {
    pub fn id(&self) -> &UnitsIntegerType  {
        &self.id
    }

    pub fn new(id: UnitsIntegerType) -> Self {
        Unit { id: id }
    }
}

impl StorageLocationProvider for Unit {
    fn get_storage_id(&self) -> usize {
        return *self.id() as usize;
    }

    fn create_from_prototype(&self, storage_id: usize) -> Self {
        Unit{ id: storage_id as u32 }
    }
}

pub struct UnitPlanner {


}



#[derive(Debug, Clone)]
pub enum UnitPlanEnum {
    Idle(IdlePlanAction),
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
    is_executed: bool,
}


impl UnitPlan {

    pub fn new(unit: Unit, plan: UnitPlanEnum) -> Self {
        UnitPlan { unit, plan, is_executed: false }
    }

    pub fn execute(&mut self,  context: &mut TickaContext) {

        // let unit_moves = Vec::<MovePlanAction>::new();

        match &self.plan {
            UnitPlanEnum::Idle(idle_plan_action) => {
                idle_plan_action.execute(&self.unit, context);
            },
            UnitPlanEnum::Move(move_plan_action) => {
                move_plan_action.execute(&self.unit, context);
            },
        }

        self.is_executed = true;
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
            UnitPlanEnum::Idle(_) => {
                return "Idle".to_string();
            },
            UnitPlanEnum::Move(move_action) => {
                return move_action.description(&self.unit, context);
            },
        }
    }

    pub fn move_to_field(&self, context: &TickaContext) -> Option<IndexedField2DLocation> {

        match &self.plan {
            UnitPlanEnum::Idle(unit_idle_plan_action) => {
                return unit_idle_plan_action.move_to_field(&self.unit, context);
            },
            UnitPlanEnum::Move(move_action) => {
                return move_action.move_to_field(&self.unit, context);
            },
        }
    }

    /// Returns the current location of the unit this plan is for.
    pub fn current_entities_location(&self, context: &TickaContext) -> IndexedField2DLocation {

        return context.get_entity_location(&self.unit).clone();
    }

    pub fn is_staying(&self, context: &TickaContext) -> bool{
        // we are staying, once we plan to go the the same field we are currently on.
        
        if let Some(move_pos) = self.move_to_field(context) {
            let cmp_result = context.get_entity_location(&self.unit).cmp(&move_pos);
            return cmp_result == std::cmp::Ordering::Equal;
        }
        
        return true;
    }

    pub fn unit(&self) -> &Unit {
        &self.unit
    }

    pub fn plan(&self) -> &UnitPlanEnum {
        &self.plan
    }
}


