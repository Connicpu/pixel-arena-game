use self::source::Source;

use failure::Fallible;

pub mod map;
pub mod source;
pub mod tileset;

pub mod raw;

pub fn load_tmx(source: Source) -> Fallible<map::Map> {
    let raw_map = raw::ParseContext::parse(source, "map", raw::Map::parse_tag)?;
    let map = map::Map::from_raw(&raw_map.data)?;
    Ok(map)
}

pub fn save_jsonmap(writer: impl std::io::Write, map: &map::Map) -> Fallible<()> {
    serde_json::to_writer(writer, map)?;
    Ok(())
}

pub fn save_binmap(writer: impl std::io::Write, map: &map::Map) -> Fallible<()> {
    let compression = flate2::Compression::best();
    let writer = flate2::write::GzEncoder::new(writer, compression);
    bincode::serialize_into(writer, map)?;
    Ok(())
}

pub fn load_jsonmap(reader: impl std::io::Read) -> Fallible<map::Map> {
    let map = serde_json::from_reader(reader)?;
    Ok(map)
}

pub fn load_binmap(reader: impl std::io::Read) -> Fallible<map::Map> {
    let reader = flate2::read::GzDecoder::new(reader);
    let map = bincode::deserialize_from(reader)?;
    Ok(map)
}
