impl ::core::cmp::PartialEq for IReferenceClock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceClock {}
impl ::core::fmt::Debug for IReferenceClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceClock").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IReferenceClock2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceClock2 {}
impl ::core::fmt::Debug for IReferenceClock2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceClock2").field(&self.0).finish()
    }
}
impl IReferenceClock2 {
    pub unsafe fn GetTime(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AdviseTime<P0>(&self, basetime: i64, streamtime: i64, hevent: P0) -> ::windows::core::Result<usize>
    where
        P0: ::std::convert::Into<super::Foundation::HANDLE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AdviseTime)(::windows::core::Vtable::as_raw(self), basetime, streamtime, hevent.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AdvisePeriodic<P0>(&self, starttime: i64, periodtime: i64, hsemaphore: P0) -> ::windows::core::Result<usize>
    where
        P0: ::std::convert::Into<super::Foundation::HANDLE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AdvisePeriodic)(::windows::core::Vtable::as_raw(self), starttime, periodtime, hsemaphore.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Unadvise(&self, dwadvisecookie: usize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Unadvise)(::windows::core::Vtable::as_raw(self), dwadvisecookie).ok()
    }
}
impl ::core::cmp::PartialEq for IReferenceClockTimerControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceClockTimerControl {}
impl ::core::fmt::Debug for IReferenceClockTimerControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceClockTimerControl").field(&self.0).finish()
    }
}
impl ::core::default::Default for MMTIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TIMECAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TIMECAPS {
    fn eq(&self, other: &Self) -> bool {
        self.wPeriodMin == other.wPeriodMin && self.wPeriodMax == other.wPeriodMax
    }
}
impl ::core::cmp::Eq for TIMECAPS {}
impl ::core::fmt::Debug for TIMECAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TIMECAPS").field("wPeriodMin", &self.wPeriodMin).field("wPeriodMax", &self.wPeriodMax).finish()
    }
}
impl ::core::default::Default for TIMECODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TIMECODE_SAMPLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TIMECODE_SAMPLE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TIMECODE_SAMPLE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TIMECODE_SAMPLE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TIMECODE_SAMPLE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TIMECODE_SAMPLE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TIMECODE_SAMPLE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TIMECODE_SAMPLE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TIMECODE_SAMPLE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
