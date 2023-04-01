use derive_getters::Getters;

use crate::traits::IField2D;




/** A 2 diamensional field of type T
/  More convinient that a Vec<Vec<T>>
*/
#[derive(Getters, Debug)]
pub struct Field2D<T: Default + Clone>  {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Default + Clone> Field2D<T> {

  pub fn new(width: usize, height: usize) -> Self {
    Self {
      data: vec![T::default(); width * height],
      width,
      height,
    }
  }
}

impl<T: Default + Clone> IField2D<T> for Field2D<T> {


  fn set(&mut self, x: usize, y: usize, value: T) {
      self.data[x * self.height + y] = value;
  }

  fn get_u32(&self, x: u32, y: u32) -> &T {
      &self.data[x as usize * self.height + y as usize]
  }

  fn get(&self, x: usize, y: usize) -> &T {
      &self.data[x * self.height + y]
  }

  fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
    let res = self.data.get_mut(x * self.height + y);
    return res.expect("Index Out of Boounds");
  }
}

