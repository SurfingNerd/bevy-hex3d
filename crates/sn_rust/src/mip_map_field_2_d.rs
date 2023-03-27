pub struct MipMapField2D<T: Default + Clone> {
    field: Field2D<T>,
    mip_map_field: Field2D<T>,
}

impl<T: Default + Clone> MipMapField2D<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            field: Field2D::new(width, height),
            mip_map_field: Field2D::new(width / 3, height / 3),
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

    pub fn calc_mip_mapped_value(&self, x: usize, y: usize) -> T {
        let mut sum = T::default();
        let mut count = 0;

        for i in 0..3 {
            for j in 0..3 {
                let x = x * 3 + i;
                let y = y * 3 + j;

                if x < self.field.width && y < self.field.height {
                    sum = sum + self.field.get(x, y).clone();
                    count += 1;
                }
            }
        }

        sum / count
    }
}
