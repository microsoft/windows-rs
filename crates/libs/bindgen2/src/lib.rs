#![doc = include_str!("../readme.md")]
#![allow(non_upper_case_globals, clippy::enum_variant_names)]
// TODO: remove this once bindgen2 is up and running
#![allow(dead_code)]

mod args;
mod io;
mod panic;
mod winmd;
mod writer;

use panic::panic;
use std::cmp::Ordering;
use std::collections::*;

enum ArgKind {
    None,
    Input,
    Output,
    Filter,
    Config,
}

/// The Windows code generator.
pub fn bindgen<I, S>(args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let args = args::expand(args);
    let mut kind = ArgKind::None;
    let mut output = None;
    let mut input = Vec::new();
    let mut include = Vec::new();
    let mut exclude = Vec::new();
    let mut config = HashMap::new();

    for arg in &args {
        if arg.starts_with('-') {
            kind = ArgKind::None;
        }

        match kind {
            ArgKind::None => match arg.as_str() {
                "--in" => kind = ArgKind::Input,
                "--out" => kind = ArgKind::Output,
                "--filter" => kind = ArgKind::Filter,
                "--config" => kind = ArgKind::Config,
                _ => panic("invalid option"),
            },
            ArgKind::Output => {
                if output.is_none() {
                    output = Some(arg.as_str());
                } else {
                    panic("too many outputs");
                }
            }
            ArgKind::Input => input.push(arg.as_str()),
            ArgKind::Filter => {
                if let Some(rest) = arg.strip_prefix('!') {
                    exclude.push(rest);
                } else {
                    include.push(arg.as_str());
                }
            }
            ArgKind::Config => {
                if let Some((key, value)) = arg.split_once('=') {
                    config.insert(key, value);
                } else {
                    config.insert(arg, "");
                }
            }
        }
    }
}
