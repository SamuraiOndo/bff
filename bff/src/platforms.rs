use std::ffi::OsStr;

use binrw::Endian;

use crate::macros::platforms::platforms;
use crate::BffResult;

platforms! {
    styles: [BF, D, DB],
    platforms: [
        PC([D("DPC"), BF("BFPC"), DB("DBC")], Little),
        UWP([D("DUA")], Little),
        Maci386([D("DMC")], Little),
        MacPPC([D("DBM")], Big),
        PS2([D("DPS")], Little),
        PS3([D("DP3")], Big),
        PSP([D("DPP"), BF("BFPSP")], Little),
        Xbox([D("DXB")], Big),
        Xbox360([D("D36")], Big),
        GameCube([D("DGC")], Big),
        Wii([D("DRV"), BF("BFWii"), DB("DBR")], Big),
        Switch([D("DNX")], Little),
    ]
}

pub fn try_extension_to_endian(extension: &OsStr) -> BffResult<Endian> {
    extension.try_into().map(<Platform as Into<Endian>>::into)
}
