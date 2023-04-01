use crate::{indexed_field2d_location::IndexedField2DLocation, indexed_field_2_d::IndexedField2D};

use derive_getters::Getters;
use std::fmt::Debug;

pub trait StorageLocationProvider {
    fn get_storage_id(&self) -> usize;
    fn create_from_prototype(&self, storage_id: usize) -> Self;
}

/// Stores mobile (moveable) Units and other stuff that can change it's location on the field.
/// holds also the location data 
/// 
#[derive(Getters, Debug)]
pub struct MobileEntityField2D<T: Clone + StorageLocationProvider + Debug> {

    field: IndexedField2D<T>,
    unit_locations: Vec<IndexedField2DLocation>,
    entity_prototype: T
}

// impl<T: Clone + StorageLocationProvider> MobileEntityField2D<T> {

//     pub fn new(x: usize, y: usize, entity_prototype: T) -> Self {

//         MobileEntityField2D {
//             field: IndexedField2D::new(x,y),
//             unit_locations: Vec::new(),
//             entity_prototype
//         }
//     }

//     pub fn spawn_entity(&mut self, x: u32, y: u32) -> T {

//         self.unit_locations.push(IndexedField2DLocation::new(x, y));

//         self.entity_prototype.create_from_prototype(self.unit_locations.len())
//     }
// }


#[derive(Getters, Debug, Clone)]
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

impl<T: Clone + StorageLocationProvider + Debug> MobileEntityField2D<T> {





    pub fn new(x: usize, y: usize, entity_prototype: T) -> Self {

        MobileEntityField2D {
            field: IndexedField2D::new(x,y),
            unit_locations: Vec::new(),
            entity_prototype
        }
    }

    pub fn width(&self) -> u32 {
        return self.field.width();
    }


    pub fn height(&self) -> u32 {
        return self.field.height();
    }

    pub fn spawn_entity(&mut self, x: u32, y: u32) -> T {

        self.unit_locations.push(IndexedField2DLocation::new(x, y));
        let result = self.entity_prototype.create_from_prototype(self.unit_locations.len()-1);
        self.field.set(x, y, Some(result.clone()));
        return result;
    }

    pub fn move_entity_u(&mut self, entity: &T, to_x: u32, to_y: u32) {
        let storage_id = entity.get_storage_id();
        let current_pos = &self.unit_locations[storage_id];

        self.move_entity(current_pos.x(), current_pos.y(), to_x, to_y);
    }

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

    pub fn get(&self, x: u32, y: u32) -> &Option<T> {
        &self.field.get_u32(x, y)
    }

    // get's the location of an entity.
    // panics, if entity is not on this field!!
    pub fn get_entity_location(&self, entity: &T) -> &IndexedField2DLocation{
        let storage_location = entity.get_storage_id();
        return &self.unit_locations[storage_location];
    }

    pub fn set_entity(&mut self, entity: &T, x: u32, y: u32) {
        
        let storage_id = entity.get_storage_id();

        // println!("storage_id {} ", storage_id);
        
        // on first call of set entity, we enlarge the positions array.
        if storage_id == 0 || storage_id >= self.unit_locations.len() {
            let start = self.unit_locations.len();
            for i in start..storage_id + 1 {

                if i == storage_id {
                    //println!("storing on unit_location {} len {}   x {x} y {y}", i, self.unit_locations.len());
                    self.unit_locations.push(IndexedField2DLocation::new(x, y));
                }  else {
                    //println!("storing on unit_location as placeholder {} len {}", i, self.unit_locations.len());
                    self.unit_locations.push(IndexedField2DLocation::new(0, 0));
                }
            }
            
            //self.unit_locations.reserve(additional)
        } else {
            self.unit_locations[storage_id] = IndexedField2DLocation::new(x, y);
        }

        self.field.set(x, y, Some(entity.clone()));
    }
  
}


