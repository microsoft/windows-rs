#![doc = include_str!("../readme.md")]
#![allow(non_upper_case_globals, clippy::enum_variant_names)]
// TODO: remove this once bindgen2 is up and running
#![allow(dead_code)]

mod winmd;

use std::cmp::Ordering;
use std::collections::*;
