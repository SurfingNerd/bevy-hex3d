use sn_rust::field_2_d::Field2D;

use crate::{unit::{Unit, UnitPlan, UnitPlanEnum}, ticka_context::TickaContext, unit_idle_plan_action::IdlePlanAction, conflict::UnitPlanMoveConflict, ticka::Ticka};


/// units leave behind phermones on move.
/// Units searches near fields for pheromones,
/// and if they detect some, that did not originate by themself,
/// then they follow this tracks. 
pub struct PheromoneSystem {
    field_index: usize,    
}

pub struct PheromoneSystemUnitPlaner{

}



impl PheromoneSystemUnitPlaner {

    pub fn get_unit_plan(&self, unit: &Unit, context: &TickaContext) -> Option<UnitPlan> {
        return Some(UnitPlan::new(unit.clone(), UnitPlanEnum::Idle(IdlePlanAction::new())));
    }

}

impl PheromoneSystem {
    pub fn new() -> Self {
        PheromoneSystem {
            field_index: 0
        }
    }

    pub fn init(&mut self, ticka: &mut Ticka) {
        self.field_index = ticka.register_field_f32();
    }

    pub fn get_field<'a>(&self, context: &'a TickaContext) -> &'a Field2D<f32> {
        &context.fields()[self.field_index]
    }

    pub fn get_pheromone(&self, context: &TickaContext, x: usize, y: usize) -> f32 {
        self.get_field(context).get(x, y).clone()
    }

    pub fn get_field_mut<'a>(&self, context: &'a mut TickaContext) -> &'a mut Field2D<f32> {
        &mut context.fields_mut()[self.field_index]
    }

    

}