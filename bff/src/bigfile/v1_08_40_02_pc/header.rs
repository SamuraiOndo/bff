use binrw::binrw;

use crate::bigfile::v1_06_63_02_pc::header::BlockDescription;
use crate::bigfile::versions::VersionTriple;

#[binrw]
#[derive(Debug)]
pub struct Header {
    #[br(temp)]
    #[bw(calc = block_descriptions.len() as u32)]
    block_count: u32,
    pub block_working_buffer_capacity_even: u32,
    pub block_working_buffer_capacity_odd: u32,
    pub total_padded_block_size: u32,
    pub version_triple: VersionTriple,
    #[br(count = block_count)]
    pub block_descriptions: Vec<BlockDescription>,
    #[br(ignore)]
    pub tag: Option<Vec<u8>>,
    #[br(temp)]
    #[bw(ignore)]
    #[brw(align_after = 2048)]
    _align: (),
}
