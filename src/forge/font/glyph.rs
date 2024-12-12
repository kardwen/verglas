use kurbo::{BezPath, PathEl, Point};
use read_fonts::tables::glyf::CurvePoint;
use write_fonts::tables::glyf::{Bbox, SimpleGlyph};

use super::ADVANCE;
use crate::{forge::svg::bounding_box, Error};

#[allow(dead_code)]
pub trait BboxMetrics {
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn bounds(&self) -> (i16, i16, i16, i16);
}

impl BboxMetrics for Bbox {
    fn width(&self) -> u16 {
        self.x_max.abs_diff(self.x_min)
    }

    fn height(&self) -> u16 {
        self.y_max.abs_diff(self.y_min)
    }

    fn bounds(&self) -> (i16, i16, i16, i16) {
        (self.x_min, self.y_min, self.x_max, self.y_max)
    }
}

struct FontTransform {
    scale_factor: f64,
    delta_y: i16,
}

impl FontTransform {
    fn new(bez_paths: &[BezPath]) -> Self {
        let bez_bbox = bounding_box(bez_paths);
        let scale_factor = ADVANCE as f64 / bez_bbox.size().max_side();

        let projected_height = Self::project(bez_bbox.abs().height(), scale_factor);
        let delta_y = ADVANCE as i16 - (ADVANCE.saturating_sub(projected_height as u16) as i16) / 2;

        Self {
            scale_factor,
            delta_y,
        }
    }

    fn transform_point(&self, point: &Point, on_curve: bool) -> CurvePoint {
        // SVG origin is in the top left corner, orientation of y is upside down
        CurvePoint {
            x: Self::project(point.x, self.scale_factor),
            y: -Self::project(point.y, self.scale_factor) + self.delta_y,
            on_curve,
        }
    }

    fn project(value: f64, scale_factor: f64) -> i16 {
        (value * scale_factor).round() as i16
    }
}

/// Creates a glyph from Beziér curves created by [`kurbo`].
/// The path must not contain cubic Bézier curves.
pub fn create_glyph(bez_paths: Vec<BezPath>) -> Result<SimpleGlyph, Error> {
    let transform = FontTransform::new(&bez_paths);
    let mut contours = Vec::new();

    for bez_path in bez_paths {
        let mut curve_points = Vec::new();
        let mut start_svg_point = Point::ZERO;
        let mut current_svg_point = Point::ZERO;

        for element in bez_path.elements() {
            match element {
                PathEl::MoveTo(svg_point) => {
                    start_svg_point = *svg_point;
                    current_svg_point = *svg_point;

                    let curve_point = transform.transform_point(svg_point, true);
                    curve_points.push(curve_point);
                }
                PathEl::LineTo(svg_point) => {
                    current_svg_point = *svg_point;

                    let curve_point = transform.transform_point(svg_point, true);
                    curve_points.push(curve_point);
                }
                PathEl::QuadTo(svg_control_point, svg_point) => {
                    current_svg_point = *svg_point;

                    let curve_control_point = transform.transform_point(svg_control_point, false);
                    let curve_point = transform.transform_point(svg_point, true);

                    curve_points.push(curve_control_point);
                    curve_points.push(curve_point);
                }
                PathEl::CurveTo(_, _, _) => {
                    return Err(Error::GlyphConversion(
                        "cubic Bézier curves are not allowed".to_string(),
                    ))
                }
                PathEl::ClosePath => {
                    let tolerance: f64 = 1e-5;
                    if start_svg_point.distance(current_svg_point) > tolerance {
                        let curve_point = transform.transform_point(&start_svg_point, true);
                        curve_points.push(curve_point);
                    }
                    if !curve_points.is_empty() {
                        contours.push(curve_points.clone().into());
                        curve_points.clear();
                    }
                }
            }
        }

        if !curve_points.is_empty() {
            contours.push(curve_points.into());
        }
    }

    let mut glyph = SimpleGlyph {
        bbox: Bbox::default(),
        contours,
        instructions: vec![],
    };
    glyph.recompute_bounding_box();
    Ok(glyph)
}
