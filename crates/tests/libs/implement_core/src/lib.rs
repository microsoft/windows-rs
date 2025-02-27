//! This contains tests that can be performed without compiling the `windows`
//! crate. This allows for faster test iteration.

#![cfg(test)]

mod base_class;
mod com_chain;
mod com_object;
mod static_com_object;
