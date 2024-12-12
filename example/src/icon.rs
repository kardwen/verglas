use iced::{
    widget::{text, Text},
    Font,
};
use std::sync::OnceLock;
use verglas::{build_icon_map, IconMap};

use crate::define_icons;

pub const FONT: Font = iced::Font::with_name("example-icons");
pub const FONT_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/example-icons.ttf"
));

static ICON_MAP: OnceLock<IconMap> = OnceLock::new();

/// Returns an [`IconMap`] that is only build once when first called
fn get_icon_map() -> &'static IconMap {
    ICON_MAP.get_or_init(|| {
        build_icon_map(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/example-icons.ttf"
        ))
        .unwrap_or_else(|e| {
            eprintln!("icon map creation failed: {:?}", e);
            IconMap::new()
        })
    })
}

define_icons! {
    cat_left => "cat-left",
    cat_right => "cat-right",
    book => "jam/book",
    cardboard_box => "jam/archive",
    open_box => "jam/inbox",
    small_box => "jam/box",
}

fn icon<'a>(name: &str) -> Text<'a> {
    let unicode = get_icon_map().get(name).copied().unwrap_or_else(|| {
        eprintln!("icon '{}' not found", name);
        char::REPLACEMENT_CHARACTER
    });

    text(unicode.to_string()).font(FONT)
}
