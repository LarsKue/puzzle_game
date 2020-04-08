
use crate::{
    managers::{
        asset_manager::{
            static_texture::{
                StaticTexture,
            },
        },
    },
    math::{
        vec2::{
            Vec2,
        },
    },
};

pub struct UnitData {
    pub position: Vec2,
    pub velocity: Vec2,
    pub texture: StaticTexture,
}
