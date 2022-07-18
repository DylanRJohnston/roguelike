use rltk::{FontCharType, RGB};
use specs::{Component, DenseVecStorage};
use specs_derive::Component;

#[derive(Debug, Component)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub foreground: RGB,
    pub background: RGB,
}
