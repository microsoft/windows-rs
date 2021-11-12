#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITpmVirtualSmartCardManager(pub ::windows::core::IUnknown);
impl ITpmVirtualSmartCardManager {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCard<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param10: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param11: ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(
        &self,
        pszfriendlyname: Param0,
        badminalgid: u8,
        pbadminkey: *const u8,
        cbadminkey: u32,
        pbadminkcv: *const u8,
        cbadminkcv: u32,
        pbpuk: *const u8,
        cbpuk: u32,
        pbpin: *const u8,
        cbpin: u32,
        fgenerate: Param10,
        pstatuscallback: Param11,
        ppszinstanceid: *mut super::super::Foundation::PWSTR,
        pfneedreboot: *mut super::super::Foundation::BOOL,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::core::mem::transmute(badminalgid),
            ::core::mem::transmute(pbadminkey),
            ::core::mem::transmute(cbadminkey),
            ::core::mem::transmute(pbadminkcv),
            ::core::mem::transmute(cbadminkcv),
            ::core::mem::transmute(pbpuk),
            ::core::mem::transmute(cbpuk),
            ::core::mem::transmute(pbpin),
            ::core::mem::transmute(cbpin),
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            ::core::mem::transmute(ppszinstanceid),
            ::core::mem::transmute(pfneedreboot),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestroyVirtualSmartCard<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(&self, pszinstanceid: Param0, pstatuscallback: Param1) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszinstanceid.into_param().abi(), pstatuscallback.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for ITpmVirtualSmartCardManager {
    type Vtable = ITpmVirtualSmartCardManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x112b1dff_d9dc_41f7_869f_d67fee7cb591);
}
impl ::core::convert::From<ITpmVirtualSmartCardManager> for ::windows::core::IUnknown {
    fn from(value: ITpmVirtualSmartCardManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITpmVirtualSmartCardManager> for ::windows::core::IUnknown {
    fn from(value: &ITpmVirtualSmartCardManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITpmVirtualSmartCardManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITpmVirtualSmartCardManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, pszfriendlyname: super::super::Foundation::PWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::windows::core::RawPtr, ppszinstanceid: *mut super::super::Foundation::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszinstanceid: super::super::Foundation::PWSTR, pstatuscallback: ::windows::core::RawPtr, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITpmVirtualSmartCardManager2(pub ::windows::core::IUnknown);
impl ITpmVirtualSmartCardManager2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCard<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param10: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param11: ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(
        &self,
        pszfriendlyname: Param0,
        badminalgid: u8,
        pbadminkey: *const u8,
        cbadminkey: u32,
        pbadminkcv: *const u8,
        cbadminkcv: u32,
        pbpuk: *const u8,
        cbpuk: u32,
        pbpin: *const u8,
        cbpin: u32,
        fgenerate: Param10,
        pstatuscallback: Param11,
        ppszinstanceid: *mut super::super::Foundation::PWSTR,
        pfneedreboot: *mut super::super::Foundation::BOOL,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::core::mem::transmute(badminalgid),
            ::core::mem::transmute(pbadminkey),
            ::core::mem::transmute(cbadminkey),
            ::core::mem::transmute(pbadminkcv),
            ::core::mem::transmute(cbadminkcv),
            ::core::mem::transmute(pbpuk),
            ::core::mem::transmute(cbpuk),
            ::core::mem::transmute(pbpin),
            ::core::mem::transmute(cbpin),
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            ::core::mem::transmute(ppszinstanceid),
            ::core::mem::transmute(pfneedreboot),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestroyVirtualSmartCard<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(&self, pszinstanceid: Param0, pstatuscallback: Param1) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszinstanceid.into_param().abi(), pstatuscallback.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCardWithPinPolicy<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param12: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param13: ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(
        &self,
        pszfriendlyname: Param0,
        badminalgid: u8,
        pbadminkey: *const u8,
        cbadminkey: u32,
        pbadminkcv: *const u8,
        cbadminkcv: u32,
        pbpuk: *const u8,
        cbpuk: u32,
        pbpin: *const u8,
        cbpin: u32,
        pbpinpolicy: *const u8,
        cbpinpolicy: u32,
        fgenerate: Param12,
        pstatuscallback: Param13,
        ppszinstanceid: *mut super::super::Foundation::PWSTR,
        pfneedreboot: *mut super::super::Foundation::BOOL,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(
            ::core::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::core::mem::transmute(badminalgid),
            ::core::mem::transmute(pbadminkey),
            ::core::mem::transmute(cbadminkey),
            ::core::mem::transmute(pbadminkcv),
            ::core::mem::transmute(cbadminkcv),
            ::core::mem::transmute(pbpuk),
            ::core::mem::transmute(cbpuk),
            ::core::mem::transmute(pbpin),
            ::core::mem::transmute(cbpin),
            ::core::mem::transmute(pbpinpolicy),
            ::core::mem::transmute(cbpinpolicy),
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            ::core::mem::transmute(ppszinstanceid),
            ::core::mem::transmute(pfneedreboot),
        )
        .ok()
    }
}
unsafe impl ::windows::core::Interface for ITpmVirtualSmartCardManager2 {
    type Vtable = ITpmVirtualSmartCardManager2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdf8a2b9_02de_47f4_bc26_aa85ab5e5267);
}
impl ::core::convert::From<ITpmVirtualSmartCardManager2> for ::windows::core::IUnknown {
    fn from(value: ITpmVirtualSmartCardManager2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITpmVirtualSmartCardManager2> for ::windows::core::IUnknown {
    fn from(value: &ITpmVirtualSmartCardManager2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITpmVirtualSmartCardManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITpmVirtualSmartCardManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManager> for &ITpmVirtualSmartCardManager2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITpmVirtualSmartCardManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, pszfriendlyname: super::super::Foundation::PWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::windows::core::RawPtr, ppszinstanceid: *mut super::super::Foundation::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszinstanceid: super::super::Foundation::PWSTR, pstatuscallback: ::windows::core::RawPtr, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        pszfriendlyname: super::super::Foundation::PWSTR,
        badminalgid: u8,
        pbadminkey: *const u8,
        cbadminkey: u32,
        pbadminkcv: *const u8,
        cbadminkcv: u32,
        pbpuk: *const u8,
        cbpuk: u32,
        pbpin: *const u8,
        cbpin: u32,
        pbpinpolicy: *const u8,
        cbpinpolicy: u32,
        fgenerate: super::super::Foundation::BOOL,
        pstatuscallback: ::windows::core::RawPtr,
        ppszinstanceid: *mut super::super::Foundation::PWSTR,
        pfneedreboot: *mut super::super::Foundation::BOOL,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITpmVirtualSmartCardManager3(pub ::windows::core::IUnknown);
impl ITpmVirtualSmartCardManager3 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCard<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param10: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param11: ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(
        &self,
        pszfriendlyname: Param0,
        badminalgid: u8,
        pbadminkey: *const u8,
        cbadminkey: u32,
        pbadminkcv: *const u8,
        cbadminkcv: u32,
        pbpuk: *const u8,
        cbpuk: u32,
        pbpin: *const u8,
        cbpin: u32,
        fgenerate: Param10,
        pstatuscallback: Param11,
        ppszinstanceid: *mut super::super::Foundation::PWSTR,
        pfneedreboot: *mut super::super::Foundation::BOOL,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(
            ::core::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::core::mem::transmute(badminalgid),
            ::core::mem::transmute(pbadminkey),
            ::core::mem::transmute(cbadminkey),
            ::core::mem::transmute(pbadminkcv),
            ::core::mem::transmute(cbadminkcv),
            ::core::mem::transmute(pbpuk),
            ::core::mem::transmute(cbpuk),
            ::core::mem::transmute(pbpin),
            ::core::mem::transmute(cbpin),
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            ::core::mem::transmute(ppszinstanceid),
            ::core::mem::transmute(pfneedreboot),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestroyVirtualSmartCard<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(&self, pszinstanceid: Param0, pstatuscallback: Param1) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszinstanceid.into_param().abi(), pstatuscallback.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCardWithPinPolicy<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param12: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param13: ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(
        &self,
        pszfriendlyname: Param0,
        badminalgid: u8,
        pbadminkey: *const u8,
        cbadminkey: u32,
        pbadminkcv: *const u8,
        cbadminkcv: u32,
        pbpuk: *const u8,
        cbpuk: u32,
        pbpin: *const u8,
        cbpin: u32,
        pbpinpolicy: *const u8,
        cbpinpolicy: u32,
        fgenerate: Param12,
        pstatuscallback: Param13,
        ppszinstanceid: *mut super::super::Foundation::PWSTR,
        pfneedreboot: *mut super::super::Foundation::BOOL,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(
            ::core::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::core::mem::transmute(badminalgid),
            ::core::mem::transmute(pbadminkey),
            ::core::mem::transmute(cbadminkey),
            ::core::mem::transmute(pbadminkcv),
            ::core::mem::transmute(cbadminkcv),
            ::core::mem::transmute(pbpuk),
            ::core::mem::transmute(cbpuk),
            ::core::mem::transmute(pbpin),
            ::core::mem::transmute(cbpin),
            ::core::mem::transmute(pbpinpolicy),
            ::core::mem::transmute(cbpinpolicy),
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            ::core::mem::transmute(ppszinstanceid),
            ::core::mem::transmute(pfneedreboot),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCardWithAttestation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param13: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param14: ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(
        &self,
        pszfriendlyname: Param0,
        badminalgid: u8,
        pbadminkey: *const u8,
        cbadminkey: u32,
        pbadminkcv: *const u8,
        cbadminkcv: u32,
        pbpuk: *const u8,
        cbpuk: u32,
        pbpin: *const u8,
        cbpin: u32,
        pbpinpolicy: *const u8,
        cbpinpolicy: u32,
        attestationtype: TPMVSC_ATTESTATION_TYPE,
        fgenerate: Param13,
        pstatuscallback: Param14,
    ) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(
            ::core::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::core::mem::transmute(badminalgid),
            ::core::mem::transmute(pbadminkey),
            ::core::mem::transmute(cbadminkey),
            ::core::mem::transmute(pbadminkcv),
            ::core::mem::transmute(cbadminkcv),
            ::core::mem::transmute(pbpuk),
            ::core::mem::transmute(cbpuk),
            ::core::mem::transmute(pbpin),
            ::core::mem::transmute(cbpin),
            ::core::mem::transmute(pbpinpolicy),
            ::core::mem::transmute(cbpinpolicy),
            ::core::mem::transmute(attestationtype),
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::core::Interface for ITpmVirtualSmartCardManager3 {
    type Vtable = ITpmVirtualSmartCardManager3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c745a97_f375_4150_be17_5950f694c699);
}
impl ::core::convert::From<ITpmVirtualSmartCardManager3> for ::windows::core::IUnknown {
    fn from(value: ITpmVirtualSmartCardManager3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITpmVirtualSmartCardManager3> for ::windows::core::IUnknown {
    fn from(value: &ITpmVirtualSmartCardManager3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITpmVirtualSmartCardManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITpmVirtualSmartCardManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManager2> for &ITpmVirtualSmartCardManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, ITpmVirtualSmartCardManager2> {
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
impl<'a> ::windows::core::IntoParam<'a, ITpmVirtualSmartCardManager> for &ITpmVirtualSmartCardManager3 {
    fn into_param(self) -> ::windows::core::Param<'a, ITpmVirtualSmartCardManager> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManager3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::core::RawPtr, pszfriendlyname: super::super::Foundation::PWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::windows::core::RawPtr, ppszinstanceid: *mut super::super::Foundation::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pszinstanceid: super::super::Foundation::PWSTR, pstatuscallback: ::windows::core::RawPtr, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        pszfriendlyname: super::super::Foundation::PWSTR,
        badminalgid: u8,
        pbadminkey: *const u8,
        cbadminkey: u32,
        pbadminkcv: *const u8,
        cbadminkcv: u32,
        pbpuk: *const u8,
        cbpuk: u32,
        pbpin: *const u8,
        cbpin: u32,
        pbpinpolicy: *const u8,
        cbpinpolicy: u32,
        fgenerate: super::super::Foundation::BOOL,
        pstatuscallback: ::windows::core::RawPtr,
        ppszinstanceid: *mut super::super::Foundation::PWSTR,
        pfneedreboot: *mut super::super::Foundation::BOOL,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::core::RawPtr,
        pszfriendlyname: super::super::Foundation::PWSTR,
        badminalgid: u8,
        pbadminkey: *const u8,
        cbadminkey: u32,
        pbadminkcv: *const u8,
        cbadminkcv: u32,
        pbpuk: *const u8,
        cbpuk: u32,
        pbpin: *const u8,
        cbpin: u32,
        pbpinpolicy: *const u8,
        cbpinpolicy: u32,
        attestationtype: TPMVSC_ATTESTATION_TYPE,
        fgenerate: super::super::Foundation::BOOL,
        pstatuscallback: ::windows::core::RawPtr,
        ppszinstanceid: *mut super::super::Foundation::PWSTR,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ITpmVirtualSmartCardManagerStatusCallback(pub ::windows::core::IUnknown);
impl ITpmVirtualSmartCardManagerStatusCallback {
    pub unsafe fn ReportProgress(&self, status: TPMVSCMGR_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(status)).ok()
    }
    pub unsafe fn ReportError(&self, error: TPMVSCMGR_ERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(error)).ok()
    }
}
unsafe impl ::windows::core::Interface for ITpmVirtualSmartCardManagerStatusCallback {
    type Vtable = ITpmVirtualSmartCardManagerStatusCallback_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a1bb35f_abb8_451c_a1ae_33d98f1bef4a);
}
impl ::core::convert::From<ITpmVirtualSmartCardManagerStatusCallback> for ::windows::core::IUnknown {
    fn from(value: ITpmVirtualSmartCardManagerStatusCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ITpmVirtualSmartCardManagerStatusCallback> for ::windows::core::IUnknown {
    fn from(value: &ITpmVirtualSmartCardManagerStatusCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITpmVirtualSmartCardManagerStatusCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITpmVirtualSmartCardManagerStatusCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManagerStatusCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, status: TPMVSCMGR_STATUS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, error: TPMVSCMGR_ERROR) -> ::windows::core::HRESULT,
);
pub const RemoteTpmVirtualSmartCardManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x152ea2a8_70dc_4c59_8b2a_32aa3ca0dcac);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TPMVSCMGR_ERROR(pub i32);
pub const TPMVSCMGR_ERROR_IMPERSONATION: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(0i32);
pub const TPMVSCMGR_ERROR_PIN_COMPLEXITY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(1i32);
pub const TPMVSCMGR_ERROR_READER_COUNT_LIMIT: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(2i32);
pub const TPMVSCMGR_ERROR_TERMINAL_SERVICES_SESSION: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(3i32);
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_INITIALIZE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(4i32);
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_CREATE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(5i32);
pub const TPMVSCMGR_ERROR_VTPMSMARTCARD_DESTROY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(6i32);
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_INITIALIZE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(7i32);
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_CREATE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(8i32);
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_DESTROY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(9i32);
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_WRITE_PROPERTY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(10i32);
pub const TPMVSCMGR_ERROR_VGIDSSIMULATOR_READ_PROPERTY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(11i32);
pub const TPMVSCMGR_ERROR_VREADER_INITIALIZE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(12i32);
pub const TPMVSCMGR_ERROR_VREADER_CREATE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(13i32);
pub const TPMVSCMGR_ERROR_VREADER_DESTROY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(14i32);
pub const TPMVSCMGR_ERROR_GENERATE_LOCATE_READER: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(15i32);
pub const TPMVSCMGR_ERROR_GENERATE_FILESYSTEM: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(16i32);
pub const TPMVSCMGR_ERROR_CARD_CREATE: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(17i32);
pub const TPMVSCMGR_ERROR_CARD_DESTROY: TPMVSCMGR_ERROR = TPMVSCMGR_ERROR(18i32);
impl ::core::convert::From<i32> for TPMVSCMGR_ERROR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TPMVSCMGR_ERROR {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TPMVSCMGR_STATUS(pub i32);
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_INITIALIZING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(0i32);
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_CREATING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(1i32);
pub const TPMVSCMGR_STATUS_VTPMSMARTCARD_DESTROYING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(2i32);
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_INITIALIZING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(3i32);
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_CREATING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(4i32);
pub const TPMVSCMGR_STATUS_VGIDSSIMULATOR_DESTROYING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(5i32);
pub const TPMVSCMGR_STATUS_VREADER_INITIALIZING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(6i32);
pub const TPMVSCMGR_STATUS_VREADER_CREATING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(7i32);
pub const TPMVSCMGR_STATUS_VREADER_DESTROYING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(8i32);
pub const TPMVSCMGR_STATUS_GENERATE_WAITING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(9i32);
pub const TPMVSCMGR_STATUS_GENERATE_AUTHENTICATING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(10i32);
pub const TPMVSCMGR_STATUS_GENERATE_RUNNING: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(11i32);
pub const TPMVSCMGR_STATUS_CARD_CREATED: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(12i32);
pub const TPMVSCMGR_STATUS_CARD_DESTROYED: TPMVSCMGR_STATUS = TPMVSCMGR_STATUS(13i32);
impl ::core::convert::From<i32> for TPMVSCMGR_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TPMVSCMGR_STATUS {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TPMVSC_ATTESTATION_TYPE(pub i32);
pub const TPMVSC_ATTESTATION_NONE: TPMVSC_ATTESTATION_TYPE = TPMVSC_ATTESTATION_TYPE(0i32);
pub const TPMVSC_ATTESTATION_AIK_ONLY: TPMVSC_ATTESTATION_TYPE = TPMVSC_ATTESTATION_TYPE(1i32);
pub const TPMVSC_ATTESTATION_AIK_AND_CERTIFICATE: TPMVSC_ATTESTATION_TYPE = TPMVSC_ATTESTATION_TYPE(2i32);
impl ::core::convert::From<i32> for TPMVSC_ATTESTATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TPMVSC_ATTESTATION_TYPE {
    type Abi = Self;
}
pub const TPMVSC_DEFAULT_ADMIN_ALGORITHM_ID: u32 = 130u32;
pub const TpmVirtualSmartCardManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16a18e86_7f6e_4c20_ad89_4ffc0db7a96a);
