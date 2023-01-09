impl ::core::default::Default for COMDLG_FILTERSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for COMDLG_FILTERSPEC {
    fn eq(&self, other: &Self) -> bool {
        self.pszName == other.pszName && self.pszSpec == other.pszSpec
    }
}
impl ::core::cmp::Eq for COMDLG_FILTERSPEC {}
impl ::core::fmt::Debug for COMDLG_FILTERSPEC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMDLG_FILTERSPEC").field("pszName", &self.pszName).field("pszSpec", &self.pszSpec).finish()
    }
}
impl ::core::default::Default for DEVICE_SCALE_FACTOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEVICE_SCALE_FACTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVICE_SCALE_FACTOR").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IObjectArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectArray {}
impl ::core::fmt::Debug for IObjectArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectArray").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IObjectCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectCollection {}
impl ::core::fmt::Debug for IObjectCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectCollection").field(&self.0).finish()
    }
}
impl IObjectCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAt<T>(&self, uiindex: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAt)(::windows::core::Vtable::as_raw(self), uiindex, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::default::Default for ITEMIDLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PERCEIVED {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PERCEIVED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PERCEIVED").field(&self.0).finish()
    }
}
impl ::core::default::Default for SHCOLSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHCOLSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHCOLSTATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SHELLDETAILS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SHITEMID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for STRRET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for STRRET_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STRRET_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STRRET_TYPE").field(&self.0).finish()
    }
}
