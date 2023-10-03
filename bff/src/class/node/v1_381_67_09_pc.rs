use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::link_header::ResourceObjectLinkHeader;
use crate::math::{Mat4f, Quat, Rect, Sphere, Vec3f, RGBA};
use crate::names::Name;

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
#[br(import(_link_header: &ResourceObjectLinkHeader))]
pub struct NodeBodyV1_381_67_09PC {
    parent_name: Name,
    head_child_name: Name,
    prev_sibling: Name,
    next_sibling: Name,
    lod_or_particles_name: Name,
    lod_data_or_particles_data_name: Name,
    user_define_name: Name,
    light_data_name: Name,
    bitmap_name: Name,
    unused_name2: Name,
    rotation: Quat,
    translation: Vec3f,
    flags: u32,
    rotation2: Quat,
    scale: f32,
    scale2: f32,
    reciprocal_scale2: f32,
    unknown10: f32,
    color: RGBA,
    sphere: Sphere,
    display_seads_rect: Rect,
    collide_seads_rect: Rect,
    negative_four: i16,
    world_transform_mat4: Mat4f,
}

pub type NodeV1_381_67_09PC = TrivialClass<ResourceObjectLinkHeader, NodeBodyV1_381_67_09PC>;
