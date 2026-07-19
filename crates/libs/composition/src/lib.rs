#![doc = include_str!("../readme.md")]

#[expect(non_snake_case)]
mod bindings;

mod brush;
mod color;
mod compositor;
mod visual;

pub use brush::CompositionColorBrush;
pub use color::Color;
pub use compositor::Compositor;
pub use visual::{ContainerVisual, SpriteVisual, Visual, VisualCollection};

pub use windows_core::Result;
pub use windows_numerics::{Vector2, Vector3};
