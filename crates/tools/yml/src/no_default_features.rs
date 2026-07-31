use super::*;

pub fn yml() {
    write_yml(".github/workflows/no-default-features.yml", |yml| {
        for manifest in helpers::crates("crates/libs") {
            let name = manifest.package.name;
            if name == "windows-composition" {
                // No default-free build exists: exactly one of the mutually
                // exclusive `system`/`reactor` stacks must be selected. Check the
                // reactor stack, which is the one consumers opt into explicitly.
                writeln!(
                    yml,
                    r"      - name: Check {name}
        run:  cargo check -p {name} --no-default-features --features reactor"
                )
                .unwrap();
                continue;
            }
            writeln!(
                yml,
                r"      - name: Check {name}
        run:  cargo check -p {name} --no-default-features"
            )
            .unwrap();
        }
    });
}
