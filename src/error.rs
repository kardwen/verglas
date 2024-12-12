use thiserror::Error;

#[cfg(feature = "forge")]
use write_fonts::{error::Error as WriteError, BuilderError};

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to create font file: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to create font file: {0}")]
    FontRead(#[from] read_fonts::ReadError),

    #[error("no icons found in source")]
    NoIconsFound,

    #[error("destination not valid")]
    InvalidDestination(String),

    #[cfg(feature = "forge")]
    #[error("SVG parsing error: {0}")]
    SvgParse(#[from] usvg::Error),

    #[error("failed to create font: {0}")]
    FontCreation(String),

    #[error("failed to create glyph: {0}")]
    GlyphConversion(String),

    #[cfg(feature = "forge")]
    #[error("failed to build font table: {0}")]
    BuildFontTable(#[from] BuilderError),

    #[cfg(feature = "forge")]
    #[error("write fonts error: {0}")]
    WriteFont(#[from] WriteError),
}
