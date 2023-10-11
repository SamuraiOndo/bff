use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::math::{RGB, RGBA};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
struct ObjectDatas {
    color: RGBA,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, ReferencedNames)]
#[br(import(_link_header: &()))]
pub struct LightDataBodyV1_291_03_06PC {
    object_datas: ObjectDatas,
    color: RGB,
    padding: [u8; 12],
    flag: u32,
    ambient: RGB,
}

pub type LightDataV1_291_03_06PC = TrivialClass<(), LightDataBodyV1_291_03_06PC>;
