impl ::core::default::Default for JsErrorCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JsErrorCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsErrorCode").field(&self.0).finish()
    }
}
impl ::core::default::Default for JsMemoryEventType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JsMemoryEventType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsMemoryEventType").field(&self.0).finish()
    }
}
impl ::core::default::Default for JsRuntimeAttributes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JsRuntimeAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsRuntimeAttributes").field(&self.0).finish()
    }
}
impl ::core::default::Default for JsRuntimeVersion {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JsRuntimeVersion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsRuntimeVersion").field(&self.0).finish()
    }
}
impl ::core::default::Default for JsValueType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JsValueType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JsValueType").field(&self.0).finish()
    }
}
