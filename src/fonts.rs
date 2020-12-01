use embedded_graphics::{fonts::Font, geometry::Size};
use {char_offset_impl, CHARS_PER_ROW};

            /// The 7 point size with a character size of 6x7 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct Junkyard7Point{}
            impl Font for Junkyard7Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/Junkyard7Point.raw");
                const CHARACTER_SIZE: Size = Size::new(6, 7);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 9 point size with a character size of 7x9 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct Junkyard9Point{}
            impl Font for Junkyard9Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/Junkyard9Point.raw");
                const CHARACTER_SIZE: Size = Size::new(7, 9);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 10 point size with a character size of 8x10 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct Junkyard10Point{}
            impl Font for Junkyard10Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/Junkyard10Point.raw");
                const CHARACTER_SIZE: Size = Size::new(8, 10);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 12 point size with a character size of 9x12 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct Junkyard12Point{}
            impl Font for Junkyard12Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/Junkyard12Point.raw");
                const CHARACTER_SIZE: Size = Size::new(9, 12);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 14 point size with a character size of 11x14 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct Junkyard14Point{}
            impl Font for Junkyard14Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/Junkyard14Point.raw");
                const CHARACTER_SIZE: Size = Size::new(11, 14);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 18 point size with a character size of 13x18 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct Junkyard18Point{}
            impl Font for Junkyard18Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/Junkyard18Point.raw");
                const CHARACTER_SIZE: Size = Size::new(13, 18);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 24 point size with a character size of 17x23 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct Junkyard24Point{}
            impl Font for Junkyard24Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/Junkyard24Point.raw");
                const CHARACTER_SIZE: Size = Size::new(17, 23);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 60 point size with a character size of 41x56 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct Junkyard60Point{}
            impl Font for Junkyard60Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/Junkyard60Point.raw");
                const CHARACTER_SIZE: Size = Size::new(41, 56);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 70 point size with a character size of 47x66 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct Junkyard70Point{}
            impl Font for Junkyard70Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/Junkyard70Point.raw");
                const CHARACTER_SIZE: Size = Size::new(47, 66);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 7 point size with a character size of 6x8 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct GNUTypeWriter7Point{}
            impl Font for GNUTypeWriter7Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/GNUTypeWriter7Point.raw");
                const CHARACTER_SIZE: Size = Size::new(6, 8);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 9 point size with a character size of 7x10 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct GNUTypeWriter9Point{}
            impl Font for GNUTypeWriter9Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/GNUTypeWriter9Point.raw");
                const CHARACTER_SIZE: Size = Size::new(7, 10);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 10 point size with a character size of 8x11 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct GNUTypeWriter10Point{}
            impl Font for GNUTypeWriter10Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/GNUTypeWriter10Point.raw");
                const CHARACTER_SIZE: Size = Size::new(8, 11);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 12 point size with a character size of 9x13 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct GNUTypeWriter12Point{}
            impl Font for GNUTypeWriter12Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/GNUTypeWriter12Point.raw");
                const CHARACTER_SIZE: Size = Size::new(9, 13);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 14 point size with a character size of 10x15 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct GNUTypeWriter14Point{}
            impl Font for GNUTypeWriter14Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/GNUTypeWriter14Point.raw");
                const CHARACTER_SIZE: Size = Size::new(10, 15);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 18 point size with a character size of 13x19 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct GNUTypeWriter18Point{}
            impl Font for GNUTypeWriter18Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/GNUTypeWriter18Point.raw");
                const CHARACTER_SIZE: Size = Size::new(13, 19);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 24 point size with a character size of 17x25 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct GNUTypeWriter24Point{}
            impl Font for GNUTypeWriter24Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/GNUTypeWriter24Point.raw");
                const CHARACTER_SIZE: Size = Size::new(17, 25);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 60 point size with a character size of 39x62 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct GNUTypeWriter60Point{}
            impl Font for GNUTypeWriter60Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/GNUTypeWriter60Point.raw");
                const CHARACTER_SIZE: Size = Size::new(39, 62);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }
            /// The 70 point size with a character size of 46x73 pixels.
            #[derive(Debug, Copy, Clone)]
            pub struct GNUTypeWriter70Point{}
            impl Font for GNUTypeWriter70Point {
              const FONT_IMAGE: &'static [u8] = include_bytes!("../data/GNUTypeWriter70Point.raw");
                const CHARACTER_SIZE: Size = Size::new(46, 73);
                const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
                fn char_offset(c: char) -> u32 {
                    char_offset_impl(c)
                }
            }