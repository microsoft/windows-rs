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
impl ITpmVirtualSmartCardManager2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCard<P0, P1, P2>(&self, pszfriendlyname: P0, badminalgid: u8, pbadminkey: &[u8], pbadminkcv: &[u8], pbpuk: &[u8], pbpin: &[u8], fgenerate: P1, pstatuscallback: P2, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<ITpmVirtualSmartCardManagerStatusCallback>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateVirtualSmartCard)(::windows::core::Vtable::as_raw(self), pszfriendlyname.into().abi(), badminalgid, ::core::mem::transmute(pbadminkey.as_ptr()), pbadminkey.len() as _, ::core::mem::transmute(pbadminkcv.as_ptr()), pbadminkcv.len() as _, ::core::mem::transmute(pbpuk.as_ptr()), pbpuk.len() as _, ::core::mem::transmute(pbpin.as_ptr()), pbpin.len() as _, fgenerate.into(), pstatuscallback.into().abi(), ppszinstanceid, pfneedreboot).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestroyVirtualSmartCard<P0, P1>(&self, pszinstanceid: P0, pstatuscallback: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<ITpmVirtualSmartCardManagerStatusCallback>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DestroyVirtualSmartCard)(::windows::core::Vtable::as_raw(self), pszinstanceid.into().abi(), pstatuscallback.into().abi(), result__.as_mut_ptr()).from_abi(result__)
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
impl ITpmVirtualSmartCardManager3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCard<P0, P1, P2>(&self, pszfriendlyname: P0, badminalgid: u8, pbadminkey: &[u8], pbadminkcv: &[u8], pbpuk: &[u8], pbpin: &[u8], fgenerate: P1, pstatuscallback: P2, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<ITpmVirtualSmartCardManagerStatusCallback>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateVirtualSmartCard)(::windows::core::Vtable::as_raw(self), pszfriendlyname.into().abi(), badminalgid, ::core::mem::transmute(pbadminkey.as_ptr()), pbadminkey.len() as _, ::core::mem::transmute(pbadminkcv.as_ptr()), pbadminkcv.len() as _, ::core::mem::transmute(pbpuk.as_ptr()), pbpuk.len() as _, ::core::mem::transmute(pbpin.as_ptr()), pbpin.len() as _, fgenerate.into(), pstatuscallback.into().abi(), ppszinstanceid, pfneedreboot).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestroyVirtualSmartCard<P0, P1>(&self, pszinstanceid: P0, pstatuscallback: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<ITpmVirtualSmartCardManagerStatusCallback>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.DestroyVirtualSmartCard)(::windows::core::Vtable::as_raw(self), pszinstanceid.into().abi(), pstatuscallback.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCardWithPinPolicy<P0, P1, P2>(&self, pszfriendlyname: P0, badminalgid: u8, pbadminkey: &[u8], pbadminkcv: &[u8], pbpuk: &[u8], pbpin: &[u8], pbpinpolicy: &[u8], fgenerate: P1, pstatuscallback: P2, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<ITpmVirtualSmartCardManagerStatusCallback>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateVirtualSmartCardWithPinPolicy)(::windows::core::Vtable::as_raw(self), pszfriendlyname.into().abi(), badminalgid, ::core::mem::transmute(pbadminkey.as_ptr()), pbadminkey.len() as _, ::core::mem::transmute(pbadminkcv.as_ptr()), pbadminkcv.len() as _, ::core::mem::transmute(pbpuk.as_ptr()), pbpuk.len() as _, ::core::mem::transmute(pbpin.as_ptr()), pbpin.len() as _, ::core::mem::transmute(pbpinpolicy.as_ptr()), pbpinpolicy.len() as _, fgenerate.into(), pstatuscallback.into().abi(), ppszinstanceid, pfneedreboot).ok()
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
impl ::core::default::Default for TPMVSCMGR_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TPMVSCMGR_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TPMVSCMGR_ERROR").field(&self.0).finish()
    }
}
impl ::core::default::Default for TPMVSCMGR_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TPMVSCMGR_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TPMVSCMGR_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for TPMVSC_ATTESTATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TPMVSC_ATTESTATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TPMVSC_ATTESTATION_TYPE").field(&self.0).finish()
    }
}
