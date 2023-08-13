use derive_more::{From, IsVariant};
use serde::Serialize;

use self::animation::Animation;
use self::binary::Binary;
use self::bitmap::Bitmap;
use self::camera::Camera;
use self::camera_zone::CameraZone;
use self::collision_vol::CollisionVol;
use self::fonts::Fonts;
use self::game_obj::GameObj;
use self::gen_world::GenWorld;
use self::gw_road::GwRoad;
use self::light::Light;
use self::light_data::LightData;
use self::lod::Lod;
use self::lod_data::LodData;
use self::material::Material;
use self::material_anim::MaterialAnim;
use self::material_obj::MaterialObj;
use self::mesh::Mesh;
use self::mesh_data::MeshData;
use self::node::Node;
use self::omni::Omni;
use self::particles::Particles;
use self::particles_data::ParticlesData;
use self::rot_shape::RotShape;
use self::rot_shape_data::RotShapeData;
use self::rtc::Rtc;
use self::skel::Skel;
use self::skin::Skin;
use self::sound::Sound;
use self::spline::Spline;
use self::spline_graph::SplineGraph;
use self::surface::Surface;
use self::surface_datas::SurfaceDatas;
use self::user_define::UserDefine;
use self::warp::Warp;
use self::world::World;
use self::world_ref::WorldRef;

use crate::error::{Error, UnimplementedClassError, UnknownClassError};
use crate::object::Object;
use crate::platforms::Platform;
use crate::traits::{NamedClass, TryFromVersionPlatform};
use crate::versions::Version;
use crate::BffResult;

pub mod animation;
pub mod binary;
pub mod bitmap;
pub mod camera;
pub mod camera_zone;
pub mod collision_vol;
pub mod fonts;
pub mod game_obj;
pub mod gen_world;
pub mod gw_road;
pub mod light;
pub mod light_data;
pub mod lod;
pub mod lod_data;
pub mod material;
pub mod material_anim;
pub mod material_obj;
pub mod mesh;
pub mod mesh_data;
pub mod node;
pub mod omni;
pub mod particles;
pub mod particles_data;
pub mod rot_shape;
pub mod rot_shape_data;
pub mod rtc;
pub mod skel;
pub mod skin;
pub mod sound;
pub mod spline;
pub mod spline_graph;
pub mod surface;
pub mod surface_datas;
pub mod trivial_class;
pub mod user_define;
pub mod warp;
pub mod world;
pub mod world_ref;

macro_rules! classes_enum {
    ($($i:ident),* $(,)?) => {
        #[derive(Serialize, Debug, From, IsVariant)]
        #[serde(untagged)]
        pub enum Class {
            $($i(Box<$i>),)*
        }
    };
}

macro_rules! objects_to_classes {
    ($($i:ident),* $(,)?) => {
        impl TryFromVersionPlatform<&Object> for Class {
            type Error = Error;

            fn try_from_version_platform(object: &Object, version: Version, platform: Platform) -> BffResult<Class> {
                match object.class_name() {
                    $(<$i as NamedClass>::NAME => Ok(Box::new(<$i as TryFromVersionPlatform<&Object>>::try_from_version_platform(object, version, platform)?).into()),)*
                    _ => Err(UnimplementedClassError::new(object.name(), object.class_name(), version, platform).into())
                }
            }
        }
    };
}

macro_rules! classes_to_names {
    ($($i:ident),* $(,)?) => {
        impl From<Class> for &'static str {
            fn from(class: Class) -> &'static str {
                match class {
                    $(Class::$i(_) => <$i as NamedClass>::REAL_NAME,)*
                }
            }
        }
    };
}

macro_rules! hashes_to_names {
    ($($i:ident),* $(,)?) => {
        impl Object {
            pub fn real_class_name(&self) -> BffResult<&'static str> {
                match self.class_name() {
                    $(<$i as NamedClass>::NAME => Ok(<$i as NamedClass>::REAL_NAME),)*
                    _ => Err(UnknownClassError::new(self.class_name()).into())
                }
            }
        }
    };
}

macro_rules! classes {
    ($($i:ident),* $(,)?) => {
        classes_enum!($($i),*);
        objects_to_classes!($($i),*);
        classes_to_names!($($i),*);
        hashes_to_names!($($i),*);
    };
}

classes! {
    Animation,
    Binary,
    Bitmap,
    Camera,
    CameraZone,
    CollisionVol,
    Fonts,
    GameObj,
    GenWorld,
    GwRoad,
    Light,
    LightData,
    Lod,
    LodData,
    Material,
    MaterialAnim,
    MaterialObj,
    Mesh,
    MeshData,
    Node,
    Omni,
    Particles,
    ParticlesData,
    RotShape,
    RotShapeData,
    Rtc,
    Skel,
    Skin,
    Sound,
    Spline,
    SplineGraph,
    Surface,
    SurfaceDatas,
    UserDefine,
    Warp,
    World,
    WorldRef,
}
