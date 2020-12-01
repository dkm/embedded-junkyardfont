use embedded_graphics::{fonts::Font, geometry::Size};
use {char_offset_impl, CHARS_PER_ROW};

            /// The 7 point size with a character size of 6x7 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct JunkyardFont7Point{}
            impl Font for JunkyardFont7Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/JunkyardFont7Point.raw");
                const CHARACTER_SIZE: Size = Size::new(6, 7);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 9 point size with a character size of 7x9 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct JunkyardFont9Point{}
            impl Font for JunkyardFont9Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/JunkyardFont9Point.raw");
                const CHARACTER_SIZE: Size = Size::new(7, 9);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 10 point size with a character size of 8x10 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct JunkyardFont10Point{}
            impl Font for JunkyardFont10Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/JunkyardFont10Point.raw");
                const CHARACTER_SIZE: Size = Size::new(8, 10);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 12 point size with a character size of 9x12 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct JunkyardFont12Point{}
            impl Font for JunkyardFont12Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/JunkyardFont12Point.raw");
                const CHARACTER_SIZE: Size = Size::new(9, 12);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 14 point size with a character size of 11x14 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct JunkyardFont14Point{}
            impl Font for JunkyardFont14Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/JunkyardFont14Point.raw");
                const CHARACTER_SIZE: Size = Size::new(11, 14);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 18 point size with a character size of 13x18 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct JunkyardFont18Point{}
            impl Font for JunkyardFont18Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/JunkyardFont18Point.raw");
                const CHARACTER_SIZE: Size = Size::new(13, 18);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 24 point size with a character size of 17x23 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct JunkyardFont24Point{}
            impl Font for JunkyardFont24Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/JunkyardFont24Point.raw");
                const CHARACTER_SIZE: Size = Size::new(17, 23);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 70 point size with a character size of 47x66 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct JunkyardFont70Point{}
            impl Font for JunkyardFont70Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/JunkyardFont70Point.raw");
                const CHARACTER_SIZE: Size = Size::new(47, 66);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }