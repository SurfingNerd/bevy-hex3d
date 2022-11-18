use bevy::prelude::{Entity};

use crate::lib::field_2D::Field2D;

pub struct Game {
    pub current_tick: u32,
    pub width: i32,
    pub height: i32,
    entities: Field2D<Option<Entity>>, //maybe refactor to sparse set - lets see if memory becomes a problem or not.,
    pub hex_spacing: hex2d::Spacing,
    pub heights_z: Field2D<f32>,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        

        Game {
            width,
            height,
            entities: Field2D::new(width as usize, height as usize),
            current_tick: 0,
            hex_spacing: hex2d::Spacing::FlatTop(0.50),
            heights_z: Field2D::new(width as usize, height as usize),
        }
        
    }

    #[allow(dead_code)]
    pub fn tick(&mut self) {
      self.current_tick += 1;
    }

    pub fn get_entity(&self, x: i32, y: i32) -> Option<Entity> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return Option::None;
        }
        self.entities.get(x as usize,y as usize).clone()
    }

    /// sets entity to new id.
    /// if position already used, returns Error with existing entity id
    pub fn set_entity(&mut self, x: i32, y: i32, entity: Entity) {
        let existing = self.entities.get(x as usize,y as usize);
        match existing {
            Some(_) => panic!("Entity already set!") ,
            None => {
                // info!("setting entity {} {}: {:?}",x, y, entity);
                self.entities.set(x as usize,y as usize, Some(entity));
            }
        }
    }

    pub fn set_height(&mut self, x: i32, y: i32, height: f32) {
        //self.heights_z[x as usize][y as usize] = height;
        self.heights_z.set(x as usize, y as usize, height);
    }

    pub fn get_height(&mut self, x: i32, y: i32) -> f32 {
        self.heights_z.get(x as usize,y as usize).clone()
    }

    pub fn set_height_field(&mut self, field: Field2D<f32>) {
        self.heights_z = field;
    }

    /// sets entity to new id.
    /// if position already used, returns Error with existing entity id
    pub fn delete_entity(&mut self, x: i32, y: i32) -> Entity {
        let result = self.entities.get(x as usize,y as usize).expect("no entity to delete.");
        self.entities.set(x as usize,y as usize,None);
        return result;
    }
}
