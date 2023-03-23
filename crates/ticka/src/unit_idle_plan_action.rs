use sn_rust::indexed_field2d_location::IndexedField2DLocation;

use crate::{unit::Unit, ticka_context::{TickaContext, TickaContextRead}, unit_plan_action::PlanAction};
use derive_getters::Getters;

#[derive(Debug, Clone)]
pub struct IdlePlanAction {
}

impl IdlePlanAction {
    pub fn new() -> Self {
        IdlePlanAction {  }
    }
}

impl PlanAction for IdlePlanAction {
    fn execute(&self, unit: &Unit, context:  &mut TickaContext) {
        // print!("moving: {:?} to {:?}", unit, self.direction);

        
        // let option = context.unit_locations_new_mut();
        // //let borowed = option.take();

        let target_location = self.move_to_field(unit, context).expect("must");
        
        // if let Some(mut new_field) = borowed {
        //     new_field.set_entity(unit, target_location.x(), target_location.y());
        // }

        if let Some(mobile_entity_field) = context.unit_locations_new_mut() {
            mobile_entity_field.set_entity(unit, target_location.x(), target_location.y());
        }

        //println!("idle entity on( -> { } {}", target_location.x(), target_location.y());
        

        // unit_locations.move_entity(current_location.x(), current_location.y(), target_location.x(), target_location.y(), context);
        // context.unit_locations()
        // let from_location = context.unit_locations().
        
    }

    fn description(&self, unit: &Unit, context: &TickaContext) -> String {

        format!("idles around")
    }

    /// returns Some, if the action does a field movement.
    fn move_to_field(&self, unit: &Unit, context: &TickaContext) -> Option<IndexedField2DLocation> {
        Some(context.unit_locations().get_entity_location(unit).clone())
    }
}

