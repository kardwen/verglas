#![allow(clippy::needless_doctest_main)]
//! 🧊 Iced SVG icon font generator
//!
//! This crate can be added as a build dependency to automatically generate a
//! TrueType font (`.ttf` file) from SVG icons in a directory for use with
//! the [Iced] GUI library.
//!
//! *This library is currently experimental.*
//!
//! [Iced]: https://github.com/iced-rs/iced
//! [example]: https://github.com/kardwen/verglas/tree/main/example
//! [build script]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
//! [`iced_fontello`]: https://github.com/hecrj/iced_fontello
//!
//! # Features
//!
//! - `forge`: Enables font generation functionality ([`make_font`])
//! - `index`: Enables icon mapping functionality ([`build_icon_map`], [`IconMap`])
//!
//! # Font generation
//!
//! Add `verglas` to your `build-dependencies`:
//!
//! ```toml
//! [build-dependencies]
//! verglas = { version = "0.1.0", features = ["forge"] }
//! ```
//!
//! Create a [build script] (`build.rs`) to generate your font:
//!
//! ```rust,ignore
//! fn main() {
//!     println!("cargo::rerun-if-changed=assets/icons/*");
//!
//!     let source_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icons");
//!     let font_file_dest = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/example-icons.ttf");
//!
//!     verglas::make_font(source_dir, font_file_dest)
//!         .expect("building icon font failed")
//! }
//! ```
//!
//! # Icon mapping
//!
//! Have a look at the [example] to see how you can integrate this library with your [Iced] application.
//!
//! Add `verglas` to your `dependencies`:
//!
//! ```toml
//! [dependencies]
//! verglas = { version = "0.1.0", features = ["index"] }
//! ```
//!
//! This lets you create a mapping between icon names and unicode values:
//!
//! ```rust,ignore
//! use iced::{
//!     widget::{text, Text},
//!     Font,
//! };
//! use std::sync::OnceLock;
//! use verglas::{build_icon_map, IconMap};
//!
//! use crate::define_icons;
//!
//! pub const FONT: Font = iced::Font::with_name("example-icons");
//! pub const FONT_BYTES: &[u8] = include_bytes!(concat!(
//!     env!("CARGO_MANIFEST_DIR"),
//!     "/assets/example-icons.ttf"
//! ));
//!
//! static ICON_MAP: OnceLock<IconMap> = OnceLock::new();
//!
//! /// Returns an [`IconMap`] that is only build once when first called
//! fn get_icon_map() -> &'static IconMap {
//!     ICON_MAP.get_or_init(|| {
//!         build_icon_map(concat!(
//!             env!("CARGO_MANIFEST_DIR"),
//!             "/assets/example-icons.ttf"
//!         ))
//!         .unwrap_or_else(|e| {
//!             eprintln!("icon map creation failed: {:?}", e);
//!             IconMap::new()
//!         })
//!     })
//! }
//!
//! define_icons! {
//!     cat_left => "cat-left",
//!     cat_right => "cat-right",
//!     book => "jam/book",
//!     cardboard_box => "jam/archive",
//!     open_box => "jam/inbox",
//!     small_box => "jam/box",
//! }
//!
//! fn icon<'a>(name: &str) -> Text<'a> {
//!     let unicode = get_icon_map().get(name).copied().unwrap_or_else(|| {
//!         eprintln!("icon '{}' not found", name);
//!         char::REPLACEMENT_CHARACTER
//!     });
//!
//!     text(unicode.to_string()).font(FONT)
//! }
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;

#[cfg_attr(docsrs, doc(cfg(feature = "forge")))]
#[cfg(feature = "forge")]
pub mod forge;

#[cfg_attr(docsrs, doc(cfg(feature = "index")))]
#[cfg(feature = "index")]
pub mod index;

pub use crate::error::Error;

#[cfg_attr(docsrs, doc(cfg(feature = "forge")))]
#[cfg(feature = "forge")]
pub use crate::forge::make_font;

#[cfg_attr(docsrs, doc(cfg(feature = "index")))]
#[cfg(feature = "index")]
pub use crate::index::{build_icon_map, IconMap};
