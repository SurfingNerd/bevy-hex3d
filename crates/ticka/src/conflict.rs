use std::collections::BTreeMap;
use sn_rust::indexed_field2d_location::IndexedField2DLocation;

use crate::{unit::{Unit, UnitPlan}, ticka_context::TickaContext, unit_idle_plan_action::IdlePlanAction};


pub struct UnitPlanMoveConflicts {

  non_conflicting_plans: Vec<UnitPlan>,
  target_locations: BTreeMap<IndexedField2DLocation, UnitPlanMoveConflict>
}

impl UnitPlanMoveConflicts {

  fn new() -> Self {
    UnitPlanMoveConflicts { target_locations: BTreeMap::new(), non_conflicting_plans: Vec::new() }
  }

  /// builds Planing conflicts from Plans.
  pub fn from_plans(plans: &Vec<UnitPlan>, context: &TickaContext) -> Self {
    let mut conflicts = UnitPlanMoveConflicts::new();
    for plan in plans.iter() {
      
      conflicts.register_plan(plan, context);
    }

    println!("moves: {}", conflicts.target_locations.len());
    conflicts.clear_non_conflicting_plans();
    return conflicts;
  }


  fn register_plan(&mut self, plan: &UnitPlan, context: &TickaContext) {
    if let Some(target_location) = plan.move_to_field(context) {

      // Note:
      // it is expensive but simple to create a Vec for each Unit movement plan.
      // we could optimize this by using a Map with a location and a Unit, so filtering 
      // out the existing Unit plans becomes much faster, 
      // and we do not allocates tons of Vecs, we are going to delete later on.
      // used_locations: BTreeMap<IndexedField2DLocation, Unit>,
      

      // check if there is already a conflict registered for this location.
      if let Some(conflict) = self.target_locations.get_mut(&target_location) {
        
        //if planAction is UnitMoveAction

        conflict.plans.push(plan.clone());

        //conflict.units.push();
      } else {
        self.target_locations.insert(target_location, UnitPlanMoveConflict { plans: vec![plan.clone()] });
      }
    }
  }

  fn clear_non_conflicting_plans(&mut self) {

    // store the nonconflicting plans.
    let mut filtered = self.target_locations.iter().filter(|(l,c)| c.plans().len() == 1);

    let mapped = filtered.map(|(l,c)| c.plans().first().expect("1").clone());
    self.non_conflicting_plans = mapped.collect(); //drained.filter(|_l, c| c.units().len());
    // self.non_conflicting_plans =
    // self.target_locations.retain(|_l,c| c.plans.len() > 1);

  }

  pub fn get_conflicting_plans(&self) -> &BTreeMap<IndexedField2DLocation, UnitPlanMoveConflict> {
    &self.target_locations
  }

  pub fn non_conflicting_plans_mut(&mut self) -> &mut Vec<UnitPlan> {
    &mut self.non_conflicting_plans
  }

  pub fn non_conflicting_plans(&self) -> &Vec<UnitPlan> {
    &self.non_conflicting_plans
  }

  pub(crate) fn resolve_conflicts(&self, context: &mut TickaContext) -> Vec<UnitPlan> {
      
      let mut result = Vec::new();

      for (l , c) in self.target_locations.iter() {
        self.resolve_conflict(c, context, &mut result);  
      }

      return result;
  }


  fn resolve_conflict(&self, conflict: &UnitPlanMoveConflict, context: &mut TickaContext, result: &mut Vec<UnitPlan>)  {
    
    if conflict.plans.len() == 1 {
     // conflict plans with a len of 1 (those are not a conflict) already got executed.
     // they exist purely for looking up the plan for a unit.
     return;
    }

    let mut conflict_has_executed_plan: usize = usize::MAX; // we mean NULL here.

    for i in 0..conflict.plans.len() {
      // todo: use fast pseudo rng here
      let plan = &conflict.plans[i]; 
      

      if plan.is_staying(context) {
        // there must be only one idling unit on this field.
        debug_assert!( conflict_has_executed_plan == usize::MAX ); 
        result.push(plan.clone());

        conflict_has_executed_plan = i;

        // we could break here for better performance,
        // but we want to check if there are more than one idling unit on this field.
        // so we catch errors in the code very ealry.

      }
   }

   // we might have idled with 1 unit, let's check if other units can idle as well.
   for i in 0..conflict.plans.len() { 
    let plan = &conflict.plans[i];
    if i  ==  conflict_has_executed_plan {
      // this unit can do it's action as it wishes.
      continue;
    } 

    let current_location = context.get_entity_location(plan.unit());
    let x = current_location.x(); 
    let y = current_location.y();

    if let Some(new_locations) = context.unit_locations_new_mut() {
      // debug_assert!()

      if let Some(own_field_challanger) = new_locations.field().get_u32(x, y) {
        // either Panic here, or handle this kind of conflicts in a separate way.
        println!("conflict between 2 Units on {x}-{y} : {:?} and unit {:?} - no further processing, unit might despawn.", plan, own_field_challanger);          
        // currently we do not place this unit on the world.
        // so it just disapears here.
        continue;
      }
    }

    if conflict_has_executed_plan == usize::MAX {
      // if we got still noone that acted, we take the first chance.
      result.push(plan.clone());
      conflict_has_executed_plan = i;
      continue;
    }
    
    result.push(UnitPlan::new(plan.unit().clone(), crate::unit::UnitPlanEnum::Idle(IdlePlanAction::new())));    
   }

  }

}

pub struct UnitPlanMoveConflict {

  plans: Vec<UnitPlan>
}


impl UnitPlanMoveConflict {

  pub fn plans(&self ) -> &Vec<UnitPlan> {
    &self.plans
  }
}

