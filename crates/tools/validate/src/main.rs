//! Validates that the in-house `Windows.Win32` metadata (the flat corpus produced by
//! `tool_win32` + `tool_wdk`, bundled as `--in default`) generates Rust that compiles
//! in every `windows-bindgen` output style.
//!
//! Running the tool regenerates the three committed `bindings_*.rs` files; the `gen`
//! workflow fails on any diff, so a change in `windows-bindgen` or the metadata shows up
//! in review. The files are also declared as (unused) modules below, so simply building
//! `tool_validate` compiles the entire flat Win32 + Wdk surface — the compile is the
//! validation. WinRT is intentionally out of scope (it is owned by the sibling
//! `windows-*` crates and is already validated by them).

fn main() {
    let time = std::time::Instant::now();

    // The three styles are independent (distinct output files, no shared state),
    // so generate them in parallel to overlap the slow bindgen passes.
    let jobs: [&[&str]; 3] = [&["--out", "crates/tools/validate/src/bindings_default.rs", "--flat", "--filter", "Windows.Win32", "--rustfmt", "max_width=800,newline_style=Unix"], &["--out", "crates/tools/validate/src/bindings_sys.rs", "--flat", "--sys", "--filter", "Windows.Win32", "--rustfmt", "max_width=800,newline_style=Unix"], &["--out", "crates/tools/validate/src/bindings_minimal.rs", "--flat", "--minimal", "--filter", "Windows.Win32", "--rustfmt", "max_width=800,newline_style=Unix"]];

    windows_threading::for_each(jobs.into_iter(), |args| {
        windows_bindgen::bindgen(args.iter().copied());
    });

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}

#[allow(warnings)]
mod bindings_default;
#[allow(warnings)]
mod bindings_minimal;
#[allow(warnings)]
mod bindings_sys;
