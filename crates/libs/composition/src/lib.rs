#![doc = include_str!("../readme.md")]

// A single wrapper surface over two composition stacks, forked at compile time.
// Exactly one of `system` (Windows.UI.Composition, default) or `reactor`
// (the lifted Microsoft.UI.Composition stack) selects the generated bindings.
// Both are generated from one filter by `tool_composition` and expose the same
// type and method names, so every wrapper module below compiles unchanged
// against either.
#[cfg(all(not(feature = "system"), not(feature = "reactor")))]
compile_error!(
    "enable exactly one composition stack: the `system` feature (default) or the `reactor` feature"
);
#[cfg(all(feature = "system", feature = "reactor"))]
compile_error!(
    "the `system` and `reactor` composition stacks are mutually exclusive; enable only one"
);

#[cfg(feature = "system")]
#[allow(non_snake_case, non_upper_case_globals, clippy::upper_case_acronyms)]
#[path = "bindings.rs"]
mod bindings;
#[cfg(feature = "reactor")]
#[allow(non_snake_case, non_upper_case_globals, clippy::upper_case_acronyms)]
#[path = "bindings_lifted.rs"]
mod bindings;

mod animation;
mod batch;
mod brush;
mod color;
mod compositor;
mod shape;
mod visual;

// The reactor host bridge: adopts a WinUI element's lifted compositor and
// attaches a visual tree built with this crate's API.
#[cfg(feature = "reactor")]
mod reactor;

// Standalone HWND hosting is a system-stack capability (it also owns the
// `windows-window` dependency); lifted composition is hosted inside a WinUI
// element instead.
#[cfg(feature = "system")]
mod stack;
#[cfg(feature = "system")]
mod target;

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
pub use visual::{BorderMode, ContainerVisual, SpriteVisual, Visual, VisualCollection};

#[cfg(feature = "system")]
pub use stack::DispatcherQueueController;
#[cfg(feature = "system")]
pub use target::DesktopWindowTarget;

#[cfg(feature = "reactor")]
pub use reactor::CompositionHostExt;

pub use windows_core::Result;
pub use windows_numerics::{Vector2, Vector3};
