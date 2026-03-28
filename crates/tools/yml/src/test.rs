use super::*;

pub fn yml() {
    write_yml(".github/workflows/test.yml", |yml| {
        let crates = helpers::crates("crates");
        let windows_crates: Vec<_> = crates
            .iter()
            .filter(|c| {
                !c.package
                    .metadata
                    .as_ref()
                    .and_then(|m| m.linux)
                    .unwrap_or(false)
            })
            .collect();
        let linux_crates: Vec<_> = crates
            .iter()
            .filter(|c| {
                c.package
                    .metadata
                    .as_ref()
                    .and_then(|m| m.linux)
                    .unwrap_or(false)
            })
            .collect();

        // This unrolling is required since "cargo test --all" consumes too much memory for the GitHub hosted runners
        // and the occasional "cargo clean" is required to avoid running out of disk space in the same runners.
        for (count, manifest) in windows_crates.iter().enumerate() {
            let name = &manifest.package.name;
            if count.is_multiple_of(50) {
                writeln!(
                    yml,
                    r"      - name: Clean
        run:  cargo clean"
                )
                .unwrap();
            }

            writeln!(
                yml,
                r"      - name: Test {name}
        run:  cargo test -p {name} --target ${{{{ matrix.target }}}}"
            )
            .unwrap();
        }

        writeln!(
            yml,
            r"      - name: Check diff
        shell: bash
        run: |
          git add -N .
          git diff --exit-code || (echo 'Tests changed code in the repo.'; exit 1)
  linux:
    runs-on: ubuntu-22.04
    steps:
      - name: Checkout
        uses: actions/checkout@v6
      - name: Update toolchain
        run: rustup update --no-self-update stable && rustup default stable"
        )
        .unwrap();

        for manifest in &linux_crates {
            let name = &manifest.package.name;
            writeln!(
                yml,
                r"      - name: Test {name}
        run:  cargo test -p {name} --target x86_64-unknown-linux-gnu"
            )
            .unwrap();
        }
    });
}
