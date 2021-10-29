#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITpmVirtualSmartCardManager(::windows::runtime::IUnknown);
impl ITpmVirtualSmartCardManager {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCard<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param10: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param11: ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(
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
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::std::mem::transmute(badminalgid),
            ::std::mem::transmute(pbadminkey),
            ::std::mem::transmute(cbadminkey),
            ::std::mem::transmute(pbadminkcv),
            ::std::mem::transmute(cbadminkcv),
            ::std::mem::transmute(pbpuk),
            ::std::mem::transmute(cbpuk),
            ::std::mem::transmute(pbpin),
            ::std::mem::transmute(cbpin),
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            ::std::mem::transmute(ppszinstanceid),
            ::std::mem::transmute(pfneedreboot),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestroyVirtualSmartCard<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(&self, pszinstanceid: Param0, pstatuscallback: Param1) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszinstanceid.into_param().abi(), pstatuscallback.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITpmVirtualSmartCardManager {
    type Vtable = ITpmVirtualSmartCardManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(288038399, 55772, 16887, [134, 159, 214, 127, 238, 124, 181, 145]);
}
impl ::std::convert::From<ITpmVirtualSmartCardManager> for ::windows::runtime::IUnknown {
    fn from(value: ITpmVirtualSmartCardManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITpmVirtualSmartCardManager> for ::windows::runtime::IUnknown {
    fn from(value: &ITpmVirtualSmartCardManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITpmVirtualSmartCardManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITpmVirtualSmartCardManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszfriendlyname: super::super::Foundation::PWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::windows::runtime::RawPtr, ppszinstanceid: *mut super::super::Foundation::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszinstanceid: super::super::Foundation::PWSTR, pstatuscallback: ::windows::runtime::RawPtr, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITpmVirtualSmartCardManager2(::windows::runtime::IUnknown);
impl ITpmVirtualSmartCardManager2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCard<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param10: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param11: ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(
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
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::std::mem::transmute(badminalgid),
            ::std::mem::transmute(pbadminkey),
            ::std::mem::transmute(cbadminkey),
            ::std::mem::transmute(pbadminkcv),
            ::std::mem::transmute(cbadminkcv),
            ::std::mem::transmute(pbpuk),
            ::std::mem::transmute(cbpuk),
            ::std::mem::transmute(pbpin),
            ::std::mem::transmute(cbpin),
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            ::std::mem::transmute(ppszinstanceid),
            ::std::mem::transmute(pfneedreboot),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestroyVirtualSmartCard<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(&self, pszinstanceid: Param0, pstatuscallback: Param1) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszinstanceid.into_param().abi(), pstatuscallback.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCardWithPinPolicy<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param12: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param13: ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(
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
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::std::mem::transmute(badminalgid),
            ::std::mem::transmute(pbadminkey),
            ::std::mem::transmute(cbadminkey),
            ::std::mem::transmute(pbadminkcv),
            ::std::mem::transmute(cbadminkcv),
            ::std::mem::transmute(pbpuk),
            ::std::mem::transmute(cbpuk),
            ::std::mem::transmute(pbpin),
            ::std::mem::transmute(cbpin),
            ::std::mem::transmute(pbpinpolicy),
            ::std::mem::transmute(cbpinpolicy),
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            ::std::mem::transmute(ppszinstanceid),
            ::std::mem::transmute(pfneedreboot),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITpmVirtualSmartCardManager2 {
    type Vtable = ITpmVirtualSmartCardManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4260930233, 734, 18420, [188, 38, 170, 133, 171, 94, 82, 103]);
}
impl ::std::convert::From<ITpmVirtualSmartCardManager2> for ::windows::runtime::IUnknown {
    fn from(value: ITpmVirtualSmartCardManager2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITpmVirtualSmartCardManager2> for ::windows::runtime::IUnknown {
    fn from(value: &ITpmVirtualSmartCardManager2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITpmVirtualSmartCardManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITpmVirtualSmartCardManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ITpmVirtualSmartCardManager2> for ITpmVirtualSmartCardManager {
    fn from(value: ITpmVirtualSmartCardManager2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITpmVirtualSmartCardManager2> for ITpmVirtualSmartCardManager {
    fn from(value: &ITpmVirtualSmartCardManager2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManager> for ITpmVirtualSmartCardManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITpmVirtualSmartCardManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITpmVirtualSmartCardManager>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManager> for &ITpmVirtualSmartCardManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITpmVirtualSmartCardManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITpmVirtualSmartCardManager>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszfriendlyname: super::super::Foundation::PWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::windows::runtime::RawPtr, ppszinstanceid: *mut super::super::Foundation::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszinstanceid: super::super::Foundation::PWSTR, pstatuscallback: ::windows::runtime::RawPtr, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
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
        pstatuscallback: ::windows::runtime::RawPtr,
        ppszinstanceid: *mut super::super::Foundation::PWSTR,
        pfneedreboot: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITpmVirtualSmartCardManager3(::windows::runtime::IUnknown);
impl ITpmVirtualSmartCardManager3 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCard<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param10: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param11: ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(
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
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::std::mem::transmute(badminalgid),
            ::std::mem::transmute(pbadminkey),
            ::std::mem::transmute(cbadminkey),
            ::std::mem::transmute(pbadminkcv),
            ::std::mem::transmute(cbadminkcv),
            ::std::mem::transmute(pbpuk),
            ::std::mem::transmute(cbpuk),
            ::std::mem::transmute(pbpin),
            ::std::mem::transmute(cbpin),
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            ::std::mem::transmute(ppszinstanceid),
            ::std::mem::transmute(pfneedreboot),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestroyVirtualSmartCard<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(&self, pszinstanceid: Param0, pstatuscallback: Param1) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszinstanceid.into_param().abi(), pstatuscallback.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCardWithPinPolicy<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param12: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param13: ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(
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
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::std::mem::transmute(badminalgid),
            ::std::mem::transmute(pbadminkey),
            ::std::mem::transmute(cbadminkey),
            ::std::mem::transmute(pbadminkcv),
            ::std::mem::transmute(cbadminkcv),
            ::std::mem::transmute(pbpuk),
            ::std::mem::transmute(cbpuk),
            ::std::mem::transmute(pbpin),
            ::std::mem::transmute(cbpin),
            ::std::mem::transmute(pbpinpolicy),
            ::std::mem::transmute(cbpinpolicy),
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            ::std::mem::transmute(ppszinstanceid),
            ::std::mem::transmute(pfneedreboot),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCardWithAttestation<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param13: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param14: ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManagerStatusCallback>>(
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
    ) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            pszfriendlyname.into_param().abi(),
            ::std::mem::transmute(badminalgid),
            ::std::mem::transmute(pbadminkey),
            ::std::mem::transmute(cbadminkey),
            ::std::mem::transmute(pbadminkcv),
            ::std::mem::transmute(cbadminkcv),
            ::std::mem::transmute(pbpuk),
            ::std::mem::transmute(cbpuk),
            ::std::mem::transmute(pbpin),
            ::std::mem::transmute(cbpin),
            ::std::mem::transmute(pbpinpolicy),
            ::std::mem::transmute(cbpinpolicy),
            ::std::mem::transmute(attestationtype),
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITpmVirtualSmartCardManager3 {
    type Vtable = ITpmVirtualSmartCardManager3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1014258327, 62325, 16720, [190, 23, 89, 80, 246, 148, 198, 153]);
}
impl ::std::convert::From<ITpmVirtualSmartCardManager3> for ::windows::runtime::IUnknown {
    fn from(value: ITpmVirtualSmartCardManager3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITpmVirtualSmartCardManager3> for ::windows::runtime::IUnknown {
    fn from(value: &ITpmVirtualSmartCardManager3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITpmVirtualSmartCardManager3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITpmVirtualSmartCardManager3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ITpmVirtualSmartCardManager3> for ITpmVirtualSmartCardManager2 {
    fn from(value: ITpmVirtualSmartCardManager3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITpmVirtualSmartCardManager3> for ITpmVirtualSmartCardManager2 {
    fn from(value: &ITpmVirtualSmartCardManager3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManager2> for ITpmVirtualSmartCardManager3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITpmVirtualSmartCardManager2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITpmVirtualSmartCardManager2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManager2> for &ITpmVirtualSmartCardManager3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITpmVirtualSmartCardManager2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITpmVirtualSmartCardManager2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ITpmVirtualSmartCardManager3> for ITpmVirtualSmartCardManager {
    fn from(value: ITpmVirtualSmartCardManager3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITpmVirtualSmartCardManager3> for ITpmVirtualSmartCardManager {
    fn from(value: &ITpmVirtualSmartCardManager3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManager> for ITpmVirtualSmartCardManager3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITpmVirtualSmartCardManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITpmVirtualSmartCardManager>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITpmVirtualSmartCardManager> for &ITpmVirtualSmartCardManager3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITpmVirtualSmartCardManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITpmVirtualSmartCardManager>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManager3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszfriendlyname: super::super::Foundation::PWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: ::windows::runtime::RawPtr, ppszinstanceid: *mut super::super::Foundation::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszinstanceid: super::super::Foundation::PWSTR, pstatuscallback: ::windows::runtime::RawPtr, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
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
        pstatuscallback: ::windows::runtime::RawPtr,
        ppszinstanceid: *mut super::super::Foundation::PWSTR,
        pfneedreboot: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
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
        pstatuscallback: ::windows::runtime::RawPtr,
        ppszinstanceid: *mut super::super::Foundation::PWSTR,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITpmVirtualSmartCardManagerStatusCallback(::windows::runtime::IUnknown);
impl ITpmVirtualSmartCardManagerStatusCallback {
    pub unsafe fn ReportProgress(&self, status: TPMVSCMGR_STATUS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(status)).ok()
    }
    pub unsafe fn ReportError(&self, error: TPMVSCMGR_ERROR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(error)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITpmVirtualSmartCardManagerStatusCallback {
    type Vtable = ITpmVirtualSmartCardManagerStatusCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(438023007, 43960, 17692, [161, 174, 51, 217, 143, 27, 239, 74]);
}
impl ::std::convert::From<ITpmVirtualSmartCardManagerStatusCallback> for ::windows::runtime::IUnknown {
    fn from(value: ITpmVirtualSmartCardManagerStatusCallback) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITpmVirtualSmartCardManagerStatusCallback> for ::windows::runtime::IUnknown {
    fn from(value: &ITpmVirtualSmartCardManagerStatusCallback) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITpmVirtualSmartCardManagerStatusCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITpmVirtualSmartCardManagerStatusCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManagerStatusCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: TPMVSCMGR_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, error: TPMVSCMGR_ERROR) -> ::windows::runtime::HRESULT,
);
pub const RemoteTpmVirtualSmartCardManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(355377832, 28892, 19545, [139, 42, 50, 170, 60, 160, 220, 172]);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for TPMVSCMGR_ERROR {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TPMVSCMGR_ERROR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for TPMVSCMGR_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TPMVSCMGR_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TPMVSC_ATTESTATION_TYPE(pub i32);
pub const TPMVSC_ATTESTATION_NONE: TPMVSC_ATTESTATION_TYPE = TPMVSC_ATTESTATION_TYPE(0i32);
pub const TPMVSC_ATTESTATION_AIK_ONLY: TPMVSC_ATTESTATION_TYPE = TPMVSC_ATTESTATION_TYPE(1i32);
pub const TPMVSC_ATTESTATION_AIK_AND_CERTIFICATE: TPMVSC_ATTESTATION_TYPE = TPMVSC_ATTESTATION_TYPE(2i32);
impl ::std::convert::From<i32> for TPMVSC_ATTESTATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TPMVSC_ATTESTATION_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const TPMVSC_DEFAULT_ADMIN_ALGORITHM_ID: u32 = 130u32;
pub const TpmVirtualSmartCardManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(379686534, 32622, 19488, [173, 137, 79, 252, 13, 183, 169, 106]);
