use bff_derive::{bff_class, NamedClass};
use serde::Serialize;

mod v1_06_63_02_pc;
mod v1_291_03_06_pc;

use v1_06_63_02_pc::MeshDataV1_06_63_02PC;
use v1_291_03_06_pc::MeshDataV1_291_03_06PC;

bff_class!(MeshData {
    (V1_291_03_06, PC) => MeshDataV1_291_03_06PC,
    (V1_06_63_02, PC) => MeshDataV1_06_63_02PC,
});
