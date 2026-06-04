use windows_time::LocalTime;

#[test]
fn now_returns_plausible_values() {
    let t = LocalTime::now();
    assert!(t.year() >= 2024);
    assert!((1..=12).contains(&t.month()));
    assert!((1..=31).contains(&t.day()));
    assert!(t.day_of_week() <= 6);
    assert!(t.hour() <= 23);
    assert!(t.minute() <= 59);
    assert!(t.second() <= 59);
    assert!(t.milliseconds() <= 999);
}

#[test]
fn new_valid() {
    let t = LocalTime::new(2025, 6, 15, 0, 14, 30, 45, 123).unwrap();
    assert_eq!(t.year(), 2025);
    assert_eq!(t.month(), 6);
    assert_eq!(t.day(), 15);
    assert_eq!(t.day_of_week(), 0);
    assert_eq!(t.hour(), 14);
    assert_eq!(t.minute(), 30);
    assert_eq!(t.second(), 45);
    assert_eq!(t.milliseconds(), 123);
}

#[test]
fn new_rejects_invalid() {
    // month 0
    assert!(LocalTime::new(2025, 0, 15, 0, 12, 0, 0, 0).is_none());
    // month 13
    assert!(LocalTime::new(2025, 13, 15, 0, 12, 0, 0, 0).is_none());
    // day 0
    assert!(LocalTime::new(2025, 6, 0, 0, 12, 0, 0, 0).is_none());
    // day 32
    assert!(LocalTime::new(2025, 6, 32, 0, 12, 0, 0, 0).is_none());
    // day_of_week 7
    assert!(LocalTime::new(2025, 6, 15, 7, 12, 0, 0, 0).is_none());
    // hour 24
    assert!(LocalTime::new(2025, 6, 15, 0, 24, 0, 0, 0).is_none());
    // minute 60
    assert!(LocalTime::new(2025, 6, 15, 0, 12, 60, 0, 0).is_none());
    // second 60
    assert!(LocalTime::new(2025, 6, 15, 0, 12, 0, 60, 0).is_none());
    // milliseconds 1000
    assert!(LocalTime::new(2025, 6, 15, 0, 12, 0, 0, 1000).is_none());
}

#[test]
fn display_format() {
    let t = LocalTime::new(2025, 1, 9, 4, 8, 5, 3, 7).unwrap();
    assert_eq!(format!("{t}"), "2025-01-09T08:05:03.007");
}

#[test]
fn now_called_twice_is_non_decreasing() {
    // Call now() twice in quick succession — the second should not be earlier.
    let a = LocalTime::now();
    let b = LocalTime::now();

    // Compare as a tuple (year, month, day, hour, minute, second, milliseconds).
    let a_tuple = (
        a.year(),
        a.month(),
        a.day(),
        a.hour(),
        a.minute(),
        a.second(),
        a.milliseconds(),
    );
    let b_tuple = (
        b.year(),
        b.month(),
        b.day(),
        b.hour(),
        b.minute(),
        b.second(),
        b.milliseconds(),
    );
    assert!(b_tuple >= a_tuple);
}
