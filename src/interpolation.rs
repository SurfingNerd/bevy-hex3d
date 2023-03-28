use hex2d::Spacing;
use sn_rust::field_2_d::Field2D;




pub fn interpolation_hex2d(x: usize, y: usize, lod: usize, field: &Field2D<i64>, large_field: &Field2D<i64>) -> i64 {


    

    let mut sum: i64 = 0;
    let mut count: i64 = 1;

    sum += field.get(x, y).clone();

    let coordinate_original = hex2d::Coordinate::new(x as i32, y as i32);

    let original_spacing = Spacing::FlatTop(0.50);



    // original pixel  pos.
    let pixel = coordinate_original.to_pixel(original_spacing);



    let ring = coordinate_original.ring_iter(1, hex2d::Spin::CCW(hex2d::Direction::XY));

    for neighbor in ring {
        if neighbor.x >= 0 && neighbor.y >= 0 {
            let x = neighbor.x as usize;
            let y = neighbor.y as usize;
            if x < field.width().clone() && y < field.height().clone() {
                sum = sum + field.get(x, y).clone();
                count += 1;
            }
        }
    }

    // let divisor = count as usize;
    //sum / 9
    return sum / count;

}
