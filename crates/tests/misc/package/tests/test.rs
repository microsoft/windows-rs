#[test]
fn test() {
    for toml in helpers::crates("../../..") {
        let package = &toml.package;
        println!("package: {}", package.name);

        assert!(
            toml.lints.map(|lints| lints.workspace).unwrap_or(false),
            "`{}` missing workspace lints",
            package.name
        );

        assert_eq!(package.edition, "2021");
        assert_eq!(package.authors, None);
        assert_eq!(package.readme, None);

        if package.publish == Some(false) {
            // The `version` field is no longer required - remove once MSRV can support it.
            assert_eq!(package.version, "0.0.0");
            assert_eq!(package.license, None);
            assert_eq!(package.repository, None);
            assert_eq!(package.categories, None);
            assert_eq!(package.description, None);
        } else {
            assert_eq!(package.license, Some("MIT OR Apache-2.0".to_string()));
            assert_eq!(
                package.repository,
                Some("https://github.com/microsoft/windows-rs".to_string())
            );
            assert_eq!(
                package.categories,
                Some(vec!["os::windows-apis".to_string()])
            );
            assert!(package.description.is_some());
            assert!(!package.description.as_ref().unwrap().is_empty());
        }
    }
}
