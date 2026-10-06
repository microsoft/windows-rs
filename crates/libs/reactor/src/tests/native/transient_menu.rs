use super::*;

#[test]
fn host_position_accounts_for_the_client_offset() {
    assert_eq!(host_coordinate(100, 108).unwrap(), 92);
    assert_eq!(host_coordinate(-100, -92).unwrap(), -108);
}

#[test]
fn application_menu_rejects_duplicate_nested_keys() {
    let menu = [
        MenuItem::item("duplicate", "First"),
        MenuItem::submenu(
            "submenu",
            "Submenu",
            [MenuItem::item("duplicate", "Second")],
        ),
    ];
    assert!(validate_menu_items(&menu, &mut HashSet::new()).is_err());
}
