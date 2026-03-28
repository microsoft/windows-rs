use super::*;

pub fn yml() {
    write_yml(".github/workflows/test.yml", |yml| {
        writeln!(
            yml,
            r"      - name: Test
        run:  cargo test --all --target ${{{{ matrix.target }}}}"
        )
        .unwrap();

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
