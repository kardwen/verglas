use kurbo::BezPath;
use std::{
    fs,
    path::{Path, PathBuf},
};

use super::svg::simplify_svg;
use crate::Error;

pub fn get_font_name(destination: &Path) -> Result<String, Error> {
    // First validate the file extension
    match destination.extension().and_then(|ext| ext.to_str()) {
        Some(ext) if ext.eq_ignore_ascii_case("ttf") => {
            // If extension is valid, extract the font name
            destination
                .file_stem()
                .and_then(|stem| stem.to_str())
                .map(|s| s.to_string())
                .ok_or_else(|| {
                    Error::InvalidDestination("invalid font name in destination path".into())
                })
        }
        _ => Err(Error::InvalidDestination(
            "destination file must have .ttf extension".into(),
        )),
    }
}

/// Return a list of IDs and paths of SVG files in a source directory and its subdirectories.
pub fn collect_svg_paths(source_dir: &Path) -> Result<Vec<(String, PathBuf)>, Error> {
    get_svg_file_paths(source_dir)?
        .into_iter()
        .map(|file_path| {
            let icon_id = file_path
                .strip_prefix(source_dir)
                .expect("prefix should contain source directory")
                .with_extension("")
                .to_string_lossy()
                .into_owned();
            Ok((icon_id, file_path))
        })
        .collect()
}

fn get_svg_file_paths(dir: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut svg_files = Vec::new();

    fn visit_dir(dir: &Path, svg_files: &mut Vec<PathBuf>) -> Result<(), Error> {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                visit_dir(&path, svg_files)?;
            } else if path.is_file()
                && path
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("svg"))
            {
                svg_files.push(path);
            }
        }
        Ok(())
    }

    visit_dir(dir, &mut svg_files)?;

    if svg_files.is_empty() {
        return Err(Error::NoIconsFound);
    }

    Ok(svg_files)
}

/// Reads an SVG file, simplifies it with [`usvg`], replaces Cubic Bézier paths with Quadratic Bézier
/// paths, and returns a vector of [`BezPath`]
pub fn read_svg_file(file_path: &Path) -> Option<Vec<BezPath>> {
    let svg_data = fs::read_to_string(file_path).ok()?;
    let glyph = simplify_svg(svg_data).ok()?;
    Some(glyph)
}
