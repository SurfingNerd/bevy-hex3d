use crate::field_2_d::Field2D;

//type MipMapRequirements<T> = T: Default + Clone + std::ops::Add<Output = T> + std::ops::Div<i32, Output = T>>

pub struct MipMapField2D<T: Default + Clone + std::ops::Add<Output = T>> {
    field: Field2D<T>,
    mip_map_field: Field2D<T>,
    div_function: fn(T, i32) -> T,
}

impl<T: Default + Clone + std::ops::Add<Output = T>> MipMapField2D<T> {
    pub fn new(width: usize, height: usize, div_function: fn(T, i32) -> T) -> Self {
        Self {
            field: Field2D::new(width, height),
            mip_map_field: Field2D::new(width / 3, height / 3),
            div_function
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.field.set(x, y, value);
    }

    pub fn get_u32(&self, x: u32, y: u32) -> &T {
        self.field.get_u32(x, y)
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        self.field.get(x, y)
    }

    pub fn finalize_mip_map(&mut self) {

        // calling it more than once costs just performance.
        // might be handy for updating after changes, like erosion.
        // good task for GPU offloading as well.
        for x in 0..self.mip_map_field.width().clone() {
            for y in 0..self.mip_map_field.height().clone() {
                self.mip_map_field.set(x, y, self.calc_mip_mapped_value(x, y));
            }
        }

    }

    pub fn calc_mip_mapped_value(&self, x: usize, y: usize) -> T {
        let mut sum = T::default();
        let mut count = 0;

        for i in 0..3 {
            for j in 0..3 {
                let x = x * 3 + i;
                let y = y * 3 + j;

                if x < *self.field.width() && y < *self.field.height() {
                    sum = sum + self.field.get(x, y).clone();
                    count += 1;
                }
            }
        }
        // let divisor = count as usize;
        //sum / 9
        return (self.div_function)(sum, count);

        // let t : T = div_result.into();
    }

    pub fn get_mip_map(&self) -> &Field2D<T> {
        &self.mip_map_field
    }
}
