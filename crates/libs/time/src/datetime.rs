use super::timespan::{TimeRangeError, TICKS_PER_DAY, TICKS_PER_SECOND};
use super::{DateTime, TimeSpan};

/// Number of seconds between 1601-01-01 UTC (the `DateTime` epoch) and
/// 1970-01-01 UTC (the Unix epoch).
pub const UNIX_EPOCH_DIFFERENCE_SECS: i64 = 11_644_473_600;

/// Tick value of the Unix epoch in `DateTime`'s 1601-based clock.
const UNIX_EPOCH_TICKS: i64 = UNIX_EPOCH_DIFFERENCE_SECS * TICKS_PER_SECOND;

impl DateTime {
    /// The `DateTime` value corresponding to 1970-01-01 00:00:00 UTC.
    pub const UNIX_EPOCH: Self = Self {
        UniversalTime: UNIX_EPOCH_TICKS,
    };

    /// The minimum representable `DateTime` (`i64::MIN` ticks).
    pub const MIN: Self = Self {
        UniversalTime: i64::MIN,
    };

    /// The maximum representable `DateTime` (`i64::MAX` ticks).
    pub const MAX: Self = Self {
        UniversalTime: i64::MAX,
    };

    /// The number of 100-nanosecond ticks per second (matches
    /// [`TimeSpan::TICKS_PER_SECOND`]).
    pub const TICKS_PER_SECOND: i64 = TICKS_PER_SECOND;

    /// Constructs a `DateTime` from a raw count of 100-nanosecond ticks since
    /// 1601-01-01 00:00:00 UTC.
    pub const fn from_ticks(ticks: i64) -> Self {
        Self {
            UniversalTime: ticks,
        }
    }

    /// Constructs a `DateTime` from a number of whole seconds since the Unix
    /// epoch, saturating on overflow.
    pub const fn from_unix_secs(secs: i64) -> Self {
        let total = secs.saturating_add(UNIX_EPOCH_DIFFERENCE_SECS);
        Self {
            UniversalTime: total.saturating_mul(TICKS_PER_SECOND),
        }
    }

    /// Constructs a `DateTime` from a number of whole milliseconds since the
    /// Unix epoch, saturating on overflow.
    pub const fn from_unix_millis(millis: i64) -> Self {
        // (millis * 10_000) ticks since Unix epoch + offset
        let unix_ticks = millis.saturating_mul(10_000);
        let offset_ticks = UNIX_EPOCH_DIFFERENCE_SECS.saturating_mul(TICKS_PER_SECOND);
        Self {
            UniversalTime: unix_ticks.saturating_add(offset_ticks),
        }
    }

    /// Returns the raw count of 100-nanosecond ticks since 1601-01-01 UTC.
    pub const fn ticks(self) -> i64 {
        self.UniversalTime
    }

    /// Returns the number of whole seconds between this `DateTime` and the
    /// Unix epoch. Negative for pre-1970 values.
    pub const fn unix_secs(self) -> i64 {
        self.UniversalTime / TICKS_PER_SECOND - UNIX_EPOCH_DIFFERENCE_SECS
    }

    /// Returns the number of whole milliseconds between this `DateTime` and
    /// the Unix epoch. Negative for pre-1970 values.
    pub const fn unix_millis(self) -> i64 {
        self.UniversalTime / 10_000 - UNIX_EPOCH_DIFFERENCE_SECS * 1_000
    }

    /// Returns the number of nanoseconds between this `DateTime` and the Unix
    /// epoch, as an `i128` to avoid overflow.
    pub const fn unix_nanos(self) -> i128 {
        (self.UniversalTime as i128) * 100 - (UNIX_EPOCH_DIFFERENCE_SECS as i128) * 1_000_000_000
    }

    /// Checked addition of a `TimeSpan`. Returns `None` on overflow.
    pub const fn checked_add(self, rhs: TimeSpan) -> Option<Self> {
        match self.UniversalTime.checked_add(rhs.Duration) {
            Some(d) => Some(Self { UniversalTime: d }),
            None => None,
        }
    }

    /// Checked subtraction of a `TimeSpan`. Returns `None` on overflow.
    pub const fn checked_sub(self, rhs: TimeSpan) -> Option<Self> {
        match self.UniversalTime.checked_sub(rhs.Duration) {
            Some(d) => Some(Self { UniversalTime: d }),
            None => None,
        }
    }

    /// Returns the signed duration from `earlier` to `self`. Returns `None` on
    /// overflow.
    pub const fn checked_duration_since(self, earlier: Self) -> Option<TimeSpan> {
        match self.UniversalTime.checked_sub(earlier.UniversalTime) {
            Some(d) => Some(TimeSpan { Duration: d }),
            None => None,
        }
    }

    /// Saturating addition of a `TimeSpan`.
    pub const fn saturating_add(self, rhs: TimeSpan) -> Self {
        Self {
            UniversalTime: self.UniversalTime.saturating_add(rhs.Duration),
        }
    }

    /// Saturating subtraction of a `TimeSpan`.
    pub const fn saturating_sub(self, rhs: TimeSpan) -> Self {
        Self {
            UniversalTime: self.UniversalTime.saturating_sub(rhs.Duration),
        }
    }

    /// Returns the current `DateTime` from the system clock.
    #[cfg(feature = "std")]
    pub fn now() -> Self {
        match Self::try_from(std::time::SystemTime::now()) {
            Ok(value) => value,
            Err(_) => Self::MAX,
        }
    }
}

impl Eq for DateTime {}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DateTime {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.UniversalTime.cmp(&other.UniversalTime)
    }
}

impl core::hash::Hash for DateTime {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.UniversalTime.hash(state);
    }
}

impl core::ops::Add<TimeSpan> for DateTime {
    type Output = Self;
    fn add(self, rhs: TimeSpan) -> Self {
        self.checked_add(rhs)
            .expect("overflow when adding TimeSpan to DateTime")
    }
}

impl core::ops::Sub<TimeSpan> for DateTime {
    type Output = Self;
    fn sub(self, rhs: TimeSpan) -> Self {
        self.checked_sub(rhs)
            .expect("overflow when subtracting TimeSpan from DateTime")
    }
}

impl core::ops::Sub<DateTime> for DateTime {
    type Output = TimeSpan;
    fn sub(self, rhs: DateTime) -> TimeSpan {
        self.checked_duration_since(rhs)
            .expect("overflow when subtracting DateTime values")
    }
}

impl core::ops::AddAssign<TimeSpan> for DateTime {
    fn add_assign(&mut self, rhs: TimeSpan) {
        self.UniversalTime = self
            .UniversalTime
            .checked_add(rhs.Duration)
            .expect("overflow when adding TimeSpan to DateTime");
    }
}

impl core::ops::SubAssign<TimeSpan> for DateTime {
    fn sub_assign(&mut self, rhs: TimeSpan) {
        self.UniversalTime = self
            .UniversalTime
            .checked_sub(rhs.Duration)
            .expect("overflow when subtracting TimeSpan from DateTime");
    }
}

/// Howard Hinnant's `civil_from_days`: converts a day-count from
/// 1970-01-01 (Unix epoch) into a proleptic Gregorian (year, month, day).
const fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32; // [1, 12]
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

/// ISO-8601 UTC formatting: `YYYY-MM-DDTHH:MM:SS.fffffffZ`. The fractional
/// part is omitted when zero. Year is at least 4 digits, padded with a leading
/// `-` for negative years.
impl core::fmt::Display for DateTime {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let ticks = self.UniversalTime;
        // Use i128 to avoid overflow when subtracting the Unix epoch offset
        // for extreme DateTime values (near i64::MIN / i64::MAX).
        let unix_ticks = (ticks as i128) - (UNIX_EPOCH_TICKS as i128);
        let ticks_per_day = TICKS_PER_DAY as i128;
        // div_euclid gives floored division; rem_euclid gives remainder in [0, ticks_per_day).
        // The day count fits in i64: ticks are bounded by i64::MIN/MAX, so days are at most
        // ~±10_811_000 (i.e., ±29_584 years), well within i64 range.
        let days = unix_ticks.div_euclid(ticks_per_day) as i64;
        let intraday = unix_ticks.rem_euclid(ticks_per_day) as i64;
        let (year, month, day) = civil_from_days(days);

        let secs = intraday / TICKS_PER_SECOND;
        let fraction = (intraday % TICKS_PER_SECOND) as u32;
        let hour = (secs / 3_600) as u32;
        let minute = ((secs % 3_600) / 60) as u32;
        let second = (secs % 60) as u32;

        if year < 0 {
            write!(f, "-{:04}", -year)?;
        } else {
            write!(f, "{year:04}")?;
        }
        write!(f, "-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}")?;

        if fraction != 0 {
            let mut frac = fraction;
            let mut digits = [0u8; 7];
            for slot in digits.iter_mut().rev() {
                *slot = (frac % 10) as u8;
                frac /= 10;
            }
            let mut len = 7;
            while len > 0 && digits[len - 1] == 0 {
                len -= 1;
            }
            f.write_str(".")?;
            for d in &digits[..len] {
                write!(f, "{d}")?;
            }
        }
        f.write_str("Z")
    }
}

#[cfg(feature = "std")]
impl TryFrom<std::time::SystemTime> for DateTime {
    type Error = TimeRangeError;
    fn try_from(value: std::time::SystemTime) -> Result<Self, Self::Error> {
        match value.duration_since(std::time::SystemTime::UNIX_EPOCH) {
            Ok(d) => {
                // Positive: ticks = duration_ticks + UNIX_EPOCH_TICKS
                let ticks = d.as_nanos() / 100;
                if ticks > (i64::MAX as u128) - (UNIX_EPOCH_TICKS as u128) {
                    return Err(TimeRangeError);
                }
                Ok(Self {
                    UniversalTime: ticks as i64 + UNIX_EPOCH_TICKS,
                })
            }
            Err(e) => {
                // Negative: value is before Unix epoch
                let d = e.duration();
                let ticks = d.as_nanos() / 100;
                if ticks > UNIX_EPOCH_TICKS as u128 {
                    return Err(TimeRangeError);
                }
                Ok(Self {
                    UniversalTime: UNIX_EPOCH_TICKS - ticks as i64,
                })
            }
        }
    }
}

#[cfg(feature = "std")]
impl TryFrom<DateTime> for std::time::SystemTime {
    type Error = TimeRangeError;
    fn try_from(value: DateTime) -> Result<Self, Self::Error> {
        let ticks = value.UniversalTime;
        // Use i128 to avoid overflow for extreme DateTime values (near i64::MIN / i64::MAX).
        let unix_ticks = (ticks as i128) - (UNIX_EPOCH_TICKS as i128);
        if unix_ticks >= 0 {
            let nanos = unix_ticks as u128 * 100;
            let secs = (nanos / 1_000_000_000) as u64;
            let subsec = (nanos % 1_000_000_000) as u32;
            std::time::SystemTime::UNIX_EPOCH
                .checked_add(std::time::Duration::new(secs, subsec))
                .ok_or(TimeRangeError)
        } else {
            let abs_ticks = (-unix_ticks) as u128;
            let nanos = abs_ticks * 100;
            let secs = (nanos / 1_000_000_000) as u64;
            let subsec = (nanos % 1_000_000_000) as u32;
            std::time::SystemTime::UNIX_EPOCH
                .checked_sub(std::time::Duration::new(secs, subsec))
                .ok_or(TimeRangeError)
        }
    }
}
