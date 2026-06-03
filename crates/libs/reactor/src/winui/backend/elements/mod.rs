//! Per-element backend dispatch modules.
//!
//! Each submodule handles `set_prop` (and eventually `attach_event`) for a
//! single element type. This splits the monolithic dispatch in `mod.rs` into
//! small, focused, independently-compilable units.

#[macro_use]
mod macros;

pub(super) mod auto_suggest_box;
pub(super) mod border;
pub(super) mod breadcrumb_bar;
pub(super) mod button;
pub(super) mod calendar_date_picker;
pub(super) mod calendar_view;
pub(super) mod canvas;
pub(super) mod check_box;
pub(super) mod color_picker;
pub(super) mod combo_box;
pub(super) mod command_bar;
pub(super) mod content_dialog;
pub(super) mod date_picker;
pub(super) mod drop_down_button;
pub(super) mod ellipse;
pub(super) mod expander;
pub(super) mod grid;
pub(super) mod hyperlink_button;
pub(super) mod image;
pub(super) mod info_badge;
pub(super) mod info_bar;
pub(super) mod line;
pub(super) mod list_box;
pub(super) mod navigation_view;
pub(super) mod number_box;
pub(super) mod password_box;
pub(super) mod person_picture;
pub(super) mod pivot;
pub(super) mod pivot_item;
pub(super) mod progress_bar;
pub(super) mod progress_ring;
pub(super) mod radio_button;
pub(super) mod radio_buttons;
pub(super) mod rating_control;
pub(super) mod rectangle;
pub(super) mod repeat_button;
pub(super) mod rich_edit_box;
pub(super) mod rich_text_block;
pub(super) mod scroll_view;
pub(super) mod scroll_viewer;
pub(super) mod slider;
pub(super) mod split_button;
pub(super) mod split_view;
pub(super) mod stack_panel;
pub(super) mod tab_view;
pub(super) mod tab_view_item;
pub(super) mod teaching_tip;
pub(super) mod text_block;
pub(super) mod text_box;
pub(super) mod time_picker;
pub(super) mod title_bar;
pub(super) mod toggle_button;
pub(super) mod toggle_switch;
pub(super) mod tree_view;
pub(super) mod viewbox;
