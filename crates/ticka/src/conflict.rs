use std::collections::BTreeMap;
use sn_rust::indexed_field2d_location::IndexedField2DLocation;

use crate::unit::{Unit, UnitPlan, MovePlanAction};

pub struct UnitPlanMoveConflicts {

  non_conflicting_plans: Vec<UnitPlan>,
  target_locations: BTreeMap<IndexedField2DLocation, UnitPlanMoveConflict>
}

impl UnitPlanMoveConflicts {

  fn new() -> Self {
    UnitPlanMoveConflicts { target_locations: BTreeMap::new(), non_conflicting_plans: Vec::new() }
  }

  /// builds Planing conflicts from Plans.
  pub fn from_plans(plans: &Vec<UnitPlan>) -> Self {
    let mut conflicts = UnitPlanMoveConflicts::new();
    for plan in plans.iter() {
      
      conflicts.register_plan(plan);
    }
    conflicts.clear_non_conflicting_plans();
    return conflicts;
  }


  fn register_plan(&mut self, plan: &UnitPlan) {
    if let Some(target_location) = plan.move_to_field() {

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
    self.target_locations.retain(|_l,c| c.plans.len() > 1);

  }

  pub fn get_conflicting_plans(&self) -> &BTreeMap<IndexedField2DLocation, UnitPlanMoveConflict> {
    &self.target_locations
  }

  pub fn non_conflicting_plans(&self) -> &Vec<UnitPlan> {
    &self.non_conflicting_plans
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

