use crate::tiled::raw::context::ParseContext;
use crate::tiled::raw::GlobalTileId;

use failure::Fallible;
use xml::attribute as xa;

#[derive(Debug)]
pub enum Data {
    Plain(Vec<Vec<GlobalTileId>>),
    Chunked(Vec<Chunk>),
}

impl Data {
    pub fn parse_tag(context: &mut ParseContext, attrs: &[xa::OwnedAttribute]) -> Fallible<Data> {
        parse_tag! {
            context; attrs;
            <data ?encoding="encoding"(String) ?compression="compression"(String)>
        }

        let is_base64 = encoding.map(|enc| enc == "base64").unwrap_or(false);
        let compression = compression.as_ref().map(|s| s.as_str());

        parse_tag! {
            context; attrs;
            <data>
                content,
                <chunk> => |c, a| Chunk::parse_tag(c, a, is_base64, compression),
            </data>
        }

        if !chunk.is_empty() {
            Ok(Data::Chunked(chunk))
        } else {
            // CSV
            let data: Fallible<_> = content
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|s| s.split(',').map(|s| Ok(GlobalTileId(s.trim().parse()?))).collect())
                .collect();

            Ok(Data::Plain(data?))
        }
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub data: Vec<GlobalTileId>,
}

impl Chunk {
    pub fn parse_tag(
        context: &mut ParseContext,
        attrs: &[xa::OwnedAttribute],
        is_base64: bool,
        compression: Option<&str>,
    ) -> Fallible<Chunk> {
        parse_tag! {
            context; attrs;
            <chunk x="x"(i32) y="y"(i32) width="width"(i32) height="height"(i32)>
                content,
            </chunk>
        }

        let data = if is_base64 {
            use byteorder::{ByteOrder, LE};
            let raw_data = base64::decode(content.trim())?;
            let byte_data = match compression {
                None => raw_data,

                Some("gzip") => {
                    use flate2::bufread::GzDecoder;
                    use std::io::Read;
                    let mut decoder = GzDecoder::new(raw_data.as_slice());
                    let mut data = Vec::new();
                    decoder.read_to_end(&mut data)?;
                    data
                }

                Some("zlib") => {
                    use flate2::bufread::ZlibDecoder;
                    use std::io::Read;
                    let mut decoder = ZlibDecoder::new(raw_data.as_slice());
                    let mut data = Vec::new();
                    decoder.read_to_end(&mut data)?;
                    data
                }

                Some(fmt) => {
                    return Err(failure::err_msg(format!(
                        "Unknown tile compression format '{}'",
                        fmt
                    )));
                }
            };

            byte_data
                .chunks(4)
                .map(LE::read_u32)
                .map(GlobalTileId)
                .collect()
        } else {
            let data: Fallible<_> = content
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .flat_map(|s| s.split(','))
                .map(|s| Ok(GlobalTileId(s.parse()?)))
                .collect();

            data?
        };

        Ok(Chunk {
            x,
            y,
            width,
            height,
            data,
        })
    }
}
