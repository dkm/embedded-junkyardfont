PNG!= find . -type f -name '*.png'
RAW = ${PNG:.png=.raw}

all: ${RAW}

CARGO_ARGS:=run --release --features=exe -- 

FONTS:=Junkyard.ttf GNUTypeWriter.ttf
SIZES:=7 9 10 12 14 18 24 60 70

pngs:
	echo "use embedded_graphics::{fonts::Font, geometry::Size};" > ../src/fonts.rs
	echo "use {char_offset_impl, CHARS_PER_ROW};" >> ../src/fonts.rs
	for f in $(FONTS); do for s in $(SIZES); do cargo  $(CARGO_ARGS) --size $$s --ttf $$f --pngprefix $$(basename $$f .ttf)  --libfile ../src/fonts.rs; done; done

%.raw: %.png
	convert $^ -depth 1 gray:$@

.PHONY: pngs
