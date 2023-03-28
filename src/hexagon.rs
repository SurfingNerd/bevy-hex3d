use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
    },
};
use sn_rust::indexed_field_2_d::IndexedField2D;
use std::{f32::consts::PI};

/// An axis-aligned Hexagon3D defined by its minimum and maximum point.
#[derive(Debug, Copy, Clone)]
pub struct Hexagon3D {
    pub diameter: f32,
    pub height: f32,

    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Hexagon3DTexturing {
    

    pub get_uv_spike: fn(hex: &Hexagon3D, spike_root: f32) -> [f32; 2],
    pub get_uv_connector: fn(
        hex: &Hexagon3D,
        neighbor: &Hexagon3D,
        pos: &[[f32;3];4],
        spike1: f32,
        spike2: f32,
        n_spike1: f32,
        n_spike2: f32,
    ) -> [[f32; 2]; 4],
}

fn get_uv_spike_height_based(hex: &Hexagon3D, spike_root: f32) -> [f32; 2] {
    return [f32::clamp(hex.y * 0.01, 0.0, 1.0) , if spike_root < 0.0 { 0.5 } else { 0.0 }];
}

fn get_uv_connector_height_based(
    _hex: &Hexagon3D,
    _neighbor: &Hexagon3D,
    pos: &[[f32;3];4],
    _spike1: f32,
    _spike2: f32,
    _n_spike1: f32,
    _n_spike2: f32,
) ->  [[f32; 2]; 4] {


    // make sure y is within the specified range
    let uvx = |y: f32 | f32::clamp(y * 0.01, 0.0, 1.0);



    //let y = pos[0][1];
    //info!("y: {}", pos[1]);
    return [[uvx(pos[0][1]), 0.0],[uvx(pos[1][1]), 1.0],[uvx(pos[2][1]), 1.0],[uvx(pos[3][1]), 0.0]];
}

impl Hexagon3DTexturing {
    pub fn new_height_based_texturing() -> Self {
        Hexagon3DTexturing {
            get_uv_spike: get_uv_spike_height_based,
            get_uv_connector: get_uv_connector_height_based,
        }
    }
}

impl Hexagon3D {
    /// Creates a new Hexagon3D centered at the origin with the supplied side lengths.
    pub fn new(diameter: f32, height: f32) -> Hexagon3D {
        Hexagon3D {
            diameter,
            height,
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn create_mesh_for_hexes(
        hexes: &IndexedField2D<Hexagon3D>,
        texturing: &Hexagon3DTexturing,
    ) -> Mesh {
        // while hexes
        //Some(hex) = hexes.next();
        // let hexes = vec![Hexagon3D::new(1., 1.)];

        // positions: &mut Vec<[f32;3]>, normals: &mut Vec<[f32;3]>, uvs: &mut Vec<[f32;2]>
        //let iter = hexes.iter();
        let count_of_hexes = hexes.indeces().len();
        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(6 * count_of_hexes);
        
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(6 * count_of_hexes);

        let mut indices = Indices::U32(vec![]);


        for loc in hexes.indeces().into_iter() {
            let hex_x = hexes.get_loc(loc); 
            
            for hex in hex_x.into_iter() {
                hex.get_mesh_artifacts(&texturing, &mut positions, &mut uvs, &mut indices);
            }
        }

        // build mesh that connects every hex to it's neighbours.

        // iterate over all indexes of the 2D hex array, and connect them to their neighbours, if it is not already done
        for x in 0..hexes.width().clone()  {
            for y in 0..hexes.height().clone() {
                let hex = if let Some(hex_s) = hexes.get_u32(x, y) {
                    hex_s
                } else {
                    continue;
                };



                //let index = x * hexes[x].len() + y;

                // we connect to the rigth, top, and top reight neighbour
                // we do not have to connecto to left, bottomleft, and bottom neighbours, because they will connect to us

                let c = hex2d::Coordinate::new(x as i32, y as i32);
                let neighbours = c.neighbors();

                // right top spikes: 5-0
                // right bottom spikes: 0-1
                // bottom spikes: 1-2
                // left bottom spikes: 2-3
                // left top spikes: 3-4
                // top spikes: 4-5

                // top -> bottom connection
                hex.connect_to_neighbour(
                    &texturing,
                    neighbours[0],
                    4.,
                    5.,
                    1.,
                    2.,
                    &hexes,
                    &mut positions,
                    &mut uvs,
                    &mut indices,
                );
                // top right  -> bottom left connection.
                hex.connect_to_neighbour(
                    &texturing,
                    neighbours[1],
                    5.,
                    0.,
                    2.,
                    3.,
                    &hexes,
                    &mut positions,
                    &mut uvs,
                    &mut indices,
                );

                // right bottom => top left connection
                hex.connect_to_neighbour(
                    &texturing,
                    neighbours[2],
                    0.,
                    1.,
                    3.,
                    4.,
                    &hexes,
                    &mut positions,
                    &mut uvs,
                    &mut indices,
                );
                // let neighbour1 = c + hex2d::Direction::XY;
            }
        }

        // sp.get_mesh_artifacts(0., 0., 0., &mut positions, &mut normals, &mut uvs, &mut indices);
        //   info!("pos: {:?}", positions);
        //   info!("normals: {:?}", normals);
        //   info!("uvs: {:?}", uvs);
        //   info!("indeces: {:?}", indices);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(indices));

        mesh.duplicate_vertices();
        mesh.compute_flat_normals();
        return mesh;
    }

    pub fn get_spike_x(&self, spike_num: f32) -> f32 {
        return (spike_num * 2. * PI / 6.).cos() * (self.diameter * 0.5) + self.x;
    }

    pub fn get_spike_z(&self, spike_num: f32) -> f32 {
        return (spike_num * 2. * PI / 6.).sin() * (self.diameter * 0.5) + self.z;
    }

    // pub fn get_spike_x_uv(&self, spike_num: f32) -> f32 {
    //     return (spike_num * 2. * PI / 6.).cos() * self.diameter * 0.5 + 0.5;
    // }

    // pub fn get_spike_z_uv(&self, spike_num: f32) -> f32 {
    //     return (spike_num * 2. * PI / 6.).sin() * self.diameter * 0.5 + 0.5;
    // }

    //
    pub fn get_spike(&self, spike_num: f32) -> [f32; 3] {
        // does the center (spike_num 0) has UV 0.5, 0.5 ?

        return 
            [
                self.get_spike_x(spike_num),
                self.y,
                self.get_spike_z(spike_num),
            ];
    }

    /// adds mesh artifacts to the provided arrays.
    pub fn get_mesh_artifacts(
        &self,
        texturing: &Hexagon3DTexturing,
        positions: &mut Vec<[f32; 3]>,
        uvs: &mut Vec<[f32; 2]>,
        indeces: &mut Indices,
    ) {

        let uv = |r| (texturing.get_uv_spike)(&self, r);

        let center = ([self.x, self.y, self.z], uv(-1.));
        let spike0 = (self.get_spike(0.), uv(0.));
        let spike1 = (self.get_spike(1.), uv(1.));
        let spike2 = (self.get_spike(2.), uv(2.));
        let spike3 = (self.get_spike(3.), uv(3.));
        let spike4 = (self.get_spike(4.), uv(4.));
        let spike5 = (self.get_spike(5.), uv(5.));
        let vertices = [center, spike0, spike1, spike2, spike3, spike4, spike5];

        let mut ic: u32 = 0; // indeces count.

        if let Indices::U32(vec) = indeces {
            ic = positions.len() as u32;
            let mut add = |i| vec.push(ic + i);

            add(2);
            add(1);
            add(0);

            add(3);
            add(2);
            add(0);

            add(4);
            add(3);
            add(0);

            add(5);
            add(4);
            add(0);

            add(6);
            add(5);
            add(0);

            add(1);
            add(6);
            add(0);

            for (position, uv) in vertices.iter() {
                positions.push(*position);
                // info!("uv: {:?} pos: {:?}", uv, position);
                uvs.push(*uv);
            }

            return;
            //return Ok(());
        } else {
            return;
        }
        //mesh
    }

    /// adds mesh artifacts to the provided arrays.
    /// connects to the neighbour at the given coordinate, if it exists.
    /// if the neighbour does not exist, it will not connect to it.
    fn connect_to_neighbour(
        &self,
        texturing: &Hexagon3DTexturing,
        neighbour: hex2d::Coordinate,
        spike_1: f32,
        spike_2: f32,
        n_spike_1: f32,
        n_spike_2: f32,
        hexes: &IndexedField2D<Hexagon3D>,
        positions: &mut Vec<[f32; 3]>,
        uvs: &mut Vec<[f32; 2]>,
        indices: &mut Indices,
    ) {
        //check if neighbour is in bounds
        if neighbour.x < 0
            || neighbour.y < 0
            || neighbour.x >= hexes.width() as i32
            || neighbour.y >= hexes.height() as i32
        {
            return;
        }

        let n_hex_o = hexes.get(neighbour.x as usize,neighbour.y as usize);;

        let n_hex = match n_hex_o {
            Some(hex) => hex,
            None => {
                // info!("No neighbour found on {} {}", neighbour.x,neighbour.y);
                return;
            } ,
        };
        // figure out if what edges we have to connect.
        // we have to connect the edges that are between our hexagon and the neighbour hexagon.

        // spike 0 is the center of the hexagon, so we do not have to connect it.

        // spike 1 has to be connected with spike 4
        // spike 2 has to be connected with spike 5

        let spike1 = self.get_spike(spike_1);
        let spike2 = self.get_spike(spike_2);

        let spike_neighbour1 = n_hex.get_spike(n_spike_1);
        let spike_neighbour2 = n_hex.get_spike(n_spike_2);

        //let vertices = [spike1, spike2, spike_neighbour4, spike_neighbour5];

        //positions.push(spike1.0);

        // don't create connectors if the neighbour is already close by

        let spike1_vec = Vec3::from_array(spike1);
        let spike_neighbour2_vec = Vec3::from_array(spike_neighbour2);

        // because of the hex geometry, the distance between the other spike pair is the same.
        // so we can save some computation time by only checking one of the pairs.
        let dist = (spike1_vec - spike_neighbour2_vec).length();
        
        if dist < 0.0001 {
            // info!("not connecting because distance is too small: {} {}-{}-{} {}-{}", dist, self.x, self.y, self.z, neighbour.x, neighbour.y);
            return;
        }

        let pos_origin: u32 = positions.len() as u32;
        positions.push(spike1); // +0
        positions.push(spike2); // +1
        positions.push(spike_neighbour1); // +2
        positions.push(spike_neighbour2); // +3


        let uvs_connector = (texturing.get_uv_connector)(
            &self,
            &n_hex,
            &[spike1, spike2, spike_neighbour1, spike_neighbour2],
            spike_1,
            spike_2,
            n_spike_1,
            n_spike_2,
        );

        // add uvs_connector to uvs.
        uvs.push(uvs_connector[0]);
        uvs.push(uvs_connector[1]);
        uvs.push(uvs_connector[2]);
        uvs.push(uvs_connector[3]);


        // let mut add_uv = | pos : &[f32; 3] | {
            
        // };

        // add_uv(&spike1);
        // add_uv(&spike2);
        // add_uv(&spike_neighbour1);
        // add_uv(&spike_neighbour2);
        

        match indices {
            Indices::U32(vec) => {
                let mut add = |i| vec.push(pos_origin + i);

                //           /\          /\
                //         /    \0   3 /    \
                //        |      |    |      |
                //        |      |    |      |
                //         \    /1   2 \    /
                //           \/          \/

                // add vertice from first spike of self to first spike of neighbour
                add(0);
                add(1);
                add(2);

                add(2);
                add(3);
                add(0);
            }
            _ => {}
        }
    }
}

impl Default for Hexagon3D {
    fn default() -> Self {
        Hexagon3D::new(1.0, 1.0)
    }
}

// impl From<Hexagon3D> for Mesh {
//     fn from(sp: &Hexagon3D) -> Self {

//       let mut positions = Vec::with_capacity(6);
//       let mut uvs = Vec::with_capacity(6);

//       let mut indices = Indices::U32(vec![]);

//       sp.get_mesh_artifacts(&mut positions, &mut uvs, &mut indices);

//       let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
//       mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
//       mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
//       mesh.set_indices(Some(indices));
//       mesh
//     }
// }
