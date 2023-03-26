use hex2d::Direction;

use crate::{unit::{Unit, UnitPlan, UnitPlanEnum}, ticka_context::TickaContext, unit_move_action::MovePlanAction, conflict::UnitPlanMoveConflict};



pub struct DirectionalMovePlanner {

    direction: Direction,
    system: DirectionalMoveSystem

}



impl DirectionalMovePlanner
{
    pub fn get_unit_plan(&self, unit: &Unit, x: u32, y:u32, context: &TickaContext) -> Option<UnitPlan> {

        // builds a unit plan if there are empty fields guiding to self.direction.

        // using all directions here, could easily result into a loop where a unit takes 1 step forward, and then 1 step backward.
        let directions = [self.direction, self.direction + hex2d::Left, self.direction + hex2d::Right];

        for direction in directions.iter() {
            let current_location_coord = hex2d::Coordinate::new(x as i32,y as i32);
            let target_location = current_location_coord + *direction;
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

