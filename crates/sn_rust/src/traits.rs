

pub trait IField2D<T: Default + Clone> {
    
    fn set(&mut self, x: usize, y: usize, value: T);
    fn get_u32(&self, x: u32, y: u32) -> &T;
    fn get(&self, x: usize, y: usize) -> &T;
    fn get_mut(&mut self, x: usize, y: usize) -> &mut T;
}