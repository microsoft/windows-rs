#![warn(unused_qualifications)]

#[expect(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    clippy::upper_case_acronyms,
)]
mod bindings;
mod manager;
mod storyboard;
mod transition;
mod variable;

use bindings::*;
use windows_core::Interface;

pub use manager::Manager;
pub use storyboard::{Keyframe, Storyboard};
pub use transition::{Transition, TransitionLibrary};
pub use variable::Variable;
pub use windows_core::Result;
