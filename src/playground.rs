use std::fs;
use std::sync::{Arc, Mutex};

use bevy::render::texture::ImageType;
use bevy::{prelude::*, render::texture::CompressedImageFormats};
use hex2d::Spacing;
use sn_rust::field_2_d::Field2D;
use sn_rust::indexed_field_2_d::IndexedField2D;
use sn_rust::mip_map_field_2_d::MipMapField2D;

use crate::components::PositionComponent;

use crate::interpolation::interpolation_hex2d;
use crate::{
    game::Game,
    hexagon::{Hexagon3D, Hexagon3DTexturing},
};

type ThreadsafeValue<T> = Arc<Mutex<Option<T>>>;

#[derive(Component)]
pub struct MeshGenTask {
    pub mesh: ThreadsafeValue<Mesh>,
    pub mutex_lod_meshes: ThreadsafeValue<Vec<Mesh>>,
    pub mutex_height: ThreadsafeValue<MipMapField2D<i64>>,
    // pub tiles_heigh_info: Vec<Vec<f32>>
}

// marker component so we know the current instantiated playground.
#[derive(Component)]
pub struct PlaygroundMarker {}

// a resource that knows the selected an all available maps,
// as well a status on loading to prevent loading of 2 maps at the same time.
#[derive(Resource)]
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
    mutex_heights: ThreadsafeValue<MipMapField2D<i64>>,
    mutex_mesh_large: ThreadsafeValue<Vec<Mesh>>,
    asset_to_load_plain: String,
    game_width: u32,
    game_height: u32,
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
    // let mut hexes_2d = IndexedField2D::<Hexagon3D>::new(game_width as usize, game_height as usize);

    let mut highest_pixel_x: f32 = 0.;
    let mut highest_pixel_y: f32 = 0.;
    let mut lowest_pixel_x: usize = usize::MAX;
    let mut lowest_pixel_y: usize = usize::MAX;

    let mut height_field = create_height_field(game_width, game_height, game_hex_spacing);
    // for (i, shape) in shapes.into_iter().enumerate() {

    // let max_x_render_mesh = 100 as u32; //(game_width as f32 * 0.8) as u32;
    // let max_y_render_mesh = 100 as u32; //(game_height as f32 * 0.8) as u32;
    for x in 0..game_width {
        for y in 0..game_height {
            let c = hex2d::Coordinate::new(x as i32, y as i32);
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

                // we get a value between 0 and 255.
                //

                //info!("pixel_value: x {} y {} -> {}",img_x, img_y, pixel_value);
                z_pixel = 100. - (1. - (pixel_value / 255.0)) * 100.;
                // info!("z_pixel", z_pixel);
                height_field.set(x as usize, y as usize, (z_pixel * 1000.0) as i64);
            } else {
                continue;
            }

            // if x < max_x_render_mesh && y < max_y_render_mesh {
            //     let hex = Hexagon3D {
            //         diameter: 1.,
            //         height: 0.,
            //         x: x_pixel,
            //         y: z_pixel,
            //         z: y_pixel,
            //     };
            //     hexes_2d.set(x, y, Some(hex));
            // }
            //hexes.push(hex);
        }
    }
    info!(
        "start creating mesh highest x - y {} {} {} {}",
        highest_pixel_x, highest_pixel_y, lowest_pixel_x, lowest_pixel_y
    );

    height_field.finalize_mip_map();
    let texturing = Hexagon3DTexturing::new_height_based_texturing();

    info!("start creating hexes including mip_maps LODS");

    //let distance_incr = 81;
    //let distance_incr = 234;
    // let distance_incr = 729;
    let pow_3 = usize::pow(3,3); // 27
    let pow_4 = usize::pow(3,4); // 81
    let pow_5 = usize::pow(3,5); // 243
    let pow_6 = usize::pow(3,6); // 729
    let pow_7 = usize::pow(3,7); // 2187
    let pow_8 = usize::pow(3,8); // 6561
    let pow_9 = usize::pow(3,9);

    //let lod_steps = [1, pow_5, pow_6, pow_7, pow_8, pow_9];

    // let lod_steps = [0, 3, 9, 27, pow_4, pow_5];

    // let lod_steps = [0, 9, 18, 27, 36, 45];
    // let mut current_distance = 0;

    //let mut increment = 243;
    let increment = pow_5;
    
    //let double_draw_distance = 27;
    let double_draw_distance = 27;

    //let lod_steps = [0, increment, increment * 2 ,increment * 3,increment * 4, increment * 5, increment * 6, increment * 7 ];

    let lod_steps = [ 0, pow_3, pow_5, 3 * pow_5, 4 * pow_5, 5 * pow_5 ];


    let hexes_lod_0 = create_hexes_lod_x(
        lod_steps[0],
        lod_steps[1] + double_draw_distance,
        0,
        game_hex_spacing,
        height_field.field(),
    );

   // let lod_1_spacing = flip_spacing(game_hex_spacing);
    // let hexes_lod_1 = create_hexes_lod_1(

    //     33,
    //     33,
    //     game_hex_spacing,
    //     &height_field.get_mip_map(),
    // );

    info!("hexes lod 0: {}", hexes_lod_0.indeces().len());

    let mut meshes = Vec::<Mesh>::new();

    for i in 0..height_field.get_mip_maps().len() {

        
        let lod = i + 1;

        if lod > lod_steps.len() - 2 {
            break;
        }

        let lod_step_min = lod_steps[lod];
        let lod_step_max = lod_steps[lod + 1] + double_draw_distance;
        
        let field = height_field.get_mip_maps().get(i).unwrap();
        let hexes = create_hexes_lod_x(
            0, // lod_step_min,
            1200, // lod_step_max + double_draw_distance,
            lod,
            game_hex_spacing,
            field,
        );

        info!("hexes lod {}: {}", lod, hexes.indeces().len());
        let mesh = Hexagon3D::create_mesh_for_hexes(hexes.as_ref(), &texturing);
        let vertices = mesh.count_vertices();
        info!("vertices lod {}: {}", lod, vertices);

        meshes.push(mesh);
    }

    let mesh_lod_0 = Hexagon3D::create_mesh_for_hexes(hexes_lod_0.as_ref(), &texturing);
    let lod_0_vertices = mesh_lod_0.count_vertices();
    info!(
        "lod 0 vertices: {} - hexes: {}",
        lod_0_vertices,
        hexes_lod_0.indeces().len()
    );

    mutex.lock().unwrap().replace(mesh_lod_0);
    mutex_heights.lock().unwrap().replace(height_field);
    mutex_mesh_large.lock().unwrap().replace(meshes);
}

fn flip_spacing(game_hex_spacing: Spacing) -> Spacing {
    match game_hex_spacing {
        Spacing::FlatTop(f) => Spacing::PointyTop(f),
        Spacing::PointyTop(f) => Spacing::FlatTop(f),
    }
}

fn create_hexes_lod_x(
    min_distance: usize,
    max_distance: usize,
    lod_level: usize,
    spacing_orig: Spacing<f32>,
    mip_map: &Field2D<i64>,
) -> Box<IndexedField2D<Hexagon3D>> {

    //let spacing = spacing_orig * lod_level * 3;


    // Lod 0: 1
    // lod 1: 3
    // lod 2: 4.5

    //let spacing_multiplier_options = [1.0, 3.0, 4.5, 6.0, 7.5, 9.0 ];
    //let spacing_multiplier_options = [1.0, 3.0, 4.5, 4.7, 7.5, 9.0 ];
    let spacing_multiplier_options = [1.0, 3.0, 9.0, 27.0, 81.0, 243.0 ];
    let spacing_multi = spacing_multiplier_options[lod_level];
    let mut spacing = match spacing_orig {
        Spacing::FlatTop(f) => {
            Spacing::FlatTop(f * spacing_multi)
        }
        Spacing::PointyTop(f) => {
            Spacing::PointyTop(f *  spacing_multi)
        }
    };

    if lod_level == 0 {
        spacing = spacing_orig;
    }

    info!("lod {} min_distance {} max_distance {} ", lod_level, min_distance, max_distance);
    let pos_x = 0;
    let pos_y = 0;

    // let current_pos = if lod_level == 0 {
    //     hex2d::Coordinate::new(pos_x as i32, pos_y as i32)
    // } else {
    //     hex2d::Coordinate::new(
    //         (pos_x / 3 * lod_level) as i32,
    //         (pos_y / 3 * lod_level) as i32,
    //     )
    // };

    let current_pos = hex2d::Coordinate::new(pos_x as i32, pos_y as i32);

    let mut result = Box::new(IndexedField2D::<Hexagon3D>::new(
        mip_map.width().clone(),
        mip_map.height().clone(),
    ));

    let lod_correction_value = 3 * lod_level; //usize::pow(3, lod_level as u32);

    let min_distance_lod_correction = if lod_level == 0 {
        min_distance
    } else {
        min_distance / lod_correction_value
    };
    let max_distance_lod_correction = if lod_level == 0 {
        max_distance
    } else {
        max_distance / lod_correction_value
    };

    for ring_distance in min_distance_lod_correction..=max_distance_lod_correction {
        info!("ring_distance: {}", ring_distance);
        let ring =
            current_pos.ring_iter(ring_distance as i32, hex2d::Spin::CW(hex2d::Direction::XZ));

        for c in ring {
            if c.x < 0
                || c.y < 0
                || c.x >= (mip_map.width().clone() as i32)
                || c.y >= (mip_map.height().clone() as i32)
            {
                // info!("c: {:?} not on playground - skipping", c);
                continue;
            }

            // info!("c: {:?}", c);
            //let c = hex2d::Coordinate::new(x as i32, y as i32);
            let (x_pixel, y_pixel) = c.to_pixel(spacing);

            let z_pixel = mip_map.get(c.x as usize, c.y as usize) / 1000;

            // info!("pixel x {} y {} ", x_pixel, y_pixel);
            let hex = Hexagon3D {
                diameter: 1.,
                height: 0.,
                x: x_pixel,
                y: z_pixel as f32,
                z: y_pixel,
            };

            result.set(c.x as u32, c.y as u32, Some(hex));
        }
    }

    return result;
}

fn create_height_field(width: u32, height: u32, hex_spacing: Spacing) -> MipMapField2D<i64> {
    

    
    let hex_spacing_orig = hex_spacing.clone();

    // let interpolation : fn(usize,usize, usize, &Field2D<i64>) -> i64 = |x,y,lod,f|  {
        
    //     hex_spacing_orig.clone();
    //     // we get the info of the x and y cooordinate in the old field field.
    //     // we should deliver surrounding nodes.
    //     return 0;
        
    // };

    let mut height_field =    MipMapField2D::<i64>::new(width as usize, height as usize, interpolation_hex2d);

    return height_field;
}

fn setup_playground(mut commands: Commands) {
    info!("setting up playground map registy");
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

    info!("setting up playground map registy - done");
    //let mesh_gen_task = MeshGenTask { mesh: Arc::new(Mutex::new(None)) };
}

fn start_loading(
    mut commands: Commands,
    game: Res<Game>,
    mut map_registry: ResMut<MapRegistry>,
    input: Res<Input<KeyCode>>,
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
    let mutex_lod_meshes: ThreadsafeValue<Vec<Mesh>> = Arc::new(Mutex::new(None));
    let mutex2 = mutex.clone();
    let map_to_load = map_registry.registered_heighmaps[map_registry.current_loaded_index].clone();

    let mutex_height: ThreadsafeValue<MipMapField2D<i64>> = Arc::new(Mutex::new(None));
    let mutex_height_clone = mutex_height.clone();

    let mutex_lod_meshes_clone = mutex_lod_meshes.clone();
    info!("start mesh generation in own thread");
    map_registry.is_loading = true;
    std::thread::spawn(move || {
        create_mesh_on_thread(
            mutex,
            mutex_height_clone,
            mutex_lod_meshes_clone,
            map_to_load,
            game_width,
            game_height,
            game_hex_spacing,
        );
    });

    let gen_task = MeshGenTask {
        mesh: mutex2,
        mutex_height: mutex_height,
        mutex_lod_meshes: mutex_lod_meshes,
    };
    commands.spawn(gen_task);
}

fn integrate_loaded_maps(
    mut commands: Commands,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
    mesh_gen_task: Query<(Entity, &MeshGenTask)>,
    old_playgrounds: Query<Entity, With<PlaygroundMarker>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut map_registry: ResMut<MapRegistry>,
    mut query_positions: Query<(&PositionComponent, &mut Transform)>,
) {
    for (entity, mesh_gen_task) in mesh_gen_task.iter() {
        let mesh = if let Ok(mut lock_guard) = mesh_gen_task.mesh.try_lock() {
            lock_guard.take()
        } else {
            return;
        };

        if mesh.is_none() {
            return;
        }

        info!("integrating loaded map");
        // info!("checking if mesh is ready");

        // despawn old entities.
        for old_playground in old_playgrounds.iter() {
            info!("despawning old playground");
            commands.entity(old_playground).despawn();
        }

        let texture: Handle<Image> = asset_server.load("mountain_texture_less_sat.png");

        let mesh_handle = meshes.add(mesh.unwrap());

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

        commands
            .spawn(PbrBundle {
                mesh: mesh_handle,
                material: mat2.clone(),
                transform: Transform {
                    translation: Vec3::new(0., 0., 0.),
                    // rotation: quat.clone(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(PlaygroundMarker {});

        info!("mesh spawned");

        map_registry.is_loading = false;
        map_registry.is_loaded = true;

        let mut lock_guard_heights = mesh_gen_task.mutex_height.lock().unwrap();

        if lock_guard_heights.is_some() {
            let mut heights = lock_guard_heights.take().unwrap();

            // update the standing position of the uis.
            for (pos, mut transform) in query_positions.iter_mut() {
                let height = heights.get_u32(pos.x, pos.y).clone();
                transform.translation.y = (height / 1000) as f32 + 0.4;
            }

            game.set_height_field(heights);
        } else {
            error!("Unexpected behavior: heights are not loaded");
        }

        if let mut lock_guard_heights = mesh_gen_task.mutex_lod_meshes.lock().unwrap() {
            if lock_guard_heights.is_some() {
                let mut meshes_lod = lock_guard_heights.take().unwrap();

                meshes_lod.reverse();
                
                // game.set_mesh_large(mesh_large);

                let mut lod_level = 1;
                while let Some(mesh) = meshes_lod.pop() {
                    let mesh_handle_large = meshes.add(mesh);

                    // let scaling = 3.0 * lod_level as f32; //f32::powi(3.0, lod_level);
                    commands
                        .spawn(PbrBundle {
                            mesh: mesh_handle_large,
                            material: mat2.clone(),
                            transform: Transform {
                                translation: Vec3::new(0., -1.0000 * (lod_level as f32), 0.),
                                // scale: Vec3::new(scaling, 1., scaling),
                                // rotation: quat.clone(),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(PlaygroundMarker {});

                    lod_level += 1;
                }
            } else {
                error!("Unexpected behavior: mesh large is not loaded");
            }
        }

        commands.entity(entity).despawn_recursive();
    }
    //mesh_gen_task
}

// info!("integrating loaded map done");

pub struct PlaygroundPlugin {}

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
