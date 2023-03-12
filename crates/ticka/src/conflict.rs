use std::collections::BTreeMap;
use sn_rust::indexed_field2d_location::IndexedField2DLocation;

use crate::unit::{Unit, UnitPlan};

pub struct UnitPlanMoveConflicts {

  
  target_locations: BTreeMap<IndexedField2DLocation, UnitPlanMoveConflict>
}

impl UnitPlanMoveConflicts {

  fn new() -> Self {
    UnitPlanMoveConflicts { target_locations: BTreeMap::new() }
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
        conflict.units.push(plan.unit().clone());
      } else {
        self.target_locations.insert(target_location, UnitPlanMoveConflict { units: vec![plan.unit().clone()] });
      }
    }
  }

  fn clear_non_conflicting_plans(&mut self) {

    self.target_locations.retain(|_l,c| c.units.len() > 1);

  }

  pub fn get_conflicting_plans(&self) -> &BTreeMap<IndexedField2DLocation, UnitPlanMoveConflict> {
    &self.target_locations
  }

}

pub struct UnitPlanMoveConflict {

  units: Vec<Unit>
}

