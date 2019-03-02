use crate::tiled::raw::context::ParseContext;

use std::collections::HashMap;

use failure::err_msg;
use failure::Fallible;
use xml::attribute as xa;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Properties {
    pub properties: HashMap<String, Property>,
}

impl Properties {
    pub fn parse_tag(
        context: &mut ParseContext,
        attrs: &[xa::OwnedAttribute],
    ) -> Fallible<Properties> {
        parse_tag! {
            context; attrs;
            <properties>
                <property> => Property::parse_tag,
            </properties>
        }

        let properties = property.into_iter().collect();

        Ok(Properties { properties })
    }
}

impl std::fmt::Debug for Properties {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.properties, fmt)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Property {
    String(String),
    Int(i64),
    Float(f32),
    Bool(bool),
    Color(math2d::Color),
    File(String),
}

impl std::fmt::Debug for Property {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Property::String(value) => std::fmt::Debug::fmt(value, fmt),
            Property::Int(value) => std::fmt::Debug::fmt(value, fmt),
            Property::Float(value) => std::fmt::Debug::fmt(value, fmt),
            Property::Bool(value) => std::fmt::Debug::fmt(value, fmt),
            Property::Color(value) => std::fmt::Debug::fmt(value, fmt),
            Property::File(value) => std::fmt::Debug::fmt(value, fmt),
        }
    }
}

impl Property {
    pub fn parse_tag(
        context: &mut ParseContext,
        attrs: &[xa::OwnedAttribute],
    ) -> Fallible<(String, Property)> {
        parse_tag! {
            context; attrs;
            <property name="name"(String) value="value"(String) ?kind="type"(String) />
        };

        let kind = kind.unwrap_or_else(|| "string".into());

        Ok(match kind.as_str() {
            "string" => (name, Property::String(value)),
            "int" => (name, Property::Int(value.parse()?)),
            "float" => (name, Property::Float(value.parse()?)),
            "bool" => (name, Property::Bool(value.parse()?)),
            "color" => (name, Property::Color(math2d::Color::from_str_argb(&value)?)),
            "file" => (name, Property::File(value)),

            _ => return Err(err_msg(format!("Unknown Tiled property type `{}`", kind))),
        })
    }
}
