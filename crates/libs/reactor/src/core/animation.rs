use std::time::Duration;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ScalarTransition {
    pub duration: Duration,
}

impl ScalarTransition {
    pub const fn new(duration: Duration) -> Self {
        Self { duration }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Vector3Transition {
    pub duration: Duration,
    pub components: Vector3Axes,
}

impl Vector3Transition {
    pub const fn new(duration: Duration) -> Self {
        Self {
            duration,
            components: Vector3Axes::All,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Vector3Axes {
    X,
    Y,
    Z,
    Xy,
    All,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct ImplicitTransitions {
    pub opacity: Option<ScalarTransition>,
    pub rotation: Option<ScalarTransition>,
    pub scale: Option<Vector3Transition>,
    pub translation: Option<Vector3Transition>,
}

impl ImplicitTransitions {
    pub fn is_empty(&self) -> bool {
        self.opacity.is_none()
            && self.rotation.is_none()
            && self.scale.is_none()
            && self.translation.is_none()
    }
}

/// Implicit transitions applied to layout-managed properties of an
/// element (offset/size). Fed to the backend via `set_layout_animation`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LayoutAnimationConfig {
    pub duration: Duration,
    pub use_spring: bool,
    pub damping_ratio: f32,
    pub period: f32,
    pub animate_offset: bool,
    pub animate_size: bool,
}

impl Default for LayoutAnimationConfig {
    fn default() -> Self {
        Self {
            duration: Duration::from_millis(300),
            use_spring: false,
            damping_ratio: 0.6,
            period: 0.08,
            animate_offset: true,
            animate_size: false,
        }
    }
}

impl LayoutAnimationConfig {
    pub fn linear(duration: Duration) -> Self {
        Self {
            duration,
            ..Self::default()
        }
    }

    pub fn spring() -> Self {
        Self {
            use_spring: true,
            ..Self::default()
        }
    }

    pub fn animate_size(mut self, v: bool) -> Self {
        self.animate_size = v;
        self
    }

    pub fn animate_offset(mut self, v: bool) -> Self {
        self.animate_offset = v;
        self
    }
}

/// One-shot property animation (opacity / scale / …) driven by
/// `Backend::run_property_animation`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AnimationConfig {
    pub opacity: Option<f64>,
    pub scale: Option<f64>,
    pub duration: Duration,
    pub easing: Easing,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            opacity: None,
            scale: None,
            duration: Duration::from_millis(300),
            easing: Easing::EaseOut,
        }
    }
}

impl AnimationConfig {
    pub fn fade_in(duration: Duration) -> Self {
        Self {
            opacity: Some(1.0),
            duration,
            easing: Easing::EaseOut,
            ..Self::default()
        }
    }

    pub fn fade_out(duration: Duration) -> Self {
        Self {
            opacity: Some(0.0),
            duration,
            easing: Easing::EaseIn,
            ..Self::default()
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Easing {
    Linear,
    EaseOut,
    EaseIn,
    EaseInOut,
}

/// Combined animation block stored on [`Modifiers`](crate::Modifiers)`.animations`. All
/// fields are optional and applied independently by the backend.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct AnimationModifiers {
    pub implicit_transitions: Option<ImplicitTransitions>,
    pub layout_animation: Option<LayoutAnimationConfig>,
    pub property_animation: Option<AnimationConfig>,
    pub enter_transition: Option<AnimationConfig>,
    pub exit_transition: Option<AnimationConfig>,
}

impl AnimationModifiers {
    pub fn is_empty(&self) -> bool {
        self.implicit_transitions
            .as_ref()
            .is_none_or(|t| t.is_empty())
            && self.layout_animation.is_none()
            && self.property_animation.is_none()
            && self.enter_transition.is_none()
            && self.exit_transition.is_none()
    }
}
