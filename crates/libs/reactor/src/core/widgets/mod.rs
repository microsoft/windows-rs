//! Built-in widget catalogue. One file per widget; each file owns the
//! widget struct, its builder methods, its `impl Widget`, any related
//! support types, and the convenience factory function (if any).

// Make the core types each widget module depends on visible via
// `use super::*;` inside the per-widget files.
pub(crate) use super::*;

/// Emit the boilerplate `kind`/`key`/`modifiers` accessors shared by
/// every `impl Widget for X`; `attached` falls through to the default
/// impl on the trait.
macro_rules! widget_header {
    ($kind:expr) => {
        fn kind(&self) -> ControlKind {
            $kind
        }
        fn key(&self) -> Option<&str> {
            self.key.as_deref()
        }
        fn modifiers(&self) -> &Modifiers {
            &self.modifiers
        }
    };
}
pub(crate) use widget_header;

pub(crate) mod auto_suggest_box;
pub(crate) mod border;
pub(crate) mod breadcrumb_bar;
pub(crate) mod button;
pub(crate) mod calendar_date_picker;
pub(crate) mod calendar_view;
pub(crate) mod canvas;
pub(crate) mod check_box;
pub(crate) mod color_picker;
pub(crate) mod combo_box;
pub(crate) mod command_bar;
pub(crate) mod content_dialog;
pub(crate) mod date_picker;
pub(crate) mod drop_down_button;
pub(crate) mod expander;
pub(crate) mod flyout;
pub(crate) mod grid;
pub(crate) mod hyperlink_button;
pub(crate) mod icon;
pub(crate) mod image;
pub(crate) mod info_badge;
pub(crate) mod info_bar;
pub(crate) mod list_box;
pub(crate) mod menu_bar;
pub(crate) mod navigation_view;
pub(crate) mod number_box;
pub(crate) mod password_box;
pub(crate) mod person_picture;
pub(crate) mod pivot;
pub(crate) mod progress_bar;
pub(crate) mod progress_ring;
pub(crate) mod radio_button;
pub(crate) mod radio_buttons;
pub(crate) mod rating_control;
pub(crate) mod relative_panel;
pub(crate) mod repeat_button;
pub(crate) mod rich_edit_box;
pub(crate) mod scroll_view;
pub(crate) mod scroll_viewer;
pub(crate) mod selector_bar;
pub(crate) mod shape;
pub(crate) mod slider;
pub(crate) mod split_button;
pub(crate) mod split_view;
pub(crate) mod stack_panel;
pub(crate) mod surface_image_source;
pub(crate) mod swap_chain_panel;
pub(crate) mod tab_view;
pub(crate) mod teaching_tip;
pub(crate) mod text_block;
pub(crate) mod text_box;
pub(crate) mod time_picker;
pub(crate) mod title_bar;
pub(crate) mod toggle_button;
pub(crate) mod toggle_switch;
pub(crate) mod tree_view;
pub(crate) mod viewbox;

pub use auto_suggest_box::*;
pub use border::*;
pub use breadcrumb_bar::*;
pub use button::*;
pub use calendar_date_picker::*;
pub use calendar_view::*;
pub use canvas::*;
pub use check_box::*;
pub use color_picker::*;
pub use combo_box::*;
pub use command_bar::*;
pub use content_dialog::*;
pub use date_picker::*;
pub use drop_down_button::*;
pub use expander::*;
pub use flyout::*;
pub use grid::*;
pub use hyperlink_button::*;
pub use icon::*;
pub use image::*;
pub use info_badge::*;
pub use info_bar::*;
pub use list_box::*;
pub use menu_bar::*;
pub use navigation_view::*;
pub use number_box::*;
pub use password_box::*;
pub use person_picture::*;
pub use pivot::*;
pub use progress_bar::*;
pub use progress_ring::*;
pub use radio_button::*;
pub use radio_buttons::*;
pub use rating_control::*;
pub use relative_panel::*;
pub use repeat_button::*;
pub use rich_edit_box::*;
pub use scroll_view::*;
pub use scroll_viewer::*;
pub use selector_bar::*;
pub use shape::*;
pub use slider::*;
pub use split_button::*;
pub use split_view::*;
pub use stack_panel::*;
pub use surface_image_source::*;
pub use swap_chain_panel::*;
pub use tab_view::*;
pub use teaching_tip::*;
pub use text_block::*;
pub use text_box::*;
pub use time_picker::*;
pub use title_bar::*;
pub use toggle_button::*;
pub use toggle_switch::*;
pub use tree_view::*;
pub use viewbox::*;
