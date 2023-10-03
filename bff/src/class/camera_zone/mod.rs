use bff_derive::bff_class;

mod v1_06_63_02_pc;
use v1_06_63_02_pc::CameraZoneV1_06_63_02PC;

bff_class!(CameraZone {
    (Asobo(1, 6, 63, 2), PC) => CameraZoneV1_06_63_02PC,
});
