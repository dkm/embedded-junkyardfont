PNG!= find . -type f -name '*.png'
RAW = ${PNG:.png=.raw}

all: ${RAW}

TTFFONT:=Junkyard.ttf
FONTPREFIX:=JunkyardFont

CARGO_ARGS:=run --release --features=exe --  --ttf $(TTFFONT) --pngprefix $(FONTPREFIX) --libfile ../src/fonts.rs

pngs:
	echo "use embedded_graphics::{fonts::Font, geometry::Size};" > ../src/fonts.rs
	echo "use {char_offset_impl, CHARS_PER_ROW};" >> ../src/fonts.rs

	cargo  $(CARGO_ARGS) --size 7
	cargo  $(CARGO_ARGS) --size 9
	cargo  $(CARGO_ARGS) --size 10
	cargo  $(CARGO_ARGS) --size 12
	cargo  $(CARGO_ARGS) --size 14
	cargo  $(CARGO_ARGS) --size 18
	cargo  $(CARGO_ARGS) --size 24
	cargo  $(CARGO_ARGS) --size 70

%.raw: %.png
	convert $^ -depth 1 gray:$@

.PHONY: pngs
