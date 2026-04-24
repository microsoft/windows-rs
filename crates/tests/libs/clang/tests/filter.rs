#![cfg(target_pointer_width = "64")]

use windows_rdl::*;

/// Verify that `.filter()` restricts RDL output to declarations from the
/// specified header suffixes.  Types from the dependency header that are
/// *used* by the filtered headers must still appear (pulled in transitively),
/// while types that are only defined in the dependency header and never
/// referenced must not appear.
#[test]
fn filter_included_headers() {
    let rdl = "tests/filter.rdl";

    clang()
        .args([
            "-x",
            "c++",
            "--target=x86_64-pc-windows-msvc",
            "-fms-extensions",
            "-Itests",
        ])
        .input_str(
            r#"
#include "filter_dep.h"
#include "filter_api1.h"
#include "filter_api2.h"
"#,
        )
        .filter("filter_api1.h")
        .filter("filter_api2.h")
        .output(rdl)
        .namespace("Test")
        .write()
        .unwrap();
}
