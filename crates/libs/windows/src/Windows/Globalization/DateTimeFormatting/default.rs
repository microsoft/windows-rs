impl ::core::cmp::PartialEq for DateTimeFormatter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DateTimeFormatter {}
impl ::core::fmt::Debug for DateTimeFormatter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DateTimeFormatter").field(&self.0).finish()
    }
}
impl ::core::default::Default for DayFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DayFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DayFormat").field(&self.0).finish()
    }
}
impl ::core::default::Default for DayOfWeekFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DayOfWeekFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DayOfWeekFormat").field(&self.0).finish()
    }
}
impl ::core::default::Default for HourFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HourFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HourFormat").field(&self.0).finish()
    }
}
impl ::core::default::Default for MinuteFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MinuteFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MinuteFormat").field(&self.0).finish()
    }
}
impl ::core::default::Default for MonthFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MonthFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MonthFormat").field(&self.0).finish()
    }
}
impl ::core::default::Default for SecondFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SecondFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondFormat").field(&self.0).finish()
    }
}
impl ::core::default::Default for YearFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for YearFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("YearFormat").field(&self.0).finish()
    }
}
