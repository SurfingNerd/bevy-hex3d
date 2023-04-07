use crate::{unit::{Unit, UnitPlan}, ticka_context::TickaContext, conflict::UnitPlanMoveConflict};




// pub trait IMovePlanner {
//     fn get_unit_plan(&self, unit: &Unit, context: &TickaContext) -> Option<UnitPlan> {
//         return None;
//     }

//     /// allows the system to react to a unit moved.
//     fn on_unit_moved(&self, unit: &Unit,  from_x: u32, from_y: u32, to_x: u32, to_y: u32, unit_plan: &UnitPlan, context: &TickaContext) {
//     }

//     /// allows the system to react to a unit moved.
//     fn on_unit_stay(&self, unit: &Unit,  x: u32, y: u32, unit_plan: &UnitPlan, context: &TickaContext) {
//     }


//     fn resolve_conflict(&self, unit: &UnitPlan, conflict: &UnitPlanMoveConflict) -> Option<UnitPlan> {
//         return None;
//     }
// }



