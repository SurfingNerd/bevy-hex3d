use crate::field_2_d::Field2D;

//type MipMapRequirements<T> = T: Default + Clone + std::ops::Add<Output = T> + std::ops::Div<i32, Output = T>>

pub struct MipMapField2D<T: Default + Clone + std::ops::Add<Output = T>> {
    field: Field2D<T>,
    mip_map_fields: Vec<Field2D<T>>,
    interpolation: fn(usize,usize, &Field2D<T>) -> T,
}

impl<T: Default + Clone + std::ops::Add<Output = T>> MipMapField2D<T> {
    pub fn new(width: usize, height: usize, interpolation: fn(usize,usize, &Field2D<T>) -> T) -> Self {
        Self {
            field: Field2D::new(width, height),
            mip_map_fields: Vec::new(),
            interpolation
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
        // for x in 0..self.mip_map_field.width().clone() {
        //     for y in 0..self.mip_map_field.height().clone() {
        //         self.mip_map_field.set(x, y, self.calc_mip_mapped_value(x, y));
        //     }
        // }

        let mut mip_mapped = self.create_mip_map(&self.field);
        let mut current_width = mip_mapped.width().clone(); 
        self.mip_map_fields.push(mip_mapped);

        
        while current_width > 3 {
            let new_mip_map = self.create_mip_map(self.mip_map_fields.last().unwrap());
            current_width = new_mip_map.width().clone();
            println!("created mip map with width: {}", current_width);
            self.mip_map_fields.push(new_mip_map);    
        }
    }

    pub fn create_mip_map(&self, field: &Field2D<T> ) -> Field2D<T> {
        let mut mip_map_field = Field2D::new(field.width() / 3, field.height() / 3);

        for x in 0..mip_map_field.width().clone() {
            for y in 0..mip_map_field.height().clone() {
                let x_pos_read = if x == 0 {1} else {x * 3 - 1};
                let y_pos_read = if y == 0 {1} else {y * 3 - 1};
                mip_map_field.set(x, y, self.calc_mip_mapped_value(field, x_pos_read , y_pos_read ));
            }
        }

        return mip_map_field;

    }

    pub fn calc_mip_mapped_value(&self, field: &Field2D<T>,  x: usize, y: usize) -> T {

        return (self.interpolation)(x,y, field);
        
        // let t : T = div_result.into();
    }

    pub fn get_mip_maps(&self) -> &Vec<Field2D<T>> {
        &self.mip_map_fields
    }

    pub fn field(&self) -> &Field2D<T> {
        &self.field
    }
    
}
