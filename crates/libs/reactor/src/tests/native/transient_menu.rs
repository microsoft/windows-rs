use super::*;

#[test]
#[should_panic(expected = "duplicate key")]
fn application_menu_rejects_duplicate_nested_keys() {
    let menu = [
        MenuItem::item("duplicate", "First"),
        MenuItem::submenu(
            "submenu",
            "Submenu",
            [MenuItem::item("duplicate", "Second")],
        ),
    ];
    validate_menu_items(&menu, &mut HashSet::new());
}

#[test]
fn retired_popup_callbacks_cannot_deliver_to_a_new_popup() {
    let delivered = Rc::new(RefCell::new(Vec::new()));
    let record = Rc::clone(&delivered);
    let callback = Callback::new(move |key| record.borrow_mut().push(key));
    let first_live = Rc::new(Cell::new(true));
    let first = menu_callback(
        Rc::new(RefCell::new(callback.clone())),
        Rc::clone(&first_live),
    );
    first.call("first".into());
    first_live.set(false);
    let second_live = Rc::new(Cell::new(true));
    let second = menu_callback(Rc::new(RefCell::new(callback)), Rc::clone(&second_live));
    first.call("stale".into());
    second.call("second".into());
    second_live.set(false);
    second.call("closed".into());
    assert_eq!(
        *delivered.borrow(),
        [Key::from("first"), Key::from("second")]
    );
}

#[test]
fn popup_callback_replacement_uses_latest_handler_without_holding_a_borrow() {
    let delivered = Rc::new(RefCell::new(Vec::new()));
    let record = Rc::clone(&delivered);
    let callback = Rc::new(RefCell::new(Callback::new(move |key| {
        record.borrow_mut().push((0, key));
    })));
    let live = Rc::new(Cell::new(true));
    let guarded = menu_callback(Rc::clone(&callback), Rc::clone(&live));
    guarded.call("first".into());
    let record = Rc::clone(&delivered);
    let slot = Rc::downgrade(&callback);
    *callback.borrow_mut() = Callback::new(move |key| {
        record.borrow_mut().push((1, key));
        *slot.upgrade().unwrap().borrow_mut() = Callback::new(|_| {});
    });
    guarded.call("second".into());
    live.set(false);
    guarded.call("closed".into());
    assert_eq!(
        *delivered.borrow(),
        [(0, Key::from("first")), (1, Key::from("second"))]
    );
}

#[test]
fn system_theme_preference_preserves_query_errors() {
    assert_eq!(theme_preference(0, 0).unwrap(), ElementTheme::Dark);
    assert_eq!(theme_preference(0, 1).unwrap(), ElementTheme::Light);
    for missing in [ERROR_FILE_NOT_FOUND, ERROR_PATH_NOT_FOUND] {
        assert_eq!(theme_preference(missing, 0).unwrap(), ElementTheme::Default);
    }
    assert_eq!(
        theme_preference(5, 0).unwrap_err().code(),
        windows_core::WIN32_ERROR(5).to_hresult()
    );
}
