use std::path::Path;
use std::process::Command;

struct Snapshot {
    name: &'static str,
    path: &'static str,
    arguments: &'static [&'static str],
}

fn main() {
    let mut arguments = std::env::args().skip(1);
    let update = match (arguments.next().as_deref(), arguments.next()) {
        (None, None) => false,
        (Some("--update"), None) => true,
        _ => panic!("usage: tool_reactor2_public_api [--update]"),
    };
    let snapshots = [
        Snapshot {
            name: "default",
            path: "crates/libs/reactor2/testing/public-api.txt",
            arguments: &[],
        },
        Snapshot {
            name: "canvas",
            path: "crates/libs/reactor2/testing/public-api-canvas.txt",
            arguments: &["--features", "canvas"],
        },
        Snapshot {
            name: "webview",
            path: "crates/libs/reactor2/testing/public-api-webview.txt",
            arguments: &["--features", "webview"],
        },
        Snapshot {
            name: "all",
            path: "crates/libs/reactor2/testing/public-api-all.txt",
            arguments: &["--all-features"],
        },
    ];

    for snapshot in snapshots {
        check_snapshot(&snapshot, update);
    }
}

fn check_snapshot(snapshot: &Snapshot, update: bool) {
    let output = Command::new("cargo")
        .args([
            "public-api",
            "-p",
            "windows-reactor2",
            "-ss",
            "--color",
            "never",
        ])
        .args(snapshot.arguments)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "failed to generate the {} public API:\n{}",
        snapshot.name,
        String::from_utf8_lossy(&output.stderr)
    );

    let actual = String::from_utf8(output.stdout)
        .unwrap()
        .replace("windows_reactor2", "windows_reactor");
    let path = Path::new(snapshot.path);
    if update {
        std::fs::write(path, actual).unwrap();
        return;
    }

    let expected = std::fs::read_to_string(path).unwrap();
    assert_eq!(
        normalize_lines(&actual),
        normalize_lines(&expected),
        "{} public API differs from {}",
        snapshot.name,
        path.display()
    );
}

fn normalize_lines(value: &str) -> String {
    value.replace("\r\n", "\n")
}
