#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ENTERPRISE_DATA_POLICIES(pub u32);
pub const ENTERPRISE_POLICY_NONE: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(0u32);
pub const ENTERPRISE_POLICY_ALLOWED: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(1u32);
pub const ENTERPRISE_POLICY_ENLIGHTENED: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(2u32);
pub const ENTERPRISE_POLICY_EXEMPT: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(4u32);
impl ::core::convert::From<u32> for ENTERPRISE_DATA_POLICIES {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ENTERPRISE_DATA_POLICIES {
    type Abi = Self;
}
impl ::core::ops::BitOr for ENTERPRISE_DATA_POLICIES {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for ENTERPRISE_DATA_POLICIES {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for ENTERPRISE_DATA_POLICIES {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for ENTERPRISE_DATA_POLICIES {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for ENTERPRISE_DATA_POLICIES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct FILE_UNPROTECT_OPTIONS {
    pub audit: bool,
}
impl FILE_UNPROTECT_OPTIONS {}
impl ::core::default::Default for FILE_UNPROTECT_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for FILE_UNPROTECT_OPTIONS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FILE_UNPROTECT_OPTIONS").field("audit", &self.audit).finish()
    }
}
impl ::core::cmp::PartialEq for FILE_UNPROTECT_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.audit == other.audit
    }
}
impl ::core::cmp::Eq for FILE_UNPROTECT_OPTIONS {}
unsafe impl ::windows::core::Abi for FILE_UNPROTECT_OPTIONS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTHREAD_NETWORK_CONTEXT {
    pub ThreadId: u32,
    pub ThreadContext: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl HTHREAD_NETWORK_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HTHREAD_NETWORK_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HTHREAD_NETWORK_CONTEXT {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HTHREAD_NETWORK_CONTEXT").field("ThreadId", &self.ThreadId).field("ThreadContext", &self.ThreadContext).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HTHREAD_NETWORK_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.ThreadId == other.ThreadId && self.ThreadContext == other.ThreadContext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HTHREAD_NETWORK_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HTHREAD_NETWORK_CONTEXT {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IProtectionPolicyManagerInterop(pub ::windows::core::IUnknown);
impl IProtectionPolicyManagerInterop {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessForWindowAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, T: ::windows::core::Interface>(&self, appwindow: Param0, sourceidentity: Param1, targetidentity: Param2) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), appwindow.into_param().abi(), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::core::Interface>(&self, appwindow: Param0) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), appwindow.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for IProtectionPolicyManagerInterop {
    type Vtable = IProtectionPolicyManagerInterop_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4652651d_c1fe_4ba1_9f0a_c0f56596f721);
}
impl ::core::convert::From<IProtectionPolicyManagerInterop> for ::windows::core::IUnknown {
    fn from(value: IProtectionPolicyManagerInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProtectionPolicyManagerInterop> for ::windows::core::IUnknown {
    fn from(value: &IProtectionPolicyManagerInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProtectionPolicyManagerInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IProtectionPolicyManagerInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IProtectionPolicyManagerInterop2(pub ::windows::core::IUnknown);
impl IProtectionPolicyManagerInterop2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessForAppWithWindowAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, T: ::windows::core::Interface>(&self, appwindow: Param0, sourceidentity: Param1, apppackagefamilyname: Param2) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), appwindow.into_param().abi(), sourceidentity.into_param().abi(), apppackagefamilyname.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessWithAuditingInfoForWindowAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(&self, appwindow: Param0, sourceidentity: Param1, targetidentity: Param2, auditinfounk: Param3) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), appwindow.into_param().abi(), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), auditinfounk.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessWithMessageForWindowAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, T: ::windows::core::Interface>(&self, appwindow: Param0, sourceidentity: Param1, targetidentity: Param2, auditinfounk: Param3, messagefromapp: Param4) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), appwindow.into_param().abi(), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), auditinfounk.into_param().abi(), messagefromapp.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessForAppWithAuditingInfoForWindowAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(&self, appwindow: Param0, sourceidentity: Param1, apppackagefamilyname: Param2, auditinfounk: Param3) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), appwindow.into_param().abi(), sourceidentity.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfounk.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessForAppWithMessageForWindowAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, T: ::windows::core::Interface>(&self, appwindow: Param0, sourceidentity: Param1, apppackagefamilyname: Param2, auditinfounk: Param3, messagefromapp: Param4) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), appwindow.into_param().abi(), sourceidentity.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfounk.into_param().abi(), messagefromapp.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for IProtectionPolicyManagerInterop2 {
    type Vtable = IProtectionPolicyManagerInterop2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x157cfbe4_a78d_4156_b384_61fdac41e686);
}
impl ::core::convert::From<IProtectionPolicyManagerInterop2> for ::windows::core::IUnknown {
    fn from(value: IProtectionPolicyManagerInterop2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProtectionPolicyManagerInterop2> for ::windows::core::IUnknown {
    fn from(value: &IProtectionPolicyManagerInterop2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProtectionPolicyManagerInterop2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IProtectionPolicyManagerInterop2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerInterop2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: ::windows::core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: ::windows::core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IProtectionPolicyManagerInterop3(pub ::windows::core::IUnknown);
impl IProtectionPolicyManagerInterop3 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessWithBehaviorForWindowAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, T: ::windows::core::Interface>(&self, appwindow: Param0, sourceidentity: Param1, targetidentity: Param2, auditinfounk: Param3, messagefromapp: Param4, behavior: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), appwindow.into_param().abi(), sourceidentity.into_param().abi(), targetidentity.into_param().abi(), auditinfounk.into_param().abi(), messagefromapp.into_param().abi(), ::core::mem::transmute(behavior), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessForAppWithBehaviorForWindowAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, T: ::windows::core::Interface>(&self, appwindow: Param0, sourceidentity: Param1, apppackagefamilyname: Param2, auditinfounk: Param3, messagefromapp: Param4, behavior: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), appwindow.into_param().abi(), sourceidentity.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfounk.into_param().abi(), messagefromapp.into_param().abi(), ::core::mem::transmute(behavior), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessToFilesForAppForWindowAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(&self, appwindow: Param0, sourceitemlistunk: Param1, apppackagefamilyname: Param2, auditinfounk: Param3) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), appwindow.into_param().abi(), sourceitemlistunk.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfounk.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, T: ::windows::core::Interface>(&self, appwindow: Param0, sourceitemlistunk: Param1, apppackagefamilyname: Param2, auditinfounk: Param3, messagefromapp: Param4, behavior: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), appwindow.into_param().abi(), sourceitemlistunk.into_param().abi(), apppackagefamilyname.into_param().abi(), auditinfounk.into_param().abi(), messagefromapp.into_param().abi(), ::core::mem::transmute(behavior), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessToFilesForProcessForWindowAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, T: ::windows::core::Interface>(&self, appwindow: Param0, sourceitemlistunk: Param1, processid: u32, auditinfounk: Param3) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), appwindow.into_param().abi(), sourceitemlistunk.into_param().abi(), ::core::mem::transmute(processid), auditinfounk.into_param().abi(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param3: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, T: ::windows::core::Interface>(&self, appwindow: Param0, sourceitemlistunk: Param1, processid: u32, auditinfounk: Param3, messagefromapp: Param4, behavior: u32) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), appwindow.into_param().abi(), sourceitemlistunk.into_param().abi(), ::core::mem::transmute(processid), auditinfounk.into_param().abi(), messagefromapp.into_param().abi(), ::core::mem::transmute(behavior), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for IProtectionPolicyManagerInterop3 {
    type Vtable = IProtectionPolicyManagerInterop3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1c03933_b398_4d93_b0fd_2972adf802c2);
}
impl ::core::convert::From<IProtectionPolicyManagerInterop3> for ::windows::core::IUnknown {
    fn from(value: IProtectionPolicyManagerInterop3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProtectionPolicyManagerInterop3> for ::windows::core::IUnknown {
    fn from(value: &IProtectionPolicyManagerInterop3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProtectionPolicyManagerInterop3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IProtectionPolicyManagerInterop3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerInterop3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: ::windows::core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behavior: u32, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appwindow: super::super::Foundation::HWND, sourceidentity: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: ::windows::core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behavior: u32, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appwindow: super::super::Foundation::HWND, sourceitemlistunk: ::windows::core::RawPtr, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appwindow: super::super::Foundation::HWND, sourceitemlistunk: ::windows::core::RawPtr, apppackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, auditinfounk: ::windows::core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behavior: u32, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appwindow: super::super::Foundation::HWND, sourceitemlistunk: ::windows::core::RawPtr, processid: u32, auditinfounk: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appwindow: super::super::Foundation::HWND, sourceitemlistunk: ::windows::core::RawPtr, processid: u32, auditinfounk: ::windows::core::RawPtr, messagefromapp: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, behavior: u32, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ProtectFileToEnterpriseIdentity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(fileorfolderpath: Param0, identity: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProtectFileToEnterpriseIdentity(fileorfolderpath: super::super::Foundation::PWSTR, identity: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        ProtectFileToEnterpriseIdentity(fileorfolderpath.into_param().abi(), identity.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SRPHOSTING_TYPE(pub i32);
pub const SRPHOSTING_TYPE_NONE: SRPHOSTING_TYPE = SRPHOSTING_TYPE(0i32);
pub const SRPHOSTING_TYPE_WINHTTP: SRPHOSTING_TYPE = SRPHOSTING_TYPE(1i32);
pub const SRPHOSTING_TYPE_WININET: SRPHOSTING_TYPE = SRPHOSTING_TYPE(2i32);
impl ::core::convert::From<i32> for SRPHOSTING_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SRPHOSTING_TYPE {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SRPHOSTING_VERSION(pub i32);
pub const SRPHOSTING_VERSION1: SRPHOSTING_VERSION = SRPHOSTING_VERSION(1i32);
impl ::core::convert::From<i32> for SRPHOSTING_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SRPHOSTING_VERSION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SrpCloseThreadNetworkContext(threadnetworkcontext: *mut HTHREAD_NETWORK_CONTEXT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpCloseThreadNetworkContext(threadnetworkcontext: *mut HTHREAD_NETWORK_CONTEXT) -> ::windows::core::HRESULT;
        }
        SrpCloseThreadNetworkContext(::core::mem::transmute(threadnetworkcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SrpCreateThreadNetworkContext<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(enterpriseid: Param0) -> ::windows::core::Result<HTHREAD_NETWORK_CONTEXT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpCreateThreadNetworkContext(enterpriseid: super::super::Foundation::PWSTR, threadnetworkcontext: *mut HTHREAD_NETWORK_CONTEXT) -> ::windows::core::HRESULT;
        }
        let mut result__: <HTHREAD_NETWORK_CONTEXT as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        SrpCreateThreadNetworkContext(enterpriseid.into_param().abi(), &mut result__).from_abi::<HTHREAD_NETWORK_CONTEXT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SrpDisablePermissiveModeFileEncryption() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpDisablePermissiveModeFileEncryption() -> ::windows::core::HRESULT;
        }
        SrpDisablePermissiveModeFileEncryption().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Appx"))]
#[inline]
pub unsafe fn SrpDoesPolicyAllowAppExecution(packageid: *const super::super::Storage::Packaging::Appx::PACKAGE_ID) -> ::windows::core::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpDoesPolicyAllowAppExecution(packageid: *const super::super::Storage::Packaging::Appx::PACKAGE_ID, isallowed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        SrpDoesPolicyAllowAppExecution(::core::mem::transmute(packageid), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SrpEnablePermissiveModeFileEncryption<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(enterpriseid: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpEnablePermissiveModeFileEncryption(enterpriseid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        SrpEnablePermissiveModeFileEncryption(enterpriseid.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SrpGetEnterpriseIds<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(tokenhandle: Param0, numberofbytes: *mut u32, enterpriseids: *mut super::super::Foundation::PWSTR, enterpriseidcount: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpGetEnterpriseIds(tokenhandle: super::super::Foundation::HANDLE, numberofbytes: *mut u32, enterpriseids: *mut super::super::Foundation::PWSTR, enterpriseidcount: *mut u32) -> ::windows::core::HRESULT;
        }
        SrpGetEnterpriseIds(tokenhandle.into_param().abi(), ::core::mem::transmute(numberofbytes), ::core::mem::transmute(enterpriseids), ::core::mem::transmute(enterpriseidcount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SrpGetEnterprisePolicy<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(tokenhandle: Param0) -> ::windows::core::Result<ENTERPRISE_DATA_POLICIES> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpGetEnterprisePolicy(tokenhandle: super::super::Foundation::HANDLE, policyflags: *mut ENTERPRISE_DATA_POLICIES) -> ::windows::core::HRESULT;
        }
        let mut result__: <ENTERPRISE_DATA_POLICIES as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        SrpGetEnterprisePolicy(tokenhandle.into_param().abi(), &mut result__).from_abi::<ENTERPRISE_DATA_POLICIES>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SrpHostingInitialize(version: SRPHOSTING_VERSION, r#type: SRPHOSTING_TYPE, pvdata: *const ::core::ffi::c_void, cbdata: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpHostingInitialize(version: SRPHOSTING_VERSION, r#type: SRPHOSTING_TYPE, pvdata: *const ::core::ffi::c_void, cbdata: u32) -> ::windows::core::HRESULT;
        }
        SrpHostingInitialize(::core::mem::transmute(version), ::core::mem::transmute(r#type), ::core::mem::transmute(pvdata), ::core::mem::transmute(cbdata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SrpHostingTerminate(r#type: SRPHOSTING_TYPE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpHostingTerminate(r#type: SRPHOSTING_TYPE);
        }
        ::core::mem::transmute(SrpHostingTerminate(::core::mem::transmute(r#type)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SrpIsTokenService<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(tokenhandle: Param0, istokenservice: *mut u8) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpIsTokenService(tokenhandle: super::super::Foundation::HANDLE, istokenservice: *mut u8) -> super::super::Foundation::NTSTATUS;
        }
        SrpIsTokenService(tokenhandle.into_param().abi(), ::core::mem::transmute(istokenservice)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SrpSetTokenEnterpriseId<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(tokenhandle: Param0, enterpriseid: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpSetTokenEnterpriseId(tokenhandle: super::super::Foundation::HANDLE, enterpriseid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        SrpSetTokenEnterpriseId(tokenhandle.into_param().abi(), enterpriseid.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnprotectFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(fileorfolderpath: Param0, options: *const FILE_UNPROTECT_OPTIONS) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnprotectFile(fileorfolderpath: super::super::Foundation::PWSTR, options: *const FILE_UNPROTECT_OPTIONS) -> ::windows::core::HRESULT;
        }
        UnprotectFile(fileorfolderpath.into_param().abi(), ::core::mem::transmute(options)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
