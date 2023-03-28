use std::{collections::BTreeSet, fmt::Debug};

use crate::{field_2_d::Field2D, indexed_field2d_location::IndexedField2DLocation};

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
// supports only optional, and allows to get the index information of available Some(T).
// makes only sense, if only a portion of the existing field is used, and most of the field is Empty. 
// for dense fields, like fields that have Some(T) in > 50% ? (to be tested) of the cases, 
// using a Field2D offers better performance
// fast in looking up what fields are used.
// fast in looking up what is stored on a specific field.
// No chunking support.
// slow in finding out where a specific T is placed on the field.
#[derive(Debug)]
pub struct IndexedField2D<T: Clone + Debug> {
    field: Field2D<Option<T>>,

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

impl<T: Clone + Debug> IndexedField2D<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            field: Field2D::new(width, height),
            indeces: BTreeSet::new(),
        }
    }

    pub fn take(&mut self, x: u32, y: u32) -> Option<T> {

        let search_location = IndexedField2DLocation::new(x, y);
        if !self.indeces.contains(&search_location) {

        }

        if self.indeces.remove(&search_location) {
            return self.field.get_mut(x as usize, y as usize).take();
        }

        return None;

    }

    pub fn move_entity(&mut self, x: u32, y: u32, to_x: u32, to_y: u32) {

        let search_location = IndexedField2DLocation::new(x, y);
        if !self.indeces.contains(&search_location) {
            panic!("trying to move entity from x: {x} y: {y}, but there is no entity at this location");
        }

        let to_search_location = IndexedField2DLocation::new(to_x, to_y);
        if self.indeces.contains(&to_search_location) {
            panic!("trying to move entity to x: {to_x} y: {to_y}, but there is already an entity at this location");
        }

        let value = self.field.get_mut(x as usize, y as usize).take();
        self.field.get_mut(to_x as usize, to_y as usize).replace(value.unwrap());

        // indeces need to be removed and inserted, because within a BTreeSet,
        // we cannot just change an existing index.

        debug_assert!(self.indeces.remove(&search_location));
        debug_assert!(self.indeces.insert(to_search_location));
    }

    pub fn set(&mut self, x: u32, y: u32, value: Option<T>) {
        // todo: only do this check in Debug or testing builds

        if self.field.get_u32(x, y).is_some() {
            panic!("trying to overwrite existing value at x: {x} y: {y}");
        }

        if value.is_some() {
            self.indeces
                .insert(IndexedField2DLocation::new(x, y));
        }

        // if there is a value: panic.

        self.field.set(x as usize, y as usize, value);
    }

    pub fn get_u32(&self, x: u32, y: u32) -> &Option<T> {
        self.field.get_u32(x, y)
    }

    pub fn get(&self, x: usize, y: usize) -> &Option<T> {
        self.field.get(x, y)
    }

    pub fn indeces(&self) -> &BTreeSet<IndexedField2DLocation> {
        &self.indeces
    }

    pub fn width(&self) -> u32 {
        *self.field.width() as u32
    }

    pub fn height(&self) -> u32 {
        *self.field.height() as u32
    }

    pub fn get_loc(&self, target_location: &IndexedField2DLocation) -> &Option<T> {
        self.get_u32(target_location.x(), target_location.y())
    }

    
}


#[test]
fn test_move_entity() {

    let mut field = IndexedField2D::<u64>::new(100, 100);

    field.set(50, 50, Some(50));
    assert_eq!(field.get(50, 50), &Some(50 as u64));

    field.move_entity(50, 50, 60, 60);

    assert_eq!(field.get(50, 50), &None);
    assert_eq!(field.get(60, 60), &Some(50 as u64));
}
