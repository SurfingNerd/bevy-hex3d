use std::future::{self, Future, IntoFuture};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, MutexGuard};
use std::{fs, thread, time::Duration};

use bevy::render::texture::ImageType;
use bevy::tasks::*;
use bevy::utils::futures;
use bevy::{prelude::*, render::texture::CompressedImageFormats};

use crate::{
    hexagon::{Hexagon3D, Hexagon3DTexturing},
    resources::Game,
};

pub struct PlaygroundPlugin {}

type ThreadsafeBox<T> = Arc<Mutex<Option<Box<T>>>>;

#[derive(Component)]
pub struct MeshGenTask {
    pub mesh: Arc<Mutex<Option<Box<Mesh>>>>,
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
    let b = rgba[index + 2] as u16;
    // let a = rgba[index + 3];

    let gray = (r + g + b) as f32 / 3.0;
    gray
}

// fn load_playground_resources(mut heighmap_cache: ResMut<HeightmapCache>, asset_server: Res<AssetServer>) {

//   heighmap_cache.heightmap = asset_server.load("textures/heightmap.png");
// }

fn setup_playground(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game: ResMut<Game>,
) {
    info!("setting up playground");
    // while!images.contains(&heightmap_cache.heightmap) {
    //   info!("waiting for heightmap");
    // }

    let hm: Image;

    //let asset_to_load = "assets/heighmap_mt_taranaki.png";
    //let asset_to_load = "assets/heighmap_schweinskopf.png";
    let asset_to_load = "assets/heightmap_alps.png";

    match fs::read(asset_to_load) {
        Ok(read_file) => {
            match Image::from_buffer(
                &read_file,
                ImageType::Extension("png"),
                CompressedImageFormats::all(),
                false,
            ) {
                Ok(image) => {
                    hm = image;
                    info!("Image loaded: {}", hm.data.len());
                }
                Err(e) => {
                    info!("error loading image {:?}", e);
                    return;
                }
            }
        }
        Err(_) => {
            info!("file does not exist");
            return;
        }
    }

    // 2 dimensional array of hexagons
    let mut hexes_2d: Box<Vec<Vec<Hexagon3D>>> = Box::new(vec![]);

    // for (i, shape) in shapes.into_iter().enumerate() {
    for x in 0..game.width {
        let mut hexes_x: Vec<Hexagon3D> = vec![];

        for y in 0..game.height {
            let c = hex2d::Coordinate::new(x, y);
            let (x_pixel, y_pixel) = c.to_pixel(game.hex_spacing);

            let mut z_pixel = 0.;

            if x_pixel < hm.size().x && -y_pixel < hm.size().y {
                let img_x = x_pixel as usize;
                let img_y = -y_pixel as usize;
                let img_height = hm.size().y as usize;
                let img_width = hm.size().x as usize;

                let pixel_value = get_grayscale(&hm.data, img_x, img_y, img_width);
                //info!("pixel_value: x {} y {} -> {}",img_x, img_y, pixel_value);
                z_pixel = 100. - (1. - (pixel_value / 255.0)) * 100.;
            }

            // info!("pixel x {} y {} ", x_pixel, y_pixel);
            let hex = Hexagon3D {
                diameter: 1.,
                height: 0.,
                x: x_pixel,
                y: z_pixel,
                z: y_pixel,
            };

            hexes_x.push(hex);

            //hexes.push(hex);
        }
        hexes_2d.push(hexes_x);
    }

    let pool = AsyncComputeTaskPool::get();

    let mutex: Arc<Mutex<Option<Box<Mesh>>>> = Arc::new(Mutex::new( None ));

    let mutex2 = mutex.clone();
    //let mesh_gen_task = MeshGenTask { mesh: Arc::new(Mutex::new(None)) };

    info!("start mesh generation in own thread");
    std::thread::spawn( move || {
        info!("start creating mesh");
        let texturing = Hexagon3DTexturing::new_height_based_texturing();
        let mesh = Hexagon3D::create_mesh_for_hexes(&hexes_2d, &texturing);
        info!("mesh created");
        mutex.lock().unwrap().replace(Box::new(mesh));
    });

    let gen_task = MeshGenTask { mesh: mutex2 };
    commands.spawn().insert(gen_task);
}

fn finish_setup_playgroud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mesh_gen_task: Query<(Entity, &MeshGenTask)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    
    for (entity, mesh_gen_task) in mesh_gen_task.iter() {

        let lock_guard = mesh_gen_task.mesh.lock().unwrap();
        // info!("checking if mesh is ready");
        if lock_guard.is_some() {

            let mesh = lock_guard.as_ref().unwrap();
        let texture: Handle<Image> = asset_server.load("mountain_texture_less_sat.png");
        
        let mesh_handle = meshes.add(*mesh.clone());

        // green: 6d9862
        // yellow? // base_color: Color::rgb(123.0 / 255., 130. / 255., 78. / 255.),
        // green: base_color: Color::rgb(0.43, 0.596, 0.384),
        // stonegrey: base_color: Color::rgb(0.647, 0.627, 0.616),
        let mat2 = materials.add(StandardMaterial {
            base_color_texture: Some(texture),
            // base_color: Color::rgb(0.647, 0.627, 0.616),
            metallic: 0.12,
            reflectance: 0.01,
            perceptual_roughness: 0.9,
            ..Default::default()
        });

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

        commands.entity(entity).despawn_recursive();
        }
        //mesh_gen_task
    }

}

impl PlaygroundPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugin for PlaygroundPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_startup_system(load_playground_resources)
            .add_startup_system(setup_playground)
            .add_system(finish_setup_playgroud);
    }
}
