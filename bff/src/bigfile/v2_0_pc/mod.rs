use std::cmp::max;
use std::collections::HashMap;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use binrw::{BinRead, BinResult, BinWrite, Endian, args, binread, parser};

use super::v1_22_pc::Resource as Resource12;
use crate::BffResult;
use crate::bigfile::BigFile;
use crate::bigfile::manifest::Manifest;
use crate::bigfile::platforms::Platform;
use crate::bigfile::versions::Version;
use crate::helpers::{DynArray, calculated_padded, read_align_to, write_align_to};
use crate::lz::{lzo_compress, lzo_decompress};
use crate::names::NameType;
use crate::names::NameType::Ubisoft64;
use crate::traits::BigFileIo;

type Resource = Resource12<20>;

#[derive(Debug)]
pub struct Block {
    pub compressed: bool,
    // TODO: Always big endian 0xDEADBEEF
    pub checksum: Option<i32>,
    pub resources: Vec<Resource>,
}

impl BinWrite for Block {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        self.resources.write_options(writer, endian, ())?;
        Ok(())
    }
}

#[parser(reader, endian)]
fn parse_blocks(decompressed_block_size: u32, block_sizes: &[u32]) -> BinResult<Vec<Block>> {
    let mut blocks = Vec::new();

    for block_size in block_sizes {
        let block_start = reader.stream_position()?;

        let checksum = Some(i32::read_options(reader, endian, ())?);
        let resource_count = u32::read_options(reader, endian, ())?;

        if *block_size != decompressed_block_size {
            let mut compressed = vec![0; (*block_size - 8) as usize];
            reader.read_exact(&mut compressed)?;
            let decompressed =
                lzo_decompress(&compressed, decompressed_block_size as usize).unwrap();
            let mut decompressed = Cursor::new(decompressed);
            blocks.push(Block {
                compressed: true,
                checksum,
                resources: Vec::<Resource>::read_options(
                    &mut decompressed,
                    endian,
                    args! { count: resource_count as usize },
                )?,
            });
            read_align_to(reader, 2048)?;
        } else {
            blocks.push(Block {
                compressed: false,
                checksum,
                resources: Vec::<Resource>::read_options(
                    reader,
                    endian,
                    args! { count: resource_count as usize },
                )?,
            });
            reader.seek(SeekFrom::Start(block_start + *block_size as u64))?;
        }
    }

    Ok(blocks)
}

#[derive(Debug, BinRead, BinWrite, Copy, Clone)]
#[brw(repr = u32)]
pub enum CompressionType {
    None,
    Lzo,
}

#[derive(Debug, BinRead, BinWrite)]
pub struct Header {
    pub decompressed_block_size: u32,
    pub compression_type: CompressionType,
    pub block_sizes: DynArray<u32>,
}

#[binread]
#[derive(Debug)]
#[br(import(version: Version, platform: Platform))]
pub struct BigFileV2_0PC {
    #[br(calc = version)]
    version: Version,
    #[br(calc = platform)]
    platform: Platform,
    #[br(temp)]
    header: Header,
    #[br(align_before = 2048, parse_with = parse_blocks, args(header.decompressed_block_size, &header.block_sizes.inner))]
    blocks: Vec<Block>,
}

impl From<BigFileV2_0PC> for BigFile {
    fn from(bigfile: BigFileV2_0PC) -> Self {
        let mut blocks = Vec::with_capacity(bigfile.blocks.len());
        let mut resources = HashMap::new();

        for block in bigfile.blocks.into_iter() {
            let mut objects = Vec::with_capacity(block.resources.len());

            for resource in block.resources {
                objects.push(crate::bigfile::manifest::ManifestObject {
                    name: resource.name,
                    compress: None,
                });
                resources.insert(resource.name, resource.into());
            }

            blocks.push(crate::bigfile::manifest::ManifestBlock {
                offset: None,
                checksum: block.checksum,
                compressed: Some(block.compressed),
                objects,
            });
        }

        Self {
            manifest: Manifest {
                version: bigfile.version,
                version_xple: None,
                platform: bigfile.platform,
                bigfile_type: None,
                pool_manifest_unused: None,
                incredi_builder_string: None,
                blocks,
                pool: None,
            },
            objects: resources,
        }
    }
}

impl BigFileIo for BigFileV2_0PC {
    fn read<R: Read + Seek>(
        reader: &mut R,
        version: Version,
        platform: Platform,
    ) -> BffResult<BigFile> {
        let endian = platform.into();
        let bigfile = Self::read_options(reader, endian, (version, platform))?;
        Ok(bigfile.into())
    }

    fn write<W: Write + Seek>(
        bigfile: &BigFile,
        writer: &mut W,
        tag: Option<&str>,
    ) -> BffResult<()> {
        let endian: Endian = bigfile.manifest.platform.into();

        // Remember starting position for writing header
        let begin = writer.stream_position()?;

        let mut decompressed_block_size = 0;

        let mut blocks = Vec::new();

        for block in bigfile.manifest.blocks.iter() {
            let mut block_writer = Cursor::new(Vec::new());

            for resource in block.objects.iter() {
                let resource = bigfile.objects.get(&resource.name).unwrap();
                Resource::dump_resource(resource, &mut block_writer, endian)?;
            }

            let block_data = block_writer.into_inner();

            decompressed_block_size = max(decompressed_block_size, block_data.len() as u32);

            blocks.push((
                block.objects.len() as u32,
                block.checksum.unwrap_or(0),
                block.compressed.unwrap_or(false),
                block_data,
            ));
        }

        decompressed_block_size = calculated_padded(decompressed_block_size as usize, 2048) as u32;
        let mut block_sizes = Vec::new();
        let mut compression_type = CompressionType::None;

        for (resource_count, checksum, compressed, mut block_data) in blocks {
            let block_begin = writer.stream_position()?;

            checksum.write_options(writer, endian, ())?;

            resource_count.write_options(writer, endian, ())?;

            block_data.resize(decompressed_block_size as usize, 0);

            if compressed {
                compression_type = CompressionType::Lzo;
                lzo_compress(&block_data, writer)?;
            } else {
                writer.write_all(&block_data)?;
            }

            let block_end = writer.stream_position()?;

            write_align_to(writer, 2048, 0)?;

            block_sizes.push((block_end - block_begin - if compressed { 8 } else { 0 }) as u32);
        }

        // Write header at the beginning of the file and restore position
        let end = writer.stream_position()?;
        writer.seek(SeekFrom::Start(begin))?;

        let header = Header {
            decompressed_block_size,
            compression_type,
            block_sizes: block_sizes.into(),
        };
        header.write_options(writer, endian, ())?;

        if let Some(tag) = tag {
            // TODO: Make sure the tag fits
            writer.write_all(tag.as_bytes())?;
        }

        writer.seek(SeekFrom::Start(end))?;

        Ok(())
    }

    const NAME_TYPE: NameType = Ubisoft64;

    type ResourceType = Resource;
}
