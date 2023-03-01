#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
#[repr(transparent)]
pub struct ITpmVirtualSmartCardManager(::windows::core::IUnknown);
impl ITpmVirtualSmartCardManager {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCard<P0, P1, P2>(&self, pszfriendlyname: P0, badminalgid: u8, pbadminkey: &[u8], pbadminkcv: &[u8], pbpuk: &[u8], pbpin: &[u8], fgenerate: P1, pstatuscallback: P2, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows::core::IntoParam<ITpmVirtualSmartCardManagerStatusCallback>,
    {
        (::windows::core::Interface::vtable(self).CreateVirtualSmartCard)(::windows::core::Interface::as_raw(self), pszfriendlyname.into_param().abi(), badminalgid, ::core::mem::transmute(pbadminkey.as_ptr()), pbadminkey.len() as _, ::core::mem::transmute(pbadminkcv.as_ptr()), pbadminkcv.len() as _, ::core::mem::transmute(pbpuk.as_ptr()), pbpuk.len() as _, ::core::mem::transmute(pbpin.as_ptr()), pbpin.len() as _, fgenerate.into_param().abi(), pstatuscallback.into_param().abi(), ppszinstanceid, pfneedreboot).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestroyVirtualSmartCard<P0, P1>(&self, pszinstanceid: P0, pstatuscallback: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<ITpmVirtualSmartCardManagerStatusCallback>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).DestroyVirtualSmartCard)(::windows::core::Interface::as_raw(self), pszinstanceid.into_param().abi(), pstatuscallback.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITpmVirtualSmartCardManager, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for ITpmVirtualSmartCardManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITpmVirtualSmartCardManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x112b1dff_d9dc_41f7_869f_d67fee7cb591);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateVirtualSmartCard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfriendlyname: ::windows::core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: *mut ::core::ffi::c_void, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateVirtualSmartCard: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DestroyVirtualSmartCard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszinstanceid: ::windows::core::PCWSTR, pstatuscallback: *mut ::core::ffi::c_void, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DestroyVirtualSmartCard: usize,
}
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
#[repr(transparent)]
pub struct ITpmVirtualSmartCardManager2(::windows::core::IUnknown);
impl ITpmVirtualSmartCardManager2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCard<P0, P1, P2>(&self, pszfriendlyname: P0, badminalgid: u8, pbadminkey: &[u8], pbadminkcv: &[u8], pbpuk: &[u8], pbpin: &[u8], fgenerate: P1, pstatuscallback: P2, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows::core::IntoParam<ITpmVirtualSmartCardManagerStatusCallback>,
    {
        (::windows::core::Interface::vtable(self).base__.CreateVirtualSmartCard)(::windows::core::Interface::as_raw(self), pszfriendlyname.into_param().abi(), badminalgid, ::core::mem::transmute(pbadminkey.as_ptr()), pbadminkey.len() as _, ::core::mem::transmute(pbadminkcv.as_ptr()), pbadminkcv.len() as _, ::core::mem::transmute(pbpuk.as_ptr()), pbpuk.len() as _, ::core::mem::transmute(pbpin.as_ptr()), pbpin.len() as _, fgenerate.into_param().abi(), pstatuscallback.into_param().abi(), ppszinstanceid, pfneedreboot).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestroyVirtualSmartCard<P0, P1>(&self, pszinstanceid: P0, pstatuscallback: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<ITpmVirtualSmartCardManagerStatusCallback>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.DestroyVirtualSmartCard)(::windows::core::Interface::as_raw(self), pszinstanceid.into_param().abi(), pstatuscallback.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCardWithPinPolicy<P0, P1, P2>(&self, pszfriendlyname: P0, badminalgid: u8, pbadminkey: &[u8], pbadminkcv: &[u8], pbpuk: &[u8], pbpin: &[u8], pbpinpolicy: &[u8], fgenerate: P1, pstatuscallback: P2, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows::core::IntoParam<ITpmVirtualSmartCardManagerStatusCallback>,
    {
        (::windows::core::Interface::vtable(self).CreateVirtualSmartCardWithPinPolicy)(
            ::windows::core::Interface::as_raw(self),
            pszfriendlyname.into_param().abi(),
            badminalgid,
            ::core::mem::transmute(pbadminkey.as_ptr()),
            pbadminkey.len() as _,
            ::core::mem::transmute(pbadminkcv.as_ptr()),
            pbadminkcv.len() as _,
            ::core::mem::transmute(pbpuk.as_ptr()),
            pbpuk.len() as _,
            ::core::mem::transmute(pbpin.as_ptr()),
            pbpin.len() as _,
            ::core::mem::transmute(pbpinpolicy.as_ptr()),
            pbpinpolicy.len() as _,
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            ppszinstanceid,
            pfneedreboot,
        )
        .ok()
    }
}
::windows::imp::interface_hierarchy!(ITpmVirtualSmartCardManager2, ::windows::core::IUnknown, ITpmVirtualSmartCardManager);
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
}
impl ::core::clone::Clone for ITpmVirtualSmartCardManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITpmVirtualSmartCardManager2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdf8a2b9_02de_47f4_bc26_aa85ab5e5267);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManager2_Vtbl {
    pub base__: ITpmVirtualSmartCardManager_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateVirtualSmartCardWithPinPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfriendlyname: ::windows::core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, fgenerate: super::super::Foundation::BOOL, pstatuscallback: *mut ::core::ffi::c_void, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateVirtualSmartCardWithPinPolicy: usize,
}
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
#[repr(transparent)]
pub struct ITpmVirtualSmartCardManager3(::windows::core::IUnknown);
impl ITpmVirtualSmartCardManager3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCard<P0, P1, P2>(&self, pszfriendlyname: P0, badminalgid: u8, pbadminkey: &[u8], pbadminkcv: &[u8], pbpuk: &[u8], pbpin: &[u8], fgenerate: P1, pstatuscallback: P2, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows::core::IntoParam<ITpmVirtualSmartCardManagerStatusCallback>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.CreateVirtualSmartCard)(::windows::core::Interface::as_raw(self), pszfriendlyname.into_param().abi(), badminalgid, ::core::mem::transmute(pbadminkey.as_ptr()), pbadminkey.len() as _, ::core::mem::transmute(pbadminkcv.as_ptr()), pbadminkcv.len() as _, ::core::mem::transmute(pbpuk.as_ptr()), pbpuk.len() as _, ::core::mem::transmute(pbpin.as_ptr()), pbpin.len() as _, fgenerate.into_param().abi(), pstatuscallback.into_param().abi(), ppszinstanceid, pfneedreboot).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DestroyVirtualSmartCard<P0, P1>(&self, pszinstanceid: P0, pstatuscallback: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<ITpmVirtualSmartCardManagerStatusCallback>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.DestroyVirtualSmartCard)(::windows::core::Interface::as_raw(self), pszinstanceid.into_param().abi(), pstatuscallback.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCardWithPinPolicy<P0, P1, P2>(&self, pszfriendlyname: P0, badminalgid: u8, pbadminkey: &[u8], pbadminkcv: &[u8], pbpuk: &[u8], pbpin: &[u8], pbpinpolicy: &[u8], fgenerate: P1, pstatuscallback: P2, ppszinstanceid: *mut ::windows::core::PWSTR, pfneedreboot: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows::core::IntoParam<ITpmVirtualSmartCardManagerStatusCallback>,
    {
        (::windows::core::Interface::vtable(self).base__.CreateVirtualSmartCardWithPinPolicy)(
            ::windows::core::Interface::as_raw(self),
            pszfriendlyname.into_param().abi(),
            badminalgid,
            ::core::mem::transmute(pbadminkey.as_ptr()),
            pbadminkey.len() as _,
            ::core::mem::transmute(pbadminkcv.as_ptr()),
            pbadminkcv.len() as _,
            ::core::mem::transmute(pbpuk.as_ptr()),
            pbpuk.len() as _,
            ::core::mem::transmute(pbpin.as_ptr()),
            pbpin.len() as _,
            ::core::mem::transmute(pbpinpolicy.as_ptr()),
            pbpinpolicy.len() as _,
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            ppszinstanceid,
            pfneedreboot,
        )
        .ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVirtualSmartCardWithAttestation<P0, P1, P2>(&self, pszfriendlyname: P0, badminalgid: u8, pbadminkey: &[u8], pbadminkcv: &[u8], pbpuk: &[u8], pbpin: &[u8], pbpinpolicy: &[u8], attestationtype: TPMVSC_ATTESTATION_TYPE, fgenerate: P1, pstatuscallback: P2) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows::core::IntoParam<ITpmVirtualSmartCardManagerStatusCallback>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).CreateVirtualSmartCardWithAttestation)(
            ::windows::core::Interface::as_raw(self),
            pszfriendlyname.into_param().abi(),
            badminalgid,
            ::core::mem::transmute(pbadminkey.as_ptr()),
            pbadminkey.len() as _,
            ::core::mem::transmute(pbadminkcv.as_ptr()),
            pbadminkcv.len() as _,
            ::core::mem::transmute(pbpuk.as_ptr()),
            pbpuk.len() as _,
            ::core::mem::transmute(pbpin.as_ptr()),
            pbpin.len() as _,
            ::core::mem::transmute(pbpinpolicy.as_ptr()),
            pbpinpolicy.len() as _,
            attestationtype,
            fgenerate.into_param().abi(),
            pstatuscallback.into_param().abi(),
            &mut result__,
        )
        .from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITpmVirtualSmartCardManager3, ::windows::core::IUnknown, ITpmVirtualSmartCardManager, ITpmVirtualSmartCardManager2);
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
}
impl ::core::clone::Clone for ITpmVirtualSmartCardManager3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITpmVirtualSmartCardManager3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c745a97_f375_4150_be17_5950f694c699);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManager3_Vtbl {
    pub base__: ITpmVirtualSmartCardManager2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateVirtualSmartCardWithAttestation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfriendlyname: ::windows::core::PCWSTR, badminalgid: u8, pbadminkey: *const u8, cbadminkey: u32, pbadminkcv: *const u8, cbadminkcv: u32, pbpuk: *const u8, cbpuk: u32, pbpin: *const u8, cbpin: u32, pbpinpolicy: *const u8, cbpinpolicy: u32, attestationtype: TPMVSC_ATTESTATION_TYPE, fgenerate: super::super::Foundation::BOOL, pstatuscallback: *mut ::core::ffi::c_void, ppszinstanceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateVirtualSmartCardWithAttestation: usize,
}
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
#[repr(transparent)]
pub struct ITpmVirtualSmartCardManagerStatusCallback(::windows::core::IUnknown);
impl ITpmVirtualSmartCardManagerStatusCallback {
    pub unsafe fn ReportProgress(&self, status: TPMVSCMGR_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReportProgress)(::windows::core::Interface::as_raw(self), status).ok()
    }
    pub unsafe fn ReportError(&self, error: TPMVSCMGR_ERROR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReportError)(::windows::core::Interface::as_raw(self), error).ok()
    }
}
::windows::imp::interface_hierarchy!(ITpmVirtualSmartCardManagerStatusCallback, ::windows::core::IUnknown);
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
}
impl ::core::clone::Clone for ITpmVirtualSmartCardManagerStatusCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITpmVirtualSmartCardManagerStatusCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a1bb35f_abb8_451c_a1ae_33d98f1bef4a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITpmVirtualSmartCardManagerStatusCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ReportProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: TPMVSCMGR_STATUS) -> ::windows::core::HRESULT,
    pub ReportError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, error: TPMVSCMGR_ERROR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const RemoteTpmVirtualSmartCardManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x152ea2a8_70dc_4c59_8b2a_32aa3ca0dcac);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TPMVSC_DEFAULT_ADMIN_ALGORITHM_ID: u32 = 130u32;
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
pub const TpmVirtualSmartCardManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16a18e86_7f6e_4c20_ad89_4ffc0db7a96a);
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for TPMVSCMGR_ERROR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TPMVSCMGR_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TPMVSCMGR_ERROR").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for TPMVSCMGR_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TPMVSCMGR_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TPMVSCMGR_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Tpm\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for TPMVSC_ATTESTATION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TPMVSC_ATTESTATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TPMVSC_ATTESTATION_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
