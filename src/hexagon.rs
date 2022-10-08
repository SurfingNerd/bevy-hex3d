use bevy::{prelude::{Mesh, info}, render::mesh::{Indices, PrimitiveTopology}};
use std::{f32::consts::PI, fmt::Error, slice::Iter};

/// An axis-aligned Hexagon3D defined by its minimum and maximum point.
#[derive(Debug, Copy, Clone)]
pub struct Hexagon3D {
    pub diameter: f32,
    pub height: f32,

    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Hexagon3D {
    /// Creates a new Hexagon3D centered at the origin with the supplied side lengths.
    pub fn new(diameter: f32, height: f32) -> Hexagon3D {
        Hexagon3D {
            diameter,
            height,
            x: 0.,
            y: 0.,
            z: 0.
        }
    }

    pub fn create_mesh_for_hexes(hexes: &Vec<Hexagon3D>) -> Mesh{

        // while hexes
      //Some(hex) = hexes.next();
      // let hexes = vec![Hexagon3D::new(1., 1.)];

      // positions: &mut Vec<[f32;3]>, normals: &mut Vec<[f32;3]>, uvs: &mut Vec<[f32;2]>
      //let iter = hexes.iter();
      let count_of_hexes = hexes.len();
      let mut positions: Vec<[f32;3]> = Vec::with_capacity(6 * count_of_hexes);
      let mut normals: Vec<[f32;3]> = Vec::with_capacity(6 * count_of_hexes);
      let mut uvs: Vec<[f32;2]> = Vec::with_capacity(6 * count_of_hexes);
      
      let mut indices = Indices::U32(vec![]);

      for hex in hexes.into_iter() { 
        hex.get_mesh_artifacts(&mut positions, &mut normals, &mut uvs, &mut indices);
      }

      // sp.get_mesh_artifacts(0., 0., 0., &mut positions, &mut normals, &mut uvs, &mut indices);
      //   info!("pos: {:?}", positions);
      //   info!("normals: {:?}", normals);
      //   info!("uvs: {:?}", uvs);
      //   info!("indeces: {:?}", indices);

      let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
      mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
      mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
      mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
      mesh.set_indices(Some(indices));
      return mesh;

    }

    pub fn get_spike_x(&self, spike_num: f32) -> f32 {
      return (spike_num * 2. * PI / 6.).cos() * (self.diameter * 0.5) + self.x;
    }

    pub fn get_spike_z(&self, spike_num: f32) -> f32 {
      return (spike_num * 2. * PI / 6.).sin() * (self.diameter * 0.5) + self.z;
    }

    pub fn get_spike_x_uv(&self, spike_num: f32) -> f32 {
      return (spike_num * 2. * PI / 6.).cos() * self.diameter * 0.5 + 0.5;
    }

    pub fn get_spike_z_uv(&self, spike_num: f32) -> f32 {
      return (spike_num * 2. * PI / 6.).sin() * self.diameter * 0.5 + 0.5;
    }

    //
    pub fn get_spike(&self, spike_num: f32) -> ([f32;3],[f32;3],[f32;2]) {
      // does the center (spike_num 0) has UV 0.5, 0.5 ?

      return (
        [self.get_spike_x(spike_num), self.y, self.get_spike_z(spike_num)],
         [0., 1., 0.], 
         [self.get_spike_x_uv(spike_num), self.get_spike_z_uv(spike_num)]);
    }

    /// adds mesh artifacts to the provided arrays.
    pub fn get_mesh_artifacts(&self, positions: &mut Vec<[f32;3]>, normals: &mut Vec<[f32;3]>, uvs: &mut Vec<[f32;2]>, indeces: &mut Indices) {
      
        let center = ([self.x, self.y, self.z], [0., 1., 0.], [0.5, 0.5]);
        let spike0 = self.get_spike(0.);
        let spike1 = self.get_spike(1.);
        let spike2 = self.get_spike(2.);
        let spike3 = self.get_spike(3.);
        let spike4 = self.get_spike(4.);
        let spike5 = self.get_spike(5.);
        let vertices = [center, spike0, spike1, spike2, spike3, spike4, spike5];
      
      let mut ic: u32 = 0; // indeces count.

      if let Indices::U32(vec) = indeces {
        ic = positions.len() as u32;
        let mut add = |i| vec.push( ic + i );

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

        for (position, normal, uv) in vertices.iter() {
            positions.push(*position);
            normals.push(*normal);
            // info!("uv: {:?} pos: {:?}", uv, position);
            uvs.push(*uv);
        }

        return;
        //return Ok(());
      }
      else {
        return;
      }
      //mesh
    }
}

impl Default for Hexagon3D {
    fn default() -> Self {
        Hexagon3D::new(1.0, 1.0)
    }
}

impl From<Hexagon3D> for Mesh {
    fn from(sp: Hexagon3D) -> Self {
      

      let mut positions = Vec::with_capacity(6);
      let mut normals = Vec::with_capacity(6);
      let mut uvs = Vec::with_capacity(6);
      
      let mut indices = Indices::U32(vec![]);

      sp.get_mesh_artifacts(&mut positions, &mut normals, &mut uvs, &mut indices);
  
      let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
      mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
      mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
      mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
      mesh.set_indices(Some(indices));
      mesh
    }
}