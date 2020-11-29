#![no_std]

//! The [ProFont](https://web.archive.org/web/20180412214402/http://tobiasjung.name/profont/)
//! monospace programming font for use with
//! [embedded-graphics](https://github.com/jamwaffles/embedded-graphics). Font data taken from the
//! [ProFont homepage](https://web.archive.org/web/20180412214402/http://tobiasjung.name/profont/).
//!
//! ### Synopsis
//!
//! Assuming `display` is something that implements the [Drawing
//! trait](https://docs.rs/embedded-graphics/0.4.4/embedded_graphics/trait.Drawing.html)
//!
//! ```ignore
//! display.draw(
//!     Text::new("Hello World")
//!         into_styled(text_style!(
//!             font = ProFont24Point,
//!             text_color = Black,
//!             background_color = White
//!         ))
//!         .translate(Point::new(10, 10))
//!         .into_iter(),
//! );
//! ```
//!
//! For a more complete example see [the example in the ssd1675
//! crate](https://github.com/wezm/ssd1675/blob/master/examples/raspberry_pi_inky_phat.rs).
//!
//! ### Glyph Coverage
//!
//! This crate provides support for [ISO/IEC 8859-1](https://en.wikipedia.org/wiki/ISO/IEC_8859-1)
//! (latin1), although do note that the font is missing a few glyphs in this range.

extern crate embedded_graphics;

#[cfg(not(feature = "exe"))]
use embedded_graphics::{fonts::Font, geometry::Size};

#[cfg(not(feature = "exe"))]
const CHARS_PER_ROW: u32 = 32;

#[cfg(not(feature = "exe"))]
fn char_offset_impl(c: char) -> u32 {
    let fallback = '?' as u32 - ' ' as u32;
    if c < ' ' {
        return fallback;
    }
    if c <= '~' {
        return c as u32 - ' ' as u32;
    }
    if c < '\u{00A0}' || c > 'ÿ' {
        return fallback;
    }
    c as u32 - ' ' as u32 - 33
}

/// The 7 point size with a character size of 5x9 pixels.
#[derive(Debug, Copy, Clone)]
pub struct JunkyardFont7Point {}

#[cfg(not(feature = "exe"))]
impl Font for JunkyardFont7Point {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../data/JunkyardFont7Point.raw");
    const CHARACTER_SIZE: Size = Size::new(6, 7);
    const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
    fn char_offset(c: char) -> u32 {
        char_offset_impl(c)
    }
}

/// The 9 point size with a character size of 6x11 pixels.
#[derive(Debug, Copy, Clone)]
pub struct JunkyardFont9Point;
#[cfg(not(feature = "exe"))]
impl Font for JunkyardFont9Point {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../data/JunkyardFont9Point.raw");
    const CHARACTER_SIZE: Size = Size::new(7, 9);
    const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
    fn char_offset(c: char) -> u32 {
        char_offset_impl(c)
    }
}

/// The 10 point size with a character size of 7x13 pixels.
#[derive(Debug, Copy, Clone)]
pub struct JunkyardFont10Point;
#[cfg(not(feature = "exe"))]
impl Font for JunkyardFont10Point {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../data/JunkyardFont10Point.raw");
    const CHARACTER_SIZE: Size = Size::new(8, 10);
    const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
    fn char_offset(c: char) -> u32 {
        char_offset_impl(c)
    }
}

/// The 12 point size with a character size of 8x15 pixels.
#[derive(Debug, Copy, Clone)]
pub struct JunkyardFont12Point;
#[cfg(not(feature = "exe"))]
impl Font for JunkyardFont12Point {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../data/JunkyardFont12Point.raw");
    const CHARACTER_SIZE: Size = Size::new(9, 12);
    const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
    fn char_offset(c: char) -> u32 {
        char_offset_impl(c)
    }
}

/// The 14 point size with a character size of 10x18 pixels.
#[derive(Debug, Copy, Clone)]
pub struct JunkyardFont14Point;
#[cfg(not(feature = "exe"))]
impl Font for JunkyardFont14Point {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../data/JunkyardFont14Point.raw");
    const CHARACTER_SIZE: Size = Size::new(11, 14);
    const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
    fn char_offset(c: char) -> u32 {
        char_offset_impl(c)
    }
}

/// The 18 point size with a character size of 12x22 pixels.
#[derive(Debug, Copy, Clone)]
pub struct JunkyardFont18Point;
#[cfg(not(feature = "exe"))]
impl Font for JunkyardFont18Point {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../data/JunkyardFont18Point.raw");
    const CHARACTER_SIZE: Size = Size::new(13, 18);
    const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
    fn char_offset(c: char) -> u32 {
        char_offset_impl(c)
    }
}

/// The 24 point size with a character size of 16x30 pixels.
#[derive(Debug, Copy, Clone)]
pub struct JunkyardFont24Point;
#[cfg(not(feature = "exe"))]
impl Font for JunkyardFont24Point {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../data/JunkyardFont24Point.raw");
    const CHARACTER_SIZE: Size = Size::new(17, 23);
    const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
    fn char_offset(c: char) -> u32 {
        char_offset_impl(c)
    }
}

/// The 24 point size with a character size of 16x30 pixels.
#[derive(Debug, Copy, Clone)]
pub struct JunkyardFont70Point;
#[cfg(not(feature = "exe"))]
impl Font for JunkyardFont70Point {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../data/JunkyardFont70Point.raw");
    const CHARACTER_SIZE: Size = Size::new(47, 66);
    const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
    fn char_offset(c: char) -> u32 {
        char_offset_impl(c)
    }
}
