use hex2d::Direction;

use crate::{unit::{Unit, UnitPlan, UnitPlanEnum}, ticka_context::TickaContext, unit_move_action::MovePlanAction, conflict::UnitPlanMoveConflict, ticka::UnitPlanner};



pub struct DirectionalMovePlanner {

    direction: Direction,
    // system: DirectionalMoveSystem

}

impl DirectionalMovePlanner {

    pub fn new(direction: Direction) -> Self {
        Self {
            direction
        }
    }

}


impl UnitPlanner for DirectionalMovePlanner {
    fn create_unit_plan(&self, unit: &Unit, context: &crate::ticka_context::TickaReadContext) -> Option<UnitPlan> {
        
        // builds a unit plan if there are empty fields guiding to self.direction.

        let current_location_location = context.get_entity_location(unit);

        // using all directions here, could easily result into a loop where a unit takes 1 step forward, and then 1 step backward.
        let directions = [self.direction, self.direction + hex2d::Left, self.direction + hex2d::Right];

        for direction in directions.iter() {
            
            let current_location_coord = hex2d::Coordinate::new(current_location_location.x() as i32,current_location_location.y() as i32);
            let target_location = current_location_coord + *direction;
            if target_location.x < 0 || target_location.y < 0 {
                continue;
            }
            let target_field = context.unit_locations().field().get(target_location.x as usize, target_location.y as usize);
            if target_field.is_none() {
                return Some(UnitPlan::new(unit.clone(), UnitPlanEnum::Move(MovePlanAction::from_single_step(*direction))));
            }
        }

        return None;
    }
}

// 
pub struct DirectionalMoveSystem {
        
}

