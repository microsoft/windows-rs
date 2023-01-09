impl ::core::cmp::PartialEq for Calendar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Calendar {}
impl ::core::fmt::Debug for Calendar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Calendar").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CurrencyAmount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CurrencyAmount {}
impl ::core::fmt::Debug for CurrencyAmount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CurrencyAmount").field(&self.0).finish()
    }
}
impl ::core::default::Default for DayOfWeek {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DayOfWeek {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DayOfWeek").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GeographicRegion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeographicRegion {}
impl ::core::fmt::Debug for GeographicRegion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeographicRegion").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for JapanesePhoneme {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JapanesePhoneme {}
impl ::core::fmt::Debug for JapanesePhoneme {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JapanesePhoneme").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Language {}
impl ::core::fmt::Debug for Language {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Language").field(&self.0).finish()
    }
}
impl ::core::default::Default for LanguageLayoutDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LanguageLayoutDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LanguageLayoutDirection").field(&self.0).finish()
    }
}
