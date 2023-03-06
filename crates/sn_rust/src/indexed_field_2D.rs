use std::collections::BTreeSet;

use crate::field2D::Field2D;

#[derive(Ord, Eq, PartialOrd, PartialEq)]
pub struct IndexedField2DLocation {
    x: u32,
    y: u32,
}

impl IndexedField2DLocation {
    pub fn new(x: u32, y: u32) -> Self {
        IndexedField2DLocation { x, y }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}

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
// supports only optional, and allows to get the index information of available Some(TValue)
pub struct IndexedField2D<TValue: Clone> {
    field: Field2D<Option<TValue>>,

    // maybe a UniqueBTreeSet offers more performance ?
    // https://docs.rs/collected/latest/collected/struct.UniqueBTreeSet.html
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

    pub fn get_i32(&self, x: i32, y: i32) -> &Option<TValue> {
        self.field.get_i32(x, y)
    }

    pub fn get(&self, x: usize, y: usize) -> &Option<TValue> {
        self.field.get(x, y)
    }
}
