use write_fonts::{tables::glyf::SimpleGlyph, FontBuilder};

pub use self::glyph::create_glyph;

mod glyph;
mod table;

use self::table::add_font_tables;
use crate::Error;

// Should be positive and < i16::MAX
const ADVANCE: u16 = 1000;

/// Creates a font builder that provides fine-grained control over TrueType font generation.
pub fn build_font(
    named_glyphs: Vec<(String, SimpleGlyph)>,
    name: &str,
) -> Result<FontBuilder, Error> {
    let mut font = FontBuilder::new();
    add_font_tables(&mut font, name, &named_glyphs)?;

    Ok(font)
}
