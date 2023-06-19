impl BOOL {
    #[inline]
    pub fn as_bool(self) -> bool {
        self.0 != 0
    }
    #[inline]
    pub fn ok(self) -> ::windows_core::Result<()> {
        if self.as_bool() {
            Ok(())
        } else {
            Err(::windows_core::Error::from_win32())
        }
    }
    #[inline]
    #[track_caller]
    pub fn unwrap(self) {
        self.ok().unwrap();
    }
    #[inline]
    #[track_caller]
    pub fn expect(self, msg: &str) {
        self.ok().expect(msg);
    }
}
impl ::core::convert::From<BOOL> for bool {
    fn from(value: BOOL) -> Self {
        value.as_bool()
    }
}
impl ::core::convert::From<&BOOL> for bool {
    fn from(value: &BOOL) -> Self {
        value.as_bool()
    }
}
impl ::core::convert::From<bool> for BOOL {
    fn from(value: bool) -> Self {
        if value {
            Self(1)
        } else {
            Self(0)
        }
    }
}
impl ::core::convert::From<&bool> for BOOL {
    fn from(value: &bool) -> Self {
        (*value).into()
    }
}
impl ::core::cmp::PartialEq<bool> for BOOL {
    fn eq(&self, other: &bool) -> bool {
        self.as_bool() == *other
    }
}
impl ::core::cmp::PartialEq<BOOL> for bool {
    fn eq(&self, other: &BOOL) -> bool {
        *self == other.as_bool()
    }
}
impl ::core::ops::Not for BOOL {
    type Output = Self;
    fn not(self) -> Self::Output {
        if self.as_bool() {
            Self(0)
        } else {
            Self(1)
        }
    }
}
impl ::windows_core::IntoParam<BOOL> for bool {
    fn into_param(self) -> ::windows_core::Param<BOOL> {
        ::windows_core::Param::Owned(self.into())
    }
}
