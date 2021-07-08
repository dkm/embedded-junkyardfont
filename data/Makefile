PNG!= find . -type f -name '*.png'
RAW = ${PNG:.png=.raw}

all: ${RAW}

CARGO_ARGS:=run --release --features=exe -- 

FONTS:=Junkyard.ttf GNUTypeWriter.ttf
SIZES:=7 9 10 12 14 18 24 60 70

pngs:
	echo "" > ../src/fonts.rs

	echo "use {super::CHARS_PER_ROW};" >> ../src/fonts.rs
	echo "use embedded_graphics::{geometry::Size,image::ImageRaw,mono_font::{mapping::StrGlyphMapping, DecorationDimensions, MonoFont},};" >> ../src/fonts.rs
	echo "const GLYPH_MAPPING: StrGlyphMapping = StrGlyphMapping::new(\"\\0 ~\\0\\u{00A0}ÿ\", '?' as usize - ' ' as usize);" >> ../src/fonts.rs
	for f in $(FONTS); do for s in $(SIZES); do cargo  $(CARGO_ARGS) --size $$s --ttf $$f --pngprefix $$(basename $$f .ttf)  --libfile ../src/fonts.rs; done; done

%.raw: %.png
	convert $^ -depth 1 gray:$@

.PHONY: pngs
