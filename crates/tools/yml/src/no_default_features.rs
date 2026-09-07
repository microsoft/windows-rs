use super::*;

pub fn yml() {
    write_yml(".github/workflows/no-default-features.yml", |yml| {
        for manifest in helpers::crates("crates/libs") {
            let name = manifest.package.name;
            if name == "windows-composition" {
                // No stack-free build exists. Check the reactor stack, which is
                // the one consumers opt into explicitly.
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
