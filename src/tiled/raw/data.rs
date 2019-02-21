use crate::tiled::raw::TileId;

#[derive(Debug)]
pub struct Data {
    pub chunks: Vec<Chunk>,
}

#[derive(Debug)]
pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub data: Vec<TileId>,
}
