use std::sync::mpsc::Sender;
use derive_getters::Getters;
use hex2d::{Coordinate, Ring, Spacing};
use sn_rust::{mobile_entity_field_2_d::MobileEntityField2D, indexed_field2d_location::IndexedField2DLocation, field_2_d::Field2D};

use crate::{unit_move_action::UnitMoveInstance, unit::Unit};

pub trait ITickaContext {

}
//#[derive(Getters)]
pub struct TickaContext<'a> {

    spacing: Spacing<f32>,
    unit_move_sender: Option<Sender<UnitMoveInstance>>,
    unit_locations: &'a mut MobileEntityField2D<Unit>,
    fields: &'a mut Vec<Field2D<f32>>,
    unit_locations_new: Option<MobileEntityField2D<Unit>>
}

#[derive(Getters)]
pub struct TickaReadContext<'a> {
    spacing: Spacing<f32>,
    unit_locations: &'a MobileEntityField2D<Unit>,
    fields: &'a Vec<Field2D<f32>>,
}

impl<'a> TickaReadContext<'a> {

    pub fn new(unit_locations: &'a MobileEntityField2D<Unit>, fields: &'a Vec<Field2D<f32>>, spacing: Spacing<f32>) -> Self {
        TickaReadContext { unit_locations, fields, spacing }
    }
    
    pub fn get_entity_location(&self, unit: &Unit) -> &IndexedField2DLocation {
        self.unit_locations.get_entity_location(unit)
    }

    pub(crate) fn unit_ring_iter(&self, x: i32, y: i32, r: i32) -> Ring<i32> {
        let coord = Coordinate::new(x, y); // .ring_iter()
        coord.ring_iter(r, hex2d::Spin::CCW(hex2d::Direction::XY))
        //self.unit_locations.
    }
}


impl<'a> TickaContext<'a> {
    pub fn new(unit_locations: &'a mut MobileEntityField2D<Unit>, unit_locations_new: Option<MobileEntityField2D<Unit>>, unit_move_sender: Option<Sender<UnitMoveInstance>>,  fields: &'a mut Vec<Field2D<f32>>, spacing: Spacing<f32>) -> Self {

        //let unit_locations: MobileEntityField2D<Unit> = MobileEntityField2D::new(width, height)
        TickaContext { unit_move_sender, unit_locations, unit_locations_new, fields, spacing }
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

    pub fn get_entity_location(&self, unit: &Unit) -> &IndexedField2DLocation {
        self.unit_locations.get_entity_location(unit)
    }

    pub fn fields(&self)-> &Vec<Field2D<f32>> {
        &self.fields
    }


    pub fn fields_mut(&mut self)-> &mut Vec<Field2D<f32>> {
        &mut self.fields
    }

    pub(crate) fn unit_ring_iter(&self, x: i32, y: i32, r: i32) -> Ring<i32> {
        let coord = Coordinate::new(x, y); // .ring_iter()
        coord.ring_iter(r, hex2d::Spin::CCW(hex2d::Direction::XY))
        //self.unit_locations.
    }

    pub(crate) fn spacing(&self) -> hex2d::Spacing<f32> {
        self.spacing.clone()
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