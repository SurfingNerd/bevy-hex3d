use std::sync::mpsc::Sender;
use derive_getters::Getters;
use sn_rust::indexed_field_2_d::IndexedField2D;

use crate::{unit_move_action::UnitMoveInstance, unit::Unit};


#[derive(Getters)]
pub struct TickaContext<'a> {

    unit_move_sender: Sender<UnitMoveInstance>,

    unit_locations: &'a IndexedField2D<Unit>
}


impl<'a> TickaContext<'a> {
    pub fn new(unit_locations: &'a mut IndexedField2D<Unit>, unit_move_sender: Sender<UnitMoveInstance>) -> Self {

        //let unit_locations: IndexedField2D<Unit> = IndexedField2D::new(width, height)
        TickaContext { unit_move_sender, unit_locations }
    }
}