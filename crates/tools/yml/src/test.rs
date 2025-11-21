use super::*;

pub fn yml() {
    write_yml(".github/workflows/test.yml", |yml| {
        // This unrolling is required since "cargo test --all" consumes too much memory for the GitHub hosted runners
        // and the occasional "cargo clean" is required to avoid running out of disk space in the same runners.
        for (count, manifest) in helpers::crates("crates").iter().enumerate() {
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
"
        )
        .unwrap();
    });
}
