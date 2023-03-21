use crate::{indexed_field2d_location::IndexedField2DLocation, indexed_field_2_d::IndexedField2D};



pub trait StorageLocationProvider {
    fn get_storage_id() -> usize;
}

/// Stores mobile (moveable) Units and other stuff that can change it's location on the field.
/// holds also the location data 
/// 
pub struct MobileEntityField2D<T: Clone + StorageLocationProvider> {

    field: IndexedField2D<T>,
    unit_locations: Vec<IndexedField2DLocation>    
}

// pub struct <T> {
//     field: IndexedField2D<T>
// }

impl<T: Clone + StorageLocationProvider > MobileEntityField2D<T> {

    pub fn move_entity(&mut self, x: u32, y: u32, to_x: u32, to_y: u32) {

        //let i = self.field.indeces().get(&IndexedField2DLocation::new(x,y));
        //if let Some(l) = i {

        //}

        if let Some(unit_element) = self.field.get_u32(x, y) {
            
        }
    }   
}