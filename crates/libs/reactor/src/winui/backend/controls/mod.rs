//! Per-control typed handlers that replace scattered match arms in the
//! main `set_prop` dispatch. Each module exports `mount` and `diff` functions
//! that operate directly on the typed widget struct.

pub mod border;
pub mod check_box;
pub mod grid;
pub mod number_box;
pub mod progress_bar;
pub mod progress_ring;
pub mod radio_button;
pub mod scroll_viewer;
pub mod slider;
pub mod stack_panel;
pub mod text_block;
pub mod text_box;
pub mod toggle_switch;
