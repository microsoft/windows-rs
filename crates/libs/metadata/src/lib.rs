#![allow(dead_code)]

use std::collections::*;
mod common;
mod imp;
pub mod reader;
pub mod writer;

pub use common::*;
use imp::*;
use std::io::*;
use std::mem::*;
use std::ptr::*;
