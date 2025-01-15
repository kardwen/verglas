use kurbo::BezPath;
use usvg::{Group, Node, Options, Tree};

mod bezier;

pub use self::bezier::bounding_box;

use self::bezier::process_svg_path;
use crate::Error;

/// Simplifies an SVG expression with `usvg` and returns a list of Bézier curves.
/// All cubic curve segments are replaced with quadratic segments.
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

#[cfg(test)]
mod tests {
    use super::*;
    use kurbo::Rect;

    #[test]
    fn svg_rectangle() {
        // A simple SVG rectangle 100x50 at position (10,10)
        let svg = r#"
            <svg xmlns="http://www.w3.org/2000/svg" width="120" height="70">
                <rect x="10" y="10" width="100" height="50"/>
            </svg>"#
            .to_string();

        let result = simplify_svg(svg).expect("failed to simplify SVG");
        let bbox = bounding_box(&result);

        assert_eq!(bbox, Rect::new(10.0, 10.0, 110.0, 60.0));
    }

    #[test]
    fn svg_two_rectangles() {
        // Two overlapping rectangles
        let svg = r#"
            <svg xmlns="http://www.w3.org/2000/svg" width="200" height="200">
                <rect x="10" y="10" width="100" height="50"/>
                <rect x="40" y="40" width="40" height="40"/>
            </svg>"#
            .to_string();

        let result = simplify_svg(svg).expect("failed to simplify SVG");
        let bbox = bounding_box(&result);

        assert_eq!(bbox, Rect::new(10.0, 10.0, 110.0, 80.0));
    }
}
