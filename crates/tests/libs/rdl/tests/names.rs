#[test]
fn enum_members_are_separate_from_items() {
    let path = std::path::Path::new(env!("OUT_DIR")).join("names.rdl");
    std::fs::write(
        &path,
        r#"
        #[win32]
        mod Test {
            #[arch(X86)]
            #[repr(i32)]
            enum Plain { VALUE = 1, FIRST = 0 }
            #[arch(X64 | Arm64)]
            #[repr(i32)]
            enum Plain { VALUE = 1, SECOND = 2 }
            #[scoped]
            #[repr(i32)]
            enum Scoped { VALUE = 2, r#type = 3 }
            const IGNORED: u32 = 5;
            mod Other {
                #[repr(i32)]
                enum Different { OUTSIDE = 4 }
            }
        }
    "#,
    )
    .unwrap();
    assert_eq!(
        windows_rdl::item_names(&path, "Test").unwrap(),
        ["Plain", "Scoped", "IGNORED"]
    );
    assert_eq!(
        windows_rdl::enum_member_names(&path, "Test").unwrap(),
        ["FIRST", "SECOND", "VALUE", "type"]
    );
    assert_eq!(
        windows_rdl::enum_member_names(&path, "Test.Other").unwrap(),
        ["OUTSIDE"]
    );
    assert!(
        windows_rdl::enum_member_names(&path, "Absent")
            .unwrap()
            .is_empty()
    );
    std::fs::write(&path, "#[win32] mod Test { enum Broken {").unwrap();
    assert!(windows_rdl::enum_member_names(&path, "Test").is_err());
    std::fs::remove_file(&path).unwrap();
    assert!(windows_rdl::enum_member_names(&path, "Test").is_err());
}
