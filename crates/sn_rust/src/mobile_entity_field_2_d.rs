use crate::{indexed_field2d_location::IndexedField2DLocation, indexed_field_2_d::IndexedField2D};

use derive_getters::Getters;

pub trait StorageLocationProvider {
    fn get_storage_id(&self) -> usize;
}

/// Stores mobile (moveable) Units and other stuff that can change it's location on the field.
/// holds also the location data 
/// 
pub struct MobileEntityField2D<T: Clone + StorageLocationProvider> {

    field: IndexedField2D<T>,
    unit_locations: Vec<IndexedField2DLocation>    
}


#[derive(Getters, Clone)]
pub struct EntityMoveEvent {
    from_x: u32,
    from_y: u32,
    to_x: u32,
    to_y: u32,
    storage_location: usize
}

// pub struct <T> {
//     field: IndexedField2D<T>
// }

impl<T: Clone + StorageLocationProvider > MobileEntityField2D<T> {

    pub fn move_entity(&mut self, x: u32, y: u32, to_x: u32, to_y: u32) -> Option<T> {

        //let i = self.field.indeces().get(&IndexedField2DLocation::new(x,y));
        //if let Some(l) = i {

        //}

        let mut result: Option<T> =  None;
        let mut movement_o: Option<EntityMoveEvent> =  None;

        if let Some(unit_element) = self.field.get_u32(x, y) {
            let storage_id = unit_element.get_storage_id();

            let mut _old_location = &self.unit_locations[storage_id];

            debug_assert!(_old_location.x() == x);
            debug_assert!(_old_location.y() == y);
            
            self.unit_locations[storage_id] = IndexedField2DLocation::new(to_x, to_y);
            
            movement_o = Some( EntityMoveEvent { from_x: x, from_y: y, to_x, to_y, storage_location: storage_id });
            result =  Some(unit_element.clone());
        }


        if let Some(movement) = movement_o {

            self.field.move_entity(x, y, to_x, to_y);

            //todo: send movement to MPSC Thread.

            return result;

        }

        return result;
    }   
}