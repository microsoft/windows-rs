use super::*;

pub struct Config {
    pub types: TypeMap,
    pub references: References,
    pub output: String,
    pub flat: bool,
    pub no_allow: bool,
    pub no_comment: bool,
    pub no_deps: bool,
    pub no_toml: bool,
    pub package: bool,
    pub rustfmt: String,
    pub sys: bool,
    pub implement: bool,
    pub derive: Derive,
    pub link: String,
    pub warnings: WarningBuilder,
}
