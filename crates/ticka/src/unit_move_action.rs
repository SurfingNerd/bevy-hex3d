use sn_rust::indexed_field2d_location::IndexedField2DLocation;

use crate::{unit::Unit, ticka_context::{TickaContext, TickaContextRead}, unit_plan_action::PlanAction};
use derive_getters::Getters;

#[derive(Debug, Clone)]
pub struct MovePlanAction {
    direction: hex2d::Direction
}

#[derive(Getters)]
pub struct UnitMoveInstance {
    from_x: u32,
    from_y: u32,
    to_x: u32,
    to_y: u32
}

impl MovePlanAction {
    pub fn from_single_step(direction: hex2d::Direction) -> Self {
        MovePlanAction { direction }
    }

    pub fn get_target_location(&self, unit: &Unit, context:  &TickaContext) -> IndexedField2DLocation {
        //&IndexedField2DLocation::new(0,0)

        let loc = context.unit_locations().get_entity_location(unit);
        //hex2d
        let coord = hex2d::Coordinate::new( loc.x() as i32, loc.y() as i32);
        //self.direction

        let target = coord + self.direction;

        return IndexedField2DLocation::new(target.x as u32, target.y as u32);

        //
    }
}

impl PlanAction for MovePlanAction {
    fn execute(&self, unit: &Unit, context:  &mut TickaContext) {
        // print!("moving: {:?} to {:?}", unit, self.direction);

        
        let target_location = self.get_target_location(unit, context);
        // let unit_locations = context.unit_locations_mut();
        // let current_location = unit_locations.get_entity_location(&unit);
        
        if let Some(mobile_entity_field) = context.unit_locations_new_mut() {
            mobile_entity_field.set_entity(unit, target_location.x(), target_location.y());
        }
        println!("move_entity( -> { } {}", target_location.x(), target_location.y());
        

        // unit_locations.move_entity(current_location.x(), current_location.y(), target_location.x(), target_location.y(), context);
        // context.unit_locations()
        // let from_location = context.unit_locations().
        
    }

    fn description(&self, unit: &Unit, context: &TickaContext) -> String {

        let l = self.get_target_location(unit, context);
        format!("Moves unit {} to {}-{}",unit.id(), l.x(), l.y() )
    }

    /// returns Some, if the action does a field movement.
    fn move_to_field(&self, unit: &Unit, context: &TickaContext) -> Option<IndexedField2DLocation> {
        Some(self.get_target_location(unit, context))
        
    }
}

