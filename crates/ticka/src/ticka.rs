use std::sync::mpsc::Sender;
use std::sync::Mutex;

use hex2d::Spacing;
use sn_rust::field_2_d::Field2D;
use sn_rust::indexed_field_2_d::IndexedField2D;
use sn_rust::mobile_entity_field_2_d::MobileEntityField2D;

use crate::conflict::{UnitPlanMoveConflict, UnitPlanMoveConflicts};
use crate::ticka_context::{TickaContext, TickaReadContext};
use crate::ticka_systems::directional_move::DirectionalMovePlanner;
use crate::ticka_systems::idle::IdleUnitPlanner;
use crate::ticka_systems::pheromone_system::PheromoneSystem;
use crate::unit::*;
use crate::unit_move_action::UnitMoveInstance;

// Unit Planner must be copyable,
// since each thread get's it's own UnitPlanner
//
pub trait UnitPlanner : Sync + Send {
    fn create_unit_plan(&self, unit: &Unit, context: &TickaReadContext) -> Option<UnitPlan>; 
}

pub struct Ticka {
    tick_counter: u64,
    units_field: MobileEntityField2D<Unit>,

    /// Maybe you want to write yourself a wrapper.
    /// terrain_height: Field2D<f32>,
    /// water_amount: Field2D<f32>,
    /// lava_amount: Field2D<f32>,
    /// fire_amount: Field2D<f32>,
    /// flammability: Field2D<f32>,
    /// moisture: Field2D<f32>,
    /// greenery: Field2D<f32>,
    fields: Vec<Field2D<f32>>,

    // unit type array matches the unit planer indexs
    unit_type: Vec<usize>,

    unit_planners: Vec<Vec<Box<dyn UnitPlanner>>>,

    unit_plan_function: fn(&Unit, &TickaContext) -> UnitPlan,

    unit_move_sender: Option<Mutex<Sender<UnitMoveInstance>>>,

    spacing: Spacing<f32>
}

// Threading concept:
// Process

impl Ticka {
    pub fn new(
        width: usize,
        height: usize,
        num_of_fields: usize,
        unit_plan_function: fn(&Unit, &TickaContext) -> UnitPlan,
        unit_move_sender_o: Option<Sender<UnitMoveInstance>>,
        spacing: Spacing<f32>
    ) -> Self {
        let mut fields: Vec<Field2D<f32>> = Vec::new();

        for _ in 0..num_of_fields {
            fields.push(Field2D::new(width, height));
        }

        let mutex_option: Option<Mutex<Sender<UnitMoveInstance>>> = None;
        let mutex = if let Some(sender) = unit_move_sender_o {
            Some(Mutex::new(sender))
        } else {
            None
        };
        
        // let vec = vec!(Field2D::new(width, height));
        let mut result = Ticka {
            tick_counter: 0,
            units_field: MobileEntityField2D::new(width, height, Unit::new(0)),
            unit_plan_function,
            fields,
            unit_move_sender: mutex_option,
            unit_type: Vec::new(),
            unit_planners: Vec::new(),
            spacing
        };

        // unit type 0 means a dead unit, 
        // we do not need planers for the dead.
        result.unit_planners.push(Vec::new());

        result.add_example_planners();



        return result;
    }

    pub fn register_unit_type(&mut self, unit_planners: Vec<Box<dyn UnitPlanner>>) -> usize {
        self.unit_planners.push(unit_planners);
        let result = self.unit_planners.len() - 1;
        println!("unit type {result} registered.");
        return result;
    }

    fn add_example_planners(&mut self) {

        
        // let mut unit_planners: Vec<Vec<Box<dyn UnitPlanner>>> = Vec::new();
        // let mut unit_planer: Vec<Box<dyn UnitPlanner>> = Vec::new();

        let system = PheromoneSystem::new(self, 3, 4);
        let planer = system.unit_planer();


        
       
        // self.unit_planners.push(Vec::new());
        // self.unit_planners.push(Vec::new());

        //let pheromone_planer_box = 

        let mut unit_pheromone_planners : Vec<Box<dyn UnitPlanner>> = Vec::new();
        let mut unit_idle_planners : Vec<Box<dyn UnitPlanner>> = Vec::new();


        //let pheromone_system : dyn UnitPlanner = planer.clone();
        unit_pheromone_planners.push( Box::new(planer.clone()));

        let directional_move = DirectionalMovePlanner::new(hex2d::Direction::XY);
        unit_pheromone_planners.push(Box::new(directional_move));

        let idle_planer = IdleUnitPlanner {};
        unit_pheromone_planners.push(Box::new(idle_planer));
        

        unit_idle_planners.push(Box::new(IdleUnitPlanner {}));



        self.register_unit_type(unit_pheromone_planners);
        self.register_unit_type(unit_idle_planners);


    }

    fn get_unit_plan(&self, unit: &Unit, context: &TickaReadContext) -> Option<UnitPlan> {

        let planer_index = self.unit_type[*unit.id() as usize];

        //for planers_d_1 in self.unit_planners.iter() {
            // for planer in planers_d_1.iter() {
            //     if let Some(plan) = planer.create_unit_plan(unit, context) {
            //         //result.push(Some(plan));
            //         //break;
            //         return Some(plan);
            //     }
            // }
        //};

        for planer in self.unit_planners[planer_index].iter() {
            if let Some(plan) = planer.create_unit_plan(unit, context) {
                //result.push(Some(plan));
                //break;
                return Some(plan);
            }
        }

        None
    }
    
    

    // fn get_units_plans(&self, units: &Vec<Option<Unit>>, context: &TickaContext) -> Vec<Option<UnitPlan>> {
    //     let mut result: Vec<Option<UnitPlan>> = Vec::with_capacity(units.len());

    //     // todo:
    //     // here we got a lot of paralellisation opportunitie.
    //     // we can just throw all the units into a job pool,
    //     // and let them calculate their plans.
    //     // and return sync once they are finished.
        
    //     for unit_option in units {
    //         if let Some(unit) = unit_option {
    //             // figure out the unit type here,
    //             // or especialy, what planner group we should use.

    //             // for now, we ask all planer groups.
    //             let current_result: Option<UnitPlan> = None;

    //             let plan = self.get_unit_plan(unit);
                
                    
                
    //         }
    //     }

    //     return result;
    // }


    pub fn register_field_f32(&mut self) -> usize {
        let new_field = Field2D::<f32>::new(
            self.units_field.field().width() as usize,
            self.units_field.field().height() as usize,
        );
        self.fields.push(new_field);
        return self.fields.len() - 1;
    }

    // pub fn register_indexed_field_f32(&mut self) -> usize {

    //     let new_field = IndexedField2D::<f32>::new(self.units_field.field().width() as usize, self.units_field.field().height() as usize);
    //     self.indexed_fields.push(new_field);
    //     return self.fields.len() - 1;
    // }

    fn get_unit_plans_2d(&mut self) -> Vec<UnitPlan> {
        //let futures: Vec<dyn Future<Output = TUnitPlan>>  = Vec::new();

        // DOTO:
        // here the big performance magic has to be done.
        // for initial architecture setup,
        //

        // let mut futures: Vec<_> = Vec::new();

        // maybe it is better to have a Plan Array that is indexed by the Unit ID,
        // instead of the order of the MobileEntityField2DLocation
        let mut plans = Vec::<UnitPlan>::new(); // .with_capacity( self.units_field.field().indeces().len());

        //println!("expecting {} plans", self.units_field.field().indeces().len());

        // spacing: Spacing<f32>,
        // unit_move_sender: Option<Sender<UnitMoveInstance>>,
        // unit_locations: &'a MobileEntityField2D<Unit>,
        // fields: &'a Vec<Field2D<f32>>,
        // unit_locations_new: Option<MobileEntityField2D<Unit>>

        let mut context = TickaReadContext::new(
            &self.units_field,
            &self.fields,
            self.spacing,
        );

        let field = context.unit_locations().field();

        for index in field.indeces().iter() {
            if let Some(unit) = field.get_u32(index.x(), index.y()) {

                if let Some(plan) = self.get_unit_plan(unit, &context) {
                    plans.push(plan);
                }

                //let plan = (self.unit_plan_function)(unit, &context);
                // let plan = self.create_unit_plan(unit, &context);
                // println!("creating plan for x {} y {}: {:?}", index.x(), index.y(), plan);
                
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

    fn execute_plans(&mut self, plans: &Vec<UnitPlan>) {
        // excutes the plans,
        // and resolves the conflicts the hard way.

        let context = TickaContext::new(&mut self.units_field, None, None, &mut self.fields, self.spacing);

        let mut conflicts = UnitPlanMoveConflicts::from_plans(plans, &context);

        let conflicting_plans = conflicts.get_conflicting_plans();
        let non_conflicting_plans = conflicts.non_conflicting_plans();

        println!(
            "pans: {} conflicts {} conflict free plans: {}",
            plans.len(),
            conflicting_plans.len(),
            non_conflicting_plans.len()
        );
        // TODO: handle conflicts here
        // let conflicting_plan_groups = conflicts.get_conflicting_plans();

        // for conflicting_plan_group in conflicting_plan_groups  {
        //     let location = conflicting_plan_group.0;
        //     let conflict  = conflicting_plan_group.1;

        //     for plan in conflict.plans().iter() {

        //     }
        // }

        // all unhandled conflicts will just idle

        let unit_move_sender: Option<Sender<UnitMoveInstance>> =
            if let Some(unit_move_sender) = &self.unit_move_sender {
                Some(unit_move_sender.lock().expect("").clone())
            } else {
                None
            };

        let height = self.units_field.height();
        let width = self.units_field.width();
        let new_field =
            MobileEntityField2D::<Unit>::new(width as usize, height as usize, Unit::new(0));

        let mut context =
            TickaContext::new(&mut self.units_field, Some(new_field), unit_move_sender, &mut self.fields, self.spacing);

        for unit_plan in conflicts.non_conflicting_plans_mut().iter_mut() {
            println!(
                "executing non conflicting unit plan for unit: {:?}",
                unit_plan
            );
            unit_plan.execute(&mut context);
        }

        // resolve conflicts here.
        let mut resolved_conflict_plans = conflicts.resolve_conflicts(&mut context);

        for unit_plan in resolved_conflict_plans.iter_mut() {
            println!(
                "executing resolved conflict unit plan for unit: {:?}",
                unit_plan
            );

            unit_plan.execute(&mut context);
        }

        // this hit's that units might despawn during the conflict resolve.
        // debug_assert!(context.unit_locations().field().indeces().len() == context.unit_locations_new_mut().as_ref().expect("").field().indeces().len());

        // now point to the new field.
        self.units_field = context.unit_locations_new_mut().take().unwrap();
    }

    // executes one tick
    pub fn tick(&mut self) {
        print!("Ticka tick!");

        // on every tick,
        // we seperate the work in different threads,
        // that do the Planning Step

        // #step 1: Planning, this step can be done in parallel,
        // since UnitPlaner don't care about each other.
        // and the conflict resolution is a seperate step.
        let mut plans = self.get_unit_plans_2d();

        // every units does it's planning

        // #Step 2: conflict detection

        let plan_conflicts = self.get_unit_plan_conflicts(&plans);

        // if a conflict occurs
        // example: (2 units trying to access the same field)

        // # Step: conflict-replan-step

        self.replan_conflicts(&plan_conflicts, &mut plans);

        // Units might replan their action, based on konflict knowledge.

        // # Conflict resolving stepget_unit_plans_2d

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

    pub fn units_mut(&mut self) -> &mut MobileEntityField2D<Unit> {
        &mut self.units_field
    }

    pub fn units(&self) -> &MobileEntityField2D<Unit> {
        &self.units_field
    }

    fn create_ticka_context(&mut self) -> TickaContext {
        return TickaContext::new(&mut self.units_field, None, None, &mut self.fields, self.spacing);
    }

    // fn create_unit_plan(&self, unit: &Unit, context: &TickaContext) -> UnitPlan {
    //     // each unit belongs to a type that has it's own planer.
    // }

    pub fn spawn_unit(&mut self, x: u32, y: u32, unit_type: usize) -> Unit {
        let result = self.units_mut().spawn_entity(x, y);

        let len_of_planers = self.unit_planners.len();
        debug_assert!(unit_type < len_of_planers, "unit_type {unit_type} not registered yet. make sure to register it first. registed: {len_of_planers}");
        
        self.unit_type.push(unit_type);

        // todo: handle other unit properties here as well.

        return result;
    }

    pub fn spacing(&self) -> Spacing {
        self.spacing
    }
}
