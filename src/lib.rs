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
const CHARS_PER_ROW: u32 = 32;

#[cfg(not(feature = "exe"))]
pub mod fonts;

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
