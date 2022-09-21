use bevy::prelude::{Res, Assets, Mesh, StandardMaterial, info};



#[allow(dead_code)]
pub fn debug_resources_system(meshes: Res<Assets<Mesh>>, materials: Res<Assets<StandardMaterial>>) {
  
  info!("meshes: {}, materials: {}", meshes.len(), materials.len());
}