use crate::tiled::raw::tileset::Tileset;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use failure::{err_msg, Fallible};
use xml::attribute as xa;

pub struct ParseContext<'a> {
    pub reader: xml::EventReader<&'a [u8]>,
    pub source: Source,
    pub tilesets: &'a mut HashMap<Source, Arc<Tileset>>,
    pub warnings: &'a mut Vec<String>,
    pub config: &'a xml::ParserConfig,
}

impl<'a> ParseContext<'a> {
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
        };

        loop {
            use xml::reader::XmlEvent;
            match ctx.reader.next()? {
                XmlEvent::StartDocument { .. } => continue,
                XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } if name.local_name == root_tag => return func(&mut ctx, attributes),
                _ => return Err(err_msg(format!("bad tileset file {}", ctx.source))),
            };
        }
    }

    pub fn warning(&mut self, msg: impl Into<String>) {
        self.warnings.push(msg.into());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Source {
    File(PathBuf),
}

impl Source {
    pub fn new_file(file: impl AsRef<Path>) -> Source {
        Source::File(dunce::canonicalize(file).unwrap())//.canonicalize().unwrap())
    }

    pub fn relative(&self, rel: &str) -> Source {
        match self {
            Source::File(file) => {
                Source::File(dunce::canonicalize(file.parent().unwrap().join(rel)).unwrap())//.canonicalize().unwrap())
            }
        }
    }

    pub fn read_all(&self) -> Fallible<Vec<u8>> {
        match self {
            Source::File(path) => Ok(std::fs::read(path)?),
        }
    }
}

impl std::fmt::Display for Source {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Source::File(path) => std::fmt::Display::fmt(&path.to_string_lossy(), fmt),
        }
    }
}
