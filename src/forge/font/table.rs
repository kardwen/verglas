use font_types::{Fixed, LongDateTime};
use write_fonts::{
    tables::{
        cmap::Cmap,
        glyf::{Bbox, GlyfLocaBuilder, Glyph, SimpleGlyph},
        head::Head,
        hhea::Hhea,
        hmtx::{Hmtx, LongMetric},
        maxp::Maxp,
        post::Post,
    },
    types::GlyphId,
    FontBuilder,
};

mod name;

use self::name::name;
use super::{glyph::BboxMetrics, ADVANCE};
use crate::Error;

pub fn add_font_tables(
    font: &mut FontBuilder,
    font_name: &str,
    named_glyphs: &[(String, SimpleGlyph)],
) -> Result<(), Error> {
    let num_glyphs = (named_glyphs.len() + 1) as u16; // +1 for .notdef glyph
    let bbox = named_glyphs
        .iter()
        .fold(Bbox::default(), |bbox, (_, glyph)| bbox.union(glyph.bbox));
    let (x_min, y_min, x_max, y_max) = bbox.bounds();

    // Add name table
    let name_table = name(font_name);
    font.add_table(&name_table)?;

    // Add head table
    let head_table = head(x_min, y_min, x_max, y_max);
    font.add_table(&head_table)?;

    // Add hhea table
    let hhea = hhea(num_glyphs);
    font.add_table(&hhea)?;

    // Add maxp table with glyph count
    let maxp = maxp(num_glyphs);
    font.add_table(&maxp)?;

    // Add cmap table for Unicode mapping
    let mappings = named_glyphs.iter().enumerate().map(|(i, _)| {
        let unicode = char::from_u32(i as u32 + 0xE000).unwrap();
        (unicode, GlyphId::new(((i + 1) as u16).into()))
    });
    let cmap = Cmap::from_mappings(mappings).unwrap();
    font.add_table(&cmap)?;

    // Add glyph data
    let mut glyf_builder = GlyfLocaBuilder::new();

    // Add .notdef glyph
    glyf_builder.add_glyph(&Glyph::Empty)?;
    let mut glyph_order = vec![".notdef"];
    let mut h_metrics = vec![LongMetric::new(ADVANCE, 1000)];
    let mut left_side_bearings = vec![0];

    // Add actual glyphs
    for (name, glyph) in named_glyphs {
        glyf_builder.add_glyph(glyph)?;
        let side_bearing = (ADVANCE.saturating_sub(glyph.bbox.width()) / 2) as i16;
        h_metrics.push(LongMetric::new(ADVANCE, side_bearing));
        left_side_bearings.push(side_bearing);
        glyph_order.push(name.as_str());
    }

    // Add hmtx table
    let hmtx = Hmtx::new(h_metrics, left_side_bearings);
    font.add_table(&hmtx)?;

    // Add glyf and loca table
    let (glyf, loca, _loca_format) = glyf_builder.build();
    font.add_table(&glyf)?;
    font.add_table(&loca)?;

    // Add post table
    let post = Post::new_v2(glyph_order);
    font.add_table(&post)?;

    Ok(())
}

pub fn head(x_min: i16, y_min: i16, x_max: i16, y_max: i16) -> Head {
    Head {
        font_revision: Fixed::from_i32(1),
        created: LongDateTime::new(0),
        modified: LongDateTime::new(0),
        units_per_em: 1000,
        x_min,
        y_min,
        x_max,
        y_max,
        index_to_loc_format: 0, // Short format
        ..Head::default()
    }
}

pub fn hhea(number_of_long_metrics: u16) -> Hhea {
    Hhea {
        ascender: (ADVANCE as i16).into(),
        descender: 0.into(),
        line_gap: 0.into(),
        advance_width_max: ADVANCE.into(),
        number_of_long_metrics,
        ..Hhea::default()
    }
}

pub fn maxp(num_glyphs: u16) -> Maxp {
    Maxp {
        num_glyphs,
        ..Maxp::default()
    }
}
