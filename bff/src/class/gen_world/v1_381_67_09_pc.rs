use bff_derive::serialize_bits;
use bilge::prelude::{bitsize, u1, u15, Bitsized, DebugBits, Number};
use binrw::BinRead;
use serde::ser::SerializeStruct;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::map::BffMap;
use crate::math::Vec2f;
use crate::math::Vec3f;
use crate::math::{Mat4f, Quat};
use crate::name::Name;
use crate::strings::FixedStringNull;
use crate::strings::PascalStringNull;

#[derive(BinRead, Debug, Serialize)]
struct Category {
    one: u32,
    node_name_arrays: DynArray<Name>,
}

#[derive(BinRead, Debug, Serialize)]
struct CAFlatSurface {
    zero0: u32,
    mat: Mat4f,
    a: f32,
    b: f32,
    c: f32,
    reciprocal: f32,
    vec: Vec3f,
    unknown1: f32,
    unknown3: f32,
    zero1: u32,
    zero2: u32,
    zero3: u32,
    zero4: u32,
    unknown9: i32,
    unknown4: u8,
    unknown20: u8,
    unknown21: u8,
    unknown22: u8,
    unknown23: u8,
    unknown24: u8,
    unknown2: u8,
}

#[derive(BinRead, Debug, Serialize)]
struct Unused10 {
    unused0: u32,
    unused1s: [u32; 8],
    unused2: u32,
    unused3: u32,
    unused4: u32,
}

#[derive(BinRead, Debug, Serialize)]
struct RegionEdge {
    region_vertices_index_a: u32,
    region_vertices_index_b: u32,
}

#[derive(BinRead, Debug, Serialize)]
struct Region {
    unknown: u8,
    region_edges_indices: DynArray<u32>,
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
pub struct GenWorldBodyV1_381_67_09PC {
    node_name: Name,
    user_define_name: Name,
    gw_road_name: Name,
    binary_names: DynArray<Name>,
    bitmap_names: DynArray<Name>,
    material_names: DynArray<Name>,
    equals41: u32,
    categories: BffMap<PascalStringNull, Category>,
    ca_flat_surfaces: DynArray<CAFlatSurface>,
    cancel_object_placement: DynArray<Mat4f>,
    unused10s: DynArray<Unused10>,
    region_vertices: DynArray<Vec2f>,
    region_edges: DynArray<RegionEdge>,
    regions: BffMap<FixedStringNull<31>, Region>,
}

pub type GenWorldV1_381_67_09PC = TrivialClass<LinkHeader, GenWorldBodyV1_381_67_09PC>;
