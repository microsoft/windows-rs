use super::*;

pub fn yml() {
    write_yml(".github/workflows/no-default-features.yml", |yml| {
        for manifest in helpers::crates("crates/libs") {
            let name = manifest.package.name;
            writeln!(
                yml,
                r"      - name: Check {name}
        run:  cargo check -p {name} --no-default-features"
            )
            .unwrap();
        }
    });
}
