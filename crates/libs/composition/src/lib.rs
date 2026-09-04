#![doc = include_str!("../readme.md")]

// `system` and `reactor` select compatible generated bindings for the same
// handwritten wrappers. Cargo features are additive, so `reactor` takes
// precedence when both are enabled through dependency feature unification.
#[cfg(all(not(feature = "system"), not(feature = "reactor")))]
compile_error!(
    "enable a composition stack: the `system` feature (default) or the `reactor` feature"
);

#[cfg(all(feature = "system", not(feature = "reactor")))]
#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    clippy::upper_case_acronyms
)]
#[path = "bindings.rs"]
mod bindings;
#[cfg(feature = "reactor")]
#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    clippy::upper_case_acronyms
)]
#[path = "bindings_lifted.rs"]
mod bindings;

mod animation;
mod batch;
mod brush;
mod color;
mod compositor;
mod shape;
mod visual;

// Only system composition hosts an HWND directly. Lifted composition is hosted
// in a WinUI element.
#[cfg(all(feature = "system", not(feature = "reactor")))]
mod stack;
mod surface;
#[cfg(all(feature = "system", not(feature = "reactor")))]
mod target;

mod sealed {
    /// Prevents downstream crates from implementing this crate's marker traits
    /// ([`Brush`](crate::Brush), [`Shape`](crate::Shape),
    /// [`Animation`](crate::Animation)).
    pub trait Sealed {}
}

// Wrapper modules import these through `super::*`.
pub(crate) use sealed::Sealed;
pub(crate) use windows_core::Interface;

pub use animation::{
    Animation, CompositionAnimation, CompositionAnimationGroup, CompositionEasingFunction,
    ImplicitAnimationCollection, ScalarKeyFrameAnimation, Vector3KeyFrameAnimation,
};
pub use batch::{BatchKind, CompositionScopedBatch};
pub use brush::{Brush, CompositionBrush, CompositionColorBrush, CompositionNineGridBrush};
pub use color::Color;
pub use compositor::Compositor;
pub use shape::{
    CompositionContainerShape, CompositionEllipseGeometry, CompositionGeometry,
    CompositionRoundedRectangleGeometry, CompositionShape, CompositionShapeCollection,
    CompositionSpriteShape, Geometry, Shape, ShapeVisual,
};
pub use visual::{BorderMode, ContainerVisual, SpriteVisual, Visual, VisualCollection};

#[cfg(all(feature = "system", not(feature = "reactor")))]
pub use stack::DispatcherQueueController;
pub use surface::{
    CompositionDrawingSurface, CompositionGraphicsDevice, CompositionSurfaceBrush, SurfaceStretch,
};
#[cfg(all(feature = "system", not(feature = "reactor")))]
pub use target::DesktopWindowTarget;

pub use windows_core::Result;
pub use windows_numerics::{Vector2, Vector3};
