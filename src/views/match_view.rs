use crate::{View, Transition, GameViewType, GameContext};
use macroquad::prelude::*;
use box2d_rs::b2_world::B2world;
use box2d_rs::b2_math::B2vec2;
use std::rc::Rc;
use std::any::Any;



use crate::game_objects::terrain::Terrain;
use crate::game_objects::player::{{create_character, handle_normal_player_input_and_movement, draw_character}};
use crate::physics::{self, NoUserData, WorldPtr};

use box2d_rs::b2_body::BodyPtr;
use box2d_rs::b2_fixture::FixturePtr;
use visioncortex::BinaryImage; 




struct MatchCollisionListener {
    terrain_body_check: BodyPtr<NoUserData>,
}



pub struct MatchView {
    world: WorldPtr<NoUserData>,
    terrain: Terrain, 
    player_body: BodyPtr<NoUserData>,
    projectiles: Vec<BodyPtr<NoUserData>>,
    game_camera: Camera2D,
    current_camera_zoom_level: f32,
    
    
    god_mode_enabled: bool,
    density_map_alpha_mode: u8,
    density_map_texture_option: Option<Texture2D>, 
    conversion_time_ms: f32,
    
    
}


impl MatchView {
   pub fn new(context: & GameContext) -> Self {
        println!("MatchView: Initializing...");
        let world = B2world::<NoUserData>::new(B2vec2 { x: 0.0, y: -98.0 });
        let world_rc = Rc::clone(&world);

        
        const DENSITY_MAP_WIDTH_UNITS_MATCH: u32 = 2048; 
        const DENSITY_MAP_HEIGHT_UNITS_MATCH: u32 = 1536;
        const DENSITY_MAP_WIDTH_F_MATCH: f32 = DENSITY_MAP_WIDTH_UNITS_MATCH as f32;
        const DENSITY_MAP_HEIGHT_F_MATCH: f32 = DENSITY_MAP_HEIGHT_UNITS_MATCH as f32;
        const DEFAULT_TERRAIN_SEED_MATCH: &str = "whacka-molee_match"; 

        let character_half_width_world = 5.0;
        let character_half_height_world = 10.0;

        let mut terrain = Terrain::new(
            world_rc, 
            DENSITY_MAP_WIDTH_UNITS_MATCH, 
            DENSITY_MAP_HEIGHT_UNITS_MATCH, 
            DEFAULT_TERRAIN_SEED_MATCH
        );

        let player_start_pos = B2vec2::zero();
        

        let player_body = create_character(
            Rc::clone(&world), 
            player_start_pos, 
            character_half_width_world, 
            character_half_height_world
        );

        let initial_zoom = 0.75;
        let mut game_camera = Camera2D {
            target: vec2(player_start_pos.x, player_start_pos.y),
            zoom: vec2(0.0,0.0), 
            ..Default::default()
        };
        
        let aspect_ratio = screen_width() / screen_height();
        let map_aspect_ratio = DENSITY_MAP_WIDTH_F_MATCH / DENSITY_MAP_HEIGHT_F_MATCH;
         if aspect_ratio > map_aspect_ratio { 
            game_camera.zoom.x = initial_zoom / (DENSITY_MAP_HEIGHT_F_MATCH * aspect_ratio);
            game_camera.zoom.y = -initial_zoom / DENSITY_MAP_HEIGHT_F_MATCH;
        } else { 
            game_camera.zoom.x = initial_zoom / DENSITY_MAP_WIDTH_F_MATCH;
            game_camera.zoom.y = -initial_zoom / (DENSITY_MAP_WIDTH_F_MATCH / aspect_ratio);
        }


        Self {
            world,
            terrain,
            player_body,
            projectiles: Vec::new(),
            game_camera,
            current_camera_zoom_level: initial_zoom,
            god_mode_enabled: false,
            density_map_alpha_mode: 0,
            density_map_texture_option: None,
            conversion_time_ms: 0.0,
            
            
        }
    }
}

impl View for MatchView {
    fn on_enter(&mut self, _context: & GameContext) {
        println!("Entered Match View");
        
    }

    fn update_and_handle_input(&mut self, dt: f32, context: & GameContext) -> Transition {
        const DENSITY_MAP_WIDTH_F_MATCH: f32 = 2048.0;
        const DENSITY_MAP_HEIGHT_F_MATCH: f32 = 1536.0;
        const PIXELS_PER_UNIT_MATCH: f32 = 1.0;
        let character_half_width_world = 5.0;
        let character_half_height_world = 10.0;
        let character_speed_world = 250.0;
        let jump_force_world = 50000.0;
        let god_mode_fly_speed = 100.0;
        let projectile_radius_world = 5.0;
        let projectile_speed_world = 750.0;
        const MIN_CAMERA_ZOOM_LEVEL_MATCH: f32 = 0.20;
        const MAX_CAMERA_ZOOM_LEVEL_MATCH: f32 = 3.0;
        const CAMERA_SCROLL_ZOOM_SPEED_MATCH: f32 = 0.1;



        if is_key_pressed(KeyCode::Key1) {
            self.density_map_alpha_mode = (self.density_map_alpha_mode + 1) % 3;
            if self.density_map_alpha_mode != 0 {
                let mut rgba_bytes = Vec::with_capacity(self.terrain.density_map.width * self.terrain.density_map.height * 4);
                for y_img in 0..self.terrain.density_map.height {
                    for x_img in 0..self.terrain.density_map.width {
                        if self.terrain.density_map.get_pixel(x_img, y_img) { rgba_bytes.extend_from_slice(&[0,0,0,255]); } 
                        else { rgba_bytes.extend_from_slice(&[255,255,255,255]); }
                    }
                }
                self.density_map_texture_option = Some(Texture2D::from_image(&Image {
                    bytes: rgba_bytes, width: self.terrain.density_map.width as u16, height: self.terrain.density_map.height as u16,
                }));
            } else { self.density_map_texture_option = None; }
        }

        self.world.borrow_mut().step(dt, 8, 3);

        // self.game_camera.target = vec2(player_pos.x, player_pos.y);

        let (_mwx, mwy) = mouse_wheel();
        if mwy != 0.0 {
            self.current_camera_zoom_level -= mwy * self.current_camera_zoom_level * CAMERA_SCROLL_ZOOM_SPEED_MATCH;
            self.current_camera_zoom_level = self.current_camera_zoom_level.clamp(MIN_CAMERA_ZOOM_LEVEL_MATCH, MAX_CAMERA_ZOOM_LEVEL_MATCH);
        }
        
        let aspect_ratio = screen_width() / screen_height();
        let map_aspect_ratio = DENSITY_MAP_WIDTH_F_MATCH / DENSITY_MAP_HEIGHT_F_MATCH;
        if aspect_ratio > map_aspect_ratio { 
            self.game_camera.zoom.x = self.current_camera_zoom_level / (DENSITY_MAP_HEIGHT_F_MATCH * aspect_ratio);
            self.game_camera.zoom.y = -self.current_camera_zoom_level / DENSITY_MAP_HEIGHT_F_MATCH;
        } else { 
            self.game_camera.zoom.x = self.current_camera_zoom_level / DENSITY_MAP_WIDTH_F_MATCH;
            self.game_camera.zoom.y = -self.current_camera_zoom_level / (DENSITY_MAP_WIDTH_F_MATCH / aspect_ratio);
        }


        Transition::None
    }

    fn draw(&self, context: &GameContext) {
        set_camera(&self.game_camera);
        self.terrain.draw();
        draw_character(self.player_body.clone(), 1.0, GREEN);
        // projectile::draw_projectiles(&self.projectiles, 1.0);

        // if let Some(ref texture) = self.density_map_texture_option {
        //      if self.density_map_alpha_mode != 0 { /* ... rysowanie tekstury mapy gęstości ... */ }
        // }

        set_default_camera();
        
        // game_view::draw_ui_info(game_view::UiInfo {
        //     player_pos: self.player_body.borrow().get_position(),
        //     projectiles_count: self.projectiles.len(),
        //     god_mode_enabled: self.god_mode_enabled,
        //     density_map_alpha_mode: self.density_map_alpha_mode,
        //     active_terrain_elements_count: self.terrain.active_triangles.len(),
        //     conversion_time_ms: self.conversion_time_ms,
        //     deformation_enabled: true, 
        //     game_camera: &self.game_camera,
        //     density_map_width_f: 2048.0, 
        //     density_map_height_f: 1536.0,
        //     camera_zoom_level: self.current_camera_zoom_level,
        // });
    }
}