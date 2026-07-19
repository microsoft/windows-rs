#![doc = include_str!("../readme.md")]

#[expect(non_snake_case, non_upper_case_globals, clippy::upper_case_acronyms)]
mod bindings;

mod animation;
mod batch;
mod brush;
mod color;
mod compositor;
mod shape;
mod stack;
mod target;
mod visual;

mod sealed {
    /// Prevents downstream crates from implementing this crate's marker traits
    /// ([`Brush`](crate::Brush), [`Shape`](crate::Shape),
    /// [`Animation`](crate::Animation)).
    pub trait Sealed {}
}

pub use animation::{Animation, CompositionAnimation, Vector3KeyFrameAnimation};
pub use batch::{BatchKind, CompositionScopedBatch};
pub use brush::{Brush, CompositionBrush, CompositionColorBrush, CompositionNineGridBrush};
pub use color::Color;
pub use compositor::Compositor;
pub use shape::{
    CompositionContainerShape, CompositionEllipseGeometry, CompositionGeometry, CompositionShape,
    CompositionShapeCollection, CompositionSpriteShape, Shape, ShapeVisual,
};
pub use stack::DispatcherQueueController;
pub use target::DesktopWindowTarget;
pub use visual::{BorderMode, ContainerVisual, SpriteVisual, Visual, VisualCollection};

pub use windows_core::Result;
pub use windows_numerics::{Vector2, Vector3};
