use sn_rust::indexed_field2d_location::IndexedField2DLocation;

use crate::{unit::Unit, ticka_context::TickaContext, unit_plan_action::PlanAction};
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
        IndexedField2DLocation::new(0,0)
    }
}

impl PlanAction for MovePlanAction {
    fn execute(&self, unit: &Unit, context:  &mut TickaContext) {
        print!("moving: {:?} to {:?}", unit, self.direction)
    }

    fn description(&self, unit: &Unit, context: &TickaContext) -> String {

        let l = self.get_target_location(unit, context);
        format!("Moves unit {} to {}-{}",unit.id(), l.x(), l.y() )
    }
}

