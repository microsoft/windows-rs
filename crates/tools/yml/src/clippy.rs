use super::*;

pub fn yml() {
    write_yml(".github/workflows/clippy.yml", |yml| {
        // This unrolling is required since "cargo clippy --all" consumes too much memory for the GitHub hosted runners.
        for manifest in helpers::crates("crates") {
            let name = manifest.package.name;

            writeln!(
                yml,
                r"      - name: Check {name}
        run:  cargo clippy -p {name} --tests"
            )
            .unwrap();
        }
    });
}
