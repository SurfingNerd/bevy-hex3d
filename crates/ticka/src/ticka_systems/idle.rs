use crate::{unit::{Unit, UnitPlan, UnitPlanEnum}, ticka_context::TickaContext, unit_idle_plan_action::IdlePlanAction, ticka::UnitPlanner};




// idle is kind of the last resort a unit could do.
// and should always be the last MovePlanner.
pub struct IdleUnitPlanner {

}

impl UnitPlanner for IdleUnitPlanner {

    fn create_unit_plan(&self, unit: &Unit, context: &crate::ticka_context::TickaReadContext) -> Option<UnitPlan> {
        Some(UnitPlan::new(unit.clone(), UnitPlanEnum::Idle(IdlePlanAction::new())))
    }
}

