//! Multi-architecture scrape orchestration for [`Clang::scrape`](crate::Clang::scrape).
//!
//! The builder holds the normal parse configuration. [`ScrapePlan`] carries the extra
//! orchestration state: arches, outputs, seed metadata, and reference winmds.

use crate::{Clang, clang_resource_dir};
use std::path::Path;
use windows_rdl::{ArchInput, merge_arch_rdl, reader};

/// Target architecture settings that differ between scrape passes.
pub struct Arch {
    /// Short name (`x64`, `arm64`, `x86`) and throwaway subdirectory name.
    pub name: String,
    /// The clang `--target` triple, e.g. `x86_64-pc-windows-msvc`.
    pub triple: String,
    /// `SupportedArchitecture` bitmask: 1 = X86, 2 = X64, 4 = Arm64.
    pub bits: i32,
    /// Extra `-D` defines for this architecture.
    pub defines: Vec<String>,
}

impl Arch {
    /// Known triple + `SupportedArchitecture` bit for a short arch name.
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

    /// Build the canonical-first arch list: `x64`, then non-`x64` extras.
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

/// Multi-arch and output state layered on top of a configured [`Clang`](crate::Clang).
pub struct ScrapePlan {
    /// Root namespace; each defining header becomes `<root>.<HeaderStem>`.
    pub root: String,
    /// Committed per-header RDL directory.
    pub rdl_dir: String,
    /// Scratch directory for per-arch throwaway RDL dirs and winmds.
    pub out_dir: String,
    /// Committed unified winmd output path.
    pub winmd: String,
    /// `archs[0]` writes `rdl_dir`; extras are folded in by arch-merge.
    pub archs: Vec<Arch>,
    /// Exclusion/reference winmds needed by both clang and RDL reader passes.
    pub reference_winmds: Vec<String>,
    /// Resolution-only winmds: they qualify external references but never exclude entities.
    pub resolution_winmds: Vec<String>,
    /// Optional hand-authored seed RDL, preserved across generated output clears.
    pub seed: Option<String>,
    /// Scrape architectures concurrently.
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

/// Prints timing details; callers add the output paths.
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

/// Find `name` in `dirs`, returning a forward-slashed path.
pub fn find_in_dirs(name: &str, dirs: &[String]) -> Option<String> {
    dirs.iter()
        .map(|dir| Path::new(dir).join(name))
        .find(|path| path.is_file())
        .map(|path| path.to_string_lossy().replace('\\', "/"))
}

struct Job<'a> {
    arch: &'a Arch,
    rdl_dir: String,
    winmd: String,
}

impl Clang {
    /// Run the plan, replaying this builder once per architecture and merging the results.
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
        let winmd_file = Path::new(&plan.winmd)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_else(|| panic!("`plan.winmd` has no file name: `{}`", plan.winmd));
        let stem = winmd_file.strip_suffix(".winmd").unwrap_or(winmd_file);
        let canonical_winmd = format!("{}/{winmd_file}", plan.out_dir);

        // The canonical arch writes committed RDL; extras write throwaway dirs.
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

        // Non-canonical arches need version-matched clang resource headers; resolve before
        // workers start so they do not race the one-time fetch.
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
            // Fold per-arch winmds back into defining-header partitions and rebuild the winmd.
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
            if let Some(seed) = &plan.seed {
                reader.input(seed);
            }
            for reference in &plan.reference_winmds {
                reader.reference(reference);
            }
            for resolution in &plan.resolution_winmds {
                reader.reference(resolution);
            }
            reader
                .output(&plan.winmd)
                .write()
                .unwrap_or_else(|e| panic!("failed to compile merged winmd `{}`: {e}", plan.winmd));
            winmd_wall = w.elapsed().as_secs_f32();
        } else {
            // Single arch: publish the canonical job's winmd.
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

    /// Scrape one architecture into `rdl_dir` and compile those partitions into `winmd`.
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
            clang.reference(reference);
        }
        for resolution in &plan.resolution_winmds {
            clang.resolution_input(resolution);
        }

        clang
            .namespace(&plan.root)
            .output(rdl_dir)
            .write_by_header()
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
            reader.reference(reference);
        }
        for resolution in &plan.resolution_winmds {
            reader.reference(resolution);
        }
        reader
            .output(winmd)
            .write()
            .unwrap_or_else(|e| panic!("failed to compile `{rdl_dir}` into `{winmd}`: {e}"));
    }
}

/// Remove stale generated `.rdl` partitions, preserving the seed file by name.
fn clear_rdl_dir(rdl_dir: &str, seed: Option<&str>) {
    std::fs::create_dir_all(rdl_dir)
        .unwrap_or_else(|e| panic!("failed to create `{rdl_dir}`: {e}"));
    let seed_name = seed.and_then(|s| Path::new(s).file_name());
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

/// Sorted, forward-slashed `.rdl` file paths in a directory.
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

/// Count committed partition files, excluding the seed.
fn count_partitions(rdl_dir: &str, seed: Option<&str>) -> usize {
    let seed_name = seed.and_then(|s| Path::new(s).file_name());
    std::fs::read_dir(rdl_dir).map_or(0, |rd| {
        rd.filter_map(|e| e.ok())
            .filter(|e| {
                let path = e.path();
                path.extension().is_some_and(|x| x == "rdl") && path.file_name() != seed_name
            })
            .count()
    })
}
