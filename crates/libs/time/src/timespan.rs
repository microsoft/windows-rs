use super::*;

/// Number of 100-nanosecond ticks per microsecond.
pub const TICKS_PER_MICROSECOND: i64 = 10;
/// Number of 100-nanosecond ticks per millisecond.
pub const TICKS_PER_MILLISECOND: i64 = 10_000;
/// Number of 100-nanosecond ticks per second.
pub const TICKS_PER_SECOND: i64 = 10_000_000;
/// Number of 100-nanosecond ticks per minute.
pub const TICKS_PER_MINUTE: i64 = TICKS_PER_SECOND * 60;
/// Number of 100-nanosecond ticks per hour.
pub const TICKS_PER_HOUR: i64 = TICKS_PER_MINUTE * 60;
/// Number of 100-nanosecond ticks per day.
pub const TICKS_PER_DAY: i64 = TICKS_PER_HOUR * 24;

impl TimeSpan {
    /// The number of 100-nanosecond ticks per microsecond.
    pub const TICKS_PER_MICROSECOND: i64 = TICKS_PER_MICROSECOND;
    /// The number of 100-nanosecond ticks per millisecond.
    pub const TICKS_PER_MILLISECOND: i64 = TICKS_PER_MILLISECOND;
    /// The number of 100-nanosecond ticks per second.
    pub const TICKS_PER_SECOND: i64 = TICKS_PER_SECOND;
    /// The number of 100-nanosecond ticks per minute.
    pub const TICKS_PER_MINUTE: i64 = TICKS_PER_MINUTE;
    /// The number of 100-nanosecond ticks per hour.
    pub const TICKS_PER_HOUR: i64 = TICKS_PER_HOUR;
    /// The number of 100-nanosecond ticks per day.
    pub const TICKS_PER_DAY: i64 = TICKS_PER_DAY;

    /// A zero-length `TimeSpan`.
    pub const ZERO: Self = Self { duration: 0 };
    /// The minimum representable `TimeSpan` (`i64::MIN` ticks).
    pub const MIN: Self = Self { duration: i64::MIN };
    /// The maximum representable `TimeSpan` (`i64::MAX` ticks).
    pub const MAX: Self = Self { duration: i64::MAX };

    /// Constructs a `TimeSpan` from a raw count of 100-nanosecond ticks.
    pub const fn from_ticks(ticks: i64) -> Self {
        Self { duration: ticks }
    }

    /// Constructs a `TimeSpan` from a number of microseconds, saturating on overflow.
    pub const fn from_micros(micros: i64) -> Self {
        Self {
            duration: micros.saturating_mul(TICKS_PER_MICROSECOND),
        }
    }

    /// Constructs a `TimeSpan` from a number of milliseconds, saturating on overflow.
    pub const fn from_millis(millis: i64) -> Self {
        Self {
            duration: millis.saturating_mul(TICKS_PER_MILLISECOND),
        }
    }

    /// Constructs a `TimeSpan` from a number of seconds, saturating on overflow.
    pub const fn from_seconds(seconds: i64) -> Self {
        Self {
            duration: seconds.saturating_mul(TICKS_PER_SECOND),
        }
    }

    /// Constructs a `TimeSpan` from a number of minutes, saturating on overflow.
    pub const fn from_minutes(minutes: i64) -> Self {
        Self {
            duration: minutes.saturating_mul(TICKS_PER_MINUTE),
        }
    }

    /// Constructs a `TimeSpan` from a number of hours, saturating on overflow.
    pub const fn from_hours(hours: i64) -> Self {
        Self {
            duration: hours.saturating_mul(TICKS_PER_HOUR),
        }
    }

    /// Constructs a `TimeSpan` from a number of days, saturating on overflow.
    pub const fn from_days(days: i64) -> Self {
        Self {
            duration: days.saturating_mul(TICKS_PER_DAY),
        }
    }

    /// Returns the count of 100-nanosecond ticks in this `TimeSpan`.
    pub const fn ticks(self) -> i64 {
        self.duration
    }

    /// Returns the total number of whole microseconds in this `TimeSpan`.
    pub const fn whole_micros(self) -> i64 {
        self.duration / TICKS_PER_MICROSECOND
    }

    /// Returns the total number of whole milliseconds in this `TimeSpan`.
    pub const fn whole_millis(self) -> i64 {
        self.duration / TICKS_PER_MILLISECOND
    }

    /// Returns the total number of whole seconds in this `TimeSpan`.
    pub const fn whole_seconds(self) -> i64 {
        self.duration / TICKS_PER_SECOND
    }

    /// Returns the total number of whole minutes in this `TimeSpan`.
    pub const fn whole_minutes(self) -> i64 {
        self.duration / TICKS_PER_MINUTE
    }

    /// Returns the total number of whole hours in this `TimeSpan`.
    pub const fn whole_hours(self) -> i64 {
        self.duration / TICKS_PER_HOUR
    }

    /// Returns the total number of whole days in this `TimeSpan`.
    pub const fn whole_days(self) -> i64 {
        self.duration / TICKS_PER_DAY
    }

    /// Returns the total duration as a count of nanoseconds.
    pub const fn whole_nanos(self) -> i128 {
        self.duration as i128 * 100
    }

    /// Returns `true` if this `TimeSpan` is zero.
    pub const fn is_zero(self) -> bool {
        self.duration == 0
    }

    /// Returns `true` if this `TimeSpan` is negative.
    pub const fn is_negative(self) -> bool {
        self.duration < 0
    }

    /// Returns `true` if this `TimeSpan` is positive (strictly greater than zero).
    pub const fn is_positive(self) -> bool {
        self.duration > 0
    }

    /// Returns the absolute value of this `TimeSpan`, saturating at `MAX` when
    /// applied to `MIN`.
    pub const fn abs(self) -> Self {
        Self {
            duration: self.duration.saturating_abs(),
        }
    }

    /// Returns the sign of this `TimeSpan`: `-1`, `0`, or `1`.
    pub const fn signum(self) -> i64 {
        self.duration.signum()
    }

    /// Checked addition. Returns `None` on overflow.
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.duration.checked_add(rhs.duration) {
            Some(d) => Some(Self { duration: d }),
            None => None,
        }
    }

    /// Checked subtraction. Returns `None` on overflow.
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.duration.checked_sub(rhs.duration) {
            Some(d) => Some(Self { duration: d }),
            None => None,
        }
    }

    /// Checked negation. Returns `None` when applied to `MIN`.
    pub const fn checked_neg(self) -> Option<Self> {
        match self.duration.checked_neg() {
            Some(d) => Some(Self { duration: d }),
            None => None,
        }
    }

    /// Checked multiplication by an integer. Returns `None` on overflow.
    pub const fn checked_mul(self, rhs: i64) -> Option<Self> {
        match self.duration.checked_mul(rhs) {
            Some(d) => Some(Self { duration: d }),
            None => None,
        }
    }

    /// Checked division by an integer. Returns `None` if `rhs` is zero or on
    /// overflow.
    pub const fn checked_div(self, rhs: i64) -> Option<Self> {
        match self.duration.checked_div(rhs) {
            Some(d) => Some(Self { duration: d }),
            None => None,
        }
    }

    /// Saturating addition.
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self {
            duration: self.duration.saturating_add(rhs.duration),
        }
    }

    /// Saturating subtraction.
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self {
            duration: self.duration.saturating_sub(rhs.duration),
        }
    }

    /// Saturating negation.
    pub const fn saturating_neg(self) -> Self {
        Self {
            duration: match self.duration.checked_neg() {
                Some(d) => d,
                None => i64::MAX,
            },
        }
    }
}

impl Eq for TimeSpan {}

impl PartialOrd for TimeSpan {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimeSpan {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.duration.cmp(&other.duration)
    }
}

impl core::hash::Hash for TimeSpan {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.duration.hash(state);
    }
}

impl core::ops::Add for TimeSpan {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.checked_add(rhs)
            .expect("overflow when adding TimeSpan values")
    }
}

impl core::ops::Sub for TimeSpan {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self.checked_sub(rhs)
            .expect("overflow when subtracting TimeSpan values")
    }
}

impl core::ops::Neg for TimeSpan {
    type Output = Self;
    fn neg(self) -> Self {
        self.checked_neg()
            .expect("overflow when negating TimeSpan value")
    }
}

impl core::ops::AddAssign for TimeSpan {
    fn add_assign(&mut self, rhs: Self) {
        self.duration = self
            .duration
            .checked_add(rhs.duration)
            .expect("overflow when adding TimeSpan values");
    }
}

impl core::ops::SubAssign for TimeSpan {
    fn sub_assign(&mut self, rhs: Self) {
        self.duration = self
            .duration
            .checked_sub(rhs.duration)
            .expect("overflow when subtracting TimeSpan values");
    }
}

impl core::ops::Mul<i64> for TimeSpan {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self {
        self.checked_mul(rhs)
            .expect("overflow when multiplying TimeSpan value")
    }
}

impl core::ops::Div<i64> for TimeSpan {
    type Output = Self;
    fn div(self, rhs: i64) -> Self {
        self.checked_div(rhs).expect(if rhs == 0 {
            "attempt to divide TimeSpan by zero"
        } else {
            "overflow when dividing TimeSpan value"
        })
    }
}

/// ISO-8601 duration formatting: `PnDTnHnMn.fffffffS`. Negative values are
/// prefixed with `-`. The `ZERO` value formats as `PT0S`.
impl core::fmt::Display for TimeSpan {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.duration == 0 {
            return f.write_str("PT0S");
        }

        let (sign, ticks) = if self.duration < 0 {
            ("-", (self.duration as i128).unsigned_abs())
        } else {
            ("", self.duration as u128)
        };

        let total_seconds = ticks / TICKS_PER_SECOND as u128;
        let fraction = (ticks % TICKS_PER_SECOND as u128) as u32;

        let days = total_seconds / 86_400;
        let rem = total_seconds % 86_400;
        let hours = rem / 3_600;
        let rem = rem % 3_600;
        let minutes = rem / 60;
        let seconds = rem % 60;

        f.write_str(sign)?;
        f.write_str("P")?;
        if days != 0 {
            write!(f, "{days}D")?;
        }

        let need_time = hours != 0 || minutes != 0 || seconds != 0 || fraction != 0 || days == 0;
        if need_time {
            f.write_str("T")?;
            if hours != 0 {
                write!(f, "{hours}H")?;
            }
            if minutes != 0 {
                write!(f, "{minutes}M")?;
            }
            if seconds != 0 || fraction != 0 || (hours == 0 && minutes == 0) {
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
                    write!(f, "{seconds}.")?;
                    for d in &digits[..len] {
                        write!(f, "{d}")?;
                    }
                    f.write_str("S")?;
                } else {
                    write!(f, "{seconds}S")?;
                }
            }
        }
        Ok(())
    }
}

/// Error returned when a conversion to `TimeSpan` or `DateTime` cannot
/// represent the source value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct TimeRangeError;

impl core::fmt::Display for TimeRangeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("value out of range for the target time type")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TimeRangeError {}

impl TryFrom<core::time::Duration> for TimeSpan {
    type Error = TimeRangeError;
    fn try_from(value: core::time::Duration) -> Result<Self, Self::Error> {
        // 100-ns ticks
        let ticks = value.as_nanos() / 100;
        if ticks > i64::MAX as u128 {
            return Err(TimeRangeError);
        }
        Ok(Self {
            duration: ticks as i64,
        })
    }
}

impl TryFrom<TimeSpan> for core::time::Duration {
    type Error = TimeRangeError;
    fn try_from(value: TimeSpan) -> Result<Self, Self::Error> {
        if value.duration < 0 {
            return Err(TimeRangeError);
        }
        // i64 ticks * 100 fits in u128
        let nanos = (value.duration as u128) * 100;
        let secs = (nanos / 1_000_000_000) as u64;
        let subsec_nanos = (nanos % 1_000_000_000) as u32;
        Ok(Self::new(secs, subsec_nanos))
    }
}
