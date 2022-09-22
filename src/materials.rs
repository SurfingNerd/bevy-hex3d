use bevy::prelude::{StandardMaterial, ResMut, Assets, Color, Handle};



pub fn get_color_material(materials: &mut ResMut<Assets<StandardMaterial>>, color: Color) -> Handle<StandardMaterial> {

  // materials.add(StandardMaterial {
  //   base_color_texture: Some(images.add(color_texture(Color::RED))),
  //   ..Default::default()
  // });

  materials.add(
    StandardMaterial {
        base_color: color,
//        metallic: 0.8,
//        reflectance: 0.95,
//        perceptual_roughness: 0.9,
        
        ..Default::default()
    }
  )

}

