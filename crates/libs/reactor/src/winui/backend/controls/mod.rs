//! Per-control typed handlers that replace scattered match arms in the
//! main `set_prop` dispatch. Each module exports `mount` and `diff` functions
//! that operate directly on the typed widget struct.

pub mod auto_suggest_box;
pub mod border;
pub mod breadcrumb_bar;
pub mod calendar_date_picker;
pub mod calendar_view;
pub mod check_box;
pub mod color_picker;
pub mod combo_box;
pub mod date_picker;
pub mod expander;
pub mod grid;
pub mod hyperlink_button;
pub mod image;
pub mod info_badge;
pub mod info_bar;
pub mod list_box;
pub mod number_box;
pub mod password_box;
pub mod person_picture;
pub mod pivot;
pub mod progress_bar;
pub mod progress_ring;
pub mod radio_button;
pub mod radio_buttons;
pub mod rating_control;
pub mod repeat_button;
pub mod rich_edit_box;
pub mod scroll_view;
pub mod scroll_viewer;
pub mod selector_bar;
pub mod shape;
pub mod slider;
pub mod split_button;
pub mod split_view;
pub mod stack_panel;
pub mod teaching_tip;
pub mod text_block;
pub mod text_box;
pub mod time_picker;
pub mod toggle_button;
pub mod toggle_switch;
pub mod tree_view;
pub mod viewbox;
