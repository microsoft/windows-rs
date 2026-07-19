use crate::bindings;
use crate::sealed::Sealed;
use crate::{Brush, Visual};
use windows_core::{Interface, Result};
use windows_numerics::Vector2;

/// The base type shared by every composition shape. A [`Shape`] can be turned
/// into one via [`Shape::as_shape`] to append it to a
/// [`CompositionShapeCollection`].
#[derive(Clone)]
pub struct CompositionShape(pub(crate) bindings::CompositionShape);

/// A shape that can be appended to a [`CompositionShapeCollection`].
///
/// This trait is sealed: only the shape types in this crate implement it.
pub trait Shape: Sealed {
    /// Returns this shape as the shared [`CompositionShape`] base type.
    fn as_shape(&self) -> Result<CompositionShape>;
}

/// The base type shared by every composition geometry.
#[derive(Clone)]
pub struct CompositionGeometry(pub(crate) bindings::CompositionGeometry);

/// An ellipse (or circle) geometry, defined by its radii.
#[derive(Clone)]
pub struct CompositionEllipseGeometry(pub(crate) bindings::CompositionEllipseGeometry);

impl CompositionEllipseGeometry {
    /// Sets the geometry's x and y radii, in DIPs.
    pub fn set_radius(&self, radius: Vector2) -> Result<()> {
        self.0.SetRadius(radius)
    }

    pub(crate) fn as_geometry(&self) -> Result<CompositionGeometry> {
        Ok(CompositionGeometry(self.0.cast()?))
    }
}

/// A shape that fills a geometry with a [`Brush`].
#[derive(Clone)]
pub struct CompositionSpriteShape(pub(crate) bindings::CompositionSpriteShape);

impl CompositionSpriteShape {
    /// Sets the brush used to fill the shape's geometry.
    pub fn set_fill_brush(&self, brush: &impl Brush) -> Result<()> {
        self.0.SetFillBrush(&brush.as_brush()?.0)
    }

    /// Sets the shape's offset from its parent, in DIPs.
    pub fn set_offset(&self, offset: Vector2) -> Result<()> {
        let shape: bindings::ICompositionShape = self.0.cast()?;
        shape.SetOffset(offset)
    }
}

impl Sealed for CompositionSpriteShape {}

impl Shape for CompositionSpriteShape {
    fn as_shape(&self) -> Result<CompositionShape> {
        Ok(CompositionShape(self.0.cast()?))
    }
}

/// A shape that groups a child [`shapes`](Self::shapes) collection.
#[derive(Clone)]
pub struct CompositionContainerShape(pub(crate) bindings::CompositionContainerShape);

impl CompositionContainerShape {
    /// Returns the collection of child shapes.
    pub fn shapes(&self) -> Result<CompositionShapeCollection> {
        Ok(CompositionShapeCollection(self.0.Shapes()?))
    }
}

impl Sealed for CompositionContainerShape {}

impl Shape for CompositionContainerShape {
    fn as_shape(&self) -> Result<CompositionShape> {
        Ok(CompositionShape(self.0.cast()?))
    }
}

/// An ordered collection of shapes owned by a [`ShapeVisual`] or a
/// [`CompositionContainerShape`].
#[derive(Clone)]
pub struct CompositionShapeCollection(pub(crate) bindings::CompositionShapeCollection);

impl CompositionShapeCollection {
    /// Appends a shape to the end of the collection.
    pub fn append(&self, shape: &impl Shape) -> Result<()> {
        self.0.Append(&shape.as_shape()?.0)
    }
}

/// A visual that renders a collection of composition [`shapes`](Self::shapes).
/// Derefs to [`Visual`], so a shape visual can be positioned and sized directly.
#[derive(Clone)]
pub struct ShapeVisual {
    visual: Visual,
    shape_visual: bindings::ShapeVisual,
}

impl ShapeVisual {
    pub(crate) fn new(shape_visual: bindings::ShapeVisual) -> Result<Self> {
        Ok(Self {
            visual: Visual(shape_visual.cast()?),
            shape_visual,
        })
    }

    /// Returns the collection of shapes rendered by the visual.
    pub fn shapes(&self) -> Result<CompositionShapeCollection> {
        Ok(CompositionShapeCollection(self.shape_visual.Shapes()?))
    }
}

impl core::ops::Deref for ShapeVisual {
    type Target = Visual;
    fn deref(&self) -> &Visual {
        &self.visual
    }
}
