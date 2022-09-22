use bevy::prelude::{Entity, info};

pub struct Game {
    pub current_tick: u32,
    pub width: i32,
    pub height: i32,
    entities: Vec<Vec<Option<Entity>>>, //maybe refactor to sparse set - lets see if memory becomes a problem or not.
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        let mut entities: Vec<Vec<Option<Entity>>> = Vec::new();

        for _x in 0..width {
            let mut y_vec: Vec<Option<Entity>> = Vec::new();
            for _y in 0..height {
                y_vec.push(Option::None);
            }
            entities.push(y_vec);
        }

        Game {
            width,
            height,
            entities,
            current_tick: 0
        }

        //return Game {width, height, };
    }

    pub fn tick(&mut self) {
      self.current_tick += 1;
    }

    pub fn get_entity(&self, x: i32, y: i32) -> Option<Entity> {
        self.entities[x as usize][y as usize]
    }

    /// sets entity to new id.
    /// if position already used, returns Error with existing entity id
    pub fn set_entity(&mut self, x: i32, y: i32, entity: Entity) {
        let existing = self.entities[x as usize][y as usize];
        match existing {
            Some(existing) => panic!("Entity already set!") ,
            None => {
                // info!("setting entity {} {}: {:?}",x, y, entity);
                self.entities[x as usize][y as usize] = Some(entity);
            }
        }
    }

    /// sets entity to new id.
    /// if position already used, returns Error with existing entity id
    pub fn delete_entity(&mut self, x: i32, y: i32) -> Entity {
        let result = self.entities[x as usize][y as usize].expect("no entity to delete.");
        self.entities[x as usize][y as usize] = None;
        return result;
    }
}
