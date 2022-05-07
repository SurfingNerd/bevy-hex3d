use bevy::{prelude::Mesh, render::mesh::{Indices, PrimitiveTopology}};
use std::f32::consts::PI;

/// An axis-aligned Hexagon3D defined by its minimum and maximum point.
#[derive(Debug, Copy, Clone)]
pub struct Hexagon3D {
    pub diameter: f32,
    pub height: f32,
}

impl Hexagon3D {
    /// Creates a new Hexagon3D centered at the origin with the supplied side lengths.
    pub fn new(diameter: f32, height: f32) -> Hexagon3D {
        Hexagon3D {
            diameter,
            height
        }
    }
}

impl Default for Hexagon3D {
    fn default() -> Self {
        Hexagon3D::new(1.0, 1.0)
    }
}

impl From<Hexagon3D> for Mesh {
    fn from(sp: Hexagon3D) -> Self {
      let center = ([0., 0., 0.], [0., 0., 1.], [0., 0.]);

      let radius = sp.diameter / 2.0;
      let x = |root: f32| (root * 2. * PI / 6.).cos() * radius;
      let y = |root: f32| (root * 2. * PI / 6.).sin() * radius;
      
  
      let spike0 = ([radius, 0., 0.], [0., 0., 1.], [0., 0.]);
      let spike1 = ([x(1.), y(1.), 0.], [0., 0., 1.], [0., 0.]);
      let spike2 = ([x(2.), y(2.), 0.], [0., 0., 1.], [0., 0.]);
      let spike3 = ([x(3.), y(3.), 0.], [0., 0., 1.], [0., 0.]);
      let spike4 = ([x(4.), y(4.), 0.], [0., 0., 1.], [0., 0.]);
      let spike5 = ([x(5.), y(5.), 0.], [0., 0., 1.], [0., 0.]);
      let vertices = [center, spike0, spike1, spike2, spike3, spike4, spike5];
      let mut positions = Vec::with_capacity(6);
      let mut normals = Vec::with_capacity(6);
      let mut uvs = Vec::with_capacity(6);
  
      for (position, normal, uv) in vertices.iter() {
          positions.push(*position);
          normals.push(*normal);
          uvs.push(*uv);
      }
  
      let indices = Indices::U32(vec![0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 1]);
  
      let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
      mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
      mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
      mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
      mesh.set_indices(Some(indices));
      mesh
    }
}