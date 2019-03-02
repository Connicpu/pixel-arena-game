use crate::tiled::raw::context::ParseContext;

use failure::Fallible;
use xml::attribute as xa;

#[derive(Debug, Serialize, Deserialize)]
pub enum Shape {
    Rectangle,
    Ellipse,
    Point,
    Polygon(Vec<math2d::Point2f>),
    Polyline(Vec<math2d::Point2f>),
}

impl Shape {
    pub fn parse_ellipse(
        context: &mut ParseContext,
        attrs: &[xa::OwnedAttribute],
    ) -> Fallible<Shape> {
        parse_tag! {
            context; attrs;
            <ellipse />
        }

        Ok(Shape::Ellipse)
    }

    pub fn parse_point(
        context: &mut ParseContext,
        attrs: &[xa::OwnedAttribute],
    ) -> Fallible<Shape> {
        parse_tag! {
            context; attrs;
            <point />
        }

        Ok(Shape::Point)
    }

    pub fn parse_polygon(
        context: &mut ParseContext,
        attrs: &[xa::OwnedAttribute],
    ) -> Fallible<Shape> {
        parse_tag! {
            context; attrs;
            <polygon points="points"(String) />
        }

        let points = Self::parse_points(&points)?;

        Ok(Shape::Polygon(points))
    }

    pub fn parse_polyline(
        context: &mut ParseContext,
        attrs: &[xa::OwnedAttribute],
    ) -> Fallible<Shape> {
        parse_tag! {
            context; attrs;
            <polyline points="points"(String) />
        }

        let points = Self::parse_points(&points)?;

        Ok(Shape::Polyline(points))
    }

    fn parse_points(points: &str) -> Fallible<Vec<math2d::Point2f>> {
        points
            .split_whitespace()
            .map(|s| s.split(','))
            .filter_map(|mut i| Some((i.next()?, i.next()?)))
            .map(|(x, y)| Ok((x.parse::<f32>()?, y.parse::<f32>()?).into()))
            .collect()
    }
}
