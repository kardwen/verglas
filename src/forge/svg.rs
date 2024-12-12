use kurbo::{BezPath, CubicBez, Point, Rect, Shape};
use usvg::{tiny_skia_path::PathSegment, Group, Node, Options, Tree};

use crate::Error;

const ACCURACY: f64 = 0.01;

/// Returns the smallest rectangle that encloses all Bézier paths.
pub fn bounding_box(bez_paths: &[BezPath]) -> Rect {
    bez_paths
        .iter()
        .map(|bez_path| bez_path.bounding_box())
        .reduce(|bbox1, bbox2| bbox1.union(bbox2))
        .unwrap_or(Rect::ZERO)
}

/// Simplifies an SVG expression with `usvg` and returns a `kurbo` Bézier curve
/// where cubic segments have been replaced with quadratic segments.
pub fn simplify_svg(svg_data: String) -> Result<Vec<BezPath>, Error> {
    // Simplify SVG with usvg
    let opt = Options::default();
    let tree = Tree::from_str(&svg_data, &opt)?;

    let mut bez_paths = vec![];
    visit_group(tree.root(), &mut bez_paths);

    Ok(bez_paths)
}

fn visit_group(group: &Group, bez_paths: &mut Vec<BezPath>) {
    for node in group.children() {
        match *node {
            Node::Path(ref svg_path) => {
                if let Some(path) = process_svg_path(svg_path) {
                    bez_paths.push(path);
                }
            }
            Node::Group(ref group) => {
                visit_group(group, bez_paths);
            }
            Node::Text(ref _text) => {}
            Node::Image(ref _image) => {}
        }
    }
}

fn process_svg_path(svg_path: &usvg::Path) -> Option<BezPath> {
    let path_data = svg_path.data();
    let mut bez_path = BezPath::new();
    let mut current_point = Point::ZERO;

    for segment in path_data.segments() {
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
