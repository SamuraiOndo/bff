use bff_derive::serialize_bits;
use bilge::prelude::{bitsize, u1, u13, Bitsized, DebugBits, Number};
use binrw::BinRead;
use serde::ser::SerializeStruct;
use serde::Serialize;

use crate::class::trivial_class::TrivialClass;

#[serialize_bits]
#[bitsize(16)]
#[derive(DebugBits, BinRead)]
struct SoundFlags {
    paused: u1,
    looping: u1,
    stereo: u1,
    padding: u13,
}

#[derive(Debug, BinRead, Serialize)]
#[br(import(_link_header: &()))]
pub struct SoundBodyV1_291_03_06PC {
    sample_rate: u32,
    data_size: u32,
    flags: SoundFlags,
    #[br(count = data_size / 2)]
    #[serde(skip_serializing)]
    data: Vec<i16>,
}

impl SoundBodyV1_291_03_06PC {
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    pub fn data(&self) -> &Vec<i16> {
        &self.data
    }
}

pub type SoundV1_291_03_06PC = TrivialClass<(), SoundBodyV1_291_03_06PC>;
