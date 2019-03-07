use crate::tiled::raw::tileset::Tileset;
use crate::tiled::source::Source;

use std::collections::HashMap;
use std::sync::Arc;

use failure::{err_msg, Fallible};
use xml::attribute as xa;

pub struct ParseContext<'a> {
    pub reader: xml::EventReader<&'a [u8]>,
    pub source: Source,
    pub tilesets: &'a mut HashMap<Source, Arc<Tileset>>,
    pub warnings: &'a mut Vec<String>,
    pub config: &'a xml::ParserConfig,
    pub parseorder: i32,
}

pub struct ParseResult<T> {
    pub data: T,
    pub tilesets: HashMap<Source, Arc<Tileset>>,
    pub warnings: Vec<String>,
}

impl<'a> ParseContext<'a> {
    pub fn parse<R>(
        source: Source,
        root_tag: &str,
        func: impl FnOnce(&mut ParseContext, &[xa::OwnedAttribute]) -> Fallible<R>,
    ) -> Fallible<ParseResult<R>> {
        let data = source.read_all()?;
        let mut tilesets = HashMap::new();
        let mut warnings = Vec::new();
        let mut config = xml::ParserConfig::default();
        config.whitespace_to_characters = true;
        config.cdata_to_characters = true;

        let data = {
            let mut ctx = ParseContext {
                reader: xml::EventReader::new(&data),
                source: source,
                tilesets: &mut tilesets,
                warnings: &mut warnings,
                config: &config,
                parseorder: 0,
            };
            ctx.parse_root(root_tag, func)?
        };

        Ok(ParseResult {
            data,
            tilesets,
            warnings,
        })
    }

    pub fn parse_root<R>(
        &mut self,
        root_tag: &str,
        func: impl FnOnce(&mut ParseContext, &[xa::OwnedAttribute]) -> Fallible<R>,
    ) -> Fallible<R> {
        loop {
            use xml::reader::XmlEvent;
            match self.reader.next()? {
                XmlEvent::StartDocument { .. } => continue,
                XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } if name.local_name == root_tag => return func(self, attributes),
                _ => return Err(err_msg(format!("bad tiled file {}", self.source))),
            };
        }
    }

    pub fn subcontext<R>(
        &mut self,
        data: &[u8],
        source: Source,
        root_tag: &str,
        func: impl FnOnce(&mut ParseContext, &[xa::OwnedAttribute]) -> Fallible<R>,
    ) -> Fallible<R> {
        let mut ctx = ParseContext {
            reader: xml::EventReader::new_with_config(data, self.config.clone()),
            source,
            tilesets: &mut *self.tilesets,
            warnings: &mut *self.warnings,
            config: self.config,
            parseorder: 0,
        };

        ctx.parse_root(root_tag, func)
    }

    pub fn warning(&mut self, msg: impl Into<String>) {
        self.warnings.push(msg.into());
    }

    pub fn parseorder(&mut self) -> ParseOrder {
        self.parseorder += 1;
        ParseOrder(self.parseorder)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParseOrder(i32);
