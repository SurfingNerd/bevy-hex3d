use std::{ops::{Index, Mul}, f32::consts::PI};

use bevy::{prelude::{Component, Commands, ResMut, Assets, Mesh, Vec3, shape::Quad, Vec2, info, PbrBundle, Transform, StandardMaterial, AlphaMode}, render::{mesh::Indices, render_resource::PrimitiveTopology}, core::Zeroable};



// pub struct GlowLinePool {

// }

#[derive(Component)]
pub struct GlowLine {
  
}

impl GlowLine {

  /// creates a glowing line. 
  pub(crate) fn create(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>, start: Vec3, end: Vec3, radius: f32) -> GlowLine {

    // todo: reuse mesh somehow.
    // meshes.get_or_insert_with(handle, insert_fn)
    let mut positions: Vec<[f32;3]> = Vec::with_capacity(6);
    // let mut normals: Vec<[f32;3]> = Vec::with_capacity(6);
    let mut uvs: Vec<[f32;2]> = Vec::with_capacity(6);
    

  
    let main_vector = end - start;
    let direction = main_vector.normalize();
    
    
    //let rotate1 = Quad::new(Vec2::new(1., 0.));
    //let rotate2 = Quad::new(Vec2::new(-1., 0.));
    //let rotate3 = Quad::new(Vec2::new(0., 1.));
    //let rotate4 = Quad::new(Vec2::new(0., -1.));

    // bevy::math::f32::Quat::
    let rotate1 = bevy::math::f32::Quat::from_rotation_x(PI / 2.);
    let rotate2 = bevy::math::f32::Quat::from_rotation_x(-PI / 2.);

    let dir1 = rotate1.mul_vec3(direction);
    let dir2 = rotate2.mul_vec3(direction);
    
    let point_origin = Vec3::zeroed();
    let point_end = direction;
    let point_edge1 = dir1 * (radius * 0.5);
    let point_edge2 = dir2 * (radius * 0.5);

    info!("main vector: {:?} l:{}", main_vector, main_vector.length());
    info!("dir1: {:?} l:{} a {}: ", dir1, dir1.length(), dir1.angle_between(direction));
    info!("dir2: {:?} l:{} a: {}", dir2, dir2.length(), dir2.angle_between(direction));
    // let dir1 = direction * rotate1;

    //let rotate1 = Rotation::from_rotation_x(1.);


    //let dir_right_angle_1 = Vec3::new(direction.y, direction.x, direction.z);
    // let dir_right_angle_2 = Vec3::new(-direction.y, direction.x, direction.z);

    // let rhs = Rhs::new();
    // direction.mul(rhs)

    positions.push(point_origin.to_array());
    positions.push(point_end.to_array());
    positions.push(point_edge1.to_array());
    positions.push(point_edge2.to_array());


    let pos_start = 0;
    let pos_end = 1;
    let pos_dir1 = 2;
    let pos_dir2 = 3;

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let indices: Vec<u32> = 
      vec!(
        pos_start, pos_dir1, pos_dir2,
        pos_start, pos_dir1, pos_end,
        pos_start, pos_dir2, pos_end,
        pos_end, pos_dir1, pos_dir2
      );


    let mut normals: Vec<[f32;3]> = Vec::with_capacity(indices.len() / 3);
    for _i in 0..(indices.len() / 3) {
      normals.push([0., 1., 0.]);
    }

    info!("{:?}", normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));
    // return mesh;

    // UV's required ??!?!
    // mesh.insert_attribute(Mesh::ATT, values)

    mesh.duplicate_vertices();
    // mesh.compute_flat_normals();

    
    let mesh_handle = meshes.add(mesh);

    let mat = StandardMaterial {
      base_color: bevy::prelude::Color::Rgba { red: 1., green: 1., blue: 1., alpha: 0.5 },
      alpha_mode: AlphaMode::Blend,
      ..Default::default()
    };

    let mat_handle = materials.add(mat);

    commands.spawn_bundle(PbrBundle {
      mesh: mesh_handle,
      material: mat_handle.clone(),
      transform: Transform {
          translation: start,
          // rotation: quat.clone(),
          ..Default::default()
      },
      ..Default::default()
    });

    GlowLine {}
  }

    

}
