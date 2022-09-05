use bracket_lib::prelude::{ColorPair, FontCharType};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub color: ColorPair,
}
