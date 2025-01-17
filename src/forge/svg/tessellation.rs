use kurbo::{BezPath, PathEl};
use lyon::{
    path::math::{point, Point},
    tessellation::{
        geometry_builder::{simple_builder, VertexBuffers},
        {StrokeOptions, StrokeTessellator},
    },
};
use usvg::{Fill, Stroke};

/// Applies a stroke to a `BezPath`.
/// Returns a single path when it is filled and two otherwise.
pub fn apply_stroke(bez_path: BezPath, stroke: &Stroke, fill: Option<&Fill>) -> Vec<BezPath> {
    let mut buffers: VertexBuffers<Point, u16> = VertexBuffers::new();
    let mut vertex_builder = simple_builder(&mut buffers);
    let mut tessellator = StrokeTessellator::new();

    // TODO: Stroke options are missing
    let line_width = stroke.width().get();
    let options = StrokeOptions::default().with_line_width(line_width);

    // Create a temporary builder (borrows the tessellator).
    let mut builder = tessellator.builder(&options, &mut vertex_builder);

    for element in bez_path.elements() {
        match element {
            PathEl::MoveTo(bez_point) => {
                builder.begin(point(bez_point.x as f32, bez_point.y as f32));
            }
            PathEl::LineTo(bez_point) => {
                builder.line_to(point(bez_point.x as f32, bez_point.y as f32));
            }
            PathEl::QuadTo(bez_control_point, bez_point) => {
                builder.quadratic_bezier_to(
                    point(bez_control_point.x as f32, bez_control_point.y as f32),
                    point(bez_point.x as f32, bez_point.y as f32),
                );
            }
            PathEl::CurveTo(c1, c2, bez_point) => {
                builder.cubic_bezier_to(
                    point(c1.x as f32, c1.y as f32),
                    point(c2.x as f32, c2.y as f32),
                    point(bez_point.x as f32, bez_point.y as f32),
                );
            }
            PathEl::ClosePath => {
                builder.close();
            }
        }
    }

    // TODO: this may panic
    let _ = builder.build();

    let vertices = buffers.vertices;

    if vertices.is_empty() {
        return vec![];
    }

    let mut result = vec![];

    if fill.is_some() {
        let mut path = BezPath::new();

        // Start with the first vertex
        path.move_to((vertices[0].x as f64, vertices[0].y as f64));

        // Create line segments between consecutive vertices
        for vertex in vertices.chunks(2) {
            if let [_v1, v2] = vertex {
                path.line_to((v2.x as f64, v2.y as f64));
            }
        }

        result.push(path);
    } else {
        // TODO: fix separation of paths
        let mut inner = BezPath::new();
        let mut outer = BezPath::new();

        // Start with the first vertex
        inner.move_to((vertices[1].x as f64, vertices[1].y as f64));
        outer.move_to((vertices[0].x as f64, vertices[0].y as f64));

        // Create line segments between consecutive vertices
        for vertex in vertices.chunks(2) {
            if let [v1, v2] = vertex {
                inner.line_to((v1.x as f64, v1.y as f64));
                outer.line_to((v2.x as f64, v2.y as f64));
            }
        }

        // Invert direction of inner path
        inner.reverse_subpaths();

        result.push(inner);
        result.push(outer);
    }

    result
}
