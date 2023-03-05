
// use std::future::join;

// use rayon::prelude::*;

use std::future::Future;

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
    tick_count: u64,
    units: OptionalLayer2D<Unit>,
    width: u64,
    unit_plan_function: fn(&Unit) -> UnitPlan ,
}

impl Ticka {

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

        let mut futures: Vec<_> = Vec::new();

        for units in self.units.data.iter() {

            //Tick::get_units_plans()
            futures.push(Ticka::get_units_plans(units));
            
            //let result = future.await;
        }
        


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
}