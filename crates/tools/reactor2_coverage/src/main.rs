use serde::Deserialize;
use std::path::{Component, Path, PathBuf};

#[derive(Deserialize)]
struct Report {
    data: Vec<ReportData>,
}

#[derive(Deserialize)]
struct ReportData {
    files: Vec<FileCoverage>,
}

#[derive(Deserialize)]
struct FileCoverage {
    filename: PathBuf,
    summary: CoverageSummary,
}

#[derive(Deserialize)]
struct CoverageSummary {
    branches: CoverageCount,
    lines: CoverageCount,
}

#[derive(Deserialize)]
struct CoverageCount {
    count: u64,
    covered: u64,
}

struct Requirement {
    name: &'static str,
    matches: fn(&Path) -> bool,
    branches: f64,
    lines: f64,
}

fn main() {
    let report_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "target/reactor2-coverage.json".to_string());
    let report: Report = serde_json::from_slice(&std::fs::read(report_path).unwrap()).unwrap();
    let source = std::env::current_dir()
        .unwrap()
        .join("crates/libs/reactor2/src");
    let files = &report.data.first().unwrap().files;
    let requirements = [
        Requirement {
            name: "app",
            matches: |path| module_matches(path, "app"),
            branches: 69.0,
            lines: 90.0,
        },
        Requirement {
            name: "arena",
            matches: |path| module_matches(path, "arena"),
            branches: 83.0,
            lines: 98.0,
        },
        Requirement {
            name: "element",
            matches: |path| module_matches(path, "element"),
            branches: 80.0,
            lines: 95.0,
        },
        Requirement {
            name: "engine",
            matches: |path| module_matches(path, "engine"),
            branches: 74.0,
            lines: 91.0,
        },
        Requirement {
            name: "runtime",
            matches: |path| path == Path::new("runtime.rs"),
            branches: 54.0,
            lines: 82.0,
        },
        Requirement {
            name: "winui",
            matches: |path| module_matches(path, "winui"),
            branches: 30.0,
            lines: 50.0,
        },
        Requirement {
            name: "winui host",
            matches: |path| path == Path::new("winui").join("host.rs"),
            branches: 55.0,
            lines: 66.0,
        },
    ];

    println!(
        "{:<10} {:>9} {:>10} {:>9} {:>10}",
        "Area", "Branches", "Required", "Lines", "Required"
    );

    let mut failed = false;
    for requirement in requirements {
        let mut branches = CoverageCount {
            count: 0,
            covered: 0,
        };
        let mut lines = CoverageCount {
            count: 0,
            covered: 0,
        };
        for file in files {
            let Ok(path) = file.filename.strip_prefix(&source) else {
                continue;
            };
            if path.components().any(|component| {
                component == Component::ParentDir || component.as_os_str() == "testing"
            }) {
                continue;
            }
            if (requirement.matches)(path) {
                branches.count += file.summary.branches.count;
                branches.covered += file.summary.branches.covered;
                lines.count += file.summary.lines.count;
                lines.covered += file.summary.lines.covered;
            }
        }
        assert!(
            branches.count != 0 && lines.count != 0,
            "coverage report does not contain {}",
            requirement.name
        );
        let branch_percent = percent(&branches);
        let line_percent = percent(&lines);
        failed |= branch_percent < requirement.branches || line_percent < requirement.lines;
        println!(
            "{:<10} {:>8.2}% {:>9.0}% {:>8.2}% {:>9.0}%",
            requirement.name, branch_percent, requirement.branches, line_percent, requirement.lines
        );
    }

    assert!(
        !failed,
        "windows-reactor2 coverage fell below its required floor."
    );
}

fn module_matches(path: &Path, name: &str) -> bool {
    path == Path::new(name).with_extension("rs") || path.starts_with(name)
}

fn percent(coverage: &CoverageCount) -> f64 {
    100.0 * coverage.covered as f64 / coverage.count as f64
}
