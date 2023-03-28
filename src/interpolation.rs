use sn_rust::field_2_d::Field2D;




pub fn create_hex2d_interpolation(x: usize, y: usize, field: &Field2D<i64>) -> i64 {

    let mut sum: i64 = 0;
    let mut count: i64 = 1;

    sum += field.get(x, y).clone();

    let coordinate = hex2d::Coordinate::new(x as i32, y as i32);

    let ring = coordinate.ring_iter(1, hex2d::Spin::CCW(hex2d::Direction::XY));

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
