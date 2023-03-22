use sn_rust::indexed_field2d_location::IndexedField2DLocation;
use crate::unit_move_action::MovePlanAction;
use crate::{unit::Unit, ticka_context::TickaContext};



pub enum UnitPlanAction {
    Move(MovePlanAction)
}

pub trait PlanAction where Self: Clone + Sized {

    /// executes the actions, changing values on the context
    fn execute(&self, unit: &Unit,  context: &mut TickaContext);
    
    /// text description of planned Action
    fn description(&self, unit: &Unit,  context: &TickaContext) -> String;

    /// returns Some, if the action does a field movement.
    fn move_to_field(&self, unit: &Unit, context: &TickaContext) -> Option<IndexedField2DLocation> {
        None
    }
}

