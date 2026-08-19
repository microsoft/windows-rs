#[cfg(feature = "canvas")]
use crate::canvas::{CanvasImage, SwapChainCanvas, SwapChainHost};
use crate::composition::CompositionHost;
use crate::element::Element;
use crate::element::controls::action::*;
use crate::element::controls::auto_suggest_box::*;
use crate::element::controls::breadcrumb_bar::*;
use crate::element::controls::collection::*;
use crate::element::controls::content::*;
use crate::element::controls::input::*;
use crate::element::controls::layout::*;
use crate::element::controls::menu::*;
use crate::element::controls::navigation::*;
use crate::element::controls::rich_text::*;
use crate::element::controls::selector::*;
use crate::element::controls::selector_bar::*;
use crate::element::controls::shape::*;
use crate::element::controls::status::*;
use crate::element::controls::tree_view::*;
use crate::element::controls::value::*;
use crate::element::values::*;
use crate::framework_properties::FrameworkProps;
use crate::interaction::{
    AutomationHeadingLevel, Callback, DropEvent, DropTarget, KeyboardAccelerator, PointerEvent,
};
use crate::resources::{ApplicationResource, ElementResources};
#[cfg(feature = "webview")]
use crate::webview::WebViewHost;
use std::time::Duration;

/// Shared builder state for a control.
///
/// Control constructors return `Framework<Control>` so shared and control-specific modifiers can
/// be called in any order. The control type determines which specialized modifiers are available.
pub struct Framework<T> {
    pub(crate) control: T,
    pub(crate) props: FrameworkProps,
}

impl<T> Framework<T> {
    pub(crate) fn new(control: T) -> Self {
        Self {
            control,
            props: FrameworkProps::default(),
        }
    }

    pub fn opacity_transition(mut self, duration: Option<Duration>) -> Self {
        self.props.set_opacity_transition(duration);
        self
    }

    pub fn scale_transition(mut self, duration: Option<Duration>) -> Self {
        self.props.set_scale_transition(duration);
        self
    }

    pub fn scale(mut self, value: Option<f32>) -> Self {
        self.props.set_scale(value);
        self
    }

    pub fn resources<K, V>(mut self, entries: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<String>,
        V: Into<ApplicationResource>,
    {
        let resources = ElementResources::new(entries);
        self.props
            .set_resources((!resources.is_empty()).then_some(resources));
        self
    }

    pub fn width(mut self, value: impl Into<Option<f64>>) -> Self {
        self.props.set_width(value.into());
        self
    }

    pub fn height(mut self, value: impl Into<Option<f64>>) -> Self {
        self.props.set_height(value.into());
        self
    }

    pub fn min_width(mut self, value: impl Into<Option<f64>>) -> Self {
        self.props.set_min_width(value.into());
        self
    }

    pub fn max_width(mut self, value: impl Into<Option<f64>>) -> Self {
        self.props.set_max_width(value.into());
        self
    }

    pub fn min_height(mut self, value: impl Into<Option<f64>>) -> Self {
        self.props.set_min_height(value.into());
        self
    }

    pub fn max_height(mut self, value: impl Into<Option<f64>>) -> Self {
        self.props.set_max_height(value.into());
        self
    }

    pub fn margin(mut self, value: impl Into<Option<Thickness>>) -> Self {
        self.props.set_margin(value.into());
        self
    }

    pub fn horizontal_alignment(mut self, value: impl Into<Option<HorizontalAlignment>>) -> Self {
        self.props.set_horizontal_alignment(value.into());
        self
    }

    pub fn vertical_alignment(mut self, value: impl Into<Option<VerticalAlignment>>) -> Self {
        self.props.set_vertical_alignment(value.into());
        self
    }

    pub fn visibility(mut self, value: impl Into<Option<Visibility>>) -> Self {
        self.props.set_visibility(value.into());
        self
    }

    pub fn opacity(mut self, value: impl Into<Option<f32>>) -> Self {
        self.props.set_opacity(value.into());
        self
    }

    pub fn automation_name(mut self, value: impl Into<String>) -> Self {
        self.props.set_automation_name(Some(value.into()));
        self
    }

    pub fn automation_id(mut self, value: impl Into<String>) -> Self {
        self.props.set_automation_id(Some(value.into()));
        self
    }

    pub fn heading_level(mut self, value: AutomationHeadingLevel) -> Self {
        self.props.set_heading_level(Some(value));
        self
    }

    pub fn help_text(mut self, value: impl Into<String>) -> Self {
        self.props.set_help_text(Some(value.into()));
        self
    }

    pub fn keyboard_accelerator(mut self, value: KeyboardAccelerator) -> Self {
        self.props.push_keyboard_accelerator(value);
        self
    }

    pub fn keyboard_accelerators(
        mut self,
        values: impl IntoIterator<Item = KeyboardAccelerator>,
    ) -> Self {
        self.props
            .set_keyboard_accelerators(values.into_iter().collect());
        self
    }

    pub fn on_pointer_pressed(mut self, handler: impl Fn(PointerEvent) + 'static) -> Self {
        self.props.set_pointer_pressed(Callback::new(handler));
        self
    }

    pub fn on_pointer_moved(mut self, handler: impl Fn(PointerEvent) + 'static) -> Self {
        self.props.set_pointer_moved(Callback::new(handler));
        self
    }

    pub fn on_pointer_released(mut self, handler: impl Fn(PointerEvent) + 'static) -> Self {
        self.props.set_pointer_released(Callback::new(handler));
        self
    }

    pub fn on_pointer_capture_lost(mut self, handler: impl Fn(PointerEvent) + 'static) -> Self {
        self.props.set_pointer_capture_lost(Callback::new(handler));
        self
    }

    pub fn on_pointer_canceled(mut self, handler: impl Fn(PointerEvent) + 'static) -> Self {
        self.props.set_pointer_canceled(Callback::new(handler));
        self
    }

    pub fn on_pointer_entered(mut self, handler: impl Fn(PointerEvent) + 'static) -> Self {
        self.props.set_pointer_entered(Callback::new(handler));
        self
    }

    pub fn on_pointer_exited(mut self, handler: impl Fn(PointerEvent) + 'static) -> Self {
        self.props.set_pointer_exited(Callback::new(handler));
        self
    }

    pub fn on_tapped(mut self, handler: impl Fn() + 'static) -> Self {
        self.props.set_tapped(Callback::new(move |()| handler()));
        self
    }

    pub fn on_right_tapped(mut self, handler: impl Fn() + 'static) -> Self {
        self.props
            .set_right_tapped(Callback::new(move |()| handler()));
        self
    }

    pub fn capture_pointer_on_press(mut self) -> Self {
        self.props.set_capture_pointer_on_press();
        self
    }

    pub fn on_drop(
        mut self,
        target: DropTarget,
        handler: impl Fn(windows_core::Result<DropEvent>) + 'static,
    ) -> Self {
        self.props.set_drop_handler(target, Callback::new(handler));
        self
    }
}

macro_rules! framework_build {
    ($control:ty) => {
        impl Framework<$control> {
            pub fn build(self) -> Element {
                let Self { control, props } = self;
                control.build_with_framework(props)
            }
        }
    };
}

macro_rules! define_framework_builders {
    ($($(#[$attr:meta])* ($control:ident, $element_pattern:pat => $element_props:expr, $mounted_pattern:pat => $mounted_props:expr),)*) => {
        $($(#[$attr])* framework_build!($control);)*
    };
}

framework_elements!(define_framework_builders);

mod enabled_control_sealed {
    pub trait Sealed {}
}

/// Identifies controls that support the shared `enabled` modifier.
#[doc(hidden)]
pub trait EnabledControl: enabled_control_sealed::Sealed {}

impl<T: EnabledControl> Framework<T> {
    pub fn enabled(mut self, value: bool) -> Self {
        self.props.set_enabled(Some(value));
        self
    }
}

mod text_style_control_sealed {
    pub trait Sealed {}
}

/// Identifies controls that support shared text-style modifiers.
#[doc(hidden)]
pub trait TextStyleControl: text_style_control_sealed::Sealed {}

impl<T: TextStyleControl> Framework<T> {
    pub fn font_size(mut self, value: impl Into<Option<f32>>) -> Self {
        self.props.set_font_size(value.into());
        self
    }

    pub fn character_spacing(mut self, value: impl Into<Option<i32>>) -> Self {
        self.props.set_character_spacing(value.into());
        self
    }

    pub fn font_weight(mut self, value: impl Into<Option<FontWeight>>) -> Self {
        self.props.set_font_weight(value.into());
        self
    }

    pub fn font_style(mut self, value: impl Into<Option<FontStyle>>) -> Self {
        self.props.set_font_style(value.into());
        self
    }

    pub fn font_stretch(mut self, value: impl Into<Option<FontStretch>>) -> Self {
        self.props.set_font_stretch(value.into());
        self
    }

    pub fn font_family(mut self, value: Option<String>) -> Self {
        self.props.set_font_family(value);
        self
    }

    pub fn foreground(mut self, value: impl IntoBrushOption) -> Self {
        self.props.set_foreground(value.into_brush_option());
        self
    }
}

macro_rules! define_catalog_builder {
    (Enabled, $control:ident) => {
        impl enabled_control_sealed::Sealed for $control {}
        impl EnabledControl for $control {}
    };
    (Text, $control:ident) => {
        impl text_style_control_sealed::Sealed for $control {}
        impl TextStyleControl for $control {}
    };
}

macro_rules! define_catalog_builders {
    (
        $(
            $(#[$attr:meta])*
            $control:ident => [
                $ui:ident,
                $text:ident,
                $enabled:ident,
                $toggle:ident,
                $attachment:ident
            ],
            [$($builder:ident),*],
        )*
    ) => {
        $(
            $(#[$attr])*
            const _: () = {
                $(define_catalog_builder!($builder, $control);)*
            };
        )*
    };
}

native_control_catalog!(define_catalog_builders);

impl Framework<TextBlock> {
    pub fn text_wrapping(mut self, value: impl Into<Option<TextWrapping>>) -> Self {
        self.props.set_text_wrapping(value.into());
        self
    }

    pub fn text_trimming(mut self, value: impl Into<Option<TextTrimming>>) -> Self {
        self.props.set_text_trimming(value.into());
        self
    }

    pub fn text_selection_enabled(mut self, value: impl Into<Option<bool>>) -> Self {
        self.props.set_text_selection_enabled(value.into());
        self
    }
}
