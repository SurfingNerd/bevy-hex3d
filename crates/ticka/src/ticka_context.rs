use std::sync::mpsc::Sender;
use derive_getters::Getters;
use sn_rust::{mobile_entity_field_2_d::MobileEntityField2D};

use crate::{unit_move_action::UnitMoveInstance, unit::Unit};


// #[derive(Getters)]
pub struct TickaContext<'a> {

    unit_move_sender: Option<Sender<UnitMoveInstance>>,

    unit_locations: &'a mut MobileEntityField2D<Unit>,
    unit_locations_new: Option<MobileEntityField2D<Unit>>
}


impl<'a> TickaContext<'a> {
    pub fn new(unit_locations: &'a mut MobileEntityField2D<Unit>, unit_locations_new: Option<MobileEntityField2D<Unit>>, unit_move_sender: Option<Sender<UnitMoveInstance>>) -> Self {

        //let unit_locations: MobileEntityField2D<Unit> = MobileEntityField2D::new(width, height)
        TickaContext { unit_move_sender, unit_locations, unit_locations_new }
    }


    pub fn unit_locations(&self) -> &MobileEntityField2D<Unit> {
        //let result: &mut MobileEntityField2D<Unit>  = self.unit_locations;
        //return result;
        self.unit_locations
    }

    pub fn unit_locations_mut(&mut self) -> &mut MobileEntityField2D<Unit> {
        //let result: &mut MobileEntityField2D<Unit>  = self.unit_locations;
        //return result;
        self.unit_locations
    }

    pub fn unit_locations_new_mut(&mut self) -> &mut Option<MobileEntityField2D<Unit>> {
        &mut self.unit_locations_new
    }

}


// #[derive(Getters)]
pub struct TickaContextRead<'a> {

    unit_locations: &'a MobileEntityField2D<Unit>
}


impl<'a> TickaContextRead<'a> {
    pub fn new(unit_locations: &'a MobileEntityField2D<Unit>) -> Self {

        //let unit_locations: MobileEntityField2D<Unit> = MobileEntityField2D::new(width, height)
        TickaContextRead { unit_locations }
    }


    pub fn unit_locations(&self) -> &MobileEntityField2D<Unit> {
        //let result: &mut MobileEntityField2D<Unit>  = self.unit_locations;
        //return result;
        self.unit_locations
    }
}