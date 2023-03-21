use std::{sync::{mpsc::{Sender, Receiver}, Mutex}, time::Duration};

use bevy::prelude::Component;
use ticka::unit_move_action::UnitMoveInstance;



#[derive(Component)]
pub struct MovementReader {
    sender_prototype: Mutex<Sender<UnitMoveInstance>>,
    receiver: Mutex<Receiver<UnitMoveInstance>>

}

impl MovementReader {


    pub fn new() -> Self {
        
        let channel = std::sync::mpsc::channel::<ticka::unit_move_action::UnitMoveInstance>();

        MovementReader { sender_prototype: Mutex::new(channel.0), receiver: Mutex::new(channel.1) }
    }

    pub fn create_sender(&self) -> Sender<UnitMoveInstance> {
        
        let lock = self.sender_prototype.lock().expect("lock");
        lock.clone()
    }

    pub fn get_movement_uis(&mut self) -> Vec<UnitMoveInstance> {

        let mut result = Vec::<UnitMoveInstance>::new();

        if let Ok(lock) = self.receiver.try_lock() {
            //let lock = self.receiver.lock().expect("");
            while let Ok(move_instance) = lock.recv_timeout(Duration::from_micros(1)) {
                result.push(move_instance);
            }    
        }

        return result;
    }

}