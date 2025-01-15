use kurbo::{BezPath, CubicBez, Point, Rect, Shape};
use usvg::{tiny_skia_path::PathSegment, Path};

const ACCURACY: f64 = 0.01;

/// Returns the smallest rectangle that encloses all Bézier paths.
pub fn bounding_box(bez_paths: &[BezPath]) -> Rect {
    bez_paths
        .iter()
        .map(|bez_path| bez_path.bounding_box())
        .reduce(|bbox1, bbox2| bbox1.union(bbox2))
        .unwrap_or(Rect::ZERO)
}

/// Takes an usvg path and converts it to a Bézier curve
/// where cubic segments have been replaced with quadratic segments.
pub fn process_svg_path(svg_path: &Path) -> Option<BezPath> {
    let path_data = svg_path.data();
    let mut segments = path_data.segments();
    segments.set_auto_close(true);

    let mut bez_path = BezPath::new();
    let mut current_point = Point::ZERO;
    for segment in segments {
        match segment {
            PathSegment::MoveTo(point) => {
                current_point = Point::new(point.x as f64, point.y as f64);
                bez_path.move_to((point.x as f64, point.y as f64));
            }
            PathSegment::LineTo(point) => {
                current_point = Point::new(point.x as f64, point.y as f64);
                bez_path.line_to((point.x as f64, point.y as f64));
            }
            PathSegment::QuadTo(c1, point) => {
                current_point = Point::new(point.x as f64, point.y as f64);
                bez_path.quad_to((c1.x as f64, c1.y as f64), (point.x as f64, point.y as f64));
            }
            PathSegment::CubicTo(c1, c2, point) => {
                let cubic = CubicBez::new(
                    current_point,
                    (c1.x as f64, c1.y as f64).into(),
                    (c2.x as f64, c2.y as f64).into(),
                    (point.x as f64, point.y as f64).into(),
                );

                // Convert to quadratic Bézier curves
                for (_, _, quad) in cubic.to_quads(ACCURACY) {
                    let control_point = Point::new(quad.p1.x, quad.p1.y);
                    current_point = Point::new(quad.p2.x, quad.p2.y);
                    bez_path.quad_to(
                        (control_point.x, control_point.y),
                        (current_point.x, current_point.y),
                    );
                }
            }
            PathSegment::Close => {
                bez_path.close_path();
            }
        }
    }

    if bez_path.is_empty() {
        return None;
    }

    Some(bez_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use usvg::{Node, Options, Tree};

    #[test]
    fn simple_svg_path() {
        let svg = r#"
            <svg width="100" height="100" xmlns="http://www.w3.org/2000/svg">
                <path d="M 10 10 H 90 V 90 H 10 Z"/>
            </svg>"#
            .to_string();

        let opt = Options::default();
        let tree = Tree::from_str(&svg, &opt).expect("failed to parse SVG string");

        let root_children = tree.root().children();
        let node = root_children.first().expect("unfitting SVG");

        let svg_path = match node {
            Node::Path(path) => path,
            _ => panic!("unfitting SVG"),
        };

        let result = process_svg_path(svg_path).expect("processing SVG path failed");

        let mut expected = BezPath::new();
        expected.move_to(Point::new(10.0, 10.0));
        expected.line_to(Point::new(90.0, 10.0));
        expected.line_to(Point::new(90.0, 90.0));
        expected.line_to(Point::new(10.0, 90.0));
        expected.line_to(Point::new(10.0, 10.0));
        expected.close_path();

        assert_eq!(result, expected);
    }
}
