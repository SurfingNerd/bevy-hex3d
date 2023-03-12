
// use std::future::join;

// use rayon::prelude::*;

use std::future::Future;

use sn_rust::indexed_field_2D::IndexedField2D;

use crate::layer_2D::{Layer2D, OptionalLayer2D};
// use layer_2D::

use crate::unit::*;

// Unit Planner must be copyable,
// since each thread get's it's own UnitPlanner
// 
pub trait UnitPlanner {

    fn make_plan(&self, unit: &Unit) -> UnitPlan; 
}

pub struct Ticka {
    tick_counter: u64,
    units: IndexedField2D<Unit>,
    unit_plan_function: fn(&Unit) -> UnitPlan ,
}

impl Ticka {

    pub fn new(width: usize, height: usize, unit_plan_function: fn(&Unit) -> UnitPlan) -> Self {
        Ticka { tick_counter: 0, units: IndexedField2D::new(width, height), unit_plan_function }
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

    async fn get_unit_plans_2D(&self) {

        //let futures: Vec<dyn Future<Output = TUnitPlan>>  = Vec::new();

        // DOTO:
        // here the big performance magic has to be done.
        // for initial architecture setup, 
        //     
         
        // let mut futures: Vec<_> = Vec::new();

        // maybe it is better to have a Plan Array that is indexed by the Unit ID,
        // instead of the order of the IndexedField2DLocation
        let mut plans = Vec::<UnitPlan>::with_capacity( self.units.indeces().len());


        for index in self.units.indeces().iter() {

            if let Some(unit) = self.units.get_u32(index.x(), index.y()) {
                let plan = (self.unit_plan_function)(unit);
                plans.push(plan);
            } else {
                panic!("Unexpected: every coordinate should match a unit!");
            }
            
        }

        // for units in self.units.iter data.iter() {

        //     //Tick::get_units_plans()
        //     futures.push(Ticka::get_units_plans(units));
            
        //     //let result = future.await;
        // }
        


        // for future in futures.iter() {

        //     let res = future.await();
            

        // }
        // join!(future_one, future_two);
    }

    // executes one tick
    pub fn tick(&self) {


        

        // on every tick, 
        // we seperate the work in different threads,
        // that do the Planning Step

        // #step 1: Planning

        // for vec in self.units.data.par_iter() {

        //     // pass every vec to a threadpool.
            
        // }

        //  self.units.data

        // self.units.data.par_iter().map( |units|  { 

        // })

        // every units does it's planning

        // #Step 2: conflict detection

        // if a conflict occurs 
        // example: (2 units trying to access the same field)

        // # Step: conflict-replan-step

        // Units might replan their action, based on konflict knowledge.

        // # Conflict resolving step

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
        &mut self.units
    }

    pub fn units(&self) -> &IndexedField2D<Unit> {
        &self.units
    }

}