const FILES: &str = include_str!("../assets/runtime.txt");

pub fn files(arch: &str) -> impl Iterator<Item = &'static str> + '_ {
    assert!(
        matches!(arch, "x86" | "x64" | "arm64"),
        "unsupported architecture: {arch}"
    );
    FILES.lines().filter_map(move |line| {
        let mut fields = line.split_whitespace();
        let name = fields.next()?;
        let mut restricted = false;
        let mut selected = false;
        for target in fields {
            assert!(
                matches!(target, "x86" | "x64" | "arm64"),
                "invalid architecture `{target}` for runtime entry `{name}`"
            );
            restricted = true;
            selected |= target == arch;
        }
        (!restricted || selected).then_some(name)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn architecture_specific_files() {
        let x64: Vec<_> = files("x64").collect();
        assert_eq!(x64, files("arm64").collect::<Vec<_>>());
        let x86: Vec<_> = files("x86").collect();
        assert!(x86.iter().all(|name| x64.contains(name)));
        assert_eq!(
            x64.into_iter()
                .filter(|name| !x86.contains(name))
                .collect::<Vec<_>>(),
            [
                "microsoft.graphics.imaging.dll",
                "sessionhandleipcproxystub.dll"
            ]
        );
    }
}
