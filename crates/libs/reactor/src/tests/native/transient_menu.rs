use super::*;

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
