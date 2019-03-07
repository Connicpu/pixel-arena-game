use super::ChunkMap;

use serde::de::{Deserializer, SeqAccess, Visitor};
use serde::ser::{SerializeSeq, Serializer};

pub fn serialize<S>(chunks: &ChunkMap, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(chunks.len()))?;
    for pair in chunks.iter() {
        seq.serialize_element(&pair)?;
    }
    seq.end()
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<ChunkMap, D::Error>
where
    D: Deserializer<'de>,
{
    struct CMVisitor;
    impl<'de> Visitor<'de> for CMVisitor {
        type Value = ChunkMap;

        fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(fmt, "a sequence of (Point2i, Chunk) pairs")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<ChunkMap, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut map = ChunkMap::new();
            while let Some((point, chunk)) = seq.next_element()? {
                map.insert(point, chunk);
            }
            Ok(map)
        }
    }
    deserializer.deserialize_seq(CMVisitor)
}
