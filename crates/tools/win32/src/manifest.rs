use serde::Deserialize;

/// Top-level manifest structure parsed from a `win32.toml` file.
#[derive(Deserialize)]
pub struct Manifest {
    /// Path to a previously compiled `.winmd` file used as a reference for
    /// cross-namespace type resolution.  When provided it is passed to both
    /// `Clang::input()` and `Reader::input()` for every namespace entry.
    pub reference: Option<String>,

    /// Clang arguments applied to every namespace entry.  Per-namespace
    /// `args` are appended after these.
    #[serde(default)]
    pub args: Vec<String>,

    /// One entry per Win32 namespace to compile.
    #[serde(default)]
    pub namespace: Vec<Namespace>,
}

/// A single namespace entry inside the manifest.
#[derive(Deserialize)]
pub struct Namespace {
    /// Dot-separated namespace name, e.g. `Windows.Win32.Foundation`.
    pub name: String,

    /// DLL name (without extension) used as the library for all functions in
    /// this namespace, e.g. `kernel32`.
    pub library: String,

    /// SDK header paths to feed to libclang.
    pub headers: Vec<String>,

    /// Path-suffix filters passed to `Clang::filter()`.  When non-empty only
    /// declarations originating from headers whose path ends with one of these
    /// suffixes are emitted.  When absent every declaration from the listed
    /// headers is emitted.
    #[serde(default)]
    pub filters: Vec<String>,

    /// Extra clang arguments merged with the top-level `args` for this
    /// namespace only.
    #[serde(default)]
    pub args: Vec<String>,
}
