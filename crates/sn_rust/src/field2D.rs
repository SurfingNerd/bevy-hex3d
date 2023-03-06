


/** A 2 diamensional field of type T
/  More convinient that a Vec<Vec<T>>
*/
// #[derive(Getters)]
pub struct Field2D<T: Default + Clone>{
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

  pub fn set(&mut self, x: usize, y: usize, value: T) {
      self.data[x * self.height + y] = value;
  }

  pub fn get_i32(&self, x: i32, y: i32) -> &T {
      &self.data[x as usize * self.height + y as usize]
  }

  pub fn get(&self, x: usize, y: usize) -> &T {
      &self.data[x * self.height + y]
  }

  pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
    let res = self.data.get_mut(x * self.height + y);
    return res.expect("Index Out of Boounds");
  }

}