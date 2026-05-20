use core::time::Duration;
use windows_time::{TimeRangeError, TimeSpan};

#[test]
fn ticks_round_trip() {
    let values = [i64::MIN, -1, 0, 1, TimeSpan::TICKS_PER_SECOND, i64::MAX];
    for v in values {
        assert_eq!(TimeSpan::from_ticks(v).ticks(), v);
    }
}

#[test]
fn unit_constructors() {
    assert_eq!(TimeSpan::from_seconds(1).ticks(), 10_000_000);
    assert_eq!(TimeSpan::from_millis(1).ticks(), 10_000);
    assert_eq!(TimeSpan::from_micros(1).ticks(), 10);
    assert_eq!(TimeSpan::from_minutes(1).ticks(), 600_000_000);
    assert_eq!(TimeSpan::from_hours(1).ticks(), 36_000_000_000);
    assert_eq!(TimeSpan::from_days(1).ticks(), 864_000_000_000);
}

#[test]
fn whole_accessors() {
    let t = TimeSpan::from_seconds(3_600) + TimeSpan::from_millis(500);
    assert_eq!(t.whole_seconds(), 3_600);
    assert_eq!(t.whole_minutes(), 60);
    assert_eq!(t.whole_hours(), 1);
    assert_eq!(t.whole_millis(), 3_600_500);
}

#[test]
fn sign_helpers() {
    assert!(TimeSpan::ZERO.is_zero());
    assert!(TimeSpan::from_seconds(-1).is_negative());
    assert!(TimeSpan::from_seconds(1).is_positive());
    assert_eq!(TimeSpan::from_seconds(-3).abs(), TimeSpan::from_seconds(3));
    assert_eq!(TimeSpan::from_seconds(-3).signum(), -1);
    assert_eq!(TimeSpan::from_seconds(3).signum(), 1);
    assert_eq!(TimeSpan::ZERO.signum(), 0);
}

#[test]
fn arithmetic_identities() {
    let a = TimeSpan::from_seconds(10);
    let b = TimeSpan::from_millis(2_500);
    assert_eq!(a + (b - a), b);
    assert_eq!(a - a, TimeSpan::ZERO);
    assert_eq!(-a + a, TimeSpan::ZERO);
    assert_eq!((a * 3).ticks(), a.ticks() * 3);
    assert_eq!((a / 2).ticks(), a.ticks() / 2);
}

#[test]
fn checked_overflow() {
    assert_eq!(TimeSpan::MAX.checked_add(TimeSpan::from_ticks(1)), None);
    assert_eq!(TimeSpan::MIN.checked_sub(TimeSpan::from_ticks(1)), None);
    assert_eq!(TimeSpan::MIN.checked_neg(), None);
    assert_eq!(TimeSpan::MAX.checked_mul(2), None);
    assert_eq!(TimeSpan::from_ticks(10).checked_div(0), None);
}

#[test]
fn saturating_overflow() {
    assert_eq!(
        TimeSpan::MAX.saturating_add(TimeSpan::from_ticks(1)),
        TimeSpan::MAX
    );
    assert_eq!(
        TimeSpan::MIN.saturating_sub(TimeSpan::from_ticks(1)),
        TimeSpan::MIN
    );
    assert_eq!(TimeSpan::MIN.saturating_neg(), TimeSpan::MAX);
}

#[test]
fn ord_consistent_with_ticks() {
    let a = TimeSpan::from_seconds(-5);
    let b = TimeSpan::from_seconds(0);
    let c = TimeSpan::from_seconds(5);
    assert!(a < b);
    assert!(b < c);
    let mut v = [c, a, b];
    v.sort();
    assert_eq!(v, [a, b, c]);
}

#[test]
fn duration_round_trip() {
    let durations = [
        Duration::ZERO,
        Duration::from_secs(1),
        Duration::from_millis(1234),
        Duration::new(60 * 60 * 24, 999_999_900),
    ];
    for d in durations {
        let ts: TimeSpan = d.try_into().unwrap();
        let back: Duration = ts.try_into().unwrap();
        assert_eq!(d, back);
    }
}

#[test]
fn duration_rejects_negative_timespan() {
    let neg = TimeSpan::from_seconds(-1);
    let err: Result<Duration, TimeRangeError> = neg.try_into();
    assert!(err.is_err());
}

#[test]
fn timespan_rejects_oversize_duration() {
    let huge = Duration::new(u64::MAX, 999_999_999);
    let err: Result<TimeSpan, TimeRangeError> = huge.try_into();
    assert!(err.is_err());
}

#[test]
fn display_zero() {
    assert_eq!(format!("{}", TimeSpan::ZERO), "PT0S");
}

#[test]
fn display_positive() {
    assert_eq!(format!("{}", TimeSpan::from_seconds(1)), "PT1S");
    assert_eq!(format!("{}", TimeSpan::from_millis(1_500)), "PT1.5S");
    assert_eq!(format!("{}", TimeSpan::from_minutes(2)), "PT2M");
    assert_eq!(format!("{}", TimeSpan::from_hours(3)), "PT3H");
    assert_eq!(format!("{}", TimeSpan::from_days(4)), "P4D");
    let t = TimeSpan::from_days(1)
        + TimeSpan::from_hours(2)
        + TimeSpan::from_minutes(3)
        + TimeSpan::from_seconds(4)
        + TimeSpan::from_millis(500);
    assert_eq!(format!("{t}"), "P1DT2H3M4.5S");
}

#[test]
fn display_negative() {
    assert_eq!(format!("{}", TimeSpan::from_seconds(-1)), "-PT1S");
    assert_eq!(
        format!(
            "{}",
            TimeSpan::from_seconds(-1) - TimeSpan::from_millis(500)
        ),
        "-PT1.5S"
    );
}

#[test]
fn display_seven_digit_fraction() {
    // 1 tick = 100ns
    let t = TimeSpan::from_ticks(1);
    assert_eq!(format!("{t}"), "PT0.0000001S");
}
