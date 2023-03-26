use crate::{unit::{Unit, UnitPlan, UnitPlanEnum}, ticka_context::TickaContext, unit_idle_plan_action::IdlePlanAction};

use super::traits::IMovePlanner;



// idle is kind of the last resort a unit could do.
// and should always be the last MovePlanner.
pub struct Idle {

}

impl IMovePlanner for Idle {

    fn get_unit_plan(&self, unit: &Unit, context: &TickaContext) -> Option<UnitPlan> {
        Some(UnitPlan::new(unit.clone(), UnitPlanEnum::Idle(IdlePlanAction::new())))
    }
}

