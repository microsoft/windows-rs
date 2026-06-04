//! Per-control typed handlers that replace scattered match arms in the
//! main `set_prop` dispatch. Each module exports `mount` and `diff` functions
//! that operate directly on the typed widget struct.

pub mod border;
pub mod stack_panel;
pub mod text_block;
