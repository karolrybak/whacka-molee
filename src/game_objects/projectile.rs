// src/projectile.rs
// v0.0.15

use box2d_rs::b2_body::{B2body, B2bodyDef, B2bodyType};
use box2d_rs::b2_fixture::{B2fixture, B2fixtureDef};
use box2d_rs::b2_math::B2vec2;
use box2d_rs::b2_shape::B2shapeDynTrait;
use box2d_rs::b2_world::B2world;
use box2d_rs::shapes::b2_circle_shape::B2circleShape as B2CircleShape;
use macroquad::prelude::{
    Camera2D, Color, MouseButton, RED, draw_circle, is_mouse_button_pressed, mouse_position, vec2,
}; // Dodano Camera2D, vec2 etc.
use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::physics::{BodyPtr, NoUserData, WorldPtr};

fn draw_projectile_circle_shape(
    body_pos: B2vec2,
    shape: &B2CircleShape,
    _scale: f32,
    color: Color,
) {
    let center_world = body_pos + shape.m_p;
    let radius_world = shape.base.m_radius;
    draw_circle(center_world.x, center_world.y, radius_world, color);
}

pub fn create_projectile(
    world: WorldPtr<NoUserData>,
    position: B2vec2,
    velocity: B2vec2,
    radius: f32,
) -> BodyPtr<NoUserData> {
    let mut body_def = B2bodyDef::default();
    body_def.body_type = B2bodyType::B2DynamicBody;
    body_def.position = position;
    body_def.bullet = true; // Pociski powinny być "bullet", aby lepiej wykrywać szybkie kolizje
    let projectile_body = B2world::<NoUserData>::create_body(world.clone(), &body_def);

    let mut circle_shape = B2CircleShape::default();
    circle_shape.base.m_radius = radius;
    let shape_def_ptr: Rc<RefCell<dyn B2shapeDynTrait>> = Rc::new(RefCell::new(circle_shape));

    let mut fd = B2fixtureDef::default();
    fd.shape = Some(shape_def_ptr);
    fd.density = 0.1; // Lekkie pociski
    fd.restitution = 0.1; // Mało sprężyste
    // fd.filter.group_index = -1; // Opcjonalnie, jeśli chcemy, aby pociski nie kolidowały z graczem
    B2body::create_fixture(projectile_body.clone(), &fd);

    projectile_body.borrow_mut().set_linear_velocity(velocity);

    projectile_body
}

pub fn draw_projectiles(projectiles: &[BodyPtr<NoUserData>], scale: f32) {
    for projectile_handle in projectiles {
        let projectile_borrow: Ref<B2body<NoUserData>> = projectile_handle.borrow();
        let projectile_pos = projectile_borrow.get_position();
        let fixture_list_proj = projectile_borrow.get_fixture_list();

        if let Some(fixture_rc) = fixture_list_proj.front() {
            let fixture_borrow: Ref<B2fixture<NoUserData>> = fixture_rc.borrow();
            let shape_rc: Rc<dyn B2shapeDynTrait> = fixture_borrow.get_shape();
            if let Some(circle_shape) = (*shape_rc).as_circle() {
                draw_projectile_circle_shape(projectile_pos, circle_shape, scale, RED);
            }
        }
    }
}

pub fn handle_projectile_shooting(
    world: WorldPtr<NoUserData>,
    projectiles_list: &mut Vec<BodyPtr<NoUserData>>,
    player_body: BodyPtr<NoUserData>,
    game_camera: &Camera2D, // Kamera potrzebna do konwersji pozycji myszy
    projectile_speed: f32,
    player_half_width: f32, // Wymiary gracza do offsetu pozycji startowej pocisku
    player_half_height: f32,
    projectile_radius: f32,
) {
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mouse_x_screen, mouse_y_screen) = mouse_position();
        let world_mouse_pos_macroquad =
            game_camera.screen_to_world(vec2(mouse_x_screen, mouse_y_screen));
        let world_mouse_pos_box2d = B2vec2 {
            x: world_mouse_pos_macroquad.x,
            y: world_mouse_pos_macroquad.y,
        };

        let player_pos_val = player_body.borrow().get_position();
        let mut aim_direction = world_mouse_pos_box2d - player_pos_val;

        if aim_direction.length_squared() > 1e-6 {
            // Unikamy dzielenia przez zero
            aim_direction.normalize();
            let shoot_velocity = B2vec2 {
                x: aim_direction.x * projectile_speed,
                y: aim_direction.y * projectile_speed,
            };
            // Offset, aby pocisk startował z "krawędzi" gracza + promień pocisku
            let projectile_start_offset = B2vec2 {
                x: aim_direction.x * (player_half_width + projectile_radius + 0.1),
                y: aim_direction.y * (player_half_height + projectile_radius + 0.1), // Używamy wysokości jeśli gracz jest wyższy niż szerszy
            };
            let projectile_start_pos = player_pos_val + projectile_start_offset;

            let new_projectile = create_projectile(
                world.clone(),
                projectile_start_pos,
                shoot_velocity,
                projectile_radius,
            );
            projectiles_list.push(new_projectile);
        }
    }
}
// src/projectile.rs v0.0.15
