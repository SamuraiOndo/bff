use bff_derive::serialize_bits;
use bilge::prelude::{bitsize, u12, u20, u4, Bitsized, DebugBits, Number};
use binrw::BinRead;
use serde::ser::SerializeStruct;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;
use crate::name::Name;

#[serialize_bits]
#[bitsize(32)]
#[derive(BinRead, DebugBits)]
struct LookupDescription {
    horizon: u12,
    altitudes_index: u20,
}

#[serialize_bits]
#[bitsize(8)]
#[derive(BinRead, DebugBits)]
struct AltitudePack {
    odd: u4,
    even: u4,
}

#[derive(BinRead, Debug, Serialize)]
struct AltitudesPacked {
    altitudes: [AltitudePack; 8],
}

#[derive(BinRead, Debug, Serialize)]
struct AltitudesUnpacked {
    altitudes: [u8; 16],
}

impl AltitudesPacked {
    const SIZE: u32 = 8;
}

#[derive(BinRead, Debug, Serialize)]
struct Internal {
    width: u32,
    height: u32,
    two: f32,
    negative_one: i32,
    denominator: f32,
    altitudes_packed_size: u32,
    altitudes_total_size: u32,
    #[br(count = altitudes_packed_size)]
    altitudes_packed: Vec<AltitudesPacked>,
    #[br(count = ((altitudes_total_size - 1) * 4 - AltitudesPacked::SIZE * altitudes_packed_size) / 16)]
    altitudes_unpacked: Vec<AltitudesUnpacked>,
    #[br(count = (width / 4) * (width / 4))]
    lookup: Vec<LookupDescription>,
}

#[derive(BinRead, Debug, Serialize)]
pub struct LinkHeader {
    link_name: Name,
}

#[derive(BinRead, Debug, Serialize)]
#[br(import(_link_header: &LinkHeader))]
pub struct BinaryBodyV1_381_67_09PC {
    data_size: u32,
    data: Internal,
}

pub type BinaryV1_381_67_09PC = TrivialClass<LinkHeader, BinaryBodyV1_381_67_09PC>;
