use std::fs;
use std::sync::{Arc, Mutex};

use bevy::render::texture::ImageType;
use bevy::{prelude::*, render::texture::CompressedImageFormats};

use crate::{
    hexagon::{Hexagon3D, Hexagon3DTexturing},
    game::Game,
};

pub struct PlaygroundPlugin {}

type ThreadsafeValue<T> = Arc<Mutex<Option<T>>>;

#[derive(Component)]
pub struct MeshGenTask {
    pub mesh: ThreadsafeValue<Mesh>,
    pub tiles_heigh_info: Vec<Vec<f32>>
}

// marker component so we know the current instantiated playground.
#[derive(Component)]
pub struct PlaygroundMarker {}

// a resource that knows the selected an all available maps,
// as well a status on loading to prevent loading of 2 maps at the same time.
pub struct MapRegistry {
    pub registered_heighmaps: Vec<String>,
    pub current_loaded_index: usize,
    pub is_loaded: bool,
    pub is_loading: bool,
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

//function that creates the mesh on a separate thread and stores it in the ThreadsafeValue box.
fn create_mesh_on_thread(
    mutex: ThreadsafeValue<Mesh>,
    asset_to_load_plain: String,
    game_width: i32,
    game_height: i32,
    game_hex_spacing: hex2d::Spacing,
) {
    let hm: Image;

    //let asset_to_load = "assets/heighmap_mt_taranaki.png";
    //let asset_to_load = "assets/heighmap_schweinskopf.png";
    let asset_to_load = format!("assets/heightmap_{}.png", asset_to_load_plain);

    info!("Loading: {}", asset_to_load);
    match fs::read(asset_to_load.clone()) {
        Ok(read_file) => {
            match Image::from_buffer(
                &read_file,
                ImageType::Extension("png"),
                CompressedImageFormats::all(),
                false,
            ) {
                Ok(image) => {
                    hm = image;
                }
                Err(e) => {
                    info!("error loading image {:?}", e);
                    return;
                }
            }
        }
        Err(_) => {
            info!("file does not exist: {}", asset_to_load);
            return;
        }
    }

    // 2 dimensional array of hexagons
    let mut hexes_2d: Box<Vec<Vec<Hexagon3D>>> = Box::new(vec![]);

    let mut highest_pixel_x: f32 = 0.;
    let mut highest_pixel_y: f32 = 0.;
    let mut lowest_pixel_x: usize = usize::MAX;
    let mut lowest_pixel_y: usize = usize::MAX;


    // for (i, shape) in shapes.into_iter().enumerate() {
    for x in 0..game_width {
        let mut hexes_x: Vec<Hexagon3D> = vec![];

        for y in 0..game_height {
            let c = hex2d::Coordinate::new(x, y);
            let (x_pixel, y_pixel) = c.to_pixel(game_hex_spacing);

            let mut z_pixel = 0.;

            if x_pixel < hm.size().x && -y_pixel < hm.size().y {
                let img_x = x_pixel as usize;
                let img_y = -y_pixel as usize;
                let img_width = hm.size().x as usize;

                //store the highest pixel x and y
                if img_x > highest_pixel_x as usize {
                    highest_pixel_x = img_x as f32;
                }

                if img_y > highest_pixel_y as usize {
                    highest_pixel_y = img_y as f32;
                }

                // store the lowest pixel x and y
                if img_x < lowest_pixel_x {
                    lowest_pixel_x = img_x;
                }

                if img_y < lowest_pixel_y {
                    lowest_pixel_y = img_y;
                }

                let pixel_value = get_grayscale(&hm.data, img_x, img_y, img_width);
                //info!("pixel_value: x {} y {} -> {}",img_x, img_y, pixel_value);
                z_pixel = 100. - (1. - (pixel_value / 255.0)) * 100.;
            }
            else {
                continue;
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
        if hexes_x.len() > 0 {
            hexes_2d.push(hexes_x);
        }
    }
    info!("start creating mesh highest x - y {} {} {} {}", highest_pixel_x, highest_pixel_y, lowest_pixel_x, lowest_pixel_y);
    
    let texturing = Hexagon3DTexturing::new_height_based_texturing();
    let mesh = Hexagon3D::create_mesh_for_hexes(&hexes_2d, &texturing);
    info!("mesh created");
    mutex.lock().unwrap().replace(mesh);
}

fn setup_playground(mut commands: Commands) {
    info!("setting up playground");
    // while!images.contains(&heightmap_cache.heightmap) {
    //   info!("waiting for heightmap");
    // }

    let map_registry = MapRegistry {
        registered_heighmaps: vec![
            "mt_taranaki".to_string(),
            "schweinskopf".to_string(),
            "alps".to_string(),
            "alps2".to_string(),
            "autobahn".to_string(),
            "lower_alps".to_string(),
            "graz".to_string(),
            "schoeckl".to_string(),
            "baernbach".to_string(),
        ],
        current_loaded_index: 6,
        is_loaded: false,
        is_loading: false,
    };

    commands.insert_resource(map_registry);

    //let mesh_gen_task = MeshGenTask { mesh: Arc::new(Mutex::new(None)) };
}

fn start_loading(
    mut commands: Commands,
    game: Res<Game>,
    mut map_registry: ResMut<MapRegistry>,
    input: Res<Input<KeyCode>>
) {
    // if we are already loading, don't do anything!
    if map_registry.is_loading {
        return;
    }

    let mut load = false;
    if !map_registry.is_loaded {
        load = true;
    } else {
        // if key page up is pressed, load next map
        if input.just_pressed(KeyCode::PageUp) {
            map_registry.current_loaded_index += 1;
            if map_registry.current_loaded_index >= map_registry.registered_heighmaps.len() {
                map_registry.current_loaded_index = 0;
            }
            load = true;
        } else if input.just_pressed(KeyCode::PageDown) { 
            // if key page down is pressed, load previous map
            
            if map_registry.current_loaded_index == 0 {
                map_registry.current_loaded_index = map_registry.registered_heighmaps.len() - 1;
            } else {
                map_registry.current_loaded_index -= 1;
            }

            load = true;
        }
    }

    if !load {
        return;
    }

    let game_width = game.width;
    let game_height = game.height;
    let game_hex_spacing = game.hex_spacing;

    let mutex: ThreadsafeValue<Mesh> = Arc::new(Mutex::new(None));
    let mutex2 = mutex.clone();
    let map_to_load = map_registry.registered_heighmaps[map_registry.current_loaded_index].clone();

    
    info!("start mesh generation in own thread");
    map_registry.is_loading = true;
    std::thread::spawn(move || {
        create_mesh_on_thread(
            mutex,
            map_to_load,
            game_width,
            game_height,
            game_hex_spacing,
        );
    });

    let gen_task = MeshGenTask { mesh: mutex2, tiles_heigh_info: Vec<Vec<f32>>::new() };
    commands.spawn().insert(gen_task);
}

fn integrate_loaded_maps(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mesh_gen_task: Query<(Entity, &MeshGenTask)>,
    old_playgrounds: Query<Entity, With<PlaygroundMarker>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut map_registry: ResMut<MapRegistry>,
) {
    for (entity, mesh_gen_task) in mesh_gen_task.iter() {
        let mut lock_guard = mesh_gen_task.mesh.lock().unwrap();
        // info!("checking if mesh is ready");
        if lock_guard.is_some() {

            // despawn old entities.
            for old_playground in old_playgrounds.iter() {
                info!("despawning old playground");
                commands.entity(old_playground).despawn();
            }

            
            // consumes lock_guard and returns the mesh
            let mesh = lock_guard.take().unwrap();

            let texture: Handle<Image> = asset_server.load("mountain_texture_less_sat.png");

            let mesh_handle = meshes.add(mesh);

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
            }).insert(PlaygroundMarker {});

            info!("mesh spawned");

            map_registry.is_loading = false;
            map_registry.is_loaded = true;

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
            .add_system(start_loading)
            .add_system(integrate_loaded_maps);
    }
}
