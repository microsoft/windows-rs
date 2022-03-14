#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
#[repr(transparent)]
pub struct ITpmVirtualSmartCardManager(::windows::core::IUnknown);
impl ITpmVirtualSmartCardManager {
    #[doc = "*Required features: `\"Win32_Security_Tpm\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCard<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param10: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param11: ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(&self, pszfriendlyname: Param0, badminalgid: u8, pbadminkey: &[u8], pbadminkcv: &[u8], pbpuk: &[u8], pbpin: &[u8], fgenerate: Param10, pstatuscallback: Param11, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateVirtualSmartCard)(
            ::core::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::core::mem::transmute(badminalgid),
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbadminkey)),
            pbadminkey.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbadminkcv)),
            pbadminkcv.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbpuk)),
            pbpuk.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbpin)),
            pbpin.len() as _,
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            ::core::mem::transmute(ppszinstanceid),
            ::core::mem::transmute(pfneedreboot),
        )
        .ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Tpm\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestroyVirtualSmartCard<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(&self, pszinstanceid: Param0, pstatuscallback: Param1) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).DestroyVirtualSmartCard)(::core::mem::transmute_copy(self), pszinstanceid.into_param().abi(), pstatuscallback.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<ITpmVirtualSmartCardManager> for ::windows::core::IUnknown {
    fn from(value: ITpmVirtualSmartCardManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITpmVirtualSmartCardManager> for ::windows::core::IUnknown {
    fn from(value: &ITpmVirtualSmartCardManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITpmVirtualSmartCardManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITpmVirtualSmartCardManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITpmVirtualSmartCardManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITpmVirtualSmartCardManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITpmVirtualSmartCardManager {}
impl ::core::fmt::Debug for ITpmVirtualSmartCardManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITpmVirtualSmartCardManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITpmVirtualSmartCardManager {
    type Vtable = ITpmVirtualSmartCardManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x112b1dff_d9dc_41f7_869f_d67fee7cb591);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateVirtualSmartCard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfriendlyname: ::windows::core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::windows::core::RawPtr, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateVirtualSmartCard: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DestroyVirtualSmartCard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszinstanceid: ::windows::core::PCWSTR, pstatuscallback: ::windows::core::RawPtr, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DestroyVirtualSmartCard: usize,
}
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
#[repr(transparent)]
pub struct ITpmVirtualSmartCardManager2(::windows::core::IUnknown);
impl ITpmVirtualSmartCardManager2 {
    #[doc = "*Required features: `\"Win32_Security_Tpm\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCard<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param10: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param11: ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(&self, pszfriendlyname: Param0, badminalgid: u8, pbadminkey: &[u8], pbadminkcv: &[u8], pbpuk: &[u8], pbpin: &[u8], fgenerate: Param10, pstatuscallback: Param11, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CreateVirtualSmartCard)(
            ::core::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::core::mem::transmute(badminalgid),
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbadminkey)),
            pbadminkey.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbadminkcv)),
            pbadminkcv.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbpuk)),
            pbpuk.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbpin)),
            pbpin.len() as _,
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            ::core::mem::transmute(ppszinstanceid),
            ::core::mem::transmute(pfneedreboot),
        )
        .ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Tpm\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestroyVirtualSmartCard<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(&self, pszinstanceid: Param0, pstatuscallback: Param1) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.DestroyVirtualSmartCard)(::core::mem::transmute_copy(self), pszinstanceid.into_param().abi(), pstatuscallback.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Security_Tpm\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCardWithPinPolicy<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param12: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param13: ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(&self, pszfriendlyname: Param0, badminalgid: u8, pbadminkey: &[u8], pbadminkcv: &[u8], pbpuk: &[u8], pbpin: &[u8], pbpinpolicy: &[u8], fgenerate: Param12, pstatuscallback: Param13, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateVirtualSmartCardWithPinPolicy)(
            ::core::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::core::mem::transmute(badminalgid),
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbadminkey)),
            pbadminkey.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbadminkcv)),
            pbadminkcv.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbpuk)),
            pbpuk.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbpin)),
            pbpin.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbpinpolicy)),
            pbpinpolicy.len() as _,
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            ::core::mem::transmute(ppszinstanceid),
            ::core::mem::transmute(pfneedreboot),
        )
        .ok()
    }
}
impl ::core::convert::From<ITpmVirtualSmartCardManager2> for ::windows::core::IUnknown {
    fn from(value: ITpmVirtualSmartCardManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITpmVirtualSmartCardManager2> for ::windows::core::IUnknown {
    fn from(value: &ITpmVirtualSmartCardManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITpmVirtualSmartCardManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITpmVirtualSmartCardManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITpmVirtualSmartCardManager2> for ITpmVirtualSmartCardManager {
    fn from(value: ITpmVirtualSmartCardManager2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITpmVirtualSmartCardManager2> for ITpmVirtualSmartCardManager {
    fn from(value: &ITpmVirtualSmartCardManager2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManager> for ITpmVirtualSmartCardManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITpmVirtualSmartCardManager> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManager> for &'a ITpmVirtualSmartCardManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITpmVirtualSmartCardManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITpmVirtualSmartCardManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITpmVirtualSmartCardManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITpmVirtualSmartCardManager2 {}
impl ::core::fmt::Debug for ITpmVirtualSmartCardManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITpmVirtualSmartCardManager2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITpmVirtualSmartCardManager2 {
    type Vtable = ITpmVirtualSmartCardManager2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdf8a2b9_02de_47f4_bc26_aa85ab5e5267);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManager2_Vtbl {
    pub base: ITpmVirtualSmartCardManager_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateVirtualSmartCardWithPinPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfriendlyname: ::windows::core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::windows::core::RawPtr, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateVirtualSmartCardWithPinPolicy: usize,
}
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
#[repr(transparent)]
pub struct ITpmVirtualSmartCardManager3(::windows::core::IUnknown);
impl ITpmVirtualSmartCardManager3 {
    #[doc = "*Required features: `\"Win32_Security_Tpm\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCard<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param10: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param11: ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(&self, pszfriendlyname: Param0, badminalgid: u8, pbadminkey: &[u8], pbadminkcv: &[u8], pbpuk: &[u8], pbpin: &[u8], fgenerate: Param10, pstatuscallback: Param11, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.CreateVirtualSmartCard)(
            ::core::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::core::mem::transmute(badminalgid),
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbadminkey)),
            pbadminkey.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbadminkcv)),
            pbadminkcv.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbpuk)),
            pbpuk.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbpin)),
            pbpin.len() as _,
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            ::core::mem::transmute(ppszinstanceid),
            ::core::mem::transmute(pfneedreboot),
        )
        .ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Tpm\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestroyVirtualSmartCard<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(&self, pszinstanceid: Param0, pstatuscallback: Param1) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.base.DestroyVirtualSmartCard)(::core::mem::transmute_copy(self), pszinstanceid.into_param().abi(), pstatuscallback.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Security_Tpm\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCardWithPinPolicy<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param12: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param13: ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(&self, pszfriendlyname: Param0, badminalgid: u8, pbadminkey: &[u8], pbadminkcv: &[u8], pbpuk: &[u8], pbpin: &[u8], pbpinpolicy: &[u8], fgenerate: Param12, pstatuscallback: Param13, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.CreateVirtualSmartCardWithPinPolicy)(
            ::core::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::core::mem::transmute(badminalgid),
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbadminkey)),
            pbadminkey.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbadminkcv)),
            pbadminkcv.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbpuk)),
            pbpuk.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbpin)),
            pbpin.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbpinpolicy)),
            pbpinpolicy.len() as _,
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            ::core::mem::transmute(ppszinstanceid),
            ::core::mem::transmute(pfneedreboot),
        )
        .ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Tpm\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCardWithAttestation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param13: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param14: ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(&self, pszfriendlyname: Param0, badminalgid: u8, pbadminkey: &[u8], pbadminkcv: &[u8], pbpuk: &[u8], pbpin: &[u8], pbpinpolicy: &[u8], attestationtype: TPMVSC_ATTESTATION_TYPE, fgenerate: Param13, pstatuscallback: Param14) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateVirtualSmartCardWithAttestation)(
            ::core::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::core::mem::transmute(badminalgid),
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbadminkey)),
            pbadminkey.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbadminkcv)),
            pbadminkcv.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbpuk)),
            pbpuk.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbpin)),
            pbpin.len() as _,
            ::core::mem::transmute(::windows::core::as_ptr_or_null(pbpinpolicy)),
            pbpinpolicy.len() as _,
            ::core::mem::transmute(attestationtype),
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            ::core::mem::transmute(&mut result__),
        )
        .from_abi::<::windows::core::PWSTR>(result__)
    }
}
impl ::core::convert::From<ITpmVirtualSmartCardManager3> for ::windows::core::IUnknown {
    fn from(value: ITpmVirtualSmartCardManager3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITpmVirtualSmartCardManager3> for ::windows::core::IUnknown {
    fn from(value: &ITpmVirtualSmartCardManager3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITpmVirtualSmartCardManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITpmVirtualSmartCardManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITpmVirtualSmartCardManager3> for ITpmVirtualSmartCardManager {
    fn from(value: ITpmVirtualSmartCardManager3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITpmVirtualSmartCardManager3> for ITpmVirtualSmartCardManager {
    fn from(value: &ITpmVirtualSmartCardManager3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManager> for ITpmVirtualSmartCardManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, ITpmVirtualSmartCardManager> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManager> for &'a ITpmVirtualSmartCardManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, ITpmVirtualSmartCardManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITpmVirtualSmartCardManager3> for ITpmVirtualSmartCardManager2 {
    fn from(value: ITpmVirtualSmartCardManager3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITpmVirtualSmartCardManager3> for ITpmVirtualSmartCardManager2 {
    fn from(value: &ITpmVirtualSmartCardManager3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManager2> for ITpmVirtualSmartCardManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, ITpmVirtualSmartCardManager2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManager2> for &'a ITpmVirtualSmartCardManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, ITpmVirtualSmartCardManager2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITpmVirtualSmartCardManager3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITpmVirtualSmartCardManager3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITpmVirtualSmartCardManager3 {}
impl ::core::fmt::Debug for ITpmVirtualSmartCardManager3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITpmVirtualSmartCardManager3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITpmVirtualSmartCardManager3 {
    type Vtable = ITpmVirtualSmartCardManager3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c745a97_f375_4150_be17_5950f694c699);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManager3_Vtbl {
    pub base: ITpmVirtualSmartCardManager2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateVirtualSmartCardWithAttestation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfriendlyname: ::windows::core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, attestationtype: TPMVSC_ATTESTATION_TYPE, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::windows::core::RawPtr, ppszinstanceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateVirtualSmartCardWithAttestation: usize,
}
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
#[repr(transparent)]
pub struct ITpmVirtualSmartCardManagerStatusCallback(::windows::core::IUnknown);
impl ITpmVirtualSmartCardManagerStatusCallback {
    #[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
    pub unsafe fn ReportProgress(&self, status: TPMVSCMGR_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReportProgress)(::core::mem::transmute_copy(self), ::core::mem::transmute(status)).ok()
    }
    #[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
    pub unsafe fn ReportError(&self, error: TPMVSCMGR_ERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReportError)(::core::mem::transmute_copy(self), ::core::mem::transmute(error)).ok()
    }
}
impl ::core::convert::From<ITpmVirtualSmartCardManagerStatusCallback> for ::windows::core::IUnknown {
    fn from(value: ITpmVirtualSmartCardManagerStatusCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITpmVirtualSmartCardManagerStatusCallback> for ::windows::core::IUnknown {
    fn from(value: &ITpmVirtualSmartCardManagerStatusCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITpmVirtualSmartCardManagerStatusCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITpmVirtualSmartCardManagerStatusCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITpmVirtualSmartCardManagerStatusCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITpmVirtualSmartCardManagerStatusCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITpmVirtualSmartCardManagerStatusCallback {}
impl ::core::fmt::Debug for ITpmVirtualSmartCardManagerStatusCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITpmVirtualSmartCardManagerStatusCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITpmVirtualSmartCardManagerStatusCallback {
    type Vtable = ITpmVirtualSmartCardManagerStatusCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a1bb35f_abb8_451c_a1ae_33d98f1bef4a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManagerStatusCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub ReportProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: TPMVSCMGR_STATUS) -> ::windows::core::HRESULT,
    pub ReportError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, error: TPMVSCMGR_ERROR) -> ::windows::core::HRESULT,
}
pub const RemoteTpmVirtualSmartCardManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x152ea2a8_70dc_4c59_8b2a_32aa3ca0dcac);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TPMVSCMGR_ERROR(pub i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_IMPERSONATION: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(0i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_PIN_COMPLEXITY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(1i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_READER_COUNT_LIMIT: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(2i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_TERMINAL_SERVICES_SESSION: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(3i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_INITIALIZE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(4i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_CREATE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(5i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_DESTROY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(6i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_INITIALIZE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(7i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_CREATE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(8i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_DESTROY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(9i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_WRITE_PROPERTY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(10i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_READ_PROPERTY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(11i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VREADER_INITIALIZE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(12i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VREADER_CREATE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(13i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_VREADER_DESTROY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(14i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_GENERATE_LOCATE_READER: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(15i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_GENERATE_FILESYSTEM: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(16i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_CARD_CREATE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(17i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_ERROR_CARD_DESTROY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(18i32);
impl ::core::marker::Copy for TPMVSCMGR_ERROR {}
impl ::core::clone::Clone for TPMVSCMGR_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TPMVSCMGR_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TPMVSCMGR_ERROR {
    type Abi = Self;
}
impl ::core::fmt::Debug for TPMVSCMGR_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TPMVSCMGR_ERROR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TPMVSCMGR_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_INITIALIZING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_CREATING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_DESTROYING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_INITIALIZING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(3i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_CREATING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(4i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_DESTROYING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(5i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VREADER_INITIALIZING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(6i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VREADER_CREATING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(7i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_VREADER_DESTROYING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(8i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_GENERATE_WAITING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(9i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_GENERATE_AUTHENTICATING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(10i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_GENERATE_RUNNING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(11i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_CARD_CREATED: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(12i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSCMGR_STATUS_CARD_DESTROYED: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(13i32);
impl ::core::marker::Copy for TPMVSCMGR_STATUS {}
impl ::core::clone::Clone for TPMVSCMGR_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TPMVSCMGR_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TPMVSCMGR_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for TPMVSCMGR_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TPMVSCMGR_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TPMVSC_ATTESTATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSC_ATTESTATION_NONE: TPMVSC_ATTESTATION_TYPE = TPMVSC_ATTESTATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSC_ATTESTATION_AIK_ONLY: TPMVSC_ATTESTATION_TYPE = TPMVSC_ATTESTATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSC_ATTESTATION_AIK_AND_CERTIFICATE: TPMVSC_ATTESTATION_TYPE = TPMVSC_ATTESTATION_TYPE(2i32);
impl ::core::marker::Copy for TPMVSC_ATTESTATION_TYPE {}
impl ::core::clone::Clone for TPMVSC_ATTESTATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TPMVSC_ATTESTATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TPMVSC_ATTESTATION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for TPMVSC_ATTESTATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TPMVSC_ATTESTATION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSC_DEFAULT_ADMIN_ALGORITHM_ID: u32 = 130u32;
pub const TpmVirtualSmartCardManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16a18e86_7f6e_4c20_ad89_4ffc0db7a96a);
#[cfg(feature = "implement")]
::core::include!("impl.rs");
