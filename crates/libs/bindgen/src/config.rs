use super::*;

use std::sync::atomic::{AtomicPtr, Ordering};

static CONFIG: AtomicPtr<Config> = AtomicPtr::new(std::ptr::null_mut());

pub fn config() -> &'static Config {
    let ptr = CONFIG.load(Ordering::Relaxed);

    if ptr.is_null() {
        panic!();
    } else {
        unsafe { &*ptr }
    }
}

pub struct Config {
    pub types: TypeMap,
    pub references: References,
    pub output: String,
    pub flat: bool,
    pub no_allow: bool,
    pub no_comment: bool,
    pub no_core: bool,
    pub no_toml: bool,
    pub package: bool,
    pub rustfmt: String,
    pub sys: bool,
    pub implement: bool,
    pub derive: Derive,
}

impl Config {
    pub fn init(value: Self) {
        let config = Box::leak(Box::new(value));
        CONFIG.store(config, Ordering::Relaxed);
    }
}
