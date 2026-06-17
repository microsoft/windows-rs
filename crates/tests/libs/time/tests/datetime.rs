use std::time::{Duration, SystemTime};
use windows_time::{DateTime, TimeSpan};

#[test]
fn unix_epoch_ticks() {
    // 369 years between 1601 and 1970, including 89 leap years
    // = (369 * 365 + 89) days = 134_774 days
    // * 86_400 secs = 11_644_473_600 secs
    // * 10_000_000 ticks = 116_444_736_000_000_000
    assert_eq!(DateTime::UNIX_EPOCH.ticks(), 116_444_736_000_000_000);
    assert_eq!(DateTime::UNIX_EPOCH.unix_secs(), 0);
    assert_eq!(DateTime::UNIX_EPOCH.unix_millis(), 0);
}

#[test]
fn from_unix_round_trip() {
    let secs = [-1_000_000_000_i64, -1, 0, 1, 1_700_000_000];
    for s in secs {
        assert_eq!(DateTime::from_unix_secs(s).unix_secs(), s);
    }
    let millis = [-1_500_i64, 0, 1, 1_700_000_000_500];
    for m in millis {
        assert_eq!(DateTime::from_unix_millis(m).unix_millis(), m);
    }
}

#[test]
fn add_sub_timespan() {
    let t = DateTime::UNIX_EPOCH;
    let span = TimeSpan::from_seconds(60);
    assert_eq!((t + span).unix_secs(), 60);
    assert_eq!((t - span).unix_secs(), -60);
    let mut m = t;
    m += span;
    m -= span;
    assert_eq!(m, t);
}

#[test]
fn sub_datetime_yields_timespan() {
    let a = DateTime::from_unix_secs(1_700_000_000);
    let b = DateTime::from_unix_secs(1_700_000_010);
    assert_eq!(b - a, TimeSpan::from_seconds(10));
    assert_eq!(a - b, TimeSpan::from_seconds(-10));
}

#[test]
fn ordering() {
    let a = DateTime::from_unix_secs(-10);
    let b = DateTime::UNIX_EPOCH;
    let c = DateTime::from_unix_secs(10);
    assert!(a < b);
    assert!(b < c);
    let mut v = [c, a, b];
    v.sort();
    assert_eq!(v, [a, b, c]);
}

#[test]
fn display_unix_epoch() {
    assert_eq!(format!("{}", DateTime::UNIX_EPOCH), "1970-01-01T00:00:00Z");
}

#[test]
fn display_1601_epoch() {
    assert_eq!(
        format!("{}", DateTime::from_ticks(0)),
        "1601-01-01T00:00:00Z"
    );
}

#[test]
fn display_fractional() {
    let t = DateTime::UNIX_EPOCH + TimeSpan::from_millis(500);
    assert_eq!(format!("{t}"), "1970-01-01T00:00:00.5Z");
    let t = DateTime::UNIX_EPOCH + TimeSpan::from_ticks(1);
    assert_eq!(format!("{t}"), "1970-01-01T00:00:00.0000001Z");
}

#[test]
fn display_specific_date() {
    // 2023-04-15T12:34:56Z, sanity check
    // Unix seconds for 2023-04-15T12:34:56Z = 1681562096
    let t = DateTime::from_unix_secs(1_681_562_096);
    assert_eq!(format!("{t}"), "2023-04-15T12:34:56Z");
}

#[test]
fn display_pre_unix() {
    // 1969-12-31T23:59:59Z
    let t = DateTime::from_unix_secs(-1);
    assert_eq!(format!("{t}"), "1969-12-31T23:59:59Z");
}

#[test]
fn system_time_round_trip() {
    let now = SystemTime::now();
    let dt: DateTime = now.try_into().unwrap();
    let back: SystemTime = dt.try_into().unwrap();
    // Round-trip should be within 1µs.
    let delta = match back.duration_since(now) {
        Ok(d) => d,
        Err(e) => e.duration(),
    };
    assert!(delta < Duration::from_micros(1), "delta = {delta:?}");
}

#[test]
#[should_panic(expected = "overflow when adding TimeSpan to DateTime")]
fn add_panics_on_overflow() {
    let _ = DateTime::MAX + TimeSpan::from_ticks(1);
}

#[test]
fn system_time_unix_epoch() {
    let dt: DateTime = SystemTime::UNIX_EPOCH.try_into().unwrap();
    assert_eq!(dt, DateTime::UNIX_EPOCH);
    let back: SystemTime = DateTime::UNIX_EPOCH.try_into().unwrap();
    assert_eq!(back, SystemTime::UNIX_EPOCH);
}

#[test]
fn checked_duration_since() {
    let a = DateTime::from_unix_secs(100);
    let b = DateTime::from_unix_secs(50);
    assert_eq!(
        a.checked_duration_since(b),
        Some(TimeSpan::from_seconds(50))
    );
    assert_eq!(DateTime::MAX.checked_add(TimeSpan::from_ticks(1)), None);
    assert_eq!(DateTime::MIN.checked_sub(TimeSpan::from_ticks(1)), None);
}

#[test]
fn now_converts_to_system_time() {
    let n = DateTime::now();
    // Verify that DateTime::now() round-trips through SystemTime without
    // assuming the system clock is after the Unix epoch.
    let st: SystemTime = n.try_into().unwrap();
    let back: DateTime = st.try_into().unwrap();
    assert_eq!(n, back);
}
