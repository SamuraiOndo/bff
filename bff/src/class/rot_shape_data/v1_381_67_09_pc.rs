// binrw casts all count directives to usize even when they already are.
#![allow(clippy::useless_conversion)]

use bff_derive::serialize_bits;
use bilge::prelude::{bitsize, u1, u19, Bitsized, DebugBits, Number};
use binrw::BinRead;
use serde::ser::SerializeStruct;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::dynarray::DynArray;
use crate::name::Name;

#[serialize_bits]
#[bitsize(32)]
#[derive(BinRead, DebugBits)]
struct ObjectDatasFlags {
    fl_objectdatas_hide: u1,
    fl_objectdatas_code_control: u1,
    fl_objectdatas_cloned: u1,
    fl_objectdatas_skinned: u1,
    fl_objectdatas_morphed: u1,
    fl_objectdatas_vreflect: u1,
    fl_objectdatas_hide_shadow: u1,
    fl_objectdatas_static_shadow: u1,
    fl_objectdatas_vp0_hide: u1,
    fl_objectdatas_vp1_hide: u1,
    fl_objectdatas_vp2_hide: u1,
    fl_objectdatas_vp3_hide: u1,
    fl_objectdatas_last: u1,
    padding: u19,
}

#[derive(BinRead, Debug, Serialize)]
pub struct LinkHeader {
    link_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &LinkHeader))]
pub struct RotShapeDataBodyV1_381_67_09PC {
    flags: ObjectDatasFlags,
    zeros: DynArray<u16>,
    #[br(count = zeros.len() * 28)]
    pad: Vec<u8>,
}

pub type RotShapeDataV1_381_67_09PC = TrivialClass<LinkHeader, RotShapeDataBodyV1_381_67_09PC>;
