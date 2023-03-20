use sn_rust::field_2_d::Field2D;
use sn_rust::indexed_field_2_d::IndexedField2D;
use crate::conflict::{UnitPlanMoveConflict, UnitPlanMoveConflicts};
use crate::ticka_context::TickaContext;
use crate::unit::*;

// Unit Planner must be copyable,
// since each thread get's it's own UnitPlanner
// 
pub trait UnitPlanner {

    fn make_plan(&self, unit: &Unit) -> UnitPlan; 
}

pub struct Ticka {
    tick_counter: u64,
    units_field: IndexedField2D<Unit>,
    terrain_height: Field2D<f32>,
    unit_plan_function: fn(&Unit) -> UnitPlan ,
}

// Threading concept:
// Process 


impl Ticka {

    pub fn new(width: usize, height: usize, unit_plan_function: fn(&Unit) -> UnitPlan) -> Self {
        Ticka { tick_counter: 0, units_field: IndexedField2D::new(width, height), unit_plan_function, terrain_height: Field2D::new(width, height) }
    }

    async fn get_units_plans(units: &Vec<Option<Unit>>) -> Vec<Option<UnitPlan>> {

        let mut result : Vec<Option<UnitPlan>> = Vec::with_capacity(units.len());

        for unit_option  in units {
            if let Some(unit) = unit_option {

                
                
            } else {
                result.push(None);
            }
        }


        return result;
    }

    fn get_unit_plans_2d(&self) ->  Vec::<UnitPlan>{

        //let futures: Vec<dyn Future<Output = TUnitPlan>>  = Vec::new();

        // DOTO:
        // here the big performance magic has to be done.
        // for initial architecture setup, 
        //     
         
        // let mut futures: Vec<_> = Vec::new();

        // maybe it is better to have a Plan Array that is indexed by the Unit ID,
        // instead of the order of the IndexedField2DLocation
        let mut plans = Vec::<UnitPlan>::with_capacity( self.units_field.indeces().len());

        

        for index in self.units_field.indeces().iter() {

            if let Some(unit) = self.units_field.get_u32(index.x(), index.y()) {
                let plan = (self.unit_plan_function)(unit);
                plans.push(plan);
            } else {
                panic!("Unexpected: every coordinate should match a unit!");
            }
        }

        return plans;
    }

    fn get_unit_plan_conflicts(&self, plans: &Vec<UnitPlan>) -> Vec<UnitPlanMoveConflict> {
        // for progressing further, we ignore movement conflicts right now.
        return Vec::new();
    }

    fn replan_conflicts(&self, plan_conflicts: &Vec<UnitPlanMoveConflict>, plans: &Vec<UnitPlan>) {
        // for progressing further, we do not do a replaning right now.

        // let plan_option = plans.get_mut(1);

        // if let Some(mut plan) = plan_option {
            
        // }
    }

    fn execute_plans(&self, plans: &Vec<UnitPlan>) {
        
        // excutes the plans, 
        // and resolves the conflicts the hard way.


        let mut conflicts = UnitPlanMoveConflicts::from_plans(plans);


        // TODO: handle conflicts here
        // let conflicting_plan_groups = conflicts.get_conflicting_plans();

        // for conflicting_plan_group in conflicting_plan_groups  {
        //     let location = conflicting_plan_group.0;
        //     let conflict  = conflicting_plan_group.1;

            
        //     for plan in conflict.plans().iter() {
                
        //     }
        // }


        // all unhandled conflicts will just idle

        let mut context = TickaContext {};

        for unit_plan in conflicts.non_conflicting_plans().iter_mut() {
            unit_plan.execute(&mut context);
        }


    }

    // executes one tick
    pub fn tick(&self) {


        

        // on every tick, 
        // we seperate the work in different threads,
        // that do the Planning Step

        // #step 1: Planning

        let mut plans = self.get_unit_plans_2d();

        // every units does it's planning

        // #Step 2: conflict detection

        let plan_conflicts = self.get_unit_plan_conflicts(&plans);
        

        // if a conflict occurs 
        // example: (2 units trying to access the same field)

        // # Step: conflict-replan-step

        self.replan_conflicts(&plan_conflicts, &mut plans);

        // Units might replan their action, based on konflict knowledge.

        // # Conflict resolving step

        self.execute_plans(&plans);

        // some conflicts are still their after, and must be resolved the hard way,
        // may the better win, or soever

        // Conflict Resolving resolves into s set of executed actions
        // Example:
        // 2 swordfighters hit each other
        // - a swordfighter might be skilled in Pary and can take advantage.
        // - a parry deflects all damage taken.
        // - a parrying fighter can hit another unit
        // - a defender on the parry can block, or second hand parry
        // there is no limit how often parry - parry hit - parry - parry hit
        // chain can take place. 
        // but since the parry skill of both reduces their parry chance,
        // a long chain is astonomical rare

        // step ?: Ground Effects

        // execute steps that are ground evects
        // GroundEffects might be stored in a 2D Array for each type,
        // used by on System.
        // example ground effects:
        // 
        // - Flowing
        // - - Lava
        // - - Water
        // - - Moistness
        // - - Outbreaking Fire attaches to flamable neighbours
        // - - Bee's Pollinution Cloud
        // - - 
    }

    pub fn units_mut(&mut self) -> &mut IndexedField2D<Unit> {
        &mut self.units_field
    }

    pub fn units(&self) -> &IndexedField2D<Unit> {
        &self.units_field
    }

}