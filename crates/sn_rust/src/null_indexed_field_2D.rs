use std::{collections::BTreeSet, ptr::null};
use crate::indexed_field2d_location::IndexedField2DLocation;



/// An 2D Field with Objects of Type T or a Null Object
/// This is a storage optimized version of IndexedField2D
/// for types that support the Null-Object Design Pattern.
/// Instead of Options, Null Objects are used.
pub struct NullIndexedField2D<T: Clone + PartialEq> {

    data: Vec<T>,
    width: usize,
    height: usize,
    indeces: BTreeSet<IndexedField2DLocation>,
    null_object: T
}

impl<T: Clone + PartialEq> NullIndexedField2D<T> {
    pub fn new(width: usize, height: usize, null_object: &T) -> Self {
        Self {
        data: vec![null_object.clone(); width * height],
        width,
        height,
        indeces: BTreeSet::new(),
        null_object: null_object.clone()
        }
    }

    #[inline(always)]
    fn raw_get (&self, x: usize, y: usize) -> &T {
        &self.data[x * self.height + y]
    }

    #[inline(always)]
    fn raw_mut (&mut self, x: usize, y: usize) -> &mut T {
        &mut self.data[x * self.height + y]
    }

    /// removes the value at the given position, and returns it's value.
    /// might be the null_object.
    pub fn take(&mut self, x: u32, y: u32) -> T {
        let search_location = IndexedField2DLocation::new(x, y);
        
        if self.indeces.remove(&search_location) {
            let null_object =  self.null_object.clone();
            return std::mem::replace(self.raw_mut(x as usize, y as usize), null_object);
        }

        return self.null_object.clone();
    }

    pub fn move_entity(&mut self, x: u32, y: u32, to_x: u32, to_y: u32) {
        let search_location = IndexedField2DLocation::new(x, y);

        #[cfg(debug_assertions)]
        if !self.indeces.contains(&search_location) {
            panic!(
                "trying to move entity from x: {x} y: {y}, but there is no entity at this location"
            );
        }

        let to_search_location = IndexedField2DLocation::new(to_x, to_y);

        #[cfg(debug_assertions)]
        if self.indeces.contains(&to_search_location) {
            panic!("trying to move entity to x: {to_x} y: {to_y}, but there is already an entity at this location");
        }


        *self.raw_mut(to_x as usize, to_y as usize) = self.take(x, y);

        // indeces need to be removed and inserted, because within a BTreeSet,
        // we cannot just change an existing index.

        self.indeces.remove(&search_location);
        self.indeces.insert(to_search_location);

        
    }

    /// sets empty cell on x y to the given value.
    pub fn set(&mut self, x: u32, y: u32, value: T) {

        // we are not checking if we overwrite existing values in production builds.
        #[cfg(debug_assertions)]
        if self.get_u32(x, y).is_some() {
            panic!("trying to overwrite existing value at x: {x} y: {y}");
        }

        self.indeces.insert(IndexedField2DLocation::new(x, y));
        *self.raw_mut(x as usize, y as usize) = value.clone();
    }

    pub fn get_u32(&self, x: u32, y: u32) -> Option<&T> {
        return self.get(x as usize, y as usize);
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        let res = &self.data[x * self.height + y];
        if res.eq(&self.null_object) {
            return None;
        }
        return Some(res);
    }

    pub fn indeces(&self) -> &BTreeSet<IndexedField2DLocation> {
        &self.indeces
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        let res = self.data.get_mut(x * self.height + y);
        return res.expect("Index Out of Boounds");
    }


}




#[test]
fn test_move_entity() {

    let mut field = NullIndexedField2D::<u64>::new(100, 100, &u64::MAX);

    field.set(50, 50, 50);
    assert_eq!(field.get(50, 50).expect("50-50 is set").clone(), 50 as u64);

    field.move_entity(50, 50, 60, 60);

    assert_eq!(field.get(50, 50), None);
    assert_eq!(field.get(60, 60).expect("60-60 is set").clone(), 50 as u64);
}
