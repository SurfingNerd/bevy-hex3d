use std::sync::Arc;

use bevy::prelude::{Entity, Resource};
use sn_rust::{field_2_d::Field2D, mip_map_field_2_d::MipMapField2D};



#[derive(Resource)]
pub struct Game {
    pub current_tick: u32,
    pub width: u32,
    pub height: u32,
    entities: Field2D<Option<Entity>>, //maybe refactor to sparse set - lets see if memory becomes a problem or not.,
    pub hex_spacing: hex2d::Spacing,
    // we divide the unit heigh throught 1000, to get the granularity we want.
    pub heights_z: MipMapField2D<i64>,
}

// mark the function not for inline
#[inline(never)]
fn div_i64_i32(a: i64, b: i32) -> i64 {
    let result = (a / b as i64).clone();
    return result;
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        

        Game {
            width,
            height,
            entities: Field2D::new(width as usize, height as usize),
            current_tick: 0,
            hex_spacing: hex2d::Spacing::FlatTop(0.50),
            heights_z: MipMapField2D::new(width as usize, height as usize, div_i64_i32 ),
        }
        
    }

    #[allow(dead_code)]
    pub fn tick(&mut self) {
      self.current_tick += 1;
    }

    pub fn get_entity(&self, x: u32, y: u32) -> Option<Entity> {
        if x >= self.width || y >= self.height {
            return Option::None;
        }
        self.entities.get(x as usize,y as usize).clone()
    }

    /// sets entity to new id.
    /// if position already used, returns Error with existing entity id
    pub fn set_entity(&mut self, x: u32, y: u32, entity: Entity) {
        let existing = self.entities.get(x as usize,y as usize);
        match existing {
            Some(_) => panic!("Entity already set!") ,
            None => {
                // info!("setting entity {} {}: {:?}",x, y, entity);
                self.entities.set(x as usize,y as usize, Some(entity));
            }
        }
    }

    #[allow(dead_code)]
    pub fn set_height(&mut self, x: u32, y: u32, height: i64) {
        //self.heights_z[x as usize][y as usize] = height;
        self.heights_z.set(x as usize, y as usize, height);
    }

    pub fn get_height(&mut self, x: u32, y: u32) -> i64 {
        self.heights_z.get(x as usize,y as usize).clone()
    }

    pub fn set_height_field(&mut self, field: MipMapField2D<i64>) {

        // self.heights_z = Arc::new(MipMapField2D::new_from_field(field, div_i64_i32));
        self.heights_z = field;
    }

    /// sets entity to new id.
    /// if position already used, returns Error with existing entity id
    pub fn delete_entity(&mut self, x: u32, y: u32) -> Entity {
        let result = self.entities.get(x as usize,y as usize).expect("no entity to delete.");
        self.entities.set(x as usize,y as usize,None);
        return result;
    }
}
