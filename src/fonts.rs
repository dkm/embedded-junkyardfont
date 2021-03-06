use super::CHARS_PER_ROW;
use embedded_graphics::{
    geometry::Size,
    image::ImageRaw,
    mono_font::{mapping::StrGlyphMapping, DecorationDimensions, MonoFont},
};
const GLYPH_MAPPING: StrGlyphMapping =
    StrGlyphMapping::new("  ~ \u{00A0}ÿ", '?' as usize - ' ' as usize);

/// The 7 point size with a character size of 6x7 pixels.
pub const JUNKYARD_7_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/Junkyard7Point.raw"),
        CHARS_PER_ROW * 6,
    ),
    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(6, 7),
    character_spacing: 0,
    baseline: (6) / 2,
    underline: DecorationDimensions::new(5, 1),
    strikethrough: DecorationDimensions::new(4, 1),
};
/// The 9 point size with a character size of 7x9 pixels.
pub const JUNKYARD_9_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/Junkyard9Point.raw"),
        CHARS_PER_ROW * 7,
    ),
    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(7, 9),
    character_spacing: 0,
    baseline: (8) / 2,
    underline: DecorationDimensions::new(7, 1),
    strikethrough: DecorationDimensions::new(5, 1),
};
/// The 10 point size with a character size of 8x10 pixels.
pub const JUNKYARD_10_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/Junkyard10Point.raw"),
        CHARS_PER_ROW * 8,
    ),
    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(8, 10),
    character_spacing: 0,
    baseline: (9) / 2,
    underline: DecorationDimensions::new(8, 1),
    strikethrough: DecorationDimensions::new(6, 1),
};
/// The 12 point size with a character size of 9x12 pixels.
pub const JUNKYARD_12_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/Junkyard12Point.raw"),
        CHARS_PER_ROW * 9,
    ),
    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(9, 12),
    character_spacing: 0,
    baseline: (10) / 2,
    underline: DecorationDimensions::new(9, 1),
    strikethrough: DecorationDimensions::new(7, 1),
};
/// The 14 point size with a character size of 11x14 pixels.
pub const JUNKYARD_14_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/Junkyard14Point.raw"),
        CHARS_PER_ROW * 11,
    ),
    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(11, 14),
    character_spacing: 0,
    baseline: (12) / 2,
    underline: DecorationDimensions::new(11, 1),
    strikethrough: DecorationDimensions::new(8, 1),
};
/// The 18 point size with a character size of 13x18 pixels.
pub const JUNKYARD_18_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/Junkyard18Point.raw"),
        CHARS_PER_ROW * 13,
    ),
    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(13, 18),
    character_spacing: 0,
    baseline: (15) / 2,
    underline: DecorationDimensions::new(14, 1),
    strikethrough: DecorationDimensions::new(10, 1),
};
/// The 24 point size with a character size of 17x23 pixels.
pub const JUNKYARD_24_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/Junkyard24Point.raw"),
        CHARS_PER_ROW * 17,
    ),
    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(17, 23),
    character_spacing: 0,
    baseline: (20) / 2,
    underline: DecorationDimensions::new(18, 1),
    strikethrough: DecorationDimensions::new(13, 1),
};
/// The 60 point size with a character size of 41x56 pixels.
pub const JUNKYARD_60_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/Junkyard60Point.raw"),
        CHARS_PER_ROW * 41,
    ),
    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(41, 56),
    character_spacing: 0,
    baseline: (48) / 2,
    underline: DecorationDimensions::new(44, 1),
    strikethrough: DecorationDimensions::new(32, 1),
};
/// The 70 point size with a character size of 47x66 pixels.
pub const JUNKYARD_70_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/Junkyard70Point.raw"),
        CHARS_PER_ROW * 47,
    ),
    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(47, 66),
    character_spacing: 0,
    baseline: (56) / 2,
    underline: DecorationDimensions::new(52, 1),
    strikethrough: DecorationDimensions::new(37, 1),
};
/// The 7 point size with a character size of 6x8 pixels.
pub const GNUTYPEWRITER_7_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/GNUTypeWriter7Point.raw"),
        CHARS_PER_ROW * 6,
    ),
    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(6, 8),
    character_spacing: 0,
    baseline: (7) / 2,
    underline: DecorationDimensions::new(6, 1),
    strikethrough: DecorationDimensions::new(4, 1),
};
/// The 9 point size with a character size of 7x10 pixels.
pub const GNUTYPEWRITER_9_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/GNUTypeWriter9Point.raw"),
        CHARS_PER_ROW * 7,
    ),
    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(7, 10),
    character_spacing: 0,
    baseline: (8) / 2,
    underline: DecorationDimensions::new(8, 1),
    strikethrough: DecorationDimensions::new(5, 1),
};
/// The 10 point size with a character size of 8x11 pixels.
pub const GNUTYPEWRITER_10_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/GNUTypeWriter10Point.raw"),
        CHARS_PER_ROW * 8,
    ),
    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(8, 11),
    character_spacing: 0,
    baseline: (9) / 2,
    underline: DecorationDimensions::new(8, 1),
    strikethrough: DecorationDimensions::new(6, 1),
};
/// The 12 point size with a character size of 9x13 pixels.
pub const GNUTYPEWRITER_12_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/GNUTypeWriter12Point.raw"),
        CHARS_PER_ROW * 9,
    ),
    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(9, 13),
    character_spacing: 0,
    baseline: (11) / 2,
    underline: DecorationDimensions::new(10, 1),
    strikethrough: DecorationDimensions::new(7, 1),
};
/// The 14 point size with a character size of 10x15 pixels.
pub const GNUTYPEWRITER_14_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/GNUTypeWriter14Point.raw"),
        CHARS_PER_ROW * 10,
    ),
    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(10, 15),
    character_spacing: 0,
    baseline: (12) / 2,
    underline: DecorationDimensions::new(12, 1),
    strikethrough: DecorationDimensions::new(8, 1),
};
/// The 18 point size with a character size of 13x19 pixels.
pub const GNUTYPEWRITER_18_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/GNUTypeWriter18Point.raw"),
        CHARS_PER_ROW * 13,
    ),
    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(13, 19),
    character_spacing: 0,
    baseline: (16) / 2,
    underline: DecorationDimensions::new(15, 1),
    strikethrough: DecorationDimensions::new(10, 1),
};
/// The 24 point size with a character size of 17x25 pixels.
pub const GNUTYPEWRITER_24_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/GNUTypeWriter24Point.raw"),
        CHARS_PER_ROW * 17,
    ),
    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(17, 25),
    character_spacing: 0,
    baseline: (21) / 2,
    underline: DecorationDimensions::new(20, 1),
    strikethrough: DecorationDimensions::new(13, 1),
};
/// The 60 point size with a character size of 39x62 pixels.
pub const GNUTYPEWRITER_60_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/GNUTypeWriter60Point.raw"),
        CHARS_PER_ROW * 39,
    ),
    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(39, 62),
    character_spacing: 0,
    baseline: (50) / 2,
    underline: DecorationDimensions::new(49, 1),
    strikethrough: DecorationDimensions::new(31, 1),
};
/// The 70 point size with a character size of 46x73 pixels.
pub const GNUTYPEWRITER_70_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/GNUTypeWriter70Point.raw"),
        CHARS_PER_ROW * 46,
    ),
    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(46, 73),
    character_spacing: 0,
    baseline: (59) / 2,
    underline: DecorationDimensions::new(58, 1),
    strikethrough: DecorationDimensions::new(36, 1),
};
