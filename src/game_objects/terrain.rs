// src/terrain.rs
// version:0.1.9
// ----START OF FILE----
use box2d_rs::b2_body::{B2body, B2bodyDef, B2bodyType};
use box2d_rs::b2_fixture::B2fixtureDef;
use box2d_rs::b2_math::B2vec2;
use box2d_rs::b2_shape::B2shapeDynTrait;
use box2d_rs::b2_timer::B2timer;
use box2d_rs::b2_world::B2world;
use box2d_rs::shapes::b2_polygon_shape::B2polygonShape;
use earcut::Earcut;
use macroquad::prelude::{BLACK as MQ_BLACK, draw_triangle as mq_draw_triangle, vec2 as mq_vec2};
use macroquad::rand::rand;
use std::cell::{Ref, RefCell};
use std::f32::consts::PI;
use std::rc::Rc;
use visioncortex::{BinaryImage, BitVec, PathI32, PathSimplifyMode, clusters};

use fastnoise_lite::{FastNoiseLite, FractalType, NoiseType};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use image::{GrayImage, Luma};

use crate::physics_object::NoUserData;
type BodyPtr<D> = Rc<RefCell<B2body<D>>>;
type WorldPtr<D> = Rc<RefCell<B2world<D>>>;

trait FromToGrayImage {
    fn from_gray_image(img: &GrayImage) -> Self;
    fn to_gray_image(&self) -> GrayImage;
    fn new_usize(width: u32, height: u32) -> BinaryImage;
}

impl FromToGrayImage for BinaryImage {
    fn from_gray_image(img: &GrayImage) -> Self {
        // 1) pobieramy wymiary i rzutujemy U32→u32 przez `as` (tu zawsze bezpieczne)
        let width = img.width() as usize;
        let height = img.height() as usize;

        // 2) budujemy BitVec, threshold = 0 (piksel > 0  → 1, inaczej 0)
        let mut bits = BitVec::with_capacity(width * height);
        for pixel in img.pixels() {
            let Luma([l]) = *pixel;
            bits.push(l > 0);
        }

        BinaryImage {
            pixels: bits,
            width,
            height,
        }
    }
    fn to_gray_image(&self) -> GrayImage {
        // walidujemy zakres u32 → u32
        let w32: u32 = self.width.try_into().expect("width exceeds u32::MAX");
        let h32: u32 = self.height.try_into().expect("height exceeds u32::MAX");

        let mut img = GrayImage::new(w32, h32);
        for (i, bit) in self.pixels.iter().enumerate() {
            let x = (i % self.width) as u32;
            let y = (i / self.width) as u32;
            let value = if bit { 255 } else { 0 };
            img.put_pixel(x, y, Luma([value]));
        }
        img
    }
    fn new_usize(width: u32, height: u32) -> BinaryImage {
        BinaryImage::new_w_h(width as usize, height as usize)
    }
}

const MIN_SPECKLE_SIZE: u32 = 10;

#[derive(Debug, Clone, Copy)]
enum TerrainGenerationType {
    HillyWithNoise,
    SwissCheese,
}

// Funkcje pomocnicze, które mogą pozostać poza strukturą lub stać się prywatnymi metodami
// Na razie zostawiam je jako funkcje modułu dla czytelności.
fn string_to_i32_seed(s: &str) -> i32 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish() as i32
}

fn save_density_map_as_png(binary_image: &BinaryImage, filename: &str) {
    let gray_image = binary_image.to_gray_image();

    match gray_image.save(filename) {
        Ok(_) => println!("Successfully saved density map to {}", filename),
        Err(e) => eprintln!("Error saving density map to {}: {}", filename, e),
    }
}

pub struct Terrain {
    pub density_map: BinaryImage,
    world: WorldPtr<NoUserData>,
    width: u32,
    height: u32,
    _seed_str: String,
    pub active_triangles: Vec<[B2vec2; 3]>,
    pub body:BodyPtr<NoUserData> 
}

impl Terrain {
    pub fn new(
        world: WorldPtr<NoUserData>,
        width: u32,
        height: u32,
        seed_str: &str,
    ) -> Self {
        // Zwraca (Terrain, ground_y_level_ref)
        let seed_i32 = string_to_i32_seed(seed_str);
        let terrain_choice_seed = seed_i32.wrapping_abs() % 2;
        let generation_type = if terrain_choice_seed == 0 {
            TerrainGenerationType::HillyWithNoise
        } else {
            TerrainGenerationType::SwissCheese
        };
        println!("Selected terrain type: {:?}", generation_type);

        let density_map = match generation_type {
            TerrainGenerationType::HillyWithNoise => {
                Self::generate_hilly_terrain_density_map_internal(width, height)
            }
            TerrainGenerationType::SwissCheese => {
                Self::generate_swiss_cheese_density_map_internal(width, height)
            }
        };

        save_density_map_as_png(&density_map, "dbg/terrain.png");

        let triangles = Self::raster_to_triangles(&density_map, 10);
        let body = Self::build_body(&world, &triangles);

        let terrain_instance = Self {
            density_map,
            world,
            width,
            height,
            _seed_str: seed_str.to_string(),
            active_triangles: triangles,
            body
        };
        (terrain_instance)
    }

    fn path_to_triangles(path: PathI32, holes: Vec<PathI32>, height: usize) -> Vec<[B2vec2; 3]> {
        let mut vertices: Vec<[f32; 2]> = Vec::new();
        let mut hole_indices: Vec<usize> = Vec::new();
        let triangles_out: &mut Vec<usize> = &mut Vec::new();
        let mut earcutter = Earcut::new();
        let mut result_triangles: Vec<[B2vec2; 3]> = Vec::new();
        for point in path.iter() {
            vertices.push([point.x as f32, (height - point.y as usize) as f32]);
        }
        // for hole in holes {
        //     for point in hole.path {
        //         vertices.push([point.x as f32, point.y as f32]);
        //         hole_indices.push(vertices.len())
        //     }
        // }
        earcutter.earcut(vertices.iter().cloned(), &hole_indices, triangles_out);
        for k in (0..triangles_out.len()).step_by(3) {
            let i1 = triangles_out[k];
            let i2 = triangles_out[k + 1];
            let i3 = triangles_out[k + 2];
            let triangle = [
                B2vec2::new(vertices[i1][0], vertices[i1][1]),
                B2vec2::new(vertices[i2][0], vertices[i2][1]),
                B2vec2::new(vertices[i3][0], vertices[i3][1]),
            ];
            if triangle[0] != triangle[1]
                && triangle[1] != triangle[2]
                && triangle[0] != triangle[2]
            {
                result_triangles.push(triangle);
            } else {
                eprintln!("Triangle has duplicate points: {:?}", triangle);
            }

            // result_triangles.push(triangle);
        }
        result_triangles
    }

    pub fn raster_to_triangles(
        binary_img: &BinaryImage,
        speckle_filter: usize,
    ) -> Vec<[B2vec2; 3]> {
        let mut result_triangles: Vec<[B2vec2; 3]> = Vec::new();
        let timer_total = B2timer::default();
        let mode = PathSimplifyMode::Polygon;
        let clusters = binary_img.to_clusters(false);
        let mut counter = 0;
        let height = binary_img.height;

        for cluster in clusters {
            let cluster_triangles: Vec<[B2vec2; 3]>;
            if cluster.size() > speckle_filter {
                let image = cluster.to_binary_image();
                let filename = format!("dbg/terrain{}{}", counter, ".png");
                counter += 1;
                save_density_map_as_png(&image, &filename);
                let paths = clusters::Cluster::image_to_paths(&image, mode);
                if paths.len() == 1 {
                    cluster_triangles = Self::path_to_triangles(paths[0].clone(), Vec::new(), height);
                } else if paths.len() > 1 {
                    cluster_triangles =
                    Self::path_to_triangles(paths[0].clone(), paths[1..paths.len()].to_vec(), height);
                } else {
                    cluster_triangles = Vec::new();
                    println!(
                        "No paths generated for cluster"
                    );
                }

                println!(
                    "Cluster {} size {} num paths {}, triangles {}",
                    counter,
                    cluster.size(),
                    paths.len(),
                    cluster_triangles.len()
                );
                result_triangles.extend(cluster_triangles);
            }
        }

        let total_time_ms = timer_total.get_milliseconds();
        println!(
            "Total terrain conversion (image_to_paths_logic + earcut) time {} triangles {}",
            total_time_ms,
            result_triangles.len()
        );
        result_triangles
    }

    fn generate_hilly_terrain_density_map_internal(width: u32, height: u32) -> BinaryImage {
        let mut binary_image = BinaryImage::new_w_h(width as usize, height as usize);

        let mut noise_generator = FastNoiseLite::with_seed(rand() as i32);
        noise_generator.set_noise_type(Some(NoiseType::OpenSimplex2));
        noise_generator.set_frequency(Some(0.005));

        let base_ground_image_y = height as f32 * 0.65;
        let hill1_amplitude = height as f32 * 0.15;
        let hill1_frequency = 2.0 * PI / (width as f32 * 0.7);
        let hill1_x_offset = width as f32 * 0.2;

        let hill2_amplitude = height as f32 * 0.10;
        let hill2_frequency = 2.0 * PI / (width as f32 * 0.55);
        let hill2_x_offset = width as f32 * 0.65;

        let noise_influence_factor = height as f32 * 0.1;

        for y_u32 in 0..height {
            for x_u32 in 0..width {
                let x_f32 = x_u32 as f32;
                let y_f32 = y_u32 as f32;

                let hill1_y_offset =
                    hill1_amplitude * ((x_f32 - hill1_x_offset) * hill1_frequency).sin();
                let hill2_y_offset =
                    hill2_amplitude * ((x_f32 - hill2_x_offset) * hill2_frequency).sin();
                let mut terrain_surface_image_y_sin =
                    base_ground_image_y - hill1_y_offset - hill2_y_offset;

                let noise_val =
                    noise_generator.get_noise_2d(x_f32 * 0.5, y_f32 * 0.1 + x_f32 * 0.02);
                let scaled_noise_offset = noise_val * noise_influence_factor;
                terrain_surface_image_y_sin += scaled_noise_offset;

                let surface_noise_freq = 0.03;
                let surface_noise_amp = height as f32 * 0.02;
                noise_generator.set_frequency(Some(surface_noise_freq));
                let surface_noise_val = noise_generator.get_noise_2d(x_f32, y_f32 * 0.5);
                terrain_surface_image_y_sin += surface_noise_val * surface_noise_amp;
                noise_generator.set_frequency(Some(0.005));

                let terrain_surface_image_y = terrain_surface_image_y_sin
                    .max(0.0)
                    .min(height as f32 - 1.0);
                let is_terrain = y_f32 >= terrain_surface_image_y;
                binary_image.set_pixel(x_u32 as usize, y_u32 as usize, is_terrain);
            }
        }
        binary_image
    }

    fn generate_swiss_cheese_density_map_internal(width: u32, height: u32) -> BinaryImage {
        let mut binary_image = BinaryImage::new_w_h(width as usize, height as usize);

        let mut hole_noise_gen = FastNoiseLite::with_seed(rand() as i32);
        hole_noise_gen.set_noise_type(Some(NoiseType::OpenSimplex2));
        hole_noise_gen.set_frequency(Some(0.002));
        hole_noise_gen.set_fractal_type(Some(FractalType::FBm));
        hole_noise_gen.set_fractal_octaves(Some(2));
        hole_noise_gen.set_fractal_lacunarity(Some(2.0));
        hole_noise_gen.set_fractal_gain(Some(0.5));

        for y in 0..height as usize {
            for x in 0..width as usize {
                binary_image.set_pixel(x, y, true);
            }
        }
        let threshold = 0.25;
        for y_u32 in 0..height {
            for x_u32 in 0..width {
                let x_f32 = x_u32 as f32;
                let y_f32 = y_u32 as f32;
                let noise_val = hole_noise_gen.get_noise_2d(x_f32, y_f32);
                let scaled_noise_val = (noise_val + 1.0) * 0.5;
                if scaled_noise_val > threshold {
                    binary_image.set_pixel(x_u32 as usize, y_u32 as usize, false);
                }
            }
        }
        binary_image
    }

    fn build_body(
        world: &WorldPtr<NoUserData>,
        triangles: &[[B2vec2; 3]],
    ) -> BodyPtr<NoUserData> {
        let body_def = B2bodyDef {
            body_type: B2bodyType::B2StaticBody,
            position: B2vec2::new(0.0, 0.0),
            ..Default::default()
        };
        let terrain_body = B2world::<NoUserData>::create_body(world.clone(), &body_def);
        for triangle_vertices in triangles.iter() {
            let mut polygon_shape = B2polygonShape::default();
            polygon_shape.set(triangle_vertices);
            if !polygon_shape.validate() {
                eprintln!(
                    "Invalid triangle for B2polygonShape: {:?}",
                    triangle_vertices
                );
                continue;
            }
            let shape_def_ptr: Rc<RefCell<dyn B2shapeDynTrait>> =
                Rc::new(RefCell::new(polygon_shape));
            let mut fd = B2fixtureDef::default();
            fd.shape = Some(shape_def_ptr);
            B2body::create_fixture(terrain_body.clone(), &fd);
        }
        let fixture_count = terrain_body.borrow().get_fixture_list().iter().count();
        println!(
            "Terrain::build_body_internal, fixture count: {}",
            fixture_count
        );
        terrain_body
    }

    pub fn deform_terrain(self, x: f32, y: f32, size: f32) {
        println!("Terrain::deform_terrain(x={}, y={}, size={})", x, y, size);
    }

    pub fn draw(&self) {
        // pixels_per_unit nie jest już potrzebne jako argument
        // ... (logika z poprzedniej funkcji draw_terrain) ...
        // Używa self.active_triangles
        for triangle in self.active_triangles.iter() {
            mq_draw_triangle(
                mq_vec2(triangle[0].x, triangle[0].y),
                mq_vec2(triangle[1].x, triangle[1].y),
                mq_vec2(triangle[2].x, triangle[2].y),
                MQ_BLACK,
            );
        }
    }

    // Helper function to check if a rectangular area in the density map is clear
    pub fn is_area_clear(&mut self, pos: B2vec2) -> bool {
        println!("Terrain::is_area_clear(pos=({},{})", pos.x, pos.y);
        true
    }

    pub fn find_safe_spawn_location(&mut self, pos: B2vec2) -> B2vec2 {
        println!(
            "Terrain::find_safe_spawn_location(pos=({},{})",
            pos.x, pos.y
        );
        pos
    }
}
// ----END OF FILE----
// src/terrain.rs
// version:0.1.9
