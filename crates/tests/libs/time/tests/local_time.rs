use windows_time::DateTime;

#[test]
fn decompose_unix_epoch() {
    let t = DateTime::UNIX_EPOCH;
    assert_eq!(t.year(), 1970);
    assert_eq!(t.month(), 1);
    assert_eq!(t.day(), 1);
    assert_eq!(t.hour(), 0);
    assert_eq!(t.minute(), 0);
    assert_eq!(t.second(), 0);
    assert_eq!(t.milliseconds(), 0);
    // 1970-01-01 was a Thursday
    assert_eq!(t.day_of_week(), 4);
}

#[test]
fn decompose_1601_epoch() {
    let t = DateTime::from_ticks(0);
    assert_eq!(t.year(), 1601);
    assert_eq!(t.month(), 1);
    assert_eq!(t.day(), 1);
    assert_eq!(t.hour(), 0);
    assert_eq!(t.minute(), 0);
    assert_eq!(t.second(), 0);
    assert_eq!(t.milliseconds(), 0);
    // 1601-01-01 was a Monday
    assert_eq!(t.day_of_week(), 1);
}

#[test]
fn decompose_specific_date() {
    // 2023-04-15T12:34:56Z (Saturday)
    let t = DateTime::from_unix_secs(1_681_562_096);
    assert_eq!(t.year(), 2023);
    assert_eq!(t.month(), 4);
    assert_eq!(t.day(), 15);
    assert_eq!(t.hour(), 12);
    assert_eq!(t.minute(), 34);
    assert_eq!(t.second(), 56);
    assert_eq!(t.milliseconds(), 0);
    assert_eq!(t.day_of_week(), 6); // Saturday
}

#[test]
fn decompose_pre_unix_epoch() {
    // 1969-12-31T23:59:59Z (Wednesday)
    let t = DateTime::from_unix_secs(-1);
    assert_eq!(t.year(), 1969);
    assert_eq!(t.month(), 12);
    assert_eq!(t.day(), 31);
    assert_eq!(t.hour(), 23);
    assert_eq!(t.minute(), 59);
    assert_eq!(t.second(), 59);
    assert_eq!(t.day_of_week(), 3); // Wednesday
}

#[test]
fn decompose_with_milliseconds() {
    use windows_time::TimeSpan;
    let t = DateTime::UNIX_EPOCH + TimeSpan::from_millis(1_500);
    assert_eq!(t.year(), 1970);
    assert_eq!(t.month(), 1);
    assert_eq!(t.day(), 1);
    assert_eq!(t.hour(), 0);
    assert_eq!(t.minute(), 0);
    assert_eq!(t.second(), 1);
    assert_eq!(t.milliseconds(), 500);
}

#[test]
fn decompose_leap_year() {
    // 2024-02-29T00:00:00Z (Thursday) — leap day
    // Unix secs for 2024-02-29: days since 1970-01-01 = 19_782
    let t = DateTime::from_unix_secs(19_782 * 86_400);
    assert_eq!(t.year(), 2024);
    assert_eq!(t.month(), 2);
    assert_eq!(t.day(), 29);
    assert_eq!(t.day_of_week(), 4); // Thursday
}

#[test]
fn decompose_end_of_day() {
    // 2025-06-04T23:59:59Z
    // Days from 1970-01-01 to 2025-06-04 = 20243
    let t = DateTime::from_unix_secs(20_243 * 86_400 + 23 * 3600 + 59 * 60 + 59);
    assert_eq!(t.year(), 2025);
    assert_eq!(t.month(), 6);
    assert_eq!(t.day(), 4);
    assert_eq!(t.hour(), 23);
    assert_eq!(t.minute(), 59);
    assert_eq!(t.second(), 59);
}

#[test]
fn decompose_year_boundary() {
    // 2000-01-01T00:00:00Z (Saturday)
    let t = DateTime::from_unix_secs(946_684_800);
    assert_eq!(t.year(), 2000);
    assert_eq!(t.month(), 1);
    assert_eq!(t.day(), 1);
    assert_eq!(t.day_of_week(), 6); // Saturday
}

#[cfg(windows)]
#[test]
fn to_local_plausible() {
    // to_local should produce a DateTime whose decomposition is within ±14 hours of UTC.
    let utc = DateTime::now();
    let local = utc.to_local();

    // The difference between local and UTC ticks should be at most ±14 hours
    // (the maximum timezone offset on Earth).
    let diff_ticks = (local.ticks() - utc.ticks()).unsigned_abs();
    let max_offset_ticks = 14 * 3600 * 10_000_000_u64;
    assert!(
        diff_ticks <= max_offset_ticks,
        "to_local offset too large: {diff_ticks} ticks"
    );
}

#[cfg(windows)]
#[test]
fn to_local_decomposition_plausible() {
    let local = DateTime::now().to_local();
    assert!(local.year() >= 2024);
    assert!((1..=12).contains(&local.month()));
    assert!((1..=31).contains(&local.day()));
    assert!(local.day_of_week() <= 6);
    assert!(local.hour() <= 23);
    assert!(local.minute() <= 59);
    assert!(local.second() <= 59);
    assert!(local.milliseconds() <= 999);
}

#[cfg(windows)]
#[test]
fn to_local_preserves_sub_second() {
    use windows_time::TimeSpan;
    // A known UTC time with 750ms — to_local should preserve the sub-second part
    // (timezone offsets are always whole minutes).
    let utc = DateTime::UNIX_EPOCH + TimeSpan::from_millis(750);
    let local = utc.to_local();
    assert_eq!(local.milliseconds(), 750);
}

#[test]
fn decompose_now_matches_display() {
    // Verify that decomposition matches what Display produces (for UTC).
    let t = DateTime::now();
    let display = format!("{t}");
    let year_str = format!("{:04}", t.year());
    let month_str = format!("{:02}", t.month());
    let day_str = format!("{:02}", t.day());
    let hour_str = format!("{:02}", t.hour());
    let min_str = format!("{:02}", t.minute());
    let sec_str = format!("{:02}", t.second());

    assert!(
        display.starts_with(&format!(
            "{year_str}-{month_str}-{day_str}T{hour_str}:{min_str}:{sec_str}"
        )),
        "Display '{display}' doesn't match decomposition"
    );
}
