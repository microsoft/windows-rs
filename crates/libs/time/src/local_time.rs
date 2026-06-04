/// A decomposed local date and time, representing a snapshot of the system's
/// wall clock in the local timezone.
///
/// This type wraps the Win32 `GetLocalTime` function and exposes individual
/// calendar/clock fields. It is intended for display purposes (clocks, date
/// pickers, logs) — not for elapsed-time measurement or arithmetic.
///
/// # Example
///
/// ```no_run
/// use windows_time::LocalTime;
///
/// let now = LocalTime::now();
/// println!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
///     now.year(), now.month(), now.day(),
///     now.hour(), now.minute(), now.second());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LocalTime {
    year: u16,
    month: u16,
    day_of_week: u16,
    day: u16,
    hour: u16,
    minute: u16,
    second: u16,
    milliseconds: u16,
}

impl LocalTime {
    /// Creates a `LocalTime` from individual field values.
    ///
    /// Returns `None` if any field is out of its valid range:
    /// - `month`: 1–12
    /// - `day`: 1–31
    /// - `day_of_week`: 0–6 (0 = Sunday)
    /// - `hour`: 0–23
    /// - `minute`: 0–59
    /// - `second`: 0–59
    /// - `milliseconds`: 0–999
    ///
    /// Note: This does not validate that `day` is correct for the given
    /// month/year (e.g., Feb 30 would be accepted).
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        year: u16,
        month: u16,
        day: u16,
        day_of_week: u16,
        hour: u16,
        minute: u16,
        second: u16,
        milliseconds: u16,
    ) -> Option<Self> {
        if month < 1
            || month > 12
            || day < 1
            || day > 31
            || day_of_week > 6
            || hour > 23
            || minute > 59
            || second > 59
            || milliseconds > 999
        {
            return None;
        }
        Some(Self {
            year,
            month,
            day_of_week,
            day,
            hour,
            minute,
            second,
            milliseconds,
        })
    }

    /// Captures the current local date and time from the system clock.
    ///
    /// This reads the system's local wall clock (including timezone and DST
    /// adjustments). The returned value is a snapshot — it does not track
    /// subsequent clock changes.
    pub fn now() -> Self {
        let st = sys::get_local_time();
        Self {
            year: st.wYear,
            month: st.wMonth,
            day_of_week: st.wDayOfWeek,
            day: st.wDay,
            hour: st.wHour,
            minute: st.wMinute,
            second: st.wSecond,
            milliseconds: st.wMilliseconds,
        }
    }

    /// The year (e.g., 2025).
    pub const fn year(&self) -> u16 {
        self.year
    }

    /// The month of the year (1 = January, 12 = December).
    pub const fn month(&self) -> u16 {
        self.month
    }

    /// The day of the month (1–31).
    pub const fn day(&self) -> u16 {
        self.day
    }

    /// The day of the week (0 = Sunday, 6 = Saturday).
    pub const fn day_of_week(&self) -> u16 {
        self.day_of_week
    }

    /// The hour (0–23).
    pub const fn hour(&self) -> u16 {
        self.hour
    }

    /// The minute (0–59).
    pub const fn minute(&self) -> u16 {
        self.minute
    }

    /// The second (0–59).
    pub const fn second(&self) -> u16 {
        self.second
    }

    /// The milliseconds (0–999).
    pub const fn milliseconds(&self) -> u16 {
        self.milliseconds
    }
}

impl core::fmt::Display for LocalTime {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}",
            self.year, self.month, self.day, self.hour, self.minute, self.second, self.milliseconds,
        )
    }
}

#[allow(unsafe_code)]
mod sys {
    #[repr(C)]
    pub struct SystemTime {
        pub wYear: u16,
        pub wMonth: u16,
        pub wDayOfWeek: u16,
        pub wDay: u16,
        pub wHour: u16,
        pub wMinute: u16,
        pub wSecond: u16,
        pub wMilliseconds: u16,
    }

    windows_link::link!("kernel32.dll" "system" fn GetLocalTime(lpsystemtime: *mut SystemTime));

    pub fn get_local_time() -> SystemTime {
        let mut st = SystemTime {
            wYear: 0,
            wMonth: 0,
            wDayOfWeek: 0,
            wDay: 0,
            wHour: 0,
            wMinute: 0,
            wSecond: 0,
            wMilliseconds: 0,
        };
        // SAFETY: GetLocalTime always succeeds and writes to the provided pointer.
        // The SystemTime struct is repr(C) and matches the Win32 SYSTEMTIME layout.
        unsafe { GetLocalTime(&mut st) };
        st
    }
}
