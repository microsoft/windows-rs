#![allow(dead_code)]

use std::collections::*;
mod flags;
mod imp;
pub mod reader;
pub mod writer;

pub use flags::*;
use imp::*;
use std::io::*;
use std::mem::*;
use std::ptr::*;
