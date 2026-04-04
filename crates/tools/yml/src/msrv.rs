use super::*;

pub fn yml() {
    write_yml(".github/workflows/msrv.yml", |yml| {
        for manifest in helpers::crates("crates/libs") {
            let name = manifest.package.name;
            let version = manifest.package.rust_version.expect("rust-version");

            writeln!(
                yml,
                "          - crate: {name}\n            version: \"{version}\""
            )
            .unwrap();
        }

        yml.push_str(
            r#"    steps:
      - name: Checkout
        uses: actions/checkout@v6
      - name: Cache rustup
        uses: actions/cache@v4
        with:
          path: ~/.rustup/toolchains
          key: rustup-${{ runner.os }}-${{ matrix.version }}
      - name: Update toolchain
        run: rustup update --no-self-update ${{ matrix.version }} && rustup default ${{ matrix.version }}
      - name: Check
        run:  cargo check -p ${{ matrix.crate }} --all-features
"#,
        );
    });
}
