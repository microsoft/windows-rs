pub mod common;
use common::run;

include!(concat!(env!("OUT_DIR"), "/tests.rs"));
