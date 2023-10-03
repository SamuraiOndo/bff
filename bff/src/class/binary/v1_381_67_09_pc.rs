use bilge::prelude::*;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::link_header::ResourceObjectLinkHeader;

#[bitsize(32)]
#[derive(BinRead, DebugBits, SerializeBits, BinWrite, DeserializeBits)]
struct LookupDescription {
    horizon: u12,
    altitudes_index: u20,
}

#[bitsize(8)]
#[derive(BinRead, DebugBits, SerializeBits, BinWrite, DeserializeBits)]
struct AltitudePack {
    odd: u4,
    even: u4,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
struct AltitudesPacked {
    altitudes: [AltitudePack; 8],
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
struct AltitudesUnpacked {
    altitudes: [u8; 16],
}

impl AltitudesPacked {
    const SIZE: u32 = 8;
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
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

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
#[br(import(_link_header: &ResourceObjectLinkHeader))]
pub struct BinaryBodyV1_381_67_09PC {
    data_size: u32,
    data: Internal,
}

pub type BinaryV1_381_67_09PC = TrivialClass<ResourceObjectLinkHeader, BinaryBodyV1_381_67_09PC>;
