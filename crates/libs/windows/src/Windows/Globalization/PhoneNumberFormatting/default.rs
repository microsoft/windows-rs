impl ::core::default::Default for PhoneNumberFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PhoneNumberFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNumberFormat").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PhoneNumberFormatter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneNumberFormatter {}
impl ::core::fmt::Debug for PhoneNumberFormatter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNumberFormatter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PhoneNumberInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneNumberInfo {}
impl ::core::fmt::Debug for PhoneNumberInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNumberInfo").field(&self.0).finish()
    }
}
impl ::core::default::Default for PhoneNumberMatchResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PhoneNumberMatchResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNumberMatchResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for PhoneNumberParseResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PhoneNumberParseResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneNumberParseResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for PredictedPhoneNumberKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PredictedPhoneNumberKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PredictedPhoneNumberKind").field(&self.0).finish()
    }
}
