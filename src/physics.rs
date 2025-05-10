use box2d_rs::b2_body::{B2body, B2bodyType};
use box2d_rs::b2_collision::B2worldManifold;
use box2d_rs::b2_contact::B2contactDynTrait;
use box2d_rs::b2_fixture::B2fixture;
use box2d_rs::b2_math::B2vec2;
use box2d_rs::b2_world::B2world;
use box2d_rs::b2_world_callbacks::B2contactListener;
use box2d_rs::b2rs_common::UserDataType;

use std::cell::RefCell;
use std::rc::Rc;

pub type BodyPtr<D> = Rc<RefCell<B2body<D>>>;
pub type WorldPtr<D> = Rc<RefCell<B2world<D>>>;
pub type FixturePtr<D> = Rc<RefCell<B2fixture<D>>>;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NoUserData;

impl UserDataType for NoUserData {
    type Fixture = ();
    type Body = ();
    type Joint = ();
}
