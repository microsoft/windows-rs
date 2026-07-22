use super::*;

/// An animated value that changes over time as transitions are applied.
#[derive(Clone)]
pub struct Variable(pub(crate) IUIAnimationVariable2);

impl Variable {
    /// Returns the current interpolated value.
    pub fn value(&self) -> Result<f64> {
        unsafe { self.0.GetValue() }
    }

    /// Copies the animation curve to a DirectComposition animation object.
    pub fn copy_curve(&self, animation: &impl Interface) -> Result<()> {
        unsafe {
            let dcomp: IDCompositionAnimation = animation.cast()?;
            self.0.GetCurve(&dcomp).ok()
        }
    }
}
