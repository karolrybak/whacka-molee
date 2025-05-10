// src/player.rs
// v0.0.13

use box2d_rs::b2_body::{B2body, B2bodyDef, B2bodyType};
use box2d_rs::b2_fixture::B2fixture;
use box2d_rs::b2_fixture::B2fixtureDef;
use box2d_rs::b2_math::{B2Rot, B2vec2};
use box2d_rs::b2_shape::B2shapeDynTrait;
use box2d_rs::b2_world::B2world;
use box2d_rs::shapes::b2_polygon_shape::B2polygonShape;
use macroquad::prelude::{Color, KeyCode, draw_rectangle, is_key_down, is_key_pressed};
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::physics::{BodyPtr, NoUserData, WorldPtr};

pub fn create_character(
    world: WorldPtr<NoUserData>,
    position: B2vec2,
    half_width: f32,
    half_height: f32,
) -> BodyPtr<NoUserData> {
    let mut body_def = B2bodyDef::default();
    body_def.body_type = B2bodyType::B2DynamicBody;
    body_def.position = position;
    body_def.fixed_rotation = true;
    let character_body = B2world::<NoUserData>::create_body(world.clone(), &body_def);

    let mut box_shape = B2polygonShape::default();
    box_shape.set_as_box(half_width, half_height);
    let shape_def_ptr: Rc<RefCell<dyn B2shapeDynTrait>> = Rc::new(RefCell::new(box_shape));

    let mut fd = B2fixtureDef::default();
    fd.shape = Some(shape_def_ptr);
    fd.density = 1.0;
    fd.friction = 0.5;
    fd.restitution = 0.0;
    B2body::create_fixture(character_body.clone(), &fd);

    character_body
}

// Ta funkcja będzie teraz obsługiwać tylko normalny ruch
pub fn handle_normal_player_input_and_movement(
    character_body: BodyPtr<NoUserData>,
    character_speed: f32,
    jump_force: f32,
    on_ground: bool,
) {
    let mut character_borrow_mut: RefMut<B2body<NoUserData>> = character_body.borrow_mut();
    let mut character_vel = character_borrow_mut.get_linear_velocity();

    if is_key_down(KeyCode::Left) {
        character_vel.x = -character_speed;
    } else if is_key_down(KeyCode::Right) {
        character_vel.x = character_speed;
    } else {
        character_vel.x *= 0.9; // Stopniowe wytracanie prędkości poziomej
    }

    if is_key_pressed(KeyCode::Space) && on_ground {
        let jump_impulse = B2vec2 {
            x: 0.0,
            y: jump_force,
        };
        character_borrow_mut.apply_linear_impulse_to_center(jump_impulse, true);
    }
    character_borrow_mut.set_linear_velocity(character_vel);
}

// Nowa funkcja do obsługi God Mode
pub fn handle_god_mode_movement(character_body: BodyPtr<NoUserData>, fly_speed: f32) {
    let mut char_body_mut: RefMut<B2body<NoUserData>> = character_body.borrow_mut();
    let mut god_vel = B2vec2::zero();
    if is_key_down(KeyCode::Left) {
        god_vel.x = -fly_speed;
    }
    if is_key_down(KeyCode::Right) {
        god_vel.x = fly_speed;
    }
    if is_key_down(KeyCode::Up) {
        god_vel.y = fly_speed;
    }
    if is_key_down(KeyCode::Down) {
        god_vel.y = -fly_speed;
    }

    // Jeśli żadna strzałka nie jest wciśnięta, prędkość powinna być 0
    if !is_key_down(KeyCode::Left) && !is_key_down(KeyCode::Right) {
        god_vel.x = 0.0;
    }
    if !is_key_down(KeyCode::Up) && !is_key_down(KeyCode::Down) {
        god_vel.y = 0.0;
    }
    char_body_mut.set_linear_velocity(god_vel);
}

pub fn set_god_mode_active(character_body: BodyPtr<NoUserData>, is_active: bool) {
    let mut char_body_mut: RefMut<B2body<NoUserData>> = character_body.borrow_mut();
    if is_active {
        char_body_mut.set_gravity_scale(0.0);
    } else {
        char_body_mut.set_gravity_scale(1.0);
    }
}

pub fn draw_character_shape(
    body_pos: B2vec2,
    body_angle: f32,
    shape: &B2polygonShape,
    _scale: f32,
    color: Color,
) {
    let vertex_count = shape.m_count;
    if vertex_count == 0 {
        return;
    }

    let body_rot = B2Rot::new(body_angle); // Potrzebne jeśli fixed_rotation = false

    // Zakładając, że kształt to prostokąt wycentrowany w body_pos i fixed_rotation=true
    if vertex_count == 4 && body_angle == 0.0 {
        // Zakładamy, że set_as_box tworzy wierzchołki w kolejności umożliwiającej obliczenie hw, hh
        // Np. (-hw, -hh), (hw, -hh), (hw, hh), (-hw, hh)
        // To jest jednak zależne od implementacji set_as_box.
        // Bezpieczniej jest, jeśli create_character przechowuje half_width, half_height.
        // Ale na razie spróbujmy wywnioskować z wierzchołków:
        let mut min_x = shape.m_vertices[0].x;
        let mut max_x = shape.m_vertices[0].x;
        let mut min_y = shape.m_vertices[0].y;
        let mut max_y = shape.m_vertices[0].y;

        for i in 1..vertex_count {
            min_x = min_x.min(shape.m_vertices[i].x);
            max_x = max_x.max(shape.m_vertices[i].x);
            min_y = min_y.min(shape.m_vertices[i].y);
            max_y = max_y.max(shape.m_vertices[i].y);
        }
        let half_width = (max_x - min_x) / 2.0;
        let half_height = (max_y - min_y) / 2.0;

        // body_pos to środek. Macroquad rysuje od lewego górnego.
        // Oś Y Box2D jest w górę, Macroquad w dół (ale kamera to odwraca).
        // Po transformacji kamery, rysujemy w koordynatach świata Box2D.
        draw_rectangle(
            body_pos.x - half_width,
            body_pos.y - half_height, // Lewy DOLNY róg w Box2D
            half_width * 2.0,
            half_height * 2.0,
            color,
        );
    } else {
        // Fallback dla innych wielokątów lub obróconych
        let mut world_vertices_mq: Vec<macroquad::prelude::Vec2> = Vec::with_capacity(vertex_count);
        for i in 0..vertex_count {
            let local_vertex = shape.m_vertices[i];
            let world_vertex =
                box2d_rs::b2_math::b2_mul_rot_by_vec2(body_rot, local_vertex) + body_pos;
            world_vertices_mq.push(macroquad::prelude::vec2(world_vertex.x, world_vertex.y));
        }
        if world_vertices_mq.len() >= 3 {
            // Prosta triangulacja wachlarzowa
            let p0_mq = world_vertices_mq[0];
            for i in 1..(world_vertices_mq.len() - 1) {
                let p1_mq = world_vertices_mq[i];
                let p2_mq = world_vertices_mq[i + 1];
                macroquad::prelude::draw_triangle(p0_mq, p1_mq, p2_mq, color);
            }
        }
    }
}

pub fn draw_character(character_body: BodyPtr<NoUserData>, scale: f32, color: Color) {
    let character_body_ref: Ref<B2body<NoUserData>> = character_body.borrow();
    let character_pos_val = character_body_ref.get_position();
    let character_angle_val = character_body_ref.get_angle();
    let fixture_list_char = character_body_ref.get_fixture_list();

    if let Some(fixture_rc) = fixture_list_char.front() {
        let fixture_borrow: Ref<B2fixture<NoUserData>> = fixture_rc.borrow();
        let shape_rc: Rc<dyn B2shapeDynTrait> = fixture_borrow.get_shape();
        if let Some(polygon_shape) = (*shape_rc).as_polygon() {
            draw_character_shape(
                character_pos_val,
                character_angle_val,
                polygon_shape,
                scale,
                color,
            );
        }
    }
}
// src/player.rs v0.0.13
