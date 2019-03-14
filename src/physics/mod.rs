use crate::tiled::tileset::tile::TileFlags;

pub struct PhyData;

impl wrapped2d::user_data::UserDataTypes for PhyData {
    type BodyData = Option<conniecs::Entity>;
    type JointData = u16;
    type FixtureData = TileFlags;
}

pub type MetaBody = wrapped2d::b2::MetaBody<PhyData>;
pub type World = wrapped2d::b2::World<PhyData>;
