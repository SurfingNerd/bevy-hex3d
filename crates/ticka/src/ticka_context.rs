use std::sync::mpsc::Sender;
use derive_getters::Getters;
use sn_rust::{mobile_entity_field_2_d::MobileEntityField2D};

use crate::{unit_move_action::UnitMoveInstance, unit::Unit};


#[derive(Getters)]
pub struct TickaContext<'a> {

    unit_move_sender: Sender<UnitMoveInstance>,

    unit_locations: &'a MobileEntityField2D<Unit>
}


impl<'a> TickaContext<'a> {
    pub fn new(unit_locations: &'a mut MobileEntityField2D<Unit>, unit_move_sender: Sender<UnitMoveInstance>) -> Self {

        //let unit_locations: MobileEntityField2D<Unit> = MobileEntityField2D::new(width, height)
        TickaContext { unit_move_sender, unit_locations }
    }
}