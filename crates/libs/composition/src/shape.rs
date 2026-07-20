use super::*;

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
    fn as_shape(&self) -> CompositionShape;
}

/// The base type shared by every composition geometry.
#[derive(Clone)]
pub struct CompositionGeometry(pub(crate) bindings::CompositionGeometry);

/// An ellipse (or circle) geometry, defined by its radii.
#[derive(Clone)]
pub struct CompositionEllipseGeometry(pub(crate) bindings::CompositionEllipseGeometry);

impl CompositionEllipseGeometry {
    /// Sets the geometry's x and y radii, in DIPs.
    pub fn set_radius(&self, radius: Vector2) {
        self.0.SetRadius(radius).unwrap();
    }

    pub(crate) fn as_geometry(&self) -> CompositionGeometry {
        CompositionGeometry(self.0.cast().unwrap())
    }
}

/// A shape that fills a geometry with a [`Brush`].
#[derive(Clone)]
pub struct CompositionSpriteShape(pub(crate) bindings::CompositionSpriteShape);

impl CompositionSpriteShape {
    /// Sets the brush used to fill the shape's geometry.
    pub fn set_fill_brush(&self, brush: &impl Brush) {
        self.0.SetFillBrush(&brush.as_brush().0).unwrap();
    }

    /// Sets the shape's offset from its parent, in DIPs.
    pub fn set_offset(&self, offset: Vector2) {
        let shape: bindings::ICompositionShape = self.0.cast().unwrap();
        shape.SetOffset(offset).unwrap();
    }
}

impl Sealed for CompositionSpriteShape {}

impl Shape for CompositionSpriteShape {
    fn as_shape(&self) -> CompositionShape {
        CompositionShape(self.0.cast().unwrap())
    }
}

/// A shape that groups a child [`shapes`](Self::shapes) collection.
#[derive(Clone)]
pub struct CompositionContainerShape(pub(crate) bindings::CompositionContainerShape);

impl CompositionContainerShape {
    /// Returns the collection of child shapes.
    pub fn shapes(&self) -> CompositionShapeCollection {
        CompositionShapeCollection(self.0.Shapes().unwrap())
    }
}

impl Sealed for CompositionContainerShape {}

impl Shape for CompositionContainerShape {
    fn as_shape(&self) -> CompositionShape {
        CompositionShape(self.0.cast().unwrap())
    }
}

/// An ordered collection of shapes owned by a [`ShapeVisual`] or a
/// [`CompositionContainerShape`].
#[derive(Clone)]
pub struct CompositionShapeCollection(pub(crate) bindings::CompositionShapeCollection);

impl CompositionShapeCollection {
    /// Appends a shape to the end of the collection.
    pub fn append(&self, shape: &impl Shape) {
        self.0.Append(&shape.as_shape().0).unwrap();
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
    pub(crate) fn new(shape_visual: bindings::ShapeVisual) -> Self {
        Self {
            visual: Visual(shape_visual.cast().unwrap()),
            shape_visual,
        }
    }

    /// Returns the collection of shapes rendered by the visual.
    pub fn shapes(&self) -> CompositionShapeCollection {
        CompositionShapeCollection(self.shape_visual.Shapes().unwrap())
    }
}

impl core::ops::Deref for ShapeVisual {
    type Target = Visual;
    fn deref(&self) -> &Visual {
        &self.visual
    }
}
