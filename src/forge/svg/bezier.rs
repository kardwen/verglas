use kurbo::{
    BezPath, CubicBez, Dashes, Join, PathSeg, Point, Rect, Shape, Stroke, StrokeOptLevel,
    StrokeOpts,
};
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
///
/// Assumes that the given path does not contain subpaths
pub fn process_svg_path(svg_path: &Path) -> Option<BezPath> {
    if svg_path.data().is_empty() {
        return None;
    }

    let has_fill = svg_path.fill().is_some();

    if let Some(svg_stroke) = svg_path.stroke() {
        let bez_path = svg_path.to_bez_path(false);

        let style = svg_stroke.to_kurbo();
        let bez_path = stroke_bez_path(&bez_path, style, has_fill);

        Some(bez_path.to_quadric_curves())
    } else if has_fill {
        let bez_path = svg_path.to_bez_path(true);
        Some(bez_path)
    } else {
        None
    }
}

/// Strokes a path
fn stroke_bez_path(bez_path: &BezPath, stroke: Stroke, has_fill: bool) -> BezPath {
    let options = StrokeOpts::default().opt_level(StrokeOptLevel::Optimized);
    let tolerance = 0.01;

    // This function creates a single path for the stroke.
    // If the path was closed, it still creates a single path
    // with a juncture where two edges are the same.
    let stroked_path = kurbo::stroke(bez_path, &stroke, &options, tolerance);

    if has_fill {
        separate_stroked_path(&stroked_path)
            .last()
            .expect("should return subpaths")
            .to_owned()
    } else {
        stroked_path
    }
}

/// Separates a path that was created by `kurbo::stroke` from a stroked and closed path.
///
/// Workaround until the upstream API provides this directly
///
/// Requires that the path from which a stroked path was created has been closed.
///
/// The path created by `kurbo::stroke` looks like this:
/// ┌──────╮
/// │ ┌──╮ │
/// │ │  ├─┤ <- Here is a seam as closing segment of the path
/// │ └──┘ │
/// └──────┘
fn separate_stroked_path(bez_path: &BezPath) -> Vec<BezPath> {
    // The last element is expected to be a seam
    return if let Some(seam) = bez_path.segments().last() {
        let mut result = vec![];
        let mut current_segments = vec![];

        // Separate the path at seam segments
        for segment in bez_path.segments() {
            if matches_seam(&segment, &seam) {
                if !current_segments.is_empty() {
                    let current_path = BezPath::from_path_segments(current_segments.into_iter());
                    result.push(current_path);
                    current_segments = vec![];
                }
            } else {
                current_segments.push(segment);
            }
        }

        result
    } else {
        vec![]
    };

    fn matches_seam(segment: &PathSeg, seam: &PathSeg) -> bool {
        fn points_match(p1: Point, p2: Point) -> bool {
            let tolerance = 0.5;
            (p1.x - p2.x).abs() <= tolerance && (p1.y - p2.y).abs() <= tolerance
        }

        // Directionally invariant comparison
        match (segment, seam) {
            (PathSeg::Line(line1), PathSeg::Line(line2)) => {
                (points_match(line1.p0, line2.p0) && points_match(line1.p1, line2.p1))
                    || (points_match(line1.p0, line2.p1) && points_match(line1.p1, line2.p0))
            }
            _ => false,
        }
    }
}

trait IntoQuadraticCurves {
    fn to_quadric_curves(&self) -> BezPath;
}

impl IntoQuadraticCurves for BezPath {
    fn to_quadric_curves(&self) -> BezPath {
        let mut result = vec![];
        for segment in self.segments() {
            match segment {
                PathSeg::Cubic(cubic) => {
                    for (_, _, quad) in cubic.to_quads(ACCURACY) {
                        result.push(PathSeg::Quad(quad))
                    }
                }
                segment => result.push(segment),
            }
        }
        BezPath::from_path_segments(result.into_iter())
    }
}

trait IntoBezPath {
    fn to_bez_path(&self, convert_cubic_curves: bool) -> BezPath;
}

impl IntoBezPath for Path {
    fn to_bez_path(&self, convert_cubic_curves: bool) -> BezPath {
        let path_data = self.data();

        let mut bez_path = BezPath::new();
        let mut current_point = Point::ZERO;
        // Note that kurbo distinguishes between elements and segments,
        // what usvg (tiny_skia_path) calls segments are elements in kurbo
        for segment in path_data.segments() {
            match segment {
                PathSegment::MoveTo(point) => {
                    current_point = (point.x as f64, point.y as f64).into();
                    bez_path.move_to((point.x as f64, point.y as f64));
                }
                PathSegment::LineTo(point) => {
                    current_point = (point.x as f64, point.y as f64).into();
                    bez_path.line_to((point.x as f64, point.y as f64));
                }
                PathSegment::QuadTo(c1, point) => {
                    current_point = (point.x as f64, point.y as f64).into();
                    bez_path.quad_to((c1.x as f64, c1.y as f64), (point.x as f64, point.y as f64));
                }
                PathSegment::CubicTo(c1, c2, point) => {
                    if convert_cubic_curves {
                        let cubic = CubicBez::new(
                            current_point,
                            (c1.x as f64, c1.y as f64).into(),
                            (c2.x as f64, c2.y as f64).into(),
                            (point.x as f64, point.y as f64).into(),
                        );

                        // Convert cubic Bézier curves to quadratic Bézier curves
                        for (_, _, quad_bez) in cubic.to_quads(ACCURACY) {
                            let control_point = quad_bez.p1;
                            current_point = quad_bez.p2;
                            bez_path.quad_to(control_point, current_point);
                        }
                    } else {
                        bez_path.curve_to(
                            (c1.x as f64, c1.y as f64),
                            (c2.x as f64, c2.y as f64),
                            (point.x as f64, point.y as f64),
                        );
                    }
                }
                PathSegment::Close => {
                    bez_path.close_path();
                }
            }
        }

        bez_path
    }
}

trait IntoKurboStroke {
    fn to_kurbo(&self) -> kurbo::Stroke;
}

impl IntoKurboStroke for usvg::Stroke {
    fn to_kurbo(&self) -> kurbo::Stroke {
        let width = self.width().get() as f64;
        let join = match self.linejoin() {
            usvg::LineJoin::Miter | usvg::LineJoin::MiterClip => Join::Miter,
            usvg::LineJoin::Round => Join::Round,
            usvg::LineJoin::Bevel => Join::Bevel,
        };
        let miter_limit = self.miterlimit().get() as f64;
        let cap = match self.linecap() {
            usvg::LineCap::Butt => kurbo::Cap::Butt,
            usvg::LineCap::Round => kurbo::Cap::Round,
            usvg::LineCap::Square => kurbo::Cap::Square,
        };

        let mut stroke = Stroke::new(width)
            .with_join(join)
            .with_miter_limit(miter_limit)
            .with_caps(cap);

        if let Some(dash_array) = self.dasharray() {
            let offset = self.dashoffset() as f64;
            let pattern = dash_array.iter().map(|value| *value as f64).collect();
            stroke.dash_offset = offset;
            stroke.dash_pattern = Dashes::from_vec(pattern);
        }

        stroke
    }
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
        expected.close_path();

        assert_eq!(result, expected);
    }
}
