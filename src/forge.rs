//! Font generation functionality
//!
//! Also provides access to lower-level functions for more fine-grained control.

use std::{
    fs,
    path::{Path, PathBuf},
};
use write_fonts::tables::glyf::SimpleGlyph;

pub use self::{file::collect_svg_paths, font::build_font};

mod file;
mod font;
mod svg;

use self::{
    file::{get_font_name, read_svg_file},
    font::create_glyph,
};
use crate::error::Error;

/// Builds a TrueType font from SVG icons in a source directory and writes it to a `.ttf` file.
///
/// The font name is derived from the file name that must be included in the destination path.
/// Glyph names are derived from the name of the SVG file (without file extension) and, when
/// present, prefixed by subdirectory names separated by "/".
pub fn make_font(
    source_dir: impl AsRef<Path>,
    font_file_dest: impl AsRef<Path>,
) -> Result<(), Error> {
    let font_name = get_font_name(font_file_dest.as_ref())?;
    let named_svg_files = collect_svg_paths(source_dir.as_ref())?;
    let named_glyphs = process_svg_files(named_svg_files)?;

    let mut font = build_font(named_glyphs, &font_name)?;
    fs::write(font_file_dest, font.build())?;

    Ok(())
}

/// Creates a list of named glyphs.
pub fn process_svg_files(
    files: Vec<(String, PathBuf)>,
) -> Result<Vec<(String, SimpleGlyph)>, Error> {
    files
        .into_iter()
        .filter_map(|(icon_id, file_path)| {
            match read_svg_file(&file_path)
                .map(|svg| create_glyph(svg).map(|glyph| (icon_id, glyph)))
                .transpose()
            {
                Ok(Some(glyph)) => Some(Ok(glyph)),
                Ok(None) => None,
                Err(e) => Some(Err(e)),
            }
        })
        .collect()
}
