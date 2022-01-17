use windows::Storage::Streams::*;

#[test]
fn nested() {
    let options = InputStreamOptions::Partial | InputStreamOptions::ReadAhead;
    assert!(options.0 == 3);

    let options = InputStreamOptions::Partial & InputStreamOptions::ReadAhead;
    assert!(options.0 == 0);

    let options = (InputStreamOptions::Partial | InputStreamOptions::ReadAhead) & InputStreamOptions::ReadAhead;
    assert!(options.0 == 2);

    let mut options = InputStreamOptions::Partial;
    options |= InputStreamOptions::ReadAhead;
    assert!(options.0 == 3);

    options &= InputStreamOptions::ReadAhead;
    assert!(options.0 == 2);
}

#[test]
fn const_pattern() {
    match InputStreamOptions::ReadAhead {
        InputStreamOptions::ReadAhead => assert!(true),
        _ => assert!(false),
    }
}
