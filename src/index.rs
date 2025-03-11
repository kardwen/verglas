//! Font indexing functionality

use font_types::GlyphId16;
use read_fonts::{
    tables::cmap::{Cmap, CmapSubtable, PlatformId},
    FontRef, TableProvider,
};
use std::{collections::HashMap, fs, path::Path};

use crate::Error;

pub type IconMap = HashMap<String, char>;

/// Returns a mapping of icon names to their assigned unicode values.
///
/// This function reads a TrueType font file and extracts the mapping between glyph names
/// and their corresponding unicode values. This is useful for determining which unicode value
/// to use for a specific icon glyph.
pub fn build_icon_map(font_path: impl AsRef<Path>) -> Result<IconMap, Error> {
    // read font file
    let font_data = fs::read(font_path)?;

    build_icon_map_from_bytes(font_data)
}

/// Returns a mapping of icon names to their assigned unicode values.
///
/// This function accepts the bytes of a TrueType font file and extracts the mapping between
/// glyph names and their corresponding unicode values. This is useful for determining which
/// unicode value to use for a specific icon glyph.
pub fn build_icon_map_from_bytes(font_data: impl AsRef<[u8]>) -> Result<IconMap, Error> {
    // Parse font
    let font = FontRef::new(font_data.as_ref())?;

    // Get the 'cmap' table which contains character to glyph mappings
    let cmap = font.cmap()?;

    // Get the 'post' table which contains glyph names
    let post = font.post()?;

    let mut icon_map = HashMap::new();

    for (unicode, glyph_id) in build_cmap_hashmap(&cmap) {
        // Get the glyph name from the post table
        if let Some(glyph_name) = post.glyph_name(glyph_id) {
            // Convert unicode value to char using from_u32
            if let Some(unicode_char) = char::from_u32(unicode as u32) {
                // Store the mapping of glyph name to unicode char
                icon_map.insert(glyph_name.to_string(), unicode_char);
            }
        }
    }

    Ok(icon_map)
}

fn build_cmap_hashmap(cmap: &Cmap) -> HashMap<u16, GlyphId16> {
    let mut mapping = HashMap::new();
    let records = cmap.encoding_records();

    let subtable: Option<CmapSubtable> = records
        .iter()
        .find(|record| {
            record.platform_id() == PlatformId::new(0)
                || (record.platform_id() == PlatformId::new(3) && record.encoding_id() == 1)
        })
        .and_then(|record| cmap.resolve_offset(record.subtable_offset()).ok());

    if let Some(CmapSubtable::Format4(cmap4)) = subtable {
        let start_codes = cmap4.start_code();
        let end_codes = cmap4.end_code();
        let id_deltas = cmap4.id_delta();
        let id_range_offsets = cmap4.id_range_offsets();
        let glyph_id_array = cmap4.glyph_id_array();
        let seg_count = (cmap4.seg_count_x2() / 2) as usize;

        for i in 0..seg_count {
            let start_code = start_codes[i].get();
            let end_code = end_codes[i].get();
            let id_delta = id_deltas[i].get();
            let id_range_offset = id_range_offsets[i].get();

            for cp in start_code..=end_code {
                let glyph_id = if id_range_offset == 0 {
                    // Use delta method
                    // Safely handle the addition with wrapping
                    let result = cp.wrapping_add(id_delta as u16);
                    GlyphId16::new(result)
                } else {
                    // Use range offset method
                    // Calculate offset more safely
                    let range_offset_words = id_range_offset as usize / 2;
                    let char_offset = (cp - start_code) as usize;

                    // Check if we're within bounds before calculating the final offset
                    if i >= id_range_offsets.len() {
                        continue;
                    }

                    let array_offset = range_offset_words + char_offset;

                    // Bounds check before accessing glyph_id_array
                    if let Some(&gid) = glyph_id_array.get(array_offset) {
                        let gid = gid.get();
                        if gid == 0 {
                            continue;
                        }
                        // Safely handle the addition with wrapping
                        GlyphId16::new(gid.wrapping_add(id_delta as u16))
                    } else {
                        continue;
                    }
                };

                mapping.insert(cp, glyph_id);
            }
        }
    }

    mapping
}
