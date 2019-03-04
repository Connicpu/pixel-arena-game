use crate::tiled::raw::context::ParseContext;
use crate::tiled::raw::context::ParseOrder;
use crate::tiled::raw::context::Source;
use crate::tiled::raw::objects::shape::Shape;
use crate::tiled::raw::objects::text::Text;
use crate::tiled::raw::properties::Properties;
use crate::tiled::raw::DrawOrder;
use crate::tiled::raw::GlobalTileId;

use failure::Fallible;
use xml::attribute as xa;

pub mod shape;
pub mod text;

#[derive(Debug)]
pub struct ObjectGroup {
    pub parse_order: ParseOrder,
    pub id: Option<i32>,
    pub name: Option<String>,
    pub color: Option<math2d::Color>,
    pub x: f32,
    pub y: f32,
    pub opacity: f32,
    pub visible: bool,
    pub offsetx: f32,
    pub offsety: f32,
    pub draworder: DrawOrder,
    pub properties: Properties,
    pub objects: Vec<Object>,
}

impl ObjectGroup {
    pub fn parse_tag(
        context: &mut ParseContext,
        attrs: &[xa::OwnedAttribute],
    ) -> Fallible<ObjectGroup> {
        use math2d::Color;

        let parse_order = context.parseorder();
        parse_tag! {
            context; attrs;
            <objectgroup
                    ?id="id"(i32)
                    ?name="name"(String)
                    ?color="color"(String)
                    ?x="x"(f32)
                    ?y="y"(f32)
                    ?opacity="opacity"(f32)
                    ?visible="visible"(i32)
                    ?offsetx="offsetx"(f32)
                    ?offsety="offsety"(f32)
                    ?draworder="draworder"(String)>
                <object> => Object::parse_tag,
                <properties> => Properties::parse_tag,
            </objectgroup>
        }

        let color = match color.map(|s| Color::from_str_argb(&s)) {
            Some(Ok(color)) => Some(color),
            Some(Err(err)) => return Err(err.into()),
            None => None,
        };
        let x = x.unwrap_or(0.0);
        let y = y.unwrap_or(0.0);
        let opacity = opacity.unwrap_or(1.0);
        let visible = visible.map(|i| i != 0).unwrap_or(true);
        let offsetx = offsetx.unwrap_or(0.0);
        let offsety = offsety.unwrap_or(0.0);
        let draworder = match draworder.as_ref().map(|s| s.as_str()).unwrap_or("") {
            "index" => DrawOrder::Index,
            _ => DrawOrder::TopDown,
        };
        let properties = properties.pop().unwrap_or_default();
        let objects = object;

        Ok(ObjectGroup {
            parse_order,
            id,
            name,
            color,
            x,
            y,
            opacity,
            visible,
            offsetx,
            offsety,
            draworder,
            properties,
            objects,
        })
    }
}

#[derive(Debug)]
pub struct Object {
    pub parse_order: ParseOrder,
    pub id: i32,
    pub name: Option<String>,
    pub kind: Option<String>,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub gid: Option<GlobalTileId>,
    pub visible: bool,
    pub template: Option<Source>,
    pub properties: Properties,
    pub shape: Shape,
    pub text: Option<Text>,
}

impl Object {
    pub fn parse_tag(context: &mut ParseContext, attrs: &[xa::OwnedAttribute]) -> Fallible<Object> {
        let parse_order = context.parseorder();
        parse_tag! {
            context; attrs;
            <object id="id"(i32) x="x"(f32) y="y"(f32)
                    ?name="name"(String) ?kind="type"(String)
                    ?width="width"(f32) ?height="height"(f32)
                    ?rotation="rotation"(f32)
                    ?gid="gid"(GlobalTileId)
                    ?visible="visible"(i32)
                    ?template="template"(String)
                >
                <properties> => Properties::parse_tag,
                <ellipse> => Shape::parse_ellipse,
                <point> => Shape::parse_point,
                <polygon> => Shape::parse_polygon,
                <polyline> => Shape::parse_polyline,
                <text> => Text::parse_tag,
            </object>
        }

        let width = width.unwrap_or(0.0);
        let height = height.unwrap_or(0.0);
        let rotation = rotation.unwrap_or(0.0);
        let visible = visible.map(|i| i != 0).unwrap_or(true);
        let template = template.map(|s| context.source.relative(&s));
        let properties = properties.pop().unwrap_or_default();
        let text = text.pop();

        let shape = if let Some(ellipse) = ellipse.pop() {
            ellipse
        } else if let Some(point) = point.pop() {
            point
        } else if let Some(polygon) = polygon.pop() {
            polygon
        } else if let Some(polyline) = polyline.pop() {
            polyline
        } else {
            Shape::Rectangle
        };

        Ok(Object {
            parse_order,
            id,
            name,
            kind,
            x,
            y,
            width,
            height,
            rotation,
            gid,
            visible,
            template,
            properties,
            shape,
            text,
        })
    }
}
