impl ::core::cmp::PartialEq for MdmAlert {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MdmAlert {}
impl ::core::fmt::Debug for MdmAlert {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MdmAlert").field(&self.0).finish()
    }
}
impl ::core::default::Default for MdmAlertDataType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MdmAlertDataType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MdmAlertDataType").field(&self.0).finish()
    }
}
impl ::core::default::Default for MdmAlertMark {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MdmAlertMark {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MdmAlertMark").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MdmSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MdmSession {}
impl ::core::fmt::Debug for MdmSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MdmSession").field(&self.0).finish()
    }
}
impl ::core::default::Default for MdmSessionState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MdmSessionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MdmSessionState").field(&self.0).finish()
    }
}
