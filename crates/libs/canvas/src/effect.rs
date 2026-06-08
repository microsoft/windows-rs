use crate::bindings::*;

/// A Direct2D image effect.
///
/// Effects process input images and produce output images. Create with
/// methods like [`crate::DrawingSession::create_shadow`] and draw the
/// result with [`crate::DrawingSession::draw_effect`].
pub struct Effect(pub(crate) ID2D1Effect);
