#![no_std]

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
