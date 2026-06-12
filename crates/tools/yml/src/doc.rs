use super::*;

pub fn yml() {
    write_yml(".github/workflows/doc.yml", |yml| {
        let crates: Vec<_> = helpers::crates("crates/libs")
            .into_iter()
            .filter(|c| {
                c.package.publish != Some(false)
                    && c.package.name != "windows"
                    && c.package.name != "windows-link"
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
    });
}
