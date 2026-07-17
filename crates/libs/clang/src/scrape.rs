//! The high-level, multi-architecture corpus scrape — the [`Clang::scrape`](crate::Clang::scrape)
//! terminal on the low-level [`clang()`](crate::clang) builder.
//!
//! Where [`clang().write()`](crate::Clang::write) parses one translation unit for one architecture,
//! `scrape` replays a configured [`Clang`](crate::Clang) builder once per architecture (swapping the
//! target triple and per-arch defines), routes every declaration to its defining-header partition,
//! arch-merges the per-arch results so a symbol that exists on (or differs across) only a subset of
//! arches gains `SupportedArchitecture`, and re-derives a single unified winmd from the merged corpus.
//!
//! Everything that differs between scrapers is configured on the `Clang` builder — the headers, the
//! include/lib directories, the import libraries, the reachability scope — exactly as a single-arch
//! scrape would set them. Only the multi-arch orchestration state that is *not* expressible on the
//! builder — the architectures, the output directories, an optional metadata seed, and the reference
//! winmds an *additive* scrape (such as the WDK) resolves against — is captured in [`ScrapePlan`].
//! Toolchain provisioning (which NuGet packages and versions supply the headers and libs) stays in
//! each tool, because that is genuinely per-scraper.

use crate::{Clang, clang_resource_dir};
use windows_rdl::{ArchInput, merge_arch_rdl, reader};

/// A target architecture: the clang triple, the `SupportedArchitecture` bitmask, and any extra
/// preprocessor defines the headers need for this target (the kernel-mode headers, for example,
/// require `_AMD64_`/`_ARM64_` in place of the `windows.h` closure that would otherwise set them).
pub struct Arch {
    /// Short name (`x64`, `arm64`, `x86`); also the throwaway sub-directory name for non-canonical arches.
    pub name: String,
    /// The clang `--target` triple, e.g. `x86_64-pc-windows-msvc`.
    pub triple: String,
    /// `SupportedArchitecture` bitmask: 1 = X86, 2 = X64, 4 = Arm64.
    pub bits: i32,
    /// Extra `-D` defines for this architecture (empty for scrapes that get their arch macros
    /// from a `windows.h` prelude).
    pub defines: Vec<String>,
}

impl Arch {
    /// The known triple + `SupportedArchitecture` bit for a short arch name, with no extra
    /// defines. Returns `None` for an unrecognized name so callers can validate a manifest.
    pub fn known(name: &str) -> Option<Self> {
        let (triple, bits) = match name {
            "x64" => ("x86_64-pc-windows-msvc", 2),
            "arm64" => ("aarch64-pc-windows-msvc", 4),
            "x86" => ("i686-pc-windows-msvc", 1),
            _ => return None,
        };
        Some(Self {
            name: name.to_string(),
            triple: triple.to_string(),
            bits,
            defines: Vec::new(),
        })
    }

    /// Builds the canonical-first arch list a manifest-driven scrape runs: `x64` (always the
    /// canonical arch that writes the committed corpus) followed by each `extra` name, skipping
    /// a redundant `"x64"`. Each arch — including the canonical one — is constructed by `build`,
    /// so a scraper that needs per-target preprocessor defines supplies them there.
    pub fn canonical_plus(extra: &[String], build: impl Fn(&str) -> Self) -> Vec<Self> {
        let mut archs = vec![build("x64")];
        for name in extra {
            if name != "x64" {
                archs.push(build(name));
            }
        }
        archs
    }
}

/// The orchestration plan for a [`Clang::scrape`](crate::Clang::scrape): the multi-arch and output
/// state that is *not* expressed on the [`Clang`](crate::Clang) builder itself. Every parse knob
/// (headers, args, reachability scope, import libraries) is configured on the builder as for a
/// single-arch scrape; this carries only what the corpus pipeline layers on top. All paths are final
/// (NuGet/SDK resolution has already happened in the caller).
pub struct ScrapePlan {
    /// Root namespace; each emitted defining header becomes `<root>.<HeaderStem>` in a flat layout.
    pub root: String,
    /// The committed per-header RDL directory (e.g. `metadata/win32`). The canonical (first)
    /// architecture writes its partitions here; the arch-merge writes the unified corpus here too.
    pub rdl_dir: String,
    /// A `target/`-side scratch directory for the per-arch throwaway RDL dirs and winmds.
    pub out_dir: String,
    /// The committed unified winmd output path (e.g. `crates/libs/bindgen/default/Windows.Win32.winmd`).
    pub winmd: String,
    /// Architectures to scrape. `archs[0]` is canonical: it writes the committed `rdl_dir`; every
    /// other arch is scraped to a throwaway dir under `out_dir` and folded in via arch-merge.
    pub archs: Vec<Arch>,
    /// Reference winmds applied to the per-arch clang parse (as a scrape-time *exclusion* reference so
    /// already-defined types are skipped) and to every RDL reader pass (so scraped types resolve their
    /// dependencies by bare name). Empty for a base scrape; set for an additive scrape like the WDK.
    /// These live on the plan rather than on [`Clang::input`](crate::Clang::input) because the reader
    /// passes need them too, not only the clang parse.
    pub reference_winmds: Vec<String>,
    /// Resolution winmds (e.g. `Windows.winmd`) applied to the per-arch clang parse via
    /// [`Clang::resolution_input`](crate::Clang::resolution_input) — classifying declarations in
    /// the `ABI::Windows::*` C++/WinRT projection namespace — and to every RDL reader pass, so a
    /// captured interop API's cross-winmd reference to a true WinRT type (`Windows.Foundation.*`)
    /// resolves as an external assembly reference. Unlike [`reference_winmds`](Self::reference_winmds)
    /// these are never an exclusion base: no entity they define is skipped or emitted. Empty for a
    /// scrape that reaches no ABI interop surface (the default).
    pub resolution_winmds: Vec<String>,
    /// Optional hand-authored metadata seed RDL (full path, living inside `rdl_dir`). Preserved
    /// across the `rdl_dir` clear, added to every reader pass, and fed to the arch-merge.
    pub seed: Option<String>,
    /// Scrape the architectures concurrently (large SDK translation units are clang-bound).
    pub parallel: bool,
}

/// What [`Clang::scrape`](crate::Clang::scrape) produced, for the caller's summary output.
pub struct Summary {
    /// Committed partition (`.rdl`) files in `rdl_dir`, excluding the seed.
    pub partitions: usize,
    /// Per-arch scrape wall-clock times, in `archs` order.
    pub arch_timings: Vec<(String, f32)>,
    /// Wall-clock time of the (possibly parallel) scrape phase.
    pub scrape_wall: f32,
    /// Arch-merge time (0.0 for a single-arch scrape).
    pub merge_wall: f32,
    /// Final unified-winmd derivation time (0.0 for a single-arch scrape).
    pub winmd_wall: f32,
    /// Whether more than one architecture was scraped and merged.
    pub multi_arch: bool,
}

/// Prints the per-arch timings, the parallel-scrape wall time, and (for a multi-arch scrape) the
/// arch-merge and unified-winmd times. Callers add their own trailing line naming the outputs.
impl std::fmt::Display for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Timing:")?;
        for (name, secs) in &self.arch_timings {
            writeln!(f, "  scrape {name:<6}         {secs:>8.2}s")?;
        }
        writeln!(f, "  scrape (parallel wall) {:>8.2}s", self.scrape_wall)?;
        if self.multi_arch {
            let arches: Vec<&str> = self.arch_timings.iter().map(|(n, _)| n.as_str()).collect();
            writeln!(f, "Arch-merged: {}", arches.join(" + "))?;
            writeln!(f, "  arch-merge             {:>8.2}s", self.merge_wall)?;
            writeln!(f, "  final winmd            {:>8.2}s", self.winmd_wall)?;
        }
        Ok(())
    }
}

/// Finds `name` in the first of `dirs` that contains it, returning the forward-slashed path. Used
/// to resolve a bare import-library name against a scraper's pinned SDK/WDK lib directories before
/// passing it to [`Clang::import_library`](crate::Clang::import_library).
pub fn find_in_dirs(name: &str, dirs: &[String]) -> Option<String> {
    dirs.iter()
        .map(|dir| std::path::Path::new(dir).join(name))
        .find(|path| path.is_file())
        .map(|path| path.to_string_lossy().replace('\\', "/"))
}

struct Job<'a> {
    arch: &'a Arch,
    rdl_dir: String,
    winmd: String,
}

impl Clang {
    /// Runs the full multi-architecture corpus scrape described by `plan`, replaying this configured
    /// builder once per architecture, and returns a [`Summary`]. Panics with a descriptive message on
    /// any failure — these are unrecoverable build errors, matching the fail-loud behaviour of the
    /// tools this drives.
    ///
    /// The builder carries the arch-invariant parse configuration: the translation-unit sources
    /// ([`input_str`](Self::input_str)), the compiler [`args`](Self::args), the reachability
    /// [`scope`](Self::scope)/[`scope_headers`](Self::scope_headers), the
    /// [`import_library`](Self::import_library) mappings, and [`drop_lib_less`](Self::drop_lib_less).
    /// `scrape` sets the per-arch [`target`](Self::target), its defines, and (for a multi-arch run)
    /// the builtin `-resource-dir` itself, so any [`target`](Self::target) set on the builder is
    /// overridden. The reference winmds live on [`ScrapePlan`] rather than on [`input`](Self::input)
    /// so the RDL reader passes see them too.
    pub fn scrape(&self, plan: &ScrapePlan) -> Summary {
        assert!(
            !plan.archs.is_empty(),
            "scraper: `plan.archs` must list at least one architecture"
        );

        std::fs::create_dir_all(&plan.out_dir)
            .unwrap_or_else(|e| panic!("failed to create `{}`: {e}", plan.out_dir));
        std::fs::create_dir_all(&plan.rdl_dir)
            .unwrap_or_else(|e| panic!("failed to create `{}`: {e}", plan.rdl_dir));

        let canonical = &plan.archs[0];
        let winmd_file = std::path::Path::new(&plan.winmd)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_else(|| panic!("`plan.winmd` has no file name: `{}`", plan.winmd));
        let stem = winmd_file.strip_suffix(".winmd").unwrap_or(winmd_file);
        let canonical_winmd = format!("{}/{winmd_file}", plan.out_dir);

        // The canonical arch writes the committed RDL dir; extras write throwaway dirs under out_dir.
        let mut jobs = vec![Job {
            arch: canonical,
            rdl_dir: plan.rdl_dir.clone(),
            winmd: canonical_winmd.clone(),
        }];
        for arch in &plan.archs[1..] {
            jobs.push(Job {
                arch,
                rdl_dir: format!("{}/{}", plan.out_dir, arch.name),
                winmd: format!("{}/{stem}.{}.winmd", plan.out_dir, arch.name),
            });
        }
        let multi_arch = jobs.len() > 1;

        // Non-canonical arches need clang's version-matched builtin resource headers so the target's
        // `intrin.h` reconciles with its compiler builtins. Resolved (and fetched on first use) once,
        // before any concurrency, so parallel workers never race the one-time download.
        let resource_dir = multi_arch.then(clang_resource_dir);

        let timings = std::sync::Mutex::new(Vec::<(String, f32)>::new());
        let scrape_start = std::time::Instant::now();
        let scrape_one = |job: &Job| {
            let t = std::time::Instant::now();
            let resource = (job.arch.bits != canonical.bits).then(|| {
                resource_dir
                    .as_deref()
                    .expect("resource dir resolved for multi-arch")
            });
            self.scrape_arch(plan, job.arch, &job.rdl_dir, &job.winmd, resource);
            timings
                .lock()
                .unwrap()
                .push((job.arch.name.clone(), t.elapsed().as_secs_f32()));
        };
        if plan.parallel {
            windows_threading::for_each(jobs.iter(), scrape_one);
        } else {
            jobs.iter().for_each(scrape_one);
        }
        let scrape_wall = scrape_start.elapsed().as_secs_f32();

        let mut merge_wall = 0.0;
        let mut winmd_wall = 0.0;
        if multi_arch {
            // Fold every per-arch winmd together so subset/divergent symbols gain
            // `SupportedArchitecture`, route each item back to its defining-header `<stem>.rdl`, and
            // re-derive the unified winmd from the merged corpus (the source of truth).
            let arch_inputs: Vec<ArchInput> = jobs
                .iter()
                .map(|j| ArchInput {
                    rdl_dir: j.rdl_dir.clone(),
                    winmd: j.winmd.clone(),
                    bits: j.arch.bits,
                })
                .collect();
            let m = std::time::Instant::now();
            merge_arch_rdl(&arch_inputs, plan.seed.as_deref(), &plan.rdl_dir)
                .unwrap_or_else(|e| panic!("arch-merge failed: {e}"));
            merge_wall = m.elapsed().as_secs_f32();

            let w = std::time::Instant::now();
            let mut reader = reader();
            reader.input(&plan.rdl_dir);
            for reference in &plan.reference_winmds {
                reader.input(reference);
            }
            for resolution in &plan.resolution_winmds {
                reader.input(resolution);
            }
            reader
                .output(&plan.winmd)
                .write()
                .unwrap_or_else(|e| panic!("failed to compile merged winmd `{}`: {e}", plan.winmd));
            winmd_wall = w.elapsed().as_secs_f32();
        } else {
            // Single arch: the canonical job's winmd is already the final output — publish it.
            std::fs::copy(&canonical_winmd, &plan.winmd)
                .unwrap_or_else(|e| panic!("failed to publish winmd to `{}`: {e}", plan.winmd));
        }

        let mut arch_timings = timings.into_inner().unwrap();
        arch_timings.sort_by_key(|(name, _)| {
            plan.archs
                .iter()
                .position(|a| a.name == *name)
                .unwrap_or(usize::MAX)
        });

        Summary {
            partitions: count_partitions(&plan.rdl_dir, plan.seed.as_deref()),
            arch_timings,
            scrape_wall,
            merge_wall,
            winmd_wall,
            multi_arch,
        }
    }

    /// Scrapes one architecture's translation units into `rdl_dir` (clearing stale partitions first,
    /// preserving the seed) and compiles those partitions into `winmd`. Clones the configured builder
    /// and layers on this arch's target, defines, optional builtin resource directory, and the plan's
    /// reference winmds.
    fn scrape_arch(
        &self,
        plan: &ScrapePlan,
        arch: &Arch,
        rdl_dir: &str,
        winmd: &str,
        resource_dir: Option<&str>,
    ) {
        clear_rdl_dir(rdl_dir, plan.seed.as_deref());

        let mut clang = self.clone();
        clang
            .target(&arch.triple)
            .args(arch.defines.iter().map(String::as_str));
        if let Some(dir) = resource_dir {
            clang.args(["-resource-dir", dir]);
        }
        for reference in &plan.reference_winmds {
            clang.input(reference);
        }
        for resolution in &plan.resolution_winmds {
            clang.resolution_input(resolution);
        }

        clang
            .write_by_header(&plan.root, &[], rdl_dir)
            .unwrap_or_else(|e| panic!("failed to generate partitions in `{rdl_dir}`: {e}"));

        let mut rdl_paths = collect_rdl_paths(rdl_dir);
        if let Some(seed) = &plan.seed
            && !rdl_paths.iter().any(|p| p == seed)
        {
            rdl_paths.push(seed.clone());
        }

        let mut reader = reader();
        reader.inputs(&rdl_paths);
        for reference in &plan.reference_winmds {
            reader.input(reference);
        }
        for resolution in &plan.resolution_winmds {
            reader.input(resolution);
        }
        reader
            .output(winmd)
            .write()
            .unwrap_or_else(|e| panic!("failed to compile `{rdl_dir}` into `{winmd}`: {e}"));
    }
}

/// Removes stale `.rdl` partitions from `rdl_dir`, preserving the seed file (matched by name).
fn clear_rdl_dir(rdl_dir: &str, seed: Option<&str>) {
    std::fs::create_dir_all(rdl_dir)
        .unwrap_or_else(|e| panic!("failed to create `{rdl_dir}`: {e}"));
    let seed_name = seed.and_then(|s| std::path::Path::new(s).file_name());
    for entry in
        std::fs::read_dir(rdl_dir).unwrap_or_else(|e| panic!("failed to read `{rdl_dir}`: {e}"))
    {
        let path = entry.unwrap().path();
        let is_seed = path.file_name() == seed_name;
        if !is_seed && path.extension().is_some_and(|x| x == "rdl") {
            std::fs::remove_file(&path)
                .unwrap_or_else(|e| panic!("failed to remove `{}`: {e}", path.display()));
        }
    }
}

/// The sorted, forward-slashed `.rdl` file paths in a directory.
fn collect_rdl_paths(rdl_dir: &str) -> Vec<String> {
    let mut paths: Vec<String> = std::fs::read_dir(rdl_dir)
        .unwrap_or_else(|e| panic!("failed to read `{rdl_dir}`: {e}"))
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|x| x == "rdl"))
        .map(|path| path.to_string_lossy().replace('\\', "/"))
        .collect();
    paths.sort();
    paths
}

/// Counts committed partition files in `rdl_dir`, excluding the seed.
fn count_partitions(rdl_dir: &str, seed: Option<&str>) -> usize {
    let seed_name = seed.and_then(|s| std::path::Path::new(s).file_name());
    std::fs::read_dir(rdl_dir).map_or(0, |rd| {
        rd.filter_map(|e| e.ok())
            .filter(|e| {
                let path = e.path();
                path.extension().is_some_and(|x| x == "rdl") && path.file_name() != seed_name
            })
            .count()
    })
}
