use super::*;

pub fn yml() {
    write_yml(".github/workflows/doc.yml", |yml| {
        let crates: Vec<_> = helpers::crates("crates/libs")
            .into_iter()
            .filter(|c| {
                c.package.publish != Some(false)
                    && c.package.name != "windows"
                    && c.package.name != "windows-link"
                    // `windows-reactor` pulls in the `reactor` composition stack, so
                    // documenting `windows-composition` (system) in the same
                    // invocation would enable both mutually exclusive stacks at
                    // once. Document it separately below.
                    && c.package.name != "windows-composition"
            })
            .collect();

        let names: Vec<_> = crates.iter().map(|c| c.package.name.as_str()).collect();
        let line = format!(
            "      - name: Check\n        run: cargo doc --no-deps {}\n",
            names
                .iter()
                .map(|n| format!("-p {n}"))
                .collect::<Vec<_>>()
                .join(" ")
        );
        yml.push_str(&line);

        // The `system` and `reactor` stacks are mutually exclusive, so document each
        // on its own rather than alongside the `reactor` consumers above.
        yml.push_str(
            "      - name: Check windows-composition\n        run: cargo doc --no-deps -p windows-composition\n",
        );
        yml.push_str(
            "      - name: Check windows-composition (reactor)\n        run: cargo doc --no-deps -p windows-composition --no-default-features --features reactor\n",
        );
        yml.push_str(
            "      - name: Check windows-canvas (composition)\n        run: cargo doc --no-deps -p windows-canvas --features composition\n",
        );
        yml.push_str(
            "      - name: Check reactor features\n        run: cargo doc --no-deps -p windows-reactor -p windows-webview --features windows-reactor/canvas,windows-webview/reactor\n",
        );
    });
}
