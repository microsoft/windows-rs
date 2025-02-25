mod clippy;
mod msrv;
mod no_default_features;
mod test;

use std::fmt::Write;

fn main() {
    test::yml();
    clippy::yml();
    no_default_features::yml();
    msrv::yml();
}
