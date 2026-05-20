use crate::Win32::Foundation::FILETIME;
use windows_time::DateTime;

// `FILETIME` and `DateTime` use the same epoch (1601-01-01 UTC) and the same
// 100-nanosecond tick unit. `FILETIME` stores its 64-bit count as a pair of
// `u32`s; `DateTime` stores its as a signed `i64`. Conversion is a lossless
// reinterpretation of the bit pattern.

impl From<FILETIME> for DateTime {
    fn from(value: FILETIME) -> Self {
        let ticks = ((value.dwHighDateTime as u64) << 32 | value.dwLowDateTime as u64) as i64;
        Self { UniversalTime: ticks }
    }
}

impl From<DateTime> for FILETIME {
    fn from(value: DateTime) -> Self {
        let ticks = value.UniversalTime as u64;
        Self { dwLowDateTime: ticks as u32, dwHighDateTime: (ticks >> 32) as u32 }
    }
}
