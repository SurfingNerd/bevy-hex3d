use derive_getters::Getters;
use hex2d::{Coordinate, Direction, Spacing};
use sn_rust::field_2_d::Field2D;
use sn_rust::traits::IField2D;

use crate::{unit::{Unit, UnitPlan, UnitPlanEnum}, ticka_context::{TickaContext, TickaReadContext}, unit_idle_plan_action::IdlePlanAction, conflict::UnitPlanMoveConflict, ticka::{Ticka, UnitPlanner}};


/// units leave behind phermones on move.
/// Units searches near fields for pheromones,
/// and if they detect some, that did not originate by themself,
/// then they follow this tracks. 
#[derive(Getters)]
pub struct PheromoneSystem  {
    unit_planer: PheromoneSystemUnitPlaner,

}

#[derive(Getters, Clone, Debug)]
pub struct PheromoneSystemUnitPlaner{

    /// storage location of pheromone field. 
    field_index: usize,

    unit_pheromone_sense_range: u64,

    /// unity try to follow pheromones, but try to avoid units of the same type
    unit_split_up_target: u64,

    direction_angles: [f32; 6],

    spacing: Spacing<f32>
}

impl UnitPlanner for PheromoneSystemUnitPlaner {

    fn create_unit_plan(&self, unit: &Unit, context: &TickaReadContext) -> Option<UnitPlan> {
        
        // just for test world
        if *unit.id() < 100 {
            return None;
        }

        let unit_location = context.get_entity_location(unit); // context.unit_locations(). .get(unit.id()).unwrap();
        
        let unit_coordinate = hex2d::Coordinate::new(unit_location.x() as i32, unit_location.y() as i32);
        // search for units of same type.

        // sum of attraction (pheromone), or repelling (same unit) forces.
        let mut forces : [f32; 6] = [0.0; 6];

        let mut current_force_multiplier: f32 = self.unit_pheromone_sense_range as f32 - 1.0;

        for range in 1..self.unit_split_up_target {
            // hex2d::Direction::all()

            current_force_multiplier -= 1.0;

            // here a 2D Index could be usefull.
            // we would need 
            
            // we look around in growing rings

            for other_unit_location in  context.unit_ring_iter(unit_location.x() as i32, unit_location.y() as i32, range as i32) {
                
                if other_unit_location.x < 0 || other_unit_location.y < 0 {
                    continue;
                }

                if let Some(other_unit) = context.unit_locations().get(other_unit_location.x as u32, other_unit_location.y as u32) {
                    let is_repelling = *other_unit.id() > 100; 
                    if is_repelling {

                        // we need to figure out the 
                        let source_pixel = unit_coordinate.to_pixel(context.spacing().clone());
                        let target_pixel = other_unit_location.to_pixel(context.spacing().clone());

                        let angle = self.get_direction_angle(unit_coordinate, other_unit_location);

                        let direction_index = self.get_direction_index_from_source_to_target(unit_coordinate, other_unit_location);

                        forces[direction_index] -= current_force_multiplier;

                        // we apply 50% of the force to the direction of the other unit.
                        // and 50% to the direction to the 2 neighbour directions.

                        // we need to calculate the angle of the source_pixel to target_pixel vector.
                        // let vector = 
                        // let angle = vector.1.atan2(vector.0);

                    }
                }
            }

        }
        
        return Some(UnitPlan::new(unit.clone(), UnitPlanEnum::Idle(IdlePlanAction::new())));
    }
}

impl PheromoneSystemUnitPlaner {

        /// @param unit_split_up_target: unity try to follow pheromones, but try to avoid units of the same type.
        pub fn new(unit_split_up_target: u64, unit_pheromone_sense_range: u64, field_index: usize, spacing: Spacing<f32>) -> Self {

            let mut direction_angles: [f32; 6] = [0.0 ;6];
            match spacing {
                hex2d::Spacing::FlatTop(_) => {
                    for d in 0..6 {
                        direction_angles[d] = Direction::all()[d].to_radians_flat();
                    }
                },
                hex2d::Spacing::PointyTop(_) => {
                    for d in 0..6 {
                        direction_angles[d] = Direction::all()[d].to_radians_pointy();
                    }
                    
                }
            }

            PheromoneSystemUnitPlaner {
                field_index,
                unit_split_up_target,
                unit_pheromone_sense_range,
                direction_angles,
                spacing
            }
        }

    pub fn get_field<'a>(&self, context: &'a TickaContext) -> &'a Field2D<f32> {
        &context.fields()[self.field_index]
    }

    pub fn get_pheromone(&self, context: &TickaContext, x: usize, y: usize) -> f32 {
        self.get_field(context).get(x, y).clone()
    }

    pub fn get_field_mut<'a>(&self, context: &'a mut TickaContext) -> &'a mut Field2D<f32> {
        &mut context.fields_mut()[self.field_index]
    }

    fn get_direction_vector(&self, a: Coordinate, b: Coordinate) -> (f32, f32) {
        let a_pixel = a.to_pixel(self.spacing);
        let b_pixel = b.to_pixel(self.spacing);

        return (b_pixel.0 - a_pixel.0, a_pixel.1 - b_pixel.1);
        //return 
    }

    fn get_direction_angle(&self, a: Coordinate, b: Coordinate) -> f32 {
        let a_pixel = a.to_pixel(self.spacing);
        let b_pixel = b.to_pixel(self.spacing);

        let vector = (b_pixel.0 - a_pixel.0, a_pixel.1 - b_pixel.1);
        return vector.1.atan2(vector.0);
    }

    fn get_direction_index_from_source_to_target(&self, unit_coordinate: Coordinate, other_unit_location: Coordinate) -> usize {
        let angle = self.get_direction_angle(unit_coordinate, other_unit_location);
        for d in 0..6  {
            let direction_angle = self.direction_angles[d];

            let next_angle = self.direction_angles[if d == 5 { 0 } else { d+1 }];
            if angle > direction_angle && angle < next_angle {
                return d;
            }
            // let direction_angle = direction.to_angle();
            // let angle_diff = (direction_angle - angle).abs();
            
        }

        return 0;
    }

}

impl PheromoneSystem {
    /// create a new pheromone system
    /// @param unit_split_up_target: unity try to follow pheromones, but try to avoid units of the same type.
    pub fn new(ticka: &mut Ticka, unit_split_up_target: u64, unit_pheromone_sense_range: u64) -> Self {
        PheromoneSystem {
            unit_planer: PheromoneSystemUnitPlaner::new(unit_split_up_target, unit_pheromone_sense_range, ticka.register_field_f32(), ticka.spacing()),
        }
    }

    // pub fn init(&mut self, ticka: &mut Ticka) {
        
    // }


}