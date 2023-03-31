#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DoMsCtfMonitor<P0>(dwflags: u32, heventforservicestop: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "msctfmonitor.dll""system" fn DoMsCtfMonitor ( dwflags : u32 , heventforservicestop : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    DoMsCtfMonitor(dwflags, heventforservicestop.into_param().abi())
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[inline]
pub unsafe fn InitLocalMsCtfMonitor(dwflags: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "msctfmonitor.dll""system" fn InitLocalMsCtfMonitor ( dwflags : u32 ) -> ::windows::core::HRESULT );
    InitLocalMsCtfMonitor(dwflags).ok()
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[inline]
pub unsafe fn UninitLocalMsCtfMonitor() -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "msctfmonitor.dll""system" fn UninitLocalMsCtfMonitor ( ) -> ::windows::core::HRESULT );
    UninitLocalMsCtfMonitor().ok()
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IAccClientDocMgr(::windows::core::IUnknown);
impl IAccClientDocMgr {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDocuments(&self) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IEnumUnknown>();
        (::windows::core::Interface::vtable(self).GetDocuments)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LookupByHWND<P0>(&self, hwnd: P0, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).LookupByHWND)(::windows::core::Interface::as_raw(self), hwnd.into_param().abi(), riid, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LookupByPoint(&self, pt: super::super::Foundation::POINT, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).LookupByPoint)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pt), riid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFocused(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetFocused)(::windows::core::Interface::as_raw(self), riid, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IAccClientDocMgr, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAccClientDocMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccClientDocMgr {}
impl ::core::fmt::Debug for IAccClientDocMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccClientDocMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAccClientDocMgr {
    type Vtable = IAccClientDocMgr_Vtbl;
}
impl ::core::clone::Clone for IAccClientDocMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccClientDocMgr {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c896039_7b6d_49e6_a8c1_45116a98292b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccClientDocMgr_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDocuments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDocuments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LookupByHWND: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LookupByHWND: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LookupByPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LookupByPoint: usize,
    pub GetFocused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IAccDictionary(::windows::core::IUnknown);
impl IAccDictionary {
    pub unsafe fn GetLocalizedString(&self, term: *const ::windows::core::GUID, lcid: u32, presult: *mut ::windows::core::BSTR, plcid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLocalizedString)(::windows::core::Interface::as_raw(self), term, lcid, ::core::mem::transmute(presult), plcid).ok()
    }
    pub unsafe fn GetParentTerm(&self, term: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetParentTerm)(::windows::core::Interface::as_raw(self), term, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMnemonicString(&self, term: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetMnemonicString)(::windows::core::Interface::as_raw(self), term, &mut result__).from_abi(result__)
    }
    pub unsafe fn LookupMnemonicTerm<P0>(&self, bstrmnemonic: P0) -> ::windows::core::Result<::windows::core::GUID>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).LookupMnemonicTerm)(::windows::core::Interface::as_raw(self), bstrmnemonic.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ConvertValueToString(&self, term: *const ::windows::core::GUID, lcid: u32, varvalue: super::super::System::Com::VARIANT, pbstrresult: *mut ::windows::core::BSTR, plcid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ConvertValueToString)(::windows::core::Interface::as_raw(self), term, lcid, ::core::mem::transmute(varvalue), ::core::mem::transmute(pbstrresult), plcid).ok()
    }
}
::windows::imp::interface_hierarchy!(IAccDictionary, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAccDictionary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccDictionary {}
impl ::core::fmt::Debug for IAccDictionary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccDictionary").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAccDictionary {
    type Vtable = IAccDictionary_Vtbl;
}
impl ::core::clone::Clone for IAccDictionary {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccDictionary {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dc4cb5f_d737_474d_ade9_5ccfc9bc1cc9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccDictionary_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetLocalizedString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, term: *const ::windows::core::GUID, lcid: u32, presult: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, plcid: *mut u32) -> ::windows::core::HRESULT,
    pub GetParentTerm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, term: *const ::windows::core::GUID, pparentterm: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetMnemonicString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, term: *const ::windows::core::GUID, presult: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub LookupMnemonicTerm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmnemonic: ::std::mem::MaybeUninit<::windows::core::BSTR>, pterm: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ConvertValueToString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, term: *const ::windows::core::GUID, lcid: u32, varvalue: super::super::System::Com::VARIANT, pbstrresult: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, plcid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ConvertValueToString: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IAccServerDocMgr(::windows::core::IUnknown);
impl IAccServerDocMgr {
    pub unsafe fn NewDocument<P0>(&self, riid: *const ::windows::core::GUID, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).NewDocument)(::windows::core::Interface::as_raw(self), riid, punk.into_param().abi()).ok()
    }
    pub unsafe fn RevokeDocument<P0>(&self, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).RevokeDocument)(::windows::core::Interface::as_raw(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn OnDocumentFocus<P0>(&self, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).OnDocumentFocus)(::windows::core::Interface::as_raw(self), punk.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IAccServerDocMgr, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAccServerDocMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccServerDocMgr {}
impl ::core::fmt::Debug for IAccServerDocMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccServerDocMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAccServerDocMgr {
    type Vtable = IAccServerDocMgr_Vtbl;
}
impl ::core::clone::Clone for IAccServerDocMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccServerDocMgr {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad7c73cf_6dd5_4855_abc2_b04bad5b9153);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccServerDocMgr_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub NewDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RevokeDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnDocumentFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IAccStore(::windows::core::IUnknown);
impl IAccStore {
    pub unsafe fn Register<P0>(&self, riid: *const ::windows::core::GUID, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).Register)(::windows::core::Interface::as_raw(self), riid, punk.into_param().abi()).ok()
    }
    pub unsafe fn Unregister<P0>(&self, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).Unregister)(::windows::core::Interface::as_raw(self), punk.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDocuments(&self) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IEnumUnknown>();
        (::windows::core::Interface::vtable(self).GetDocuments)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LookupByHWND<P0>(&self, hwnd: P0, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).LookupByHWND)(::windows::core::Interface::as_raw(self), hwnd.into_param().abi(), riid, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LookupByPoint(&self, pt: super::super::Foundation::POINT, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).LookupByPoint)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pt), riid, &mut result__).from_abi(result__)
    }
    pub unsafe fn OnDocumentFocus<P0>(&self, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).OnDocumentFocus)(::windows::core::Interface::as_raw(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn GetFocused(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetFocused)(::windows::core::Interface::as_raw(self), riid, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IAccStore, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAccStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccStore {}
impl ::core::fmt::Debug for IAccStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAccStore {
    type Vtable = IAccStore_Vtbl;
}
impl ::core::clone::Clone for IAccStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAccStore {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2cd4a63_2b72_4d48_b739_95e4765195ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccStore_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unregister: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDocuments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDocuments: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LookupByHWND: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LookupByHWND: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub LookupByPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LookupByPoint: usize,
    pub OnDocumentFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFocused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IAnchor(::windows::core::IUnknown);
impl IAnchor {
    pub unsafe fn SetGravity(&self, gravity: TsGravity) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGravity)(::windows::core::Interface::as_raw(self), gravity).ok()
    }
    pub unsafe fn GetGravity(&self) -> ::windows::core::Result<TsGravity> {
        let mut result__ = ::windows::core::zeroed::<TsGravity>();
        (::windows::core::Interface::vtable(self).GetGravity)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEqual<P0>(&self, pawith: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<IAnchor>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsEqual)(::windows::core::Interface::as_raw(self), pawith.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Compare<P0>(&self, pawith: P0) -> ::windows::core::Result<i32>
    where
        P0: ::windows::core::IntoParam<IAnchor>,
    {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Compare)(::windows::core::Interface::as_raw(self), pawith.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Shift<P0>(&self, dwflags: u32, cchreq: i32, pcch: *mut i32, pahaltanchor: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAnchor>,
    {
        (::windows::core::Interface::vtable(self).Shift)(::windows::core::Interface::as_raw(self), dwflags, cchreq, pcch, pahaltanchor.into_param().abi()).ok()
    }
    pub unsafe fn ShiftTo<P0>(&self, pasite: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAnchor>,
    {
        (::windows::core::Interface::vtable(self).ShiftTo)(::windows::core::Interface::as_raw(self), pasite.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShiftRegion(&self, dwflags: u32, dir: TsShiftDir) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).ShiftRegion)(::windows::core::Interface::as_raw(self), dwflags, dir, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetChangeHistoryMask(&self, dwmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetChangeHistoryMask)(::windows::core::Interface::as_raw(self), dwmask).ok()
    }
    pub unsafe fn GetChangeHistory(&self) -> ::windows::core::Result<ANCHOR_CHANGE_HISTORY_FLAGS> {
        let mut result__ = ::windows::core::zeroed::<ANCHOR_CHANGE_HISTORY_FLAGS>();
        (::windows::core::Interface::vtable(self).GetChangeHistory)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ClearChangeHistory(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearChangeHistory)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IAnchor> {
        let mut result__ = ::windows::core::zeroed::<IAnchor>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IAnchor, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IAnchor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAnchor {}
impl ::core::fmt::Debug for IAnchor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAnchor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAnchor {
    type Vtable = IAnchor_Vtbl;
}
impl ::core::clone::Clone for IAnchor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAnchor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0feb7e34_5a60_4356_8ef7_abdec2ff7cf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnchor_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetGravity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gravity: TsGravity) -> ::windows::core::HRESULT,
    pub GetGravity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgravity: *mut TsGravity) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pawith: *mut ::core::ffi::c_void, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEqual: usize,
    pub Compare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pawith: *mut ::core::ffi::c_void, plresult: *mut i32) -> ::windows::core::HRESULT,
    pub Shift: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, cchreq: i32, pcch: *mut i32, pahaltanchor: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShiftTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pasite: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ShiftRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, dir: TsShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShiftRegion: usize,
    pub SetChangeHistoryMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT,
    pub GetChangeHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwhistory: *mut ANCHOR_CHANGE_HISTORY_FLAGS) -> ::windows::core::HRESULT,
    pub ClearChangeHistory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaclone: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IClonableWrapper(::windows::core::IUnknown);
impl IClonableWrapper {
    pub unsafe fn CloneNewWrapper<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).CloneNewWrapper)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IClonableWrapper, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IClonableWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClonableWrapper {}
impl ::core::fmt::Debug for IClonableWrapper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClonableWrapper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IClonableWrapper {
    type Vtable = IClonableWrapper_Vtbl;
}
impl ::core::clone::Clone for IClonableWrapper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IClonableWrapper {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb33e75ff_e84c_4dca_a25c_33b8dc003374);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClonableWrapper_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CloneNewWrapper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ICoCreateLocally(::windows::core::IUnknown);
impl ICoCreateLocally {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CoCreateLocally<P0>(&self, rclsid: *const ::windows::core::GUID, dwclscontext: u32, riid: *const ::windows::core::GUID, punk: *mut ::core::option::Option<::windows::core::IUnknown>, riidparam: *const ::windows::core::GUID, punkparam: P0, varparam: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).CoCreateLocally)(::windows::core::Interface::as_raw(self), rclsid, dwclscontext, riid, ::core::mem::transmute(punk), riidparam, punkparam.into_param().abi(), ::core::mem::transmute(varparam)).ok()
    }
}
::windows::imp::interface_hierarchy!(ICoCreateLocally, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICoCreateLocally {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoCreateLocally {}
impl ::core::fmt::Debug for ICoCreateLocally {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoCreateLocally").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICoCreateLocally {
    type Vtable = ICoCreateLocally_Vtbl;
}
impl ::core::clone::Clone for ICoCreateLocally {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoCreateLocally {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03de00aa_f272_41e3_99cb_03c5e8114ea0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoCreateLocally_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CoCreateLocally: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, dwclscontext: u32, riid: *const ::windows::core::GUID, punk: *mut *mut ::core::ffi::c_void, riidparam: *const ::windows::core::GUID, punkparam: *mut ::core::ffi::c_void, varparam: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CoCreateLocally: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ICoCreatedLocally(::windows::core::IUnknown);
impl ICoCreatedLocally {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn LocalInit<P0, P1>(&self, punklocalobject: P0, riidparam: *const ::windows::core::GUID, punkparam: P1, varparam: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).LocalInit)(::windows::core::Interface::as_raw(self), punklocalobject.into_param().abi(), riidparam, punkparam.into_param().abi(), ::core::mem::transmute(varparam)).ok()
    }
}
::windows::imp::interface_hierarchy!(ICoCreatedLocally, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ICoCreatedLocally {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoCreatedLocally {}
impl ::core::fmt::Debug for ICoCreatedLocally {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoCreatedLocally").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICoCreatedLocally {
    type Vtable = ICoCreatedLocally_Vtbl;
}
impl ::core::clone::Clone for ICoCreatedLocally {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICoCreatedLocally {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a53eb6c_1908_4742_8cff_2cee2e93f94c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoCreatedLocally_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub LocalInit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punklocalobject: *mut ::core::ffi::c_void, riidparam: *const ::windows::core::GUID, punkparam: *mut ::core::ffi::c_void, varparam: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    LocalInit: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IDocWrap(::windows::core::IUnknown);
impl IDocWrap {
    pub unsafe fn SetDoc<P0>(&self, riid: *const ::windows::core::GUID, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).SetDoc)(::windows::core::Interface::as_raw(self), riid, punk.into_param().abi()).ok()
    }
    pub unsafe fn GetWrappedDoc(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetWrappedDoc)(::windows::core::Interface::as_raw(self), riid, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDocWrap, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDocWrap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDocWrap {}
impl ::core::fmt::Debug for IDocWrap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDocWrap").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDocWrap {
    type Vtable = IDocWrap_Vtbl;
}
impl ::core::clone::Clone for IDocWrap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDocWrap {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcd285fe_0be0_43bd_99c9_aaaec513c555);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDocWrap_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetDoc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetWrappedDoc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IEnumITfCompositionView(::windows::core::IUnknown);
impl IEnumITfCompositionView {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumITfCompositionView> {
        let mut result__ = ::windows::core::zeroed::<IEnumITfCompositionView>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Next(&self, rgcompositionview: &mut [::core::option::Option<ITfCompositionView>], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgcompositionview.len() as _, ::core::mem::transmute(rgcompositionview.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), ulcount).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumITfCompositionView, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumITfCompositionView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumITfCompositionView {}
impl ::core::fmt::Debug for IEnumITfCompositionView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumITfCompositionView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumITfCompositionView {
    type Vtable = IEnumITfCompositionView_Vtbl;
}
impl ::core::clone::Clone for IEnumITfCompositionView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumITfCompositionView {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5efd22ba_7838_46cb_88e2_cadb14124f8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumITfCompositionView_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, rgcompositionview: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IEnumSpeechCommands(::windows::core::IUnknown);
impl IEnumSpeechCommands {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumSpeechCommands> {
        let mut result__ = ::windows::core::zeroed::<IEnumSpeechCommands>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Next(&self, pspcmds: &mut [*mut u16], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), pspcmds.len() as _, ::core::mem::transmute(pspcmds.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), ulcount).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumSpeechCommands, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumSpeechCommands {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSpeechCommands {}
impl ::core::fmt::Debug for IEnumSpeechCommands {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSpeechCommands").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumSpeechCommands {
    type Vtable = IEnumSpeechCommands_Vtbl;
}
impl ::core::clone::Clone for IEnumSpeechCommands {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumSpeechCommands {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c5dac4f_083c_4b85_a4c9_71746048adca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSpeechCommands_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, pspcmds: *mut *mut u16, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IEnumTfCandidates(::windows::core::IUnknown);
impl IEnumTfCandidates {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfCandidates> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfCandidates>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Next(&self, ppcand: &mut [::core::option::Option<ITfCandidateString>], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ppcand.len() as _, ::core::mem::transmute(ppcand.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), ulcount).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumTfCandidates, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumTfCandidates {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfCandidates {}
impl ::core::fmt::Debug for IEnumTfCandidates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfCandidates").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumTfCandidates {
    type Vtable = IEnumTfCandidates_Vtbl;
}
impl ::core::clone::Clone for IEnumTfCandidates {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumTfCandidates {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdefb1926_6c80_4ce8_87d4_d6b72b812bde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfCandidates_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, ppcand: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IEnumTfContextViews(::windows::core::IUnknown);
impl IEnumTfContextViews {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfContextViews> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfContextViews>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Next(&self, rgviews: &mut [::core::option::Option<ITfContextView>], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgviews.len() as _, ::core::mem::transmute(rgviews.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), ulcount).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumTfContextViews, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumTfContextViews {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfContextViews {}
impl ::core::fmt::Debug for IEnumTfContextViews {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfContextViews").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumTfContextViews {
    type Vtable = IEnumTfContextViews_Vtbl;
}
impl ::core::clone::Clone for IEnumTfContextViews {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumTfContextViews {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0c0f8dd_cf38_44e1_bb0f_68cf0d551c78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfContextViews_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, rgviews: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IEnumTfContexts(::windows::core::IUnknown);
impl IEnumTfContexts {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfContexts> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfContexts>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Next(&self, rgcontext: &mut [::core::option::Option<ITfContext>], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgcontext.len() as _, ::core::mem::transmute(rgcontext.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), ulcount).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumTfContexts, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumTfContexts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfContexts {}
impl ::core::fmt::Debug for IEnumTfContexts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfContexts").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumTfContexts {
    type Vtable = IEnumTfContexts_Vtbl;
}
impl ::core::clone::Clone for IEnumTfContexts {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumTfContexts {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f1a7ea6_1654_4502_a86e_b2902344d507);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfContexts_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, rgcontext: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IEnumTfDisplayAttributeInfo(::windows::core::IUnknown);
impl IEnumTfDisplayAttributeInfo {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfDisplayAttributeInfo> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfDisplayAttributeInfo>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Next(&self, rginfo: &mut [::core::option::Option<ITfDisplayAttributeInfo>], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rginfo.len() as _, ::core::mem::transmute(rginfo.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), ulcount).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumTfDisplayAttributeInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumTfDisplayAttributeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfDisplayAttributeInfo {}
impl ::core::fmt::Debug for IEnumTfDisplayAttributeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfDisplayAttributeInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumTfDisplayAttributeInfo {
    type Vtable = IEnumTfDisplayAttributeInfo_Vtbl;
}
impl ::core::clone::Clone for IEnumTfDisplayAttributeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumTfDisplayAttributeInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cef04d7_cb75_4e80_a7ab_5f5bc7d332de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfDisplayAttributeInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, rginfo: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IEnumTfDocumentMgrs(::windows::core::IUnknown);
impl IEnumTfDocumentMgrs {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfDocumentMgrs> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfDocumentMgrs>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Next(&self, rgdocumentmgr: &mut [::core::option::Option<ITfDocumentMgr>], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgdocumentmgr.len() as _, ::core::mem::transmute(rgdocumentmgr.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), ulcount).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumTfDocumentMgrs, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumTfDocumentMgrs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfDocumentMgrs {}
impl ::core::fmt::Debug for IEnumTfDocumentMgrs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfDocumentMgrs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumTfDocumentMgrs {
    type Vtable = IEnumTfDocumentMgrs_Vtbl;
}
impl ::core::clone::Clone for IEnumTfDocumentMgrs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumTfDocumentMgrs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e808_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfDocumentMgrs_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, rgdocumentmgr: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IEnumTfFunctionProviders(::windows::core::IUnknown);
impl IEnumTfFunctionProviders {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfFunctionProviders> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfFunctionProviders>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Next(&self, ppcmdobj: &mut [::core::option::Option<ITfFunctionProvider>], pcfetch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ppcmdobj.len() as _, ::core::mem::transmute(ppcmdobj.as_ptr()), pcfetch).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), ulcount).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumTfFunctionProviders, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumTfFunctionProviders {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfFunctionProviders {}
impl ::core::fmt::Debug for IEnumTfFunctionProviders {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfFunctionProviders").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumTfFunctionProviders {
    type Vtable = IEnumTfFunctionProviders_Vtbl;
}
impl ::core::clone::Clone for IEnumTfFunctionProviders {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumTfFunctionProviders {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4b24db0_0990_11d3_8df0_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfFunctionProviders_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, ppcmdobj: *mut *mut ::core::ffi::c_void, pcfetch: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IEnumTfInputProcessorProfiles(::windows::core::IUnknown);
impl IEnumTfInputProcessorProfiles {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfInputProcessorProfiles> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfInputProcessorProfiles>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Next(&self, pprofile: &mut [TF_INPUTPROCESSORPROFILE], pcfetch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), pprofile.len() as _, ::core::mem::transmute(pprofile.as_ptr()), pcfetch).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), ulcount).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumTfInputProcessorProfiles, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumTfInputProcessorProfiles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfInputProcessorProfiles {}
impl ::core::fmt::Debug for IEnumTfInputProcessorProfiles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfInputProcessorProfiles").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumTfInputProcessorProfiles {
    type Vtable = IEnumTfInputProcessorProfiles_Vtbl;
}
impl ::core::clone::Clone for IEnumTfInputProcessorProfiles {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumTfInputProcessorProfiles {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71c6e74d_0f28_11d8_a82a_00065b84435c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfInputProcessorProfiles_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, pprofile: *mut TF_INPUTPROCESSORPROFILE, pcfetch: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IEnumTfLangBarItems(::windows::core::IUnknown);
impl IEnumTfLangBarItems {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfLangBarItems> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfLangBarItems>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Next(&self, ppitem: &mut [::core::option::Option<ITfLangBarItem>], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ppitem.len() as _, ::core::mem::transmute(ppitem.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), ulcount).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumTfLangBarItems, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumTfLangBarItems {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfLangBarItems {}
impl ::core::fmt::Debug for IEnumTfLangBarItems {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfLangBarItems").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumTfLangBarItems {
    type Vtable = IEnumTfLangBarItems_Vtbl;
}
impl ::core::clone::Clone for IEnumTfLangBarItems {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumTfLangBarItems {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x583f34d0_de25_11d2_afdd_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfLangBarItems_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, ppitem: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IEnumTfLanguageProfiles(::windows::core::IUnknown);
impl IEnumTfLanguageProfiles {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfLanguageProfiles> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfLanguageProfiles>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Next(&self, pprofile: &mut [TF_LANGUAGEPROFILE], pcfetch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), pprofile.len() as _, ::core::mem::transmute(pprofile.as_ptr()), pcfetch).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), ulcount).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumTfLanguageProfiles, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumTfLanguageProfiles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfLanguageProfiles {}
impl ::core::fmt::Debug for IEnumTfLanguageProfiles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfLanguageProfiles").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumTfLanguageProfiles {
    type Vtable = IEnumTfLanguageProfiles_Vtbl;
}
impl ::core::clone::Clone for IEnumTfLanguageProfiles {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumTfLanguageProfiles {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d61bf11_ac5f_42c8_a4cb_931bcc28c744);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfLanguageProfiles_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, pprofile: *mut TF_LANGUAGEPROFILE, pcfetch: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IEnumTfLatticeElements(::windows::core::IUnknown);
impl IEnumTfLatticeElements {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfLatticeElements> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfLatticeElements>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Next(&self, rgselements: &mut [TF_LMLATTELEMENT], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgselements.len() as _, ::core::mem::transmute(rgselements.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), ulcount).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumTfLatticeElements, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumTfLatticeElements {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfLatticeElements {}
impl ::core::fmt::Debug for IEnumTfLatticeElements {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfLatticeElements").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumTfLatticeElements {
    type Vtable = IEnumTfLatticeElements_Vtbl;
}
impl ::core::clone::Clone for IEnumTfLatticeElements {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumTfLatticeElements {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56988052_47da_4a05_911a_e3d941f17145);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfLatticeElements_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, rgselements: *mut TF_LMLATTELEMENT, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IEnumTfProperties(::windows::core::IUnknown);
impl IEnumTfProperties {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfProperties> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfProperties>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Next(&self, ppprop: &mut [::core::option::Option<ITfProperty>], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ppprop.len() as _, ::core::mem::transmute(ppprop.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), ulcount).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumTfProperties, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumTfProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfProperties {}
impl ::core::fmt::Debug for IEnumTfProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumTfProperties {
    type Vtable = IEnumTfProperties_Vtbl;
}
impl ::core::clone::Clone for IEnumTfProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumTfProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19188cb0_aca9_11d2_afc5_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfProperties_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, ppprop: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IEnumTfPropertyValue(::windows::core::IUnknown);
impl IEnumTfPropertyValue {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfPropertyValue> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfPropertyValue>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Next(&self, rgvalues: &mut [TF_PROPERTYVAL], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgvalues.len() as _, ::core::mem::transmute(rgvalues.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), ulcount).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumTfPropertyValue, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumTfPropertyValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfPropertyValue {}
impl ::core::fmt::Debug for IEnumTfPropertyValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfPropertyValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumTfPropertyValue {
    type Vtable = IEnumTfPropertyValue_Vtbl;
}
impl ::core::clone::Clone for IEnumTfPropertyValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumTfPropertyValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ed8981b_7c10_4d7d_9fb3_ab72e9c75f72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfPropertyValue_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, rgvalues: *mut TF_PROPERTYVAL, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Next: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IEnumTfRanges(::windows::core::IUnknown);
impl IEnumTfRanges {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfRanges> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfRanges>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Next(&self, pprange: &mut [::core::option::Option<ITfRange>], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), pprange.len() as _, ::core::mem::transmute(pprange.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), ulcount).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumTfRanges, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumTfRanges {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfRanges {}
impl ::core::fmt::Debug for IEnumTfRanges {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfRanges").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumTfRanges {
    type Vtable = IEnumTfRanges_Vtbl;
}
impl ::core::clone::Clone for IEnumTfRanges {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumTfRanges {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf99d3f40_8e32_11d2_bf46_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfRanges_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, pprange: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IEnumTfUIElements(::windows::core::IUnknown);
impl IEnumTfUIElements {
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTfUIElements> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfUIElements>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Next(&self, ppelement: &mut [::core::option::Option<ITfUIElement>], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ppelement.len() as _, ::core::mem::transmute(ppelement.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip(&self, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), ulcount).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumTfUIElements, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumTfUIElements {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTfUIElements {}
impl ::core::fmt::Debug for IEnumTfUIElements {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTfUIElements").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumTfUIElements {
    type Vtable = IEnumTfUIElements_Vtbl;
}
impl ::core::clone::Clone for IEnumTfUIElements {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumTfUIElements {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x887aa91e_acba_4931_84da_3c5208cf543f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTfUIElements_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, ppelement: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IInternalDocWrap(::windows::core::IUnknown);
impl IInternalDocWrap {
    pub unsafe fn NotifyRevoke(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NotifyRevoke)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IInternalDocWrap, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IInternalDocWrap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInternalDocWrap {}
impl ::core::fmt::Debug for IInternalDocWrap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInternalDocWrap").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IInternalDocWrap {
    type Vtable = IInternalDocWrap_Vtbl;
}
impl ::core::clone::Clone for IInternalDocWrap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInternalDocWrap {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1aa6466_9db4_40ba_be03_77c38e8e60b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInternalDocWrap_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub NotifyRevoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ISpeechCommandProvider(::windows::core::IUnknown);
impl ISpeechCommandProvider {
    pub unsafe fn EnumSpeechCommands(&self, langid: u16) -> ::windows::core::Result<IEnumSpeechCommands> {
        let mut result__ = ::windows::core::zeroed::<IEnumSpeechCommands>();
        (::windows::core::Interface::vtable(self).EnumSpeechCommands)(::windows::core::Interface::as_raw(self), langid, &mut result__).from_abi(result__)
    }
    pub unsafe fn ProcessCommand(&self, pszcommand: &[u16], langid: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessCommand)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pszcommand.as_ptr()), pszcommand.len() as _, langid).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpeechCommandProvider, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpeechCommandProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpeechCommandProvider {}
impl ::core::fmt::Debug for ISpeechCommandProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechCommandProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpeechCommandProvider {
    type Vtable = ISpeechCommandProvider_Vtbl;
}
impl ::core::clone::Clone for ISpeechCommandProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpeechCommandProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38e09d4c_586d_435a_b592_c8a86691dec6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechCommandProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EnumSpeechCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProcessCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcommand: ::windows::core::PCWSTR, cch: u32, langid: u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITextStoreACP(::windows::core::IUnknown);
impl ITextStoreACP {
    pub unsafe fn AdviseSink<P0>(&self, riid: *const ::windows::core::GUID, punk: P0, dwmask: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).AdviseSink)(::windows::core::Interface::as_raw(self), riid, punk.into_param().abi(), dwmask).ok()
    }
    pub unsafe fn UnadviseSink<P0>(&self, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).UnadviseSink)(::windows::core::Interface::as_raw(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn RequestLock(&self, dwlockflags: u32) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
        (::windows::core::Interface::vtable(self).RequestLock)(::windows::core::Interface::as_raw(self), dwlockflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<TS_STATUS> {
        let mut result__ = ::windows::core::zeroed::<TS_STATUS>();
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryInsert(&self, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryInsert)(::windows::core::Interface::as_raw(self), acpteststart, acptestend, cch, pacpresultstart, pacpresultend).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSelection(&self, ulindex: u32, pselection: &mut [TS_SELECTION_ACP], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSelection)(::windows::core::Interface::as_raw(self), ulindex, pselection.len() as _, ::core::mem::transmute(pselection.as_ptr()), pcfetched).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSelection(&self, pselection: &[TS_SELECTION_ACP]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSelection)(::windows::core::Interface::as_raw(self), pselection.len() as _, ::core::mem::transmute(pselection.as_ptr())).ok()
    }
    pub unsafe fn GetText(&self, acpstart: i32, acpend: i32, pchplain: &mut [u16], pcchplainret: *mut u32, prgruninfo: &mut [TS_RUNINFO], pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetText)(::windows::core::Interface::as_raw(self), acpstart, acpend, ::core::mem::transmute(pchplain.as_ptr()), pchplain.len() as _, pcchplainret, ::core::mem::transmute(prgruninfo.as_ptr()), prgruninfo.len() as _, pcruninforet, pacpnext).ok()
    }
    pub unsafe fn SetText(&self, dwflags: u32, acpstart: i32, acpend: i32, pchtext: &[u16]) -> ::windows::core::Result<TS_TEXTCHANGE> {
        let mut result__ = ::windows::core::zeroed::<TS_TEXTCHANGE>();
        (::windows::core::Interface::vtable(self).SetText)(::windows::core::Interface::as_raw(self), dwflags, acpstart, acpend, ::core::mem::transmute(pchtext.as_ptr()), pchtext.len() as _, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText(&self, acpstart: i32, acpend: i32) -> ::windows::core::Result<super::super::System::Com::IDataObject> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IDataObject>();
        (::windows::core::Interface::vtable(self).GetFormattedText)(::windows::core::Interface::as_raw(self), acpstart, acpend, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEmbedded(&self, acppos: i32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetEmbedded)(::windows::core::Interface::as_raw(self), acppos, rguidservice, riid, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn QueryInsertEmbedded(&self, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).QueryInsertEmbedded)(::windows::core::Interface::as_raw(self), pguidservice, pformatetc, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbedded<P0>(&self, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: P0) -> ::windows::core::Result<TS_TEXTCHANGE>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IDataObject>,
    {
        let mut result__ = ::windows::core::zeroed::<TS_TEXTCHANGE>();
        (::windows::core::Interface::vtable(self).InsertEmbedded)(::windows::core::Interface::as_raw(self), dwflags, acpstart, acpend, pdataobject.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn InsertTextAtSelection(&self, dwflags: u32, pchtext: &[u16], pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InsertTextAtSelection)(::windows::core::Interface::as_raw(self), dwflags, ::core::mem::transmute(pchtext.as_ptr()), pchtext.len() as _, pacpstart, pacpend, pchange).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbeddedAtSelection<P0>(&self, dwflags: u32, pdataobject: P0, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IDataObject>,
    {
        (::windows::core::Interface::vtable(self).InsertEmbeddedAtSelection)(::windows::core::Interface::as_raw(self), dwflags, pdataobject.into_param().abi(), pacpstart, pacpend, pchange).ok()
    }
    pub unsafe fn RequestSupportedAttrs(&self, dwflags: u32, pafilterattrs: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestSupportedAttrs)(::windows::core::Interface::as_raw(self), dwflags, pafilterattrs.len() as _, ::core::mem::transmute(pafilterattrs.as_ptr())).ok()
    }
    pub unsafe fn RequestAttrsAtPosition(&self, acppos: i32, pafilterattrs: &[::windows::core::GUID], dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestAttrsAtPosition)(::windows::core::Interface::as_raw(self), acppos, pafilterattrs.len() as _, ::core::mem::transmute(pafilterattrs.as_ptr()), dwflags).ok()
    }
    pub unsafe fn RequestAttrsTransitioningAtPosition(&self, acppos: i32, pafilterattrs: &[::windows::core::GUID], dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestAttrsTransitioningAtPosition)(::windows::core::Interface::as_raw(self), acppos, pafilterattrs.len() as _, ::core::mem::transmute(pafilterattrs.as_ptr()), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindNextAttrTransition(&self, acpstart: i32, acphalt: i32, pafilterattrs: &[::windows::core::GUID], dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FindNextAttrTransition)(::windows::core::Interface::as_raw(self), acpstart, acphalt, pafilterattrs.len() as _, ::core::mem::transmute(pafilterattrs.as_ptr()), dwflags, pacpnext, pffound, plfoundoffset).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RetrieveRequestedAttrs(&self, paattrvals: &mut [TS_ATTRVAL], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RetrieveRequestedAttrs)(::windows::core::Interface::as_raw(self), paattrvals.len() as _, ::core::mem::transmute(paattrvals.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn GetEndACP(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).GetEndACP)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetActiveView(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetActiveView)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetACPFromPoint(&self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).GetACPFromPoint)(::windows::core::Interface::as_raw(self), vcview, ptscreen, dwflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTextExt(&self, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTextExt)(::windows::core::Interface::as_raw(self), vcview, acpstart, acpend, prc, pfclipped).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetScreenExt(&self, vcview: u32) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::RECT>();
        (::windows::core::Interface::vtable(self).GetScreenExt)(::windows::core::Interface::as_raw(self), vcview, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWnd(&self, vcview: u32) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).GetWnd)(::windows::core::Interface::as_raw(self), vcview, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITextStoreACP, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITextStoreACP {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreACP {}
impl ::core::fmt::Debug for ITextStoreACP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoreACP").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextStoreACP {
    type Vtable = ITextStoreACP_Vtbl;
}
impl ::core::clone::Clone for ITextStoreACP {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITextStoreACP {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28888fe3_c2a0_483a_a3ea_8cb1ce51ff3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACP_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AdviseSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT,
    pub UnadviseSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RequestLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlockflags: u32, phrsession: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT,
    pub QueryInsert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSelection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSelection: usize,
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pchplain: ::windows::core::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pchtext: ::windows::core::PCWSTR, cch: u32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFormattedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFormattedText: usize,
    pub GetEmbedded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acppos: i32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub QueryInsertEmbedded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    QueryInsertEmbedded: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbedded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: *mut ::core::ffi::c_void, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbedded: usize,
    pub InsertTextAtSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pchtext: ::windows::core::PCWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbeddedAtSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pdataobject: *mut ::core::ffi::c_void, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbeddedAtSelection: usize,
    pub RequestSupportedAttrs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RequestAttrsAtPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT,
    pub RequestAttrsTransitioningAtPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FindNextAttrTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindNextAttrTransition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RetrieveRequestedAttrs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RetrieveRequestedAttrs: usize,
    pub GetEndACP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pacp: *mut i32) -> ::windows::core::HRESULT,
    pub GetActiveView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvcview: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetACPFromPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetACPFromPoint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTextExt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTextExt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetScreenExt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetScreenExt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcview: u32, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWnd: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITextStoreACP2(::windows::core::IUnknown);
impl ITextStoreACP2 {
    pub unsafe fn AdviseSink<P0>(&self, riid: *const ::windows::core::GUID, punk: P0, dwmask: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).AdviseSink)(::windows::core::Interface::as_raw(self), riid, punk.into_param().abi(), dwmask).ok()
    }
    pub unsafe fn UnadviseSink<P0>(&self, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).UnadviseSink)(::windows::core::Interface::as_raw(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn RequestLock(&self, dwlockflags: u32) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
        (::windows::core::Interface::vtable(self).RequestLock)(::windows::core::Interface::as_raw(self), dwlockflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<TS_STATUS> {
        let mut result__ = ::windows::core::zeroed::<TS_STATUS>();
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryInsert(&self, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryInsert)(::windows::core::Interface::as_raw(self), acpteststart, acptestend, cch, pacpresultstart, pacpresultend).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSelection(&self, ulindex: u32, pselection: &mut [TS_SELECTION_ACP], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSelection)(::windows::core::Interface::as_raw(self), ulindex, pselection.len() as _, ::core::mem::transmute(pselection.as_ptr()), pcfetched).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSelection(&self, pselection: &[TS_SELECTION_ACP]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSelection)(::windows::core::Interface::as_raw(self), pselection.len() as _, ::core::mem::transmute(pselection.as_ptr())).ok()
    }
    pub unsafe fn GetText(&self, acpstart: i32, acpend: i32, pchplain: &mut [u16], pcchplainret: *mut u32, prgruninfo: &mut [TS_RUNINFO], pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetText)(::windows::core::Interface::as_raw(self), acpstart, acpend, ::core::mem::transmute(pchplain.as_ptr()), pchplain.len() as _, pcchplainret, ::core::mem::transmute(prgruninfo.as_ptr()), prgruninfo.len() as _, pcruninforet, pacpnext).ok()
    }
    pub unsafe fn SetText(&self, dwflags: u32, acpstart: i32, acpend: i32, pchtext: &[u16]) -> ::windows::core::Result<TS_TEXTCHANGE> {
        let mut result__ = ::windows::core::zeroed::<TS_TEXTCHANGE>();
        (::windows::core::Interface::vtable(self).SetText)(::windows::core::Interface::as_raw(self), dwflags, acpstart, acpend, ::core::mem::transmute(pchtext.as_ptr()), pchtext.len() as _, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText(&self, acpstart: i32, acpend: i32) -> ::windows::core::Result<super::super::System::Com::IDataObject> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IDataObject>();
        (::windows::core::Interface::vtable(self).GetFormattedText)(::windows::core::Interface::as_raw(self), acpstart, acpend, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEmbedded(&self, acppos: i32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetEmbedded)(::windows::core::Interface::as_raw(self), acppos, rguidservice, riid, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn QueryInsertEmbedded(&self, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).QueryInsertEmbedded)(::windows::core::Interface::as_raw(self), pguidservice, pformatetc, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbedded<P0>(&self, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: P0) -> ::windows::core::Result<TS_TEXTCHANGE>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IDataObject>,
    {
        let mut result__ = ::windows::core::zeroed::<TS_TEXTCHANGE>();
        (::windows::core::Interface::vtable(self).InsertEmbedded)(::windows::core::Interface::as_raw(self), dwflags, acpstart, acpend, pdataobject.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn InsertTextAtSelection(&self, dwflags: u32, pchtext: &[u16], pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InsertTextAtSelection)(::windows::core::Interface::as_raw(self), dwflags, ::core::mem::transmute(pchtext.as_ptr()), pchtext.len() as _, pacpstart, pacpend, pchange).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbeddedAtSelection<P0>(&self, dwflags: u32, pdataobject: P0, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IDataObject>,
    {
        (::windows::core::Interface::vtable(self).InsertEmbeddedAtSelection)(::windows::core::Interface::as_raw(self), dwflags, pdataobject.into_param().abi(), pacpstart, pacpend, pchange).ok()
    }
    pub unsafe fn RequestSupportedAttrs(&self, dwflags: u32, pafilterattrs: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestSupportedAttrs)(::windows::core::Interface::as_raw(self), dwflags, pafilterattrs.len() as _, ::core::mem::transmute(pafilterattrs.as_ptr())).ok()
    }
    pub unsafe fn RequestAttrsAtPosition(&self, acppos: i32, pafilterattrs: &[::windows::core::GUID], dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestAttrsAtPosition)(::windows::core::Interface::as_raw(self), acppos, pafilterattrs.len() as _, ::core::mem::transmute(pafilterattrs.as_ptr()), dwflags).ok()
    }
    pub unsafe fn RequestAttrsTransitioningAtPosition(&self, acppos: i32, pafilterattrs: &[::windows::core::GUID], dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestAttrsTransitioningAtPosition)(::windows::core::Interface::as_raw(self), acppos, pafilterattrs.len() as _, ::core::mem::transmute(pafilterattrs.as_ptr()), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindNextAttrTransition(&self, acpstart: i32, acphalt: i32, pafilterattrs: &[::windows::core::GUID], dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FindNextAttrTransition)(::windows::core::Interface::as_raw(self), acpstart, acphalt, pafilterattrs.len() as _, ::core::mem::transmute(pafilterattrs.as_ptr()), dwflags, pacpnext, pffound, plfoundoffset).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RetrieveRequestedAttrs(&self, paattrvals: &mut [TS_ATTRVAL], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RetrieveRequestedAttrs)(::windows::core::Interface::as_raw(self), paattrvals.len() as _, ::core::mem::transmute(paattrvals.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn GetEndACP(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).GetEndACP)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetActiveView(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetActiveView)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetACPFromPoint(&self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).GetACPFromPoint)(::windows::core::Interface::as_raw(self), vcview, ptscreen, dwflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTextExt(&self, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTextExt)(::windows::core::Interface::as_raw(self), vcview, acpstart, acpend, prc, pfclipped).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetScreenExt(&self, vcview: u32) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::RECT>();
        (::windows::core::Interface::vtable(self).GetScreenExt)(::windows::core::Interface::as_raw(self), vcview, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITextStoreACP2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITextStoreACP2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreACP2 {}
impl ::core::fmt::Debug for ITextStoreACP2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoreACP2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextStoreACP2 {
    type Vtable = ITextStoreACP2_Vtbl;
}
impl ::core::clone::Clone for ITextStoreACP2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITextStoreACP2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf86ad89f_5fe4_4b8d_bb9f_ef3797a84f1f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACP2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AdviseSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT,
    pub UnadviseSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RequestLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlockflags: u32, phrsession: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT,
    pub QueryInsert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSelection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSelection: usize,
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pchplain: ::windows::core::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pchtext: ::windows::core::PCWSTR, cch: u32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFormattedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFormattedText: usize,
    pub GetEmbedded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acppos: i32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub QueryInsertEmbedded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    QueryInsertEmbedded: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbedded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: *mut ::core::ffi::c_void, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbedded: usize,
    pub InsertTextAtSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pchtext: ::windows::core::PCWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbeddedAtSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pdataobject: *mut ::core::ffi::c_void, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbeddedAtSelection: usize,
    pub RequestSupportedAttrs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RequestAttrsAtPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT,
    pub RequestAttrsTransitioningAtPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FindNextAttrTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindNextAttrTransition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RetrieveRequestedAttrs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RetrieveRequestedAttrs: usize,
    pub GetEndACP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pacp: *mut i32) -> ::windows::core::HRESULT,
    pub GetActiveView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvcview: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetACPFromPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetACPFromPoint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTextExt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTextExt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetScreenExt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetScreenExt: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITextStoreACPEx(::windows::core::IUnknown);
impl ITextStoreACPEx {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ScrollToRect(&self, acpstart: i32, acpend: i32, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ScrollToRect)(::windows::core::Interface::as_raw(self), acpstart, acpend, ::core::mem::transmute(rc), dwposition).ok()
    }
}
::windows::imp::interface_hierarchy!(ITextStoreACPEx, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITextStoreACPEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreACPEx {}
impl ::core::fmt::Debug for ITextStoreACPEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoreACPEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextStoreACPEx {
    type Vtable = ITextStoreACPEx_Vtbl;
}
impl ::core::clone::Clone for ITextStoreACPEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITextStoreACPEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2de3bc2_3d8e_11d3_81a9_f753fbe61a00);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACPEx_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ScrollToRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ScrollToRect: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITextStoreACPServices(::windows::core::IUnknown);
impl ITextStoreACPServices {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Serialize<P0, P1, P2>(&self, pprop: P0, prange: P1, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfProperty>,
        P1: ::windows::core::IntoParam<ITfRange>,
        P2: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).Serialize)(::windows::core::Interface::as_raw(self), pprop.into_param().abi(), prange.into_param().abi(), phdr, pstream.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Unserialize<P0, P1, P2>(&self, pprop: P0, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: P1, ploader: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfProperty>,
        P1: ::windows::core::IntoParam<super::super::System::Com::IStream>,
        P2: ::windows::core::IntoParam<ITfPersistentPropertyLoaderACP>,
    {
        (::windows::core::Interface::vtable(self).Unserialize)(::windows::core::Interface::as_raw(self), pprop.into_param().abi(), phdr, pstream.into_param().abi(), ploader.into_param().abi()).ok()
    }
    pub unsafe fn ForceLoadProperty<P0>(&self, pprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfProperty>,
    {
        (::windows::core::Interface::vtable(self).ForceLoadProperty)(::windows::core::Interface::as_raw(self), pprop.into_param().abi()).ok()
    }
    pub unsafe fn CreateRange(&self, acpstart: i32, acpend: i32) -> ::windows::core::Result<ITfRangeACP> {
        let mut result__ = ::windows::core::zeroed::<ITfRangeACP>();
        (::windows::core::Interface::vtable(self).CreateRange)(::windows::core::Interface::as_raw(self), acpstart, acpend, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITextStoreACPServices, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITextStoreACPServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreACPServices {}
impl ::core::fmt::Debug for ITextStoreACPServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoreACPServices").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextStoreACPServices {
    type Vtable = ITextStoreACPServices_Vtbl;
}
impl ::core::clone::Clone for ITextStoreACPServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITextStoreACPServices {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e901_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACPServices_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprop: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Serialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Unserialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprop: *mut ::core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut ::core::ffi::c_void, ploader: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Unserialize: usize,
    pub ForceLoadProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprop: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITextStoreACPSink(::windows::core::IUnknown);
impl ITextStoreACPSink {
    pub unsafe fn OnTextChange(&self, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnTextChange)(::windows::core::Interface::as_raw(self), dwflags, pchange).ok()
    }
    pub unsafe fn OnSelectionChange(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSelectionChange)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnLayoutChange)(::windows::core::Interface::as_raw(self), lcode, vcview).ok()
    }
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStatusChange)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn OnAttrsChange(&self, acpstart: i32, acpend: i32, paattrs: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnAttrsChange)(::windows::core::Interface::as_raw(self), acpstart, acpend, paattrs.len() as _, ::core::mem::transmute(paattrs.as_ptr())).ok()
    }
    pub unsafe fn OnLockGranted(&self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnLockGranted)(::windows::core::Interface::as_raw(self), dwlockflags).ok()
    }
    pub unsafe fn OnStartEditTransaction(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStartEditTransaction)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnEndEditTransaction(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnEndEditTransaction)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ITextStoreACPSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITextStoreACPSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreACPSink {}
impl ::core::fmt::Debug for ITextStoreACPSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoreACPSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextStoreACPSink {
    type Vtable = ITextStoreACPSink_Vtbl;
}
impl ::core::clone::Clone for ITextStoreACPSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITextStoreACPSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22d44c94_a419_4542_a272_ae26093ececf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACPSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnTextChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> ::windows::core::HRESULT,
    pub OnSelectionChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnLayoutChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::HRESULT,
    pub OnStatusChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub OnAttrsChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnLockGranted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::HRESULT,
    pub OnStartEditTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnEndEditTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITextStoreACPSinkEx(::windows::core::IUnknown);
impl ITextStoreACPSinkEx {
    pub unsafe fn OnTextChange(&self, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnTextChange)(::windows::core::Interface::as_raw(self), dwflags, pchange).ok()
    }
    pub unsafe fn OnSelectionChange(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnSelectionChange)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnLayoutChange)(::windows::core::Interface::as_raw(self), lcode, vcview).ok()
    }
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnStatusChange)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn OnAttrsChange(&self, acpstart: i32, acpend: i32, paattrs: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnAttrsChange)(::windows::core::Interface::as_raw(self), acpstart, acpend, paattrs.len() as _, ::core::mem::transmute(paattrs.as_ptr())).ok()
    }
    pub unsafe fn OnLockGranted(&self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnLockGranted)(::windows::core::Interface::as_raw(self), dwlockflags).ok()
    }
    pub unsafe fn OnStartEditTransaction(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnStartEditTransaction)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnEndEditTransaction(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnEndEditTransaction)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnDisconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDisconnect)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ITextStoreACPSinkEx, ::windows::core::IUnknown, ITextStoreACPSink);
impl ::core::cmp::PartialEq for ITextStoreACPSinkEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreACPSinkEx {}
impl ::core::fmt::Debug for ITextStoreACPSinkEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoreACPSinkEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextStoreACPSinkEx {
    type Vtable = ITextStoreACPSinkEx_Vtbl;
}
impl ::core::clone::Clone for ITextStoreACPSinkEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITextStoreACPSinkEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bdf9464_41e2_43e3_950c_a6865ba25cd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreACPSinkEx_Vtbl {
    pub base__: ITextStoreACPSink_Vtbl,
    pub OnDisconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITextStoreAnchor(::windows::core::IUnknown);
impl ITextStoreAnchor {
    pub unsafe fn AdviseSink<P0>(&self, riid: *const ::windows::core::GUID, punk: P0, dwmask: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).AdviseSink)(::windows::core::Interface::as_raw(self), riid, punk.into_param().abi(), dwmask).ok()
    }
    pub unsafe fn UnadviseSink<P0>(&self, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).UnadviseSink)(::windows::core::Interface::as_raw(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn RequestLock(&self, dwlockflags: u32) -> ::windows::core::Result<::windows::core::HRESULT> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
        (::windows::core::Interface::vtable(self).RequestLock)(::windows::core::Interface::as_raw(self), dwlockflags, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<TS_STATUS> {
        let mut result__ = ::windows::core::zeroed::<TS_STATUS>();
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryInsert<P0, P1>(&self, pateststart: P0, patestend: P1, cch: u32, pparesultstart: *mut ::core::option::Option<IAnchor>, pparesultend: *mut ::core::option::Option<IAnchor>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAnchor>,
        P1: ::windows::core::IntoParam<IAnchor>,
    {
        (::windows::core::Interface::vtable(self).QueryInsert)(::windows::core::Interface::as_raw(self), pateststart.into_param().abi(), patestend.into_param().abi(), cch, ::core::mem::transmute(pparesultstart), ::core::mem::transmute(pparesultend)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSelection(&self, ulindex: u32, pselection: &mut [TS_SELECTION_ANCHOR], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSelection)(::windows::core::Interface::as_raw(self), ulindex, pselection.len() as _, ::core::mem::transmute(pselection.as_ptr()), pcfetched).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSelection(&self, pselection: &[TS_SELECTION_ANCHOR]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSelection)(::windows::core::Interface::as_raw(self), pselection.len() as _, ::core::mem::transmute(pselection.as_ptr())).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText<P0, P1, P2>(&self, dwflags: u32, pastart: P0, paend: P1, pchtext: &mut [u16], pcch: *mut u32, fupdateanchor: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAnchor>,
        P1: ::windows::core::IntoParam<IAnchor>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).GetText)(::windows::core::Interface::as_raw(self), dwflags, pastart.into_param().abi(), paend.into_param().abi(), ::core::mem::transmute(pchtext.as_ptr()), pchtext.len() as _, pcch, fupdateanchor.into_param().abi()).ok()
    }
    pub unsafe fn SetText<P0, P1>(&self, dwflags: u32, pastart: P0, paend: P1, pchtext: &[u16]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAnchor>,
        P1: ::windows::core::IntoParam<IAnchor>,
    {
        (::windows::core::Interface::vtable(self).SetText)(::windows::core::Interface::as_raw(self), dwflags, pastart.into_param().abi(), paend.into_param().abi(), ::core::mem::transmute(pchtext.as_ptr()), pchtext.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText<P0, P1>(&self, pastart: P0, paend: P1) -> ::windows::core::Result<super::super::System::Com::IDataObject>
    where
        P0: ::windows::core::IntoParam<IAnchor>,
        P1: ::windows::core::IntoParam<IAnchor>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IDataObject>();
        (::windows::core::Interface::vtable(self).GetFormattedText)(::windows::core::Interface::as_raw(self), pastart.into_param().abi(), paend.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEmbedded<P0>(&self, dwflags: u32, papos: P0, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<IAnchor>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetEmbedded)(::windows::core::Interface::as_raw(self), dwflags, papos.into_param().abi(), rguidservice, riid, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbedded<P0, P1, P2>(&self, dwflags: u32, pastart: P0, paend: P1, pdataobject: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAnchor>,
        P1: ::windows::core::IntoParam<IAnchor>,
        P2: ::windows::core::IntoParam<super::super::System::Com::IDataObject>,
    {
        (::windows::core::Interface::vtable(self).InsertEmbedded)(::windows::core::Interface::as_raw(self), dwflags, pastart.into_param().abi(), paend.into_param().abi(), pdataobject.into_param().abi()).ok()
    }
    pub unsafe fn RequestSupportedAttrs(&self, dwflags: u32, pafilterattrs: &[::windows::core::GUID]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RequestSupportedAttrs)(::windows::core::Interface::as_raw(self), dwflags, pafilterattrs.len() as _, ::core::mem::transmute(pafilterattrs.as_ptr())).ok()
    }
    pub unsafe fn RequestAttrsAtPosition<P0>(&self, papos: P0, pafilterattrs: &[::windows::core::GUID], dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAnchor>,
    {
        (::windows::core::Interface::vtable(self).RequestAttrsAtPosition)(::windows::core::Interface::as_raw(self), papos.into_param().abi(), pafilterattrs.len() as _, ::core::mem::transmute(pafilterattrs.as_ptr()), dwflags).ok()
    }
    pub unsafe fn RequestAttrsTransitioningAtPosition<P0>(&self, papos: P0, pafilterattrs: &[::windows::core::GUID], dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAnchor>,
    {
        (::windows::core::Interface::vtable(self).RequestAttrsTransitioningAtPosition)(::windows::core::Interface::as_raw(self), papos.into_param().abi(), pafilterattrs.len() as _, ::core::mem::transmute(pafilterattrs.as_ptr()), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindNextAttrTransition<P0, P1>(&self, pastart: P0, pahalt: P1, pafilterattrs: &[::windows::core::GUID], dwflags: u32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAnchor>,
        P1: ::windows::core::IntoParam<IAnchor>,
    {
        (::windows::core::Interface::vtable(self).FindNextAttrTransition)(::windows::core::Interface::as_raw(self), pastart.into_param().abi(), pahalt.into_param().abi(), pafilterattrs.len() as _, ::core::mem::transmute(pafilterattrs.as_ptr()), dwflags, pffound, plfoundoffset).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RetrieveRequestedAttrs(&self, paattrvals: &mut [TS_ATTRVAL], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RetrieveRequestedAttrs)(::windows::core::Interface::as_raw(self), paattrvals.len() as _, ::core::mem::transmute(paattrvals.as_ptr()), pcfetched).ok()
    }
    pub unsafe fn GetStart(&self) -> ::windows::core::Result<IAnchor> {
        let mut result__ = ::windows::core::zeroed::<IAnchor>();
        (::windows::core::Interface::vtable(self).GetStart)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEnd(&self) -> ::windows::core::Result<IAnchor> {
        let mut result__ = ::windows::core::zeroed::<IAnchor>();
        (::windows::core::Interface::vtable(self).GetEnd)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetActiveView(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetActiveView)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAnchorFromPoint(&self, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows::core::Result<IAnchor> {
        let mut result__ = ::windows::core::zeroed::<IAnchor>();
        (::windows::core::Interface::vtable(self).GetAnchorFromPoint)(::windows::core::Interface::as_raw(self), vcview, ptscreen, dwflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTextExt<P0, P1>(&self, vcview: u32, pastart: P0, paend: P1, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAnchor>,
        P1: ::windows::core::IntoParam<IAnchor>,
    {
        (::windows::core::Interface::vtable(self).GetTextExt)(::windows::core::Interface::as_raw(self), vcview, pastart.into_param().abi(), paend.into_param().abi(), prc, pfclipped).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetScreenExt(&self, vcview: u32) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::RECT>();
        (::windows::core::Interface::vtable(self).GetScreenExt)(::windows::core::Interface::as_raw(self), vcview, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWnd(&self, vcview: u32) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).GetWnd)(::windows::core::Interface::as_raw(self), vcview, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn QueryInsertEmbedded(&self, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).QueryInsertEmbedded)(::windows::core::Interface::as_raw(self), pguidservice, pformatetc, &mut result__).from_abi(result__)
    }
    pub unsafe fn InsertTextAtSelection(&self, dwflags: u32, pchtext: &[u16], ppastart: *mut ::core::option::Option<IAnchor>, ppaend: *mut ::core::option::Option<IAnchor>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InsertTextAtSelection)(::windows::core::Interface::as_raw(self), dwflags, ::core::mem::transmute(pchtext.as_ptr()), pchtext.len() as _, ::core::mem::transmute(ppastart), ::core::mem::transmute(ppaend)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbeddedAtSelection<P0>(&self, dwflags: u32, pdataobject: P0, ppastart: *mut ::core::option::Option<IAnchor>, ppaend: *mut ::core::option::Option<IAnchor>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IDataObject>,
    {
        (::windows::core::Interface::vtable(self).InsertEmbeddedAtSelection)(::windows::core::Interface::as_raw(self), dwflags, pdataobject.into_param().abi(), ::core::mem::transmute(ppastart), ::core::mem::transmute(ppaend)).ok()
    }
}
::windows::imp::interface_hierarchy!(ITextStoreAnchor, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITextStoreAnchor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreAnchor {}
impl ::core::fmt::Debug for ITextStoreAnchor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoreAnchor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextStoreAnchor {
    type Vtable = ITextStoreAnchor_Vtbl;
}
impl ::core::clone::Clone for ITextStoreAnchor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITextStoreAnchor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b2077b0_5f18_4dec_bee9_3cc722f5dfe0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreAnchor_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AdviseSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows::core::HRESULT,
    pub UnadviseSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RequestLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlockflags: u32, phrsession: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT,
    pub QueryInsert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pateststart: *mut ::core::ffi::c_void, patestend: *mut ::core::ffi::c_void, cch: u32, pparesultstart: *mut *mut ::core::ffi::c_void, pparesultend: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ANCHOR, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSelection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ANCHOR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSelection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, pchtext: ::windows::core::PWSTR, cchreq: u32, pcch: *mut u32, fupdateanchor: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetText: usize,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, pchtext: ::windows::core::PCWSTR, cch: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFormattedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFormattedText: usize,
    pub GetEmbedded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, papos: *mut ::core::ffi::c_void, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbedded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbedded: usize,
    pub RequestSupportedAttrs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RequestAttrsAtPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, papos: *mut ::core::ffi::c_void, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT,
    pub RequestAttrsTransitioningAtPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, papos: *mut ::core::ffi::c_void, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FindNextAttrTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pastart: *mut ::core::ffi::c_void, pahalt: *mut ::core::ffi::c_void, cfilterattrs: u32, pafilterattrs: *const ::windows::core::GUID, dwflags: u32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindNextAttrTransition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RetrieveRequestedAttrs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RetrieveRequestedAttrs: usize,
    pub GetStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppastart: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppaend: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetActiveView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvcview: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAnchorFromPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, ppasite: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAnchorFromPoint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTextExt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcview: u32, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTextExt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetScreenExt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetScreenExt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vcview: u32, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWnd: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub QueryInsertEmbedded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    QueryInsertEmbedded: usize,
    pub InsertTextAtSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pchtext: ::windows::core::PCWSTR, cch: u32, ppastart: *mut *mut ::core::ffi::c_void, ppaend: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbeddedAtSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pdataobject: *mut ::core::ffi::c_void, ppastart: *mut *mut ::core::ffi::c_void, ppaend: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbeddedAtSelection: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITextStoreAnchorEx(::windows::core::IUnknown);
impl ITextStoreAnchorEx {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ScrollToRect<P0, P1>(&self, pstart: P0, pend: P1, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAnchor>,
        P1: ::windows::core::IntoParam<IAnchor>,
    {
        (::windows::core::Interface::vtable(self).ScrollToRect)(::windows::core::Interface::as_raw(self), pstart.into_param().abi(), pend.into_param().abi(), ::core::mem::transmute(rc), dwposition).ok()
    }
}
::windows::imp::interface_hierarchy!(ITextStoreAnchorEx, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITextStoreAnchorEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreAnchorEx {}
impl ::core::fmt::Debug for ITextStoreAnchorEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoreAnchorEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextStoreAnchorEx {
    type Vtable = ITextStoreAnchorEx_Vtbl;
}
impl ::core::clone::Clone for ITextStoreAnchorEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITextStoreAnchorEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2de3bc1_3d8e_11d3_81a9_f753fbe61a00);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreAnchorEx_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ScrollToRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstart: *mut ::core::ffi::c_void, pend: *mut ::core::ffi::c_void, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ScrollToRect: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITextStoreAnchorSink(::windows::core::IUnknown);
impl ITextStoreAnchorSink {
    pub unsafe fn OnTextChange<P0, P1>(&self, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: P0, paend: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAnchor>,
        P1: ::windows::core::IntoParam<IAnchor>,
    {
        (::windows::core::Interface::vtable(self).OnTextChange)(::windows::core::Interface::as_raw(self), dwflags, pastart.into_param().abi(), paend.into_param().abi()).ok()
    }
    pub unsafe fn OnSelectionChange(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSelectionChange)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnLayoutChange)(::windows::core::Interface::as_raw(self), lcode, vcview).ok()
    }
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStatusChange)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn OnAttrsChange<P0, P1>(&self, pastart: P0, paend: P1, paattrs: &[::windows::core::GUID]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAnchor>,
        P1: ::windows::core::IntoParam<IAnchor>,
    {
        (::windows::core::Interface::vtable(self).OnAttrsChange)(::windows::core::Interface::as_raw(self), pastart.into_param().abi(), paend.into_param().abi(), paattrs.len() as _, ::core::mem::transmute(paattrs.as_ptr())).ok()
    }
    pub unsafe fn OnLockGranted(&self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnLockGranted)(::windows::core::Interface::as_raw(self), dwlockflags).ok()
    }
    pub unsafe fn OnStartEditTransaction(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStartEditTransaction)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnEndEditTransaction(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnEndEditTransaction)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ITextStoreAnchorSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITextStoreAnchorSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreAnchorSink {}
impl ::core::fmt::Debug for ITextStoreAnchorSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoreAnchorSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextStoreAnchorSink {
    type Vtable = ITextStoreAnchorSink_Vtbl;
}
impl ::core::clone::Clone for ITextStoreAnchorSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITextStoreAnchorSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e905_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreAnchorSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnTextChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnSelectionChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnLayoutChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::HRESULT,
    pub OnStatusChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub OnAttrsChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, cattrs: u32, paattrs: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnLockGranted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::HRESULT,
    pub OnStartEditTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnEndEditTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITextStoreSinkAnchorEx(::windows::core::IUnknown);
impl ITextStoreSinkAnchorEx {
    pub unsafe fn OnTextChange<P0, P1>(&self, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: P0, paend: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAnchor>,
        P1: ::windows::core::IntoParam<IAnchor>,
    {
        (::windows::core::Interface::vtable(self).base__.OnTextChange)(::windows::core::Interface::as_raw(self), dwflags, pastart.into_param().abi(), paend.into_param().abi()).ok()
    }
    pub unsafe fn OnSelectionChange(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnSelectionChange)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnLayoutChange(&self, lcode: TsLayoutCode, vcview: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnLayoutChange)(::windows::core::Interface::as_raw(self), lcode, vcview).ok()
    }
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnStatusChange)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn OnAttrsChange<P0, P1>(&self, pastart: P0, paend: P1, paattrs: &[::windows::core::GUID]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IAnchor>,
        P1: ::windows::core::IntoParam<IAnchor>,
    {
        (::windows::core::Interface::vtable(self).base__.OnAttrsChange)(::windows::core::Interface::as_raw(self), pastart.into_param().abi(), paend.into_param().abi(), paattrs.len() as _, ::core::mem::transmute(paattrs.as_ptr())).ok()
    }
    pub unsafe fn OnLockGranted(&self, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnLockGranted)(::windows::core::Interface::as_raw(self), dwlockflags).ok()
    }
    pub unsafe fn OnStartEditTransaction(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnStartEditTransaction)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnEndEditTransaction(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnEndEditTransaction)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnDisconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnDisconnect)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ITextStoreSinkAnchorEx, ::windows::core::IUnknown, ITextStoreAnchorSink);
impl ::core::cmp::PartialEq for ITextStoreSinkAnchorEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextStoreSinkAnchorEx {}
impl ::core::fmt::Debug for ITextStoreSinkAnchorEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextStoreSinkAnchorEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITextStoreSinkAnchorEx {
    type Vtable = ITextStoreSinkAnchorEx_Vtbl;
}
impl ::core::clone::Clone for ITextStoreSinkAnchorEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITextStoreSinkAnchorEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25642426_028d_4474_977b_111bb114fe3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextStoreSinkAnchorEx_Vtbl {
    pub base__: ITextStoreAnchorSink_Vtbl,
    pub OnDisconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfActiveLanguageProfileNotifySink(::windows::core::IUnknown);
impl ITfActiveLanguageProfileNotifySink {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnActivated<P0>(&self, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, factivated: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).OnActivated)(::windows::core::Interface::as_raw(self), clsid, guidprofile, factivated.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfActiveLanguageProfileNotifySink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfActiveLanguageProfileNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfActiveLanguageProfileNotifySink {}
impl ::core::fmt::Debug for ITfActiveLanguageProfileNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfActiveLanguageProfileNotifySink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfActiveLanguageProfileNotifySink {
    type Vtable = ITfActiveLanguageProfileNotifySink_Vtbl;
}
impl ::core::clone::Clone for ITfActiveLanguageProfileNotifySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfActiveLanguageProfileNotifySink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb246cb75_a93e_4652_bf8c_b3fe0cfd7e57);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfActiveLanguageProfileNotifySink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, factivated: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnActivated: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfCandidateList(::windows::core::IUnknown);
impl ITfCandidateList {
    pub unsafe fn EnumCandidates(&self) -> ::windows::core::Result<IEnumTfCandidates> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfCandidates>();
        (::windows::core::Interface::vtable(self).EnumCandidates)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCandidate(&self, nindex: u32) -> ::windows::core::Result<ITfCandidateString> {
        let mut result__ = ::windows::core::zeroed::<ITfCandidateString>();
        (::windows::core::Interface::vtable(self).GetCandidate)(::windows::core::Interface::as_raw(self), nindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCandidateNum(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCandidateNum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetResult(&self, nindex: u32, imcr: TfCandidateResult) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetResult)(::windows::core::Interface::as_raw(self), nindex, imcr).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfCandidateList, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfCandidateList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCandidateList {}
impl ::core::fmt::Debug for ITfCandidateList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCandidateList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfCandidateList {
    type Vtable = ITfCandidateList_Vtbl;
}
impl ::core::clone::Clone for ITfCandidateList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfCandidateList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3ad50fb_9bdb_49e3_a843_6c76520fbf5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCandidateList_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EnumCandidates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCandidate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, ppcand: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCandidateNum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pncnt: *mut u32) -> ::windows::core::HRESULT,
    pub SetResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, imcr: TfCandidateResult) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfCandidateListUIElement(::windows::core::IUnknown);
impl ITfCandidateListUIElement {
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDescription)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetGUID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, bshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.Show)(::windows::core::Interface::as_raw(self), bshow.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsShown(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsShown)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetUpdatedFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetUpdatedFlags)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDocumentMgr(&self) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__ = ::windows::core::zeroed::<ITfDocumentMgr>();
        (::windows::core::Interface::vtable(self).GetDocumentMgr)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSelection)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetString(&self, uindex: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetString)(::windows::core::Interface::as_raw(self), uindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPageIndex(&self, pindex: &mut [u32], pupagecnt: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPageIndex)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pindex.as_ptr()), pindex.len() as _, pupagecnt).ok()
    }
    pub unsafe fn SetPageIndex(&self, pindex: &[u32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPageIndex)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pindex.as_ptr()), pindex.len() as _).ok()
    }
    pub unsafe fn GetCurrentPage(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCurrentPage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfCandidateListUIElement, ::windows::core::IUnknown, ITfUIElement);
impl ::core::cmp::PartialEq for ITfCandidateListUIElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCandidateListUIElement {}
impl ::core::fmt::Debug for ITfCandidateListUIElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCandidateListUIElement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfCandidateListUIElement {
    type Vtable = ITfCandidateListUIElement_Vtbl;
}
impl ::core::clone::Clone for ITfCandidateListUIElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfCandidateListUIElement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea1ea138_19df_11d7_a6d2_00065b84435c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCandidateListUIElement_Vtbl {
    pub base__: ITfUIElement_Vtbl,
    pub GetUpdatedFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetDocumentMgr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdim: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows::core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puindex: *mut u32) -> ::windows::core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, pstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetPageIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> ::windows::core::HRESULT,
    pub SetPageIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pindex: *const u32, upagecnt: u32) -> ::windows::core::HRESULT,
    pub GetCurrentPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pupage: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfCandidateListUIElementBehavior(::windows::core::IUnknown);
impl ITfCandidateListUIElementBehavior {
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.GetDescription)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.base__.GetGUID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, bshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.Show)(::windows::core::Interface::as_raw(self), bshow.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsShown(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.base__.IsShown)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetUpdatedFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetUpdatedFlags)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDocumentMgr(&self) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__ = ::windows::core::zeroed::<ITfDocumentMgr>();
        (::windows::core::Interface::vtable(self).base__.GetDocumentMgr)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSelection(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetSelection)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetString(&self, uindex: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetString)(::windows::core::Interface::as_raw(self), uindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPageIndex(&self, pindex: &mut [u32], pupagecnt: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPageIndex)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pindex.as_ptr()), pindex.len() as _, pupagecnt).ok()
    }
    pub unsafe fn SetPageIndex(&self, pindex: &[u32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetPageIndex)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pindex.as_ptr()), pindex.len() as _).ok()
    }
    pub unsafe fn GetCurrentPage(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetCurrentPage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSelection(&self, nindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSelection)(::windows::core::Interface::as_raw(self), nindex).ok()
    }
    pub unsafe fn Finalize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Finalize)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Abort(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Abort)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfCandidateListUIElementBehavior, ::windows::core::IUnknown, ITfUIElement, ITfCandidateListUIElement);
impl ::core::cmp::PartialEq for ITfCandidateListUIElementBehavior {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCandidateListUIElementBehavior {}
impl ::core::fmt::Debug for ITfCandidateListUIElementBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCandidateListUIElementBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfCandidateListUIElementBehavior {
    type Vtable = ITfCandidateListUIElementBehavior_Vtbl;
}
impl ::core::clone::Clone for ITfCandidateListUIElementBehavior {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfCandidateListUIElementBehavior {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85fad185_58ce_497a_9460_355366b64b9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCandidateListUIElementBehavior_Vtbl {
    pub base__: ITfCandidateListUIElement_Vtbl,
    pub SetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32) -> ::windows::core::HRESULT,
    pub Finalize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfCandidateString(::windows::core::IUnknown);
impl ITfCandidateString {
    pub unsafe fn GetString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetString)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetIndex)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfCandidateString, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfCandidateString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCandidateString {}
impl ::core::fmt::Debug for ITfCandidateString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCandidateString").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfCandidateString {
    type Vtable = ITfCandidateString_Vtbl;
}
impl ::core::clone::Clone for ITfCandidateString {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfCandidateString {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x581f317e_fd9d_443f_b972_ed00467c5d40);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCandidateString_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnindex: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfCategoryMgr(::windows::core::IUnknown);
impl ITfCategoryMgr {
    pub unsafe fn RegisterCategory(&self, rclsid: *const ::windows::core::GUID, rcatid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterCategory)(::windows::core::Interface::as_raw(self), rclsid, rcatid, rguid).ok()
    }
    pub unsafe fn UnregisterCategory(&self, rclsid: *const ::windows::core::GUID, rcatid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterCategory)(::windows::core::Interface::as_raw(self), rclsid, rcatid, rguid).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumCategoriesInItem(&self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::System::Com::IEnumGUID> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IEnumGUID>();
        (::windows::core::Interface::vtable(self).EnumCategoriesInItem)(::windows::core::Interface::as_raw(self), rguid, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumItemsInCategory(&self, rcatid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::System::Com::IEnumGUID> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IEnumGUID>();
        (::windows::core::Interface::vtable(self).EnumItemsInCategory)(::windows::core::Interface::as_raw(self), rcatid, &mut result__).from_abi(result__)
    }
    pub unsafe fn FindClosestCategory(&self, rguid: *const ::windows::core::GUID, pcatid: *mut ::windows::core::GUID, ppcatidlist: &[*const ::windows::core::GUID]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FindClosestCategory)(::windows::core::Interface::as_raw(self), rguid, pcatid, ::core::mem::transmute(ppcatidlist.as_ptr()), ppcatidlist.len() as _).ok()
    }
    pub unsafe fn RegisterGUIDDescription(&self, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID, pchdesc: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterGUIDDescription)(::windows::core::Interface::as_raw(self), rclsid, rguid, ::core::mem::transmute(pchdesc.as_ptr()), pchdesc.len() as _).ok()
    }
    pub unsafe fn UnregisterGUIDDescription(&self, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterGUIDDescription)(::windows::core::Interface::as_raw(self), rclsid, rguid).ok()
    }
    pub unsafe fn GetGUIDDescription(&self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetGUIDDescription)(::windows::core::Interface::as_raw(self), rguid, &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterGUIDDWORD(&self, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID, dw: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterGUIDDWORD)(::windows::core::Interface::as_raw(self), rclsid, rguid, dw).ok()
    }
    pub unsafe fn UnregisterGUIDDWORD(&self, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterGUIDDWORD)(::windows::core::Interface::as_raw(self), rclsid, rguid).ok()
    }
    pub unsafe fn GetGUIDDWORD(&self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetGUIDDWORD)(::windows::core::Interface::as_raw(self), rguid, &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterGUID(&self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).RegisterGUID)(::windows::core::Interface::as_raw(self), rguid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self, guidatom: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetGUID)(::windows::core::Interface::as_raw(self), guidatom, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEqualTfGuidAtom(&self, guidatom: u32, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsEqualTfGuidAtom)(::windows::core::Interface::as_raw(self), guidatom, rguid, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfCategoryMgr, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfCategoryMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCategoryMgr {}
impl ::core::fmt::Debug for ITfCategoryMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCategoryMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfCategoryMgr {
    type Vtable = ITfCategoryMgr_Vtbl;
}
impl ::core::clone::Clone for ITfCategoryMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfCategoryMgr {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3acefb5_f69d_4905_938f_fcadcf4be830);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCategoryMgr_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RegisterCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rcatid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub UnregisterCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rcatid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumCategoriesInItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumCategoriesInItem: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumItemsInCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rcatid: *const ::windows::core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumItemsInCategory: usize,
    pub FindClosestCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pcatid: *mut ::windows::core::GUID, ppcatidlist: *const *const ::windows::core::GUID, ulcount: u32) -> ::windows::core::HRESULT,
    pub RegisterGUIDDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID, pchdesc: ::windows::core::PCWSTR, cch: u32) -> ::windows::core::HRESULT,
    pub UnregisterGUIDDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetGUIDDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pbstrdesc: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub RegisterGUIDDWORD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID, dw: u32) -> ::windows::core::HRESULT,
    pub UnregisterGUIDDWORD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetGUIDDWORD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pdw: *mut u32) -> ::windows::core::HRESULT,
    pub RegisterGUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pguidatom: *mut u32) -> ::windows::core::HRESULT,
    pub GetGUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidatom: u32, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEqualTfGuidAtom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidatom: u32, rguid: *const ::windows::core::GUID, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEqualTfGuidAtom: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfCleanupContextDurationSink(::windows::core::IUnknown);
impl ITfCleanupContextDurationSink {
    pub unsafe fn OnStartCleanupContext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStartCleanupContext)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnEndCleanupContext(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnEndCleanupContext)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfCleanupContextDurationSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfCleanupContextDurationSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCleanupContextDurationSink {}
impl ::core::fmt::Debug for ITfCleanupContextDurationSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCleanupContextDurationSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfCleanupContextDurationSink {
    type Vtable = ITfCleanupContextDurationSink_Vtbl;
}
impl ::core::clone::Clone for ITfCleanupContextDurationSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfCleanupContextDurationSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45c35144_154e_4797_bed8_d33ae7bf8794);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCleanupContextDurationSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnStartCleanupContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnEndCleanupContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfCleanupContextSink(::windows::core::IUnknown);
impl ITfCleanupContextSink {
    pub unsafe fn OnCleanupContext<P0>(&self, ecwrite: u32, pic: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfContext>,
    {
        (::windows::core::Interface::vtable(self).OnCleanupContext)(::windows::core::Interface::as_raw(self), ecwrite, pic.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfCleanupContextSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfCleanupContextSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCleanupContextSink {}
impl ::core::fmt::Debug for ITfCleanupContextSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCleanupContextSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfCleanupContextSink {
    type Vtable = ITfCleanupContextSink_Vtbl;
}
impl ::core::clone::Clone for ITfCleanupContextSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfCleanupContextSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01689689_7acb_4e9b_ab7c_7ea46b12b522);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCleanupContextSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnCleanupContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecwrite: u32, pic: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfClientId(::windows::core::IUnknown);
impl ITfClientId {
    pub unsafe fn GetClientId(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetClientId)(::windows::core::Interface::as_raw(self), rclsid, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfClientId, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfClientId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfClientId {}
impl ::core::fmt::Debug for ITfClientId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfClientId").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfClientId {
    type Vtable = ITfClientId_Vtbl;
}
impl ::core::clone::Clone for ITfClientId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfClientId {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd60a7b49_1b9f_4be2_b702_47e9dc05dec3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfClientId_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetClientId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ptid: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfCompartment(::windows::core::IUnknown);
impl ITfCompartment {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue(&self, tid: u32, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValue)(::windows::core::Interface::as_raw(self), tid, pvarvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetValue(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetValue)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfCompartment, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfCompartment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCompartment {}
impl ::core::fmt::Debug for ITfCompartment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCompartment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfCompartment {
    type Vtable = ITfCompartment_Vtbl;
}
impl ::core::clone::Clone for ITfCompartment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfCompartment {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb08f7a9_607a_4384_8623_056892b64371);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCompartment_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tid: u32, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetValue: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfCompartmentEventSink(::windows::core::IUnknown);
impl ITfCompartmentEventSink {
    pub unsafe fn OnChange(&self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnChange)(::windows::core::Interface::as_raw(self), rguid).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfCompartmentEventSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfCompartmentEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCompartmentEventSink {}
impl ::core::fmt::Debug for ITfCompartmentEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCompartmentEventSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfCompartmentEventSink {
    type Vtable = ITfCompartmentEventSink_Vtbl;
}
impl ::core::clone::Clone for ITfCompartmentEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfCompartmentEventSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x743abd5f_f26d_48df_8cc5_238492419b64);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCompartmentEventSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfCompartmentMgr(::windows::core::IUnknown);
impl ITfCompartmentMgr {
    pub unsafe fn GetCompartment(&self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<ITfCompartment> {
        let mut result__ = ::windows::core::zeroed::<ITfCompartment>();
        (::windows::core::Interface::vtable(self).GetCompartment)(::windows::core::Interface::as_raw(self), rguid, &mut result__).from_abi(result__)
    }
    pub unsafe fn ClearCompartment(&self, tid: u32, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearCompartment)(::windows::core::Interface::as_raw(self), tid, rguid).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumCompartments(&self) -> ::windows::core::Result<super::super::System::Com::IEnumGUID> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IEnumGUID>();
        (::windows::core::Interface::vtable(self).EnumCompartments)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfCompartmentMgr, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfCompartmentMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCompartmentMgr {}
impl ::core::fmt::Debug for ITfCompartmentMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCompartmentMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfCompartmentMgr {
    type Vtable = ITfCompartmentMgr_Vtbl;
}
impl ::core::clone::Clone for ITfCompartmentMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfCompartmentMgr {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7dcf57ac_18ad_438b_824d_979bffb74b7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCompartmentMgr_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCompartment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, ppcomp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ClearCompartment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tid: u32, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumCompartments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumCompartments: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfComposition(::windows::core::IUnknown);
impl ITfComposition {
    pub unsafe fn GetRange(&self) -> ::windows::core::Result<ITfRange> {
        let mut result__ = ::windows::core::zeroed::<ITfRange>();
        (::windows::core::Interface::vtable(self).GetRange)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ShiftStart<P0>(&self, ecwrite: u32, pnewstart: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).ShiftStart)(::windows::core::Interface::as_raw(self), ecwrite, pnewstart.into_param().abi()).ok()
    }
    pub unsafe fn ShiftEnd<P0>(&self, ecwrite: u32, pnewend: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).ShiftEnd)(::windows::core::Interface::as_raw(self), ecwrite, pnewend.into_param().abi()).ok()
    }
    pub unsafe fn EndComposition(&self, ecwrite: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndComposition)(::windows::core::Interface::as_raw(self), ecwrite).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfComposition, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfComposition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfComposition {}
impl ::core::fmt::Debug for ITfComposition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfComposition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfComposition {
    type Vtable = ITfComposition_Vtbl;
}
impl ::core::clone::Clone for ITfComposition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfComposition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20168d64_5a8f_4a5a_b7bd_cfa29f4d0fd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfComposition_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprange: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShiftStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecwrite: u32, pnewstart: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShiftEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecwrite: u32, pnewend: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EndComposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecwrite: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfCompositionSink(::windows::core::IUnknown);
impl ITfCompositionSink {
    pub unsafe fn OnCompositionTerminated<P0>(&self, ecwrite: u32, pcomposition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfComposition>,
    {
        (::windows::core::Interface::vtable(self).OnCompositionTerminated)(::windows::core::Interface::as_raw(self), ecwrite, pcomposition.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfCompositionSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfCompositionSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCompositionSink {}
impl ::core::fmt::Debug for ITfCompositionSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCompositionSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfCompositionSink {
    type Vtable = ITfCompositionSink_Vtbl;
}
impl ::core::clone::Clone for ITfCompositionSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfCompositionSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa781718c_579a_4b15_a280_32b8577acc5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCompositionSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnCompositionTerminated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecwrite: u32, pcomposition: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfCompositionView(::windows::core::IUnknown);
impl ITfCompositionView {
    pub unsafe fn GetOwnerClsid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetOwnerClsid)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRange(&self) -> ::windows::core::Result<ITfRange> {
        let mut result__ = ::windows::core::zeroed::<ITfRange>();
        (::windows::core::Interface::vtable(self).GetRange)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfCompositionView, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfCompositionView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCompositionView {}
impl ::core::fmt::Debug for ITfCompositionView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCompositionView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfCompositionView {
    type Vtable = ITfCompositionView_Vtbl;
}
impl ::core::clone::Clone for ITfCompositionView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfCompositionView {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7540241_f9a1_4364_befc_dbcd2c4395b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCompositionView_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetOwnerClsid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprange: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfConfigureSystemKeystrokeFeed(::windows::core::IUnknown);
impl ITfConfigureSystemKeystrokeFeed {
    pub unsafe fn DisableSystemKeystrokeFeed(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisableSystemKeystrokeFeed)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnableSystemKeystrokeFeed(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnableSystemKeystrokeFeed)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfConfigureSystemKeystrokeFeed, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfConfigureSystemKeystrokeFeed {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfConfigureSystemKeystrokeFeed {}
impl ::core::fmt::Debug for ITfConfigureSystemKeystrokeFeed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfConfigureSystemKeystrokeFeed").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfConfigureSystemKeystrokeFeed {
    type Vtable = ITfConfigureSystemKeystrokeFeed_Vtbl;
}
impl ::core::clone::Clone for ITfConfigureSystemKeystrokeFeed {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfConfigureSystemKeystrokeFeed {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d2c969a_bc9c_437c_84ee_951c49b1a764);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfConfigureSystemKeystrokeFeed_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub DisableSystemKeystrokeFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnableSystemKeystrokeFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfContext(::windows::core::IUnknown);
impl ITfContext {
    pub unsafe fn RequestEditSession<P0>(&self, tid: u32, pes: P0, dwflags: TF_CONTEXT_EDIT_CONTEXT_FLAGS) -> ::windows::core::Result<::windows::core::HRESULT>
    where
        P0: ::windows::core::IntoParam<ITfEditSession>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
        (::windows::core::Interface::vtable(self).RequestEditSession)(::windows::core::Interface::as_raw(self), tid, pes.into_param().abi(), dwflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InWriteSession(&self, tid: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).InWriteSession)(::windows::core::Interface::as_raw(self), tid, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSelection(&self, ec: u32, ulindex: u32, pselection: &mut [TF_SELECTION], pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSelection)(::windows::core::Interface::as_raw(self), ec, ulindex, pselection.len() as _, ::core::mem::transmute(pselection.as_ptr()), pcfetched).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSelection(&self, ec: u32, pselection: &[TF_SELECTION]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSelection)(::windows::core::Interface::as_raw(self), ec, pselection.len() as _, ::core::mem::transmute(pselection.as_ptr())).ok()
    }
    pub unsafe fn GetStart(&self, ec: u32) -> ::windows::core::Result<ITfRange> {
        let mut result__ = ::windows::core::zeroed::<ITfRange>();
        (::windows::core::Interface::vtable(self).GetStart)(::windows::core::Interface::as_raw(self), ec, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEnd(&self, ec: u32) -> ::windows::core::Result<ITfRange> {
        let mut result__ = ::windows::core::zeroed::<ITfRange>();
        (::windows::core::Interface::vtable(self).GetEnd)(::windows::core::Interface::as_raw(self), ec, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetActiveView(&self) -> ::windows::core::Result<ITfContextView> {
        let mut result__ = ::windows::core::zeroed::<ITfContextView>();
        (::windows::core::Interface::vtable(self).GetActiveView)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumViews(&self) -> ::windows::core::Result<IEnumTfContextViews> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfContextViews>();
        (::windows::core::Interface::vtable(self).EnumViews)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<TS_STATUS> {
        let mut result__ = ::windows::core::zeroed::<TS_STATUS>();
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetProperty(&self, guidprop: *const ::windows::core::GUID) -> ::windows::core::Result<ITfProperty> {
        let mut result__ = ::windows::core::zeroed::<ITfProperty>();
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), guidprop, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAppProperty(&self, guidprop: *const ::windows::core::GUID) -> ::windows::core::Result<ITfReadOnlyProperty> {
        let mut result__ = ::windows::core::zeroed::<ITfReadOnlyProperty>();
        (::windows::core::Interface::vtable(self).GetAppProperty)(::windows::core::Interface::as_raw(self), guidprop, &mut result__).from_abi(result__)
    }
    pub unsafe fn TrackProperties(&self, prgprop: &[*const ::windows::core::GUID], prgappprop: &[*const ::windows::core::GUID]) -> ::windows::core::Result<ITfReadOnlyProperty> {
        let mut result__ = ::windows::core::zeroed::<ITfReadOnlyProperty>();
        (::windows::core::Interface::vtable(self).TrackProperties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prgprop.as_ptr()), prgprop.len() as _, ::core::mem::transmute(prgappprop.as_ptr()), prgappprop.len() as _, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumProperties(&self) -> ::windows::core::Result<IEnumTfProperties> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfProperties>();
        (::windows::core::Interface::vtable(self).EnumProperties)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDocumentMgr(&self) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__ = ::windows::core::zeroed::<ITfDocumentMgr>();
        (::windows::core::Interface::vtable(self).GetDocumentMgr)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateRangeBackup<P0>(&self, ec: u32, prange: P0) -> ::windows::core::Result<ITfRangeBackup>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        let mut result__ = ::windows::core::zeroed::<ITfRangeBackup>();
        (::windows::core::Interface::vtable(self).CreateRangeBackup)(::windows::core::Interface::as_raw(self), ec, prange.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfContext, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContext {}
impl ::core::fmt::Debug for ITfContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfContext {
    type Vtable = ITfContext_Vtbl;
}
impl ::core::clone::Clone for ITfContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e7fd_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContext_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RequestEditSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tid: u32, pes: *mut ::core::ffi::c_void, dwflags: TF_CONTEXT_EDIT_CONTEXT_FLAGS, phrsession: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InWriteSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tid: u32, pfwritesession: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InWriteSession: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, ulindex: u32, ulcount: u32, pselection: *mut TF_SELECTION, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSelection: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, ulcount: u32, pselection: *const TF_SELECTION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSelection: usize,
    pub GetStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, ppstart: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, ppend: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetActiveView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumViews: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidprop: *const ::windows::core::GUID, ppprop: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAppProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidprop: *const ::windows::core::GUID, ppprop: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TrackProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prgprop: *const *const ::windows::core::GUID, cprop: u32, prgappprop: *const *const ::windows::core::GUID, cappprop: u32, ppproperty: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDocumentMgr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdm: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateRangeBackup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, ppbackup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfContextComposition(::windows::core::IUnknown);
impl ITfContextComposition {
    pub unsafe fn StartComposition<P0, P1>(&self, ecwrite: u32, pcompositionrange: P0, psink: P1) -> ::windows::core::Result<ITfComposition>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
        P1: ::windows::core::IntoParam<ITfCompositionSink>,
    {
        let mut result__ = ::windows::core::zeroed::<ITfComposition>();
        (::windows::core::Interface::vtable(self).StartComposition)(::windows::core::Interface::as_raw(self), ecwrite, pcompositionrange.into_param().abi(), psink.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumCompositions(&self) -> ::windows::core::Result<IEnumITfCompositionView> {
        let mut result__ = ::windows::core::zeroed::<IEnumITfCompositionView>();
        (::windows::core::Interface::vtable(self).EnumCompositions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FindComposition<P0>(&self, ecread: u32, ptestrange: P0) -> ::windows::core::Result<IEnumITfCompositionView>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        let mut result__ = ::windows::core::zeroed::<IEnumITfCompositionView>();
        (::windows::core::Interface::vtable(self).FindComposition)(::windows::core::Interface::as_raw(self), ecread, ptestrange.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn TakeOwnership<P0, P1>(&self, ecwrite: u32, pcomposition: P0, psink: P1) -> ::windows::core::Result<ITfComposition>
    where
        P0: ::windows::core::IntoParam<ITfCompositionView>,
        P1: ::windows::core::IntoParam<ITfCompositionSink>,
    {
        let mut result__ = ::windows::core::zeroed::<ITfComposition>();
        (::windows::core::Interface::vtable(self).TakeOwnership)(::windows::core::Interface::as_raw(self), ecwrite, pcomposition.into_param().abi(), psink.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfContextComposition, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfContextComposition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContextComposition {}
impl ::core::fmt::Debug for ITfContextComposition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfContextComposition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfContextComposition {
    type Vtable = ITfContextComposition_Vtbl;
}
impl ::core::clone::Clone for ITfContextComposition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfContextComposition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd40c8aae_ac92_4fc7_9a11_0ee0e23aa39b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextComposition_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub StartComposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecwrite: u32, pcompositionrange: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, ppcomposition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumCompositions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindComposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecread: u32, ptestrange: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TakeOwnership: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecwrite: u32, pcomposition: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, ppcomposition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfContextKeyEventSink(::windows::core::IUnknown);
impl ITfContextKeyEventSink {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnKeyDown<P0, P1>(&self, wparam: P0, lparam: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).OnKeyDown)(::windows::core::Interface::as_raw(self), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnKeyUp<P0, P1>(&self, wparam: P0, lparam: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).OnKeyUp)(::windows::core::Interface::as_raw(self), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTestKeyDown<P0, P1>(&self, wparam: P0, lparam: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).OnTestKeyDown)(::windows::core::Interface::as_raw(self), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTestKeyUp<P0, P1>(&self, wparam: P0, lparam: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).OnTestKeyUp)(::windows::core::Interface::as_raw(self), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfContextKeyEventSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfContextKeyEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContextKeyEventSink {}
impl ::core::fmt::Debug for ITfContextKeyEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfContextKeyEventSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfContextKeyEventSink {
    type Vtable = ITfContextKeyEventSink_Vtbl;
}
impl ::core::clone::Clone for ITfContextKeyEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfContextKeyEventSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0552ba5d_c835_4934_bf50_846aaa67432f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextKeyEventSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnKeyDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnKeyDown: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnKeyUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnKeyUp: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTestKeyDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTestKeyDown: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTestKeyUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTestKeyUp: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfContextOwner(::windows::core::IUnknown);
impl ITfContextOwner {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetACPFromPoint(&self, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).GetACPFromPoint)(::windows::core::Interface::as_raw(self), ptscreen, dwflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTextExt(&self, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTextExt)(::windows::core::Interface::as_raw(self), acpstart, acpend, prc, pfclipped).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetScreenExt(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::RECT>();
        (::windows::core::Interface::vtable(self).GetScreenExt)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<TS_STATUS> {
        let mut result__ = ::windows::core::zeroed::<TS_STATUS>();
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWnd(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).GetWnd)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAttribute(&self, rguidattribute: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetAttribute)(::windows::core::Interface::as_raw(self), rguidattribute, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfContextOwner, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfContextOwner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContextOwner {}
impl ::core::fmt::Debug for ITfContextOwner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfContextOwner").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfContextOwner {
    type Vtable = ITfContextOwner_Vtbl;
}
impl ::core::clone::Clone for ITfContextOwner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfContextOwner {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e80c_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextOwner_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetACPFromPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetACPFromPoint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTextExt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTextExt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetScreenExt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetScreenExt: usize,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWnd: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidattribute: *const ::windows::core::GUID, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetAttribute: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfContextOwnerCompositionServices(::windows::core::IUnknown);
impl ITfContextOwnerCompositionServices {
    pub unsafe fn StartComposition<P0, P1>(&self, ecwrite: u32, pcompositionrange: P0, psink: P1) -> ::windows::core::Result<ITfComposition>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
        P1: ::windows::core::IntoParam<ITfCompositionSink>,
    {
        let mut result__ = ::windows::core::zeroed::<ITfComposition>();
        (::windows::core::Interface::vtable(self).base__.StartComposition)(::windows::core::Interface::as_raw(self), ecwrite, pcompositionrange.into_param().abi(), psink.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumCompositions(&self) -> ::windows::core::Result<IEnumITfCompositionView> {
        let mut result__ = ::windows::core::zeroed::<IEnumITfCompositionView>();
        (::windows::core::Interface::vtable(self).base__.EnumCompositions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FindComposition<P0>(&self, ecread: u32, ptestrange: P0) -> ::windows::core::Result<IEnumITfCompositionView>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        let mut result__ = ::windows::core::zeroed::<IEnumITfCompositionView>();
        (::windows::core::Interface::vtable(self).base__.FindComposition)(::windows::core::Interface::as_raw(self), ecread, ptestrange.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn TakeOwnership<P0, P1>(&self, ecwrite: u32, pcomposition: P0, psink: P1) -> ::windows::core::Result<ITfComposition>
    where
        P0: ::windows::core::IntoParam<ITfCompositionView>,
        P1: ::windows::core::IntoParam<ITfCompositionSink>,
    {
        let mut result__ = ::windows::core::zeroed::<ITfComposition>();
        (::windows::core::Interface::vtable(self).base__.TakeOwnership)(::windows::core::Interface::as_raw(self), ecwrite, pcomposition.into_param().abi(), psink.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn TerminateComposition<P0>(&self, pcomposition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfCompositionView>,
    {
        (::windows::core::Interface::vtable(self).TerminateComposition)(::windows::core::Interface::as_raw(self), pcomposition.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfContextOwnerCompositionServices, ::windows::core::IUnknown, ITfContextComposition);
impl ::core::cmp::PartialEq for ITfContextOwnerCompositionServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContextOwnerCompositionServices {}
impl ::core::fmt::Debug for ITfContextOwnerCompositionServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfContextOwnerCompositionServices").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfContextOwnerCompositionServices {
    type Vtable = ITfContextOwnerCompositionServices_Vtbl;
}
impl ::core::clone::Clone for ITfContextOwnerCompositionServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfContextOwnerCompositionServices {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86462810_593b_4916_9764_19c08e9ce110);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextOwnerCompositionServices_Vtbl {
    pub base__: ITfContextComposition_Vtbl,
    pub TerminateComposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcomposition: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfContextOwnerCompositionSink(::windows::core::IUnknown);
impl ITfContextOwnerCompositionSink {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnStartComposition<P0>(&self, pcomposition: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<ITfCompositionView>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).OnStartComposition)(::windows::core::Interface::as_raw(self), pcomposition.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn OnUpdateComposition<P0, P1>(&self, pcomposition: P0, prangenew: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfCompositionView>,
        P1: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).OnUpdateComposition)(::windows::core::Interface::as_raw(self), pcomposition.into_param().abi(), prangenew.into_param().abi()).ok()
    }
    pub unsafe fn OnEndComposition<P0>(&self, pcomposition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfCompositionView>,
    {
        (::windows::core::Interface::vtable(self).OnEndComposition)(::windows::core::Interface::as_raw(self), pcomposition.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfContextOwnerCompositionSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfContextOwnerCompositionSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContextOwnerCompositionSink {}
impl ::core::fmt::Debug for ITfContextOwnerCompositionSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfContextOwnerCompositionSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfContextOwnerCompositionSink {
    type Vtable = ITfContextOwnerCompositionSink_Vtbl;
}
impl ::core::clone::Clone for ITfContextOwnerCompositionSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfContextOwnerCompositionSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f20aa40_b57a_4f34_96ab_3576f377cc79);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextOwnerCompositionSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnStartComposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcomposition: *mut ::core::ffi::c_void, pfok: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnStartComposition: usize,
    pub OnUpdateComposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcomposition: *mut ::core::ffi::c_void, prangenew: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnEndComposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcomposition: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfContextOwnerServices(::windows::core::IUnknown);
impl ITfContextOwnerServices {
    pub unsafe fn OnLayoutChange(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnLayoutChange)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnStatusChange(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStatusChange)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn OnAttributeChange(&self, rguidattribute: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnAttributeChange)(::windows::core::Interface::as_raw(self), rguidattribute).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Serialize<P0, P1, P2>(&self, pprop: P0, prange: P1, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfProperty>,
        P1: ::windows::core::IntoParam<ITfRange>,
        P2: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).Serialize)(::windows::core::Interface::as_raw(self), pprop.into_param().abi(), prange.into_param().abi(), phdr, pstream.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Unserialize<P0, P1, P2>(&self, pprop: P0, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: P1, ploader: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfProperty>,
        P1: ::windows::core::IntoParam<super::super::System::Com::IStream>,
        P2: ::windows::core::IntoParam<ITfPersistentPropertyLoaderACP>,
    {
        (::windows::core::Interface::vtable(self).Unserialize)(::windows::core::Interface::as_raw(self), pprop.into_param().abi(), phdr, pstream.into_param().abi(), ploader.into_param().abi()).ok()
    }
    pub unsafe fn ForceLoadProperty<P0>(&self, pprop: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfProperty>,
    {
        (::windows::core::Interface::vtable(self).ForceLoadProperty)(::windows::core::Interface::as_raw(self), pprop.into_param().abi()).ok()
    }
    pub unsafe fn CreateRange(&self, acpstart: i32, acpend: i32) -> ::windows::core::Result<ITfRangeACP> {
        let mut result__ = ::windows::core::zeroed::<ITfRangeACP>();
        (::windows::core::Interface::vtable(self).CreateRange)(::windows::core::Interface::as_raw(self), acpstart, acpend, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfContextOwnerServices, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfContextOwnerServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContextOwnerServices {}
impl ::core::fmt::Debug for ITfContextOwnerServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfContextOwnerServices").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfContextOwnerServices {
    type Vtable = ITfContextOwnerServices_Vtbl;
}
impl ::core::clone::Clone for ITfContextOwnerServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfContextOwnerServices {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb23eb630_3e1c_11d3_a745_0050040ab407);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextOwnerServices_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnLayoutChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnStatusChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub OnAttributeChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidattribute: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprop: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Serialize: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Unserialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprop: *mut ::core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut ::core::ffi::c_void, ploader: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Unserialize: usize,
    pub ForceLoadProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprop: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfContextView(::windows::core::IUnknown);
impl ITfContextView {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRangeFromPoint(&self, ec: u32, ppt: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows::core::Result<ITfRange> {
        let mut result__ = ::windows::core::zeroed::<ITfRange>();
        (::windows::core::Interface::vtable(self).GetRangeFromPoint)(::windows::core::Interface::as_raw(self), ec, ppt, dwflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTextExt<P0>(&self, ec: u32, prange: P0, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).GetTextExt)(::windows::core::Interface::as_raw(self), ec, prange.into_param().abi(), prc, pfclipped).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetScreenExt(&self) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::RECT>();
        (::windows::core::Interface::vtable(self).GetScreenExt)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWnd(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).GetWnd)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfContextView, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfContextView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfContextView {}
impl ::core::fmt::Debug for ITfContextView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfContextView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfContextView {
    type Vtable = ITfContextView_Vtbl;
}
impl ::core::clone::Clone for ITfContextView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfContextView {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2433bf8e_0f9b_435c_ba2c_180611978c30);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfContextView_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRangeFromPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, ppt: *const super::super::Foundation::POINT, dwflags: u32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRangeFromPoint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTextExt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTextExt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetScreenExt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetScreenExt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWnd: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfCreatePropertyStore(::windows::core::IUnknown);
impl ITfCreatePropertyStore {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsStoreSerializable<P0, P1>(&self, guidprop: *const ::windows::core::GUID, prange: P0, ppropstore: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
        P1: ::windows::core::IntoParam<ITfPropertyStore>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsStoreSerializable)(::windows::core::Interface::as_raw(self), guidprop, prange.into_param().abi(), ppropstore.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertyStore<P0, P1>(&self, guidprop: *const ::windows::core::GUID, prange: P0, cb: u32, pstream: P1) -> ::windows::core::Result<ITfPropertyStore>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
        P1: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        let mut result__ = ::windows::core::zeroed::<ITfPropertyStore>();
        (::windows::core::Interface::vtable(self).CreatePropertyStore)(::windows::core::Interface::as_raw(self), guidprop, prange.into_param().abi(), cb, pstream.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfCreatePropertyStore, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfCreatePropertyStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfCreatePropertyStore {}
impl ::core::fmt::Debug for ITfCreatePropertyStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfCreatePropertyStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfCreatePropertyStore {
    type Vtable = ITfCreatePropertyStore_Vtbl;
}
impl ::core::clone::Clone for ITfCreatePropertyStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfCreatePropertyStore {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2463fbf0_b0af_11d2_afc5_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfCreatePropertyStore_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsStoreSerializable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidprop: *const ::windows::core::GUID, prange: *mut ::core::ffi::c_void, ppropstore: *mut ::core::ffi::c_void, pfserializable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsStoreSerializable: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePropertyStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidprop: *const ::windows::core::GUID, prange: *mut ::core::ffi::c_void, cb: u32, pstream: *mut ::core::ffi::c_void, ppstore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePropertyStore: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfDisplayAttributeInfo(::windows::core::IUnknown);
impl ITfDisplayAttributeInfo {
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetGUID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetDescription)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributeInfo(&self, pda: *mut TF_DISPLAYATTRIBUTE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttributeInfo)(::windows::core::Interface::as_raw(self), pda).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAttributeInfo(&self, pda: *const TF_DISPLAYATTRIBUTE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAttributeInfo)(::windows::core::Interface::as_raw(self), pda).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfDisplayAttributeInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfDisplayAttributeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfDisplayAttributeInfo {}
impl ::core::fmt::Debug for ITfDisplayAttributeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfDisplayAttributeInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfDisplayAttributeInfo {
    type Vtable = ITfDisplayAttributeInfo_Vtbl;
}
impl ::core::clone::Clone for ITfDisplayAttributeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfDisplayAttributeInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70528852_2f26_4aea_8c96_215150578932);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfDisplayAttributeInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetGUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdesc: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAttributeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pda: *mut TF_DISPLAYATTRIBUTE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAttributeInfo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAttributeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pda: *const TF_DISPLAYATTRIBUTE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAttributeInfo: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfDisplayAttributeMgr(::windows::core::IUnknown);
impl ITfDisplayAttributeMgr {
    pub unsafe fn OnUpdateInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnUpdateInfo)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn EnumDisplayAttributeInfo(&self) -> ::windows::core::Result<IEnumTfDisplayAttributeInfo> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfDisplayAttributeInfo>();
        (::windows::core::Interface::vtable(self).EnumDisplayAttributeInfo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayAttributeInfo(&self, guid: *const ::windows::core::GUID, ppinfo: *mut ::core::option::Option<ITfDisplayAttributeInfo>, pclsidowner: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplayAttributeInfo)(::windows::core::Interface::as_raw(self), guid, ::core::mem::transmute(ppinfo), pclsidowner).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfDisplayAttributeMgr, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfDisplayAttributeMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfDisplayAttributeMgr {}
impl ::core::fmt::Debug for ITfDisplayAttributeMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfDisplayAttributeMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfDisplayAttributeMgr {
    type Vtable = ITfDisplayAttributeMgr_Vtbl;
}
impl ::core::clone::Clone for ITfDisplayAttributeMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfDisplayAttributeMgr {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ded7393_5db1_475c_9e71_a39111b0ff67);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfDisplayAttributeMgr_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnUpdateInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumDisplayAttributeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDisplayAttributeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, ppinfo: *mut *mut ::core::ffi::c_void, pclsidowner: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfDisplayAttributeNotifySink(::windows::core::IUnknown);
impl ITfDisplayAttributeNotifySink {
    pub unsafe fn OnUpdateInfo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnUpdateInfo)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfDisplayAttributeNotifySink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfDisplayAttributeNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfDisplayAttributeNotifySink {}
impl ::core::fmt::Debug for ITfDisplayAttributeNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfDisplayAttributeNotifySink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfDisplayAttributeNotifySink {
    type Vtable = ITfDisplayAttributeNotifySink_Vtbl;
}
impl ::core::clone::Clone for ITfDisplayAttributeNotifySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfDisplayAttributeNotifySink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad56f402_e162_4f25_908f_7d577cf9bda9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfDisplayAttributeNotifySink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnUpdateInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfDisplayAttributeProvider(::windows::core::IUnknown);
impl ITfDisplayAttributeProvider {
    pub unsafe fn EnumDisplayAttributeInfo(&self) -> ::windows::core::Result<IEnumTfDisplayAttributeInfo> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfDisplayAttributeInfo>();
        (::windows::core::Interface::vtable(self).EnumDisplayAttributeInfo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDisplayAttributeInfo(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<ITfDisplayAttributeInfo> {
        let mut result__ = ::windows::core::zeroed::<ITfDisplayAttributeInfo>();
        (::windows::core::Interface::vtable(self).GetDisplayAttributeInfo)(::windows::core::Interface::as_raw(self), guid, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfDisplayAttributeProvider, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfDisplayAttributeProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfDisplayAttributeProvider {}
impl ::core::fmt::Debug for ITfDisplayAttributeProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfDisplayAttributeProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfDisplayAttributeProvider {
    type Vtable = ITfDisplayAttributeProvider_Vtbl;
}
impl ::core::clone::Clone for ITfDisplayAttributeProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfDisplayAttributeProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfee47777_163c_4769_996a_6e9c50ad8f54);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfDisplayAttributeProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EnumDisplayAttributeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDisplayAttributeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, ppinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfDocumentMgr(::windows::core::IUnknown);
impl ITfDocumentMgr {
    pub unsafe fn CreateContext<P0>(&self, tidowner: u32, dwflags: u32, punk: P0, ppic: *mut ::core::option::Option<ITfContext>, pectextstore: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).CreateContext)(::windows::core::Interface::as_raw(self), tidowner, dwflags, punk.into_param().abi(), ::core::mem::transmute(ppic), pectextstore).ok()
    }
    pub unsafe fn Push<P0>(&self, pic: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfContext>,
    {
        (::windows::core::Interface::vtable(self).Push)(::windows::core::Interface::as_raw(self), pic.into_param().abi()).ok()
    }
    pub unsafe fn Pop(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Pop)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetTop(&self) -> ::windows::core::Result<ITfContext> {
        let mut result__ = ::windows::core::zeroed::<ITfContext>();
        (::windows::core::Interface::vtable(self).GetTop)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBase(&self) -> ::windows::core::Result<ITfContext> {
        let mut result__ = ::windows::core::zeroed::<ITfContext>();
        (::windows::core::Interface::vtable(self).GetBase)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumContexts(&self) -> ::windows::core::Result<IEnumTfContexts> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfContexts>();
        (::windows::core::Interface::vtable(self).EnumContexts)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfDocumentMgr, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfDocumentMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfDocumentMgr {}
impl ::core::fmt::Debug for ITfDocumentMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfDocumentMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfDocumentMgr {
    type Vtable = ITfDocumentMgr_Vtbl;
}
impl ::core::clone::Clone for ITfDocumentMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfDocumentMgr {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e7f4_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfDocumentMgr_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tidowner: u32, dwflags: u32, punk: *mut ::core::ffi::c_void, ppic: *mut *mut ::core::ffi::c_void, pectextstore: *mut u32) -> ::windows::core::HRESULT,
    pub Push: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Pop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub GetTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppic: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetBase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppic: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumContexts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfEditRecord(::windows::core::IUnknown);
impl ITfEditRecord {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSelectionStatus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetSelectionStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTextAndPropertyUpdates(&self, dwflags: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS, prgproperties: &[*const ::windows::core::GUID]) -> ::windows::core::Result<IEnumTfRanges> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfRanges>();
        (::windows::core::Interface::vtable(self).GetTextAndPropertyUpdates)(::windows::core::Interface::as_raw(self), dwflags, ::core::mem::transmute(prgproperties.as_ptr()), prgproperties.len() as _, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfEditRecord, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfEditRecord {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfEditRecord {}
impl ::core::fmt::Debug for ITfEditRecord {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfEditRecord").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfEditRecord {
    type Vtable = ITfEditRecord_Vtbl;
}
impl ::core::clone::Clone for ITfEditRecord {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfEditRecord {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42d4d099_7c1a_4a89_b836_6c6f22160df0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfEditRecord_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSelectionStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfchanged: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSelectionStatus: usize,
    pub GetTextAndPropertyUpdates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS, prgproperties: *const *const ::windows::core::GUID, cproperties: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfEditSession(::windows::core::IUnknown);
impl ITfEditSession {
    pub unsafe fn DoEditSession(&self, ec: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DoEditSession)(::windows::core::Interface::as_raw(self), ec).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfEditSession, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfEditSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfEditSession {}
impl ::core::fmt::Debug for ITfEditSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfEditSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfEditSession {
    type Vtable = ITfEditSession_Vtbl;
}
impl ::core::clone::Clone for ITfEditSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfEditSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e803_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfEditSession_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub DoEditSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfEditTransactionSink(::windows::core::IUnknown);
impl ITfEditTransactionSink {
    pub unsafe fn OnStartEditTransaction<P0>(&self, pic: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfContext>,
    {
        (::windows::core::Interface::vtable(self).OnStartEditTransaction)(::windows::core::Interface::as_raw(self), pic.into_param().abi()).ok()
    }
    pub unsafe fn OnEndEditTransaction<P0>(&self, pic: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfContext>,
    {
        (::windows::core::Interface::vtable(self).OnEndEditTransaction)(::windows::core::Interface::as_raw(self), pic.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfEditTransactionSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfEditTransactionSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfEditTransactionSink {}
impl ::core::fmt::Debug for ITfEditTransactionSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfEditTransactionSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfEditTransactionSink {
    type Vtable = ITfEditTransactionSink_Vtbl;
}
impl ::core::clone::Clone for ITfEditTransactionSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfEditTransactionSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x708fbf70_b520_416b_b06c_2c41ab44f8ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfEditTransactionSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnStartEditTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnEndEditTransaction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfFnAdviseText(::windows::core::IUnknown);
impl ITfFnAdviseText {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OnTextUpdate<P0>(&self, prange: P0, pchtext: &[u16]) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).OnTextUpdate)(::windows::core::Interface::as_raw(self), prange.into_param().abi(), ::core::mem::transmute(pchtext.as_ptr()), pchtext.len() as _).ok()
    }
    pub unsafe fn OnLatticeUpdate<P0, P1>(&self, prange: P0, plattice: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
        P1: ::windows::core::IntoParam<ITfLMLattice>,
    {
        (::windows::core::Interface::vtable(self).OnLatticeUpdate)(::windows::core::Interface::as_raw(self), prange.into_param().abi(), plattice.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfFnAdviseText, ::windows::core::IUnknown, ITfFunction);
impl ::core::cmp::PartialEq for ITfFnAdviseText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnAdviseText {}
impl ::core::fmt::Debug for ITfFnAdviseText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnAdviseText").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfFnAdviseText {
    type Vtable = ITfFnAdviseText_Vtbl;
}
impl ::core::clone::Clone for ITfFnAdviseText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfFnAdviseText {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3527268b_7d53_4dd9_92b7_7296ae461249);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnAdviseText_Vtbl {
    pub base__: ITfFunction_Vtbl,
    pub OnTextUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, pchtext: ::windows::core::PCWSTR, cch: i32) -> ::windows::core::HRESULT,
    pub OnLatticeUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, plattice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfFnBalloon(::windows::core::IUnknown);
impl ITfFnBalloon {
    pub unsafe fn UpdateBalloon(&self, style: TfLBBalloonStyle, pch: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateBalloon)(::windows::core::Interface::as_raw(self), style, ::core::mem::transmute(pch.as_ptr()), pch.len() as _).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfFnBalloon, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfFnBalloon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnBalloon {}
impl ::core::fmt::Debug for ITfFnBalloon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnBalloon").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfFnBalloon {
    type Vtable = ITfFnBalloon_Vtbl;
}
impl ::core::clone::Clone for ITfFnBalloon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfFnBalloon {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bab89e4_5fbe_45f4_a5bc_dca36ad225a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnBalloon_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub UpdateBalloon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, style: TfLBBalloonStyle, pch: ::windows::core::PCWSTR, cch: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfFnConfigure(::windows::core::IUnknown);
impl ITfFnConfigure {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, hwndparent: P0, langid: u16, rguidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).Show)(::windows::core::Interface::as_raw(self), hwndparent.into_param().abi(), langid, rguidprofile).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfFnConfigure, ::windows::core::IUnknown, ITfFunction);
impl ::core::cmp::PartialEq for ITfFnConfigure {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnConfigure {}
impl ::core::fmt::Debug for ITfFnConfigure {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnConfigure").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfFnConfigure {
    type Vtable = ITfFnConfigure_Vtbl;
}
impl ::core::clone::Clone for ITfFnConfigure {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfFnConfigure {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88f567c6_1757_49f8_a1b2_89234c1eeff9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnConfigure_Vtbl {
    pub base__: ITfFunction_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Show: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfFnConfigureRegisterEudc(::windows::core::IUnknown);
impl ITfFnConfigureRegisterEudc {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0, P1>(&self, hwndparent: P0, langid: u16, rguidprofile: *const ::windows::core::GUID, bstrregistered: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Show)(::windows::core::Interface::as_raw(self), hwndparent.into_param().abi(), langid, rguidprofile, bstrregistered.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfFnConfigureRegisterEudc, ::windows::core::IUnknown, ITfFunction);
impl ::core::cmp::PartialEq for ITfFnConfigureRegisterEudc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnConfigureRegisterEudc {}
impl ::core::fmt::Debug for ITfFnConfigureRegisterEudc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnConfigureRegisterEudc").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfFnConfigureRegisterEudc {
    type Vtable = ITfFnConfigureRegisterEudc_Vtbl;
}
impl ::core::clone::Clone for ITfFnConfigureRegisterEudc {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfFnConfigureRegisterEudc {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5e26ff5_d7ad_4304_913f_21a2ed95a1b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnConfigureRegisterEudc_Vtbl {
    pub base__: ITfFunction_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::core::GUID, bstrregistered: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Show: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfFnConfigureRegisterWord(::windows::core::IUnknown);
impl ITfFnConfigureRegisterWord {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0, P1>(&self, hwndparent: P0, langid: u16, rguidprofile: *const ::windows::core::GUID, bstrregistered: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Show)(::windows::core::Interface::as_raw(self), hwndparent.into_param().abi(), langid, rguidprofile, bstrregistered.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfFnConfigureRegisterWord, ::windows::core::IUnknown, ITfFunction);
impl ::core::cmp::PartialEq for ITfFnConfigureRegisterWord {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnConfigureRegisterWord {}
impl ::core::fmt::Debug for ITfFnConfigureRegisterWord {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnConfigureRegisterWord").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfFnConfigureRegisterWord {
    type Vtable = ITfFnConfigureRegisterWord_Vtbl;
}
impl ::core::clone::Clone for ITfFnConfigureRegisterWord {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfFnConfigureRegisterWord {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb95808a_6d8f_4bca_8400_5390b586aedf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnConfigureRegisterWord_Vtbl {
    pub base__: ITfFunction_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows::core::GUID, bstrregistered: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Show: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfFnCustomSpeechCommand(::windows::core::IUnknown);
impl ITfFnCustomSpeechCommand {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSpeechCommandProvider<P0>(&self, pspcmdprovider: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).SetSpeechCommandProvider)(::windows::core::Interface::as_raw(self), pspcmdprovider.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfFnCustomSpeechCommand, ::windows::core::IUnknown, ITfFunction);
impl ::core::cmp::PartialEq for ITfFnCustomSpeechCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnCustomSpeechCommand {}
impl ::core::fmt::Debug for ITfFnCustomSpeechCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnCustomSpeechCommand").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfFnCustomSpeechCommand {
    type Vtable = ITfFnCustomSpeechCommand_Vtbl;
}
impl ::core::clone::Clone for ITfFnCustomSpeechCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfFnCustomSpeechCommand {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfca6c349_a12f_43a3_8dd6_5a5a4282577b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnCustomSpeechCommand_Vtbl {
    pub base__: ITfFunction_Vtbl,
    pub SetSpeechCommandProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pspcmdprovider: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfFnGetLinguisticAlternates(::windows::core::IUnknown);
impl ITfFnGetLinguisticAlternates {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAlternates<P0>(&self, prange: P0) -> ::windows::core::Result<ITfCandidateList>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        let mut result__ = ::windows::core::zeroed::<ITfCandidateList>();
        (::windows::core::Interface::vtable(self).GetAlternates)(::windows::core::Interface::as_raw(self), prange.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfFnGetLinguisticAlternates, ::windows::core::IUnknown, ITfFunction);
impl ::core::cmp::PartialEq for ITfFnGetLinguisticAlternates {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnGetLinguisticAlternates {}
impl ::core::fmt::Debug for ITfFnGetLinguisticAlternates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnGetLinguisticAlternates").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfFnGetLinguisticAlternates {
    type Vtable = ITfFnGetLinguisticAlternates_Vtbl;
}
impl ::core::clone::Clone for ITfFnGetLinguisticAlternates {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfFnGetLinguisticAlternates {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea163ce2_7a65_4506_82a3_c528215da64e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnGetLinguisticAlternates_Vtbl {
    pub base__: ITfFunction_Vtbl,
    pub GetAlternates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, ppcandidatelist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfFnGetPreferredTouchKeyboardLayout(::windows::core::IUnknown);
impl ITfFnGetPreferredTouchKeyboardLayout {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLayout(&self, ptkblayouttype: *mut TKBLayoutType, pwpreferredlayoutid: *const u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLayout)(::windows::core::Interface::as_raw(self), ptkblayouttype, pwpreferredlayoutid).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfFnGetPreferredTouchKeyboardLayout, ::windows::core::IUnknown, ITfFunction);
impl ::core::cmp::PartialEq for ITfFnGetPreferredTouchKeyboardLayout {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnGetPreferredTouchKeyboardLayout {}
impl ::core::fmt::Debug for ITfFnGetPreferredTouchKeyboardLayout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnGetPreferredTouchKeyboardLayout").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfFnGetPreferredTouchKeyboardLayout {
    type Vtable = ITfFnGetPreferredTouchKeyboardLayout_Vtbl;
}
impl ::core::clone::Clone for ITfFnGetPreferredTouchKeyboardLayout {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfFnGetPreferredTouchKeyboardLayout {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f309a41_590a_4acc_a97f_d8efff13fdfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnGetPreferredTouchKeyboardLayout_Vtbl {
    pub base__: ITfFunction_Vtbl,
    pub GetLayout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptkblayouttype: *mut TKBLayoutType, pwpreferredlayoutid: *const u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfFnGetSAPIObject(::windows::core::IUnknown);
impl ITfFnGetSAPIObject {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Get(&self, sobj: TfSapiObject) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).Get)(::windows::core::Interface::as_raw(self), sobj, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfFnGetSAPIObject, ::windows::core::IUnknown, ITfFunction);
impl ::core::cmp::PartialEq for ITfFnGetSAPIObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnGetSAPIObject {}
impl ::core::fmt::Debug for ITfFnGetSAPIObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnGetSAPIObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfFnGetSAPIObject {
    type Vtable = ITfFnGetSAPIObject_Vtbl;
}
impl ::core::clone::Clone for ITfFnGetSAPIObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfFnGetSAPIObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c0ab7ea_167d_4f59_bfb5_4693755e90ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnGetSAPIObject_Vtbl {
    pub base__: ITfFunction_Vtbl,
    pub Get: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sobj: TfSapiObject, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfFnLMInternal(::windows::core::IUnknown);
impl ITfFnLMInternal {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryRange<P0>(&self, prange: P0, ppnewrange: *mut ::core::option::Option<ITfRange>, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).base__.QueryRange)(::windows::core::Interface::as_raw(self), prange.into_param().abi(), ::core::mem::transmute(ppnewrange), pfaccepted).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryLangID(&self, langid: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.QueryLangID)(::windows::core::Interface::as_raw(self), langid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetReconversion<P0>(&self, prange: P0) -> ::windows::core::Result<ITfCandidateList>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        let mut result__ = ::windows::core::zeroed::<ITfCandidateList>();
        (::windows::core::Interface::vtable(self).base__.GetReconversion)(::windows::core::Interface::as_raw(self), prange.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Reconvert<P0>(&self, prange: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).base__.Reconvert)(::windows::core::Interface::as_raw(self), prange.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryKey<P0, P1, P2>(&self, fup: P0, vkey: P1, lparamkeydata: P2) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.QueryKey)(::windows::core::Interface::as_raw(self), fup.into_param().abi(), vkey.into_param().abi(), lparamkeydata.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvokeKey<P0, P1, P2>(&self, fup: P0, vkey: P1, lparamkeydata: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).base__.InvokeKey)(::windows::core::Interface::as_raw(self), fup.into_param().abi(), vkey.into_param().abi(), lparamkeydata.into_param().abi()).ok()
    }
    pub unsafe fn InvokeFunc<P0>(&self, pic: P0, refguidfunc: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfContext>,
    {
        (::windows::core::Interface::vtable(self).base__.InvokeFunc)(::windows::core::Interface::as_raw(self), pic.into_param().abi(), refguidfunc).ok()
    }
    pub unsafe fn ProcessLattice<P0>(&self, prange: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).ProcessLattice)(::windows::core::Interface::as_raw(self), prange.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfFnLMInternal, ::windows::core::IUnknown, ITfFunction, ITfFnLMProcessor);
impl ::core::cmp::PartialEq for ITfFnLMInternal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnLMInternal {}
impl ::core::fmt::Debug for ITfFnLMInternal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnLMInternal").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfFnLMInternal {
    type Vtable = ITfFnLMInternal_Vtbl;
}
impl ::core::clone::Clone for ITfFnLMInternal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfFnLMInternal {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04b825b1_ac9a_4f7b_b5ad_c7168f1ee445);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnLMInternal_Vtbl {
    pub base__: ITfFnLMProcessor_Vtbl,
    pub ProcessLattice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfFnLMProcessor(::windows::core::IUnknown);
impl ITfFnLMProcessor {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryRange<P0>(&self, prange: P0, ppnewrange: *mut ::core::option::Option<ITfRange>, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).QueryRange)(::windows::core::Interface::as_raw(self), prange.into_param().abi(), ::core::mem::transmute(ppnewrange), pfaccepted).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryLangID(&self, langid: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).QueryLangID)(::windows::core::Interface::as_raw(self), langid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetReconversion<P0>(&self, prange: P0) -> ::windows::core::Result<ITfCandidateList>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        let mut result__ = ::windows::core::zeroed::<ITfCandidateList>();
        (::windows::core::Interface::vtable(self).GetReconversion)(::windows::core::Interface::as_raw(self), prange.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Reconvert<P0>(&self, prange: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).Reconvert)(::windows::core::Interface::as_raw(self), prange.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryKey<P0, P1, P2>(&self, fup: P0, vkey: P1, lparamkeydata: P2) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).QueryKey)(::windows::core::Interface::as_raw(self), fup.into_param().abi(), vkey.into_param().abi(), lparamkeydata.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvokeKey<P0, P1, P2>(&self, fup: P0, vkey: P1, lparamkeydata: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).InvokeKey)(::windows::core::Interface::as_raw(self), fup.into_param().abi(), vkey.into_param().abi(), lparamkeydata.into_param().abi()).ok()
    }
    pub unsafe fn InvokeFunc<P0>(&self, pic: P0, refguidfunc: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfContext>,
    {
        (::windows::core::Interface::vtable(self).InvokeFunc)(::windows::core::Interface::as_raw(self), pic.into_param().abi(), refguidfunc).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfFnLMProcessor, ::windows::core::IUnknown, ITfFunction);
impl ::core::cmp::PartialEq for ITfFnLMProcessor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnLMProcessor {}
impl ::core::fmt::Debug for ITfFnLMProcessor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnLMProcessor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfFnLMProcessor {
    type Vtable = ITfFnLMProcessor_Vtbl;
}
impl ::core::clone::Clone for ITfFnLMProcessor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfFnLMProcessor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7afbf8e7_ac4b_4082_b058_890899d3a010);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnLMProcessor_Vtbl {
    pub base__: ITfFunction_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, ppnewrange: *mut *mut ::core::ffi::c_void, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryRange: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryLangID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryLangID: usize,
    pub GetReconversion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, ppcandlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reconvert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM, pfinterested: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryKey: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InvokeKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InvokeKey: usize,
    pub InvokeFunc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, refguidfunc: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfFnLangProfileUtil(::windows::core::IUnknown);
impl ITfFnLangProfileUtil {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RegisterActiveProfiles(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterActiveProfiles)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsProfileAvailableForLang(&self, langid: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsProfileAvailableForLang)(::windows::core::Interface::as_raw(self), langid, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfFnLangProfileUtil, ::windows::core::IUnknown, ITfFunction);
impl ::core::cmp::PartialEq for ITfFnLangProfileUtil {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnLangProfileUtil {}
impl ::core::fmt::Debug for ITfFnLangProfileUtil {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnLangProfileUtil").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfFnLangProfileUtil {
    type Vtable = ITfFnLangProfileUtil_Vtbl;
}
impl ::core::clone::Clone for ITfFnLangProfileUtil {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfFnLangProfileUtil {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa87a8574_a6c1_4e15_99f0_3d3965f548eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnLangProfileUtil_Vtbl {
    pub base__: ITfFunction_Vtbl,
    pub RegisterActiveProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsProfileAvailableForLang: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16, pfavailable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsProfileAvailableForLang: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfFnPlayBack(::windows::core::IUnknown);
impl ITfFnPlayBack {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryRange<P0>(&self, prange: P0, ppnewrange: *mut ::core::option::Option<ITfRange>, pfplayable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).QueryRange)(::windows::core::Interface::as_raw(self), prange.into_param().abi(), ::core::mem::transmute(ppnewrange), pfplayable).ok()
    }
    pub unsafe fn Play<P0>(&self, prange: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).Play)(::windows::core::Interface::as_raw(self), prange.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfFnPlayBack, ::windows::core::IUnknown, ITfFunction);
impl ::core::cmp::PartialEq for ITfFnPlayBack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnPlayBack {}
impl ::core::fmt::Debug for ITfFnPlayBack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnPlayBack").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfFnPlayBack {
    type Vtable = ITfFnPlayBack_Vtbl;
}
impl ::core::clone::Clone for ITfFnPlayBack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfFnPlayBack {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3a416a4_0f64_11d3_b5b7_00c04fc324a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnPlayBack_Vtbl {
    pub base__: ITfFunction_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, ppnewrange: *mut *mut ::core::ffi::c_void, pfplayable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryRange: usize,
    pub Play: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfFnPropertyUIStatus(::windows::core::IUnknown);
impl ITfFnPropertyUIStatus {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStatus(&self, refguidprop: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), refguidprop, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStatus(&self, refguidprop: *const ::windows::core::GUID, dw: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStatus)(::windows::core::Interface::as_raw(self), refguidprop, dw).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfFnPropertyUIStatus, ::windows::core::IUnknown, ITfFunction);
impl ::core::cmp::PartialEq for ITfFnPropertyUIStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnPropertyUIStatus {}
impl ::core::fmt::Debug for ITfFnPropertyUIStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnPropertyUIStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfFnPropertyUIStatus {
    type Vtable = ITfFnPropertyUIStatus_Vtbl;
}
impl ::core::clone::Clone for ITfFnPropertyUIStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfFnPropertyUIStatus {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2338ac6e_2b9d_44c0_a75e_ee64f256b3bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnPropertyUIStatus_Vtbl {
    pub base__: ITfFunction_Vtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, refguidprop: *const ::windows::core::GUID, pdw: *mut u32) -> ::windows::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, refguidprop: *const ::windows::core::GUID, dw: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfFnReconversion(::windows::core::IUnknown);
impl ITfFnReconversion {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryRange<P0>(&self, prange: P0, ppnewrange: *mut ::core::option::Option<ITfRange>, pfconvertable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).QueryRange)(::windows::core::Interface::as_raw(self), prange.into_param().abi(), ::core::mem::transmute(ppnewrange), pfconvertable).ok()
    }
    pub unsafe fn GetReconversion<P0>(&self, prange: P0) -> ::windows::core::Result<ITfCandidateList>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        let mut result__ = ::windows::core::zeroed::<ITfCandidateList>();
        (::windows::core::Interface::vtable(self).GetReconversion)(::windows::core::Interface::as_raw(self), prange.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Reconvert<P0>(&self, prange: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).Reconvert)(::windows::core::Interface::as_raw(self), prange.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfFnReconversion, ::windows::core::IUnknown, ITfFunction);
impl ::core::cmp::PartialEq for ITfFnReconversion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnReconversion {}
impl ::core::fmt::Debug for ITfFnReconversion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnReconversion").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfFnReconversion {
    type Vtable = ITfFnReconversion_Vtbl;
}
impl ::core::clone::Clone for ITfFnReconversion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfFnReconversion {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4cea93c0_0a58_11d3_8df0_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnReconversion_Vtbl {
    pub base__: ITfFunction_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, ppnewrange: *mut *mut ::core::ffi::c_void, pfconvertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryRange: usize,
    pub GetReconversion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, ppcandlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reconvert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfFnSearchCandidateProvider(::windows::core::IUnknown);
impl ITfFnSearchCandidateProvider {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSearchCandidates<P0, P1>(&self, bstrquery: P0, bstrapplicationid: P1) -> ::windows::core::Result<ITfCandidateList>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ITfCandidateList>();
        (::windows::core::Interface::vtable(self).GetSearchCandidates)(::windows::core::Interface::as_raw(self), bstrquery.into_param().abi(), bstrapplicationid.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetResult<P0, P1, P2>(&self, bstrquery: P0, bstrapplicationid: P1, bstrresult: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetResult)(::windows::core::Interface::as_raw(self), bstrquery.into_param().abi(), bstrapplicationid.into_param().abi(), bstrresult.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfFnSearchCandidateProvider, ::windows::core::IUnknown, ITfFunction);
impl ::core::cmp::PartialEq for ITfFnSearchCandidateProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnSearchCandidateProvider {}
impl ::core::fmt::Debug for ITfFnSearchCandidateProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnSearchCandidateProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfFnSearchCandidateProvider {
    type Vtable = ITfFnSearchCandidateProvider_Vtbl;
}
impl ::core::clone::Clone for ITfFnSearchCandidateProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfFnSearchCandidateProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87a2ad8f_f27b_4920_8501_67602280175d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnSearchCandidateProvider_Vtbl {
    pub base__: ITfFunction_Vtbl,
    pub GetSearchCandidates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrquery: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrapplicationid: ::std::mem::MaybeUninit<::windows::core::BSTR>, pplist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrquery: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrapplicationid: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrresult: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfFnShowHelp(::windows::core::IUnknown);
impl ITfFnShowHelp {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, hwndparent: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).Show)(::windows::core::Interface::as_raw(self), hwndparent.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfFnShowHelp, ::windows::core::IUnknown, ITfFunction);
impl ::core::cmp::PartialEq for ITfFnShowHelp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFnShowHelp {}
impl ::core::fmt::Debug for ITfFnShowHelp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFnShowHelp").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfFnShowHelp {
    type Vtable = ITfFnShowHelp_Vtbl;
}
impl ::core::clone::Clone for ITfFnShowHelp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfFnShowHelp {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ab1d30c_094d_4c29_8ea5_0bf59be87bf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFnShowHelp_Vtbl {
    pub base__: ITfFunction_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Show: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfFunction(::windows::core::IUnknown);
impl ITfFunction {
    pub unsafe fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetDisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfFunction, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfFunction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFunction {}
impl ::core::fmt::Debug for ITfFunction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFunction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfFunction {
    type Vtable = ITfFunction_Vtbl;
}
impl ::core::clone::Clone for ITfFunction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfFunction {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb593490_098f_11d3_8df0_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFunction_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfFunctionProvider(::windows::core::IUnknown);
impl ITfFunctionProvider {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetDescription)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFunction(&self, rguid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetFunction)(::windows::core::Interface::as_raw(self), rguid, riid, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfFunctionProvider, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfFunctionProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfFunctionProvider {}
impl ::core::fmt::Debug for ITfFunctionProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfFunctionProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfFunctionProvider {
    type Vtable = ITfFunctionProvider_Vtbl;
}
impl ::core::clone::Clone for ITfFunctionProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfFunctionProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x101d6610_0990_11d3_8df0_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfFunctionProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdesc: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetFunction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfInputProcessorProfileActivationSink(::windows::core::IUnknown);
impl ITfInputProcessorProfileActivationSink {
    pub unsafe fn OnActivated<P0>(&self, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, catid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: P0, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<HKL>,
    {
        (::windows::core::Interface::vtable(self).OnActivated)(::windows::core::Interface::as_raw(self), dwprofiletype, langid, clsid, catid, guidprofile, hkl.into_param().abi(), dwflags).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfInputProcessorProfileActivationSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfInputProcessorProfileActivationSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInputProcessorProfileActivationSink {}
impl ::core::fmt::Debug for ITfInputProcessorProfileActivationSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfInputProcessorProfileActivationSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfInputProcessorProfileActivationSink {
    type Vtable = ITfInputProcessorProfileActivationSink_Vtbl;
}
impl ::core::clone::Clone for ITfInputProcessorProfileActivationSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfInputProcessorProfileActivationSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71c6e74e_0f28_11d8_a82a_00065b84435c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputProcessorProfileActivationSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, catid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, dwflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfInputProcessorProfileMgr(::windows::core::IUnknown);
impl ITfInputProcessorProfileMgr {
    pub unsafe fn ActivateProfile<P0>(&self, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: P0, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<HKL>,
    {
        (::windows::core::Interface::vtable(self).ActivateProfile)(::windows::core::Interface::as_raw(self), dwprofiletype, langid, clsid, guidprofile, hkl.into_param().abi(), dwflags).ok()
    }
    pub unsafe fn DeactivateProfile<P0>(&self, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: P0, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<HKL>,
    {
        (::windows::core::Interface::vtable(self).DeactivateProfile)(::windows::core::Interface::as_raw(self), dwprofiletype, langid, clsid, guidprofile, hkl.into_param().abi(), dwflags).ok()
    }
    pub unsafe fn GetProfile<P0>(&self, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: P0, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<HKL>,
    {
        (::windows::core::Interface::vtable(self).GetProfile)(::windows::core::Interface::as_raw(self), dwprofiletype, langid, clsid, guidprofile, hkl.into_param().abi(), pprofile).ok()
    }
    pub unsafe fn EnumProfiles(&self, langid: u16) -> ::windows::core::Result<IEnumTfInputProcessorProfiles> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfInputProcessorProfiles>();
        (::windows::core::Interface::vtable(self).EnumProfiles)(::windows::core::Interface::as_raw(self), langid, &mut result__).from_abi(result__)
    }
    pub unsafe fn ReleaseInputProcessor(&self, rclsid: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseInputProcessor)(::windows::core::Interface::as_raw(self), rclsid, dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterProfile<P0, P1>(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchdesc: &[u16], pchiconfile: &[u16], uiconindex: u32, hklsubstitute: P0, dwpreferredlayout: u32, benabledbydefault: P1, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<HKL>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).RegisterProfile)(::windows::core::Interface::as_raw(self), rclsid, langid, guidprofile, ::core::mem::transmute(pchdesc.as_ptr()), pchdesc.len() as _, ::core::mem::transmute(pchiconfile.as_ptr()), pchiconfile.len() as _, uiconindex, hklsubstitute.into_param().abi(), dwpreferredlayout, benabledbydefault.into_param().abi(), dwflags).ok()
    }
    pub unsafe fn UnregisterProfile(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterProfile)(::windows::core::Interface::as_raw(self), rclsid, langid, guidprofile, dwflags).ok()
    }
    pub unsafe fn GetActiveProfile(&self, catid: *const ::windows::core::GUID, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetActiveProfile)(::windows::core::Interface::as_raw(self), catid, pprofile).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfInputProcessorProfileMgr, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfInputProcessorProfileMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInputProcessorProfileMgr {}
impl ::core::fmt::Debug for ITfInputProcessorProfileMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfInputProcessorProfileMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfInputProcessorProfileMgr {
    type Vtable = ITfInputProcessorProfileMgr_Vtbl;
}
impl ::core::clone::Clone for ITfInputProcessorProfileMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfInputProcessorProfileMgr {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71c6e74c_0f28_11d8_a82a_00065b84435c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputProcessorProfileMgr_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ActivateProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, dwflags: u32) -> ::windows::core::HRESULT,
    pub DeactivateProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, dwflags: u32) -> ::windows::core::HRESULT,
    pub GetProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows::core::GUID, guidprofile: *const ::windows::core::GUID, hkl: HKL, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows::core::HRESULT,
    pub EnumProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReleaseInputProcessor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchdesc: ::windows::core::PCWSTR, cchdesc: u32, pchiconfile: ::windows::core::PCWSTR, cchfile: u32, uiconindex: u32, hklsubstitute: HKL, dwpreferredlayout: u32, benabledbydefault: super::super::Foundation::BOOL, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterProfile: usize,
    pub UnregisterProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, dwflags: u32) -> ::windows::core::HRESULT,
    pub GetActiveProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, catid: *const ::windows::core::GUID, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfInputProcessorProfileSubstituteLayout(::windows::core::IUnknown);
impl ITfInputProcessorProfileSubstituteLayout {
    pub unsafe fn GetSubstituteKeyboardLayout(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<HKL> {
        let mut result__ = ::windows::core::zeroed::<HKL>();
        (::windows::core::Interface::vtable(self).GetSubstituteKeyboardLayout)(::windows::core::Interface::as_raw(self), rclsid, langid, guidprofile, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfInputProcessorProfileSubstituteLayout, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfInputProcessorProfileSubstituteLayout {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInputProcessorProfileSubstituteLayout {}
impl ::core::fmt::Debug for ITfInputProcessorProfileSubstituteLayout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfInputProcessorProfileSubstituteLayout").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfInputProcessorProfileSubstituteLayout {
    type Vtable = ITfInputProcessorProfileSubstituteLayout_Vtbl;
}
impl ::core::clone::Clone for ITfInputProcessorProfileSubstituteLayout {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfInputProcessorProfileSubstituteLayout {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fd67194_1002_4513_bff2_c0ddf6258552);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputProcessorProfileSubstituteLayout_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSubstituteKeyboardLayout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, phkl: *mut HKL) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfInputProcessorProfiles(::windows::core::IUnknown);
impl ITfInputProcessorProfiles {
    pub unsafe fn Register(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Register)(::windows::core::Interface::as_raw(self), rclsid).ok()
    }
    pub unsafe fn Unregister(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unregister)(::windows::core::Interface::as_raw(self), rclsid).ok()
    }
    pub unsafe fn AddLanguageProfile(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchdesc: &[u16], pchiconfile: &[u16], uiconindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddLanguageProfile)(::windows::core::Interface::as_raw(self), rclsid, langid, guidprofile, ::core::mem::transmute(pchdesc.as_ptr()), pchdesc.len() as _, ::core::mem::transmute(pchiconfile.as_ptr()), pchiconfile.len() as _, uiconindex).ok()
    }
    pub unsafe fn RemoveLanguageProfile(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveLanguageProfile)(::windows::core::Interface::as_raw(self), rclsid, langid, guidprofile).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumInputProcessorInfo(&self) -> ::windows::core::Result<super::super::System::Com::IEnumGUID> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IEnumGUID>();
        (::windows::core::Interface::vtable(self).EnumInputProcessorInfo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDefaultLanguageProfile(&self, langid: u16, catid: *const ::windows::core::GUID, pclsid: *mut ::windows::core::GUID, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDefaultLanguageProfile)(::windows::core::Interface::as_raw(self), langid, catid, pclsid, pguidprofile).ok()
    }
    pub unsafe fn SetDefaultLanguageProfile(&self, langid: u16, rclsid: *const ::windows::core::GUID, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDefaultLanguageProfile)(::windows::core::Interface::as_raw(self), langid, rclsid, guidprofiles).ok()
    }
    pub unsafe fn ActivateLanguageProfile(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ActivateLanguageProfile)(::windows::core::Interface::as_raw(self), rclsid, langid, guidprofiles).ok()
    }
    pub unsafe fn GetActiveLanguageProfile(&self, rclsid: *const ::windows::core::GUID, plangid: *mut u16, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetActiveLanguageProfile)(::windows::core::Interface::as_raw(self), rclsid, plangid, pguidprofile).ok()
    }
    pub unsafe fn GetLanguageProfileDescription(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetLanguageProfileDescription)(::windows::core::Interface::as_raw(self), rclsid, langid, guidprofile, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrentLanguage(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).GetCurrentLanguage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ChangeCurrentLanguage(&self, langid: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ChangeCurrentLanguage)(::windows::core::Interface::as_raw(self), langid).ok()
    }
    pub unsafe fn GetLanguageList(&self, pplangid: *mut *mut u16, pulcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLanguageList)(::windows::core::Interface::as_raw(self), pplangid, pulcount).ok()
    }
    pub unsafe fn EnumLanguageProfiles(&self, langid: u16) -> ::windows::core::Result<IEnumTfLanguageProfiles> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfLanguageProfiles>();
        (::windows::core::Interface::vtable(self).EnumLanguageProfiles)(::windows::core::Interface::as_raw(self), langid, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableLanguageProfile<P0>(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).EnableLanguageProfile)(::windows::core::Interface::as_raw(self), rclsid, langid, guidprofile, fenable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEnabledLanguageProfile(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsEnabledLanguageProfile)(::windows::core::Interface::as_raw(self), rclsid, langid, guidprofile, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableLanguageProfileByDefault<P0>(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).EnableLanguageProfileByDefault)(::windows::core::Interface::as_raw(self), rclsid, langid, guidprofile, fenable.into_param().abi()).ok()
    }
    pub unsafe fn SubstituteKeyboardLayout<P0>(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, hkl: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<HKL>,
    {
        (::windows::core::Interface::vtable(self).SubstituteKeyboardLayout)(::windows::core::Interface::as_raw(self), rclsid, langid, guidprofile, hkl.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfInputProcessorProfiles, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfInputProcessorProfiles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInputProcessorProfiles {}
impl ::core::fmt::Debug for ITfInputProcessorProfiles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfInputProcessorProfiles").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfInputProcessorProfiles {
    type Vtable = ITfInputProcessorProfiles_Vtbl;
}
impl ::core::clone::Clone for ITfInputProcessorProfiles {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfInputProcessorProfiles {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f02b6c5_7842_4ee6_8a0b_9a24183a95ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputProcessorProfiles_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Unregister: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub AddLanguageProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchdesc: ::windows::core::PCWSTR, cchdesc: u32, pchiconfile: ::windows::core::PCWSTR, cchfile: u32, uiconindex: u32) -> ::windows::core::HRESULT,
    pub RemoveLanguageProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumInputProcessorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumInputProcessorInfo: usize,
    pub GetDefaultLanguageProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16, catid: *const ::windows::core::GUID, pclsid: *mut ::windows::core::GUID, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetDefaultLanguageProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16, rclsid: *const ::windows::core::GUID, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ActivateLanguageProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetActiveLanguageProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, plangid: *mut u16, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetLanguageProfileDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pbstrprofile: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetCurrentLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plangid: *mut u16) -> ::windows::core::HRESULT,
    pub ChangeCurrentLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16) -> ::windows::core::HRESULT,
    pub GetLanguageList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplangid: *mut *mut u16, pulcount: *mut u32) -> ::windows::core::HRESULT,
    pub EnumLanguageProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableLanguageProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableLanguageProfile: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEnabledLanguageProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEnabledLanguageProfile: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableLanguageProfileByDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableLanguageProfileByDefault: usize,
    pub SubstituteKeyboardLayout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, hkl: HKL) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfInputProcessorProfilesEx(::windows::core::IUnknown);
impl ITfInputProcessorProfilesEx {
    pub unsafe fn Register(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Register)(::windows::core::Interface::as_raw(self), rclsid).ok()
    }
    pub unsafe fn Unregister(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Unregister)(::windows::core::Interface::as_raw(self), rclsid).ok()
    }
    pub unsafe fn AddLanguageProfile(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchdesc: &[u16], pchiconfile: &[u16], uiconindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AddLanguageProfile)(::windows::core::Interface::as_raw(self), rclsid, langid, guidprofile, ::core::mem::transmute(pchdesc.as_ptr()), pchdesc.len() as _, ::core::mem::transmute(pchiconfile.as_ptr()), pchiconfile.len() as _, uiconindex).ok()
    }
    pub unsafe fn RemoveLanguageProfile(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RemoveLanguageProfile)(::windows::core::Interface::as_raw(self), rclsid, langid, guidprofile).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumInputProcessorInfo(&self) -> ::windows::core::Result<super::super::System::Com::IEnumGUID> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IEnumGUID>();
        (::windows::core::Interface::vtable(self).base__.EnumInputProcessorInfo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDefaultLanguageProfile(&self, langid: u16, catid: *const ::windows::core::GUID, pclsid: *mut ::windows::core::GUID, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDefaultLanguageProfile)(::windows::core::Interface::as_raw(self), langid, catid, pclsid, pguidprofile).ok()
    }
    pub unsafe fn SetDefaultLanguageProfile(&self, langid: u16, rclsid: *const ::windows::core::GUID, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetDefaultLanguageProfile)(::windows::core::Interface::as_raw(self), langid, rclsid, guidprofiles).ok()
    }
    pub unsafe fn ActivateLanguageProfile(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofiles: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ActivateLanguageProfile)(::windows::core::Interface::as_raw(self), rclsid, langid, guidprofiles).ok()
    }
    pub unsafe fn GetActiveLanguageProfile(&self, rclsid: *const ::windows::core::GUID, plangid: *mut u16, pguidprofile: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetActiveLanguageProfile)(::windows::core::Interface::as_raw(self), rclsid, plangid, pguidprofile).ok()
    }
    pub unsafe fn GetLanguageProfileDescription(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetLanguageProfileDescription)(::windows::core::Interface::as_raw(self), rclsid, langid, guidprofile, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrentLanguage(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).base__.GetCurrentLanguage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ChangeCurrentLanguage(&self, langid: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ChangeCurrentLanguage)(::windows::core::Interface::as_raw(self), langid).ok()
    }
    pub unsafe fn GetLanguageList(&self, pplangid: *mut *mut u16, pulcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetLanguageList)(::windows::core::Interface::as_raw(self), pplangid, pulcount).ok()
    }
    pub unsafe fn EnumLanguageProfiles(&self, langid: u16) -> ::windows::core::Result<IEnumTfLanguageProfiles> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfLanguageProfiles>();
        (::windows::core::Interface::vtable(self).base__.EnumLanguageProfiles)(::windows::core::Interface::as_raw(self), langid, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableLanguageProfile<P0>(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.EnableLanguageProfile)(::windows::core::Interface::as_raw(self), rclsid, langid, guidprofile, fenable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEnabledLanguageProfile(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsEnabledLanguageProfile)(::windows::core::Interface::as_raw(self), rclsid, langid, guidprofile, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableLanguageProfileByDefault<P0>(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, fenable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.EnableLanguageProfileByDefault)(::windows::core::Interface::as_raw(self), rclsid, langid, guidprofile, fenable.into_param().abi()).ok()
    }
    pub unsafe fn SubstituteKeyboardLayout<P0>(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, hkl: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<HKL>,
    {
        (::windows::core::Interface::vtable(self).base__.SubstituteKeyboardLayout)(::windows::core::Interface::as_raw(self), rclsid, langid, guidprofile, hkl.into_param().abi()).ok()
    }
    pub unsafe fn SetLanguageProfileDisplayName(&self, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchfile: &[u16], uresid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLanguageProfileDisplayName)(::windows::core::Interface::as_raw(self), rclsid, langid, guidprofile, ::core::mem::transmute(pchfile.as_ptr()), pchfile.len() as _, uresid).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfInputProcessorProfilesEx, ::windows::core::IUnknown, ITfInputProcessorProfiles);
impl ::core::cmp::PartialEq for ITfInputProcessorProfilesEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInputProcessorProfilesEx {}
impl ::core::fmt::Debug for ITfInputProcessorProfilesEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfInputProcessorProfilesEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfInputProcessorProfilesEx {
    type Vtable = ITfInputProcessorProfilesEx_Vtbl;
}
impl ::core::clone::Clone for ITfInputProcessorProfilesEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfInputProcessorProfilesEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x892f230f_fe00_4a41_a98e_fcd6de0d35ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputProcessorProfilesEx_Vtbl {
    pub base__: ITfInputProcessorProfiles_Vtbl,
    pub SetLanguageProfileDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, langid: u16, guidprofile: *const ::windows::core::GUID, pchfile: ::windows::core::PCWSTR, cchfile: u32, uresid: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfInputScope(::windows::core::IUnknown);
impl ITfInputScope {
    pub unsafe fn GetInputScopes(&self, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInputScopes)(::windows::core::Interface::as_raw(self), pprginputscopes, pccount).ok()
    }
    pub unsafe fn GetPhrase(&self, ppbstrphrases: *mut *mut ::windows::core::BSTR, pccount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPhrase)(::windows::core::Interface::as_raw(self), ppbstrphrases, pccount).ok()
    }
    pub unsafe fn GetRegularExpression(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetRegularExpression)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSRGS(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetSRGS)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetXML(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetXML)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfInputScope, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfInputScope {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInputScope {}
impl ::core::fmt::Debug for ITfInputScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfInputScope").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfInputScope {
    type Vtable = ITfInputScope_Vtbl;
}
impl ::core::clone::Clone for ITfInputScope {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfInputScope {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfde1eaee_6924_4cdf_91e7_da38cff5559d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputScope_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetInputScopes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> ::windows::core::HRESULT,
    pub GetPhrase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbstrphrases: *mut *mut ::windows::core::BSTR, pccount: *mut u32) -> ::windows::core::HRESULT,
    pub GetRegularExpression: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrregexp: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetSRGS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsrgs: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrxml: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfInputScope2(::windows::core::IUnknown);
impl ITfInputScope2 {
    pub unsafe fn GetInputScopes(&self, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetInputScopes)(::windows::core::Interface::as_raw(self), pprginputscopes, pccount).ok()
    }
    pub unsafe fn GetPhrase(&self, ppbstrphrases: *mut *mut ::windows::core::BSTR, pccount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPhrase)(::windows::core::Interface::as_raw(self), ppbstrphrases, pccount).ok()
    }
    pub unsafe fn GetRegularExpression(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetRegularExpression)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSRGS(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetSRGS)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetXML(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetXML)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumWordList(&self) -> ::windows::core::Result<super::super::System::Com::IEnumString> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IEnumString>();
        (::windows::core::Interface::vtable(self).EnumWordList)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfInputScope2, ::windows::core::IUnknown, ITfInputScope);
impl ::core::cmp::PartialEq for ITfInputScope2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInputScope2 {}
impl ::core::fmt::Debug for ITfInputScope2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfInputScope2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfInputScope2 {
    type Vtable = ITfInputScope2_Vtbl;
}
impl ::core::clone::Clone for ITfInputScope2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfInputScope2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5731eaa0_6bc2_4681_a532_92fbb74d7c41);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInputScope2_Vtbl {
    pub base__: ITfInputScope_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumWordList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumstring: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumWordList: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfInsertAtSelection(::windows::core::IUnknown);
impl ITfInsertAtSelection {
    pub unsafe fn InsertTextAtSelection(&self, ec: u32, dwflags: INSERT_TEXT_AT_SELECTION_FLAGS, pchtext: &[u16]) -> ::windows::core::Result<ITfRange> {
        let mut result__ = ::windows::core::zeroed::<ITfRange>();
        (::windows::core::Interface::vtable(self).InsertTextAtSelection)(::windows::core::Interface::as_raw(self), ec, dwflags, ::core::mem::transmute(pchtext.as_ptr()), pchtext.len() as _, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbeddedAtSelection<P0>(&self, ec: u32, dwflags: u32, pdataobject: P0) -> ::windows::core::Result<ITfRange>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IDataObject>,
    {
        let mut result__ = ::windows::core::zeroed::<ITfRange>();
        (::windows::core::Interface::vtable(self).InsertEmbeddedAtSelection)(::windows::core::Interface::as_raw(self), ec, dwflags, pdataobject.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfInsertAtSelection, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfInsertAtSelection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfInsertAtSelection {}
impl ::core::fmt::Debug for ITfInsertAtSelection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfInsertAtSelection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfInsertAtSelection {
    type Vtable = ITfInsertAtSelection_Vtbl;
}
impl ::core::clone::Clone for ITfInsertAtSelection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfInsertAtSelection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55ce16ba_3014_41c1_9ceb_fade1446ac6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfInsertAtSelection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub InsertTextAtSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, dwflags: INSERT_TEXT_AT_SELECTION_FLAGS, pchtext: ::windows::core::PCWSTR, cch: i32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbeddedAtSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pdataobject: *mut ::core::ffi::c_void, pprange: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbeddedAtSelection: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfIntegratableCandidateListUIElement(::windows::core::IUnknown);
impl ITfIntegratableCandidateListUIElement {
    pub unsafe fn SetIntegrationStyle(&self, guidintegrationstyle: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIntegrationStyle)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guidintegrationstyle)).ok()
    }
    pub unsafe fn GetSelectionStyle(&self) -> ::windows::core::Result<TfIntegratableCandidateListSelectionStyle> {
        let mut result__ = ::windows::core::zeroed::<TfIntegratableCandidateListSelectionStyle>();
        (::windows::core::Interface::vtable(self).GetSelectionStyle)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnKeyDown<P0, P1>(&self, wparam: P0, lparam: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).OnKeyDown)(::windows::core::Interface::as_raw(self), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowCandidateNumbers(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).ShowCandidateNumbers)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FinalizeExactCompositionString(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FinalizeExactCompositionString)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfIntegratableCandidateListUIElement, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfIntegratableCandidateListUIElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfIntegratableCandidateListUIElement {}
impl ::core::fmt::Debug for ITfIntegratableCandidateListUIElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfIntegratableCandidateListUIElement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfIntegratableCandidateListUIElement {
    type Vtable = ITfIntegratableCandidateListUIElement_Vtbl;
}
impl ::core::clone::Clone for ITfIntegratableCandidateListUIElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfIntegratableCandidateListUIElement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7a6f54f_b180_416f_b2bf_7bf2e4683d7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfIntegratableCandidateListUIElement_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetIntegrationStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidintegrationstyle: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetSelectionStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptfselectionstyle: *mut TfIntegratableCandidateListSelectionStyle) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnKeyDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnKeyDown: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowCandidateNumbers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowCandidateNumbers: usize,
    pub FinalizeExactCompositionString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfKeyEventSink(::windows::core::IUnknown);
impl ITfKeyEventSink {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnSetFocus<P0>(&self, fforeground: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).OnSetFocus)(::windows::core::Interface::as_raw(self), fforeground.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTestKeyDown<P0, P1, P2>(&self, pic: P0, wparam: P1, lparam: P2) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<ITfContext>,
        P1: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).OnTestKeyDown)(::windows::core::Interface::as_raw(self), pic.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTestKeyUp<P0, P1, P2>(&self, pic: P0, wparam: P1, lparam: P2) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<ITfContext>,
        P1: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).OnTestKeyUp)(::windows::core::Interface::as_raw(self), pic.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnKeyDown<P0, P1, P2>(&self, pic: P0, wparam: P1, lparam: P2) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<ITfContext>,
        P1: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).OnKeyDown)(::windows::core::Interface::as_raw(self), pic.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnKeyUp<P0, P1, P2>(&self, pic: P0, wparam: P1, lparam: P2) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<ITfContext>,
        P1: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).OnKeyUp)(::windows::core::Interface::as_raw(self), pic.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnPreservedKey<P0>(&self, pic: P0, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<ITfContext>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).OnPreservedKey)(::windows::core::Interface::as_raw(self), pic.into_param().abi(), rguid, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfKeyEventSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfKeyEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfKeyEventSink {}
impl ::core::fmt::Debug for ITfKeyEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfKeyEventSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfKeyEventSink {
    type Vtable = ITfKeyEventSink_Vtbl;
}
impl ::core::clone::Clone for ITfKeyEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfKeyEventSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e7f5_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfKeyEventSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnSetFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fforeground: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnSetFocus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTestKeyDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTestKeyDown: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTestKeyUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTestKeyUp: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnKeyDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnKeyDown: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnKeyUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnKeyUp: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnPreservedKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnPreservedKey: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfKeyTraceEventSink(::windows::core::IUnknown);
impl ITfKeyTraceEventSink {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnKeyTraceDown<P0, P1>(&self, wparam: P0, lparam: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).OnKeyTraceDown)(::windows::core::Interface::as_raw(self), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnKeyTraceUp<P0, P1>(&self, wparam: P0, lparam: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).OnKeyTraceUp)(::windows::core::Interface::as_raw(self), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfKeyTraceEventSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfKeyTraceEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfKeyTraceEventSink {}
impl ::core::fmt::Debug for ITfKeyTraceEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfKeyTraceEventSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfKeyTraceEventSink {
    type Vtable = ITfKeyTraceEventSink_Vtbl;
}
impl ::core::clone::Clone for ITfKeyTraceEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfKeyTraceEventSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cd4c13b_1c36_4191_a70a_7f3e611f367d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfKeyTraceEventSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnKeyTraceDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnKeyTraceDown: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnKeyTraceUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnKeyTraceUp: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfKeystrokeMgr(::windows::core::IUnknown);
impl ITfKeystrokeMgr {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AdviseKeyEventSink<P0, P1>(&self, tid: u32, psink: P0, fforeground: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfKeyEventSink>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).AdviseKeyEventSink)(::windows::core::Interface::as_raw(self), tid, psink.into_param().abi(), fforeground.into_param().abi()).ok()
    }
    pub unsafe fn UnadviseKeyEventSink(&self, tid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnadviseKeyEventSink)(::windows::core::Interface::as_raw(self), tid).ok()
    }
    pub unsafe fn GetForeground(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetForeground)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TestKeyDown<P0, P1>(&self, wparam: P0, lparam: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).TestKeyDown)(::windows::core::Interface::as_raw(self), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TestKeyUp<P0, P1>(&self, wparam: P0, lparam: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).TestKeyUp)(::windows::core::Interface::as_raw(self), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn KeyDown<P0, P1>(&self, wparam: P0, lparam: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).KeyDown)(::windows::core::Interface::as_raw(self), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn KeyUp<P0, P1>(&self, wparam: P0, lparam: P1) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).KeyUp)(::windows::core::Interface::as_raw(self), wparam.into_param().abi(), lparam.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPreservedKey<P0>(&self, pic: P0, pprekey: *const TF_PRESERVEDKEY) -> ::windows::core::Result<::windows::core::GUID>
    where
        P0: ::windows::core::IntoParam<ITfContext>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetPreservedKey)(::windows::core::Interface::as_raw(self), pic.into_param().abi(), pprekey, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPreservedKey(&self, rguid: *const ::windows::core::GUID, pprekey: *const TF_PRESERVEDKEY) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsPreservedKey)(::windows::core::Interface::as_raw(self), rguid, pprekey, &mut result__).from_abi(result__)
    }
    pub unsafe fn PreserveKey(&self, tid: u32, rguid: *const ::windows::core::GUID, prekey: *const TF_PRESERVEDKEY, pchdesc: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PreserveKey)(::windows::core::Interface::as_raw(self), tid, rguid, prekey, ::core::mem::transmute(pchdesc.as_ptr()), pchdesc.len() as _).ok()
    }
    pub unsafe fn UnpreserveKey(&self, rguid: *const ::windows::core::GUID, pprekey: *const TF_PRESERVEDKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnpreserveKey)(::windows::core::Interface::as_raw(self), rguid, pprekey).ok()
    }
    pub unsafe fn SetPreservedKeyDescription(&self, rguid: *const ::windows::core::GUID, pchdesc: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPreservedKeyDescription)(::windows::core::Interface::as_raw(self), rguid, ::core::mem::transmute(pchdesc.as_ptr()), pchdesc.len() as _).ok()
    }
    pub unsafe fn GetPreservedKeyDescription(&self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetPreservedKeyDescription)(::windows::core::Interface::as_raw(self), rguid, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SimulatePreservedKey<P0>(&self, pic: P0, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<ITfContext>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).SimulatePreservedKey)(::windows::core::Interface::as_raw(self), pic.into_param().abi(), rguid, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfKeystrokeMgr, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfKeystrokeMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfKeystrokeMgr {}
impl ::core::fmt::Debug for ITfKeystrokeMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfKeystrokeMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfKeystrokeMgr {
    type Vtable = ITfKeystrokeMgr_Vtbl;
}
impl ::core::clone::Clone for ITfKeystrokeMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfKeystrokeMgr {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e7f0_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfKeystrokeMgr_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AdviseKeyEventSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tid: u32, psink: *mut ::core::ffi::c_void, fforeground: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AdviseKeyEventSink: usize,
    pub UnadviseKeyEventSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tid: u32) -> ::windows::core::HRESULT,
    pub GetForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TestKeyDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TestKeyDown: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TestKeyUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TestKeyUp: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub KeyDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    KeyDown: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub KeyUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    KeyUp: usize,
    pub GetPreservedKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, pprekey: *const TF_PRESERVEDKEY, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPreservedKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pprekey: *const TF_PRESERVEDKEY, pfregistered: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPreservedKey: usize,
    pub PreserveKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tid: u32, rguid: *const ::windows::core::GUID, prekey: *const TF_PRESERVEDKEY, pchdesc: ::windows::core::PCWSTR, cchdesc: u32) -> ::windows::core::HRESULT,
    pub UnpreserveKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pprekey: *const TF_PRESERVEDKEY) -> ::windows::core::HRESULT,
    pub SetPreservedKeyDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pchdesc: ::windows::core::PCWSTR, cchdesc: u32) -> ::windows::core::HRESULT,
    pub GetPreservedKeyDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pbstrdesc: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SimulatePreservedKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SimulatePreservedKey: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfLMLattice(::windows::core::IUnknown);
impl ITfLMLattice {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryType(&self, rguidtype: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).QueryType)(::windows::core::Interface::as_raw(self), rguidtype, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumLatticeElements(&self, dwframestart: u32, rguidtype: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumTfLatticeElements> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfLatticeElements>();
        (::windows::core::Interface::vtable(self).EnumLatticeElements)(::windows::core::Interface::as_raw(self), dwframestart, rguidtype, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfLMLattice, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfLMLattice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLMLattice {}
impl ::core::fmt::Debug for ITfLMLattice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfLMLattice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfLMLattice {
    type Vtable = ITfLMLattice_Vtbl;
}
impl ::core::clone::Clone for ITfLMLattice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfLMLattice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4236675_a5bf_4570_9d42_5d6d7b02d59b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLMLattice_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidtype: *const ::windows::core::GUID, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryType: usize,
    pub EnumLatticeElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwframestart: u32, rguidtype: *const ::windows::core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfLangBarEventSink(::windows::core::IUnknown);
impl ITfLangBarEventSink {
    pub unsafe fn OnSetFocus(&self, dwthreadid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSetFocus)(::windows::core::Interface::as_raw(self), dwthreadid).ok()
    }
    pub unsafe fn OnThreadTerminate(&self, dwthreadid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnThreadTerminate)(::windows::core::Interface::as_raw(self), dwthreadid).ok()
    }
    pub unsafe fn OnThreadItemChange(&self, dwthreadid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnThreadItemChange)(::windows::core::Interface::as_raw(self), dwthreadid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnModalInput<P0, P1>(&self, dwthreadid: u32, umsg: u32, wparam: P0, lparam: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).OnModalInput)(::windows::core::Interface::as_raw(self), dwthreadid, umsg, wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    pub unsafe fn ShowFloating(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ShowFloating)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetItemFloatingRect(&self, dwthreadid: u32, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::RECT>();
        (::windows::core::Interface::vtable(self).GetItemFloatingRect)(::windows::core::Interface::as_raw(self), dwthreadid, rguid, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfLangBarEventSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfLangBarEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarEventSink {}
impl ::core::fmt::Debug for ITfLangBarEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfLangBarEventSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfLangBarEventSink {
    type Vtable = ITfLangBarEventSink_Vtbl;
}
impl ::core::clone::Clone for ITfLangBarEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfLangBarEventSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18a4e900_e0ae_11d2_afdd_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarEventSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnSetFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwthreadid: u32) -> ::windows::core::HRESULT,
    pub OnThreadTerminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwthreadid: u32) -> ::windows::core::HRESULT,
    pub OnThreadItemChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwthreadid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnModalInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwthreadid: u32, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnModalInput: usize,
    pub ShowFloating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetItemFloatingRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwthreadid: u32, rguid: *const ::windows::core::GUID, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetItemFloatingRect: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfLangBarItem(::windows::core::IUnknown);
impl ITfLangBarItem {
    pub unsafe fn GetInfo(&self, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInfo)(::windows::core::Interface::as_raw(self), pinfo).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, fshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Show)(::windows::core::Interface::as_raw(self), fshow.into_param().abi()).ok()
    }
    pub unsafe fn GetTooltipString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetTooltipString)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfLangBarItem, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfLangBarItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarItem {}
impl ::core::fmt::Debug for ITfLangBarItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfLangBarItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfLangBarItem {
    type Vtable = ITfLangBarItem_Vtbl;
}
impl ::core::clone::Clone for ITfLangBarItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfLangBarItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73540d69_edeb_4ee9_96c9_23aa30b25916);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItem_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Show: usize,
    pub GetTooltipString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtooltip: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfLangBarItemBalloon(::windows::core::IUnknown);
impl ITfLangBarItemBalloon {
    pub unsafe fn GetInfo(&self, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetInfo)(::windows::core::Interface::as_raw(self), pinfo).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, fshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.Show)(::windows::core::Interface::as_raw(self), fshow.into_param().abi()).ok()
    }
    pub unsafe fn GetTooltipString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetTooltipString)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnClick(&self, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnClick)(::windows::core::Interface::as_raw(self), click, ::core::mem::transmute(pt), prcarea).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreferredSize(&self, pszdefault: *const super::super::Foundation::SIZE) -> ::windows::core::Result<super::super::Foundation::SIZE> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::SIZE>();
        (::windows::core::Interface::vtable(self).GetPreferredSize)(::windows::core::Interface::as_raw(self), pszdefault, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBalloonInfo(&self) -> ::windows::core::Result<TF_LBBALLOONINFO> {
        let mut result__ = ::windows::core::zeroed::<TF_LBBALLOONINFO>();
        (::windows::core::Interface::vtable(self).GetBalloonInfo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfLangBarItemBalloon, ::windows::core::IUnknown, ITfLangBarItem);
impl ::core::cmp::PartialEq for ITfLangBarItemBalloon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarItemBalloon {}
impl ::core::fmt::Debug for ITfLangBarItemBalloon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfLangBarItemBalloon").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfLangBarItemBalloon {
    type Vtable = ITfLangBarItemBalloon_Vtbl;
}
impl ::core::clone::Clone for ITfLangBarItemBalloon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfLangBarItemBalloon {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01c2d285_d3c7_4b7b_b5b5_d97411d0c283);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemBalloon_Vtbl {
    pub base__: ITfLangBarItem_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnClick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnClick: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPreferredSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPreferredSize: usize,
    pub GetBalloonInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut TF_LBBALLOONINFO) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfLangBarItemBitmap(::windows::core::IUnknown);
impl ITfLangBarItemBitmap {
    pub unsafe fn GetInfo(&self, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetInfo)(::windows::core::Interface::as_raw(self), pinfo).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, fshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.Show)(::windows::core::Interface::as_raw(self), fshow.into_param().abi()).ok()
    }
    pub unsafe fn GetTooltipString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetTooltipString)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnClick(&self, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnClick)(::windows::core::Interface::as_raw(self), click, ::core::mem::transmute(pt), prcarea).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreferredSize(&self, pszdefault: *const super::super::Foundation::SIZE) -> ::windows::core::Result<super::super::Foundation::SIZE> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::SIZE>();
        (::windows::core::Interface::vtable(self).GetPreferredSize)(::windows::core::Interface::as_raw(self), pszdefault, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn DrawBitmap(&self, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DrawBitmap)(::windows::core::Interface::as_raw(self), bmwidth, bmheight, dwflags, phbmp, phbmpmask).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfLangBarItemBitmap, ::windows::core::IUnknown, ITfLangBarItem);
impl ::core::cmp::PartialEq for ITfLangBarItemBitmap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarItemBitmap {}
impl ::core::fmt::Debug for ITfLangBarItemBitmap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfLangBarItemBitmap").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfLangBarItemBitmap {
    type Vtable = ITfLangBarItemBitmap_Vtbl;
}
impl ::core::clone::Clone for ITfLangBarItemBitmap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfLangBarItemBitmap {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73830352_d722_4179_ada5_f045c98df355);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemBitmap_Vtbl {
    pub base__: ITfLangBarItem_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnClick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnClick: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPreferredSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPreferredSize: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub DrawBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    DrawBitmap: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfLangBarItemBitmapButton(::windows::core::IUnknown);
impl ITfLangBarItemBitmapButton {
    pub unsafe fn GetInfo(&self, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetInfo)(::windows::core::Interface::as_raw(self), pinfo).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, fshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.Show)(::windows::core::Interface::as_raw(self), fshow.into_param().abi()).ok()
    }
    pub unsafe fn GetTooltipString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetTooltipString)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnClick(&self, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnClick)(::windows::core::Interface::as_raw(self), click, ::core::mem::transmute(pt), prcarea).ok()
    }
    pub unsafe fn InitMenu<P0>(&self, pmenu: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfMenu>,
    {
        (::windows::core::Interface::vtable(self).InitMenu)(::windows::core::Interface::as_raw(self), pmenu.into_param().abi()).ok()
    }
    pub unsafe fn OnMenuSelect(&self, wid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnMenuSelect)(::windows::core::Interface::as_raw(self), wid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreferredSize(&self, pszdefault: *const super::super::Foundation::SIZE) -> ::windows::core::Result<super::super::Foundation::SIZE> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::SIZE>();
        (::windows::core::Interface::vtable(self).GetPreferredSize)(::windows::core::Interface::as_raw(self), pszdefault, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn DrawBitmap(&self, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DrawBitmap)(::windows::core::Interface::as_raw(self), bmwidth, bmheight, dwflags, phbmp, phbmpmask).ok()
    }
    pub unsafe fn GetText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfLangBarItemBitmapButton, ::windows::core::IUnknown, ITfLangBarItem);
impl ::core::cmp::PartialEq for ITfLangBarItemBitmapButton {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarItemBitmapButton {}
impl ::core::fmt::Debug for ITfLangBarItemBitmapButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfLangBarItemBitmapButton").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfLangBarItemBitmapButton {
    type Vtable = ITfLangBarItemBitmapButton_Vtbl;
}
impl ::core::clone::Clone for ITfLangBarItemBitmapButton {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfLangBarItemBitmapButton {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa26a0525_3fae_4fa0_89ee_88a964f9f1b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemBitmapButton_Vtbl {
    pub base__: ITfLangBarItem_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnClick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnClick: usize,
    pub InitMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmenu: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnMenuSelect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPreferredSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPreferredSize: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub DrawBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    DrawBitmap: usize,
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfLangBarItemButton(::windows::core::IUnknown);
impl ITfLangBarItemButton {
    pub unsafe fn GetInfo(&self, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetInfo)(::windows::core::Interface::as_raw(self), pinfo).ok()
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, fshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.Show)(::windows::core::Interface::as_raw(self), fshow.into_param().abi()).ok()
    }
    pub unsafe fn GetTooltipString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetTooltipString)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnClick(&self, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnClick)(::windows::core::Interface::as_raw(self), click, ::core::mem::transmute(pt), prcarea).ok()
    }
    pub unsafe fn InitMenu<P0>(&self, pmenu: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfMenu>,
    {
        (::windows::core::Interface::vtable(self).InitMenu)(::windows::core::Interface::as_raw(self), pmenu.into_param().abi()).ok()
    }
    pub unsafe fn OnMenuSelect(&self, wid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnMenuSelect)(::windows::core::Interface::as_raw(self), wid).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetIcon(&self) -> ::windows::core::Result<super::WindowsAndMessaging::HICON> {
        let mut result__ = ::windows::core::zeroed::<super::WindowsAndMessaging::HICON>();
        (::windows::core::Interface::vtable(self).GetIcon)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfLangBarItemButton, ::windows::core::IUnknown, ITfLangBarItem);
impl ::core::cmp::PartialEq for ITfLangBarItemButton {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarItemButton {}
impl ::core::fmt::Debug for ITfLangBarItemButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfLangBarItemButton").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfLangBarItemButton {
    type Vtable = ITfLangBarItemButton_Vtbl;
}
impl ::core::clone::Clone for ITfLangBarItemButton {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfLangBarItemButton {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28c7f1d0_de25_11d2_afdd_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemButton_Vtbl {
    pub base__: ITfLangBarItem_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnClick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnClick: usize,
    pub InitMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmenu: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnMenuSelect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phicon: *mut super::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetIcon: usize,
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfLangBarItemMgr(::windows::core::IUnknown);
impl ITfLangBarItemMgr {
    pub unsafe fn EnumItems(&self) -> ::windows::core::Result<IEnumTfLangBarItems> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfLangBarItems>();
        (::windows::core::Interface::vtable(self).EnumItems)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetItem(&self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<ITfLangBarItem> {
        let mut result__ = ::windows::core::zeroed::<ITfLangBarItem>();
        (::windows::core::Interface::vtable(self).GetItem)(::windows::core::Interface::as_raw(self), rguid, &mut result__).from_abi(result__)
    }
    pub unsafe fn AddItem<P0>(&self, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfLangBarItem>,
    {
        (::windows::core::Interface::vtable(self).AddItem)(::windows::core::Interface::as_raw(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn RemoveItem<P0>(&self, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfLangBarItem>,
    {
        (::windows::core::Interface::vtable(self).RemoveItem)(::windows::core::Interface::as_raw(self), punk.into_param().abi()).ok()
    }
    pub unsafe fn AdviseItemSink<P0>(&self, punk: P0, pdwcookie: *mut u32, rguiditem: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfLangBarItemSink>,
    {
        (::windows::core::Interface::vtable(self).AdviseItemSink)(::windows::core::Interface::as_raw(self), punk.into_param().abi(), pdwcookie, rguiditem).ok()
    }
    pub unsafe fn UnadviseItemSink(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnadviseItemSink)(::windows::core::Interface::as_raw(self), dwcookie).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetItemFloatingRect(&self, dwthreadid: u32, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::RECT>();
        (::windows::core::Interface::vtable(self).GetItemFloatingRect)(::windows::core::Interface::as_raw(self), dwthreadid, rguid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetItemsStatus(&self, ulcount: u32, prgguid: *const ::windows::core::GUID, pdwstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetItemsStatus)(::windows::core::Interface::as_raw(self), ulcount, prgguid, pdwstatus).ok()
    }
    pub unsafe fn GetItemNum(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetItemNum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetItems(&self, ulcount: u32, ppitem: *mut ::core::option::Option<ITfLangBarItem>, pinfo: *mut TF_LANGBARITEMINFO, pdwstatus: *mut u32, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetItems)(::windows::core::Interface::as_raw(self), ulcount, ::core::mem::transmute(ppitem), pinfo, pdwstatus, pcfetched).ok()
    }
    pub unsafe fn AdviseItemsSink(&self, ulcount: u32, ppunk: *const ::core::option::Option<ITfLangBarItemSink>, pguiditem: *const ::windows::core::GUID, pdwcookie: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AdviseItemsSink)(::windows::core::Interface::as_raw(self), ulcount, ::core::mem::transmute(ppunk), pguiditem, pdwcookie).ok()
    }
    pub unsafe fn UnadviseItemsSink(&self, pdwcookie: &[u32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnadviseItemsSink)(::windows::core::Interface::as_raw(self), pdwcookie.len() as _, ::core::mem::transmute(pdwcookie.as_ptr())).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfLangBarItemMgr, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfLangBarItemMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarItemMgr {}
impl ::core::fmt::Debug for ITfLangBarItemMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfLangBarItemMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfLangBarItemMgr {
    type Vtable = ITfLangBarItemMgr_Vtbl;
}
impl ::core::clone::Clone for ITfLangBarItemMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfLangBarItemMgr {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba468c55_9956_4fb1_a59d_52a7dd7cc6aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemMgr_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EnumItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AdviseItemSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, pdwcookie: *mut u32, rguiditem: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub UnadviseItemSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetItemFloatingRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwthreadid: u32, rguid: *const ::windows::core::GUID, prc: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetItemFloatingRect: usize,
    pub GetItemsStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, prgguid: *const ::windows::core::GUID, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    pub GetItemNum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, ppitem: *mut *mut ::core::ffi::c_void, pinfo: *mut TF_LANGBARITEMINFO, pdwstatus: *mut u32, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub AdviseItemsSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, ppunk: *const *mut ::core::ffi::c_void, pguiditem: *const ::windows::core::GUID, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub UnadviseItemsSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, pdwcookie: *const u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfLangBarItemSink(::windows::core::IUnknown);
impl ITfLangBarItemSink {
    pub unsafe fn OnUpdate(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnUpdate)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfLangBarItemSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfLangBarItemSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarItemSink {}
impl ::core::fmt::Debug for ITfLangBarItemSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfLangBarItemSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfLangBarItemSink {
    type Vtable = ITfLangBarItemSink_Vtbl;
}
impl ::core::clone::Clone for ITfLangBarItemSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfLangBarItemSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57dbe1a0_de25_11d2_afdd_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarItemSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfLangBarMgr(::windows::core::IUnknown);
impl ITfLangBarMgr {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AdviseEventSink<P0, P1>(&self, psink: P0, hwnd: P1, dwflags: u32, pdwcookie: *const u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfLangBarEventSink>,
        P1: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).AdviseEventSink)(::windows::core::Interface::as_raw(self), psink.into_param().abi(), hwnd.into_param().abi(), dwflags, pdwcookie).ok()
    }
    pub unsafe fn UnadviseEventSink(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnadviseEventSink)(::windows::core::Interface::as_raw(self), dwcookie).ok()
    }
    pub unsafe fn GetThreadMarshalInterface(&self, dwthreadid: u32, dwtype: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetThreadMarshalInterface)(::windows::core::Interface::as_raw(self), dwthreadid, dwtype, riid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetThreadLangBarItemMgr(&self, dwthreadid: u32, pplbi: *mut ::core::option::Option<ITfLangBarItemMgr>, pdwthreadid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetThreadLangBarItemMgr)(::windows::core::Interface::as_raw(self), dwthreadid, ::core::mem::transmute(pplbi), pdwthreadid).ok()
    }
    pub unsafe fn GetInputProcessorProfiles(&self, dwthreadid: u32, ppaip: *mut ::core::option::Option<ITfInputProcessorProfiles>, pdwthreadid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInputProcessorProfiles)(::windows::core::Interface::as_raw(self), dwthreadid, ::core::mem::transmute(ppaip), pdwthreadid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RestoreLastFocus<P0>(&self, pdwthreadid: *mut u32, fprev: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).RestoreLastFocus)(::windows::core::Interface::as_raw(self), pdwthreadid, fprev.into_param().abi()).ok()
    }
    pub unsafe fn SetModalInput<P0>(&self, psink: P0, dwthreadid: u32, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfLangBarEventSink>,
    {
        (::windows::core::Interface::vtable(self).SetModalInput)(::windows::core::Interface::as_raw(self), psink.into_param().abi(), dwthreadid, dwflags).ok()
    }
    pub unsafe fn ShowFloating(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ShowFloating)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetShowFloatingStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetShowFloatingStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfLangBarMgr, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfLangBarMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLangBarMgr {}
impl ::core::fmt::Debug for ITfLangBarMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfLangBarMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfLangBarMgr {
    type Vtable = ITfLangBarMgr_Vtbl;
}
impl ::core::clone::Clone for ITfLangBarMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfLangBarMgr {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87955690_e627_11d2_8ddb_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLangBarMgr_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AdviseEventSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, dwflags: u32, pdwcookie: *const u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AdviseEventSink: usize,
    pub UnadviseEventSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
    pub GetThreadMarshalInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwthreadid: u32, dwtype: u32, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetThreadLangBarItemMgr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwthreadid: u32, pplbi: *mut *mut ::core::ffi::c_void, pdwthreadid: *mut u32) -> ::windows::core::HRESULT,
    pub GetInputProcessorProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwthreadid: u32, ppaip: *mut *mut ::core::ffi::c_void, pdwthreadid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RestoreLastFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwthreadid: *mut u32, fprev: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RestoreLastFocus: usize,
    pub SetModalInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, dwthreadid: u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub ShowFloating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
    pub GetShowFloatingStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfLanguageProfileNotifySink(::windows::core::IUnknown);
impl ITfLanguageProfileNotifySink {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnLanguageChange(&self, langid: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).OnLanguageChange)(::windows::core::Interface::as_raw(self), langid, &mut result__).from_abi(result__)
    }
    pub unsafe fn OnLanguageChanged(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnLanguageChanged)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfLanguageProfileNotifySink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfLanguageProfileNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfLanguageProfileNotifySink {}
impl ::core::fmt::Debug for ITfLanguageProfileNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfLanguageProfileNotifySink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfLanguageProfileNotifySink {
    type Vtable = ITfLanguageProfileNotifySink_Vtbl;
}
impl ::core::clone::Clone for ITfLanguageProfileNotifySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfLanguageProfileNotifySink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43c9fe15_f494_4c17_9de2_b8a4ac350aa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfLanguageProfileNotifySink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnLanguageChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16, pfaccept: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnLanguageChange: usize,
    pub OnLanguageChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfMSAAControl(::windows::core::IUnknown);
impl ITfMSAAControl {
    pub unsafe fn SystemEnableMSAA(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SystemEnableMSAA)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SystemDisableMSAA(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SystemDisableMSAA)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfMSAAControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfMSAAControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfMSAAControl {}
impl ::core::fmt::Debug for ITfMSAAControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfMSAAControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfMSAAControl {
    type Vtable = ITfMSAAControl_Vtbl;
}
impl ::core::clone::Clone for ITfMSAAControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfMSAAControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5f8fb3b_393f_4f7c_84cb_504924c2705a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfMSAAControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SystemEnableMSAA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SystemDisableMSAA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfMenu(::windows::core::IUnknown);
impl ITfMenu {
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn AddMenuItem<P0, P1>(&self, uid: u32, dwflags: u32, hbmp: P0, hbmpmask: P1, pch: &[u16], ppmenu: *mut ::core::option::Option<ITfMenu>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HBITMAP>,
        P1: ::windows::core::IntoParam<super::super::Graphics::Gdi::HBITMAP>,
    {
        (::windows::core::Interface::vtable(self).AddMenuItem)(::windows::core::Interface::as_raw(self), uid, dwflags, hbmp.into_param().abi(), hbmpmask.into_param().abi(), ::core::mem::transmute(pch.as_ptr()), pch.len() as _, ::core::mem::transmute(ppmenu)).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfMenu, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfMenu {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfMenu {}
impl ::core::fmt::Debug for ITfMenu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfMenu").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfMenu {
    type Vtable = ITfMenu_Vtbl;
}
impl ::core::clone::Clone for ITfMenu {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfMenu {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f8a98e4_aaa0_4f15_8c5b_07e0df0a3dd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfMenu_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub AddMenuItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uid: u32, dwflags: u32, hbmp: super::super::Graphics::Gdi::HBITMAP, hbmpmask: super::super::Graphics::Gdi::HBITMAP, pch: ::windows::core::PCWSTR, cch: u32, ppmenu: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    AddMenuItem: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfMessagePump(::windows::core::IUnknown);
impl ITfMessagePump {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn PeekMessageA<P0>(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: P0, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).PeekMessageA)(::windows::core::Interface::as_raw(self), pmsg, hwnd.into_param().abi(), wmsgfiltermin, wmsgfiltermax, wremovemsg, pfresult).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetMessageA<P0>(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: P0, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).GetMessageA)(::windows::core::Interface::as_raw(self), pmsg, hwnd.into_param().abi(), wmsgfiltermin, wmsgfiltermax, pfresult).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn PeekMessageW<P0>(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: P0, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).PeekMessageW)(::windows::core::Interface::as_raw(self), pmsg, hwnd.into_param().abi(), wmsgfiltermin, wmsgfiltermax, wremovemsg, pfresult).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub unsafe fn GetMessageW<P0>(&self, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: P0, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).GetMessageW)(::windows::core::Interface::as_raw(self), pmsg, hwnd.into_param().abi(), wmsgfiltermin, wmsgfiltermax, pfresult).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfMessagePump, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfMessagePump {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfMessagePump {}
impl ::core::fmt::Debug for ITfMessagePump {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfMessagePump").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfMessagePump {
    type Vtable = ITfMessagePump_Vtbl;
}
impl ::core::clone::Clone for ITfMessagePump {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfMessagePump {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f1b8ad8_0b6b_4874_90c5_bd76011e8f7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfMessagePump_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub PeekMessageA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    PeekMessageA: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub GetMessageA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    GetMessageA: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub PeekMessageW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    PeekMessageW: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub GetMessageW: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))]
    GetMessageW: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfMouseSink(::windows::core::IUnknown);
impl ITfMouseSink {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnMouseEvent(&self, uedge: u32, uquadrant: u32, dwbtnstatus: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).OnMouseEvent)(::windows::core::Interface::as_raw(self), uedge, uquadrant, dwbtnstatus, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfMouseSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfMouseSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfMouseSink {}
impl ::core::fmt::Debug for ITfMouseSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfMouseSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfMouseSink {
    type Vtable = ITfMouseSink_Vtbl;
}
impl ::core::clone::Clone for ITfMouseSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfMouseSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1adaaa2_3a24_449d_ac96_5183e7f5c217);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfMouseSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnMouseEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uedge: u32, uquadrant: u32, dwbtnstatus: u32, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnMouseEvent: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfMouseTracker(::windows::core::IUnknown);
impl ITfMouseTracker {
    pub unsafe fn AdviseMouseSink<P0, P1>(&self, range: P0, psink: P1) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
        P1: ::windows::core::IntoParam<ITfMouseSink>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).AdviseMouseSink)(::windows::core::Interface::as_raw(self), range.into_param().abi(), psink.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnadviseMouseSink(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnadviseMouseSink)(::windows::core::Interface::as_raw(self), dwcookie).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfMouseTracker, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfMouseTracker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfMouseTracker {}
impl ::core::fmt::Debug for ITfMouseTracker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfMouseTracker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfMouseTracker {
    type Vtable = ITfMouseTracker_Vtbl;
}
impl ::core::clone::Clone for ITfMouseTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfMouseTracker {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09d146cd_a544_4132_925b_7afa8ef322d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfMouseTracker_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AdviseMouseSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, range: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub UnadviseMouseSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfMouseTrackerACP(::windows::core::IUnknown);
impl ITfMouseTrackerACP {
    pub unsafe fn AdviseMouseSink<P0, P1>(&self, range: P0, psink: P1) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<ITfRangeACP>,
        P1: ::windows::core::IntoParam<ITfMouseSink>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).AdviseMouseSink)(::windows::core::Interface::as_raw(self), range.into_param().abi(), psink.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnadviseMouseSink(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnadviseMouseSink)(::windows::core::Interface::as_raw(self), dwcookie).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfMouseTrackerACP, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfMouseTrackerACP {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfMouseTrackerACP {}
impl ::core::fmt::Debug for ITfMouseTrackerACP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfMouseTrackerACP").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfMouseTrackerACP {
    type Vtable = ITfMouseTrackerACP_Vtbl;
}
impl ::core::clone::Clone for ITfMouseTrackerACP {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfMouseTrackerACP {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bdd78e2_c16e_47fd_b883_ce6facc1a208);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfMouseTrackerACP_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AdviseMouseSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, range: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub UnadviseMouseSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfPersistentPropertyLoaderACP(::windows::core::IUnknown);
impl ITfPersistentPropertyLoaderACP {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadProperty(&self, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IStream>();
        (::windows::core::Interface::vtable(self).LoadProperty)(::windows::core::Interface::as_raw(self), phdr, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfPersistentPropertyLoaderACP, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfPersistentPropertyLoaderACP {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfPersistentPropertyLoaderACP {}
impl ::core::fmt::Debug for ITfPersistentPropertyLoaderACP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfPersistentPropertyLoaderACP").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfPersistentPropertyLoaderACP {
    type Vtable = ITfPersistentPropertyLoaderACP_Vtbl;
}
impl ::core::clone::Clone for ITfPersistentPropertyLoaderACP {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfPersistentPropertyLoaderACP {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ef89150_0807_11d3_8df0_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfPersistentPropertyLoaderACP_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadProperty: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfPreservedKeyNotifySink(::windows::core::IUnknown);
impl ITfPreservedKeyNotifySink {
    pub unsafe fn OnUpdated(&self, pprekey: *const TF_PRESERVEDKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnUpdated)(::windows::core::Interface::as_raw(self), pprekey).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfPreservedKeyNotifySink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfPreservedKeyNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfPreservedKeyNotifySink {}
impl ::core::fmt::Debug for ITfPreservedKeyNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfPreservedKeyNotifySink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfPreservedKeyNotifySink {
    type Vtable = ITfPreservedKeyNotifySink_Vtbl;
}
impl ::core::clone::Clone for ITfPreservedKeyNotifySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfPreservedKeyNotifySink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f77c993_d2b1_446e_853e_5912efc8a286);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfPreservedKeyNotifySink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprekey: *const TF_PRESERVEDKEY) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfProperty(::windows::core::IUnknown);
impl ITfProperty {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumRanges<P0>(&self, ec: u32, ppenum: *mut ::core::option::Option<IEnumTfRanges>, ptargetrange: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).base__.EnumRanges)(::windows::core::Interface::as_raw(self), ec, ::core::mem::transmute(ppenum), ptargetrange.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetValue<P0>(&self, ec: u32, prange: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.GetValue)(::windows::core::Interface::as_raw(self), ec, prange.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<ITfContext> {
        let mut result__ = ::windows::core::zeroed::<ITfContext>();
        (::windows::core::Interface::vtable(self).base__.GetContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FindRange<P0>(&self, ec: u32, prange: P0, pprange: *mut ::core::option::Option<ITfRange>, apos: TfAnchor) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).FindRange)(::windows::core::Interface::as_raw(self), ec, prange.into_param().abi(), ::core::mem::transmute(pprange), apos).ok()
    }
    pub unsafe fn SetValueStore<P0, P1>(&self, ec: u32, prange: P0, ppropstore: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
        P1: ::windows::core::IntoParam<ITfPropertyStore>,
    {
        (::windows::core::Interface::vtable(self).SetValueStore)(::windows::core::Interface::as_raw(self), ec, prange.into_param().abi(), ppropstore.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue<P0>(&self, ec: u32, prange: P0, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).SetValue)(::windows::core::Interface::as_raw(self), ec, prange.into_param().abi(), pvarvalue).ok()
    }
    pub unsafe fn Clear<P0>(&self, ec: u32, prange: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).Clear)(::windows::core::Interface::as_raw(self), ec, prange.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfProperty, ::windows::core::IUnknown, ITfReadOnlyProperty);
impl ::core::cmp::PartialEq for ITfProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfProperty {}
impl ::core::fmt::Debug for ITfProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfProperty {
    type Vtable = ITfProperty_Vtbl;
}
impl ::core::clone::Clone for ITfProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfProperty {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2449660_9542_11d2_bf46_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfProperty_Vtbl {
    pub base__: ITfReadOnlyProperty_Vtbl,
    pub FindRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, pprange: *mut *mut ::core::ffi::c_void, apos: TfAnchor) -> ::windows::core::HRESULT,
    pub SetValueStore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, ppropstore: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, pvarvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfPropertyStore(::windows::core::IUnknown);
impl ITfPropertyStore {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDataType(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetDataType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetData(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTextUpdated<P0>(&self, dwflags: u32, prangenew: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).OnTextUpdated)(::windows::core::Interface::as_raw(self), dwflags, prangenew.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Shrink<P0>(&self, prangenew: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).Shrink)(::windows::core::Interface::as_raw(self), prangenew.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Divide<P0, P1>(&self, prangethis: P0, prangenew: P1) -> ::windows::core::Result<ITfPropertyStore>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
        P1: ::windows::core::IntoParam<ITfRange>,
    {
        let mut result__ = ::windows::core::zeroed::<ITfPropertyStore>();
        (::windows::core::Interface::vtable(self).Divide)(::windows::core::Interface::as_raw(self), prangethis.into_param().abi(), prangenew.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ITfPropertyStore> {
        let mut result__ = ::windows::core::zeroed::<ITfPropertyStore>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPropertyRangeCreator(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetPropertyRangeCreator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Serialize<P0>(&self, pstream: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Serialize)(::windows::core::Interface::as_raw(self), pstream.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfPropertyStore, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfPropertyStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfPropertyStore {}
impl ::core::fmt::Debug for ITfPropertyStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfPropertyStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfPropertyStore {
    type Vtable = ITfPropertyStore_Vtbl;
}
impl ::core::clone::Clone for ITfPropertyStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfPropertyStore {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6834b120_88cb_11d2_bf45_00105a2799b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfPropertyStore_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetDataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetData: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTextUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, prangenew: *mut ::core::ffi::c_void, pfaccept: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTextUpdated: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Shrink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prangenew: *mut ::core::ffi::c_void, pffree: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Shrink: usize,
    pub Divide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prangethis: *mut ::core::ffi::c_void, prangenew: *mut ::core::ffi::c_void, pppropstore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropstore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPropertyRangeCreator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Serialize: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfQueryEmbedded(::windows::core::IUnknown);
impl ITfQueryEmbedded {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn QueryInsertEmbedded(&self, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).QueryInsertEmbedded)(::windows::core::Interface::as_raw(self), pguidservice, pformatetc, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfQueryEmbedded, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfQueryEmbedded {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfQueryEmbedded {}
impl ::core::fmt::Debug for ITfQueryEmbedded {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfQueryEmbedded").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfQueryEmbedded {
    type Vtable = ITfQueryEmbedded_Vtbl;
}
impl ::core::clone::Clone for ITfQueryEmbedded {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfQueryEmbedded {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fab9bdb_d250_4169_84e5_6be118fdd7a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfQueryEmbedded_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub QueryInsertEmbedded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows::core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    QueryInsertEmbedded: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfRange(::windows::core::IUnknown);
impl ITfRange {
    pub unsafe fn GetText(&self, ec: u32, dwflags: u32, pchtext: &mut [u16], pcch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetText)(::windows::core::Interface::as_raw(self), ec, dwflags, ::core::mem::transmute(pchtext.as_ptr()), pchtext.len() as _, pcch).ok()
    }
    pub unsafe fn SetText(&self, ec: u32, dwflags: u32, pchtext: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetText)(::windows::core::Interface::as_raw(self), ec, dwflags, ::core::mem::transmute(pchtext.as_ptr()), pchtext.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText(&self, ec: u32) -> ::windows::core::Result<super::super::System::Com::IDataObject> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IDataObject>();
        (::windows::core::Interface::vtable(self).GetFormattedText)(::windows::core::Interface::as_raw(self), ec, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEmbedded(&self, ec: u32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).GetEmbedded)(::windows::core::Interface::as_raw(self), ec, rguidservice, riid, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbedded<P0>(&self, ec: u32, dwflags: u32, pdataobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IDataObject>,
    {
        (::windows::core::Interface::vtable(self).InsertEmbedded)(::windows::core::Interface::as_raw(self), ec, dwflags, pdataobject.into_param().abi()).ok()
    }
    pub unsafe fn ShiftStart(&self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ShiftStart)(::windows::core::Interface::as_raw(self), ec, cchreq, pcch, phalt).ok()
    }
    pub unsafe fn ShiftEnd(&self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ShiftEnd)(::windows::core::Interface::as_raw(self), ec, cchreq, pcch, phalt).ok()
    }
    pub unsafe fn ShiftStartToRange<P0>(&self, ec: u32, prange: P0, apos: TfAnchor) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).ShiftStartToRange)(::windows::core::Interface::as_raw(self), ec, prange.into_param().abi(), apos).ok()
    }
    pub unsafe fn ShiftEndToRange<P0>(&self, ec: u32, prange: P0, apos: TfAnchor) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).ShiftEndToRange)(::windows::core::Interface::as_raw(self), ec, prange.into_param().abi(), apos).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShiftStartRegion(&self, ec: u32, dir: TfShiftDir) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).ShiftStartRegion)(::windows::core::Interface::as_raw(self), ec, dir, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShiftEndRegion(&self, ec: u32, dir: TfShiftDir) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).ShiftEndRegion)(::windows::core::Interface::as_raw(self), ec, dir, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEmpty(&self, ec: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsEmpty)(::windows::core::Interface::as_raw(self), ec, &mut result__).from_abi(result__)
    }
    pub unsafe fn Collapse(&self, ec: u32, apos: TfAnchor) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Collapse)(::windows::core::Interface::as_raw(self), ec, apos).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEqualStart<P0>(&self, ec: u32, pwith: P0, apos: TfAnchor) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsEqualStart)(::windows::core::Interface::as_raw(self), ec, pwith.into_param().abi(), apos, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEqualEnd<P0>(&self, ec: u32, pwith: P0, apos: TfAnchor) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsEqualEnd)(::windows::core::Interface::as_raw(self), ec, pwith.into_param().abi(), apos, &mut result__).from_abi(result__)
    }
    pub unsafe fn CompareStart<P0>(&self, ec: u32, pwith: P0, apos: TfAnchor) -> ::windows::core::Result<i32>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).CompareStart)(::windows::core::Interface::as_raw(self), ec, pwith.into_param().abi(), apos, &mut result__).from_abi(result__)
    }
    pub unsafe fn CompareEnd<P0>(&self, ec: u32, pwith: P0, apos: TfAnchor) -> ::windows::core::Result<i32>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).CompareEnd)(::windows::core::Interface::as_raw(self), ec, pwith.into_param().abi(), apos, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AdjustForInsert(&self, ec: u32, cchinsert: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).AdjustForInsert)(::windows::core::Interface::as_raw(self), ec, cchinsert, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGravity(&self, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGravity)(::windows::core::Interface::as_raw(self), pgstart, pgend).ok()
    }
    pub unsafe fn SetGravity(&self, ec: u32, gstart: TfGravity, gend: TfGravity) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGravity)(::windows::core::Interface::as_raw(self), ec, gstart, gend).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ITfRange> {
        let mut result__ = ::windows::core::zeroed::<ITfRange>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<ITfContext> {
        let mut result__ = ::windows::core::zeroed::<ITfContext>();
        (::windows::core::Interface::vtable(self).GetContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfRange, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfRange {}
impl ::core::fmt::Debug for ITfRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfRange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfRange {
    type Vtable = ITfRange_Vtbl;
}
impl ::core::clone::Clone for ITfRange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfRange {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e7ff_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfRange_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pchtext: ::windows::core::PWSTR, cchmax: u32, pcch: *mut u32) -> ::windows::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pchtext: ::windows::core::PCWSTR, cch: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFormattedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFormattedText: usize,
    pub GetEmbedded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InsertEmbedded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pdataobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InsertEmbedded: usize,
    pub ShiftStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::HRESULT,
    pub ShiftEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::HRESULT,
    pub ShiftStartToRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, apos: TfAnchor) -> ::windows::core::HRESULT,
    pub ShiftEndToRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, apos: TfAnchor) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ShiftStartRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShiftStartRegion: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShiftEndRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShiftEndRegion: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEmpty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, pfempty: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEmpty: usize,
    pub Collapse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, apos: TfAnchor) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEqualStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, pwith: *mut ::core::ffi::c_void, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEqualStart: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEqualEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, pwith: *mut ::core::ffi::c_void, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEqualEnd: usize,
    pub CompareStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, pwith: *mut ::core::ffi::c_void, apos: TfAnchor, plresult: *mut i32) -> ::windows::core::HRESULT,
    pub CompareEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, pwith: *mut ::core::ffi::c_void, apos: TfAnchor, plresult: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AdjustForInsert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, cchinsert: u32, pfinsertok: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AdjustForInsert: usize,
    pub GetGravity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> ::windows::core::HRESULT,
    pub SetGravity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, gstart: TfGravity, gend: TfGravity) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclone: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfRangeACP(::windows::core::IUnknown);
impl ITfRangeACP {
    pub unsafe fn GetText(&self, ec: u32, dwflags: u32, pchtext: &mut [u16], pcch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetText)(::windows::core::Interface::as_raw(self), ec, dwflags, ::core::mem::transmute(pchtext.as_ptr()), pchtext.len() as _, pcch).ok()
    }
    pub unsafe fn SetText(&self, ec: u32, dwflags: u32, pchtext: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetText)(::windows::core::Interface::as_raw(self), ec, dwflags, ::core::mem::transmute(pchtext.as_ptr()), pchtext.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormattedText(&self, ec: u32) -> ::windows::core::Result<super::super::System::Com::IDataObject> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IDataObject>();
        (::windows::core::Interface::vtable(self).base__.GetFormattedText)(::windows::core::Interface::as_raw(self), ec, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEmbedded(&self, ec: u32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.GetEmbedded)(::windows::core::Interface::as_raw(self), ec, rguidservice, riid, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InsertEmbedded<P0>(&self, ec: u32, dwflags: u32, pdataobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IDataObject>,
    {
        (::windows::core::Interface::vtable(self).base__.InsertEmbedded)(::windows::core::Interface::as_raw(self), ec, dwflags, pdataobject.into_param().abi()).ok()
    }
    pub unsafe fn ShiftStart(&self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ShiftStart)(::windows::core::Interface::as_raw(self), ec, cchreq, pcch, phalt).ok()
    }
    pub unsafe fn ShiftEnd(&self, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ShiftEnd)(::windows::core::Interface::as_raw(self), ec, cchreq, pcch, phalt).ok()
    }
    pub unsafe fn ShiftStartToRange<P0>(&self, ec: u32, prange: P0, apos: TfAnchor) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).base__.ShiftStartToRange)(::windows::core::Interface::as_raw(self), ec, prange.into_param().abi(), apos).ok()
    }
    pub unsafe fn ShiftEndToRange<P0>(&self, ec: u32, prange: P0, apos: TfAnchor) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).base__.ShiftEndToRange)(::windows::core::Interface::as_raw(self), ec, prange.into_param().abi(), apos).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShiftStartRegion(&self, ec: u32, dir: TfShiftDir) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.ShiftStartRegion)(::windows::core::Interface::as_raw(self), ec, dir, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShiftEndRegion(&self, ec: u32, dir: TfShiftDir) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.ShiftEndRegion)(::windows::core::Interface::as_raw(self), ec, dir, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEmpty(&self, ec: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsEmpty)(::windows::core::Interface::as_raw(self), ec, &mut result__).from_abi(result__)
    }
    pub unsafe fn Collapse(&self, ec: u32, apos: TfAnchor) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Collapse)(::windows::core::Interface::as_raw(self), ec, apos).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEqualStart<P0>(&self, ec: u32, pwith: P0, apos: TfAnchor) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsEqualStart)(::windows::core::Interface::as_raw(self), ec, pwith.into_param().abi(), apos, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEqualEnd<P0>(&self, ec: u32, pwith: P0, apos: TfAnchor) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsEqualEnd)(::windows::core::Interface::as_raw(self), ec, pwith.into_param().abi(), apos, &mut result__).from_abi(result__)
    }
    pub unsafe fn CompareStart<P0>(&self, ec: u32, pwith: P0, apos: TfAnchor) -> ::windows::core::Result<i32>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.CompareStart)(::windows::core::Interface::as_raw(self), ec, pwith.into_param().abi(), apos, &mut result__).from_abi(result__)
    }
    pub unsafe fn CompareEnd<P0>(&self, ec: u32, pwith: P0, apos: TfAnchor) -> ::windows::core::Result<i32>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.CompareEnd)(::windows::core::Interface::as_raw(self), ec, pwith.into_param().abi(), apos, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AdjustForInsert(&self, ec: u32, cchinsert: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.AdjustForInsert)(::windows::core::Interface::as_raw(self), ec, cchinsert, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGravity(&self, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetGravity)(::windows::core::Interface::as_raw(self), pgstart, pgend).ok()
    }
    pub unsafe fn SetGravity(&self, ec: u32, gstart: TfGravity, gend: TfGravity) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetGravity)(::windows::core::Interface::as_raw(self), ec, gstart, gend).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ITfRange> {
        let mut result__ = ::windows::core::zeroed::<ITfRange>();
        (::windows::core::Interface::vtable(self).base__.Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<ITfContext> {
        let mut result__ = ::windows::core::zeroed::<ITfContext>();
        (::windows::core::Interface::vtable(self).base__.GetContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetExtent(&self, pacpanchor: *mut i32, pcch: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetExtent)(::windows::core::Interface::as_raw(self), pacpanchor, pcch).ok()
    }
    pub unsafe fn SetExtent(&self, acpanchor: i32, cch: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetExtent)(::windows::core::Interface::as_raw(self), acpanchor, cch).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfRangeACP, ::windows::core::IUnknown, ITfRange);
impl ::core::cmp::PartialEq for ITfRangeACP {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfRangeACP {}
impl ::core::fmt::Debug for ITfRangeACP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfRangeACP").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfRangeACP {
    type Vtable = ITfRangeACP_Vtbl;
}
impl ::core::clone::Clone for ITfRangeACP {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfRangeACP {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x057a6296_029b_4154_b79a_0d461d4ea94c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfRangeACP_Vtbl {
    pub base__: ITfRange_Vtbl,
    pub GetExtent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pacpanchor: *mut i32, pcch: *mut i32) -> ::windows::core::HRESULT,
    pub SetExtent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acpanchor: i32, cch: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfRangeBackup(::windows::core::IUnknown);
impl ITfRangeBackup {
    pub unsafe fn Restore<P0>(&self, ec: u32, prange: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).Restore)(::windows::core::Interface::as_raw(self), ec, prange.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfRangeBackup, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfRangeBackup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfRangeBackup {}
impl ::core::fmt::Debug for ITfRangeBackup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfRangeBackup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfRangeBackup {
    type Vtable = ITfRangeBackup_Vtbl;
}
impl ::core::clone::Clone for ITfRangeBackup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfRangeBackup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x463a506d_6992_49d2_9b88_93d55e70bb16);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfRangeBackup_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Restore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfReadOnlyProperty(::windows::core::IUnknown);
impl ITfReadOnlyProperty {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumRanges<P0>(&self, ec: u32, ppenum: *mut ::core::option::Option<IEnumTfRanges>, ptargetrange: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        (::windows::core::Interface::vtable(self).EnumRanges)(::windows::core::Interface::as_raw(self), ec, ::core::mem::transmute(ppenum), ptargetrange.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetValue<P0>(&self, ec: u32, prange: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<ITfRange>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetValue)(::windows::core::Interface::as_raw(self), ec, prange.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<ITfContext> {
        let mut result__ = ::windows::core::zeroed::<ITfContext>();
        (::windows::core::Interface::vtable(self).GetContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfReadOnlyProperty, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfReadOnlyProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfReadOnlyProperty {}
impl ::core::fmt::Debug for ITfReadOnlyProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfReadOnlyProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfReadOnlyProperty {
    type Vtable = ITfReadOnlyProperty_Vtbl;
}
impl ::core::clone::Clone for ITfReadOnlyProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfReadOnlyProperty {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17d49a3d_f8b8_4b2f_b254_52319dd64c53);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfReadOnlyProperty_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub EnumRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, ppenum: *mut *mut ::core::ffi::c_void, ptargetrange: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetValue: usize,
    pub GetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfReadingInformationUIElement(::windows::core::IUnknown);
impl ITfReadingInformationUIElement {
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDescription)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetGUID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, bshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.Show)(::windows::core::Interface::as_raw(self), bshow.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsShown(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsShown)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetUpdatedFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetUpdatedFlags)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetContext(&self) -> ::windows::core::Result<ITfContext> {
        let mut result__ = ::windows::core::zeroed::<ITfContext>();
        (::windows::core::Interface::vtable(self).GetContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetString)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMaxReadingStringLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMaxReadingStringLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetErrorIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetErrorIndex)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsVerticalOrderPreferred(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsVerticalOrderPreferred)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfReadingInformationUIElement, ::windows::core::IUnknown, ITfUIElement);
impl ::core::cmp::PartialEq for ITfReadingInformationUIElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfReadingInformationUIElement {}
impl ::core::fmt::Debug for ITfReadingInformationUIElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfReadingInformationUIElement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfReadingInformationUIElement {
    type Vtable = ITfReadingInformationUIElement_Vtbl;
}
impl ::core::clone::Clone for ITfReadingInformationUIElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfReadingInformationUIElement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea1ea139_19df_11d7_a6d2_00065b84435c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfReadingInformationUIElement_Vtbl {
    pub base__: ITfUIElement_Vtbl,
    pub GetUpdatedFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppic: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetMaxReadingStringLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcchmax: *mut u32) -> ::windows::core::HRESULT,
    pub GetErrorIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, perrorindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsVerticalOrderPreferred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfvertical: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsVerticalOrderPreferred: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfReverseConversion(::windows::core::IUnknown);
impl ITfReverseConversion {
    pub unsafe fn DoReverseConversion<P0>(&self, lpstr: P0) -> ::windows::core::Result<ITfReverseConversionList>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ITfReverseConversionList>();
        (::windows::core::Interface::vtable(self).DoReverseConversion)(::windows::core::Interface::as_raw(self), lpstr.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfReverseConversion, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfReverseConversion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfReverseConversion {}
impl ::core::fmt::Debug for ITfReverseConversion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfReverseConversion").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfReverseConversion {
    type Vtable = ITfReverseConversion_Vtbl;
}
impl ::core::clone::Clone for ITfReverseConversion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfReverseConversion {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa415e162_157d_417d_8a8c_0ab26c7d2781);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfReverseConversion_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub DoReverseConversion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpstr: ::windows::core::PCWSTR, pplist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfReverseConversionList(::windows::core::IUnknown);
impl ITfReverseConversionList {
    pub unsafe fn GetLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetString(&self, uindex: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetString)(::windows::core::Interface::as_raw(self), uindex, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfReverseConversionList, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfReverseConversionList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfReverseConversionList {}
impl ::core::fmt::Debug for ITfReverseConversionList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfReverseConversionList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfReverseConversionList {
    type Vtable = ITfReverseConversionList_Vtbl;
}
impl ::core::clone::Clone for ITfReverseConversionList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfReverseConversionList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x151d69f0_86f4_4674_b721_56911e797f47);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfReverseConversionList_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puindex: *mut u32) -> ::windows::core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, pbstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfReverseConversionMgr(::windows::core::IUnknown);
impl ITfReverseConversionMgr {
    pub unsafe fn GetReverseConversion(&self, langid: u16, guidprofile: *const ::windows::core::GUID, dwflag: u32) -> ::windows::core::Result<ITfReverseConversion> {
        let mut result__ = ::windows::core::zeroed::<ITfReverseConversion>();
        (::windows::core::Interface::vtable(self).GetReverseConversion)(::windows::core::Interface::as_raw(self), langid, guidprofile, dwflag, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfReverseConversionMgr, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfReverseConversionMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfReverseConversionMgr {}
impl ::core::fmt::Debug for ITfReverseConversionMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfReverseConversionMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfReverseConversionMgr {
    type Vtable = ITfReverseConversionMgr_Vtbl;
}
impl ::core::clone::Clone for ITfReverseConversionMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfReverseConversionMgr {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb643c236_c493_41b6_abb3_692412775cc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfReverseConversionMgr_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetReverseConversion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16, guidprofile: *const ::windows::core::GUID, dwflag: u32, ppreverseconversion: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfSource(::windows::core::IUnknown);
impl ITfSource {
    pub unsafe fn AdviseSink<P0>(&self, riid: *const ::windows::core::GUID, punk: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).AdviseSink)(::windows::core::Interface::as_raw(self), riid, punk.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn UnadviseSink(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnadviseSink)(::windows::core::Interface::as_raw(self), dwcookie).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfSource, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfSource {}
impl ::core::fmt::Debug for ITfSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfSource {
    type Vtable = ITfSource_Vtbl;
}
impl ::core::clone::Clone for ITfSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ea48a35_60ae_446f_8fd6_e6a8d82459f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSource_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AdviseSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT,
    pub UnadviseSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfSourceSingle(::windows::core::IUnknown);
impl ITfSourceSingle {
    pub unsafe fn AdviseSingleSink<P0>(&self, tid: u32, riid: *const ::windows::core::GUID, punk: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).AdviseSingleSink)(::windows::core::Interface::as_raw(self), tid, riid, punk.into_param().abi()).ok()
    }
    pub unsafe fn UnadviseSingleSink(&self, tid: u32, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnadviseSingleSink)(::windows::core::Interface::as_raw(self), tid, riid).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfSourceSingle, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfSourceSingle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfSourceSingle {}
impl ::core::fmt::Debug for ITfSourceSingle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfSourceSingle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfSourceSingle {
    type Vtable = ITfSourceSingle_Vtbl;
}
impl ::core::clone::Clone for ITfSourceSingle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfSourceSingle {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73131f9c_56a9_49dd_b0ee_d046633f7528);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSourceSingle_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AdviseSingleSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tid: u32, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnadviseSingleSink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tid: u32, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfSpeechUIServer(::windows::core::IUnknown);
impl ITfSpeechUIServer {
    pub unsafe fn Initialize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowUI<P0>(&self, fshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).ShowUI)(::windows::core::Interface::as_raw(self), fshow.into_param().abi()).ok()
    }
    pub unsafe fn UpdateBalloon(&self, style: TfLBBalloonStyle, pch: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateBalloon)(::windows::core::Interface::as_raw(self), style, ::core::mem::transmute(pch.as_ptr()), pch.len() as _).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfSpeechUIServer, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfSpeechUIServer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfSpeechUIServer {}
impl ::core::fmt::Debug for ITfSpeechUIServer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfSpeechUIServer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfSpeechUIServer {
    type Vtable = ITfSpeechUIServer_Vtbl;
}
impl ::core::clone::Clone for ITfSpeechUIServer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfSpeechUIServer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90e9a944_9244_489f_a78f_de67afc013a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSpeechUIServer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowUI: usize,
    pub UpdateBalloon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, style: TfLBBalloonStyle, pch: ::windows::core::PCWSTR, cch: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfStatusSink(::windows::core::IUnknown);
impl ITfStatusSink {
    pub unsafe fn OnStatusChange<P0>(&self, pic: P0, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfContext>,
    {
        (::windows::core::Interface::vtable(self).OnStatusChange)(::windows::core::Interface::as_raw(self), pic.into_param().abi(), dwflags).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfStatusSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfStatusSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfStatusSink {}
impl ::core::fmt::Debug for ITfStatusSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfStatusSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfStatusSink {
    type Vtable = ITfStatusSink_Vtbl;
}
impl ::core::clone::Clone for ITfStatusSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfStatusSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b7d8d73_b267_4f69_b32e_1ca321ce4f45);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfStatusSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnStatusChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfSystemDeviceTypeLangBarItem(::windows::core::IUnknown);
impl ITfSystemDeviceTypeLangBarItem {
    pub unsafe fn SetIconMode(&self, dwflags: LANG_BAR_ITEM_ICON_MODE_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIconMode)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn GetIconMode(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetIconMode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfSystemDeviceTypeLangBarItem, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfSystemDeviceTypeLangBarItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfSystemDeviceTypeLangBarItem {}
impl ::core::fmt::Debug for ITfSystemDeviceTypeLangBarItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfSystemDeviceTypeLangBarItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfSystemDeviceTypeLangBarItem {
    type Vtable = ITfSystemDeviceTypeLangBarItem_Vtbl;
}
impl ::core::clone::Clone for ITfSystemDeviceTypeLangBarItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfSystemDeviceTypeLangBarItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45672eb9_9059_46a2_838d_4530355f6a77);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSystemDeviceTypeLangBarItem_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetIconMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: LANG_BAR_ITEM_ICON_MODE_FLAGS) -> ::windows::core::HRESULT,
    pub GetIconMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfSystemLangBarItem(::windows::core::IUnknown);
impl ITfSystemLangBarItem {
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn SetIcon<P0>(&self, hicon: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::WindowsAndMessaging::HICON>,
    {
        (::windows::core::Interface::vtable(self).SetIcon)(::windows::core::Interface::as_raw(self), hicon.into_param().abi()).ok()
    }
    pub unsafe fn SetTooltipString(&self, pchtooltip: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTooltipString)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pchtooltip.as_ptr()), pchtooltip.len() as _).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfSystemLangBarItem, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfSystemLangBarItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfSystemLangBarItem {}
impl ::core::fmt::Debug for ITfSystemLangBarItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfSystemLangBarItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfSystemLangBarItem {
    type Vtable = ITfSystemLangBarItem_Vtbl;
}
impl ::core::clone::Clone for ITfSystemLangBarItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfSystemLangBarItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e13e9ec_6b33_4d4a_b5eb_8a92f029f356);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSystemLangBarItem_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub SetIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hicon: super::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    SetIcon: usize,
    pub SetTooltipString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchtooltip: ::windows::core::PCWSTR, cch: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfSystemLangBarItemSink(::windows::core::IUnknown);
impl ITfSystemLangBarItemSink {
    pub unsafe fn InitMenu<P0>(&self, pmenu: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfMenu>,
    {
        (::windows::core::Interface::vtable(self).InitMenu)(::windows::core::Interface::as_raw(self), pmenu.into_param().abi()).ok()
    }
    pub unsafe fn OnMenuSelect(&self, wid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnMenuSelect)(::windows::core::Interface::as_raw(self), wid).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfSystemLangBarItemSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfSystemLangBarItemSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfSystemLangBarItemSink {}
impl ::core::fmt::Debug for ITfSystemLangBarItemSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfSystemLangBarItemSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfSystemLangBarItemSink {
    type Vtable = ITfSystemLangBarItemSink_Vtbl;
}
impl ::core::clone::Clone for ITfSystemLangBarItemSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfSystemLangBarItemSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1449d9ab_13cf_4687_aa3e_8d8b18574396);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSystemLangBarItemSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub InitMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmenu: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnMenuSelect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wid: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfSystemLangBarItemText(::windows::core::IUnknown);
impl ITfSystemLangBarItemText {
    pub unsafe fn SetItemText(&self, pch: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetItemText)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pch.as_ptr()), pch.len() as _).ok()
    }
    pub unsafe fn GetItemText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetItemText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfSystemLangBarItemText, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfSystemLangBarItemText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfSystemLangBarItemText {}
impl ::core::fmt::Debug for ITfSystemLangBarItemText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfSystemLangBarItemText").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfSystemLangBarItemText {
    type Vtable = ITfSystemLangBarItemText_Vtbl;
}
impl ::core::clone::Clone for ITfSystemLangBarItemText {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfSystemLangBarItemText {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c4ce0e5_ba49_4b52_ac6b_3b397b4f701f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfSystemLangBarItemText_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetItemText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pch: ::windows::core::PCWSTR, cch: u32) -> ::windows::core::HRESULT,
    pub GetItemText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfTextEditSink(::windows::core::IUnknown);
impl ITfTextEditSink {
    pub unsafe fn OnEndEdit<P0, P1>(&self, pic: P0, ecreadonly: u32, peditrecord: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfContext>,
        P1: ::windows::core::IntoParam<ITfEditRecord>,
    {
        (::windows::core::Interface::vtable(self).OnEndEdit)(::windows::core::Interface::as_raw(self), pic.into_param().abi(), ecreadonly, peditrecord.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfTextEditSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfTextEditSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfTextEditSink {}
impl ::core::fmt::Debug for ITfTextEditSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfTextEditSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfTextEditSink {
    type Vtable = ITfTextEditSink_Vtbl;
}
impl ::core::clone::Clone for ITfTextEditSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfTextEditSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8127d409_ccd3_4683_967a_b43d5b482bf7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTextEditSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnEndEdit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, ecreadonly: u32, peditrecord: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfTextInputProcessor(::windows::core::IUnknown);
impl ITfTextInputProcessor {
    pub unsafe fn Activate<P0>(&self, ptim: P0, tid: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfThreadMgr>,
    {
        (::windows::core::Interface::vtable(self).Activate)(::windows::core::Interface::as_raw(self), ptim.into_param().abi(), tid).ok()
    }
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Deactivate)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfTextInputProcessor, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfTextInputProcessor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfTextInputProcessor {}
impl ::core::fmt::Debug for ITfTextInputProcessor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfTextInputProcessor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfTextInputProcessor {
    type Vtable = ITfTextInputProcessor_Vtbl;
}
impl ::core::clone::Clone for ITfTextInputProcessor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfTextInputProcessor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e7f7_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTextInputProcessor_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptim: *mut ::core::ffi::c_void, tid: u32) -> ::windows::core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfTextInputProcessorEx(::windows::core::IUnknown);
impl ITfTextInputProcessorEx {
    pub unsafe fn Activate<P0>(&self, ptim: P0, tid: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfThreadMgr>,
    {
        (::windows::core::Interface::vtable(self).base__.Activate)(::windows::core::Interface::as_raw(self), ptim.into_param().abi(), tid).ok()
    }
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Deactivate)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ActivateEx<P0>(&self, ptim: P0, tid: u32, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfThreadMgr>,
    {
        (::windows::core::Interface::vtable(self).ActivateEx)(::windows::core::Interface::as_raw(self), ptim.into_param().abi(), tid, dwflags).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfTextInputProcessorEx, ::windows::core::IUnknown, ITfTextInputProcessor);
impl ::core::cmp::PartialEq for ITfTextInputProcessorEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfTextInputProcessorEx {}
impl ::core::fmt::Debug for ITfTextInputProcessorEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfTextInputProcessorEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfTextInputProcessorEx {
    type Vtable = ITfTextInputProcessorEx_Vtbl;
}
impl ::core::clone::Clone for ITfTextInputProcessorEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfTextInputProcessorEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e4e2102_f9cd_433d_b496_303ce03a6507);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTextInputProcessorEx_Vtbl {
    pub base__: ITfTextInputProcessor_Vtbl,
    pub ActivateEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptim: *mut ::core::ffi::c_void, tid: u32, dwflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfTextLayoutSink(::windows::core::IUnknown);
impl ITfTextLayoutSink {
    pub unsafe fn OnLayoutChange<P0, P1>(&self, pic: P0, lcode: TfLayoutCode, pview: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfContext>,
        P1: ::windows::core::IntoParam<ITfContextView>,
    {
        (::windows::core::Interface::vtable(self).OnLayoutChange)(::windows::core::Interface::as_raw(self), pic.into_param().abi(), lcode, pview.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfTextLayoutSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfTextLayoutSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfTextLayoutSink {}
impl ::core::fmt::Debug for ITfTextLayoutSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfTextLayoutSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfTextLayoutSink {
    type Vtable = ITfTextLayoutSink_Vtbl;
}
impl ::core::clone::Clone for ITfTextLayoutSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfTextLayoutSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2af2d06a_dd5b_4927_a0b4_54f19c91fade);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTextLayoutSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnLayoutChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, lcode: TfLayoutCode, pview: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfThreadFocusSink(::windows::core::IUnknown);
impl ITfThreadFocusSink {
    pub unsafe fn OnSetThreadFocus(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnSetThreadFocus)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnKillThreadFocus(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnKillThreadFocus)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfThreadFocusSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfThreadFocusSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfThreadFocusSink {}
impl ::core::fmt::Debug for ITfThreadFocusSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfThreadFocusSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfThreadFocusSink {
    type Vtable = ITfThreadFocusSink_Vtbl;
}
impl ::core::clone::Clone for ITfThreadFocusSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfThreadFocusSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0f1db0c_3a20_405c_a303_96b6010a885f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfThreadFocusSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnSetThreadFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnKillThreadFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfThreadMgr(::windows::core::IUnknown);
impl ITfThreadMgr {
    pub unsafe fn Activate(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Activate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Deactivate)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CreateDocumentMgr(&self) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__ = ::windows::core::zeroed::<ITfDocumentMgr>();
        (::windows::core::Interface::vtable(self).CreateDocumentMgr)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDocumentMgrs(&self) -> ::windows::core::Result<IEnumTfDocumentMgrs> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfDocumentMgrs>();
        (::windows::core::Interface::vtable(self).EnumDocumentMgrs)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFocus(&self) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__ = ::windows::core::zeroed::<ITfDocumentMgr>();
        (::windows::core::Interface::vtable(self).GetFocus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFocus<P0>(&self, pdimfocus: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfDocumentMgr>,
    {
        (::windows::core::Interface::vtable(self).SetFocus)(::windows::core::Interface::as_raw(self), pdimfocus.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AssociateFocus<P0, P1>(&self, hwnd: P0, pdimnew: P1) -> ::windows::core::Result<ITfDocumentMgr>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<ITfDocumentMgr>,
    {
        let mut result__ = ::windows::core::zeroed::<ITfDocumentMgr>();
        (::windows::core::Interface::vtable(self).AssociateFocus)(::windows::core::Interface::as_raw(self), hwnd.into_param().abi(), pdimnew.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsThreadFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsThreadFocus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFunctionProvider(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<ITfFunctionProvider> {
        let mut result__ = ::windows::core::zeroed::<ITfFunctionProvider>();
        (::windows::core::Interface::vtable(self).GetFunctionProvider)(::windows::core::Interface::as_raw(self), clsid, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumFunctionProviders(&self) -> ::windows::core::Result<IEnumTfFunctionProviders> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfFunctionProviders>();
        (::windows::core::Interface::vtable(self).EnumFunctionProviders)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGlobalCompartment(&self) -> ::windows::core::Result<ITfCompartmentMgr> {
        let mut result__ = ::windows::core::zeroed::<ITfCompartmentMgr>();
        (::windows::core::Interface::vtable(self).GetGlobalCompartment)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfThreadMgr, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfThreadMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfThreadMgr {}
impl ::core::fmt::Debug for ITfThreadMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfThreadMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfThreadMgr {
    type Vtable = ITfThreadMgr_Vtbl;
}
impl ::core::clone::Clone for ITfThreadMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfThreadMgr {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e801_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfThreadMgr_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptid: *mut u32) -> ::windows::core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateDocumentMgr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdim: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumDocumentMgrs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdimfocus: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdimfocus: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AssociateFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pdimnew: *mut ::core::ffi::c_void, ppdimprev: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AssociateFocus: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsThreadFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfthreadfocus: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsThreadFocus: usize,
    pub GetFunctionProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, ppfuncprov: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumFunctionProviders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetGlobalCompartment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcompmgr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfThreadMgr2(::windows::core::IUnknown);
impl ITfThreadMgr2 {
    pub unsafe fn Activate(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).Activate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Deactivate)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CreateDocumentMgr(&self) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__ = ::windows::core::zeroed::<ITfDocumentMgr>();
        (::windows::core::Interface::vtable(self).CreateDocumentMgr)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDocumentMgrs(&self) -> ::windows::core::Result<IEnumTfDocumentMgrs> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfDocumentMgrs>();
        (::windows::core::Interface::vtable(self).EnumDocumentMgrs)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFocus(&self) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__ = ::windows::core::zeroed::<ITfDocumentMgr>();
        (::windows::core::Interface::vtable(self).GetFocus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFocus<P0>(&self, pdimfocus: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfDocumentMgr>,
    {
        (::windows::core::Interface::vtable(self).SetFocus)(::windows::core::Interface::as_raw(self), pdimfocus.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsThreadFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsThreadFocus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFunctionProvider(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<ITfFunctionProvider> {
        let mut result__ = ::windows::core::zeroed::<ITfFunctionProvider>();
        (::windows::core::Interface::vtable(self).GetFunctionProvider)(::windows::core::Interface::as_raw(self), clsid, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumFunctionProviders(&self) -> ::windows::core::Result<IEnumTfFunctionProviders> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfFunctionProviders>();
        (::windows::core::Interface::vtable(self).EnumFunctionProviders)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGlobalCompartment(&self) -> ::windows::core::Result<ITfCompartmentMgr> {
        let mut result__ = ::windows::core::zeroed::<ITfCompartmentMgr>();
        (::windows::core::Interface::vtable(self).GetGlobalCompartment)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ActivateEx(&self, ptid: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ActivateEx)(::windows::core::Interface::as_raw(self), ptid, dwflags).ok()
    }
    pub unsafe fn GetActiveFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetActiveFlags)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SuspendKeystrokeHandling(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SuspendKeystrokeHandling)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ResumeKeystrokeHandling(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResumeKeystrokeHandling)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfThreadMgr2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfThreadMgr2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfThreadMgr2 {}
impl ::core::fmt::Debug for ITfThreadMgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfThreadMgr2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfThreadMgr2 {
    type Vtable = ITfThreadMgr2_Vtbl;
}
impl ::core::clone::Clone for ITfThreadMgr2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfThreadMgr2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ab198ef_6477_4ee8_8812_6780edb82d5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfThreadMgr2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptid: *mut u32) -> ::windows::core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateDocumentMgr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdim: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumDocumentMgrs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdimfocus: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdimfocus: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsThreadFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfthreadfocus: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsThreadFocus: usize,
    pub GetFunctionProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, ppfuncprov: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumFunctionProviders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetGlobalCompartment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcompmgr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ActivateEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptid: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub GetActiveFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows::core::HRESULT,
    pub SuspendKeystrokeHandling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ResumeKeystrokeHandling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfThreadMgrEventSink(::windows::core::IUnknown);
impl ITfThreadMgrEventSink {
    pub unsafe fn OnInitDocumentMgr<P0>(&self, pdim: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfDocumentMgr>,
    {
        (::windows::core::Interface::vtable(self).OnInitDocumentMgr)(::windows::core::Interface::as_raw(self), pdim.into_param().abi()).ok()
    }
    pub unsafe fn OnUninitDocumentMgr<P0>(&self, pdim: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfDocumentMgr>,
    {
        (::windows::core::Interface::vtable(self).OnUninitDocumentMgr)(::windows::core::Interface::as_raw(self), pdim.into_param().abi()).ok()
    }
    pub unsafe fn OnSetFocus<P0, P1>(&self, pdimfocus: P0, pdimprevfocus: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfDocumentMgr>,
        P1: ::windows::core::IntoParam<ITfDocumentMgr>,
    {
        (::windows::core::Interface::vtable(self).OnSetFocus)(::windows::core::Interface::as_raw(self), pdimfocus.into_param().abi(), pdimprevfocus.into_param().abi()).ok()
    }
    pub unsafe fn OnPushContext<P0>(&self, pic: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfContext>,
    {
        (::windows::core::Interface::vtable(self).OnPushContext)(::windows::core::Interface::as_raw(self), pic.into_param().abi()).ok()
    }
    pub unsafe fn OnPopContext<P0>(&self, pic: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfContext>,
    {
        (::windows::core::Interface::vtable(self).OnPopContext)(::windows::core::Interface::as_raw(self), pic.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfThreadMgrEventSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfThreadMgrEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfThreadMgrEventSink {}
impl ::core::fmt::Debug for ITfThreadMgrEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfThreadMgrEventSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfThreadMgrEventSink {
    type Vtable = ITfThreadMgrEventSink_Vtbl;
}
impl ::core::clone::Clone for ITfThreadMgrEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfThreadMgrEventSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa80e80e_2021_11d2_93e0_0060b067b86e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfThreadMgrEventSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnInitDocumentMgr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdim: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnUninitDocumentMgr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdim: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnSetFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdimfocus: *mut ::core::ffi::c_void, pdimprevfocus: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnPushContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnPopContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfThreadMgrEx(::windows::core::IUnknown);
impl ITfThreadMgrEx {
    pub unsafe fn Activate(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.Activate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Deactivate)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CreateDocumentMgr(&self) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__ = ::windows::core::zeroed::<ITfDocumentMgr>();
        (::windows::core::Interface::vtable(self).base__.CreateDocumentMgr)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDocumentMgrs(&self) -> ::windows::core::Result<IEnumTfDocumentMgrs> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfDocumentMgrs>();
        (::windows::core::Interface::vtable(self).base__.EnumDocumentMgrs)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFocus(&self) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__ = ::windows::core::zeroed::<ITfDocumentMgr>();
        (::windows::core::Interface::vtable(self).base__.GetFocus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFocus<P0>(&self, pdimfocus: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfDocumentMgr>,
    {
        (::windows::core::Interface::vtable(self).base__.SetFocus)(::windows::core::Interface::as_raw(self), pdimfocus.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AssociateFocus<P0, P1>(&self, hwnd: P0, pdimnew: P1) -> ::windows::core::Result<ITfDocumentMgr>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<ITfDocumentMgr>,
    {
        let mut result__ = ::windows::core::zeroed::<ITfDocumentMgr>();
        (::windows::core::Interface::vtable(self).base__.AssociateFocus)(::windows::core::Interface::as_raw(self), hwnd.into_param().abi(), pdimnew.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsThreadFocus(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsThreadFocus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFunctionProvider(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<ITfFunctionProvider> {
        let mut result__ = ::windows::core::zeroed::<ITfFunctionProvider>();
        (::windows::core::Interface::vtable(self).base__.GetFunctionProvider)(::windows::core::Interface::as_raw(self), clsid, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumFunctionProviders(&self) -> ::windows::core::Result<IEnumTfFunctionProviders> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfFunctionProviders>();
        (::windows::core::Interface::vtable(self).base__.EnumFunctionProviders)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGlobalCompartment(&self) -> ::windows::core::Result<ITfCompartmentMgr> {
        let mut result__ = ::windows::core::zeroed::<ITfCompartmentMgr>();
        (::windows::core::Interface::vtable(self).base__.GetGlobalCompartment)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ActivateEx(&self, ptid: *mut u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ActivateEx)(::windows::core::Interface::as_raw(self), ptid, dwflags).ok()
    }
    pub unsafe fn GetActiveFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetActiveFlags)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfThreadMgrEx, ::windows::core::IUnknown, ITfThreadMgr);
impl ::core::cmp::PartialEq for ITfThreadMgrEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfThreadMgrEx {}
impl ::core::fmt::Debug for ITfThreadMgrEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfThreadMgrEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfThreadMgrEx {
    type Vtable = ITfThreadMgrEx_Vtbl;
}
impl ::core::clone::Clone for ITfThreadMgrEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfThreadMgrEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e90ade3_7594_4cb0_bb58_69628f5f458c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfThreadMgrEx_Vtbl {
    pub base__: ITfThreadMgr_Vtbl,
    pub ActivateEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptid: *mut u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub GetActiveFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfToolTipUIElement(::windows::core::IUnknown);
impl ITfToolTipUIElement {
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDescription)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetGUID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, bshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.Show)(::windows::core::Interface::as_raw(self), bshow.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsShown(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsShown)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetString)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfToolTipUIElement, ::windows::core::IUnknown, ITfUIElement);
impl ::core::cmp::PartialEq for ITfToolTipUIElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfToolTipUIElement {}
impl ::core::fmt::Debug for ITfToolTipUIElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfToolTipUIElement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfToolTipUIElement {
    type Vtable = ITfToolTipUIElement_Vtbl;
}
impl ::core::clone::Clone for ITfToolTipUIElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfToolTipUIElement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52b18b5c_555d_46b2_b00a_fa680144fbdb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfToolTipUIElement_Vtbl {
    pub base__: ITfUIElement_Vtbl,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfTransitoryExtensionSink(::windows::core::IUnknown);
impl ITfTransitoryExtensionSink {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnTransitoryExtensionUpdated<P0, P1, P2>(&self, pic: P0, ecreadonly: u32, presultrange: P1, pcompositionrange: P2) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<ITfContext>,
        P1: ::windows::core::IntoParam<ITfRange>,
        P2: ::windows::core::IntoParam<ITfRange>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).OnTransitoryExtensionUpdated)(::windows::core::Interface::as_raw(self), pic.into_param().abi(), ecreadonly, presultrange.into_param().abi(), pcompositionrange.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfTransitoryExtensionSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfTransitoryExtensionSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfTransitoryExtensionSink {}
impl ::core::fmt::Debug for ITfTransitoryExtensionSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfTransitoryExtensionSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfTransitoryExtensionSink {
    type Vtable = ITfTransitoryExtensionSink_Vtbl;
}
impl ::core::clone::Clone for ITfTransitoryExtensionSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfTransitoryExtensionSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa615096f_1c57_4813_8a15_55ee6e5a839c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTransitoryExtensionSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnTransitoryExtensionUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, ecreadonly: u32, presultrange: *mut ::core::ffi::c_void, pcompositionrange: *mut ::core::ffi::c_void, pfdeleteresultrange: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnTransitoryExtensionUpdated: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfTransitoryExtensionUIElement(::windows::core::IUnknown);
impl ITfTransitoryExtensionUIElement {
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).base__.GetDescription)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetGUID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, bshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.Show)(::windows::core::Interface::as_raw(self), bshow.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsShown(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.IsShown)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDocumentMgr(&self) -> ::windows::core::Result<ITfDocumentMgr> {
        let mut result__ = ::windows::core::zeroed::<ITfDocumentMgr>();
        (::windows::core::Interface::vtable(self).GetDocumentMgr)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfTransitoryExtensionUIElement, ::windows::core::IUnknown, ITfUIElement);
impl ::core::cmp::PartialEq for ITfTransitoryExtensionUIElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfTransitoryExtensionUIElement {}
impl ::core::fmt::Debug for ITfTransitoryExtensionUIElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfTransitoryExtensionUIElement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfTransitoryExtensionUIElement {
    type Vtable = ITfTransitoryExtensionUIElement_Vtbl;
}
impl ::core::clone::Clone for ITfTransitoryExtensionUIElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfTransitoryExtensionUIElement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x858f956a_972f_42a2_a2f2_0321e1abe209);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfTransitoryExtensionUIElement_Vtbl {
    pub base__: ITfUIElement_Vtbl,
    pub GetDocumentMgr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdim: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfUIElement(::windows::core::IUnknown);
impl ITfUIElement {
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetDescription)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetGUID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<P0>(&self, bshow: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Show)(::windows::core::Interface::as_raw(self), bshow.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsShown(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsShown)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfUIElement, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfUIElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfUIElement {}
impl ::core::fmt::Debug for ITfUIElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfUIElement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfUIElement {
    type Vtable = ITfUIElement_Vtbl;
}
impl ::core::clone::Clone for ITfUIElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfUIElement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea1ea137_19df_11d7_a6d2_00065b84435c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfUIElement_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetGUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bshow: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Show: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsShown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsShown: usize,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfUIElementMgr(::windows::core::IUnknown);
impl ITfUIElementMgr {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginUIElement<P0>(&self, pelement: P0, pbshow: *mut super::super::Foundation::BOOL, pdwuielementid: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ITfUIElement>,
    {
        (::windows::core::Interface::vtable(self).BeginUIElement)(::windows::core::Interface::as_raw(self), pelement.into_param().abi(), pbshow, pdwuielementid).ok()
    }
    pub unsafe fn UpdateUIElement(&self, dwuielementid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateUIElement)(::windows::core::Interface::as_raw(self), dwuielementid).ok()
    }
    pub unsafe fn EndUIElement(&self, dwuielementid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndUIElement)(::windows::core::Interface::as_raw(self), dwuielementid).ok()
    }
    pub unsafe fn GetUIElement(&self, dwuielementid: u32) -> ::windows::core::Result<ITfUIElement> {
        let mut result__ = ::windows::core::zeroed::<ITfUIElement>();
        (::windows::core::Interface::vtable(self).GetUIElement)(::windows::core::Interface::as_raw(self), dwuielementid, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumUIElements(&self) -> ::windows::core::Result<IEnumTfUIElements> {
        let mut result__ = ::windows::core::zeroed::<IEnumTfUIElements>();
        (::windows::core::Interface::vtable(self).EnumUIElements)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ITfUIElementMgr, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfUIElementMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfUIElementMgr {}
impl ::core::fmt::Debug for ITfUIElementMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfUIElementMgr").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfUIElementMgr {
    type Vtable = ITfUIElementMgr_Vtbl;
}
impl ::core::clone::Clone for ITfUIElementMgr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfUIElementMgr {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea1ea135_19df_11d7_a6d2_00065b84435c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfUIElementMgr_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginUIElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pelement: *mut ::core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL, pdwuielementid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginUIElement: usize,
    pub UpdateUIElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows::core::HRESULT,
    pub EndUIElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows::core::HRESULT,
    pub GetUIElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwuielementid: u32, ppelement: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumUIElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct ITfUIElementSink(::windows::core::IUnknown);
impl ITfUIElementSink {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginUIElement(&self, dwuielementid: u32, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginUIElement)(::windows::core::Interface::as_raw(self), dwuielementid, pbshow).ok()
    }
    pub unsafe fn UpdateUIElement(&self, dwuielementid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateUIElement)(::windows::core::Interface::as_raw(self), dwuielementid).ok()
    }
    pub unsafe fn EndUIElement(&self, dwuielementid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndUIElement)(::windows::core::Interface::as_raw(self), dwuielementid).ok()
    }
}
::windows::imp::interface_hierarchy!(ITfUIElementSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ITfUIElementSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITfUIElementSink {}
impl ::core::fmt::Debug for ITfUIElementSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITfUIElementSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ITfUIElementSink {
    type Vtable = ITfUIElementSink_Vtbl;
}
impl ::core::clone::Clone for ITfUIElementSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITfUIElementSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea1ea136_19df_11d7_a6d2_00065b84435c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITfUIElementSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginUIElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwuielementid: u32, pbshow: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginUIElement: usize,
    pub UpdateUIElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows::core::HRESULT,
    pub EndUIElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IUIManagerEventSink(::windows::core::IUnknown);
impl IUIManagerEventSink {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnWindowOpening(&self, prcbounds: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnWindowOpening)(::windows::core::Interface::as_raw(self), prcbounds).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnWindowOpened(&self, prcbounds: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnWindowOpened)(::windows::core::Interface::as_raw(self), prcbounds).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnWindowUpdating(&self, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnWindowUpdating)(::windows::core::Interface::as_raw(self), prcupdatedbounds).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnWindowUpdated(&self, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnWindowUpdated)(::windows::core::Interface::as_raw(self), prcupdatedbounds).ok()
    }
    pub unsafe fn OnWindowClosing(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnWindowClosing)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn OnWindowClosed(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnWindowClosed)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IUIManagerEventSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IUIManagerEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUIManagerEventSink {}
impl ::core::fmt::Debug for IUIManagerEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUIManagerEventSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IUIManagerEventSink {
    type Vtable = IUIManagerEventSink_Vtbl;
}
impl ::core::clone::Clone for IUIManagerEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUIManagerEventSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd91d690_a7e8_4265_9b38_8bb3bbaba7de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIManagerEventSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnWindowOpening: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prcbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnWindowOpening: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnWindowOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prcbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnWindowOpened: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnWindowUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnWindowUpdating: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OnWindowUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnWindowUpdated: usize,
    pub OnWindowClosing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnWindowClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
pub struct IVersionInfo(::windows::core::IUnknown);
impl IVersionInfo {
    pub unsafe fn GetSubcomponentCount(&self, ulsub: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSubcomponentCount)(::windows::core::Interface::as_raw(self), ulsub, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetImplementationID(&self, ulsub: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetImplementationID)(::windows::core::Interface::as_raw(self), ulsub, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBuildVersion(&self, ulsub: u32, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBuildVersion)(::windows::core::Interface::as_raw(self), ulsub, pdwmajor, pdwminor).ok()
    }
    pub unsafe fn GetComponentDescription(&self, ulsub: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetComponentDescription)(::windows::core::Interface::as_raw(self), ulsub, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetInstanceDescription(&self, ulsub: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetInstanceDescription)(::windows::core::Interface::as_raw(self), ulsub, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IVersionInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IVersionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVersionInfo {}
impl ::core::fmt::Debug for IVersionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVersionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVersionInfo {
    type Vtable = IVersionInfo_Vtbl;
}
impl ::core::clone::Clone for IVersionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVersionInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x401518ec_db00_4611_9b29_2a0e4b9afa85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVersionInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSubcomponentCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulsub: u32, ulcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetImplementationID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulsub: u32, implid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetBuildVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulsub: u32, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows::core::HRESULT,
    pub GetComponentDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulsub: u32, pimplstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetInstanceDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulsub: u32, pimplstr: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const AccClientDocMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc48cc30_4f3e_4fa1_803b_ad0e196a83b1);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const AccDictionary: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6572ee16_5fe5_4331_bb6d_76a49c56e423);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const AccServerDocMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6089a37e_eb8a_482d_bd6f_f9f46904d16d);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const AccStore: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5440837f_4bff_4ae5_a1b1_7722ecc6332a);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const CLSID_TF_CategoryMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4b544a1_438d_4b41_9325_869523e2d6c7);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const CLSID_TF_ClassicLangBar: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3318360c_1afc_4d09_a86b_9f9cb6dceb9c);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const CLSID_TF_DisplayAttributeMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ce74de4_53d3_4d74_8b83_431b3828ba53);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const CLSID_TF_InputProcessorProfiles: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33c53a50_f456_4884_b049_85fd643ecfed);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const CLSID_TF_LangBarItemMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9931692_a2b3_4fab_bf33_9ec6f9fb96ac);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const CLSID_TF_LangBarMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebb08c45_6c4a_4fdc_ae53_4eb8c4c7db8e);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const CLSID_TF_ThreadMgr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x529a9e6b_6587_4f23_ab9e_9c7d683e3c50);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const CLSID_TF_TransitoryExtensionUIEntry: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae6be008_07fb_400d_8beb_337a64f7051f);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const CLSID_TsfServices: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39aedc00_6b60_46db_8d31_3642be0e4373);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const DCM_FLAGS_CTFMON: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const DCM_FLAGS_LOCALTHREADTSF: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const DCM_FLAGS_TASKENG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const DocWrap: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf426f7e_7a5e_44d6_830c_a390ea9462a3);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_APP_FUNCTIONPROVIDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4caef01e_12af_4b0e_9db1_a6ec5b881208);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_COMPARTMENT_CONVERSIONMODEBIAS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5497f516_ee91_436e_b946_aa2c05f1ac5b);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_COMPARTMENT_EMPTYCONTEXT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7487dbf_804e_41c5_894d_ad96fd4eea13);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_COMPARTMENT_ENABLED_PROFILES_UPDATED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92c1fd48_a9ae_4a7c_be08_4329e4723817);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_COMPARTMENT_HANDWRITING_OPENCLOSE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9ae2c6b_1866_4361_af72_7aa30948890e);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_COMPARTMENT_KEYBOARD_DISABLED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71a5b253_1951_466b_9fbc_9c8808fa84f2);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_COMPARTMENT_KEYBOARD_INPUTMODE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6592511_bcee_4122_a7c4_09f4b3fa4396);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_COMPARTMENT_KEYBOARD_INPUTMODE_CONVERSION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccf05dd8_4a87_11d7_a6e2_00065b84435c);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_COMPARTMENT_KEYBOARD_INPUTMODE_SENTENCE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccf05dd9_4a87_11d7_a6e2_00065b84435c);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_COMPARTMENT_KEYBOARD_OPENCLOSE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58273aad_01bb_4164_95c6_755ba0b5162d);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_COMPARTMENT_SAPI_AUDIO: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51af2086_cc6b_457d_b5aa_8b19dc290ab4);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_COMPARTMENT_SPEECH_CFGMENU: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb6c5c2d_4e83_4bb6_91a2_e019bff6762d);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_COMPARTMENT_SPEECH_DISABLED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56c5c607_0703_4e59_8e52_cbc84e8bbe35);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_COMPARTMENT_SPEECH_GLOBALSTATE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a54fe8e_0d08_460c_a75d_87035ff436c5);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_COMPARTMENT_SPEECH_OPENCLOSE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x544d6a63_e2e8_4752_bbd1_000960bca083);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_COMPARTMENT_SPEECH_UI_STATUS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd92016f0_9367_4fe7_9abf_bc59dacbe0e3);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_COMPARTMENT_TIPUISTATUS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x148ca3ec_0366_401c_8d75_ed978d85fbc9);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_COMPARTMENT_TRANSITORYEXTENSION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8be347f5_c7a0_11d7_b408_00065b84435c);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_COMPARTMENT_TRANSITORYEXTENSION_DOCUMENTMANAGER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8be347f7_c7a0_11d7_b408_00065b84435c);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_COMPARTMENT_TRANSITORYEXTENSION_PARENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8be347f8_c7a0_11d7_b408_00065b84435c);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_INTEGRATIONSTYLE_SEARCHBOX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6d1bd11_82f7_4903_ae21_1a6397cde2eb);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_LBI_INPUTMODE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c77a81e_41cc_4178_a3a7_5f8a987568e6);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_LBI_SAPILAYR_CFGMENUBUTTON: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd02f24a1_942d_422e_8d99_b4f2addee999);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_MODEBIAS_CHINESE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7add26de_4328_489b_83ae_6493750cad5c);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_MODEBIAS_CONVERSATION: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f4ec104_1790_443b_95f1_e10f939d6546);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_MODEBIAS_DATETIME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2bdb372_7f61_4039_92ef_1c35599f0222);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_MODEBIAS_FILENAME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7f707fe_44c6_4fca_8e76_86ab50c7931b);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_MODEBIAS_FULLWIDTHALPHANUMERIC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81489fb8_b36a_473d_8146_e4a2258b24ae);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_MODEBIAS_FULLWIDTHHANGUL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc01ae6c9_45b5_4fd0_9cb1_9f4cebc39fea);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_MODEBIAS_HALFWIDTHKATAKANA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x005f6b63_78d4_41cc_8859_485ca821a795);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_MODEBIAS_HANGUL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76ef0541_23b3_4d77_a074_691801ccea17);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_MODEBIAS_HIRAGANA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd73d316e_9b91_46f1_a280_31597f52c694);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_MODEBIAS_KATAKANA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e0eeddd_3a1a_499e_8543_3c7ee7949811);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_MODEBIAS_NAME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfddc10f0_d239_49bf_b8fc_5410caaa427e);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_MODEBIAS_NONE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_MODEBIAS_NUMERIC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4021766c_e872_48fd_9cee_4ec5c75e16c3);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_MODEBIAS_READING: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe31643a3_6466_4cbf_8d8b_0bd4d8545461);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_MODEBIAS_URLHISTORY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b0e54d9_63f2_4c68_84d4_79aee7a59f09);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_PROP_ATTRIBUTE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34b45670_7526_11d2_a147_00105a2799b5);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_PROP_COMPOSING: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe12ac060_af15_11d2_afc5_00105a2799b5);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_PROP_INPUTSCOPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1713dd5a_68e7_4a5b_9af6_592a595c778d);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_PROP_LANGID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3280ce20_8032_11d2_b603_00105a2799b5);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_PROP_MODEBIAS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x372e0716_974f_40ac_a088_08cdc92ebfbc);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_PROP_READING: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5463f7c0_8e31_11d2_bf46_00105a2799b5);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_PROP_TEXTOWNER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1e2d520_0969_11d3_8df0_00105a2799b5);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_PROP_TKB_ALTERNATES: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70b2a803_968d_462e_b93b_2164c91517f7);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_SYSTEM_FUNCTIONPROVIDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a698bb0_0f21_11d3_8df1_00105a2799b5);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TFCAT_CATEGORY_OF_TIP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x534c48c1_0607_4098_a521_4fc899c73e90);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TFCAT_DISPLAYATTRIBUTEPROPERTY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb95f181b_ea4c_4af1_8056_7c321abbb091);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TFCAT_DISPLAYATTRIBUTEPROVIDER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x046b8c80_1647_40f7_9b21_b93b81aabc1b);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TFCAT_PROPSTYLE_STATIC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x565fb8d8_6bd4_4ca1_b223_0f2ccb8f4f96);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TFCAT_PROP_AUDIODATA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7be3a9_e8ab_4d47_a8fe_254fa423436d);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TFCAT_PROP_INKDATA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c6a82ae_b0d7_4f14_a745_14f28b009d61);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TFCAT_TIPCAP_COMLESS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x364215d9_75bc_11d7_a6ef_00065b84435c);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TFCAT_TIPCAP_DUALMODE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3af314a2_d79f_4b1b_9992_15086d339b05);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TFCAT_TIPCAP_IMMERSIVEONLY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3a4259ac_640d_4ad4_89f7_1eb67e7c4ee8);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TFCAT_TIPCAP_IMMERSIVESUPPORT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x13a016df_560b_46cd_947a_4c3af1e0e35d);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TFCAT_TIPCAP_INPUTMODECOMPARTMENT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccf05dd7_4a87_11d7_a6e2_00065b84435c);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TFCAT_TIPCAP_LOCALSERVER: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74769ee9_4a66_4f9d_90d6_bf8b7c3eb461);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TFCAT_TIPCAP_SECUREMODE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49d2f9ce_1f5e_11d7_a6d3_00065b84435c);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TFCAT_TIPCAP_SYSTRAYSUPPORT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25504fb4_7bab_4bc1_9c69_cf81890f0ef5);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TFCAT_TIPCAP_TSF3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07dcb4af_98de_4548_bef7_25bd45979a1f);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TFCAT_TIPCAP_UIELEMENTENABLED: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49d2f9cf_1f5e_11d7_a6d3_00065b84435c);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TFCAT_TIPCAP_WOW16: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x364215da_75bc_11d7_a6ef_00065b84435c);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TFCAT_TIP_HANDWRITING: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x246ecb87_c2f2_4abe_905b_c8b38add2c43);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TFCAT_TIP_KEYBOARD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34745c63_b2f0_4784_8b67_5e12c8701a31);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TFCAT_TIP_SPEECH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5a73cd1_8355_426b_a161_259808f26b14);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TFCAT_TRANSITORYEXTENSIONUI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6302de22_a5cf_4b02_bfe8_4d72b2bed3c6);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TS_SERVICE_ACCESSIBLE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9786200_a5bf_4a0f_8c24_fb16f5d1aabb);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TS_SERVICE_ACTIVEX: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea937a50_c9a6_4b7d_894a_49d99b784834);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GUID_TS_SERVICE_DATAOBJECT: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6086fbb5_e225_46ce_a770_c1bbd3e05d7b);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GXFPF_NEAREST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GXFPF_ROUND_NEAREST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const ILMCM_CHECKLAYOUTANDTIPENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const ILMCM_LANGUAGEBAROFF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const LIBID_MSAATEXTLib: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x150e2d7a_dac1_4582_947d_2a8fd78b82cd);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const MSAAControl: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08cd963f_7a3e_4f5c_9bd8_d692bb043c5b);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CHAR_EMBEDDED: u32 = 65532u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CLUIE_COUNT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CLUIE_CURRENTPAGE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CLUIE_DOCUMENTMGR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CLUIE_PAGEINDEX: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CLUIE_SELECTION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CLUIE_STRING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_COMMANDING_ENABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_COMMANDING_ON: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CONVERSIONMODE_ALPHANUMERIC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CONVERSIONMODE_CHARCODE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CONVERSIONMODE_EUDC: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CONVERSIONMODE_FIXED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CONVERSIONMODE_FULLSHAPE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CONVERSIONMODE_KATAKANA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CONVERSIONMODE_NATIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CONVERSIONMODE_NOCONVERSION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CONVERSIONMODE_ROMAN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CONVERSIONMODE_SOFTKEYBOARD: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CONVERSIONMODE_SYMBOL: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_DEFAULT_SELECTION: u64 = 18446744073709551615u64;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_DICTATION_ENABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_DICTATION_ON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_DISABLE_BALLOON: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_DISABLE_COMMANDING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_DISABLE_DICTATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_DISABLE_SPEECH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ENABLE_PROCESS_ATOM: ::windows::core::PCWSTR = ::windows::core::w!("_CTF_ENABLE_PROCESS_ATOM_");
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220218i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_COMPOSITION_REJECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220216i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_DISCONNECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220220i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_EMPTYCONTEXT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220215i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220982i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_INVALIDPOINT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220985i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_INVALIDPOS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220992i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_INVALIDVIEW: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220219i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_LOCKED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220224i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_NOCONVERSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147219968i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_NOINTERFACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220988i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_NOLAYOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220986i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_NOLOCK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220991i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_NOOBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220990i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_NOPROVIDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220221i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_NOSELECTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220987i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_NOSERVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220989i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_NOTOWNEDRANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220222i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_RANGE_NOT_COVERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220217i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_READONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220983i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_STACKFULL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220223i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_E_SYNCHRONOUS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220984i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_FLOATINGLANGBAR_WNDTITLE: ::windows::core::PCWSTR = ::windows::core::w!("TF_FloatingLangBar_WndTitle");
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_FLOATINGLANGBAR_WNDTITLEA: ::windows::core::PCSTR = ::windows::core::s!("TF_FloatingLangBar_WndTitle");
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_FLOATINGLANGBAR_WNDTITLEW: ::windows::core::PCWSTR = ::windows::core::w!("TF_FloatingLangBar_WndTitle");
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_HF_OBJECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IE_CORRECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_INVALID_COOKIE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_INVALID_EDIT_COOKIE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPPMF_DISABLEPROFILE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPPMF_DONTCARECURRENTINPUTLANGUAGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPPMF_ENABLEPROFILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPPMF_FORPROCESS: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPPMF_FORSESSION: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPPMF_FORSYSTEMALL: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPP_CAPS_COMLESSSUPPORT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPP_CAPS_DISABLEONTRANSITORY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPP_CAPS_IMMERSIVESUPPORT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPP_CAPS_SECUREMODESUPPORT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPP_CAPS_SYSTRAYSUPPORT: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPP_CAPS_UIELEMENTENABLED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPP_CAPS_WOW16SUPPORT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPP_FLAG_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPP_FLAG_ENABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPP_FLAG_SUBSTITUTEDBYINPUTPROCESSOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IPSINK_FLAG_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_BALLOON: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_BITMAP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_BMPF_VERTICAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_CUSTOMUI: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_DESC_MAXLEN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_ICON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STATUS: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STATUS_BTN_TOGGLED: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STATUS_DISABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STATUS_HIDDEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STYLE_BTN_BUTTON: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STYLE_BTN_MENU: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STYLE_BTN_TOGGLE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STYLE_HIDDENBYDEFAULT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STYLE_HIDDENSTATUSCONTROL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STYLE_HIDEONNOOTHERITEMS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STYLE_SHOWNINTRAY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STYLE_SHOWNINTRAYONLY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_STYLE_TEXTCOLORICON: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_TEXT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_TOOLTIP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBMENUF_CHECKED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBMENUF_GRAYED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBMENUF_RADIOCHECKED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBMENUF_SEPARATOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBMENUF_SUBMENU: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MENUREADY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MOD_ALT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MOD_CONTROL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MOD_IGNORE_ALL_MODIFIER: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MOD_LALT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MOD_LCONTROL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MOD_LSHIFT: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MOD_ON_KEYUP: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MOD_RALT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MOD_RCONTROL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MOD_RSHIFT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_MOD_SHIFT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_POPF_ALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_PROCESS_ATOM: ::windows::core::PCWSTR = ::windows::core::w!("_CTF_PROCESS_ATOM_");
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_PROFILETYPE_INPUTPROCESSOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_PROFILETYPE_KEYBOARDLAYOUT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_PROFILE_ARRAY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd38eff65_aa46_4fd5_91a7_67845fb02f5b);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_PROFILE_CANTONESE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0aec109c_7e96_11d4_b2ef_0080c882687e);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_PROFILE_CHANGJIE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bdf9f03_c7d3_11d4_b2ab_0080c882687e);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_PROFILE_DAYI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x037b2c25_480c_4d7f_b027_d6ca6b69788a);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_PROFILE_NEWCHANGJIE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3ba907a_6c7e_11d4_97fa_0080c882687e);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_PROFILE_NEWPHONETIC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2f9c502_1742_11d4_9790_0080c882687e);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_PROFILE_NEWQUICK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b883ba0_c1c7_11d4_87f9_0080c882687e);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_PROFILE_PHONETIC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x761309de_317a_11d4_9b5d_0080c882687e);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_PROFILE_PINYIN: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3ba9077_6c7e_11d4_97fa_0080c882687e);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_PROFILE_QUICK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6024b45f_5c54_11d4_b921_0080c882687e);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_PROFILE_SIMPLEFAST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa550b04_5ad7_411f_a5ac_ca038ec515d7);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_PROFILE_TIGRINYA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3cab88b7_cc3e_46a6_9765_b772ad7761ff);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_PROFILE_WUBI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82590c13_f4dd_44f4_ba1d_8667246fdf8e);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_PROFILE_YI: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x409c8376_007b_4357_ae8e_26316ee3fb0d);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_PROPUI_STATUS_SAVETOFILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RCM_COMLESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RCM_HINT_COLLISION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RCM_HINT_READING_LENGTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RCM_VKEY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RIP_FLAG_FREEUNUSEDLIBRARIES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RIUIE_CONTEXT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RIUIE_ERRORINDEX: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RIUIE_MAXREADINGSTRINGLENGTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RIUIE_STRING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RIUIE_VERTICALORDER: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RP_HIDDENINSETTINGUI: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RP_LOCALPROCESS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RP_LOCALTHREAD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_RP_SUBITEMINSETTINGUI: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SD_LOADING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SD_READONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SENTENCEMODE_AUTOMATIC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SENTENCEMODE_CONVERSATION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SENTENCEMODE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SENTENCEMODE_PHRASEPREDICT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SENTENCEMODE_PLAURALCLAUSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SENTENCEMODE_SINGLECONVERT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_DESKBAND: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_DOCK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_EXTRAICONSONMINIMIZED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_HIDDEN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_HIGHTRANSPARENCY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_LABELS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_LOWTRANSPARENCY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_MINIMIZED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_NOEXTRAICONSONMINIMIZED: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_NOLABELS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_NOTRANSPARENCY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SFT_SHOWNORMAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SHOW_BALLOON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SPEECHUI_SHOWN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SS_DISJOINTSEL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SS_REGIONS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SS_TKBAUTOCORRECTENABLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SS_TKBPREDICTIONENABLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SS_TRANSITORY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ST_CORRECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_S_ASYNC: ::windows::core::HRESULT = ::windows::core::HRESULT(262912i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TF_IGNOREEND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TF_MOVESTART: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMAE_COMLESS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMAE_CONSOLE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMAE_NOACTIVATEKEYBOARDLAYOUT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMAE_NOACTIVATETIP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMAE_SECUREMODE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMAE_UIELEMENTENABLEDONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMAE_WOW16: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMF_ACTIVATED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMF_COMLESS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMF_CONSOLE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMF_IMMERSIVEMODE: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMF_NOACTIVATETIP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMF_SECUREMODE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMF_UIELEMENTENABLEDONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TMF_WOW16: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TRANSITORYEXTENSION_ATSELECTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TRANSITORYEXTENSION_FLOATING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TRANSITORYEXTENSION_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_TU_CORRECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_URP_ALLPROFILES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_URP_LOCALPROCESS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_URP_LOCALTHREAD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_US_HIDETIPUI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKBL_CLASSIC_TRADITIONAL_CHINESE_CHANGJIE: u32 = 61506u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKBL_CLASSIC_TRADITIONAL_CHINESE_DAYI: u32 = 61507u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKBL_CLASSIC_TRADITIONAL_CHINESE_PHONETIC: u32 = 1028u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKBL_OPT_JAPANESE_ABC: u32 = 1041u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKBL_OPT_KOREAN_HANGUL_2_BULSIK: u32 = 1042u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKBL_OPT_SIMPLIFIED_CHINESE_PINYIN: u32 = 2052u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKBL_OPT_TRADITIONAL_CHINESE_PHONETIC: u32 = 1028u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKBL_UNDEFINED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKB_ALTERNATES_AUTOCORRECTION_APPLIED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKB_ALTERNATES_FOR_AUTOCORRECTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKB_ALTERNATES_FOR_PREDICTION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKB_ALTERNATES_STANDARD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_App: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa80f77df_4237_40e5_849c_b5fa51c13ac7);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_App_IncorrectGrammar: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd54e398_ad03_4b74_b6b3_5edb19996388);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_App_IncorrectSpelling: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf42de43c_ef12_430d_944c_9a08970a25d2);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x573ea825_749b_4f8a_9cfd_21c3605ca828);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_FaceName: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb536aeb6_053b_4eb8_b65a_50da1e81e72e);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_SizePts: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8493302_a5e9_456d_af04_8005e4130f03);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68b2a77f_6b0e_4f28_8177_571c2f3a42b1);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Animation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcf73d22_e029_47b7_bb36_f263a3d004cc);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Animation_BlinkingBackground: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86e5b104_0104_4b10_b585_00f2527522b5);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Animation_LasVegasLights: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf40423d5_0f87_4f8f_bada_e6d60c25e152);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Animation_MarchingBlackAnts: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7644e067_f186_4902_bfc6_ec815aa20e9d);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Animation_MarchingRedAnts: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78368dad_50fb_4c6f_840b_d486bb6cf781);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Animation_Shimmer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ce31b58_5293_4c36_8809_bf8bb51a27b3);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Animation_SparkleText: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x533aad20_962c_4e9f_8c09_b42ea4749711);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Animation_WipeDown: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5872e874_367b_4803_b160_c90ff62569d0);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Animation_WipeRight: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb855cbe3_3d2c_4600_b1e9_e1c9ce02f842);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_BackgroundColor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb50eaa4e_3091_4468_81db_d79ea190c7c7);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Blink: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfb2c036_7acf_4532_b720_b416dd7765a8);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Bold: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48813a43_8a20_4940_8e58_97823f7b268a);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Capitalize: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d85a3ba_b4fd_43b3_befc_6b985c843141);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Color: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x857a7a37_b8af_4e9a_81b4_acf700c8411b);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Emboss: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd8ed742_349e_4e37_82fb_437979cb53a7);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Engrave: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c3371de_8332_4897_be5d_89233223179a);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Height: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e937477_12e6_458b_926a_1fa44ee8f391);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Hidden: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1e28770_881c_475f_863f_887a647b1090);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Italic: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8740682a_a765_48e1_acfc_d22222b2f810);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Kerning: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc26e1b4_2f9a_47c8_8bff_bf1eb7cce0dd);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Lowercase: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76d8ccb5_ca7b_4498_8ee9_d5c4f6f74c60);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Outlined: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10e6db31_db0d_4ac6_a7f5_9c9cff6f2ab4);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Overline: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3989f4a_992b_4301_8ce1_a5b7c6d1f3c8);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Overline_Double: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc46063a_e115_46e3_bcd8_ca6772aa95b4);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Overline_Single: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8440d94c_51ce_47b2_8d4c_15751e5f721b);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Position: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15cd26ab_f2fb_4062_b5a6_9a49e1a5cc0b);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Protected: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c557cb2_14cf_4554_a574_ecb2f7e7efd4);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Shadow: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f686d2f_c6cd_4c56_8a1a_994a4b9766be);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_SmallCaps: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfacb6bc6_9100_4cc6_b969_11eea45a86b4);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Spacing: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98c1200d_8f06_409a_8e49_6a554bf7c153);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Strikethrough: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c562193_2d08_4668_9601_ced41309d7af);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Strikethrough_Double: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62489b31_a3e7_4f94_ac43_ebaf8fcc7a9f);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Strikethrough_Single: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75d736b6_3c8f_4b97_ab78_1877cb990d31);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Subscript: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5774fb84_389b_43bc_a74b_1568347cf0f4);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Superscript: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ea4993c_563c_49aa_9372_0bef09a9255b);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Underline: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3c9c9f3_7902_444b_9a7b_48e70f4b50f7);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Underline_Double: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74d24aa6_1db3_4c69_a176_31120e7586d5);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Underline_Single: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b6720e5_0f73_4951_a6b3_6f19e43c9461);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Uppercase: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33a300e8_e340_4937_b697_8f234045cd9a);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Font_Style_Weight: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12f3189c_8bb0_461b_b1fa_eaf907047fe0);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_List: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x436d673b_26f1_4aee_9e65_8f83a4ed4884);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_List_LevelIndel: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f7cc899_311f_487b_ad5d_e2a459e12d42);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_List_Type: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae3e665e_4bce_49e3_a0fe_2db47d3a17ae);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_List_Type_Arabic: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1338c5d6_98a3_4fa3_9bd1_7a60eef8e9e0);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_List_Type_Bullet: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbccd77c5_4c4d_4ce2_b102_559f3b2bfcea);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_List_Type_LowerLetter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96372285_f3cf_491e_a925_3832347fd237);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_List_Type_LowerRoman: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90466262_3980_4b8e_9368_918bd1218a41);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_List_Type_UpperLetter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7987b7cd_ce52_428b_9b95_a357f6f10c45);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_List_Type_UpperRoman: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f6ab552_4a80_467f_b2f1_127e2aa3ba9e);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_OTHERS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3c32af9_57d0_46a9_bca8_dac238a13057);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7edb8e68_81f9_449d_a15a_87a8388faac0);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_Alignment: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x139941e6_1767_456d_938e_35ba568b5cd4);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_Alignment_Center: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4a95c16_53bf_4d55_8b87_4bdd8d4275fc);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_Alignment_Justify: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed350740_a0f7_42d3_8ea8_f81b6488faf0);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_Alignment_Left: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16ae95d3_6361_43a2_8495_d00f397f1693);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_Alignment_Right: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb36f0f98_1b9e_4360_8616_03fb08a78456);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_EmbeddedObject: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7edb8e68_81f9_449d_a15a_87a8388faac0);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_Hyphenation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdadf4525_618e_49eb_b1a8_3b68bd7648e3);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_Language: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8c04ef1_5753_4c25_8887_85443fe5f819);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_Link: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47cd9051_3722_4cd8_b7c8_4e17ca1759f5);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_Orientation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bab707f_8785_4c39_8b52_96f878303ffb);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_Para: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5edc5822_99dc_4dd6_aec3_b62baa5b2e7c);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_Para_FirstLineIndent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07c97a13_7472_4dd8_90a9_91e3d7e4f29c);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_Para_LeftIndent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb2848e9_7471_41c9_b6b3_8a1450e01897);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_Para_LineSpacing: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x699b380d_7f8c_46d6_a73b_dfe3d1538df3);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_Para_LineSpacing_AtLeast: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xadfedf31_2d44_4434_a5ff_7f4c4990a905);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_Para_LineSpacing_Double: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82fb1805_a6c4_4231_ac12_6260af2aba28);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_Para_LineSpacing_Exactly: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d45ad40_23de_48d7_a6b3_765420c620cc);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_Para_LineSpacing_Multiple: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x910f1e3c_d6d0_4f65_8a3c_42b4b31868c5);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_Para_LineSpacing_OnePtFive: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0428a021_0397_4b57_9a17_0795994cd3c5);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_Para_LineSpacing_Single: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed350740_a0f7_42d3_8ea8_f81b6488faf0);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_Para_RightIndent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c7f26f9_a5e2_48da_b98a_520cb16513bf);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_Para_SpaceAfter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b0a3f55_22dc_425f_a411_93da1d8f9baa);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_Para_SpaceBefore: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8df98589_194a_4601_b251_9865a3e906dd);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_ReadOnly: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85836617_de32_4afd_a50f_a2db110e6e4d);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_RightToLeft: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca666e71_1b08_453d_bfdd_28e08c8aaf7a);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TSATTRID_Text_VerticalWriting: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bba8195_046f_4ea9_b311_97fd66c4274b);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_AS_ATTR_CHANGE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_AS_LAYOUT_CHANGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_AS_SEL_CHANGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_AS_STATUS_CHANGE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_AS_TEXT_CHANGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_ATTR_FIND_BACKWARDS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_ATTR_FIND_HIDDEN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_ATTR_FIND_UPDATESTART: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_ATTR_FIND_WANT_END: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_ATTR_FIND_WANT_OFFSET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_ATTR_FIND_WANT_VALUE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_CHAR_EMBEDDED: u32 = 65532u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_CHAR_REGION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_CHAR_REPLACEMENT: u32 = 65533u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_DEFAULT_SELECTION: u64 = 18446744073709551615u64;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_E_FORMAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220982i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_E_INVALIDPOINT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220985i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_E_INVALIDPOS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220992i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_E_NOINTERFACE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220988i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_E_NOLAYOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220986i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_E_NOLOCK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220991i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_E_NOOBJECT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220990i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_E_NOSELECTION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220987i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_E_NOSERVICE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220989i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_E_READONLY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220983i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_E_SYNCHRONOUS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147220984i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_GEA_HIDDEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_GTA_HIDDEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_IAS_NOQUERY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_IAS_QUERYONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_IE_COMPOSITION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_IE_CORRECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_LF_SYNC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SD_EMBEDDEDHANDWRITINGVIEW_ENABLED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SD_EMBEDDEDHANDWRITINGVIEW_VISIBLE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SD_INPUTPANEMANUALDISPLAYENABLE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SD_LOADING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SD_READONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SD_RESERVED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SD_TKBAUTOCORRECTENABLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SD_TKBPREDICTIONENABLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SD_UIINTEGRATIONENABLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SHIFT_COUNT_HIDDEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SHIFT_COUNT_ONLY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SHIFT_HALT_HIDDEN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SHIFT_HALT_VISIBLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SS_DISJOINTSEL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SS_NOHIDDENTEXT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SS_REGIONS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SS_TKBAUTOCORRECTENABLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SS_TKBPREDICTIONENABLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SS_TRANSITORY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SS_UWPCONTROL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_STRF_END: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_STRF_MID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_STRF_START: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_S_ASYNC: ::windows::core::HRESULT = ::windows::core::HRESULT(262912i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_VCOOKIE_NUL: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ANCHOR_CHANGE_HISTORY_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_CH_PRECEDING_DEL: ANCHOR_CHANGE_HISTORY_FLAGS = ANCHOR_CHANGE_HISTORY_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_CH_FOLLOWING_DEL: ANCHOR_CHANGE_HISTORY_FLAGS = ANCHOR_CHANGE_HISTORY_FLAGS(2u32);
impl ::core::marker::Copy for ANCHOR_CHANGE_HISTORY_FLAGS {}
impl ::core::clone::Clone for ANCHOR_CHANGE_HISTORY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ANCHOR_CHANGE_HISTORY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ANCHOR_CHANGE_HISTORY_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ANCHOR_CHANGE_HISTORY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ANCHOR_CHANGE_HISTORY_FLAGS").field(&self.0).finish()
    }
}
impl ANCHOR_CHANGE_HISTORY_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for ANCHOR_CHANGE_HISTORY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ANCHOR_CHANGE_HISTORY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ANCHOR_CHANGE_HISTORY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ANCHOR_CHANGE_HISTORY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ANCHOR_CHANGE_HISTORY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GET_TEXT_AND_PROPERTY_UPDATES_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_GTP_NONE: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS = GET_TEXT_AND_PROPERTY_UPDATES_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_GTP_INCL_TEXT: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS = GET_TEXT_AND_PROPERTY_UPDATES_FLAGS(1u32);
impl ::core::marker::Copy for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {}
impl ::core::clone::Clone for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_TEXT_AND_PROPERTY_UPDATES_FLAGS").field(&self.0).finish()
    }
}
impl GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GET_TEXT_AND_PROPERTY_UPDATES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INSERT_TEXT_AT_SELECTION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IAS_NOQUERY: INSERT_TEXT_AT_SELECTION_FLAGS = INSERT_TEXT_AT_SELECTION_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IAS_QUERYONLY: INSERT_TEXT_AT_SELECTION_FLAGS = INSERT_TEXT_AT_SELECTION_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_IAS_NO_DEFAULT_COMPOSITION: INSERT_TEXT_AT_SELECTION_FLAGS = INSERT_TEXT_AT_SELECTION_FLAGS(2147483648u32);
impl ::core::marker::Copy for INSERT_TEXT_AT_SELECTION_FLAGS {}
impl ::core::clone::Clone for INSERT_TEXT_AT_SELECTION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSERT_TEXT_AT_SELECTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for INSERT_TEXT_AT_SELECTION_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for INSERT_TEXT_AT_SELECTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSERT_TEXT_AT_SELECTION_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InputScope(pub i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_DEFAULT: InputScope = InputScope(0i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_URL: InputScope = InputScope(1i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_FILE_FULLFILEPATH: InputScope = InputScope(2i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_FILE_FILENAME: InputScope = InputScope(3i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_EMAIL_USERNAME: InputScope = InputScope(4i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_EMAIL_SMTPEMAILADDRESS: InputScope = InputScope(5i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_LOGINNAME: InputScope = InputScope(6i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_PERSONALNAME_FULLNAME: InputScope = InputScope(7i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_PERSONALNAME_PREFIX: InputScope = InputScope(8i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_PERSONALNAME_GIVENNAME: InputScope = InputScope(9i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_PERSONALNAME_MIDDLENAME: InputScope = InputScope(10i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_PERSONALNAME_SURNAME: InputScope = InputScope(11i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_PERSONALNAME_SUFFIX: InputScope = InputScope(12i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ADDRESS_FULLPOSTALADDRESS: InputScope = InputScope(13i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ADDRESS_POSTALCODE: InputScope = InputScope(14i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ADDRESS_STREET: InputScope = InputScope(15i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ADDRESS_STATEORPROVINCE: InputScope = InputScope(16i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ADDRESS_CITY: InputScope = InputScope(17i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ADDRESS_COUNTRYNAME: InputScope = InputScope(18i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ADDRESS_COUNTRYSHORTNAME: InputScope = InputScope(19i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_CURRENCY_AMOUNTANDSYMBOL: InputScope = InputScope(20i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_CURRENCY_AMOUNT: InputScope = InputScope(21i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_DATE_FULLDATE: InputScope = InputScope(22i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_DATE_MONTH: InputScope = InputScope(23i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_DATE_DAY: InputScope = InputScope(24i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_DATE_YEAR: InputScope = InputScope(25i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_DATE_MONTHNAME: InputScope = InputScope(26i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_DATE_DAYNAME: InputScope = InputScope(27i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_DIGITS: InputScope = InputScope(28i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_NUMBER: InputScope = InputScope(29i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ONECHAR: InputScope = InputScope(30i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_PASSWORD: InputScope = InputScope(31i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_TELEPHONE_FULLTELEPHONENUMBER: InputScope = InputScope(32i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_TELEPHONE_COUNTRYCODE: InputScope = InputScope(33i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_TELEPHONE_AREACODE: InputScope = InputScope(34i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_TELEPHONE_LOCALNUMBER: InputScope = InputScope(35i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_TIME_FULLTIME: InputScope = InputScope(36i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_TIME_HOUR: InputScope = InputScope(37i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_TIME_MINORSEC: InputScope = InputScope(38i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_NUMBER_FULLWIDTH: InputScope = InputScope(39i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ALPHANUMERIC_HALFWIDTH: InputScope = InputScope(40i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ALPHANUMERIC_FULLWIDTH: InputScope = InputScope(41i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_CURRENCY_CHINESE: InputScope = InputScope(42i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_BOPOMOFO: InputScope = InputScope(43i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_HIRAGANA: InputScope = InputScope(44i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_KATAKANA_HALFWIDTH: InputScope = InputScope(45i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_KATAKANA_FULLWIDTH: InputScope = InputScope(46i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_HANJA: InputScope = InputScope(47i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_HANGUL_HALFWIDTH: InputScope = InputScope(48i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_HANGUL_FULLWIDTH: InputScope = InputScope(49i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_SEARCH: InputScope = InputScope(50i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_FORMULA: InputScope = InputScope(51i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_SEARCH_INCREMENTAL: InputScope = InputScope(52i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_CHINESE_HALFWIDTH: InputScope = InputScope(53i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_CHINESE_FULLWIDTH: InputScope = InputScope(54i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_NATIVE_SCRIPT: InputScope = InputScope(55i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_YOMI: InputScope = InputScope(56i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_TEXT: InputScope = InputScope(57i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_CHAT: InputScope = InputScope(58i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_NAME_OR_PHONENUMBER: InputScope = InputScope(59i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_EMAILNAME_OR_ADDRESS: InputScope = InputScope(60i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_PRIVATE: InputScope = InputScope(61i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_MAPS: InputScope = InputScope(62i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_NUMERIC_PASSWORD: InputScope = InputScope(63i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_NUMERIC_PIN: InputScope = InputScope(64i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ALPHANUMERIC_PIN: InputScope = InputScope(65i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ALPHANUMERIC_PIN_SET: InputScope = InputScope(66i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_FORMULA_NUMBER: InputScope = InputScope(67i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_CHAT_WITHOUT_EMOJI: InputScope = InputScope(68i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_PHRASELIST: InputScope = InputScope(-1i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_REGULAREXPRESSION: InputScope = InputScope(-2i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_SRGS: InputScope = InputScope(-3i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_XML: InputScope = InputScope(-4i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const IS_ENUMSTRING: InputScope = InputScope(-5i32);
impl ::core::marker::Copy for InputScope {}
impl ::core::clone::Clone for InputScope {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InputScope {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for InputScope {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for InputScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputScope").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LANG_BAR_ITEM_ICON_MODE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_DTLBI_NONE: LANG_BAR_ITEM_ICON_MODE_FLAGS = LANG_BAR_ITEM_ICON_MODE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_DTLBI_USEPROFILEICON: LANG_BAR_ITEM_ICON_MODE_FLAGS = LANG_BAR_ITEM_ICON_MODE_FLAGS(1u32);
impl ::core::marker::Copy for LANG_BAR_ITEM_ICON_MODE_FLAGS {}
impl ::core::clone::Clone for LANG_BAR_ITEM_ICON_MODE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LANG_BAR_ITEM_ICON_MODE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for LANG_BAR_ITEM_ICON_MODE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for LANG_BAR_ITEM_ICON_MODE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LANG_BAR_ITEM_ICON_MODE_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TEXT_STORE_CHANGE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_TC_NONE: TEXT_STORE_CHANGE_FLAGS = TEXT_STORE_CHANGE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_TC_CORRECTION: TEXT_STORE_CHANGE_FLAGS = TEXT_STORE_CHANGE_FLAGS(1u32);
impl ::core::marker::Copy for TEXT_STORE_CHANGE_FLAGS {}
impl ::core::clone::Clone for TEXT_STORE_CHANGE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TEXT_STORE_CHANGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TEXT_STORE_CHANGE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TEXT_STORE_CHANGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TEXT_STORE_CHANGE_FLAGS").field(&self.0).finish()
    }
}
impl TEXT_STORE_CHANGE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for TEXT_STORE_CHANGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TEXT_STORE_CHANGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TEXT_STORE_CHANGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TEXT_STORE_CHANGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TEXT_STORE_CHANGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TEXT_STORE_LOCK_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_LF_READ: TEXT_STORE_LOCK_FLAGS = TEXT_STORE_LOCK_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_LF_READWRITE: TEXT_STORE_LOCK_FLAGS = TEXT_STORE_LOCK_FLAGS(6u32);
impl ::core::marker::Copy for TEXT_STORE_LOCK_FLAGS {}
impl ::core::clone::Clone for TEXT_STORE_LOCK_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TEXT_STORE_LOCK_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TEXT_STORE_LOCK_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TEXT_STORE_LOCK_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TEXT_STORE_LOCK_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TEXT_STORE_TEXT_CHANGE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_ST_NONE: TEXT_STORE_TEXT_CHANGE_FLAGS = TEXT_STORE_TEXT_CHANGE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_ST_CORRECTION: TEXT_STORE_TEXT_CHANGE_FLAGS = TEXT_STORE_TEXT_CHANGE_FLAGS(1u32);
impl ::core::marker::Copy for TEXT_STORE_TEXT_CHANGE_FLAGS {}
impl ::core::clone::Clone for TEXT_STORE_TEXT_CHANGE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TEXT_STORE_TEXT_CHANGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TEXT_STORE_TEXT_CHANGE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TEXT_STORE_TEXT_CHANGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TEXT_STORE_TEXT_CHANGE_FLAGS").field(&self.0).finish()
    }
}
impl TEXT_STORE_TEXT_CHANGE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for TEXT_STORE_TEXT_CHANGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TEXT_STORE_TEXT_CHANGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TEXT_STORE_TEXT_CHANGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TEXT_STORE_TEXT_CHANGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TEXT_STORE_TEXT_CHANGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TF_CONTEXT_EDIT_CONTEXT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ES_ASYNCDONTCARE: TF_CONTEXT_EDIT_CONTEXT_FLAGS = TF_CONTEXT_EDIT_CONTEXT_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ES_SYNC: TF_CONTEXT_EDIT_CONTEXT_FLAGS = TF_CONTEXT_EDIT_CONTEXT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ES_READ: TF_CONTEXT_EDIT_CONTEXT_FLAGS = TF_CONTEXT_EDIT_CONTEXT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ES_READWRITE: TF_CONTEXT_EDIT_CONTEXT_FLAGS = TF_CONTEXT_EDIT_CONTEXT_FLAGS(6u32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ES_ASYNC: TF_CONTEXT_EDIT_CONTEXT_FLAGS = TF_CONTEXT_EDIT_CONTEXT_FLAGS(8u32);
impl ::core::marker::Copy for TF_CONTEXT_EDIT_CONTEXT_FLAGS {}
impl ::core::clone::Clone for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TF_CONTEXT_EDIT_CONTEXT_FLAGS").field(&self.0).finish()
    }
}
impl TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TF_CONTEXT_EDIT_CONTEXT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TF_DA_ATTR_INFO(pub i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ATTR_INPUT: TF_DA_ATTR_INFO = TF_DA_ATTR_INFO(0i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ATTR_TARGET_CONVERTED: TF_DA_ATTR_INFO = TF_DA_ATTR_INFO(1i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ATTR_CONVERTED: TF_DA_ATTR_INFO = TF_DA_ATTR_INFO(2i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ATTR_TARGET_NOTCONVERTED: TF_DA_ATTR_INFO = TF_DA_ATTR_INFO(3i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ATTR_INPUT_ERROR: TF_DA_ATTR_INFO = TF_DA_ATTR_INFO(4i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ATTR_FIXEDCONVERTED: TF_DA_ATTR_INFO = TF_DA_ATTR_INFO(5i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ATTR_OTHER: TF_DA_ATTR_INFO = TF_DA_ATTR_INFO(-1i32);
impl ::core::marker::Copy for TF_DA_ATTR_INFO {}
impl ::core::clone::Clone for TF_DA_ATTR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TF_DA_ATTR_INFO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TF_DA_ATTR_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TF_DA_ATTR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TF_DA_ATTR_INFO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TF_DA_COLORTYPE(pub i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CT_NONE: TF_DA_COLORTYPE = TF_DA_COLORTYPE(0i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CT_SYSCOLOR: TF_DA_COLORTYPE = TF_DA_COLORTYPE(1i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_CT_COLORREF: TF_DA_COLORTYPE = TF_DA_COLORTYPE(2i32);
impl ::core::marker::Copy for TF_DA_COLORTYPE {}
impl ::core::clone::Clone for TF_DA_COLORTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TF_DA_COLORTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TF_DA_COLORTYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TF_DA_COLORTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TF_DA_COLORTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TF_DA_LINESTYLE(pub i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LS_NONE: TF_DA_LINESTYLE = TF_DA_LINESTYLE(0i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LS_SOLID: TF_DA_LINESTYLE = TF_DA_LINESTYLE(1i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LS_DOT: TF_DA_LINESTYLE = TF_DA_LINESTYLE(2i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LS_DASH: TF_DA_LINESTYLE = TF_DA_LINESTYLE(3i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LS_SQUIGGLE: TF_DA_LINESTYLE = TF_DA_LINESTYLE(4i32);
impl ::core::marker::Copy for TF_DA_LINESTYLE {}
impl ::core::clone::Clone for TF_DA_LINESTYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TF_DA_LINESTYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TF_DA_LINESTYLE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TF_DA_LINESTYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TF_DA_LINESTYLE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TKBLayoutType(pub i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKBLT_UNDEFINED: TKBLayoutType = TKBLayoutType(0i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKBLT_CLASSIC: TKBLayoutType = TKBLayoutType(1i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TKBLT_OPTIMIZED: TKBLayoutType = TKBLayoutType(2i32);
impl ::core::marker::Copy for TKBLayoutType {}
impl ::core::clone::Clone for TKBLayoutType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TKBLayoutType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TKBLayoutType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TKBLayoutType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TKBLayoutType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TfActiveSelEnd(pub i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_AE_NONE: TfActiveSelEnd = TfActiveSelEnd(0i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_AE_START: TfActiveSelEnd = TfActiveSelEnd(1i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_AE_END: TfActiveSelEnd = TfActiveSelEnd(2i32);
impl ::core::marker::Copy for TfActiveSelEnd {}
impl ::core::clone::Clone for TfActiveSelEnd {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TfActiveSelEnd {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TfActiveSelEnd {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TfActiveSelEnd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TfActiveSelEnd").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TfAnchor(pub i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ANCHOR_START: TfAnchor = TfAnchor(0i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_ANCHOR_END: TfAnchor = TfAnchor(1i32);
impl ::core::marker::Copy for TfAnchor {}
impl ::core::clone::Clone for TfAnchor {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TfAnchor {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TfAnchor {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TfAnchor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TfAnchor").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TfCandidateResult(pub i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const CAND_FINALIZED: TfCandidateResult = TfCandidateResult(0i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const CAND_SELECTED: TfCandidateResult = TfCandidateResult(1i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const CAND_CANCELED: TfCandidateResult = TfCandidateResult(2i32);
impl ::core::marker::Copy for TfCandidateResult {}
impl ::core::clone::Clone for TfCandidateResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TfCandidateResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TfCandidateResult {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TfCandidateResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TfCandidateResult").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TfGravity(pub i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_GRAVITY_BACKWARD: TfGravity = TfGravity(0i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_GRAVITY_FORWARD: TfGravity = TfGravity(1i32);
impl ::core::marker::Copy for TfGravity {}
impl ::core::clone::Clone for TfGravity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TfGravity {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TfGravity {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TfGravity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TfGravity").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TfIntegratableCandidateListSelectionStyle(pub i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const STYLE_ACTIVE_SELECTION: TfIntegratableCandidateListSelectionStyle = TfIntegratableCandidateListSelectionStyle(0i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const STYLE_IMPLIED_SELECTION: TfIntegratableCandidateListSelectionStyle = TfIntegratableCandidateListSelectionStyle(1i32);
impl ::core::marker::Copy for TfIntegratableCandidateListSelectionStyle {}
impl ::core::clone::Clone for TfIntegratableCandidateListSelectionStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TfIntegratableCandidateListSelectionStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TfIntegratableCandidateListSelectionStyle {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TfIntegratableCandidateListSelectionStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TfIntegratableCandidateListSelectionStyle").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TfLBBalloonStyle(pub i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LB_BALLOON_RECO: TfLBBalloonStyle = TfLBBalloonStyle(0i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LB_BALLOON_SHOW: TfLBBalloonStyle = TfLBBalloonStyle(1i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LB_BALLOON_MISS: TfLBBalloonStyle = TfLBBalloonStyle(2i32);
impl ::core::marker::Copy for TfLBBalloonStyle {}
impl ::core::clone::Clone for TfLBBalloonStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TfLBBalloonStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TfLBBalloonStyle {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TfLBBalloonStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TfLBBalloonStyle").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TfLBIClick(pub i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_CLK_RIGHT: TfLBIClick = TfLBIClick(1i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LBI_CLK_LEFT: TfLBIClick = TfLBIClick(2i32);
impl ::core::marker::Copy for TfLBIClick {}
impl ::core::clone::Clone for TfLBIClick {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TfLBIClick {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TfLBIClick {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TfLBIClick {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TfLBIClick").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TfLayoutCode(pub i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LC_CREATE: TfLayoutCode = TfLayoutCode(0i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LC_CHANGE: TfLayoutCode = TfLayoutCode(1i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_LC_DESTROY: TfLayoutCode = TfLayoutCode(2i32);
impl ::core::marker::Copy for TfLayoutCode {}
impl ::core::clone::Clone for TfLayoutCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TfLayoutCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TfLayoutCode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TfLayoutCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TfLayoutCode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TfSapiObject(pub i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GETIF_RESMGR: TfSapiObject = TfSapiObject(0i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GETIF_RECOCONTEXT: TfSapiObject = TfSapiObject(1i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GETIF_RECOGNIZER: TfSapiObject = TfSapiObject(2i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GETIF_VOICE: TfSapiObject = TfSapiObject(3i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GETIF_DICTGRAM: TfSapiObject = TfSapiObject(4i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const GETIF_RECOGNIZERNOINIT: TfSapiObject = TfSapiObject(5i32);
impl ::core::marker::Copy for TfSapiObject {}
impl ::core::clone::Clone for TfSapiObject {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TfSapiObject {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TfSapiObject {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TfSapiObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TfSapiObject").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TfShiftDir(pub i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SD_BACKWARD: TfShiftDir = TfShiftDir(0i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TF_SD_FORWARD: TfShiftDir = TfShiftDir(1i32);
impl ::core::marker::Copy for TfShiftDir {}
impl ::core::clone::Clone for TfShiftDir {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TfShiftDir {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TfShiftDir {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TfShiftDir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TfShiftDir").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TsActiveSelEnd(pub i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_AE_NONE: TsActiveSelEnd = TsActiveSelEnd(0i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_AE_START: TsActiveSelEnd = TsActiveSelEnd(1i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_AE_END: TsActiveSelEnd = TsActiveSelEnd(2i32);
impl ::core::marker::Copy for TsActiveSelEnd {}
impl ::core::clone::Clone for TsActiveSelEnd {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TsActiveSelEnd {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TsActiveSelEnd {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TsActiveSelEnd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TsActiveSelEnd").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TsGravity(pub i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_GR_BACKWARD: TsGravity = TsGravity(0i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_GR_FORWARD: TsGravity = TsGravity(1i32);
impl ::core::marker::Copy for TsGravity {}
impl ::core::clone::Clone for TsGravity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TsGravity {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TsGravity {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TsGravity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TsGravity").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TsLayoutCode(pub i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_LC_CREATE: TsLayoutCode = TsLayoutCode(0i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_LC_CHANGE: TsLayoutCode = TsLayoutCode(1i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_LC_DESTROY: TsLayoutCode = TsLayoutCode(2i32);
impl ::core::marker::Copy for TsLayoutCode {}
impl ::core::clone::Clone for TsLayoutCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TsLayoutCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TsLayoutCode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TsLayoutCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TsLayoutCode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TsRunType(pub i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_RT_PLAIN: TsRunType = TsRunType(0i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_RT_HIDDEN: TsRunType = TsRunType(1i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_RT_OPAQUE: TsRunType = TsRunType(2i32);
impl ::core::marker::Copy for TsRunType {}
impl ::core::clone::Clone for TsRunType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TsRunType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TsRunType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TsRunType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TsRunType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TsShiftDir(pub i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SD_BACKWARD: TsShiftDir = TsShiftDir(0i32);
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub const TS_SD_FORWARD: TsShiftDir = TsShiftDir(1i32);
impl ::core::marker::Copy for TsShiftDir {}
impl ::core::clone::Clone for TsShiftDir {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TsShiftDir {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TsShiftDir {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TsShiftDir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TsShiftDir").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HKL(pub isize);
impl HKL {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HKL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HKL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HKL {}
impl ::core::fmt::Debug for HKL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HKL").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HKL {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TF_DA_COLOR {
    pub r#type: TF_DA_COLORTYPE,
    pub Anonymous: TF_DA_COLOR_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TF_DA_COLOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TF_DA_COLOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TF_DA_COLOR {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TF_DA_COLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union TF_DA_COLOR_0 {
    pub nIndex: i32,
    pub cr: super::super::Foundation::COLORREF,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TF_DA_COLOR_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TF_DA_COLOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TF_DA_COLOR_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TF_DA_COLOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TF_DISPLAYATTRIBUTE {
    pub crText: TF_DA_COLOR,
    pub crBk: TF_DA_COLOR,
    pub lsStyle: TF_DA_LINESTYLE,
    pub fBoldLine: super::super::Foundation::BOOL,
    pub crLine: TF_DA_COLOR,
    pub bAttr: TF_DA_ATTR_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TF_DISPLAYATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TF_DISPLAYATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TF_DISPLAYATTRIBUTE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TF_DISPLAYATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub struct TF_HALTCOND {
    pub pHaltRange: ::std::mem::ManuallyDrop<::core::option::Option<ITfRange>>,
    pub aHaltPos: TfAnchor,
    pub dwFlags: u32,
}
impl ::core::clone::Clone for TF_HALTCOND {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for TF_HALTCOND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TF_HALTCOND").field("pHaltRange", &self.pHaltRange).field("aHaltPos", &self.aHaltPos).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::windows::core::TypeKind for TF_HALTCOND {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TF_HALTCOND {
    fn eq(&self, other: &Self) -> bool {
        self.pHaltRange == other.pHaltRange && self.aHaltPos == other.aHaltPos && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for TF_HALTCOND {}
impl ::core::default::Default for TF_HALTCOND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub struct TF_INPUTPROCESSORPROFILE {
    pub dwProfileType: u32,
    pub langid: u16,
    pub clsid: ::windows::core::GUID,
    pub guidProfile: ::windows::core::GUID,
    pub catid: ::windows::core::GUID,
    pub hklSubstitute: HKL,
    pub dwCaps: u32,
    pub hkl: HKL,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for TF_INPUTPROCESSORPROFILE {}
impl ::core::clone::Clone for TF_INPUTPROCESSORPROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TF_INPUTPROCESSORPROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TF_INPUTPROCESSORPROFILE").field("dwProfileType", &self.dwProfileType).field("langid", &self.langid).field("clsid", &self.clsid).field("guidProfile", &self.guidProfile).field("catid", &self.catid).field("hklSubstitute", &self.hklSubstitute).field("dwCaps", &self.dwCaps).field("hkl", &self.hkl).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::windows::core::TypeKind for TF_INPUTPROCESSORPROFILE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TF_INPUTPROCESSORPROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.dwProfileType == other.dwProfileType && self.langid == other.langid && self.clsid == other.clsid && self.guidProfile == other.guidProfile && self.catid == other.catid && self.hklSubstitute == other.hklSubstitute && self.dwCaps == other.dwCaps && self.hkl == other.hkl && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for TF_INPUTPROCESSORPROFILE {}
impl ::core::default::Default for TF_INPUTPROCESSORPROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub struct TF_LANGBARITEMINFO {
    pub clsidService: ::windows::core::GUID,
    pub guidItem: ::windows::core::GUID,
    pub dwStyle: u32,
    pub ulSort: u32,
    pub szDescription: [u16; 32],
}
impl ::core::marker::Copy for TF_LANGBARITEMINFO {}
impl ::core::clone::Clone for TF_LANGBARITEMINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TF_LANGBARITEMINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TF_LANGBARITEMINFO").field("clsidService", &self.clsidService).field("guidItem", &self.guidItem).field("dwStyle", &self.dwStyle).field("ulSort", &self.ulSort).field("szDescription", &self.szDescription).finish()
    }
}
impl ::windows::core::TypeKind for TF_LANGBARITEMINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TF_LANGBARITEMINFO {
    fn eq(&self, other: &Self) -> bool {
        self.clsidService == other.clsidService && self.guidItem == other.guidItem && self.dwStyle == other.dwStyle && self.ulSort == other.ulSort && self.szDescription == other.szDescription
    }
}
impl ::core::cmp::Eq for TF_LANGBARITEMINFO {}
impl ::core::default::Default for TF_LANGBARITEMINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TF_LANGUAGEPROFILE {
    pub clsid: ::windows::core::GUID,
    pub langid: u16,
    pub catid: ::windows::core::GUID,
    pub fActive: super::super::Foundation::BOOL,
    pub guidProfile: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TF_LANGUAGEPROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TF_LANGUAGEPROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TF_LANGUAGEPROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TF_LANGUAGEPROFILE").field("clsid", &self.clsid).field("langid", &self.langid).field("catid", &self.catid).field("fActive", &self.fActive).field("guidProfile", &self.guidProfile).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TF_LANGUAGEPROFILE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TF_LANGUAGEPROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.clsid == other.clsid && self.langid == other.langid && self.catid == other.catid && self.fActive == other.fActive && self.guidProfile == other.guidProfile
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TF_LANGUAGEPROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TF_LANGUAGEPROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub struct TF_LBBALLOONINFO {
    pub style: TfLBBalloonStyle,
    pub bstrText: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
}
impl ::core::clone::Clone for TF_LBBALLOONINFO {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for TF_LBBALLOONINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TF_LBBALLOONINFO").field("style", &self.style).field("bstrText", &self.bstrText).finish()
    }
}
impl ::windows::core::TypeKind for TF_LBBALLOONINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TF_LBBALLOONINFO {
    fn eq(&self, other: &Self) -> bool {
        self.style == other.style && self.bstrText == other.bstrText
    }
}
impl ::core::cmp::Eq for TF_LBBALLOONINFO {}
impl ::core::default::Default for TF_LBBALLOONINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub struct TF_LMLATTELEMENT {
    pub dwFrameStart: u32,
    pub dwFrameLen: u32,
    pub dwFlags: u32,
    pub Anonymous: TF_LMLATTELEMENT_0,
    pub bstrText: ::std::mem::ManuallyDrop<::windows::core::BSTR>,
}
impl ::core::clone::Clone for TF_LMLATTELEMENT {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::windows::core::TypeKind for TF_LMLATTELEMENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for TF_LMLATTELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub union TF_LMLATTELEMENT_0 {
    pub iCost: i32,
}
impl ::core::marker::Copy for TF_LMLATTELEMENT_0 {}
impl ::core::clone::Clone for TF_LMLATTELEMENT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for TF_LMLATTELEMENT_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for TF_LMLATTELEMENT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub struct TF_PERSISTENT_PROPERTY_HEADER_ACP {
    pub guidType: ::windows::core::GUID,
    pub ichStart: i32,
    pub cch: i32,
    pub cb: u32,
    pub dwPrivate: u32,
    pub clsidTIP: ::windows::core::GUID,
}
impl ::core::marker::Copy for TF_PERSISTENT_PROPERTY_HEADER_ACP {}
impl ::core::clone::Clone for TF_PERSISTENT_PROPERTY_HEADER_ACP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TF_PERSISTENT_PROPERTY_HEADER_ACP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TF_PERSISTENT_PROPERTY_HEADER_ACP").field("guidType", &self.guidType).field("ichStart", &self.ichStart).field("cch", &self.cch).field("cb", &self.cb).field("dwPrivate", &self.dwPrivate).field("clsidTIP", &self.clsidTIP).finish()
    }
}
impl ::windows::core::TypeKind for TF_PERSISTENT_PROPERTY_HEADER_ACP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TF_PERSISTENT_PROPERTY_HEADER_ACP {
    fn eq(&self, other: &Self) -> bool {
        self.guidType == other.guidType && self.ichStart == other.ichStart && self.cch == other.cch && self.cb == other.cb && self.dwPrivate == other.dwPrivate && self.clsidTIP == other.clsidTIP
    }
}
impl ::core::cmp::Eq for TF_PERSISTENT_PROPERTY_HEADER_ACP {}
impl ::core::default::Default for TF_PERSISTENT_PROPERTY_HEADER_ACP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub struct TF_PRESERVEDKEY {
    pub uVKey: u32,
    pub uModifiers: u32,
}
impl ::core::marker::Copy for TF_PRESERVEDKEY {}
impl ::core::clone::Clone for TF_PRESERVEDKEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TF_PRESERVEDKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TF_PRESERVEDKEY").field("uVKey", &self.uVKey).field("uModifiers", &self.uModifiers).finish()
    }
}
impl ::windows::core::TypeKind for TF_PRESERVEDKEY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TF_PRESERVEDKEY {
    fn eq(&self, other: &Self) -> bool {
        self.uVKey == other.uVKey && self.uModifiers == other.uModifiers
    }
}
impl ::core::cmp::Eq for TF_PRESERVEDKEY {}
impl ::core::default::Default for TF_PRESERVEDKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct TF_PROPERTYVAL {
    pub guidId: ::windows::core::GUID,
    pub varValue: super::super::System::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for TF_PROPERTYVAL {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::TypeKind for TF_PROPERTYVAL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for TF_PROPERTYVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TF_SELECTION {
    pub range: ::std::mem::ManuallyDrop<::core::option::Option<ITfRange>>,
    pub style: TF_SELECTIONSTYLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TF_SELECTION {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TF_SELECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TF_SELECTION").field("range", &self.range).field("style", &self.style).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TF_SELECTION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TF_SELECTION {
    fn eq(&self, other: &Self) -> bool {
        self.range == other.range && self.style == other.style
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TF_SELECTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TF_SELECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TF_SELECTIONSTYLE {
    pub ase: TfActiveSelEnd,
    pub fInterimChar: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TF_SELECTIONSTYLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TF_SELECTIONSTYLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TF_SELECTIONSTYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TF_SELECTIONSTYLE").field("ase", &self.ase).field("fInterimChar", &self.fInterimChar).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TF_SELECTIONSTYLE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TF_SELECTIONSTYLE {
    fn eq(&self, other: &Self) -> bool {
        self.ase == other.ase && self.fInterimChar == other.fInterimChar
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TF_SELECTIONSTYLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TF_SELECTIONSTYLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct TS_ATTRVAL {
    pub idAttr: ::windows::core::GUID,
    pub dwOverlapId: u32,
    pub varValue: super::super::System::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for TS_ATTRVAL {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::TypeKind for TS_ATTRVAL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for TS_ATTRVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub struct TS_RUNINFO {
    pub uCount: u32,
    pub r#type: TsRunType,
}
impl ::core::marker::Copy for TS_RUNINFO {}
impl ::core::clone::Clone for TS_RUNINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TS_RUNINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TS_RUNINFO").field("uCount", &self.uCount).field("type", &self.r#type).finish()
    }
}
impl ::windows::core::TypeKind for TS_RUNINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TS_RUNINFO {
    fn eq(&self, other: &Self) -> bool {
        self.uCount == other.uCount && self.r#type == other.r#type
    }
}
impl ::core::cmp::Eq for TS_RUNINFO {}
impl ::core::default::Default for TS_RUNINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TS_SELECTIONSTYLE {
    pub ase: TsActiveSelEnd,
    pub fInterimChar: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TS_SELECTIONSTYLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TS_SELECTIONSTYLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TS_SELECTIONSTYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TS_SELECTIONSTYLE").field("ase", &self.ase).field("fInterimChar", &self.fInterimChar).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TS_SELECTIONSTYLE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TS_SELECTIONSTYLE {
    fn eq(&self, other: &Self) -> bool {
        self.ase == other.ase && self.fInterimChar == other.fInterimChar
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TS_SELECTIONSTYLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TS_SELECTIONSTYLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TS_SELECTION_ACP {
    pub acpStart: i32,
    pub acpEnd: i32,
    pub style: TS_SELECTIONSTYLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TS_SELECTION_ACP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TS_SELECTION_ACP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TS_SELECTION_ACP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TS_SELECTION_ACP").field("acpStart", &self.acpStart).field("acpEnd", &self.acpEnd).field("style", &self.style).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TS_SELECTION_ACP {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TS_SELECTION_ACP {
    fn eq(&self, other: &Self) -> bool {
        self.acpStart == other.acpStart && self.acpEnd == other.acpEnd && self.style == other.style
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TS_SELECTION_ACP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TS_SELECTION_ACP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TS_SELECTION_ANCHOR {
    pub paStart: ::std::mem::ManuallyDrop<::core::option::Option<IAnchor>>,
    pub paEnd: ::std::mem::ManuallyDrop<::core::option::Option<IAnchor>>,
    pub style: TS_SELECTIONSTYLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TS_SELECTION_ANCHOR {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TS_SELECTION_ANCHOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TS_SELECTION_ANCHOR").field("paStart", &self.paStart).field("paEnd", &self.paEnd).field("style", &self.style).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for TS_SELECTION_ANCHOR {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TS_SELECTION_ANCHOR {
    fn eq(&self, other: &Self) -> bool {
        self.paStart == other.paStart && self.paEnd == other.paEnd && self.style == other.style
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TS_SELECTION_ANCHOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TS_SELECTION_ANCHOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub struct TS_STATUS {
    pub dwDynamicFlags: u32,
    pub dwStaticFlags: u32,
}
impl ::core::marker::Copy for TS_STATUS {}
impl ::core::clone::Clone for TS_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TS_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TS_STATUS").field("dwDynamicFlags", &self.dwDynamicFlags).field("dwStaticFlags", &self.dwStaticFlags).finish()
    }
}
impl ::windows::core::TypeKind for TS_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TS_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.dwDynamicFlags == other.dwDynamicFlags && self.dwStaticFlags == other.dwStaticFlags
    }
}
impl ::core::cmp::Eq for TS_STATUS {}
impl ::core::default::Default for TS_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_TextServices\"`*"]
pub struct TS_TEXTCHANGE {
    pub acpStart: i32,
    pub acpOldEnd: i32,
    pub acpNewEnd: i32,
}
impl ::core::marker::Copy for TS_TEXTCHANGE {}
impl ::core::clone::Clone for TS_TEXTCHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TS_TEXTCHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TS_TEXTCHANGE").field("acpStart", &self.acpStart).field("acpOldEnd", &self.acpOldEnd).field("acpNewEnd", &self.acpNewEnd).finish()
    }
}
impl ::windows::core::TypeKind for TS_TEXTCHANGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TS_TEXTCHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.acpStart == other.acpStart && self.acpOldEnd == other.acpOldEnd && self.acpNewEnd == other.acpNewEnd
    }
}
impl ::core::cmp::Eq for TS_TEXTCHANGE {}
impl ::core::default::Default for TS_TEXTCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
