use std::{thread, time::Duration, fs};

use bevy::{prelude::{Plugin, App, Handle, Image, Res, ResMut, AssetServer, Assets, Mesh, StandardMaterial, Color, Commands, PbrBundle, Transform, Vec3, info}, render::texture::{ImageType, CompressedImageFormats}};

use crate::{hexagon::Hexagon3D, resources::Game};

pub struct PlaygroundPlugin {

}


struct HeightmapCache {
  pub heightmap: Handle<Image>
}

impl HeightmapCache {
  pub fn new() -> Self {
      HeightmapCache {
          heightmap: Handle::default()
      }
  }
}

fn get_grayscale(rgba: &Vec<u8>, x: usize, y: usize, width: usize) -> f32 {
  let index = (y * width + x) * 4;
  //let index = (x * height + y) * 4;
  // let r:u16 = rgba[index] as u16;
  // let g:u16 = rgba[index + 1] as u16;
  // let b: u16= rgba[index + 2] as u16;
  // let a = rgba[index + 3];
  
  let r = rgba[index + 0] as u16;
  let g = rgba[index + 1] as u16;
  let b= rgba[index + 2] as u16;
  // let a = rgba[index + 3];
  
  let gray = (r + g + b ) as f32 / 3.0;
  gray
}



// fn load_playground_resources(mut heighmap_cache: ResMut<HeightmapCache>, asset_server: Res<AssetServer>) {

//   heighmap_cache.heightmap = asset_server.load("textures/heightmap.png");
// }


fn setup_playground(
  mut commands: Commands,
  heightmap_cache: Res<HeightmapCache>,
  asset_server: Res<AssetServer>,
  images: Res<Assets<Image>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  game: ResMut<Game>
) {

  // while!images.contains(&heightmap_cache.heightmap) {
  //   info!("waiting for heightmap");
  // }
  
  let mut hm: Image;
  let mut is_loaded = false;

  match fs::read("assets/heighmap_mt_taranaki.png")
  {
    Ok(read_file) => {
      match Image::from_buffer(&read_file, ImageType::Extension("png"), CompressedImageFormats::all(), false) {
        Ok(image) => {
          hm = image;
          is_loaded = true;
          info!("Image loaded: {}", hm.data.len());
        },
        Err(e) => {
          info!("error loading image {:?}", e);
          return;
        }
      }
    },
    Err(_) => {
      info!("file does not exist");
      return;
    }
  }

  info!("235 178 should be white: {}", get_grayscale(&hm.data, 235, 178, hm.size().x as usize));
  info!("240 32 should be gray: {}", get_grayscale(&hm.data, 240, 32, hm.size().x as usize));
  info!("450 334 should be black: {}", get_grayscale(&hm.data, 450, 334, hm.size().x as usize));
  
  // while !is_loaded {
  //   if let Some(heighmap) = images.get(&heightmap_cache.heightmap) {
  //     info!("heighmap: {:?}", heighmap);
  //     hm = heighmap;
  //     is_loaded = true;
  //   }
  //   else {
  //     info!("heighmap not loaded");
  //     thread::sleep(Duration::from_millis(100));
  //   }
  // }


  let mut hexes: Vec<Hexagon3D> = vec![];
  
  // for (i, shape) in shapes.into_iter().enumerate() {
  for x in 0..game.width {
      for y in 0..game.height {
          let c = hex2d::Coordinate::new(x, y);
          let (x_pixel, y_pixel) = c.to_pixel(hex2d::Spacing::FlatTop(0.51));

          let mut z_pixel = 0.;

          if x_pixel < hm.size().x && -y_pixel < hm.size().y {
            let img_x = x_pixel as usize;
            let img_y = -y_pixel as usize;
            let img_height = hm.size().y as usize;
            let img_width = hm.size().x as usize;

            
            let pixel_value = get_grayscale(&hm.data, img_x, img_y,img_width);
            //info!("pixel_value: x {} y {} -> {}",img_x, img_y, pixel_value);
            z_pixel =  100. - (1. - (pixel_value / 255.0)) * 100.;
          }

          // info!("pixel x {} y {} ", x_pixel, y_pixel);
          let hex = Hexagon3D {
              diameter: 1.,
              height: 0.,
              x: x_pixel,
              y: z_pixel,
              z: y_pixel,
          };

          hexes.push(hex);
      }
  }

  let mesh = Hexagon3D::create_mesh_for_hexes(&hexes);
  let mesh_handle = meshes.add(mesh);
  
  // green: 6d9862
  // yellow? // base_color: Color::rgb(123.0 / 255., 130. / 255., 78. / 255.),
  // green: base_color: Color::rgb(0.43, 0.596, 0.384),
  // stonegrey: base_color: Color::rgb(0.647, 0.627, 0.616),
  let mat2 = materials.add(
      StandardMaterial {
          base_color: Color::rgb(0.647, 0.627, 0.616),
          metallic: 0.12,
          reflectance: 0.01,
          perceptual_roughness: 0.9,
          
          ..Default::default()
      }
  );

  commands.spawn_bundle(PbrBundle {
      mesh: mesh_handle,
      material: mat2.clone(),
      transform: Transform {
          translation: Vec3::new(0., 0., 0.),
          // rotation: quat.clone(),
          ..Default::default()
      },
      ..Default::default()
  });
}


impl PlaygroundPlugin {

  pub fn new() -> Self {
    Self {}
  }


  
  fn process_heighmap(
    heighmap: Res<HeightmapCache>,
    images: ResMut<Assets<Image>>
) {
    if let Some(map) =  images.get(&heighmap.heightmap) {
        info!("Heighmap found!" );
    } else {
        info!("Heighmap not found!" );
    }
    
    // if let Some(heighmap) = heighmap.heighmap  {
    //     info!("Heighmap found!", );
    // }
}

}

impl Plugin for PlaygroundPlugin {
  fn build(&self, app: &mut App) {

    app
    .insert_resource(HeightmapCache::new())
    //.add_startup_system(load_playground_resources)
    .add_startup_system(setup_playground)
    ;

  }
}
