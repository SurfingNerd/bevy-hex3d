use std::collections::BTreeSet;

use crate::{field2D::Field2D, indexed_field2d_location::IndexedField2DLocation};

// type Indeces = BTreeSet<IndexedField2DLocation>;

// impl Ord for IndexedField2DLocation {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {

//         if self.x < other.x {
//             return std::cmp::Ordering::Less;
//         }

//         if self.y < other.y {
//             return std::cmp::Ordering::Less;
//         }

//         if (self.x == other.x && self.y == other.y) {
//             return std::cmp::Ordering::Equal;
//         }

//         return std::cmp::Ordering::Greater;
//     }
// }

// supports indexed access for the underlying field 2D.
// supports only optional, and allows to get the index information of available Some(TValue).
// makes only sense, if only a portion of the existing field is used, and most of the field is Empty. 
// for dense fields, like fields that have Some(TValue) in > 50% ? (to be tested) of the cases, 
// using a Field2D offers better performance
// fast in looking up what fields are used.
// fast in looking up what is stored on a specific field.
// No chunking support.
// slow in finding out where a specific TValue is placed on the field.
pub struct IndexedField2D<TValue: Clone> {
    field: Field2D<Option<TValue>>,

    // maybe a UniqueBTreeSet offers more performance ?
    // just a theory, needs to get confimed.
    // maybe doing when everything iis finished and gater performance tests.
    // https://docs.rs/collected/latest/collected/struct.UniqueBTreeSet.html

    // indeces of filled fields.
    indeces: BTreeSet<IndexedField2DLocation>,
}

// impl Indeces{

//     pub fn insert(&self, x: u32, y: u32) {
//         self.insert(IndexedField2DLocation::new(x, y))
//     }
// }

impl<TValue: Clone> IndexedField2D<TValue> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            field: Field2D::new(width, height),
            indeces: BTreeSet::new(),
        }
    }

    pub fn take(&mut self, x: usize, y: usize) -> Option<TValue> {
        self.field.get_mut(x, y).take()
    }

    pub fn set(&mut self, x: usize, y: usize, value: Option<TValue>) {
        // todo: only do this check in Debug or testing builds

        if self.field.get(x, y).is_some() {
            panic!("trying to overwrite existing value at x: {x} y: {y}");
        }

        if value.is_some() {
            self.indeces
                .insert(IndexedField2DLocation::new(x as u32, y as u32));
        }

        // if there is a value: panic.

        self.field.set(x, y, value);
    }

    pub fn get_u32(&self, x: u32, y: u32) -> &Option<TValue> {
        self.field.get_u32(x, y)
    }

    pub fn get(&self, x: usize, y: usize) -> &Option<TValue> {
        self.field.get(x, y)
    }

    pub fn indeces(&self) -> &BTreeSet<IndexedField2DLocation> {
        &self.indeces
    } 
}
