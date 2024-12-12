use write_fonts::{
    tables::name::{Name, NameRecord},
    types::NameId,
    OffsetMarker,
};

pub fn name(font_name: &str) -> Name {
    let name_records = vec![
        NameRecord {
            platform_id: 1,
            encoding_id: 0,
            language_id: 0,
            name_id: NameId::COPYRIGHT_NOTICE,
            string: OffsetMarker::new(
                "Copyright remains with the copyright holders of the SVG icons".to_string(),
            ),
        },
        NameRecord {
            platform_id: 1,
            encoding_id: 0,
            language_id: 0,
            name_id: NameId::FAMILY_NAME,
            string: OffsetMarker::new(font_name.to_string()),
        },
        NameRecord {
            platform_id: 1,
            encoding_id: 0,
            language_id: 0,
            name_id: NameId::SUBFAMILY_NAME,
            string: OffsetMarker::new("Regular".to_string()),
        },
        NameRecord {
            platform_id: 1,
            encoding_id: 0,
            language_id: 0,
            name_id: NameId::UNIQUE_ID,
            string: OffsetMarker::new(font_name.to_string()),
        },
        NameRecord {
            platform_id: 1,
            encoding_id: 0,
            language_id: 0,
            name_id: NameId::FULL_NAME,
            string: OffsetMarker::new(font_name.to_string()),
        },
        NameRecord {
            platform_id: 1,
            encoding_id: 0,
            language_id: 0,
            name_id: NameId::VERSION_STRING,
            string: OffsetMarker::new("Version 1.0".to_string()),
        },
        NameRecord {
            platform_id: 1,
            encoding_id: 0,
            language_id: 0,
            name_id: NameId::POSTSCRIPT_NAME,
            string: OffsetMarker::new(font_name.to_string()),
        },
        NameRecord {
            platform_id: 1,
            encoding_id: 0,
            language_id: 0,
            name_id: NameId::DESCRIPTION,
            string: OffsetMarker::new("Icon font generated from SVG files".to_string()),
        },
        NameRecord {
            platform_id: 1,
            encoding_id: 0,
            language_id: 0,
            name_id: NameId::VENDOR_URL,
            string: OffsetMarker::new("https://github.com/kardwen/verglas".to_string()),
        },
        NameRecord {
            platform_id: 3,
            encoding_id: 1,
            language_id: 0x0409,
            name_id: NameId::COPYRIGHT_NOTICE,
            string: OffsetMarker::new(
                "Copyright remains with the copyright holders of the SVG icons".to_string(),
            ),
        },
        NameRecord {
            platform_id: 3,
            encoding_id: 1,
            language_id: 0x0409,
            name_id: NameId::FAMILY_NAME,
            string: OffsetMarker::new(font_name.to_string()),
        },
        NameRecord {
            platform_id: 3,
            encoding_id: 1,
            language_id: 0x0409,
            name_id: NameId::SUBFAMILY_NAME,
            string: OffsetMarker::new("Regular".to_string()),
        },
        NameRecord {
            platform_id: 3,
            encoding_id: 1,
            language_id: 0x0409,
            name_id: NameId::UNIQUE_ID,
            string: OffsetMarker::new(font_name.to_string()),
        },
        NameRecord {
            platform_id: 3,
            encoding_id: 1,
            language_id: 0x0409,
            name_id: NameId::FULL_NAME,
            string: OffsetMarker::new(font_name.to_string()),
        },
        NameRecord {
            platform_id: 3,
            encoding_id: 1,
            language_id: 0x0409,
            name_id: NameId::VERSION_STRING,
            string: OffsetMarker::new("Version 1.0".to_string()),
        },
        NameRecord {
            platform_id: 3,
            encoding_id: 1,
            language_id: 0x0409,
            name_id: NameId::POSTSCRIPT_NAME,
            string: OffsetMarker::new(font_name.to_string()),
        },
        NameRecord {
            platform_id: 3,
            encoding_id: 1,
            language_id: 0x0409,
            name_id: NameId::DESCRIPTION,
            string: OffsetMarker::new("Icon font generated from SVG files".to_string()),
        },
        NameRecord {
            platform_id: 3,
            encoding_id: 1,
            language_id: 0x0409,
            name_id: NameId::VENDOR_URL,
            string: OffsetMarker::new("https://github.com/kardwen/verglas".to_string()),
        },
    ];

    Name::new(name_records)
}
