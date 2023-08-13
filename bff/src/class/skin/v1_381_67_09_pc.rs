use bff_derive::serialize_bits;
use bilge::prelude::{bitsize, u1, u15, Bitsized, DebugBits, Number};
use binrw::BinRead;
use serde::ser::SerializeStruct;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::math::{Mat4f, Quat};
use crate::name::Name;

#[derive(BinRead, Debug, Serialize)]
#[br(import(bone_name_count: u32))]
struct SkinSubsection {
    animation_node_names: [Name; 4],
    #[br(count = bone_name_count)]
    bone_names: Vec<Name>,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(bone_name_count: u32))]
struct SkinSection {
    #[br(args { inner: (bone_name_count,) })]
    skin_subsections: DynArray<SkinSubsection>,
}

#[serialize_bits]
#[bitsize(32)]
#[derive(BinRead, DebugBits)]
struct ObjectFlags {
    fl_object_init: u1,
    fl_object_max_bsphere: u1,
    fl_object_skinned: u1,
    fl_object_morphed: u1,
    fl_object_orientedbbox: u1,
    fl_object_no_seaddisplay: u1,
    fl_object_no_seadcollide: u1,
    fl_object_no_display: u1,
    fl_object_transparent: u1,
    fl_object_optimized_vertex: u1,
    fl_object_linear_mapping: u1,
    fl_object_skinned_with_one_bone: u1,
    fl_object_light_baked: u1,
    fl_object_light_baked_with_material: u1,
    fl_object_shadow_receiver: u1,
    fl_object_no_tesselate: u1,
    fl_object_last: u1,
    padding: u15,
}

#[derive(BinRead, Debug, Serialize)]
#[br(repr = u16)]
enum ObjectType {
    Points = 0,
    Surface = 1,
    Spline = 2,
    Skin = 3,
    RotShape = 4,
    Lod = 5,
    Mesh = 6,
    Camera = 7,
    SplineZone = 9,
    Occluder = 10,
    CameraZone = 11,
    Light = 12,
    HFog = 13,
    CollisionVol = 14,
    Emiter = 15,
    Omni = 16,
    Graph = 17,
    Particles = 18,
    Flare = 19,
    HField = 20,
    Tree = 21,
    GenWorld = 22,
    Road = 23,
    GenWorldSurface = 24,
    SplineGraph = 25,
    WorldRef = 26,
}

#[derive(BinRead, Debug, Serialize)]
pub struct LinkHeader {
    link_name: Name,
    data_name: Name,
    rot: Quat,
    transform: Mat4f,
    radius: f32,
    flags: ObjectFlags,
    r#type: ObjectType,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &LinkHeader))]
pub struct SkinBodyV1_381_67_09PC {
    mesh_names: DynArray<Name>,
    zeros: [u32; 4],
    one_and_a_half: f32,
    bone_name_count: u32,
    #[br(args { inner: (bone_name_count,) })]
    skin_sections: DynArray<SkinSection>,
}

pub type SkinV1_381_67_09PC = TrivialClass<LinkHeader, SkinBodyV1_381_67_09PC>;
