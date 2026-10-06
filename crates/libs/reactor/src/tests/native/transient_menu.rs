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

#[test]
fn retired_popup_callbacks_cannot_deliver_to_a_new_popup() {
    let delivered = Rc::new(RefCell::new(Vec::new()));
    let record = Rc::clone(&delivered);
    let callback = Callback::new(move |key| record.borrow_mut().push(key));
    let first_live = Rc::new(Cell::new(true));
    let first = menu_callback(callback.clone(), Rc::clone(&first_live));
    first.call("first".into());
    first_live.set(false);
    let second_live = Rc::new(Cell::new(true));
    let second = menu_callback(callback, Rc::clone(&second_live));
    first.call("stale".into());
    second.call("second".into());
    second_live.set(false);
    second.call("closed".into());
    assert_eq!(
        *delivered.borrow(),
        [Key::from("first"), Key::from("second")]
    );
}
