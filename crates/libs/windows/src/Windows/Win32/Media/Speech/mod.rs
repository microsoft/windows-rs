#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct IEnumSpObjectTokens(::windows::core::IUnknown);
impl IEnumSpObjectTokens {
    pub unsafe fn Next(&self, celt: u32, pelt: *mut ::core::option::Option<ISpObjectToken>, pceltfetched: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, ::core::mem::transmute(pelt), ::core::mem::transmute(pceltfetched.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumSpObjectTokens> {
        let mut result__ = ::windows::core::zeroed::<IEnumSpObjectTokens>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Item(&self, index: u32) -> ::windows::core::Result<ISpObjectToken> {
        let mut result__ = ::windows::core::zeroed::<ISpObjectToken>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCount(&self, pcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), pcount).ok()
    }
}
::windows::imp::interface_hierarchy!(IEnumSpObjectTokens, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumSpObjectTokens {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSpObjectTokens {}
impl ::core::fmt::Debug for IEnumSpObjectTokens {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSpObjectTokens").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumSpObjectTokens {
    type Vtable = IEnumSpObjectTokens_Vtbl;
}
impl ::core::clone::Clone for IEnumSpObjectTokens {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumSpObjectTokens {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06b64f9e_7fda_11d2_b4f2_00c04f797396);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSpObjectTokens_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, pelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pptoken: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpAudio(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpAudio {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.base__.Read)(::windows::core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.base__.Write)(::windows::core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK, plibnewposition: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Seek)(::windows::core::Interface::as_raw(self), dlibmove, dworigin, ::core::mem::transmute(plibnewposition.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetSize)(::windows::core::Interface::as_raw(self), libnewsize).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyTo<P0>(&self, pstm: P0, cb: u64, pcbread: ::core::option::Option<*mut u64>, pcbwritten: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.CopyTo)(::windows::core::Interface::as_raw(self), pstm.into_param().abi(), cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Commit(&self, grfcommitflags: super::super::System::Com::STGC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Commit)(::windows::core::Interface::as_raw(self), grfcommitflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Revert)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: super::super::System::Com::LOCKTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.LockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.UnlockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Stat(&self, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: super::super::System::Com::STATFLAG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Stat)(::windows::core::Interface::as_raw(self), pstatstg, grfstatflag).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IStream>();
        (::windows::core::Interface::vtable(self).base__.base__.Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormat(&self, pguidformatid: *const ::windows::core::GUID) -> ::windows::core::Result<*mut super::Audio::WAVEFORMATEX> {
        let mut result__ = ::windows::core::zeroed::<*mut super::Audio::WAVEFORMATEX>();
        (::windows::core::Interface::vtable(self).base__.GetFormat)(::windows::core::Interface::as_raw(self), pguidformatid, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetState(&self, newstate: SPAUDIOSTATE, ullreserved: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetState)(::windows::core::Interface::as_raw(self), newstate, ullreserved).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetFormat(&self, rguidfmtid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFormat)(::windows::core::Interface::as_raw(self), rguidfmtid, pwaveformatex).ok()
    }
    pub unsafe fn GetStatus(&self, pstatus: *mut SPAUDIOSTATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), pstatus).ok()
    }
    pub unsafe fn SetBufferInfo(&self, pbuffinfo: *const SPAUDIOBUFFERINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBufferInfo)(::windows::core::Interface::as_raw(self), pbuffinfo).ok()
    }
    pub unsafe fn GetBufferInfo(&self, pbuffinfo: *mut SPAUDIOBUFFERINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBufferInfo)(::windows::core::Interface::as_raw(self), pbuffinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetDefaultFormat(&self, pformatid: *mut ::windows::core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDefaultFormat)(::windows::core::Interface::as_raw(self), pformatid, ppcomemwaveformatex).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EventHandle(&self) -> super::super::Foundation::HANDLE {
        (::windows::core::Interface::vtable(self).EventHandle)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetVolumeLevel(&self, plevel: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVolumeLevel)(::windows::core::Interface::as_raw(self), plevel).ok()
    }
    pub unsafe fn SetVolumeLevel(&self, level: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVolumeLevel)(::windows::core::Interface::as_raw(self), level).ok()
    }
    pub unsafe fn GetBufferNotifySize(&self, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBufferNotifySize)(::windows::core::Interface::as_raw(self), pcbsize).ok()
    }
    pub unsafe fn SetBufferNotifySize(&self, cbsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBufferNotifySize)(::windows::core::Interface::as_raw(self), cbsize).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpAudio, ::windows::core::IUnknown, super::super::System::Com::ISequentialStream, super::super::System::Com::IStream, ISpStreamFormat);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpAudio {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpAudio {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpAudio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpAudio").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpAudio {
    type Vtable = ISpAudio_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpAudio {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpAudio {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc05c768f_fae8_4ec2_8e07_338321c12452);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpAudio_Vtbl {
    pub base__: ISpStreamFormat_Vtbl,
    pub SetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newstate: SPAUDIOSTATE, ullreserved: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidfmtid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    SetFormat: usize,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut SPAUDIOSTATUS) -> ::windows::core::HRESULT,
    pub SetBufferInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffinfo: *const SPAUDIOBUFFERINFO) -> ::windows::core::HRESULT,
    pub GetBufferInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbuffinfo: *mut SPAUDIOBUFFERINFO) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetDefaultFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatid: *mut ::windows::core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetDefaultFormat: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EventHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE,
    #[cfg(not(feature = "Win32_Foundation"))]
    EventHandle: usize,
    pub GetVolumeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plevel: *mut u32) -> ::windows::core::HRESULT,
    pub SetVolumeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: u32) -> ::windows::core::HRESULT,
    pub GetBufferNotifySize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows::core::HRESULT,
    pub SetBufferNotifySize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbsize: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpContainerLexicon(::windows::core::IUnknown);
impl ISpContainerLexicon {
    pub unsafe fn GetPronunciations<P0>(&self, pszword: P0, langid: u16, dwflags: u32, pwordpronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetPronunciations)(::windows::core::Interface::as_raw(self), pszword.into_param().abi(), langid, dwflags, pwordpronunciationlist).ok()
    }
    pub unsafe fn AddPronunciation<P0>(&self, pszword: P0, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: ::core::option::Option<*const u16>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddPronunciation)(::windows::core::Interface::as_raw(self), pszword.into_param().abi(), langid, epartofspeech, ::core::mem::transmute(pszpronunciation.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn RemovePronunciation<P0>(&self, pszword: P0, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: ::core::option::Option<*const u16>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.RemovePronunciation)(::windows::core::Interface::as_raw(self), pszword.into_param().abi(), langid, epartofspeech, ::core::mem::transmute(pszpronunciation.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetGeneration(&self, pdwgeneration: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetGeneration)(::windows::core::Interface::as_raw(self), pdwgeneration).ok()
    }
    pub unsafe fn GetGenerationChange(&self, dwflags: u32, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetGenerationChange)(::windows::core::Interface::as_raw(self), dwflags, pdwgeneration, pwordlist).ok()
    }
    pub unsafe fn GetWords(&self, dwflags: u32, pdwgeneration: *mut u32, pdwcookie: ::core::option::Option<*mut u32>, pwordlist: *mut SPWORDLIST) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetWords)(::windows::core::Interface::as_raw(self), dwflags, pdwgeneration, ::core::mem::transmute(pdwcookie.unwrap_or(::std::ptr::null_mut())), pwordlist).ok()
    }
    pub unsafe fn AddLexicon<P0>(&self, paddlexicon: P0, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpLexicon>,
    {
        (::windows::core::Interface::vtable(self).AddLexicon)(::windows::core::Interface::as_raw(self), paddlexicon.into_param().abi(), dwflags).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpContainerLexicon, ::windows::core::IUnknown, ISpLexicon);
impl ::core::cmp::PartialEq for ISpContainerLexicon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpContainerLexicon {}
impl ::core::fmt::Debug for ISpContainerLexicon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpContainerLexicon").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpContainerLexicon {
    type Vtable = ISpContainerLexicon_Vtbl;
}
impl ::core::clone::Clone for ISpContainerLexicon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpContainerLexicon {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8565572f_c094_41cc_b56e_10bd9c3ff044);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpContainerLexicon_Vtbl {
    pub base__: ISpLexicon_Vtbl,
    pub AddLexicon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paddlexicon: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpDataKey(::windows::core::IUnknown);
impl ISpDataKey {
    pub unsafe fn SetData<P0>(&self, pszvaluename: P0, cbdata: u32, pdata: *const u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetData)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), cbdata, pdata).ok()
    }
    pub unsafe fn GetData<P0>(&self, pszvaluename: P0, pcbdata: *mut u32, pdata: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetData)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), pcbdata, pdata).ok()
    }
    pub unsafe fn SetStringValue<P0, P1>(&self, pszvaluename: P0, pszvalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetStringValue)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), pszvalue.into_param().abi()).ok()
    }
    pub unsafe fn GetStringValue<P0>(&self, pszvaluename: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetStringValue)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDWORD<P0>(&self, pszvaluename: P0, dwvalue: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDWORD)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), dwvalue).ok()
    }
    pub unsafe fn GetDWORD<P0>(&self, pszvaluename: P0, pdwvalue: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetDWORD)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), pdwvalue).ok()
    }
    pub unsafe fn OpenKey<P0>(&self, pszsubkeyname: P0) -> ::windows::core::Result<ISpDataKey>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISpDataKey>();
        (::windows::core::Interface::vtable(self).OpenKey)(::windows::core::Interface::as_raw(self), pszsubkeyname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateKey<P0>(&self, pszsubkey: P0) -> ::windows::core::Result<ISpDataKey>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISpDataKey>();
        (::windows::core::Interface::vtable(self).CreateKey)(::windows::core::Interface::as_raw(self), pszsubkey.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteKey<P0>(&self, pszsubkey: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteKey)(::windows::core::Interface::as_raw(self), pszsubkey.into_param().abi()).ok()
    }
    pub unsafe fn DeleteValue<P0>(&self, pszvaluename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteValue)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi()).ok()
    }
    pub unsafe fn EnumKeys(&self, index: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).EnumKeys)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumValues(&self, index: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).EnumValues)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISpDataKey, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpDataKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpDataKey {}
impl ::core::fmt::Debug for ISpDataKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpDataKey").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpDataKey {
    type Vtable = ISpDataKey_Vtbl;
}
impl ::core::clone::Clone for ISpDataKey {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpDataKey {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14056581_e16c_11d2_bb90_00c04f8ee6c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpDataKey_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszvaluename: ::windows::core::PCWSTR, cbdata: u32, pdata: *const u8) -> ::windows::core::HRESULT,
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszvaluename: ::windows::core::PCWSTR, pcbdata: *mut u32, pdata: *mut u8) -> ::windows::core::HRESULT,
    pub SetStringValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszvaluename: ::windows::core::PCWSTR, pszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetStringValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszvaluename: ::windows::core::PCWSTR, ppszvalue: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetDWORD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszvaluename: ::windows::core::PCWSTR, dwvalue: u32) -> ::windows::core::HRESULT,
    pub GetDWORD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszvaluename: ::windows::core::PCWSTR, pdwvalue: *mut u32) -> ::windows::core::HRESULT,
    pub OpenKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubkeyname: ::windows::core::PCWSTR, ppsubkey: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubkey: ::windows::core::PCWSTR, ppsubkey: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DeleteKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsubkey: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub DeleteValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszvaluename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub EnumKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppszsubkeyname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub EnumValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppszvaluename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpDisplayAlternates(::windows::core::IUnknown);
impl ISpDisplayAlternates {
    pub unsafe fn GetDisplayAlternates(&self, pphrase: *const SPDISPLAYPHRASE, crequestcount: u32, ppcomemphrases: *mut *mut SPDISPLAYPHRASE, pcphrasesreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplayAlternates)(::windows::core::Interface::as_raw(self), pphrase, crequestcount, ppcomemphrases, pcphrasesreturned).ok()
    }
    pub unsafe fn SetFullStopTrailSpace(&self, ultrailspace: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFullStopTrailSpace)(::windows::core::Interface::as_raw(self), ultrailspace).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpDisplayAlternates, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpDisplayAlternates {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpDisplayAlternates {}
impl ::core::fmt::Debug for ISpDisplayAlternates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpDisplayAlternates").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpDisplayAlternates {
    type Vtable = ISpDisplayAlternates_Vtbl;
}
impl ::core::clone::Clone for ISpDisplayAlternates {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpDisplayAlternates {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8d7c7e2_0dde_44b7_afe3_b0c991fbeb5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpDisplayAlternates_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDisplayAlternates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphrase: *const SPDISPLAYPHRASE, crequestcount: u32, ppcomemphrases: *mut *mut SPDISPLAYPHRASE, pcphrasesreturned: *mut u32) -> ::windows::core::HRESULT,
    pub SetFullStopTrailSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ultrailspace: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpEnginePronunciation(::windows::core::IUnknown);
impl ISpEnginePronunciation {
    pub unsafe fn Normalize<P0, P1, P2>(&self, pszword: P0, pszleftcontext: P1, pszrightcontext: P2, langid: u16, pnormalizationlist: *mut SPNORMALIZATIONLIST) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Normalize)(::windows::core::Interface::as_raw(self), pszword.into_param().abi(), pszleftcontext.into_param().abi(), pszrightcontext.into_param().abi(), langid, pnormalizationlist).ok()
    }
    pub unsafe fn GetPronunciations<P0, P1, P2>(&self, pszword: P0, pszleftcontext: P1, pszrightcontext: P2, langid: u16, penginepronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetPronunciations)(::windows::core::Interface::as_raw(self), pszword.into_param().abi(), pszleftcontext.into_param().abi(), pszrightcontext.into_param().abi(), langid, penginepronunciationlist).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpEnginePronunciation, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpEnginePronunciation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpEnginePronunciation {}
impl ::core::fmt::Debug for ISpEnginePronunciation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpEnginePronunciation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpEnginePronunciation {
    type Vtable = ISpEnginePronunciation_Vtbl;
}
impl ::core::clone::Clone for ISpEnginePronunciation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpEnginePronunciation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc360ce4b_76d1_4214_ad68_52657d5083da);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpEnginePronunciation_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Normalize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszword: ::windows::core::PCWSTR, pszleftcontext: ::windows::core::PCWSTR, pszrightcontext: ::windows::core::PCWSTR, langid: u16, pnormalizationlist: *mut SPNORMALIZATIONLIST) -> ::windows::core::HRESULT,
    pub GetPronunciations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszword: ::windows::core::PCWSTR, pszleftcontext: ::windows::core::PCWSTR, pszrightcontext: ::windows::core::PCWSTR, langid: u16, penginepronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpEventSink(::windows::core::IUnknown);
impl ISpEventSink {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddEvents(&self, peventarray: *const SPEVENT, ulcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddEvents)(::windows::core::Interface::as_raw(self), peventarray, ulcount).ok()
    }
    pub unsafe fn GetEventInterest(&self, pulleventinterest: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetEventInterest)(::windows::core::Interface::as_raw(self), pulleventinterest).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpEventSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpEventSink {}
impl ::core::fmt::Debug for ISpEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpEventSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpEventSink {
    type Vtable = ISpEventSink_Vtbl;
}
impl ::core::clone::Clone for ISpEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpEventSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe7a9cc9_5f9e_11d2_960f_00c04f8ee628);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpEventSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AddEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peventarray: *const SPEVENT, ulcount: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddEvents: usize,
    pub GetEventInterest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulleventinterest: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpEventSource(::windows::core::IUnknown);
impl ISpEventSource {
    pub unsafe fn SetNotifySink<P0>(&self, pnotifysink: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpNotifySink>,
    {
        (::windows::core::Interface::vtable(self).base__.SetNotifySink)(::windows::core::Interface::as_raw(self), pnotifysink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotifyWindowMessage<P0, P1, P2>(&self, hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).base__.SetNotifyWindowMessage)(::windows::core::Interface::as_raw(self), hwnd.into_param().abi(), msg, wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotifyCallbackFunction<P0, P1>(&self, pfncallback: *mut SPNOTIFYCALLBACK, wparam: P0, lparam: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).base__.SetNotifyCallbackFunction)(::windows::core::Interface::as_raw(self), pfncallback, wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotifyCallbackInterface<P0, P1, P2>(&self, pspcallback: P0, wparam: P1, lparam: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpNotifyCallback>,
        P1: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).base__.SetNotifyCallbackInterface)(::windows::core::Interface::as_raw(self), pspcallback.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    pub unsafe fn SetNotifyWin32Event(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetNotifyWin32Event)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForNotifyEvent(&self, dwmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.WaitForNotifyEvent)(::windows::core::Interface::as_raw(self), dwmilliseconds).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNotifyEventHandle(&self) -> super::super::Foundation::HANDLE {
        (::windows::core::Interface::vtable(self).base__.GetNotifyEventHandle)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetInterest(&self, ulleventinterest: u64, ullqueuedinterest: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInterest)(::windows::core::Interface::as_raw(self), ulleventinterest, ullqueuedinterest).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEvents(&self, ulcount: u32, peventarray: *mut SPEVENT, pulfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetEvents)(::windows::core::Interface::as_raw(self), ulcount, peventarray, pulfetched).ok()
    }
    pub unsafe fn GetInfo(&self, pinfo: *mut SPEVENTSOURCEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInfo)(::windows::core::Interface::as_raw(self), pinfo).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpEventSource, ::windows::core::IUnknown, ISpNotifySource);
impl ::core::cmp::PartialEq for ISpEventSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpEventSource {}
impl ::core::fmt::Debug for ISpEventSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpEventSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpEventSource {
    type Vtable = ISpEventSource_Vtbl;
}
impl ::core::clone::Clone for ISpEventSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpEventSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe7a9cce_5f9e_11d2_960f_00c04f8ee628);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpEventSource_Vtbl {
    pub base__: ISpNotifySource_Vtbl,
    pub SetInterest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulleventinterest: u64, ullqueuedinterest: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEvents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, peventarray: *mut SPEVENT, pulfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEvents: usize,
    pub GetInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut SPEVENTSOURCEINFO) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpEventSource2(::windows::core::IUnknown);
impl ISpEventSource2 {
    pub unsafe fn SetNotifySink<P0>(&self, pnotifysink: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpNotifySink>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetNotifySink)(::windows::core::Interface::as_raw(self), pnotifysink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotifyWindowMessage<P0, P1, P2>(&self, hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetNotifyWindowMessage)(::windows::core::Interface::as_raw(self), hwnd.into_param().abi(), msg, wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotifyCallbackFunction<P0, P1>(&self, pfncallback: *mut SPNOTIFYCALLBACK, wparam: P0, lparam: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetNotifyCallbackFunction)(::windows::core::Interface::as_raw(self), pfncallback, wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotifyCallbackInterface<P0, P1, P2>(&self, pspcallback: P0, wparam: P1, lparam: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpNotifyCallback>,
        P1: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetNotifyCallbackInterface)(::windows::core::Interface::as_raw(self), pspcallback.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    pub unsafe fn SetNotifyWin32Event(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetNotifyWin32Event)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForNotifyEvent(&self, dwmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.WaitForNotifyEvent)(::windows::core::Interface::as_raw(self), dwmilliseconds).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNotifyEventHandle(&self) -> super::super::Foundation::HANDLE {
        (::windows::core::Interface::vtable(self).base__.base__.GetNotifyEventHandle)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetInterest(&self, ulleventinterest: u64, ullqueuedinterest: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetInterest)(::windows::core::Interface::as_raw(self), ulleventinterest, ullqueuedinterest).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEvents(&self, ulcount: u32, peventarray: *mut SPEVENT, pulfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetEvents)(::windows::core::Interface::as_raw(self), ulcount, peventarray, pulfetched).ok()
    }
    pub unsafe fn GetInfo(&self, pinfo: *mut SPEVENTSOURCEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetInfo)(::windows::core::Interface::as_raw(self), pinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEventsEx(&self, ulcount: u32, peventarray: *mut SPEVENTEX, pulfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetEventsEx)(::windows::core::Interface::as_raw(self), ulcount, peventarray, pulfetched).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpEventSource2, ::windows::core::IUnknown, ISpNotifySource, ISpEventSource);
impl ::core::cmp::PartialEq for ISpEventSource2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpEventSource2 {}
impl ::core::fmt::Debug for ISpEventSource2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpEventSource2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpEventSource2 {
    type Vtable = ISpEventSource2_Vtbl;
}
impl ::core::clone::Clone for ISpEventSource2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpEventSource2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2373a435_6a4b_429e_a6ac_d4231a61975b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpEventSource2_Vtbl {
    pub base__: ISpEventSource_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEventsEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulcount: u32, peventarray: *mut SPEVENTEX, pulfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEventsEx: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpGrammarBuilder(::windows::core::IUnknown);
impl ISpGrammarBuilder {
    pub unsafe fn ResetGrammar(&self, newlanguage: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResetGrammar)(::windows::core::Interface::as_raw(self), newlanguage).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRule<P0, P1>(&self, pszrulename: P0, dwruleid: u32, dwattributes: u32, fcreateifnotexist: P1, phinitialstate: *mut *mut SPSTATEHANDLE__) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).GetRule)(::windows::core::Interface::as_raw(self), pszrulename.into_param().abi(), dwruleid, dwattributes, fcreateifnotexist.into_param().abi(), phinitialstate).ok()
    }
    pub unsafe fn ClearRule(&self, hstate: *mut SPSTATEHANDLE__) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearRule)(::windows::core::Interface::as_raw(self), hstate).ok()
    }
    pub unsafe fn CreateNewState(&self, hstate: *mut SPSTATEHANDLE__, phstate: *mut *mut SPSTATEHANDLE__) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateNewState)(::windows::core::Interface::as_raw(self), hstate, phstate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddWordTransition<P0, P1>(&self, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, psz: P0, pszseparators: P1, ewordtype: SPGRAMMARWORDTYPE, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddWordTransition)(::windows::core::Interface::as_raw(self), hfromstate, htostate, psz.into_param().abi(), pszseparators.into_param().abi(), ewordtype, weight, ppropinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddRuleTransition(&self, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, hrule: *mut SPSTATEHANDLE__, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddRuleTransition)(::windows::core::Interface::as_raw(self), hfromstate, htostate, hrule, weight, ppropinfo).ok()
    }
    pub unsafe fn AddResource<P0, P1>(&self, hrulestate: *mut SPSTATEHANDLE__, pszresourcename: P0, pszresourcevalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddResource)(::windows::core::Interface::as_raw(self), hrulestate, pszresourcename.into_param().abi(), pszresourcevalue.into_param().abi()).ok()
    }
    pub unsafe fn Commit(&self, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::windows::core::Interface::as_raw(self), dwreserved).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpGrammarBuilder, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpGrammarBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpGrammarBuilder {}
impl ::core::fmt::Debug for ISpGrammarBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpGrammarBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpGrammarBuilder {
    type Vtable = ISpGrammarBuilder_Vtbl;
}
impl ::core::clone::Clone for ISpGrammarBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpGrammarBuilder {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8137828f_591a_4a42_be58_49ea7ebaac68);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpGrammarBuilder_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ResetGrammar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newlanguage: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszrulename: ::windows::core::PCWSTR, dwruleid: u32, dwattributes: u32, fcreateifnotexist: super::super::Foundation::BOOL, phinitialstate: *mut *mut SPSTATEHANDLE__) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRule: usize,
    pub ClearRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hstate: *mut SPSTATEHANDLE__) -> ::windows::core::HRESULT,
    pub CreateNewState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hstate: *mut SPSTATEHANDLE__, phstate: *mut *mut SPSTATEHANDLE__) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddWordTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, psz: ::windows::core::PCWSTR, pszseparators: ::windows::core::PCWSTR, ewordtype: SPGRAMMARWORDTYPE, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddWordTransition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddRuleTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, hrule: *mut SPSTATEHANDLE__, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddRuleTransition: usize,
    pub AddResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrulestate: *mut SPSTATEHANDLE__, pszresourcename: ::windows::core::PCWSTR, pszresourcevalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpGrammarBuilder2(::windows::core::IUnknown);
impl ISpGrammarBuilder2 {
    pub unsafe fn AddTextSubset<P0>(&self, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, psz: P0, ematchmode: SPMATCHINGMODE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddTextSubset)(::windows::core::Interface::as_raw(self), hfromstate, htostate, psz.into_param().abi(), ematchmode).ok()
    }
    pub unsafe fn SetPhoneticAlphabet(&self, phoneticalphabet: PHONETICALPHABET) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPhoneticAlphabet)(::windows::core::Interface::as_raw(self), phoneticalphabet).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpGrammarBuilder2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpGrammarBuilder2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpGrammarBuilder2 {}
impl ::core::fmt::Debug for ISpGrammarBuilder2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpGrammarBuilder2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpGrammarBuilder2 {
    type Vtable = ISpGrammarBuilder2_Vtbl;
}
impl ::core::clone::Clone for ISpGrammarBuilder2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpGrammarBuilder2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ab10026_20cc_4b20_8c22_a49c9ba78f60);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpGrammarBuilder2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddTextSubset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, psz: ::windows::core::PCWSTR, ematchmode: SPMATCHINGMODE) -> ::windows::core::HRESULT,
    pub SetPhoneticAlphabet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phoneticalphabet: PHONETICALPHABET) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpLexicon(::windows::core::IUnknown);
impl ISpLexicon {
    pub unsafe fn GetPronunciations<P0>(&self, pszword: P0, langid: u16, dwflags: u32, pwordpronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetPronunciations)(::windows::core::Interface::as_raw(self), pszword.into_param().abi(), langid, dwflags, pwordpronunciationlist).ok()
    }
    pub unsafe fn AddPronunciation<P0>(&self, pszword: P0, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: ::core::option::Option<*const u16>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddPronunciation)(::windows::core::Interface::as_raw(self), pszword.into_param().abi(), langid, epartofspeech, ::core::mem::transmute(pszpronunciation.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn RemovePronunciation<P0>(&self, pszword: P0, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: ::core::option::Option<*const u16>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).RemovePronunciation)(::windows::core::Interface::as_raw(self), pszword.into_param().abi(), langid, epartofspeech, ::core::mem::transmute(pszpronunciation.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetGeneration(&self, pdwgeneration: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGeneration)(::windows::core::Interface::as_raw(self), pdwgeneration).ok()
    }
    pub unsafe fn GetGenerationChange(&self, dwflags: u32, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGenerationChange)(::windows::core::Interface::as_raw(self), dwflags, pdwgeneration, pwordlist).ok()
    }
    pub unsafe fn GetWords(&self, dwflags: u32, pdwgeneration: *mut u32, pdwcookie: ::core::option::Option<*mut u32>, pwordlist: *mut SPWORDLIST) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetWords)(::windows::core::Interface::as_raw(self), dwflags, pdwgeneration, ::core::mem::transmute(pdwcookie.unwrap_or(::std::ptr::null_mut())), pwordlist).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpLexicon, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpLexicon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpLexicon {}
impl ::core::fmt::Debug for ISpLexicon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpLexicon").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpLexicon {
    type Vtable = ISpLexicon_Vtbl;
}
impl ::core::clone::Clone for ISpLexicon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpLexicon {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda41a7c2_5383_4db2_916b_6c1719e3db58);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpLexicon_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPronunciations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszword: ::windows::core::PCWSTR, langid: u16, dwflags: u32, pwordpronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows::core::HRESULT,
    pub AddPronunciation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszword: ::windows::core::PCWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> ::windows::core::HRESULT,
    pub RemovePronunciation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszword: ::windows::core::PCWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> ::windows::core::HRESULT,
    pub GetGeneration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32) -> ::windows::core::HRESULT,
    pub GetGenerationChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::HRESULT,
    pub GetWords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpMMSysAudio(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpMMSysAudio {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.Read)(::windows::core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.base__.base__.Write)(::windows::core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK, plibnewposition: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.Seek)(::windows::core::Interface::as_raw(self), dlibmove, dworigin, ::core::mem::transmute(plibnewposition.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.SetSize)(::windows::core::Interface::as_raw(self), libnewsize).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyTo<P0>(&self, pstm: P0, cb: u64, pcbread: ::core::option::Option<*mut u64>, pcbwritten: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.base__.CopyTo)(::windows::core::Interface::as_raw(self), pstm.into_param().abi(), cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Commit(&self, grfcommitflags: super::super::System::Com::STGC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.Commit)(::windows::core::Interface::as_raw(self), grfcommitflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.Revert)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: super::super::System::Com::LOCKTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.LockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.UnlockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Stat(&self, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: super::super::System::Com::STATFLAG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.base__.Stat)(::windows::core::Interface::as_raw(self), pstatstg, grfstatflag).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IStream>();
        (::windows::core::Interface::vtable(self).base__.base__.base__.Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormat(&self, pguidformatid: *const ::windows::core::GUID) -> ::windows::core::Result<*mut super::Audio::WAVEFORMATEX> {
        let mut result__ = ::windows::core::zeroed::<*mut super::Audio::WAVEFORMATEX>();
        (::windows::core::Interface::vtable(self).base__.base__.GetFormat)(::windows::core::Interface::as_raw(self), pguidformatid, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetState(&self, newstate: SPAUDIOSTATE, ullreserved: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetState)(::windows::core::Interface::as_raw(self), newstate, ullreserved).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetFormat(&self, rguidfmtid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetFormat)(::windows::core::Interface::as_raw(self), rguidfmtid, pwaveformatex).ok()
    }
    pub unsafe fn GetStatus(&self, pstatus: *mut SPAUDIOSTATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetStatus)(::windows::core::Interface::as_raw(self), pstatus).ok()
    }
    pub unsafe fn SetBufferInfo(&self, pbuffinfo: *const SPAUDIOBUFFERINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetBufferInfo)(::windows::core::Interface::as_raw(self), pbuffinfo).ok()
    }
    pub unsafe fn GetBufferInfo(&self, pbuffinfo: *mut SPAUDIOBUFFERINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetBufferInfo)(::windows::core::Interface::as_raw(self), pbuffinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetDefaultFormat(&self, pformatid: *mut ::windows::core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDefaultFormat)(::windows::core::Interface::as_raw(self), pformatid, ppcomemwaveformatex).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EventHandle(&self) -> super::super::Foundation::HANDLE {
        (::windows::core::Interface::vtable(self).base__.EventHandle)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetVolumeLevel(&self, plevel: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetVolumeLevel)(::windows::core::Interface::as_raw(self), plevel).ok()
    }
    pub unsafe fn SetVolumeLevel(&self, level: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetVolumeLevel)(::windows::core::Interface::as_raw(self), level).ok()
    }
    pub unsafe fn GetBufferNotifySize(&self, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetBufferNotifySize)(::windows::core::Interface::as_raw(self), pcbsize).ok()
    }
    pub unsafe fn SetBufferNotifySize(&self, cbsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetBufferNotifySize)(::windows::core::Interface::as_raw(self), cbsize).ok()
    }
    pub unsafe fn GetDeviceId(&self, pudeviceid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeviceId)(::windows::core::Interface::as_raw(self), pudeviceid).ok()
    }
    pub unsafe fn SetDeviceId(&self, udeviceid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDeviceId)(::windows::core::Interface::as_raw(self), udeviceid).ok()
    }
    pub unsafe fn GetMMHandle(&self, phandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMMHandle)(::windows::core::Interface::as_raw(self), phandle).ok()
    }
    pub unsafe fn GetLineId(&self, pulineid: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLineId)(::windows::core::Interface::as_raw(self), pulineid).ok()
    }
    pub unsafe fn SetLineId(&self, ulineid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLineId)(::windows::core::Interface::as_raw(self), ulineid).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpMMSysAudio, ::windows::core::IUnknown, super::super::System::Com::ISequentialStream, super::super::System::Com::IStream, ISpStreamFormat, ISpAudio);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpMMSysAudio {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpMMSysAudio {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpMMSysAudio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpMMSysAudio").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpMMSysAudio {
    type Vtable = ISpMMSysAudio_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpMMSysAudio {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpMMSysAudio {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15806f6e_1d70_4b48_98e6_3b1a007509ab);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpMMSysAudio_Vtbl {
    pub base__: ISpAudio_Vtbl,
    pub GetDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pudeviceid: *mut u32) -> ::windows::core::HRESULT,
    pub SetDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, udeviceid: u32) -> ::windows::core::HRESULT,
    pub GetMMHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phandle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulineid: *mut u32) -> ::windows::core::HRESULT,
    pub SetLineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulineid: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpNotifyCallback(::std::ptr::NonNull<::std::ffi::c_void>);
impl ISpNotifyCallback {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NotifyCallback<P0, P1>(&self, wparam: P0, lparam: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).NotifyCallback)(::windows::core::Interface::as_raw(self), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ISpNotifyCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpNotifyCallback {}
impl ::core::fmt::Debug for ISpNotifyCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpNotifyCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpNotifyCallback {
    type Vtable = ISpNotifyCallback_Vtbl;
}
impl ::core::clone::Clone for ISpNotifyCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpNotifyCallback_Vtbl {
    #[cfg(feature = "Win32_Foundation")]
    pub NotifyCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    NotifyCallback: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpNotifySink(::windows::core::IUnknown);
impl ISpNotifySink {
    pub unsafe fn Notify(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Notify)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpNotifySink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpNotifySink {}
impl ::core::fmt::Debug for ISpNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpNotifySink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpNotifySink {
    type Vtable = ISpNotifySink_Vtbl;
}
impl ::core::clone::Clone for ISpNotifySink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpNotifySink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x259684dc_37c3_11d2_9603_00c04f8ee628);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpNotifySink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Notify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpNotifySource(::windows::core::IUnknown);
impl ISpNotifySource {
    pub unsafe fn SetNotifySink<P0>(&self, pnotifysink: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpNotifySink>,
    {
        (::windows::core::Interface::vtable(self).SetNotifySink)(::windows::core::Interface::as_raw(self), pnotifysink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotifyWindowMessage<P0, P1, P2>(&self, hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).SetNotifyWindowMessage)(::windows::core::Interface::as_raw(self), hwnd.into_param().abi(), msg, wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotifyCallbackFunction<P0, P1>(&self, pfncallback: *mut SPNOTIFYCALLBACK, wparam: P0, lparam: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).SetNotifyCallbackFunction)(::windows::core::Interface::as_raw(self), pfncallback, wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotifyCallbackInterface<P0, P1, P2>(&self, pspcallback: P0, wparam: P1, lparam: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpNotifyCallback>,
        P1: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).SetNotifyCallbackInterface)(::windows::core::Interface::as_raw(self), pspcallback.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    pub unsafe fn SetNotifyWin32Event(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNotifyWin32Event)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForNotifyEvent(&self, dwmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WaitForNotifyEvent)(::windows::core::Interface::as_raw(self), dwmilliseconds).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNotifyEventHandle(&self) -> super::super::Foundation::HANDLE {
        (::windows::core::Interface::vtable(self).GetNotifyEventHandle)(::windows::core::Interface::as_raw(self))
    }
}
::windows::imp::interface_hierarchy!(ISpNotifySource, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpNotifySource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpNotifySource {}
impl ::core::fmt::Debug for ISpNotifySource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpNotifySource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpNotifySource {
    type Vtable = ISpNotifySource_Vtbl;
}
impl ::core::clone::Clone for ISpNotifySource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpNotifySource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5eff4aef_8487_11d2_961c_00c04f8ee628);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpNotifySource_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetNotifySink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnotifysink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNotifyWindowMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNotifyWindowMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNotifyCallbackFunction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfncallback: *mut SPNOTIFYCALLBACK, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNotifyCallbackFunction: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetNotifyCallbackInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pspcallback: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetNotifyCallbackInterface: usize,
    pub SetNotifyWin32Event: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WaitForNotifyEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmilliseconds: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNotifyEventHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNotifyEventHandle: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpNotifyTranslator(::windows::core::IUnknown);
impl ISpNotifyTranslator {
    pub unsafe fn Notify(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Notify)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitWindowMessage<P0, P1, P2>(&self, hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).InitWindowMessage)(::windows::core::Interface::as_raw(self), hwnd.into_param().abi(), msg, wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitCallback<P0, P1>(&self, pfncallback: *mut SPNOTIFYCALLBACK, wparam: P0, lparam: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).InitCallback)(::windows::core::Interface::as_raw(self), pfncallback, wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitSpNotifyCallback<P0, P1, P2>(&self, pspcallback: P0, wparam: P1, lparam: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpNotifyCallback>,
        P1: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).InitSpNotifyCallback)(::windows::core::Interface::as_raw(self), pspcallback.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitWin32Event<P0, P1>(&self, hevent: P0, fclosehandleonrelease: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).InitWin32Event)(::windows::core::Interface::as_raw(self), hevent.into_param().abi(), fclosehandleonrelease.into_param().abi()).ok()
    }
    pub unsafe fn Wait(&self, dwmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Wait)(::windows::core::Interface::as_raw(self), dwmilliseconds).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEventHandle(&self) -> super::super::Foundation::HANDLE {
        (::windows::core::Interface::vtable(self).GetEventHandle)(::windows::core::Interface::as_raw(self))
    }
}
::windows::imp::interface_hierarchy!(ISpNotifyTranslator, ::windows::core::IUnknown, ISpNotifySink);
impl ::core::cmp::PartialEq for ISpNotifyTranslator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpNotifyTranslator {}
impl ::core::fmt::Debug for ISpNotifyTranslator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpNotifyTranslator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpNotifyTranslator {
    type Vtable = ISpNotifyTranslator_Vtbl;
}
impl ::core::clone::Clone for ISpNotifyTranslator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpNotifyTranslator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaca16614_5d3d_11d2_960e_00c04f8ee628);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpNotifyTranslator_Vtbl {
    pub base__: ISpNotifySink_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub InitWindowMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitWindowMessage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfncallback: *mut SPNOTIFYCALLBACK, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitCallback: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitSpNotifyCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pspcallback: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitSpNotifyCallback: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub InitWin32Event: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, fclosehandleonrelease: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitWin32Event: usize,
    pub Wait: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmilliseconds: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEventHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEventHandle: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpObjectToken(::windows::core::IUnknown);
impl ISpObjectToken {
    pub unsafe fn SetData<P0>(&self, pszvaluename: P0, cbdata: u32, pdata: *const u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetData)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), cbdata, pdata).ok()
    }
    pub unsafe fn GetData<P0>(&self, pszvaluename: P0, pcbdata: *mut u32, pdata: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetData)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), pcbdata, pdata).ok()
    }
    pub unsafe fn SetStringValue<P0, P1>(&self, pszvaluename: P0, pszvalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetStringValue)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), pszvalue.into_param().abi()).ok()
    }
    pub unsafe fn GetStringValue<P0>(&self, pszvaluename: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetStringValue)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDWORD<P0>(&self, pszvaluename: P0, dwvalue: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDWORD)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), dwvalue).ok()
    }
    pub unsafe fn GetDWORD<P0>(&self, pszvaluename: P0, pdwvalue: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetDWORD)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), pdwvalue).ok()
    }
    pub unsafe fn OpenKey<P0>(&self, pszsubkeyname: P0) -> ::windows::core::Result<ISpDataKey>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISpDataKey>();
        (::windows::core::Interface::vtable(self).base__.OpenKey)(::windows::core::Interface::as_raw(self), pszsubkeyname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateKey<P0>(&self, pszsubkey: P0) -> ::windows::core::Result<ISpDataKey>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISpDataKey>();
        (::windows::core::Interface::vtable(self).base__.CreateKey)(::windows::core::Interface::as_raw(self), pszsubkey.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteKey<P0>(&self, pszsubkey: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteKey)(::windows::core::Interface::as_raw(self), pszsubkey.into_param().abi()).ok()
    }
    pub unsafe fn DeleteValue<P0>(&self, pszvaluename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteValue)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi()).ok()
    }
    pub unsafe fn EnumKeys(&self, index: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.EnumKeys)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumValues(&self, index: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.EnumValues)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<P0, P1, P2>(&self, pszcategoryid: P0, psztokenid: P1, fcreateifnotexist: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetId)(::windows::core::Interface::as_raw(self), pszcategoryid.into_param().abi(), psztokenid.into_param().abi(), fcreateifnotexist.into_param().abi()).ok()
    }
    pub unsafe fn GetId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCategory(&self) -> ::windows::core::Result<ISpObjectTokenCategory> {
        let mut result__ = ::windows::core::zeroed::<ISpObjectTokenCategory>();
        (::windows::core::Interface::vtable(self).GetCategory)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateInstance<P0>(&self, punkouter: P0, dwclscontext: u32, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).CreateInstance)(::windows::core::Interface::as_raw(self), punkouter.into_param().abi(), dwclscontext, riid, ppvobject).ok()
    }
    pub unsafe fn GetStorageFileName<P0, P1>(&self, clsidcaller: *const ::windows::core::GUID, pszvaluename: P0, pszfilenamespecifier: P1, nfolder: u32) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetStorageFileName)(::windows::core::Interface::as_raw(self), clsidcaller, pszvaluename.into_param().abi(), pszfilenamespecifier.into_param().abi(), nfolder, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveStorageFileName<P0, P1>(&self, clsidcaller: *const ::windows::core::GUID, pszkeyname: P0, fdeletefile: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).RemoveStorageFileName)(::windows::core::Interface::as_raw(self), clsidcaller, pszkeyname.into_param().abi(), fdeletefile.into_param().abi()).ok()
    }
    pub unsafe fn Remove(&self, pclsidcaller: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pclsidcaller.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUISupported<P0, P1>(&self, psztypeofui: P0, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: P1, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).IsUISupported)(::windows::core::Interface::as_raw(self), psztypeofui.into_param().abi(), pvextradata, cbextradata, punkobject.into_param().abi(), pfsupported).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayUI<P0, P1, P2, P3>(&self, hwndparent: P0, psztitle: P1, psztypeofui: P2, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).DisplayUI)(::windows::core::Interface::as_raw(self), hwndparent.into_param().abi(), psztitle.into_param().abi(), psztypeofui.into_param().abi(), pvextradata, cbextradata, punkobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MatchesAttributes<P0>(&self, pszattributes: P0, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).MatchesAttributes)(::windows::core::Interface::as_raw(self), pszattributes.into_param().abi(), pfmatches).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpObjectToken, ::windows::core::IUnknown, ISpDataKey);
impl ::core::cmp::PartialEq for ISpObjectToken {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpObjectToken {}
impl ::core::fmt::Debug for ISpObjectToken {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpObjectToken").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpObjectToken {
    type Vtable = ISpObjectToken_Vtbl;
}
impl ::core::clone::Clone for ISpObjectToken {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpObjectToken {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14056589_e16c_11d2_bb90_00c04f8ee6c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpObjectToken_Vtbl {
    pub base__: ISpDataKey_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcategoryid: ::windows::core::PCWSTR, psztokenid: ::windows::core::PCWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetId: usize,
    pub GetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszcomemtokenid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptokencategory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStorageFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsidcaller: *const ::windows::core::GUID, pszvaluename: ::windows::core::PCWSTR, pszfilenamespecifier: ::windows::core::PCWSTR, nfolder: u32, ppszfilepath: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveStorageFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsidcaller: *const ::windows::core::GUID, pszkeyname: ::windows::core::PCWSTR, fdeletefile: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveStorageFileName: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsidcaller: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUISupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztypeofui: ::windows::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: *mut ::core::ffi::c_void, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUISupported: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, psztitle: ::windows::core::PCWSTR, psztypeofui: ::windows::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayUI: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MatchesAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszattributes: ::windows::core::PCWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MatchesAttributes: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpObjectTokenCategory(::windows::core::IUnknown);
impl ISpObjectTokenCategory {
    pub unsafe fn SetData<P0>(&self, pszvaluename: P0, cbdata: u32, pdata: *const u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetData)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), cbdata, pdata).ok()
    }
    pub unsafe fn GetData<P0>(&self, pszvaluename: P0, pcbdata: *mut u32, pdata: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetData)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), pcbdata, pdata).ok()
    }
    pub unsafe fn SetStringValue<P0, P1>(&self, pszvaluename: P0, pszvalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetStringValue)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), pszvalue.into_param().abi()).ok()
    }
    pub unsafe fn GetStringValue<P0>(&self, pszvaluename: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetStringValue)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDWORD<P0>(&self, pszvaluename: P0, dwvalue: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDWORD)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), dwvalue).ok()
    }
    pub unsafe fn GetDWORD<P0>(&self, pszvaluename: P0, pdwvalue: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetDWORD)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), pdwvalue).ok()
    }
    pub unsafe fn OpenKey<P0>(&self, pszsubkeyname: P0) -> ::windows::core::Result<ISpDataKey>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISpDataKey>();
        (::windows::core::Interface::vtable(self).base__.OpenKey)(::windows::core::Interface::as_raw(self), pszsubkeyname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateKey<P0>(&self, pszsubkey: P0) -> ::windows::core::Result<ISpDataKey>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISpDataKey>();
        (::windows::core::Interface::vtable(self).base__.CreateKey)(::windows::core::Interface::as_raw(self), pszsubkey.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteKey<P0>(&self, pszsubkey: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteKey)(::windows::core::Interface::as_raw(self), pszsubkey.into_param().abi()).ok()
    }
    pub unsafe fn DeleteValue<P0>(&self, pszvaluename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteValue)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi()).ok()
    }
    pub unsafe fn EnumKeys(&self, index: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.EnumKeys)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumValues(&self, index: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.EnumValues)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<P0, P1>(&self, pszcategoryid: P0, fcreateifnotexist: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetId)(::windows::core::Interface::as_raw(self), pszcategoryid.into_param().abi(), fcreateifnotexist.into_param().abi()).ok()
    }
    pub unsafe fn GetId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDataKey(&self, spdkl: SPDATAKEYLOCATION) -> ::windows::core::Result<ISpDataKey> {
        let mut result__ = ::windows::core::zeroed::<ISpDataKey>();
        (::windows::core::Interface::vtable(self).GetDataKey)(::windows::core::Interface::as_raw(self), spdkl, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumTokens<P0, P1>(&self, pzsreqattribs: P0, pszoptattribs: P1) -> ::windows::core::Result<IEnumSpObjectTokens>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IEnumSpObjectTokens>();
        (::windows::core::Interface::vtable(self).EnumTokens)(::windows::core::Interface::as_raw(self), pzsreqattribs.into_param().abi(), pszoptattribs.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDefaultTokenId<P0>(&self, psztokenid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDefaultTokenId)(::windows::core::Interface::as_raw(self), psztokenid.into_param().abi()).ok()
    }
    pub unsafe fn GetDefaultTokenId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetDefaultTokenId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISpObjectTokenCategory, ::windows::core::IUnknown, ISpDataKey);
impl ::core::cmp::PartialEq for ISpObjectTokenCategory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpObjectTokenCategory {}
impl ::core::fmt::Debug for ISpObjectTokenCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpObjectTokenCategory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpObjectTokenCategory {
    type Vtable = ISpObjectTokenCategory_Vtbl;
}
impl ::core::clone::Clone for ISpObjectTokenCategory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpObjectTokenCategory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d3d3845_39af_4850_bbf9_40b49780011d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpObjectTokenCategory_Vtbl {
    pub base__: ISpDataKey_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcategoryid: ::windows::core::PCWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetId: usize,
    pub GetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszcomemcategoryid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetDataKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, spdkl: SPDATAKEYLOCATION, ppdatakey: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumTokens: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pzsreqattribs: ::windows::core::PCWSTR, pszoptattribs: ::windows::core::PCWSTR, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDefaultTokenId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztokenid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetDefaultTokenId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszcomemtokenid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpObjectTokenInit(::windows::core::IUnknown);
impl ISpObjectTokenInit {
    pub unsafe fn SetData<P0>(&self, pszvaluename: P0, cbdata: u32, pdata: *const u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetData)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), cbdata, pdata).ok()
    }
    pub unsafe fn GetData<P0>(&self, pszvaluename: P0, pcbdata: *mut u32, pdata: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.GetData)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), pcbdata, pdata).ok()
    }
    pub unsafe fn SetStringValue<P0, P1>(&self, pszvaluename: P0, pszvalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetStringValue)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), pszvalue.into_param().abi()).ok()
    }
    pub unsafe fn GetStringValue<P0>(&self, pszvaluename: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.GetStringValue)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDWORD<P0>(&self, pszvaluename: P0, dwvalue: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetDWORD)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), dwvalue).ok()
    }
    pub unsafe fn GetDWORD<P0>(&self, pszvaluename: P0, pdwvalue: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.GetDWORD)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), pdwvalue).ok()
    }
    pub unsafe fn OpenKey<P0>(&self, pszsubkeyname: P0) -> ::windows::core::Result<ISpDataKey>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISpDataKey>();
        (::windows::core::Interface::vtable(self).base__.base__.OpenKey)(::windows::core::Interface::as_raw(self), pszsubkeyname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateKey<P0>(&self, pszsubkey: P0) -> ::windows::core::Result<ISpDataKey>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISpDataKey>();
        (::windows::core::Interface::vtable(self).base__.base__.CreateKey)(::windows::core::Interface::as_raw(self), pszsubkey.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteKey<P0>(&self, pszsubkey: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteKey)(::windows::core::Interface::as_raw(self), pszsubkey.into_param().abi()).ok()
    }
    pub unsafe fn DeleteValue<P0>(&self, pszvaluename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.DeleteValue)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi()).ok()
    }
    pub unsafe fn EnumKeys(&self, index: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.EnumKeys)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumValues(&self, index: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.base__.EnumValues)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<P0, P1, P2>(&self, pszcategoryid: P0, psztokenid: P1, fcreateifnotexist: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.SetId)(::windows::core::Interface::as_raw(self), pszcategoryid.into_param().abi(), psztokenid.into_param().abi(), fcreateifnotexist.into_param().abi()).ok()
    }
    pub unsafe fn GetId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCategory(&self) -> ::windows::core::Result<ISpObjectTokenCategory> {
        let mut result__ = ::windows::core::zeroed::<ISpObjectTokenCategory>();
        (::windows::core::Interface::vtable(self).base__.GetCategory)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateInstance<P0>(&self, punkouter: P0, dwclscontext: u32, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.CreateInstance)(::windows::core::Interface::as_raw(self), punkouter.into_param().abi(), dwclscontext, riid, ppvobject).ok()
    }
    pub unsafe fn GetStorageFileName<P0, P1>(&self, clsidcaller: *const ::windows::core::GUID, pszvaluename: P0, pszfilenamespecifier: P1, nfolder: u32) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetStorageFileName)(::windows::core::Interface::as_raw(self), clsidcaller, pszvaluename.into_param().abi(), pszfilenamespecifier.into_param().abi(), nfolder, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveStorageFileName<P0, P1>(&self, clsidcaller: *const ::windows::core::GUID, pszkeyname: P0, fdeletefile: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.RemoveStorageFileName)(::windows::core::Interface::as_raw(self), clsidcaller, pszkeyname.into_param().abi(), fdeletefile.into_param().abi()).ok()
    }
    pub unsafe fn Remove(&self, pclsidcaller: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Remove)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pclsidcaller.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUISupported<P0, P1>(&self, psztypeofui: P0, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: P1, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.IsUISupported)(::windows::core::Interface::as_raw(self), psztypeofui.into_param().abi(), pvextradata, cbextradata, punkobject.into_param().abi(), pfsupported).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayUI<P0, P1, P2, P3>(&self, hwndparent: P0, psztitle: P1, psztypeofui: P2, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P3: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.DisplayUI)(::windows::core::Interface::as_raw(self), hwndparent.into_param().abi(), psztitle.into_param().abi(), psztypeofui.into_param().abi(), pvextradata, cbextradata, punkobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MatchesAttributes<P0>(&self, pszattributes: P0, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.MatchesAttributes)(::windows::core::Interface::as_raw(self), pszattributes.into_param().abi(), pfmatches).ok()
    }
    pub unsafe fn InitFromDataKey<P0, P1, P2>(&self, pszcategoryid: P0, psztokenid: P1, pdatakey: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<ISpDataKey>,
    {
        (::windows::core::Interface::vtable(self).InitFromDataKey)(::windows::core::Interface::as_raw(self), pszcategoryid.into_param().abi(), psztokenid.into_param().abi(), pdatakey.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpObjectTokenInit, ::windows::core::IUnknown, ISpDataKey, ISpObjectToken);
impl ::core::cmp::PartialEq for ISpObjectTokenInit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpObjectTokenInit {}
impl ::core::fmt::Debug for ISpObjectTokenInit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpObjectTokenInit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpObjectTokenInit {
    type Vtable = ISpObjectTokenInit_Vtbl;
}
impl ::core::clone::Clone for ISpObjectTokenInit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpObjectTokenInit {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8aab0cf_346f_49d8_9499_c8b03f161d51);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpObjectTokenInit_Vtbl {
    pub base__: ISpObjectToken_Vtbl,
    pub InitFromDataKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcategoryid: ::windows::core::PCWSTR, psztokenid: ::windows::core::PCWSTR, pdatakey: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpObjectWithToken(::windows::core::IUnknown);
impl ISpObjectWithToken {
    pub unsafe fn SetObjectToken<P0>(&self, ptoken: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpObjectToken>,
    {
        (::windows::core::Interface::vtable(self).SetObjectToken)(::windows::core::Interface::as_raw(self), ptoken.into_param().abi()).ok()
    }
    pub unsafe fn GetObjectToken(&self) -> ::windows::core::Result<ISpObjectToken> {
        let mut result__ = ::windows::core::zeroed::<ISpObjectToken>();
        (::windows::core::Interface::vtable(self).GetObjectToken)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISpObjectWithToken, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpObjectWithToken {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpObjectWithToken {}
impl ::core::fmt::Debug for ISpObjectWithToken {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpObjectWithToken").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpObjectWithToken {
    type Vtable = ISpObjectWithToken_Vtbl;
}
impl ::core::clone::Clone for ISpObjectWithToken {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpObjectWithToken {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b559f40_e952_11d2_bb91_00c04f8ee6c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpObjectWithToken_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetObjectToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptoken: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetObjectToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptoken: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpPhoneConverter(::windows::core::IUnknown);
impl ISpPhoneConverter {
    pub unsafe fn SetObjectToken<P0>(&self, ptoken: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpObjectToken>,
    {
        (::windows::core::Interface::vtable(self).base__.SetObjectToken)(::windows::core::Interface::as_raw(self), ptoken.into_param().abi()).ok()
    }
    pub unsafe fn GetObjectToken(&self) -> ::windows::core::Result<ISpObjectToken> {
        let mut result__ = ::windows::core::zeroed::<ISpObjectToken>();
        (::windows::core::Interface::vtable(self).base__.GetObjectToken)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PhoneToId<P0>(&self, pszphone: P0) -> ::windows::core::Result<u16>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).PhoneToId)(::windows::core::Interface::as_raw(self), pszphone.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn IdToPhone(&self, pid: *const u16, pszphone: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IdToPhone)(::windows::core::Interface::as_raw(self), pid, ::core::mem::transmute(pszphone)).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpPhoneConverter, ::windows::core::IUnknown, ISpObjectWithToken);
impl ::core::cmp::PartialEq for ISpPhoneConverter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpPhoneConverter {}
impl ::core::fmt::Debug for ISpPhoneConverter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpPhoneConverter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpPhoneConverter {
    type Vtable = ISpPhoneConverter_Vtbl;
}
impl ::core::clone::Clone for ISpPhoneConverter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpPhoneConverter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8445c581_0cac_4a38_abfe_9b2ce2826455);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpPhoneConverter_Vtbl {
    pub base__: ISpObjectWithToken_Vtbl,
    pub PhoneToId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszphone: ::windows::core::PCWSTR, pid: *mut u16) -> ::windows::core::HRESULT,
    pub IdToPhone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pid: *const u16, pszphone: ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpPhoneticAlphabetConverter(::windows::core::IUnknown);
impl ISpPhoneticAlphabetConverter {
    pub unsafe fn GetLangId(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::windows::core::zeroed::<u16>();
        (::windows::core::Interface::vtable(self).GetLangId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLangId(&self, langid: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLangId)(::windows::core::Interface::as_raw(self), langid).ok()
    }
    pub unsafe fn SAPI2UPS(&self, pszsapiid: *const u16, pszupsid: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SAPI2UPS)(::windows::core::Interface::as_raw(self), pszsapiid, ::core::mem::transmute(pszupsid.as_ptr()), pszupsid.len() as _).ok()
    }
    pub unsafe fn UPS2SAPI(&self, pszupsid: *const u16, pszsapiid: &mut [u16]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UPS2SAPI)(::windows::core::Interface::as_raw(self), pszupsid, ::core::mem::transmute(pszsapiid.as_ptr()), pszsapiid.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMaxConvertLength<P0>(&self, csrclength: u32, bsapi2ups: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetMaxConvertLength)(::windows::core::Interface::as_raw(self), csrclength, bsapi2ups.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISpPhoneticAlphabetConverter, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpPhoneticAlphabetConverter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpPhoneticAlphabetConverter {}
impl ::core::fmt::Debug for ISpPhoneticAlphabetConverter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpPhoneticAlphabetConverter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpPhoneticAlphabetConverter {
    type Vtable = ISpPhoneticAlphabetConverter_Vtbl;
}
impl ::core::clone::Clone for ISpPhoneticAlphabetConverter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpPhoneticAlphabetConverter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x133adcd4_19b4_4020_9fdc_842e78253b17);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpPhoneticAlphabetConverter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetLangId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plangid: *mut u16) -> ::windows::core::HRESULT,
    pub SetLangId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16) -> ::windows::core::HRESULT,
    pub SAPI2UPS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszsapiid: *const u16, pszupsid: *mut u16, cmaxlength: u32) -> ::windows::core::HRESULT,
    pub UPS2SAPI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszupsid: *const u16, pszsapiid: *mut u16, cmaxlength: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMaxConvertLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, csrclength: u32, bsapi2ups: super::super::Foundation::BOOL, pcmaxdestlength: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMaxConvertLength: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpPhoneticAlphabetSelection(::windows::core::IUnknown);
impl ISpPhoneticAlphabetSelection {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsAlphabetUPS(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsAlphabetUPS)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAlphabetToUPS<P0>(&self, fforceups: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetAlphabetToUPS)(::windows::core::Interface::as_raw(self), fforceups.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpPhoneticAlphabetSelection, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpPhoneticAlphabetSelection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpPhoneticAlphabetSelection {}
impl ::core::fmt::Debug for ISpPhoneticAlphabetSelection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpPhoneticAlphabetSelection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpPhoneticAlphabetSelection {
    type Vtable = ISpPhoneticAlphabetSelection_Vtbl;
}
impl ::core::clone::Clone for ISpPhoneticAlphabetSelection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpPhoneticAlphabetSelection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2745efd_42ce_48ca_81f1_a96e02538a90);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpPhoneticAlphabetSelection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsAlphabetUPS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisups: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsAlphabetUPS: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAlphabetToUPS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fforceups: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAlphabetToUPS: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpPhrase(::windows::core::IUnknown);
impl ISpPhrase {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetPhrase(&self) -> ::windows::core::Result<*mut SPPHRASE> {
        let mut result__ = ::windows::core::zeroed::<*mut SPPHRASE>();
        (::windows::core::Interface::vtable(self).GetPhrase)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSerializedPhrase(&self) -> ::windows::core::Result<*mut SPSERIALIZEDPHRASE> {
        let mut result__ = ::windows::core::zeroed::<*mut SPSERIALIZEDPHRASE>();
        (::windows::core::Interface::vtable(self).GetSerializedPhrase)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText<P0>(&self, ulstart: u32, ulcount: u32, fusetextreplacements: P0, ppszcomemtext: *mut ::windows::core::PWSTR, pbdisplayattributes: ::core::option::Option<*mut u8>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).GetText)(::windows::core::Interface::as_raw(self), ulstart, ulcount, fusetextreplacements.into_param().abi(), ppszcomemtext, ::core::mem::transmute(pbdisplayattributes.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Discard(&self, dwvaluetypes: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Discard)(::windows::core::Interface::as_raw(self), dwvaluetypes).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpPhrase, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpPhrase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpPhrase {}
impl ::core::fmt::Debug for ISpPhrase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpPhrase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpPhrase {
    type Vtable = ISpPhrase_Vtbl;
}
impl ::core::clone::Clone for ISpPhrase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpPhrase {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a5c0354_b621_4b5a_8791_d306ed379e53);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpPhrase_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetPhrase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcomemphrase: *mut *mut SPPHRASE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetPhrase: usize,
    pub GetSerializedPhrase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcomemphrase: *mut *mut SPSERIALIZEDPHRASE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulstart: u32, ulcount: u32, fusetextreplacements: super::super::Foundation::BOOL, ppszcomemtext: *mut ::windows::core::PWSTR, pbdisplayattributes: *mut u8) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetText: usize,
    pub Discard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwvaluetypes: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpPhrase2(::windows::core::IUnknown);
impl ISpPhrase2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetPhrase(&self) -> ::windows::core::Result<*mut SPPHRASE> {
        let mut result__ = ::windows::core::zeroed::<*mut SPPHRASE>();
        (::windows::core::Interface::vtable(self).base__.GetPhrase)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSerializedPhrase(&self) -> ::windows::core::Result<*mut SPSERIALIZEDPHRASE> {
        let mut result__ = ::windows::core::zeroed::<*mut SPSERIALIZEDPHRASE>();
        (::windows::core::Interface::vtable(self).base__.GetSerializedPhrase)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText<P0>(&self, ulstart: u32, ulcount: u32, fusetextreplacements: P0, ppszcomemtext: *mut ::windows::core::PWSTR, pbdisplayattributes: ::core::option::Option<*mut u8>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.GetText)(::windows::core::Interface::as_raw(self), ulstart, ulcount, fusetextreplacements.into_param().abi(), ppszcomemtext, ::core::mem::transmute(pbdisplayattributes.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Discard(&self, dwvaluetypes: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Discard)(::windows::core::Interface::as_raw(self), dwvaluetypes).ok()
    }
    pub unsafe fn GetXMLResult(&self, ppszcomemxmlresult: *mut ::windows::core::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetXMLResult)(::windows::core::Interface::as_raw(self), ppszcomemxmlresult, options).ok()
    }
    pub unsafe fn GetXMLErrorInfo(&self, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetXMLErrorInfo)(::windows::core::Interface::as_raw(self), psemanticerrorinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAudio(&self, ulstartelement: u32, celements: u32) -> ::windows::core::Result<ISpStreamFormat> {
        let mut result__ = ::windows::core::zeroed::<ISpStreamFormat>();
        (::windows::core::Interface::vtable(self).GetAudio)(::windows::core::Interface::as_raw(self), ulstartelement, celements, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISpPhrase2, ::windows::core::IUnknown, ISpPhrase);
impl ::core::cmp::PartialEq for ISpPhrase2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpPhrase2 {}
impl ::core::fmt::Debug for ISpPhrase2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpPhrase2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpPhrase2 {
    type Vtable = ISpPhrase2_Vtbl;
}
impl ::core::clone::Clone for ISpPhrase2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpPhrase2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf264da52_e457_4696_b856_a737b717af79);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpPhrase2_Vtbl {
    pub base__: ISpPhrase_Vtbl,
    pub GetXMLResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszcomemxmlresult: *mut ::windows::core::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows::core::HRESULT,
    pub GetXMLErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAudio: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpPhraseAlt(::windows::core::IUnknown);
impl ISpPhraseAlt {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetPhrase(&self) -> ::windows::core::Result<*mut SPPHRASE> {
        let mut result__ = ::windows::core::zeroed::<*mut SPPHRASE>();
        (::windows::core::Interface::vtable(self).base__.GetPhrase)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSerializedPhrase(&self) -> ::windows::core::Result<*mut SPSERIALIZEDPHRASE> {
        let mut result__ = ::windows::core::zeroed::<*mut SPSERIALIZEDPHRASE>();
        (::windows::core::Interface::vtable(self).base__.GetSerializedPhrase)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText<P0>(&self, ulstart: u32, ulcount: u32, fusetextreplacements: P0, ppszcomemtext: *mut ::windows::core::PWSTR, pbdisplayattributes: ::core::option::Option<*mut u8>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.GetText)(::windows::core::Interface::as_raw(self), ulstart, ulcount, fusetextreplacements.into_param().abi(), ppszcomemtext, ::core::mem::transmute(pbdisplayattributes.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Discard(&self, dwvaluetypes: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Discard)(::windows::core::Interface::as_raw(self), dwvaluetypes).ok()
    }
    pub unsafe fn GetAltInfo(&self, ppparent: *mut ::core::option::Option<ISpPhrase>, pulstartelementinparent: *mut u32, pcelementsinparent: *mut u32, pcelementsinalt: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAltInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppparent), pulstartelementinparent, pcelementsinparent, pcelementsinalt).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpPhraseAlt, ::windows::core::IUnknown, ISpPhrase);
impl ::core::cmp::PartialEq for ISpPhraseAlt {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpPhraseAlt {}
impl ::core::fmt::Debug for ISpPhraseAlt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpPhraseAlt").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpPhraseAlt {
    type Vtable = ISpPhraseAlt_Vtbl;
}
impl ::core::clone::Clone for ISpPhraseAlt {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpPhraseAlt {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fcebc98_4e49_4067_9c6c_d86a0e092e3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpPhraseAlt_Vtbl {
    pub base__: ISpPhrase_Vtbl,
    pub GetAltInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppparent: *mut *mut ::core::ffi::c_void, pulstartelementinparent: *mut u32, pcelementsinparent: *mut u32, pcelementsinalt: *mut u32) -> ::windows::core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpProperties(::windows::core::IUnknown);
impl ISpProperties {
    pub unsafe fn SetPropertyNum<P0>(&self, pname: P0, lvalue: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetPropertyNum)(::windows::core::Interface::as_raw(self), pname.into_param().abi(), lvalue).ok()
    }
    pub unsafe fn GetPropertyNum<P0>(&self, pname: P0, plvalue: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetPropertyNum)(::windows::core::Interface::as_raw(self), pname.into_param().abi(), plvalue).ok()
    }
    pub unsafe fn SetPropertyString<P0, P1>(&self, pname: P0, pvalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetPropertyString)(::windows::core::Interface::as_raw(self), pname.into_param().abi(), pvalue.into_param().abi()).ok()
    }
    pub unsafe fn GetPropertyString<P0>(&self, pname: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetPropertyString)(::windows::core::Interface::as_raw(self), pname.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISpProperties, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpProperties {}
impl ::core::fmt::Debug for ISpProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpProperties {
    type Vtable = ISpProperties_Vtbl;
}
impl ::core::clone::Clone for ISpProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b4fb971_b115_4de1_ad97_e482e3bf6ee4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpProperties_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetPropertyNum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: ::windows::core::PCWSTR, lvalue: i32) -> ::windows::core::HRESULT,
    pub GetPropertyNum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: ::windows::core::PCWSTR, plvalue: *mut i32) -> ::windows::core::HRESULT,
    pub SetPropertyString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: ::windows::core::PCWSTR, pvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetPropertyString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pname: ::windows::core::PCWSTR, ppcomemvalue: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpRecoContext(::windows::core::IUnknown);
impl ISpRecoContext {
    pub unsafe fn SetNotifySink<P0>(&self, pnotifysink: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpNotifySink>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetNotifySink)(::windows::core::Interface::as_raw(self), pnotifysink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotifyWindowMessage<P0, P1, P2>(&self, hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetNotifyWindowMessage)(::windows::core::Interface::as_raw(self), hwnd.into_param().abi(), msg, wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotifyCallbackFunction<P0, P1>(&self, pfncallback: *mut SPNOTIFYCALLBACK, wparam: P0, lparam: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetNotifyCallbackFunction)(::windows::core::Interface::as_raw(self), pfncallback, wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotifyCallbackInterface<P0, P1, P2>(&self, pspcallback: P0, wparam: P1, lparam: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpNotifyCallback>,
        P1: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetNotifyCallbackInterface)(::windows::core::Interface::as_raw(self), pspcallback.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    pub unsafe fn SetNotifyWin32Event(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetNotifyWin32Event)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForNotifyEvent(&self, dwmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.WaitForNotifyEvent)(::windows::core::Interface::as_raw(self), dwmilliseconds).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNotifyEventHandle(&self) -> super::super::Foundation::HANDLE {
        (::windows::core::Interface::vtable(self).base__.base__.GetNotifyEventHandle)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetInterest(&self, ulleventinterest: u64, ullqueuedinterest: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetInterest)(::windows::core::Interface::as_raw(self), ulleventinterest, ullqueuedinterest).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEvents(&self, ulcount: u32, peventarray: *mut SPEVENT, pulfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetEvents)(::windows::core::Interface::as_raw(self), ulcount, peventarray, pulfetched).ok()
    }
    pub unsafe fn GetInfo(&self, pinfo: *mut SPEVENTSOURCEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetInfo)(::windows::core::Interface::as_raw(self), pinfo).ok()
    }
    pub unsafe fn GetRecognizer(&self) -> ::windows::core::Result<ISpRecognizer> {
        let mut result__ = ::windows::core::zeroed::<ISpRecognizer>();
        (::windows::core::Interface::vtable(self).GetRecognizer)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateGrammar(&self, ullgrammarid: u64) -> ::windows::core::Result<ISpRecoGrammar> {
        let mut result__ = ::windows::core::zeroed::<ISpRecoGrammar>();
        (::windows::core::Interface::vtable(self).CreateGrammar)(::windows::core::Interface::as_raw(self), ullgrammarid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStatus(&self, pstatus: *mut SPRECOCONTEXTSTATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), pstatus).ok()
    }
    pub unsafe fn GetMaxAlternates(&self, pcalternates: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMaxAlternates)(::windows::core::Interface::as_raw(self), pcalternates).ok()
    }
    pub unsafe fn SetMaxAlternates(&self, calternates: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMaxAlternates)(::windows::core::Interface::as_raw(self), calternates).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetAudioOptions(&self, options: SPAUDIOOPTIONS, paudioformatid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAudioOptions)(::windows::core::Interface::as_raw(self), options, paudioformatid, pwaveformatex).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetAudioOptions(&self, poptions: *mut SPAUDIOOPTIONS, paudioformatid: *mut ::windows::core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAudioOptions)(::windows::core::Interface::as_raw(self), poptions, paudioformatid, ppcomemwfex).ok()
    }
    pub unsafe fn DeserializeResult(&self, pserializedresult: *const SPSERIALIZEDRESULT) -> ::windows::core::Result<ISpRecoResult> {
        let mut result__ = ::windows::core::zeroed::<ISpRecoResult>();
        (::windows::core::Interface::vtable(self).DeserializeResult)(::windows::core::Interface::as_raw(self), pserializedresult, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Bookmark<P0>(&self, options: SPBOOKMARKOPTIONS, ullstreamposition: u64, lparamevent: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).Bookmark)(::windows::core::Interface::as_raw(self), options, ullstreamposition, lparamevent.into_param().abi()).ok()
    }
    pub unsafe fn SetAdaptationData<P0>(&self, padaptationdata: P0, cch: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetAdaptationData)(::windows::core::Interface::as_raw(self), padaptationdata.into_param().abi(), cch).ok()
    }
    pub unsafe fn Pause(&self, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Pause)(::windows::core::Interface::as_raw(self), dwreserved).ok()
    }
    pub unsafe fn Resume(&self, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resume)(::windows::core::Interface::as_raw(self), dwreserved).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVoice<P0, P1>(&self, pvoice: P0, fallowformatchanges: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpVoice>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetVoice)(::windows::core::Interface::as_raw(self), pvoice.into_param().abi(), fallowformatchanges.into_param().abi()).ok()
    }
    pub unsafe fn GetVoice(&self) -> ::windows::core::Result<ISpVoice> {
        let mut result__ = ::windows::core::zeroed::<ISpVoice>();
        (::windows::core::Interface::vtable(self).GetVoice)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetVoicePurgeEvent(&self, ulleventinterest: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVoicePurgeEvent)(::windows::core::Interface::as_raw(self), ulleventinterest).ok()
    }
    pub unsafe fn GetVoicePurgeEvent(&self, pulleventinterest: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVoicePurgeEvent)(::windows::core::Interface::as_raw(self), pulleventinterest).ok()
    }
    pub unsafe fn SetContextState(&self, econtextstate: SPCONTEXTSTATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetContextState)(::windows::core::Interface::as_raw(self), econtextstate).ok()
    }
    pub unsafe fn GetContextState(&self, pecontextstate: *mut SPCONTEXTSTATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetContextState)(::windows::core::Interface::as_raw(self), pecontextstate).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpRecoContext, ::windows::core::IUnknown, ISpNotifySource, ISpEventSource);
impl ::core::cmp::PartialEq for ISpRecoContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpRecoContext {}
impl ::core::fmt::Debug for ISpRecoContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpRecoContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpRecoContext {
    type Vtable = ISpRecoContext_Vtbl;
}
impl ::core::clone::Clone for ISpRecoContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpRecoContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf740a62f_7c15_489e_8234_940a33d9272d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpRecoContext_Vtbl {
    pub base__: ISpEventSource_Vtbl,
    pub GetRecognizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprecognizer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateGrammar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ullgrammarid: u64, ppgrammar: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut SPRECOCONTEXTSTATUS) -> ::windows::core::HRESULT,
    pub GetMaxAlternates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcalternates: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaxAlternates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, calternates: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub SetAudioOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: SPAUDIOOPTIONS, paudioformatid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    SetAudioOptions: usize,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetAudioOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poptions: *mut SPAUDIOOPTIONS, paudioformatid: *mut ::windows::core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetAudioOptions: usize,
    pub DeserializeResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pserializedresult: *const SPSERIALIZEDRESULT, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Bookmark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: SPBOOKMARKOPTIONS, ullstreamposition: u64, lparamevent: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Bookmark: usize,
    pub SetAdaptationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padaptationdata: ::windows::core::PCWSTR, cch: u32) -> ::windows::core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVoice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvoice: *mut ::core::ffi::c_void, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVoice: usize,
    pub GetVoice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppvoice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetVoicePurgeEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulleventinterest: u64) -> ::windows::core::HRESULT,
    pub GetVoicePurgeEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulleventinterest: *mut u64) -> ::windows::core::HRESULT,
    pub SetContextState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, econtextstate: SPCONTEXTSTATE) -> ::windows::core::HRESULT,
    pub GetContextState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pecontextstate: *mut SPCONTEXTSTATE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpRecoContext2(::windows::core::IUnknown);
impl ISpRecoContext2 {
    pub unsafe fn SetGrammarOptions(&self, egrammaroptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGrammarOptions)(::windows::core::Interface::as_raw(self), egrammaroptions).ok()
    }
    pub unsafe fn GetGrammarOptions(&self, pegrammaroptions: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGrammarOptions)(::windows::core::Interface::as_raw(self), pegrammaroptions).ok()
    }
    pub unsafe fn SetAdaptationData2<P0, P1>(&self, padaptationdata: P0, cch: u32, ptopicname: P1, eadaptationsettings: u32, erelevance: SPADAPTATIONRELEVANCE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetAdaptationData2)(::windows::core::Interface::as_raw(self), padaptationdata.into_param().abi(), cch, ptopicname.into_param().abi(), eadaptationsettings, erelevance).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpRecoContext2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpRecoContext2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpRecoContext2 {}
impl ::core::fmt::Debug for ISpRecoContext2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpRecoContext2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpRecoContext2 {
    type Vtable = ISpRecoContext2_Vtbl;
}
impl ::core::clone::Clone for ISpRecoContext2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpRecoContext2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbead311c_52ff_437f_9464_6b21054ca73d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpRecoContext2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetGrammarOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, egrammaroptions: u32) -> ::windows::core::HRESULT,
    pub GetGrammarOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pegrammaroptions: *mut u32) -> ::windows::core::HRESULT,
    pub SetAdaptationData2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padaptationdata: ::windows::core::PCWSTR, cch: u32, ptopicname: ::windows::core::PCWSTR, eadaptationsettings: u32, erelevance: SPADAPTATIONRELEVANCE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpRecoGrammar(::windows::core::IUnknown);
impl ISpRecoGrammar {
    pub unsafe fn ResetGrammar(&self, newlanguage: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ResetGrammar)(::windows::core::Interface::as_raw(self), newlanguage).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRule<P0, P1>(&self, pszrulename: P0, dwruleid: u32, dwattributes: u32, fcreateifnotexist: P1, phinitialstate: *mut *mut SPSTATEHANDLE__) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.GetRule)(::windows::core::Interface::as_raw(self), pszrulename.into_param().abi(), dwruleid, dwattributes, fcreateifnotexist.into_param().abi(), phinitialstate).ok()
    }
    pub unsafe fn ClearRule(&self, hstate: *mut SPSTATEHANDLE__) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ClearRule)(::windows::core::Interface::as_raw(self), hstate).ok()
    }
    pub unsafe fn CreateNewState(&self, hstate: *mut SPSTATEHANDLE__, phstate: *mut *mut SPSTATEHANDLE__) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CreateNewState)(::windows::core::Interface::as_raw(self), hstate, phstate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddWordTransition<P0, P1>(&self, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, psz: P0, pszseparators: P1, ewordtype: SPGRAMMARWORDTYPE, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddWordTransition)(::windows::core::Interface::as_raw(self), hfromstate, htostate, psz.into_param().abi(), pszseparators.into_param().abi(), ewordtype, weight, ppropinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddRuleTransition(&self, hfromstate: *mut SPSTATEHANDLE__, htostate: *mut SPSTATEHANDLE__, hrule: *mut SPSTATEHANDLE__, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AddRuleTransition)(::windows::core::Interface::as_raw(self), hfromstate, htostate, hrule, weight, ppropinfo).ok()
    }
    pub unsafe fn AddResource<P0, P1>(&self, hrulestate: *mut SPSTATEHANDLE__, pszresourcename: P0, pszresourcevalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddResource)(::windows::core::Interface::as_raw(self), hrulestate, pszresourcename.into_param().abi(), pszresourcevalue.into_param().abi()).ok()
    }
    pub unsafe fn Commit(&self, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Commit)(::windows::core::Interface::as_raw(self), dwreserved).ok()
    }
    pub unsafe fn GetGrammarId(&self, pullgrammarid: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGrammarId)(::windows::core::Interface::as_raw(self), pullgrammarid).ok()
    }
    pub unsafe fn GetRecoContext(&self) -> ::windows::core::Result<ISpRecoContext> {
        let mut result__ = ::windows::core::zeroed::<ISpRecoContext>();
        (::windows::core::Interface::vtable(self).GetRecoContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LoadCmdFromFile<P0>(&self, pszfilename: P0, options: SPLOADOPTIONS) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).LoadCmdFromFile)(::windows::core::Interface::as_raw(self), pszfilename.into_param().abi(), options).ok()
    }
    pub unsafe fn LoadCmdFromObject<P0>(&self, rcid: *const ::windows::core::GUID, pszgrammarname: P0, options: SPLOADOPTIONS) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).LoadCmdFromObject)(::windows::core::Interface::as_raw(self), rcid, pszgrammarname.into_param().abi(), options).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadCmdFromResource<P0, P1, P2>(&self, hmodule: P0, pszresourcename: P1, pszresourcetype: P2, wlanguage: u16, options: SPLOADOPTIONS) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HMODULE>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).LoadCmdFromResource)(::windows::core::Interface::as_raw(self), hmodule.into_param().abi(), pszresourcename.into_param().abi(), pszresourcetype.into_param().abi(), wlanguage, options).ok()
    }
    pub unsafe fn LoadCmdFromMemory(&self, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LoadCmdFromMemory)(::windows::core::Interface::as_raw(self), pgrammar, options).ok()
    }
    pub unsafe fn LoadCmdFromProprietaryGrammar<P0>(&self, rguidparam: *const ::windows::core::GUID, pszstringparam: P0, pvdataprarm: *const ::core::ffi::c_void, cbdatasize: u32, options: SPLOADOPTIONS) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).LoadCmdFromProprietaryGrammar)(::windows::core::Interface::as_raw(self), rguidparam, pszstringparam.into_param().abi(), pvdataprarm, cbdatasize, options).ok()
    }
    pub unsafe fn SetRuleState<P0>(&self, pszname: P0, preserved: *mut ::core::ffi::c_void, newstate: SPRULESTATE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetRuleState)(::windows::core::Interface::as_raw(self), pszname.into_param().abi(), preserved, newstate).ok()
    }
    pub unsafe fn SetRuleIdState(&self, ulruleid: u32, newstate: SPRULESTATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRuleIdState)(::windows::core::Interface::as_raw(self), ulruleid, newstate).ok()
    }
    pub unsafe fn LoadDictation<P0>(&self, psztopicname: P0, options: SPLOADOPTIONS) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).LoadDictation)(::windows::core::Interface::as_raw(self), psztopicname.into_param().abi(), options).ok()
    }
    pub unsafe fn UnloadDictation(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnloadDictation)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetDictationState(&self, newstate: SPRULESTATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDictationState)(::windows::core::Interface::as_raw(self), newstate).ok()
    }
    pub unsafe fn SetWordSequenceData(&self, ptext: ::core::option::Option<&[u16]>, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWordSequenceData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptext.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ptext.as_deref().map_or(0, |slice| slice.len() as _), pinfo).ok()
    }
    pub unsafe fn SetTextSelection(&self, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTextSelection)(::windows::core::Interface::as_raw(self), pinfo).ok()
    }
    pub unsafe fn IsPronounceable<P0>(&self, pszword: P0, pwordpronounceable: *mut SPWORDPRONOUNCEABLE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).IsPronounceable)(::windows::core::Interface::as_raw(self), pszword.into_param().abi(), pwordpronounceable).ok()
    }
    pub unsafe fn SetGrammarState(&self, egrammarstate: SPGRAMMARSTATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGrammarState)(::windows::core::Interface::as_raw(self), egrammarstate).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SaveCmd<P0>(&self, pstream: P0, ppszcomemerrortext: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).SaveCmd)(::windows::core::Interface::as_raw(self), pstream.into_param().abi(), ::core::mem::transmute(ppszcomemerrortext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetGrammarState(&self, pegrammarstate: *mut SPGRAMMARSTATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGrammarState)(::windows::core::Interface::as_raw(self), pegrammarstate).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpRecoGrammar, ::windows::core::IUnknown, ISpGrammarBuilder);
impl ::core::cmp::PartialEq for ISpRecoGrammar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpRecoGrammar {}
impl ::core::fmt::Debug for ISpRecoGrammar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpRecoGrammar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpRecoGrammar {
    type Vtable = ISpRecoGrammar_Vtbl;
}
impl ::core::clone::Clone for ISpRecoGrammar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpRecoGrammar {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2177db29_7f45_47d0_8554_067e91c80502);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpRecoGrammar_Vtbl {
    pub base__: ISpGrammarBuilder_Vtbl,
    pub GetGrammarId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pullgrammarid: *mut u64) -> ::windows::core::HRESULT,
    pub GetRecoContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprecoctxt: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LoadCmdFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilename: ::windows::core::PCWSTR, options: SPLOADOPTIONS) -> ::windows::core::HRESULT,
    pub LoadCmdFromObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rcid: *const ::windows::core::GUID, pszgrammarname: ::windows::core::PCWSTR, options: SPLOADOPTIONS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub LoadCmdFromResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmodule: super::super::Foundation::HMODULE, pszresourcename: ::windows::core::PCWSTR, pszresourcetype: ::windows::core::PCWSTR, wlanguage: u16, options: SPLOADOPTIONS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LoadCmdFromResource: usize,
    pub LoadCmdFromMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS) -> ::windows::core::HRESULT,
    pub LoadCmdFromProprietaryGrammar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidparam: *const ::windows::core::GUID, pszstringparam: ::windows::core::PCWSTR, pvdataprarm: *const ::core::ffi::c_void, cbdatasize: u32, options: SPLOADOPTIONS) -> ::windows::core::HRESULT,
    pub SetRuleState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, preserved: *mut ::core::ffi::c_void, newstate: SPRULESTATE) -> ::windows::core::HRESULT,
    pub SetRuleIdState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulruleid: u32, newstate: SPRULESTATE) -> ::windows::core::HRESULT,
    pub LoadDictation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztopicname: ::windows::core::PCWSTR, options: SPLOADOPTIONS) -> ::windows::core::HRESULT,
    pub UnloadDictation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDictationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newstate: SPRULESTATE) -> ::windows::core::HRESULT,
    pub SetWordSequenceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptext: ::windows::core::PCWSTR, cchtext: u32, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows::core::HRESULT,
    pub SetTextSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows::core::HRESULT,
    pub IsPronounceable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszword: ::windows::core::PCWSTR, pwordpronounceable: *mut SPWORDPRONOUNCEABLE) -> ::windows::core::HRESULT,
    pub SetGrammarState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, egrammarstate: SPGRAMMARSTATE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SaveCmd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, ppszcomemerrortext: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SaveCmd: usize,
    pub GetGrammarState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pegrammarstate: *mut SPGRAMMARSTATE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpRecoGrammar2(::windows::core::IUnknown);
impl ISpRecoGrammar2 {
    pub unsafe fn GetRules(&self, ppcomemrules: *mut *mut SPRULE, punumrules: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRules)(::windows::core::Interface::as_raw(self), ppcomemrules, punumrules).ok()
    }
    pub unsafe fn LoadCmdFromFile2<P0, P1, P2>(&self, pszfilename: P0, options: SPLOADOPTIONS, pszsharinguri: P1, pszbaseuri: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).LoadCmdFromFile2)(::windows::core::Interface::as_raw(self), pszfilename.into_param().abi(), options, pszsharinguri.into_param().abi(), pszbaseuri.into_param().abi()).ok()
    }
    pub unsafe fn LoadCmdFromMemory2<P0, P1>(&self, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS, pszsharinguri: P0, pszbaseuri: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).LoadCmdFromMemory2)(::windows::core::Interface::as_raw(self), pgrammar, options, pszsharinguri.into_param().abi(), pszbaseuri.into_param().abi()).ok()
    }
    pub unsafe fn SetRulePriority<P0>(&self, pszrulename: P0, ulruleid: u32, nrulepriority: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetRulePriority)(::windows::core::Interface::as_raw(self), pszrulename.into_param().abi(), ulruleid, nrulepriority).ok()
    }
    pub unsafe fn SetRuleWeight<P0>(&self, pszrulename: P0, ulruleid: u32, flweight: f32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetRuleWeight)(::windows::core::Interface::as_raw(self), pszrulename.into_param().abi(), ulruleid, flweight).ok()
    }
    pub unsafe fn SetDictationWeight(&self, flweight: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDictationWeight)(::windows::core::Interface::as_raw(self), flweight).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetGrammarLoader<P0>(&self, ploader: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechResourceLoader>,
    {
        (::windows::core::Interface::vtable(self).SetGrammarLoader)(::windows::core::Interface::as_raw(self), ploader.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Urlmon\"`*"]
    #[cfg(feature = "Win32_System_Com_Urlmon")]
    pub unsafe fn SetSMLSecurityManager<P0>(&self, psmlsecuritymanager: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::Urlmon::IInternetSecurityManager>,
    {
        (::windows::core::Interface::vtable(self).SetSMLSecurityManager)(::windows::core::Interface::as_raw(self), psmlsecuritymanager.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpRecoGrammar2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpRecoGrammar2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpRecoGrammar2 {}
impl ::core::fmt::Debug for ISpRecoGrammar2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpRecoGrammar2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpRecoGrammar2 {
    type Vtable = ISpRecoGrammar2_Vtbl;
}
impl ::core::clone::Clone for ISpRecoGrammar2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpRecoGrammar2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b37bc9e_9ed6_44a3_93d3_18f022b79ec3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpRecoGrammar2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetRules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcomemrules: *mut *mut SPRULE, punumrules: *mut u32) -> ::windows::core::HRESULT,
    pub LoadCmdFromFile2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilename: ::windows::core::PCWSTR, options: SPLOADOPTIONS, pszsharinguri: ::windows::core::PCWSTR, pszbaseuri: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub LoadCmdFromMemory2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS, pszsharinguri: ::windows::core::PCWSTR, pszbaseuri: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetRulePriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszrulename: ::windows::core::PCWSTR, ulruleid: u32, nrulepriority: i32) -> ::windows::core::HRESULT,
    pub SetRuleWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszrulename: ::windows::core::PCWSTR, ulruleid: u32, flweight: f32) -> ::windows::core::HRESULT,
    pub SetDictationWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flweight: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetGrammarLoader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ploader: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetGrammarLoader: usize,
    #[cfg(feature = "Win32_System_Com_Urlmon")]
    pub SetSMLSecurityManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psmlsecuritymanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_Urlmon"))]
    SetSMLSecurityManager: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpRecoResult(::windows::core::IUnknown);
impl ISpRecoResult {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetPhrase(&self) -> ::windows::core::Result<*mut SPPHRASE> {
        let mut result__ = ::windows::core::zeroed::<*mut SPPHRASE>();
        (::windows::core::Interface::vtable(self).base__.GetPhrase)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSerializedPhrase(&self) -> ::windows::core::Result<*mut SPSERIALIZEDPHRASE> {
        let mut result__ = ::windows::core::zeroed::<*mut SPSERIALIZEDPHRASE>();
        (::windows::core::Interface::vtable(self).base__.GetSerializedPhrase)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText<P0>(&self, ulstart: u32, ulcount: u32, fusetextreplacements: P0, ppszcomemtext: *mut ::windows::core::PWSTR, pbdisplayattributes: ::core::option::Option<*mut u8>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.GetText)(::windows::core::Interface::as_raw(self), ulstart, ulcount, fusetextreplacements.into_param().abi(), ppszcomemtext, ::core::mem::transmute(pbdisplayattributes.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Discard(&self, dwvaluetypes: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Discard)(::windows::core::Interface::as_raw(self), dwvaluetypes).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetResultTimes(&self, ptimes: *mut SPRECORESULTTIMES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetResultTimes)(::windows::core::Interface::as_raw(self), ptimes).ok()
    }
    pub unsafe fn GetAlternates(&self, ulstartelement: u32, celements: u32, ppphrases: &mut [::core::option::Option<ISpPhraseAlt>], pcphrasesreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAlternates)(::windows::core::Interface::as_raw(self), ulstartelement, celements, ppphrases.len() as _, ::core::mem::transmute(ppphrases.as_ptr()), pcphrasesreturned).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAudio(&self, ulstartelement: u32, celements: u32) -> ::windows::core::Result<ISpStreamFormat> {
        let mut result__ = ::windows::core::zeroed::<ISpStreamFormat>();
        (::windows::core::Interface::vtable(self).GetAudio)(::windows::core::Interface::as_raw(self), ulstartelement, celements, &mut result__).from_abi(result__)
    }
    pub unsafe fn SpeakAudio(&self, ulstartelement: u32, celements: u32, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SpeakAudio)(::windows::core::Interface::as_raw(self), ulstartelement, celements, dwflags, pulstreamnumber).ok()
    }
    pub unsafe fn Serialize(&self, ppcomemserializedresult: *mut *mut SPSERIALIZEDRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Serialize)(::windows::core::Interface::as_raw(self), ppcomemserializedresult).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn ScaleAudio(&self, paudioformatid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ScaleAudio)(::windows::core::Interface::as_raw(self), paudioformatid, pwaveformatex).ok()
    }
    pub unsafe fn GetRecoContext(&self) -> ::windows::core::Result<ISpRecoContext> {
        let mut result__ = ::windows::core::zeroed::<ISpRecoContext>();
        (::windows::core::Interface::vtable(self).GetRecoContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISpRecoResult, ::windows::core::IUnknown, ISpPhrase);
impl ::core::cmp::PartialEq for ISpRecoResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpRecoResult {}
impl ::core::fmt::Debug for ISpRecoResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpRecoResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpRecoResult {
    type Vtable = ISpRecoResult_Vtbl;
}
impl ::core::clone::Clone for ISpRecoResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpRecoResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20b053be_e235_43cd_9a2a_8d17a48b7842);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpRecoResult_Vtbl {
    pub base__: ISpPhrase_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetResultTimes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptimes: *mut SPRECORESULTTIMES) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetResultTimes: usize,
    pub GetAlternates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, ulrequestcount: u32, ppphrases: *mut *mut ::core::ffi::c_void, pcphrasesreturned: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAudio: usize,
    pub SpeakAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows::core::HRESULT,
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcomemserializedresult: *mut *mut SPSERIALIZEDRESULT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub ScaleAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paudioformatid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    ScaleAudio: usize,
    pub GetRecoContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprecocontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpRecoResult2(::windows::core::IUnknown);
impl ISpRecoResult2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetPhrase(&self) -> ::windows::core::Result<*mut SPPHRASE> {
        let mut result__ = ::windows::core::zeroed::<*mut SPPHRASE>();
        (::windows::core::Interface::vtable(self).base__.base__.GetPhrase)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSerializedPhrase(&self) -> ::windows::core::Result<*mut SPSERIALIZEDPHRASE> {
        let mut result__ = ::windows::core::zeroed::<*mut SPSERIALIZEDPHRASE>();
        (::windows::core::Interface::vtable(self).base__.base__.GetSerializedPhrase)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText<P0>(&self, ulstart: u32, ulcount: u32, fusetextreplacements: P0, ppszcomemtext: *mut ::windows::core::PWSTR, pbdisplayattributes: ::core::option::Option<*mut u8>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.GetText)(::windows::core::Interface::as_raw(self), ulstart, ulcount, fusetextreplacements.into_param().abi(), ppszcomemtext, ::core::mem::transmute(pbdisplayattributes.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Discard(&self, dwvaluetypes: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Discard)(::windows::core::Interface::as_raw(self), dwvaluetypes).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetResultTimes(&self, ptimes: *mut SPRECORESULTTIMES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetResultTimes)(::windows::core::Interface::as_raw(self), ptimes).ok()
    }
    pub unsafe fn GetAlternates(&self, ulstartelement: u32, celements: u32, ppphrases: &mut [::core::option::Option<ISpPhraseAlt>], pcphrasesreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetAlternates)(::windows::core::Interface::as_raw(self), ulstartelement, celements, ppphrases.len() as _, ::core::mem::transmute(ppphrases.as_ptr()), pcphrasesreturned).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAudio(&self, ulstartelement: u32, celements: u32) -> ::windows::core::Result<ISpStreamFormat> {
        let mut result__ = ::windows::core::zeroed::<ISpStreamFormat>();
        (::windows::core::Interface::vtable(self).base__.GetAudio)(::windows::core::Interface::as_raw(self), ulstartelement, celements, &mut result__).from_abi(result__)
    }
    pub unsafe fn SpeakAudio(&self, ulstartelement: u32, celements: u32, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SpeakAudio)(::windows::core::Interface::as_raw(self), ulstartelement, celements, dwflags, pulstreamnumber).ok()
    }
    pub unsafe fn Serialize(&self, ppcomemserializedresult: *mut *mut SPSERIALIZEDRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Serialize)(::windows::core::Interface::as_raw(self), ppcomemserializedresult).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn ScaleAudio(&self, paudioformatid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ScaleAudio)(::windows::core::Interface::as_raw(self), paudioformatid, pwaveformatex).ok()
    }
    pub unsafe fn GetRecoContext(&self) -> ::windows::core::Result<ISpRecoContext> {
        let mut result__ = ::windows::core::zeroed::<ISpRecoContext>();
        (::windows::core::Interface::vtable(self).base__.GetRecoContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CommitAlternate<P0>(&self, pphrasealt: P0) -> ::windows::core::Result<ISpRecoResult>
    where
        P0: ::windows::core::IntoParam<ISpPhraseAlt>,
    {
        let mut result__ = ::windows::core::zeroed::<ISpRecoResult>();
        (::windows::core::Interface::vtable(self).CommitAlternate)(::windows::core::Interface::as_raw(self), pphrasealt.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CommitText<P0>(&self, ulstartelement: u32, celements: u32, pszcorrecteddata: P0, ecommitflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).CommitText)(::windows::core::Interface::as_raw(self), ulstartelement, celements, pszcorrecteddata.into_param().abi(), ecommitflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTextFeedback<P0, P1>(&self, pszfeedback: P0, fsuccessful: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetTextFeedback)(::windows::core::Interface::as_raw(self), pszfeedback.into_param().abi(), fsuccessful.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpRecoResult2, ::windows::core::IUnknown, ISpPhrase, ISpRecoResult);
impl ::core::cmp::PartialEq for ISpRecoResult2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpRecoResult2 {}
impl ::core::fmt::Debug for ISpRecoResult2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpRecoResult2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpRecoResult2 {
    type Vtable = ISpRecoResult2_Vtbl;
}
impl ::core::clone::Clone for ISpRecoResult2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpRecoResult2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27cac6c4_88f2_41f2_8817_0c95e59f1e6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpRecoResult2_Vtbl {
    pub base__: ISpRecoResult_Vtbl,
    pub CommitAlternate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphrasealt: *mut ::core::ffi::c_void, ppnewresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CommitText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, pszcorrecteddata: ::windows::core::PCWSTR, ecommitflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTextFeedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfeedback: ::windows::core::PCWSTR, fsuccessful: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTextFeedback: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpRecognizer(::windows::core::IUnknown);
impl ISpRecognizer {
    pub unsafe fn SetPropertyNum<P0>(&self, pname: P0, lvalue: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPropertyNum)(::windows::core::Interface::as_raw(self), pname.into_param().abi(), lvalue).ok()
    }
    pub unsafe fn GetPropertyNum<P0>(&self, pname: P0, plvalue: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetPropertyNum)(::windows::core::Interface::as_raw(self), pname.into_param().abi(), plvalue).ok()
    }
    pub unsafe fn SetPropertyString<P0, P1>(&self, pname: P0, pvalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPropertyString)(::windows::core::Interface::as_raw(self), pname.into_param().abi(), pvalue.into_param().abi()).ok()
    }
    pub unsafe fn GetPropertyString<P0>(&self, pname: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetPropertyString)(::windows::core::Interface::as_raw(self), pname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRecognizer<P0>(&self, precognizer: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpObjectToken>,
    {
        (::windows::core::Interface::vtable(self).SetRecognizer)(::windows::core::Interface::as_raw(self), precognizer.into_param().abi()).ok()
    }
    pub unsafe fn GetRecognizer(&self) -> ::windows::core::Result<ISpObjectToken> {
        let mut result__ = ::windows::core::zeroed::<ISpObjectToken>();
        (::windows::core::Interface::vtable(self).GetRecognizer)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInput<P0, P1>(&self, punkinput: P0, fallowformatchanges: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetInput)(::windows::core::Interface::as_raw(self), punkinput.into_param().abi(), fallowformatchanges.into_param().abi()).ok()
    }
    pub unsafe fn GetInputObjectToken(&self) -> ::windows::core::Result<ISpObjectToken> {
        let mut result__ = ::windows::core::zeroed::<ISpObjectToken>();
        (::windows::core::Interface::vtable(self).GetInputObjectToken)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInputStream(&self) -> ::windows::core::Result<ISpStreamFormat> {
        let mut result__ = ::windows::core::zeroed::<ISpStreamFormat>();
        (::windows::core::Interface::vtable(self).GetInputStream)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateRecoContext(&self) -> ::windows::core::Result<ISpRecoContext> {
        let mut result__ = ::windows::core::zeroed::<ISpRecoContext>();
        (::windows::core::Interface::vtable(self).CreateRecoContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRecoProfile(&self) -> ::windows::core::Result<ISpObjectToken> {
        let mut result__ = ::windows::core::zeroed::<ISpObjectToken>();
        (::windows::core::Interface::vtable(self).GetRecoProfile)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRecoProfile<P0>(&self, ptoken: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpObjectToken>,
    {
        (::windows::core::Interface::vtable(self).SetRecoProfile)(::windows::core::Interface::as_raw(self), ptoken.into_param().abi()).ok()
    }
    pub unsafe fn IsSharedInstance(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsSharedInstance)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetRecoState(&self, pstate: *mut SPRECOSTATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRecoState)(::windows::core::Interface::as_raw(self), pstate).ok()
    }
    pub unsafe fn SetRecoState(&self, newstate: SPRECOSTATE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRecoState)(::windows::core::Interface::as_raw(self), newstate).ok()
    }
    pub unsafe fn GetStatus(&self, pstatus: *mut SPRECOGNIZERSTATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), pstatus).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormat(&self, waveformattype: SPSTREAMFORMATTYPE, pformatid: *mut ::windows::core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFormat)(::windows::core::Interface::as_raw(self), waveformattype, pformatid, ppcomemwfex).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUISupported<P0>(&self, psztypeofui: P0, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).IsUISupported)(::windows::core::Interface::as_raw(self), psztypeofui.into_param().abi(), pvextradata, cbextradata, pfsupported).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayUI<P0, P1, P2>(&self, hwndparent: P0, psztitle: P1, psztypeofui: P2, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DisplayUI)(::windows::core::Interface::as_raw(self), hwndparent.into_param().abi(), psztitle.into_param().abi(), psztypeofui.into_param().abi(), pvextradata, cbextradata).ok()
    }
    pub unsafe fn EmulateRecognition<P0>(&self, pphrase: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpPhrase>,
    {
        (::windows::core::Interface::vtable(self).EmulateRecognition)(::windows::core::Interface::as_raw(self), pphrase.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpRecognizer, ::windows::core::IUnknown, ISpProperties);
impl ::core::cmp::PartialEq for ISpRecognizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpRecognizer {}
impl ::core::fmt::Debug for ISpRecognizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpRecognizer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpRecognizer {
    type Vtable = ISpRecognizer_Vtbl;
}
impl ::core::clone::Clone for ISpRecognizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpRecognizer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2b5f241_daa0_4507_9e16_5a1eaa2b7a5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpRecognizer_Vtbl {
    pub base__: ISpProperties_Vtbl,
    pub SetRecognizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precognizer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRecognizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprecognizer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkinput: *mut ::core::ffi::c_void, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInput: usize,
    pub GetInputObjectToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptoken: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetInputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetInputStream: usize,
    pub CreateRecoContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnewctxt: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRecoProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptoken: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRecoProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptoken: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsSharedInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRecoState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut SPRECOSTATE) -> ::windows::core::HRESULT,
    pub SetRecoState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newstate: SPRECOSTATE) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut SPRECOGNIZERSTATUS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waveformattype: SPSTREAMFORMATTYPE, pformatid: *mut ::windows::core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetFormat: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUISupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztypeofui: ::windows::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUISupported: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, psztitle: ::windows::core::PCWSTR, psztypeofui: ::windows::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayUI: usize,
    pub EmulateRecognition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphrase: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpRecognizer2(::windows::core::IUnknown);
impl ISpRecognizer2 {
    pub unsafe fn EmulateRecognitionEx<P0>(&self, pphrase: P0, dwcompareflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpPhrase>,
    {
        (::windows::core::Interface::vtable(self).EmulateRecognitionEx)(::windows::core::Interface::as_raw(self), pphrase.into_param().abi(), dwcompareflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTrainingState<P0, P1>(&self, fdoingtraining: P0, fadaptfromtrainingdata: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetTrainingState)(::windows::core::Interface::as_raw(self), fdoingtraining.into_param().abi(), fadaptfromtrainingdata.into_param().abi()).ok()
    }
    pub unsafe fn ResetAcousticModelAdaptation(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResetAcousticModelAdaptation)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpRecognizer2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpRecognizer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpRecognizer2 {}
impl ::core::fmt::Debug for ISpRecognizer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpRecognizer2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpRecognizer2 {
    type Vtable = ISpRecognizer2_Vtbl;
}
impl ::core::clone::Clone for ISpRecognizer2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpRecognizer2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fc6d974_c81e_4098_93c5_0147f61ed4d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpRecognizer2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EmulateRecognitionEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pphrase: *mut ::core::ffi::c_void, dwcompareflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTrainingState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fdoingtraining: super::super::Foundation::BOOL, fadaptfromtrainingdata: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTrainingState: usize,
    pub ResetAcousticModelAdaptation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpRegDataKey(::windows::core::IUnknown);
impl ISpRegDataKey {
    pub unsafe fn SetData<P0>(&self, pszvaluename: P0, cbdata: u32, pdata: *const u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetData)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), cbdata, pdata).ok()
    }
    pub unsafe fn GetData<P0>(&self, pszvaluename: P0, pcbdata: *mut u32, pdata: *mut u8) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetData)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), pcbdata, pdata).ok()
    }
    pub unsafe fn SetStringValue<P0, P1>(&self, pszvaluename: P0, pszvalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetStringValue)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), pszvalue.into_param().abi()).ok()
    }
    pub unsafe fn GetStringValue<P0>(&self, pszvaluename: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetStringValue)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDWORD<P0>(&self, pszvaluename: P0, dwvalue: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDWORD)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), dwvalue).ok()
    }
    pub unsafe fn GetDWORD<P0>(&self, pszvaluename: P0, pdwvalue: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetDWORD)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi(), pdwvalue).ok()
    }
    pub unsafe fn OpenKey<P0>(&self, pszsubkeyname: P0) -> ::windows::core::Result<ISpDataKey>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISpDataKey>();
        (::windows::core::Interface::vtable(self).base__.OpenKey)(::windows::core::Interface::as_raw(self), pszsubkeyname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateKey<P0>(&self, pszsubkey: P0) -> ::windows::core::Result<ISpDataKey>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISpDataKey>();
        (::windows::core::Interface::vtable(self).base__.CreateKey)(::windows::core::Interface::as_raw(self), pszsubkey.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteKey<P0>(&self, pszsubkey: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteKey)(::windows::core::Interface::as_raw(self), pszsubkey.into_param().abi()).ok()
    }
    pub unsafe fn DeleteValue<P0>(&self, pszvaluename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.DeleteValue)(::windows::core::Interface::as_raw(self), pszvaluename.into_param().abi()).ok()
    }
    pub unsafe fn EnumKeys(&self, index: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.EnumKeys)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumValues(&self, index: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.EnumValues)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub unsafe fn SetKey<P0, P1>(&self, hkey: P0, freadonly: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Registry::HKEY>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetKey)(::windows::core::Interface::as_raw(self), hkey.into_param().abi(), freadonly.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpRegDataKey, ::windows::core::IUnknown, ISpDataKey);
impl ::core::cmp::PartialEq for ISpRegDataKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpRegDataKey {}
impl ::core::fmt::Debug for ISpRegDataKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpRegDataKey").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpRegDataKey {
    type Vtable = ISpRegDataKey_Vtbl;
}
impl ::core::clone::Clone for ISpRegDataKey {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpRegDataKey {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92a66e2b_c830_4149_83df_6fc2ba1e7a5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpRegDataKey_Vtbl {
    pub base__: ISpDataKey_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub SetKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hkey: super::super::System::Registry::HKEY, freadonly: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Registry")))]
    SetKey: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpResourceManager(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpResourceManager {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryService(&self, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.QueryService)(::windows::core::Interface::as_raw(self), guidservice, riid, ppvobject).ok()
    }
    pub unsafe fn SetObject<P0>(&self, guidserviceid: *const ::windows::core::GUID, punkobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).SetObject)(::windows::core::Interface::as_raw(self), guidserviceid, punkobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetObject<P0>(&self, guidserviceid: *const ::windows::core::GUID, objectclsid: *const ::windows::core::GUID, objectiid: *const ::windows::core::GUID, freleasewhenlastexternalrefreleased: P0, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).GetObject)(::windows::core::Interface::as_raw(self), guidserviceid, objectclsid, objectiid, freleasewhenlastexternalrefreleased.into_param().abi(), ppobject).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpResourceManager, ::windows::core::IUnknown, super::super::System::Com::IServiceProvider);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpResourceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpResourceManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpResourceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpResourceManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpResourceManager {
    type Vtable = ISpResourceManager_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpResourceManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpResourceManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93384e18_5014_43d5_adbb_a78e055926bd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpResourceManager_Vtbl {
    pub base__: super::super::System::Com::IServiceProvider_Vtbl,
    pub SetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidserviceid: *const ::windows::core::GUID, punkobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidserviceid: *const ::windows::core::GUID, objectclsid: *const ::windows::core::GUID, objectiid: *const ::windows::core::GUID, freleasewhenlastexternalrefreleased: super::super::Foundation::BOOL, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetObject: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpSerializeState(::windows::core::IUnknown);
impl ISpSerializeState {
    pub unsafe fn GetSerializedState(&self, ppbdata: *mut *mut u8, pulsize: *mut u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSerializedState)(::windows::core::Interface::as_raw(self), ppbdata, pulsize, dwreserved).ok()
    }
    pub unsafe fn SetSerializedState(&self, pbdata: *const u8, ulsize: u32, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSerializedState)(::windows::core::Interface::as_raw(self), pbdata, ulsize, dwreserved).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpSerializeState, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpSerializeState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpSerializeState {}
impl ::core::fmt::Debug for ISpSerializeState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpSerializeState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpSerializeState {
    type Vtable = ISpSerializeState_Vtbl;
}
impl ::core::clone::Clone for ISpSerializeState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpSerializeState {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x21b501a0_0ec7_46c9_92c3_a2bc784c54b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpSerializeState_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSerializedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbdata: *mut *mut u8, pulsize: *mut u32, dwreserved: u32) -> ::windows::core::HRESULT,
    pub SetSerializedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbdata: *const u8, ulsize: u32, dwreserved: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpShortcut(::windows::core::IUnknown);
impl ISpShortcut {
    pub unsafe fn AddShortcut<P0, P1>(&self, pszdisplay: P0, langid: u16, pszspoken: P1, shtype: SPSHORTCUTTYPE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddShortcut)(::windows::core::Interface::as_raw(self), pszdisplay.into_param().abi(), langid, pszspoken.into_param().abi(), shtype).ok()
    }
    pub unsafe fn RemoveShortcut<P0, P1>(&self, pszdisplay: P0, langid: u16, pszspoken: P1, shtype: SPSHORTCUTTYPE) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).RemoveShortcut)(::windows::core::Interface::as_raw(self), pszdisplay.into_param().abi(), langid, pszspoken.into_param().abi(), shtype).ok()
    }
    pub unsafe fn GetShortcuts(&self, langid: u16, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetShortcuts)(::windows::core::Interface::as_raw(self), langid, pshortcutpairlist).ok()
    }
    pub unsafe fn GetGeneration(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetGeneration)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetWordsFromGenerationChange(&self, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetWordsFromGenerationChange)(::windows::core::Interface::as_raw(self), pdwgeneration, pwordlist).ok()
    }
    pub unsafe fn GetWords(&self, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetWords)(::windows::core::Interface::as_raw(self), pdwgeneration, pdwcookie, pwordlist).ok()
    }
    pub unsafe fn GetShortcutsForGeneration(&self, pdwgeneration: *mut u32, pdwcookie: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetShortcutsForGeneration)(::windows::core::Interface::as_raw(self), pdwgeneration, pdwcookie, pshortcutpairlist).ok()
    }
    pub unsafe fn GetGenerationChange(&self, pdwgeneration: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGenerationChange)(::windows::core::Interface::as_raw(self), pdwgeneration, pshortcutpairlist).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpShortcut, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpShortcut {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpShortcut {}
impl ::core::fmt::Debug for ISpShortcut {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpShortcut").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpShortcut {
    type Vtable = ISpShortcut_Vtbl;
}
impl ::core::clone::Clone for ISpShortcut {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpShortcut {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3df681e2_ea56_11d9_8bde_f66bad1e3f3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpShortcut_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddShortcut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdisplay: ::windows::core::PCWSTR, langid: u16, pszspoken: ::windows::core::PCWSTR, shtype: SPSHORTCUTTYPE) -> ::windows::core::HRESULT,
    pub RemoveShortcut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdisplay: ::windows::core::PCWSTR, langid: u16, pszspoken: ::windows::core::PCWSTR, shtype: SPSHORTCUTTYPE) -> ::windows::core::HRESULT,
    pub GetShortcuts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: u16, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows::core::HRESULT,
    pub GetGeneration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32) -> ::windows::core::HRESULT,
    pub GetWordsFromGenerationChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::HRESULT,
    pub GetWords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows::core::HRESULT,
    pub GetShortcutsForGeneration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pdwcookie: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows::core::HRESULT,
    pub GetGenerationChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpStream(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpStream {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.base__.Read)(::windows::core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.base__.Write)(::windows::core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK, plibnewposition: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Seek)(::windows::core::Interface::as_raw(self), dlibmove, dworigin, ::core::mem::transmute(plibnewposition.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetSize)(::windows::core::Interface::as_raw(self), libnewsize).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyTo<P0>(&self, pstm: P0, cb: u64, pcbread: ::core::option::Option<*mut u64>, pcbwritten: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.CopyTo)(::windows::core::Interface::as_raw(self), pstm.into_param().abi(), cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Commit(&self, grfcommitflags: super::super::System::Com::STGC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Commit)(::windows::core::Interface::as_raw(self), grfcommitflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Revert)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: super::super::System::Com::LOCKTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.LockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.UnlockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Stat(&self, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: super::super::System::Com::STATFLAG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Stat)(::windows::core::Interface::as_raw(self), pstatstg, grfstatflag).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IStream>();
        (::windows::core::Interface::vtable(self).base__.base__.Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormat(&self, pguidformatid: *const ::windows::core::GUID) -> ::windows::core::Result<*mut super::Audio::WAVEFORMATEX> {
        let mut result__ = ::windows::core::zeroed::<*mut super::Audio::WAVEFORMATEX>();
        (::windows::core::Interface::vtable(self).base__.GetFormat)(::windows::core::Interface::as_raw(self), pguidformatid, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
    pub unsafe fn SetBaseStream<P0>(&self, pstream: P0, rguidformat: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).SetBaseStream)(::windows::core::Interface::as_raw(self), pstream.into_param().abi(), rguidformat, pwaveformatex).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBaseStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IStream>();
        (::windows::core::Interface::vtable(self).GetBaseStream)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn BindToFile<P0>(&self, pszfilename: P0, emode: SPFILEMODE, pformatid: ::core::option::Option<*const ::windows::core::GUID>, pwaveformatex: ::core::option::Option<*const super::Audio::WAVEFORMATEX>, ulleventinterest: u64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).BindToFile)(::windows::core::Interface::as_raw(self), pszfilename.into_param().abi(), emode, ::core::mem::transmute(pformatid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pwaveformatex.unwrap_or(::std::ptr::null())), ulleventinterest).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpStream, ::windows::core::IUnknown, super::super::System::Com::ISequentialStream, super::super::System::Com::IStream, ISpStreamFormat);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpStream {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpStream").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpStream {
    type Vtable = ISpStream_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12e3cca9_7518_44c5_a5e7_ba5a79cb929e);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpStream_Vtbl {
    pub base__: ISpStreamFormat_Vtbl,
    #[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
    pub SetBaseStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, rguidformat: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com")))]
    SetBaseStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBaseStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBaseStream: usize,
    #[cfg(feature = "Win32_Media_Audio")]
    pub BindToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfilename: ::windows::core::PCWSTR, emode: SPFILEMODE, pformatid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX, ulleventinterest: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    BindToFile: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpStreamFormat(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpStreamFormat {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.Read)(::windows::core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.Write)(::windows::core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK, plibnewposition: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Seek)(::windows::core::Interface::as_raw(self), dlibmove, dworigin, ::core::mem::transmute(plibnewposition.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetSize)(::windows::core::Interface::as_raw(self), libnewsize).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyTo<P0>(&self, pstm: P0, cb: u64, pcbread: ::core::option::Option<*mut u64>, pcbwritten: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyTo)(::windows::core::Interface::as_raw(self), pstm.into_param().abi(), cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Commit(&self, grfcommitflags: super::super::System::Com::STGC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Commit)(::windows::core::Interface::as_raw(self), grfcommitflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Revert)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: super::super::System::Com::LOCKTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.LockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.UnlockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Stat(&self, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: super::super::System::Com::STATFLAG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Stat)(::windows::core::Interface::as_raw(self), pstatstg, grfstatflag).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IStream>();
        (::windows::core::Interface::vtable(self).base__.Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormat(&self, pguidformatid: *const ::windows::core::GUID) -> ::windows::core::Result<*mut super::Audio::WAVEFORMATEX> {
        let mut result__ = ::windows::core::zeroed::<*mut super::Audio::WAVEFORMATEX>();
        (::windows::core::Interface::vtable(self).GetFormat)(::windows::core::Interface::as_raw(self), pguidformatid, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpStreamFormat, ::windows::core::IUnknown, super::super::System::Com::ISequentialStream, super::super::System::Com::IStream);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpStreamFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpStreamFormat {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpStreamFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpStreamFormat").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpStreamFormat {
    type Vtable = ISpStreamFormat_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpStreamFormat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpStreamFormat {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbed530be_2606_4f4d_a1c0_54c5cda5566f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpStreamFormat_Vtbl {
    pub base__: super::super::System::Com::IStream_Vtbl,
    #[cfg(feature = "Win32_Media_Audio")]
    pub GetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidformatid: *const ::windows::core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    GetFormat: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpStreamFormatConverter(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpStreamFormatConverter {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.base__.Read)(::windows::core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.base__.Write)(::windows::core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK, plibnewposition: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Seek)(::windows::core::Interface::as_raw(self), dlibmove, dworigin, ::core::mem::transmute(plibnewposition.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetSize)(::windows::core::Interface::as_raw(self), libnewsize).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyTo<P0>(&self, pstm: P0, cb: u64, pcbread: ::core::option::Option<*mut u64>, pcbwritten: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.CopyTo)(::windows::core::Interface::as_raw(self), pstm.into_param().abi(), cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Commit(&self, grfcommitflags: super::super::System::Com::STGC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Commit)(::windows::core::Interface::as_raw(self), grfcommitflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Revert)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: super::super::System::Com::LOCKTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.LockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.UnlockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Stat(&self, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: super::super::System::Com::STATFLAG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Stat)(::windows::core::Interface::as_raw(self), pstatstg, grfstatflag).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IStream>();
        (::windows::core::Interface::vtable(self).base__.base__.Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn GetFormat(&self, pguidformatid: *const ::windows::core::GUID) -> ::windows::core::Result<*mut super::Audio::WAVEFORMATEX> {
        let mut result__ = ::windows::core::zeroed::<*mut super::Audio::WAVEFORMATEX>();
        (::windows::core::Interface::vtable(self).base__.GetFormat)(::windows::core::Interface::as_raw(self), pguidformatid, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SetBaseStream<P0, P1, P2>(&self, pstream: P0, fsetformattobasestreamformat: P1, fwritetobasestream: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpStreamFormat>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetBaseStream)(::windows::core::Interface::as_raw(self), pstream.into_param().abi(), fsetformattobasestreamformat.into_param().abi(), fwritetobasestream.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBaseStream(&self) -> ::windows::core::Result<ISpStreamFormat> {
        let mut result__ = ::windows::core::zeroed::<ISpStreamFormat>();
        (::windows::core::Interface::vtable(self).GetBaseStream)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn SetFormat(&self, rguidformatidofconvertedstream: *const ::windows::core::GUID, pwaveformatexofconvertedstream: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFormat)(::windows::core::Interface::as_raw(self), rguidformatidofconvertedstream, pwaveformatexofconvertedstream).ok()
    }
    pub unsafe fn ResetSeekPosition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResetSeekPosition)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ScaleConvertedToBaseOffset(&self, ulloffsetconvertedstream: u64) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).ScaleConvertedToBaseOffset)(::windows::core::Interface::as_raw(self), ulloffsetconvertedstream, &mut result__).from_abi(result__)
    }
    pub unsafe fn ScaleBaseToConvertedOffset(&self, ulloffsetbasestream: u64) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).ScaleBaseToConvertedOffset)(::windows::core::Interface::as_raw(self), ulloffsetbasestream, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpStreamFormatConverter, ::windows::core::IUnknown, super::super::System::Com::ISequentialStream, super::super::System::Com::IStream, ISpStreamFormat);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpStreamFormatConverter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpStreamFormatConverter {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpStreamFormatConverter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpStreamFormatConverter").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpStreamFormatConverter {
    type Vtable = ISpStreamFormatConverter_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpStreamFormatConverter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpStreamFormatConverter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x678a932c_ea71_4446_9b41_78fda6280a29);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpStreamFormatConverter_Vtbl {
    pub base__: ISpStreamFormat_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SetBaseStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, fsetformattobasestreamformat: super::super::Foundation::BOOL, fwritetobasestream: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SetBaseStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBaseStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBaseStream: usize,
    #[cfg(feature = "Win32_Media_Audio")]
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rguidformatidofconvertedstream: *const ::windows::core::GUID, pwaveformatexofconvertedstream: *const super::Audio::WAVEFORMATEX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_Audio"))]
    SetFormat: usize,
    pub ResetSeekPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ScaleConvertedToBaseOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulloffsetconvertedstream: u64, pulloffsetbasestream: *mut u64) -> ::windows::core::HRESULT,
    pub ScaleBaseToConvertedOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulloffsetbasestream: u64, pulloffsetconvertedstream: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpTranscript(::windows::core::IUnknown);
impl ISpTranscript {
    pub unsafe fn GetTranscript(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetTranscript)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AppendTranscript<P0>(&self, psztranscript: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AppendTranscript)(::windows::core::Interface::as_raw(self), psztranscript.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpTranscript, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISpTranscript {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpTranscript {}
impl ::core::fmt::Debug for ISpTranscript {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpTranscript").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpTranscript {
    type Vtable = ISpTranscript_Vtbl;
}
impl ::core::clone::Clone for ISpTranscript {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpTranscript {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10f63bce_201a_11d3_ac70_00c04f8ee6c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpTranscript_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetTranscript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsztranscript: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub AppendTranscript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztranscript: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpVoice(::windows::core::IUnknown);
impl ISpVoice {
    pub unsafe fn SetNotifySink<P0>(&self, pnotifysink: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpNotifySink>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetNotifySink)(::windows::core::Interface::as_raw(self), pnotifysink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotifyWindowMessage<P0, P1, P2>(&self, hwnd: P0, msg: u32, wparam: P1, lparam: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetNotifyWindowMessage)(::windows::core::Interface::as_raw(self), hwnd.into_param().abi(), msg, wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotifyCallbackFunction<P0, P1>(&self, pfncallback: *mut SPNOTIFYCALLBACK, wparam: P0, lparam: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetNotifyCallbackFunction)(::windows::core::Interface::as_raw(self), pfncallback, wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNotifyCallbackInterface<P0, P1, P2>(&self, pspcallback: P0, wparam: P1, lparam: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpNotifyCallback>,
        P1: ::windows::core::IntoParam<super::super::Foundation::WPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetNotifyCallbackInterface)(::windows::core::Interface::as_raw(self), pspcallback.into_param().abi(), wparam.into_param().abi(), lparam.into_param().abi()).ok()
    }
    pub unsafe fn SetNotifyWin32Event(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetNotifyWin32Event)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn WaitForNotifyEvent(&self, dwmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.WaitForNotifyEvent)(::windows::core::Interface::as_raw(self), dwmilliseconds).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNotifyEventHandle(&self) -> super::super::Foundation::HANDLE {
        (::windows::core::Interface::vtable(self).base__.base__.GetNotifyEventHandle)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetInterest(&self, ulleventinterest: u64, ullqueuedinterest: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetInterest)(::windows::core::Interface::as_raw(self), ulleventinterest, ullqueuedinterest).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEvents(&self, ulcount: u32, peventarray: *mut SPEVENT, pulfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetEvents)(::windows::core::Interface::as_raw(self), ulcount, peventarray, pulfetched).ok()
    }
    pub unsafe fn GetInfo(&self, pinfo: *mut SPEVENTSOURCEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetInfo)(::windows::core::Interface::as_raw(self), pinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOutput<P0, P1>(&self, punkoutput: P0, fallowformatchanges: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetOutput)(::windows::core::Interface::as_raw(self), punkoutput.into_param().abi(), fallowformatchanges.into_param().abi()).ok()
    }
    pub unsafe fn GetOutputObjectToken(&self) -> ::windows::core::Result<ISpObjectToken> {
        let mut result__ = ::windows::core::zeroed::<ISpObjectToken>();
        (::windows::core::Interface::vtable(self).GetOutputObjectToken)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetOutputStream(&self) -> ::windows::core::Result<ISpStreamFormat> {
        let mut result__ = ::windows::core::zeroed::<ISpStreamFormat>();
        (::windows::core::Interface::vtable(self).GetOutputStream)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Pause)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resume)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetVoice<P0>(&self, ptoken: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpObjectToken>,
    {
        (::windows::core::Interface::vtable(self).SetVoice)(::windows::core::Interface::as_raw(self), ptoken.into_param().abi()).ok()
    }
    pub unsafe fn GetVoice(&self) -> ::windows::core::Result<ISpObjectToken> {
        let mut result__ = ::windows::core::zeroed::<ISpObjectToken>();
        (::windows::core::Interface::vtable(self).GetVoice)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Speak<P0>(&self, pwcs: P0, dwflags: u32, pulstreamnumber: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Speak)(::windows::core::Interface::as_raw(self), pwcs.into_param().abi(), dwflags, ::core::mem::transmute(pulstreamnumber.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SpeakStream<P0>(&self, pstream: P0, dwflags: u32, pulstreamnumber: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).SpeakStream)(::windows::core::Interface::as_raw(self), pstream.into_param().abi(), dwflags, ::core::mem::transmute(pulstreamnumber.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetStatus(&self, pstatus: *mut SPVOICESTATUS, ppszlastbookmark: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), pstatus, ppszlastbookmark).ok()
    }
    pub unsafe fn Skip<P0>(&self, pitemtype: P0, lnumitems: i32, pulnumskipped: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), pitemtype.into_param().abi(), lnumitems, pulnumskipped).ok()
    }
    pub unsafe fn SetPriority(&self, epriority: SPVPRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPriority)(::windows::core::Interface::as_raw(self), epriority).ok()
    }
    pub unsafe fn GetPriority(&self, pepriority: *mut SPVPRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPriority)(::windows::core::Interface::as_raw(self), pepriority).ok()
    }
    pub unsafe fn SetAlertBoundary(&self, eboundary: SPEVENTENUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAlertBoundary)(::windows::core::Interface::as_raw(self), eboundary).ok()
    }
    pub unsafe fn GetAlertBoundary(&self, peboundary: *mut SPEVENTENUM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAlertBoundary)(::windows::core::Interface::as_raw(self), peboundary).ok()
    }
    pub unsafe fn SetRate(&self, rateadjust: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRate)(::windows::core::Interface::as_raw(self), rateadjust).ok()
    }
    pub unsafe fn GetRate(&self, prateadjust: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRate)(::windows::core::Interface::as_raw(self), prateadjust).ok()
    }
    pub unsafe fn SetVolume(&self, usvolume: u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVolume)(::windows::core::Interface::as_raw(self), usvolume).ok()
    }
    pub unsafe fn GetVolume(&self, pusvolume: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVolume)(::windows::core::Interface::as_raw(self), pusvolume).ok()
    }
    pub unsafe fn WaitUntilDone(&self, mstimeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WaitUntilDone)(::windows::core::Interface::as_raw(self), mstimeout).ok()
    }
    pub unsafe fn SetSyncSpeakTimeout(&self, mstimeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSyncSpeakTimeout)(::windows::core::Interface::as_raw(self), mstimeout).ok()
    }
    pub unsafe fn GetSyncSpeakTimeout(&self, pmstimeout: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSyncSpeakTimeout)(::windows::core::Interface::as_raw(self), pmstimeout).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SpeakCompleteEvent(&self) -> super::super::Foundation::HANDLE {
        (::windows::core::Interface::vtable(self).SpeakCompleteEvent)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUISupported<P0>(&self, psztypeofui: P0, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).IsUISupported)(::windows::core::Interface::as_raw(self), psztypeofui.into_param().abi(), pvextradata, cbextradata, pfsupported).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayUI<P0, P1, P2>(&self, hwndparent: P0, psztitle: P1, psztypeofui: P2, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).DisplayUI)(::windows::core::Interface::as_raw(self), hwndparent.into_param().abi(), psztitle.into_param().abi(), psztypeofui.into_param().abi(), pvextradata, cbextradata).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpVoice, ::windows::core::IUnknown, ISpNotifySource, ISpEventSource);
impl ::core::cmp::PartialEq for ISpVoice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpVoice {}
impl ::core::fmt::Debug for ISpVoice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpVoice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpVoice {
    type Vtable = ISpVoice_Vtbl;
}
impl ::core::clone::Clone for ISpVoice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpVoice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c44df74_72b9_4992_a1ec_ef996e0422d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpVoice_Vtbl {
    pub base__: ISpEventSource_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkoutput: *mut ::core::ffi::c_void, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOutput: usize,
    pub GetOutputObjectToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppobjecttoken: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetOutputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetOutputStream: usize,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetVoice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptoken: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetVoice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pptoken: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Speak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwcs: ::windows::core::PCWSTR, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SpeakStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SpeakStream: usize,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut SPVOICESTATUS, ppszlastbookmark: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitemtype: ::windows::core::PCWSTR, lnumitems: i32, pulnumskipped: *mut u32) -> ::windows::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, epriority: SPVPRIORITY) -> ::windows::core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pepriority: *mut SPVPRIORITY) -> ::windows::core::HRESULT,
    pub SetAlertBoundary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eboundary: SPEVENTENUM) -> ::windows::core::HRESULT,
    pub GetAlertBoundary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peboundary: *mut SPEVENTENUM) -> ::windows::core::HRESULT,
    pub SetRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rateadjust: i32) -> ::windows::core::HRESULT,
    pub GetRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prateadjust: *mut i32) -> ::windows::core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usvolume: u16) -> ::windows::core::HRESULT,
    pub GetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pusvolume: *mut u16) -> ::windows::core::HRESULT,
    pub WaitUntilDone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mstimeout: u32) -> ::windows::core::HRESULT,
    pub SetSyncSpeakTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mstimeout: u32) -> ::windows::core::HRESULT,
    pub GetSyncSpeakTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmstimeout: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SpeakCompleteEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE,
    #[cfg(not(feature = "Win32_Foundation"))]
    SpeakCompleteEvent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUISupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztypeofui: ::windows::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUISupported: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DisplayUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, psztitle: ::windows::core::PCWSTR, psztypeofui: ::windows::core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DisplayUI: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
pub struct ISpXMLRecoResult(::windows::core::IUnknown);
impl ISpXMLRecoResult {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetPhrase(&self) -> ::windows::core::Result<*mut SPPHRASE> {
        let mut result__ = ::windows::core::zeroed::<*mut SPPHRASE>();
        (::windows::core::Interface::vtable(self).base__.base__.GetPhrase)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSerializedPhrase(&self) -> ::windows::core::Result<*mut SPSERIALIZEDPHRASE> {
        let mut result__ = ::windows::core::zeroed::<*mut SPSERIALIZEDPHRASE>();
        (::windows::core::Interface::vtable(self).base__.base__.GetSerializedPhrase)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText<P0>(&self, ulstart: u32, ulcount: u32, fusetextreplacements: P0, ppszcomemtext: *mut ::windows::core::PWSTR, pbdisplayattributes: ::core::option::Option<*mut u8>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.GetText)(::windows::core::Interface::as_raw(self), ulstart, ulcount, fusetextreplacements.into_param().abi(), ppszcomemtext, ::core::mem::transmute(pbdisplayattributes.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Discard(&self, dwvaluetypes: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Discard)(::windows::core::Interface::as_raw(self), dwvaluetypes).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetResultTimes(&self, ptimes: *mut SPRECORESULTTIMES) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetResultTimes)(::windows::core::Interface::as_raw(self), ptimes).ok()
    }
    pub unsafe fn GetAlternates(&self, ulstartelement: u32, celements: u32, ppphrases: &mut [::core::option::Option<ISpPhraseAlt>], pcphrasesreturned: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetAlternates)(::windows::core::Interface::as_raw(self), ulstartelement, celements, ppphrases.len() as _, ::core::mem::transmute(ppphrases.as_ptr()), pcphrasesreturned).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAudio(&self, ulstartelement: u32, celements: u32) -> ::windows::core::Result<ISpStreamFormat> {
        let mut result__ = ::windows::core::zeroed::<ISpStreamFormat>();
        (::windows::core::Interface::vtable(self).base__.GetAudio)(::windows::core::Interface::as_raw(self), ulstartelement, celements, &mut result__).from_abi(result__)
    }
    pub unsafe fn SpeakAudio(&self, ulstartelement: u32, celements: u32, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SpeakAudio)(::windows::core::Interface::as_raw(self), ulstartelement, celements, dwflags, pulstreamnumber).ok()
    }
    pub unsafe fn Serialize(&self, ppcomemserializedresult: *mut *mut SPSERIALIZEDRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Serialize)(::windows::core::Interface::as_raw(self), ppcomemserializedresult).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Audio\"`*"]
    #[cfg(feature = "Win32_Media_Audio")]
    pub unsafe fn ScaleAudio(&self, paudioformatid: *const ::windows::core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ScaleAudio)(::windows::core::Interface::as_raw(self), paudioformatid, pwaveformatex).ok()
    }
    pub unsafe fn GetRecoContext(&self) -> ::windows::core::Result<ISpRecoContext> {
        let mut result__ = ::windows::core::zeroed::<ISpRecoContext>();
        (::windows::core::Interface::vtable(self).base__.GetRecoContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetXMLResult(&self, ppszcomemxmlresult: *mut ::windows::core::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetXMLResult)(::windows::core::Interface::as_raw(self), ppszcomemxmlresult, options).ok()
    }
    pub unsafe fn GetXMLErrorInfo(&self, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetXMLErrorInfo)(::windows::core::Interface::as_raw(self), psemanticerrorinfo).ok()
    }
}
::windows::imp::interface_hierarchy!(ISpXMLRecoResult, ::windows::core::IUnknown, ISpPhrase, ISpRecoResult);
impl ::core::cmp::PartialEq for ISpXMLRecoResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpXMLRecoResult {}
impl ::core::fmt::Debug for ISpXMLRecoResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpXMLRecoResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISpXMLRecoResult {
    type Vtable = ISpXMLRecoResult_Vtbl;
}
impl ::core::clone::Clone for ISpXMLRecoResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISpXMLRecoResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae39362b_45a8_4074_9b9e_ccf49aa2d0b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpXMLRecoResult_Vtbl {
    pub base__: ISpRecoResult_Vtbl,
    pub GetXMLResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszcomemxmlresult: *mut ::windows::core::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows::core::HRESULT,
    pub GetXMLErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechAudio(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechAudio {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Format(&self) -> ::windows::core::Result<ISpeechAudioFormat> {
        let mut result__ = ::windows::core::zeroed::<ISpeechAudioFormat>();
        (::windows::core::Interface::vtable(self).base__.Format)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_Format<P0>(&self, audioformat: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechAudioFormat>,
    {
        (::windows::core::Interface::vtable(self).base__.putref_Format)(::windows::core::Interface::as_raw(self), audioformat.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Read(&self, buffer: *mut super::super::System::Com::VARIANT, numberofbytes: i32, bytesread: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Read)(::windows::core::Interface::as_raw(self), buffer, numberofbytes, bytesread).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Write(&self, buffer: super::super::System::Com::VARIANT) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.Write)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(buffer), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Seek(&self, position: super::super::System::Com::VARIANT, origin: SpeechStreamSeekPositionType) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.Seek)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(position), origin, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Status(&self) -> ::windows::core::Result<ISpeechAudioStatus> {
        let mut result__ = ::windows::core::zeroed::<ISpeechAudioStatus>();
        (::windows::core::Interface::vtable(self).Status)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BufferInfo(&self) -> ::windows::core::Result<ISpeechAudioBufferInfo> {
        let mut result__ = ::windows::core::zeroed::<ISpeechAudioBufferInfo>();
        (::windows::core::Interface::vtable(self).BufferInfo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DefaultFormat(&self) -> ::windows::core::Result<ISpeechAudioFormat> {
        let mut result__ = ::windows::core::zeroed::<ISpeechAudioFormat>();
        (::windows::core::Interface::vtable(self).DefaultFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Volume(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Volume)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetVolume(&self, volume: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVolume)(::windows::core::Interface::as_raw(self), volume).ok()
    }
    pub unsafe fn BufferNotifySize(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).BufferNotifySize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBufferNotifySize(&self, buffernotifysize: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBufferNotifySize)(::windows::core::Interface::as_raw(self), buffernotifysize).ok()
    }
    pub unsafe fn EventHandle(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).EventHandle)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetState(&self, state: SpeechAudioState) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetState)(::windows::core::Interface::as_raw(self), state).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechAudio, ::windows::core::IUnknown, super::super::System::Com::IDispatch, ISpeechBaseStream);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechAudio {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechAudio {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechAudio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechAudio").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechAudio {
    type Vtable = ISpeechAudio_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechAudio {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechAudio {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcff8e175_019e_11d3_a08e_00c04f8ef9b5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechAudio_Vtbl {
    pub base__: ISpeechBaseStream_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Status: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub BufferInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bufferinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    BufferInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DefaultFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamformat: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DefaultFormat: usize,
    pub Volume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volume: *mut i32) -> ::windows::core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volume: i32) -> ::windows::core::HRESULT,
    pub BufferNotifySize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffernotifysize: *mut i32) -> ::windows::core::HRESULT,
    pub SetBufferNotifySize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffernotifysize: i32) -> ::windows::core::HRESULT,
    pub EventHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandle: *mut i32) -> ::windows::core::HRESULT,
    pub SetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: SpeechAudioState) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechAudioBufferInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechAudioBufferInfo {
    pub unsafe fn MinNotification(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).MinNotification)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMinNotification(&self, minnotification: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMinNotification)(::windows::core::Interface::as_raw(self), minnotification).ok()
    }
    pub unsafe fn BufferSize(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).BufferSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBufferSize(&self, buffersize: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBufferSize)(::windows::core::Interface::as_raw(self), buffersize).ok()
    }
    pub unsafe fn EventBias(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).EventBias)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEventBias(&self, eventbias: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventBias)(::windows::core::Interface::as_raw(self), eventbias).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechAudioBufferInfo, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechAudioBufferInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechAudioBufferInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechAudioBufferInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechAudioBufferInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechAudioBufferInfo {
    type Vtable = ISpeechAudioBufferInfo_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechAudioBufferInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechAudioBufferInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11b103d8_1142_4edf_a093_82fb3915f8cc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechAudioBufferInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub MinNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minnotification: *mut i32) -> ::windows::core::HRESULT,
    pub SetMinNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minnotification: i32) -> ::windows::core::HRESULT,
    pub BufferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffersize: *mut i32) -> ::windows::core::HRESULT,
    pub SetBufferSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffersize: i32) -> ::windows::core::HRESULT,
    pub EventBias: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventbias: *mut i32) -> ::windows::core::HRESULT,
    pub SetEventBias: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventbias: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechAudioFormat(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechAudioFormat {
    pub unsafe fn Type(&self) -> ::windows::core::Result<SpeechAudioFormatType> {
        let mut result__ = ::windows::core::zeroed::<SpeechAudioFormatType>();
        (::windows::core::Interface::vtable(self).Type)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetType(&self, audioformat: SpeechAudioFormatType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetType)(::windows::core::Interface::as_raw(self), audioformat).ok()
    }
    pub unsafe fn Guid(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Guid)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetGuid<P0>(&self, guid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetGuid)(::windows::core::Interface::as_raw(self), guid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWaveFormatEx(&self) -> ::windows::core::Result<ISpeechWaveFormatEx> {
        let mut result__ = ::windows::core::zeroed::<ISpeechWaveFormatEx>();
        (::windows::core::Interface::vtable(self).GetWaveFormatEx)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetWaveFormatEx<P0>(&self, speechwaveformatex: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechWaveFormatEx>,
    {
        (::windows::core::Interface::vtable(self).SetWaveFormatEx)(::windows::core::Interface::as_raw(self), speechwaveformatex.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechAudioFormat, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechAudioFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechAudioFormat {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechAudioFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechAudioFormat").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechAudioFormat {
    type Vtable = ISpeechAudioFormat_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechAudioFormat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechAudioFormat {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6e9c590_3e18_40e3_8299_061f98bde7c7);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechAudioFormat_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audioformat: *mut SpeechAudioFormatType) -> ::windows::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audioformat: SpeechAudioFormatType) -> ::windows::core::HRESULT,
    pub Guid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWaveFormatEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, speechwaveformatex: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWaveFormatEx: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetWaveFormatEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, speechwaveformatex: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetWaveFormatEx: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechAudioStatus(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechAudioStatus {
    pub unsafe fn FreeBufferSpace(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).FreeBufferSpace)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NonBlockingIO(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).NonBlockingIO)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<SpeechAudioState> {
        let mut result__ = ::windows::core::zeroed::<SpeechAudioState>();
        (::windows::core::Interface::vtable(self).State)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CurrentSeekPosition(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).CurrentSeekPosition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CurrentDevicePosition(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).CurrentDevicePosition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechAudioStatus, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechAudioStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechAudioStatus {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechAudioStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechAudioStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechAudioStatus {
    type Vtable = ISpeechAudioStatus_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechAudioStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechAudioStatus {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc62d9c91_7458_47f6_862d_1ef86fb0b278);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechAudioStatus_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub FreeBufferSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, freebufferspace: *mut i32) -> ::windows::core::HRESULT,
    pub NonBlockingIO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nonblockingio: *mut i32) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: *mut SpeechAudioState) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CurrentSeekPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentseekposition: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CurrentSeekPosition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CurrentDevicePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentdeviceposition: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CurrentDevicePosition: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechBaseStream(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechBaseStream {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Format(&self) -> ::windows::core::Result<ISpeechAudioFormat> {
        let mut result__ = ::windows::core::zeroed::<ISpeechAudioFormat>();
        (::windows::core::Interface::vtable(self).Format)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_Format<P0>(&self, audioformat: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechAudioFormat>,
    {
        (::windows::core::Interface::vtable(self).putref_Format)(::windows::core::Interface::as_raw(self), audioformat.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Read(&self, buffer: *mut super::super::System::Com::VARIANT, numberofbytes: i32, bytesread: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Read)(::windows::core::Interface::as_raw(self), buffer, numberofbytes, bytesread).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Write(&self, buffer: super::super::System::Com::VARIANT) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Write)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(buffer), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Seek(&self, position: super::super::System::Com::VARIANT, origin: SpeechStreamSeekPositionType) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).Seek)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(position), origin, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechBaseStream, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechBaseStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechBaseStream {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechBaseStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechBaseStream").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechBaseStream {
    type Vtable = ISpeechBaseStream_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechBaseStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechBaseStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6450336f_7d49_4ced_8097_49d6dee37294);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechBaseStream_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audioformat: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Format: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audioformat: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Format: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut super::super::System::Com::VARIANT, numberofbytes: i32, bytesread: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Read: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: super::super::System::Com::VARIANT, byteswritten: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Write: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Seek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: super::super::System::Com::VARIANT, origin: SpeechStreamSeekPositionType, newposition: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Seek: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechCustomStream(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechCustomStream {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Format(&self) -> ::windows::core::Result<ISpeechAudioFormat> {
        let mut result__ = ::windows::core::zeroed::<ISpeechAudioFormat>();
        (::windows::core::Interface::vtable(self).base__.Format)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_Format<P0>(&self, audioformat: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechAudioFormat>,
    {
        (::windows::core::Interface::vtable(self).base__.putref_Format)(::windows::core::Interface::as_raw(self), audioformat.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Read(&self, buffer: *mut super::super::System::Com::VARIANT, numberofbytes: i32, bytesread: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Read)(::windows::core::Interface::as_raw(self), buffer, numberofbytes, bytesread).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Write(&self, buffer: super::super::System::Com::VARIANT) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.Write)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(buffer), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Seek(&self, position: super::super::System::Com::VARIANT, origin: SpeechStreamSeekPositionType) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.Seek)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(position), origin, &mut result__).from_abi(result__)
    }
    pub unsafe fn BaseStream(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).BaseStream)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn putref_BaseStream<P0>(&self, punkstream: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).putref_BaseStream)(::windows::core::Interface::as_raw(self), punkstream.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechCustomStream, ::windows::core::IUnknown, super::super::System::Com::IDispatch, ISpeechBaseStream);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechCustomStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechCustomStream {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechCustomStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechCustomStream").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechCustomStream {
    type Vtable = ISpeechCustomStream_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechCustomStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechCustomStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a9e9f4f_104f_4db8_a115_efd7fd0c97ae);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechCustomStream_Vtbl {
    pub base__: ISpeechBaseStream_Vtbl,
    pub BaseStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunkstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub putref_BaseStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechDataKey(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechDataKey {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetBinaryValue<P0>(&self, valuename: P0, value: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetBinaryValue)(::windows::core::Interface::as_raw(self), valuename.into_param().abi(), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetBinaryValue<P0>(&self, valuename: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetBinaryValue)(::windows::core::Interface::as_raw(self), valuename.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStringValue<P0, P1>(&self, valuename: P0, value: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetStringValue)(::windows::core::Interface::as_raw(self), valuename.into_param().abi(), value.into_param().abi()).ok()
    }
    pub unsafe fn GetStringValue<P0>(&self, valuename: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetStringValue)(::windows::core::Interface::as_raw(self), valuename.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLongValue<P0>(&self, valuename: P0, value: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetLongValue)(::windows::core::Interface::as_raw(self), valuename.into_param().abi(), value).ok()
    }
    pub unsafe fn GetLongValue<P0>(&self, valuename: P0) -> ::windows::core::Result<i32>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).GetLongValue)(::windows::core::Interface::as_raw(self), valuename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenKey<P0>(&self, subkeyname: P0) -> ::windows::core::Result<ISpeechDataKey>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISpeechDataKey>();
        (::windows::core::Interface::vtable(self).OpenKey)(::windows::core::Interface::as_raw(self), subkeyname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateKey<P0>(&self, subkeyname: P0) -> ::windows::core::Result<ISpeechDataKey>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISpeechDataKey>();
        (::windows::core::Interface::vtable(self).CreateKey)(::windows::core::Interface::as_raw(self), subkeyname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteKey<P0>(&self, subkeyname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteKey)(::windows::core::Interface::as_raw(self), subkeyname.into_param().abi()).ok()
    }
    pub unsafe fn DeleteValue<P0>(&self, valuename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DeleteValue)(::windows::core::Interface::as_raw(self), valuename.into_param().abi()).ok()
    }
    pub unsafe fn EnumKeys(&self, index: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).EnumKeys)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumValues(&self, index: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).EnumValues)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechDataKey, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechDataKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechDataKey {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechDataKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechDataKey").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechDataKey {
    type Vtable = ISpeechDataKey_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechDataKey {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechDataKey {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce17c09b_4efa_44d5_a4c9_59d9585ab0cd);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechDataKey_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetBinaryValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuename: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetBinaryValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetBinaryValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuename: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetBinaryValue: usize,
    pub SetStringValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuename: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetStringValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuename: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetLongValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuename: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: i32) -> ::windows::core::HRESULT,
    pub GetLongValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuename: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OpenKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subkeyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, subkey: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OpenKey: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subkeyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, subkey: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateKey: usize,
    pub DeleteKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subkeyname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub DeleteValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub EnumKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, subkeyname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub EnumValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, valuename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechFileStream(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechFileStream {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Format(&self) -> ::windows::core::Result<ISpeechAudioFormat> {
        let mut result__ = ::windows::core::zeroed::<ISpeechAudioFormat>();
        (::windows::core::Interface::vtable(self).base__.Format)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_Format<P0>(&self, audioformat: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechAudioFormat>,
    {
        (::windows::core::Interface::vtable(self).base__.putref_Format)(::windows::core::Interface::as_raw(self), audioformat.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Read(&self, buffer: *mut super::super::System::Com::VARIANT, numberofbytes: i32, bytesread: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Read)(::windows::core::Interface::as_raw(self), buffer, numberofbytes, bytesread).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Write(&self, buffer: super::super::System::Com::VARIANT) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.Write)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(buffer), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Seek(&self, position: super::super::System::Com::VARIANT, origin: SpeechStreamSeekPositionType) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.Seek)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(position), origin, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Open<P0, P1>(&self, filename: P0, filemode: SpeechStreamFileMode, doevents: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).Open)(::windows::core::Interface::as_raw(self), filename.into_param().abi(), filemode, doevents.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechFileStream, ::windows::core::IUnknown, super::super::System::Com::IDispatch, ISpeechBaseStream);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechFileStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechFileStream {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechFileStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechFileStream").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechFileStream {
    type Vtable = ISpeechFileStream_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechFileStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechFileStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf67f125_ab39_4e93_b4a2_cc2e66e182a7);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechFileStream_Vtbl {
    pub base__: ISpeechBaseStream_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Open: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows::core::BSTR>, filemode: SpeechStreamFileMode, doevents: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Open: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechGrammarRule(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechGrammarRule {
    pub unsafe fn Attributes(&self) -> ::windows::core::Result<SpeechRuleAttributes> {
        let mut result__ = ::windows::core::zeroed::<SpeechRuleAttributes>();
        (::windows::core::Interface::vtable(self).Attributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitialState(&self) -> ::windows::core::Result<ISpeechGrammarRuleState> {
        let mut result__ = ::windows::core::zeroed::<ISpeechGrammarRuleState>();
        (::windows::core::Interface::vtable(self).InitialState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Id)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn AddResource<P0, P1>(&self, resourcename: P0, resourcevalue: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddResource)(::windows::core::Interface::as_raw(self), resourcename.into_param().abi(), resourcevalue.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddState(&self) -> ::windows::core::Result<ISpeechGrammarRuleState> {
        let mut result__ = ::windows::core::zeroed::<ISpeechGrammarRuleState>();
        (::windows::core::Interface::vtable(self).AddState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechGrammarRule, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechGrammarRule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechGrammarRule {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechGrammarRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechGrammarRule").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechGrammarRule {
    type Vtable = ISpeechGrammarRule_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechGrammarRule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechGrammarRule {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xafe719cf_5dd1_44f2_999c_7a399f1cfccc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechGrammarRule_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Attributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: *mut SpeechRuleAttributes) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InitialState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitialState: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcename: ::std::mem::MaybeUninit<::windows::core::BSTR>, resourcevalue: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddState: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechGrammarRuleState(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechGrammarRuleState {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Rule(&self) -> ::windows::core::Result<ISpeechGrammarRule> {
        let mut result__ = ::windows::core::zeroed::<ISpeechGrammarRule>();
        (::windows::core::Interface::vtable(self).Rule)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Transitions(&self) -> ::windows::core::Result<ISpeechGrammarRuleStateTransitions> {
        let mut result__ = ::windows::core::zeroed::<ISpeechGrammarRuleStateTransitions>();
        (::windows::core::Interface::vtable(self).Transitions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddWordTransition<P0, P1, P2, P3>(&self, deststate: P0, words: P1, separators: P2, r#type: SpeechGrammarWordType, propertyname: P3, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechGrammarRuleState>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddWordTransition)(::windows::core::Interface::as_raw(self), deststate.into_param().abi(), words.into_param().abi(), separators.into_param().abi(), r#type, propertyname.into_param().abi(), propertyid, propertyvalue, weight).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddRuleTransition<P0, P1, P2>(&self, destinationstate: P0, rule: P1, propertyname: P2, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechGrammarRuleState>,
        P1: ::windows::core::IntoParam<ISpeechGrammarRule>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddRuleTransition)(::windows::core::Interface::as_raw(self), destinationstate.into_param().abi(), rule.into_param().abi(), propertyname.into_param().abi(), propertyid, propertyvalue, weight).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddSpecialTransition<P0, P1>(&self, destinationstate: P0, r#type: SpeechSpecialTransitionType, propertyname: P1, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechGrammarRuleState>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddSpecialTransition)(::windows::core::Interface::as_raw(self), destinationstate.into_param().abi(), r#type, propertyname.into_param().abi(), propertyid, propertyvalue, weight).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechGrammarRuleState, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechGrammarRuleState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechGrammarRuleState {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechGrammarRuleState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechGrammarRuleState").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechGrammarRuleState {
    type Vtable = ISpeechGrammarRuleState_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechGrammarRuleState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechGrammarRuleState {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4286f2c_ee67_45ae_b928_28d695362eda);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechGrammarRuleState_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Rule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rule: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Rule: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Transitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Transitions: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddWordTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deststate: *mut ::core::ffi::c_void, words: ::std::mem::MaybeUninit<::windows::core::BSTR>, separators: ::std::mem::MaybeUninit<::windows::core::BSTR>, r#type: SpeechGrammarWordType, propertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddWordTransition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddRuleTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationstate: *mut ::core::ffi::c_void, rule: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddRuleTransition: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddSpecialTransition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationstate: *mut ::core::ffi::c_void, r#type: SpeechSpecialTransitionType, propertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, propertyid: i32, propertyvalue: *const super::super::System::Com::VARIANT, weight: f32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddSpecialTransition: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechGrammarRuleStateTransition(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechGrammarRuleStateTransition {
    pub unsafe fn Type(&self) -> ::windows::core::Result<SpeechGrammarRuleStateTransitionType> {
        let mut result__ = ::windows::core::zeroed::<SpeechGrammarRuleStateTransitionType>();
        (::windows::core::Interface::vtable(self).Type)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Text(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Text)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Rule(&self) -> ::windows::core::Result<ISpeechGrammarRule> {
        let mut result__ = ::windows::core::zeroed::<ISpeechGrammarRule>();
        (::windows::core::Interface::vtable(self).Rule)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Weight(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).Weight)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PropertyName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).PropertyName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PropertyId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).PropertyId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PropertyValue(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).PropertyValue)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NextState(&self) -> ::windows::core::Result<ISpeechGrammarRuleState> {
        let mut result__ = ::windows::core::zeroed::<ISpeechGrammarRuleState>();
        (::windows::core::Interface::vtable(self).NextState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechGrammarRuleStateTransition, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechGrammarRuleStateTransition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechGrammarRuleStateTransition {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechGrammarRuleStateTransition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechGrammarRuleStateTransition").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechGrammarRuleStateTransition {
    type Vtable = ISpeechGrammarRuleStateTransition_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechGrammarRuleStateTransition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechGrammarRuleStateTransition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcafd1db1_41d1_4a06_9863_e2e81da17a9a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechGrammarRuleStateTransition_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut SpeechGrammarRuleStateTransitionType) -> ::windows::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Rule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rule: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Rule: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Weight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, weight: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Weight: usize,
    pub PropertyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub PropertyId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PropertyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PropertyValue: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub NextState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nextstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    NextState: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechGrammarRuleStateTransitions(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechGrammarRuleStateTransitions {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<ISpeechGrammarRuleStateTransition> {
        let mut result__ = ::windows::core::zeroed::<ISpeechGrammarRuleStateTransition>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechGrammarRuleStateTransitions, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechGrammarRuleStateTransitions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechGrammarRuleStateTransitions {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechGrammarRuleStateTransitions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechGrammarRuleStateTransitions").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechGrammarRuleStateTransitions {
    type Vtable = ISpeechGrammarRuleStateTransitions_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechGrammarRuleStateTransitions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechGrammarRuleStateTransitions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeabce657_75bc_44a2_aa7f_c56476742963);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechGrammarRuleStateTransitions_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, transition: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechGrammarRules(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechGrammarRules {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn FindRule(&self, rulenameorid: super::super::System::Com::VARIANT) -> ::windows::core::Result<ISpeechGrammarRule> {
        let mut result__ = ::windows::core::zeroed::<ISpeechGrammarRule>();
        (::windows::core::Interface::vtable(self).FindRule)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rulenameorid), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<ISpeechGrammarRule> {
        let mut result__ = ::windows::core::zeroed::<ISpeechGrammarRule>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Dynamic(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).Dynamic)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, rulename: P0, attributes: SpeechRuleAttributes, ruleid: i32) -> ::windows::core::Result<ISpeechGrammarRule>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISpeechGrammarRule>();
        (::windows::core::Interface::vtable(self).Add)(::windows::core::Interface::as_raw(self), rulename.into_param().abi(), attributes, ruleid, &mut result__).from_abi(result__)
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CommitAndSave(&self, errortext: *mut ::windows::core::BSTR, savestream: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CommitAndSave)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(errortext), savestream).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechGrammarRules, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechGrammarRules {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechGrammarRules {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechGrammarRules {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechGrammarRules").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechGrammarRules {
    type Vtable = ISpeechGrammarRules_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechGrammarRules {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechGrammarRules {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ffa3b44_fc2d_40d1_8afc_32911c7f1ad1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechGrammarRules_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub FindRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rulenameorid: super::super::System::Com::VARIANT, rule: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    FindRule: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, rule: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Dynamic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dynamic: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Dynamic: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rulename: ::std::mem::MaybeUninit<::windows::core::BSTR>, attributes: SpeechRuleAttributes, ruleid: i32, rule: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CommitAndSave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errortext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, savestream: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CommitAndSave: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechLexicon(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechLexicon {
    pub unsafe fn GenerationId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).GenerationId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWords(&self, flags: SpeechLexiconType, generationid: *mut i32, words: *mut ::core::option::Option<ISpeechLexiconWords>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetWords)(::windows::core::Interface::as_raw(self), flags, generationid, ::core::mem::transmute(words)).ok()
    }
    pub unsafe fn AddPronunciation<P0, P1>(&self, bstrword: P0, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddPronunciation)(::windows::core::Interface::as_raw(self), bstrword.into_param().abi(), langid, partofspeech, bstrpronunciation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddPronunciationByPhoneIds<P0>(&self, bstrword: P0, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).AddPronunciationByPhoneIds)(::windows::core::Interface::as_raw(self), bstrword.into_param().abi(), langid, partofspeech, phoneids).ok()
    }
    pub unsafe fn RemovePronunciation<P0, P1>(&self, bstrword: P0, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).RemovePronunciation)(::windows::core::Interface::as_raw(self), bstrword.into_param().abi(), langid, partofspeech, bstrpronunciation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RemovePronunciationByPhoneIds<P0>(&self, bstrword: P0, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).RemovePronunciationByPhoneIds)(::windows::core::Interface::as_raw(self), bstrword.into_param().abi(), langid, partofspeech, phoneids).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetPronunciations<P0>(&self, bstrword: P0, langid: i32, typeflags: SpeechLexiconType) -> ::windows::core::Result<ISpeechLexiconPronunciations>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISpeechLexiconPronunciations>();
        (::windows::core::Interface::vtable(self).GetPronunciations)(::windows::core::Interface::as_raw(self), bstrword.into_param().abi(), langid, typeflags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetGenerationChange(&self, generationid: *mut i32, ppwords: *mut ::core::option::Option<ISpeechLexiconWords>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGenerationChange)(::windows::core::Interface::as_raw(self), generationid, ::core::mem::transmute(ppwords)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechLexicon, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechLexicon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechLexicon {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechLexicon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechLexicon").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechLexicon {
    type Vtable = ISpeechLexicon_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechLexicon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechLexicon {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3da7627a_c7ae_4b23_8708_638c50362c25);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechLexicon_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub GenerationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, generationid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: SpeechLexiconType, generationid: *mut i32, words: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWords: usize,
    pub AddPronunciation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrword: ::std::mem::MaybeUninit<::windows::core::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AddPronunciationByPhoneIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrword: ::std::mem::MaybeUninit<::windows::core::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AddPronunciationByPhoneIds: usize,
    pub RemovePronunciation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrword: ::std::mem::MaybeUninit<::windows::core::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RemovePronunciationByPhoneIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrword: ::std::mem::MaybeUninit<::windows::core::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RemovePronunciationByPhoneIds: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetPronunciations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrword: ::std::mem::MaybeUninit<::windows::core::BSTR>, langid: i32, typeflags: SpeechLexiconType, pppronunciations: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetPronunciations: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetGenerationChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, generationid: *mut i32, ppwords: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetGenerationChange: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechLexiconPronunciation(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechLexiconPronunciation {
    pub unsafe fn Type(&self) -> ::windows::core::Result<SpeechLexiconType> {
        let mut result__ = ::windows::core::zeroed::<SpeechLexiconType>();
        (::windows::core::Interface::vtable(self).Type)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LangId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).LangId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PartOfSpeech(&self) -> ::windows::core::Result<SpeechPartOfSpeech> {
        let mut result__ = ::windows::core::zeroed::<SpeechPartOfSpeech>();
        (::windows::core::Interface::vtable(self).PartOfSpeech)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PhoneIds(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).PhoneIds)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Symbolic(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Symbolic)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechLexiconPronunciation, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechLexiconPronunciation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechLexiconPronunciation {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechLexiconPronunciation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechLexiconPronunciation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechLexiconPronunciation {
    type Vtable = ISpeechLexiconPronunciation_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechLexiconPronunciation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechLexiconPronunciation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95252c5d_9e43_4f4a_9899_48ee73352f9f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechLexiconPronunciation_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lexicontype: *mut SpeechLexiconType) -> ::windows::core::HRESULT,
    pub LangId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: *mut i32) -> ::windows::core::HRESULT,
    pub PartOfSpeech: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partofspeech: *mut SpeechPartOfSpeech) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PhoneIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phoneids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PhoneIds: usize,
    pub Symbolic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, symbolic: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechLexiconPronunciations(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechLexiconPronunciations {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<ISpeechLexiconPronunciation> {
        let mut result__ = ::windows::core::zeroed::<ISpeechLexiconPronunciation>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechLexiconPronunciations, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechLexiconPronunciations {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechLexiconPronunciations {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechLexiconPronunciations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechLexiconPronunciations").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechLexiconPronunciations {
    type Vtable = ISpeechLexiconPronunciations_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechLexiconPronunciations {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechLexiconPronunciations {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72829128_5682_4704_a0d4_3e2bb6f2ead3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechLexiconPronunciations_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, pronunciation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechLexiconWord(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechLexiconWord {
    pub unsafe fn LangId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).LangId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<SpeechWordType> {
        let mut result__ = ::windows::core::zeroed::<SpeechWordType>();
        (::windows::core::Interface::vtable(self).Type)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Word(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Word)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Pronunciations(&self) -> ::windows::core::Result<ISpeechLexiconPronunciations> {
        let mut result__ = ::windows::core::zeroed::<ISpeechLexiconPronunciations>();
        (::windows::core::Interface::vtable(self).Pronunciations)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechLexiconWord, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechLexiconWord {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechLexiconWord {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechLexiconWord {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechLexiconWord").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechLexiconWord {
    type Vtable = ISpeechLexiconWord_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechLexiconWord {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechLexiconWord {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e5b933c_c9be_48ed_8842_1ee51bb1d4ff);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechLexiconWord_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub LangId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, langid: *mut i32) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wordtype: *mut SpeechWordType) -> ::windows::core::HRESULT,
    pub Word: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, word: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Pronunciations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pronunciations: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Pronunciations: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechLexiconWords(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechLexiconWords {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<ISpeechLexiconWord> {
        let mut result__ = ::windows::core::zeroed::<ISpeechLexiconWord>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechLexiconWords, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechLexiconWords {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechLexiconWords {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechLexiconWords {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechLexiconWords").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechLexiconWords {
    type Vtable = ISpeechLexiconWords_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechLexiconWords {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechLexiconWords {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d199862_415e_47d5_ac4f_faa608b424e6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechLexiconWords_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, word: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechMMSysAudio(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechMMSysAudio {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Format(&self) -> ::windows::core::Result<ISpeechAudioFormat> {
        let mut result__ = ::windows::core::zeroed::<ISpeechAudioFormat>();
        (::windows::core::Interface::vtable(self).base__.base__.Format)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_Format<P0>(&self, audioformat: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechAudioFormat>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.putref_Format)(::windows::core::Interface::as_raw(self), audioformat.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Read(&self, buffer: *mut super::super::System::Com::VARIANT, numberofbytes: i32, bytesread: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Read)(::windows::core::Interface::as_raw(self), buffer, numberofbytes, bytesread).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Write(&self, buffer: super::super::System::Com::VARIANT) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.base__.Write)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(buffer), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Seek(&self, position: super::super::System::Com::VARIANT, origin: SpeechStreamSeekPositionType) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.base__.Seek)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(position), origin, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Status(&self) -> ::windows::core::Result<ISpeechAudioStatus> {
        let mut result__ = ::windows::core::zeroed::<ISpeechAudioStatus>();
        (::windows::core::Interface::vtable(self).base__.Status)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BufferInfo(&self) -> ::windows::core::Result<ISpeechAudioBufferInfo> {
        let mut result__ = ::windows::core::zeroed::<ISpeechAudioBufferInfo>();
        (::windows::core::Interface::vtable(self).base__.BufferInfo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DefaultFormat(&self) -> ::windows::core::Result<ISpeechAudioFormat> {
        let mut result__ = ::windows::core::zeroed::<ISpeechAudioFormat>();
        (::windows::core::Interface::vtable(self).base__.DefaultFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Volume(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.Volume)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetVolume(&self, volume: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetVolume)(::windows::core::Interface::as_raw(self), volume).ok()
    }
    pub unsafe fn BufferNotifySize(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.BufferNotifySize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBufferNotifySize(&self, buffernotifysize: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetBufferNotifySize)(::windows::core::Interface::as_raw(self), buffernotifysize).ok()
    }
    pub unsafe fn EventHandle(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.EventHandle)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetState(&self, state: SpeechAudioState) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetState)(::windows::core::Interface::as_raw(self), state).ok()
    }
    pub unsafe fn DeviceId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).DeviceId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDeviceId(&self, deviceid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDeviceId)(::windows::core::Interface::as_raw(self), deviceid).ok()
    }
    pub unsafe fn LineId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).LineId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLineId(&self, lineid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLineId)(::windows::core::Interface::as_raw(self), lineid).ok()
    }
    pub unsafe fn MMHandle(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).MMHandle)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechMMSysAudio, ::windows::core::IUnknown, super::super::System::Com::IDispatch, ISpeechBaseStream, ISpeechAudio);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechMMSysAudio {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechMMSysAudio {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechMMSysAudio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechMMSysAudio").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechMMSysAudio {
    type Vtable = ISpeechMMSysAudio_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechMMSysAudio {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechMMSysAudio {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c76af6d_1fd7_4831_81d1_3b71d5a13c44);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechMMSysAudio_Vtbl {
    pub base__: ISpeechAudio_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut i32) -> ::windows::core::HRESULT,
    pub SetDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: i32) -> ::windows::core::HRESULT,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lineid: *mut i32) -> ::windows::core::HRESULT,
    pub SetLineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lineid: i32) -> ::windows::core::HRESULT,
    pub MMHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechMemoryStream(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechMemoryStream {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Format(&self) -> ::windows::core::Result<ISpeechAudioFormat> {
        let mut result__ = ::windows::core::zeroed::<ISpeechAudioFormat>();
        (::windows::core::Interface::vtable(self).base__.Format)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_Format<P0>(&self, audioformat: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechAudioFormat>,
    {
        (::windows::core::Interface::vtable(self).base__.putref_Format)(::windows::core::Interface::as_raw(self), audioformat.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Read(&self, buffer: *mut super::super::System::Com::VARIANT, numberofbytes: i32, bytesread: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Read)(::windows::core::Interface::as_raw(self), buffer, numberofbytes, bytesread).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Write(&self, buffer: super::super::System::Com::VARIANT) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.Write)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(buffer), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Seek(&self, position: super::super::System::Com::VARIANT, origin: SpeechStreamSeekPositionType) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.Seek)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(position), origin, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetData(&self, data: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(data)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetData(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GetData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechMemoryStream, ::windows::core::IUnknown, super::super::System::Com::IDispatch, ISpeechBaseStream);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechMemoryStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechMemoryStream {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechMemoryStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechMemoryStream").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechMemoryStream {
    type Vtable = ISpeechMemoryStream_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechMemoryStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechMemoryStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeeb14b68_808b_4abe_a5ea_b51da7588008);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechMemoryStream_Vtbl {
    pub base__: ISpeechBaseStream_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetData: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechObjectToken(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechObjectToken {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Id)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DataKey(&self) -> ::windows::core::Result<ISpeechDataKey> {
        let mut result__ = ::windows::core::zeroed::<ISpeechDataKey>();
        (::windows::core::Interface::vtable(self).DataKey)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Category(&self) -> ::windows::core::Result<ISpeechObjectTokenCategory> {
        let mut result__ = ::windows::core::zeroed::<ISpeechObjectTokenCategory>();
        (::windows::core::Interface::vtable(self).Category)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDescription(&self, locale: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetDescription)(::windows::core::Interface::as_raw(self), locale, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<P0, P1, P2>(&self, id: P0, categoryid: P1, createifnotexist: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetId)(::windows::core::Interface::as_raw(self), id.into_param().abi(), categoryid.into_param().abi(), createifnotexist.into_param().abi()).ok()
    }
    pub unsafe fn GetAttribute<P0>(&self, attributename: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetAttribute)(::windows::core::Interface::as_raw(self), attributename.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateInstance<P0>(&self, punkouter: P0, clscontext: SpeechTokenContext) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).CreateInstance)(::windows::core::Interface::as_raw(self), punkouter.into_param().abi(), clscontext, &mut result__).from_abi(result__)
    }
    pub unsafe fn Remove<P0>(&self, objectstorageclsid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self), objectstorageclsid.into_param().abi()).ok()
    }
    pub unsafe fn GetStorageFileName<P0, P1, P2>(&self, objectstorageclsid: P0, keyname: P1, filename: P2, folder: SpeechTokenShellFolder) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetStorageFileName)(::windows::core::Interface::as_raw(self), objectstorageclsid.into_param().abi(), keyname.into_param().abi(), filename.into_param().abi(), folder, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveStorageFileName<P0, P1, P2>(&self, objectstorageclsid: P0, keyname: P1, deletefile: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).RemoveStorageFileName)(::windows::core::Interface::as_raw(self), objectstorageclsid.into_param().abi(), keyname.into_param().abi(), deletefile.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn IsUISupported<P0, P1>(&self, typeofui: P0, extradata: *const super::super::System::Com::VARIANT, object: P1) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsUISupported)(::windows::core::Interface::as_raw(self), typeofui.into_param().abi(), extradata, object.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DisplayUI<P0, P1, P2>(&self, hwnd: i32, title: P0, typeofui: P1, extradata: *const super::super::System::Com::VARIANT, object: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).DisplayUI)(::windows::core::Interface::as_raw(self), hwnd, title.into_param().abi(), typeofui.into_param().abi(), extradata, object.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MatchesAttributes<P0>(&self, attributes: P0) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).MatchesAttributes)(::windows::core::Interface::as_raw(self), attributes.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechObjectToken, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechObjectToken {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechObjectToken {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechObjectToken {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechObjectToken").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechObjectToken {
    type Vtable = ISpeechObjectToken_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechObjectToken {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechObjectToken {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc74a3adc_b727_4500_a84a_b526721c8b8c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechObjectToken_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub DataKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datakey: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DataKey: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Category: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Category: usize,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, locale: i32, description: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows::core::BSTR>, categoryid: ::std::mem::MaybeUninit<::windows::core::BSTR>, createifnotexist: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetId: usize,
    pub GetAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributename: ::std::mem::MaybeUninit<::windows::core::BSTR>, attributevalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, clscontext: SpeechTokenContext, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectstorageclsid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub GetStorageFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectstorageclsid: ::std::mem::MaybeUninit<::windows::core::BSTR>, keyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, filename: ::std::mem::MaybeUninit<::windows::core::BSTR>, folder: SpeechTokenShellFolder, filepath: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveStorageFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectstorageclsid: ::std::mem::MaybeUninit<::windows::core::BSTR>, keyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, deletefile: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveStorageFileName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub IsUISupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typeofui: ::std::mem::MaybeUninit<::windows::core::BSTR>, extradata: *const super::super::System::Com::VARIANT, object: *mut ::core::ffi::c_void, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    IsUISupported: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DisplayUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: i32, title: ::std::mem::MaybeUninit<::windows::core::BSTR>, typeofui: ::std::mem::MaybeUninit<::windows::core::BSTR>, extradata: *const super::super::System::Com::VARIANT, object: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DisplayUI: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MatchesAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: ::std::mem::MaybeUninit<::windows::core::BSTR>, matches: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MatchesAttributes: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechObjectTokenCategory(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechObjectTokenCategory {
    pub unsafe fn Id(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Id)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDefault<P0>(&self, tokenid: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDefault)(::windows::core::Interface::as_raw(self), tokenid.into_param().abi()).ok()
    }
    pub unsafe fn Default(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Default)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetId<P0, P1>(&self, id: P0, createifnotexist: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetId)(::windows::core::Interface::as_raw(self), id.into_param().abi(), createifnotexist.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDataKey(&self, location: SpeechDataKeyLocation) -> ::windows::core::Result<ISpeechDataKey> {
        let mut result__ = ::windows::core::zeroed::<ISpeechDataKey>();
        (::windows::core::Interface::vtable(self).GetDataKey)(::windows::core::Interface::as_raw(self), location, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateTokens<P0, P1>(&self, requiredattributes: P0, optionalattributes: P1) -> ::windows::core::Result<ISpeechObjectTokens>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISpeechObjectTokens>();
        (::windows::core::Interface::vtable(self).EnumerateTokens)(::windows::core::Interface::as_raw(self), requiredattributes.into_param().abi(), optionalattributes.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechObjectTokenCategory, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechObjectTokenCategory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechObjectTokenCategory {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechObjectTokenCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechObjectTokenCategory").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechObjectTokenCategory {
    type Vtable = ISpeechObjectTokenCategory_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechObjectTokenCategory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechObjectTokenCategory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca7eac50_2d01_4145_86d4_5ae7d70f4469);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechObjectTokenCategory_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tokenid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Default: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tokenid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows::core::BSTR>, createifnotexist: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetId: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDataKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: SpeechDataKeyLocation, datakey: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDataKey: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumerateTokens: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requiredattributes: ::std::mem::MaybeUninit<::windows::core::BSTR>, optionalattributes: ::std::mem::MaybeUninit<::windows::core::BSTR>, tokens: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumerateTokens: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechObjectTokens(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechObjectTokens {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<ISpeechObjectToken> {
        let mut result__ = ::windows::core::zeroed::<ISpeechObjectToken>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechObjectTokens, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechObjectTokens {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechObjectTokens {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechObjectTokens {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechObjectTokens").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechObjectTokens {
    type Vtable = ISpeechObjectTokens_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechObjectTokens {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechObjectTokens {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9285b776_2e7b_4bc0_b53e_580eb6fa967f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechObjectTokens_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, token: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechPhoneConverter(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhoneConverter {
    pub unsafe fn LanguageId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).LanguageId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLanguageId(&self, languageid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLanguageId)(::windows::core::Interface::as_raw(self), languageid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PhoneToId<P0>(&self, phonemes: P0) -> ::windows::core::Result<super::super::System::Com::VARIANT>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).PhoneToId)(::windows::core::Interface::as_raw(self), phonemes.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn IdToPhone(&self, idarray: super::super::System::Com::VARIANT) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).IdToPhone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(idarray), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechPhoneConverter, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechPhoneConverter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechPhoneConverter {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechPhoneConverter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechPhoneConverter").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechPhoneConverter {
    type Vtable = ISpeechPhoneConverter_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechPhoneConverter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechPhoneConverter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3e4f353_433f_43d6_89a1_6a62a7054c3d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechPhoneConverter_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub LanguageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languageid: *mut i32) -> ::windows::core::HRESULT,
    pub SetLanguageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languageid: i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PhoneToId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonemes: ::std::mem::MaybeUninit<::windows::core::BSTR>, idarray: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PhoneToId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub IdToPhone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idarray: super::super::System::Com::VARIANT, phonemes: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    IdToPhone: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechPhraseAlternate(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseAlternate {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RecoResult(&self) -> ::windows::core::Result<ISpeechRecoResult> {
        let mut result__ = ::windows::core::zeroed::<ISpeechRecoResult>();
        (::windows::core::Interface::vtable(self).RecoResult)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StartElementInResult(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).StartElementInResult)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NumberOfElementsInResult(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).NumberOfElementsInResult)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PhraseInfo(&self) -> ::windows::core::Result<ISpeechPhraseInfo> {
        let mut result__ = ::windows::core::zeroed::<ISpeechPhraseInfo>();
        (::windows::core::Interface::vtable(self).PhraseInfo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechPhraseAlternate, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechPhraseAlternate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechPhraseAlternate {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechPhraseAlternate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechPhraseAlternate").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechPhraseAlternate {
    type Vtable = ISpeechPhraseAlternate_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechPhraseAlternate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechPhraseAlternate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27864a2a_2b9f_4cb8_92d3_0d2722fd1e73);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechPhraseAlternate_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RecoResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recoresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RecoResult: usize,
    pub StartElementInResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startelement: *mut i32) -> ::windows::core::HRESULT,
    pub NumberOfElementsInResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub PhraseInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phraseinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PhraseInfo: usize,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechPhraseAlternates(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseAlternates {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<ISpeechPhraseAlternate> {
        let mut result__ = ::windows::core::zeroed::<ISpeechPhraseAlternate>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechPhraseAlternates, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechPhraseAlternates {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechPhraseAlternates {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechPhraseAlternates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechPhraseAlternates").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechPhraseAlternates {
    type Vtable = ISpeechPhraseAlternates_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechPhraseAlternates {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechPhraseAlternates {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb238b6d5_f276_4c3d_a6c1_2974801c3cc2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechPhraseAlternates_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, phrasealternate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechPhraseElement(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseElement {
    pub unsafe fn AudioTimeOffset(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).AudioTimeOffset)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AudioSizeTime(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).AudioSizeTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AudioStreamOffset(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).AudioStreamOffset)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AudioSizeBytes(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).AudioSizeBytes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RetainedStreamOffset(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).RetainedStreamOffset)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RetainedSizeBytes(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).RetainedSizeBytes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DisplayText(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).DisplayText)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LexicalForm(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).LexicalForm)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Pronunciation(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).Pronunciation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DisplayAttributes(&self) -> ::windows::core::Result<SpeechDisplayAttributes> {
        let mut result__ = ::windows::core::zeroed::<SpeechDisplayAttributes>();
        (::windows::core::Interface::vtable(self).DisplayAttributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RequiredConfidence(&self) -> ::windows::core::Result<SpeechEngineConfidence> {
        let mut result__ = ::windows::core::zeroed::<SpeechEngineConfidence>();
        (::windows::core::Interface::vtable(self).RequiredConfidence)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ActualConfidence(&self) -> ::windows::core::Result<SpeechEngineConfidence> {
        let mut result__ = ::windows::core::zeroed::<SpeechEngineConfidence>();
        (::windows::core::Interface::vtable(self).ActualConfidence)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EngineConfidence(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).EngineConfidence)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechPhraseElement, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechPhraseElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechPhraseElement {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechPhraseElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechPhraseElement").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechPhraseElement {
    type Vtable = ISpeechPhraseElement_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechPhraseElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechPhraseElement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6176f96_e373_4801_b223_3b62c068c0b4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechPhraseElement_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub AudioTimeOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiotimeoffset: *mut i32) -> ::windows::core::HRESULT,
    pub AudioSizeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiosizetime: *mut i32) -> ::windows::core::HRESULT,
    pub AudioStreamOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiostreamoffset: *mut i32) -> ::windows::core::HRESULT,
    pub AudioSizeBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiosizebytes: *mut i32) -> ::windows::core::HRESULT,
    pub RetainedStreamOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retainedstreamoffset: *mut i32) -> ::windows::core::HRESULT,
    pub RetainedSizeBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retainedsizebytes: *mut i32) -> ::windows::core::HRESULT,
    pub DisplayText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displaytext: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub LexicalForm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lexicalform: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Pronunciation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pronunciation: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Pronunciation: usize,
    pub DisplayAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayattributes: *mut SpeechDisplayAttributes) -> ::windows::core::HRESULT,
    pub RequiredConfidence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requiredconfidence: *mut SpeechEngineConfidence) -> ::windows::core::HRESULT,
    pub ActualConfidence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actualconfidence: *mut SpeechEngineConfidence) -> ::windows::core::HRESULT,
    pub EngineConfidence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, engineconfidence: *mut f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechPhraseElements(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseElements {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<ISpeechPhraseElement> {
        let mut result__ = ::windows::core::zeroed::<ISpeechPhraseElement>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechPhraseElements, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechPhraseElements {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechPhraseElements {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechPhraseElements {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechPhraseElements").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechPhraseElements {
    type Vtable = ISpeechPhraseElements_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechPhraseElements {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechPhraseElements {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0626b328_3478_467d_a0b3_d0853b93dda3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechPhraseElements_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, element: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechPhraseInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseInfo {
    pub unsafe fn LanguageId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).LanguageId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GrammarId(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).GrammarId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn StartTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).StartTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AudioStreamPosition(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).AudioStreamPosition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AudioSizeBytes(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).AudioSizeBytes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RetainedSizeBytes(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).RetainedSizeBytes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AudioSizeTime(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).AudioSizeTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Rule(&self) -> ::windows::core::Result<ISpeechPhraseRule> {
        let mut result__ = ::windows::core::zeroed::<ISpeechPhraseRule>();
        (::windows::core::Interface::vtable(self).Rule)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<ISpeechPhraseProperties> {
        let mut result__ = ::windows::core::zeroed::<ISpeechPhraseProperties>();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Elements(&self) -> ::windows::core::Result<ISpeechPhraseElements> {
        let mut result__ = ::windows::core::zeroed::<ISpeechPhraseElements>();
        (::windows::core::Interface::vtable(self).Elements)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Replacements(&self) -> ::windows::core::Result<ISpeechPhraseReplacements> {
        let mut result__ = ::windows::core::zeroed::<ISpeechPhraseReplacements>();
        (::windows::core::Interface::vtable(self).Replacements)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EngineId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).EngineId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EnginePrivateData(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).EnginePrivateData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SaveToMemory(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).SaveToMemory)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetText<P0>(&self, startelement: i32, elements: i32, usereplacements: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetText)(::windows::core::Interface::as_raw(self), startelement, elements, usereplacements.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayAttributes<P0>(&self, startelement: i32, elements: i32, usereplacements: P0) -> ::windows::core::Result<SpeechDisplayAttributes>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::windows::core::zeroed::<SpeechDisplayAttributes>();
        (::windows::core::Interface::vtable(self).GetDisplayAttributes)(::windows::core::Interface::as_raw(self), startelement, elements, usereplacements.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechPhraseInfo, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechPhraseInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechPhraseInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechPhraseInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechPhraseInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechPhraseInfo {
    type Vtable = ISpeechPhraseInfo_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechPhraseInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechPhraseInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x961559cf_4e67_4662_8bf0_d93f1fcd61b3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechPhraseInfo_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub LanguageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languageid: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GrammarId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grammarid: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GrammarId: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    StartTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub AudioStreamPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiostreamposition: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    AudioStreamPosition: usize,
    pub AudioSizeBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paudiosizebytes: *mut i32) -> ::windows::core::HRESULT,
    pub RetainedSizeBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retainedsizebytes: *mut i32) -> ::windows::core::HRESULT,
    pub AudioSizeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiosizetime: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Rule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rule: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Rule: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, properties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Elements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, elements: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Elements: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Replacements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, replacements: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Replacements: usize,
    pub EngineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, engineidguid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EnginePrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, privatedata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EnginePrivateData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SaveToMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phraseblock: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SaveToMemory: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, usereplacements: super::super::Foundation::VARIANT_BOOL, text: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetText: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDisplayAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, usereplacements: super::super::Foundation::VARIANT_BOOL, displayattributes: *mut SpeechDisplayAttributes) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDisplayAttributes: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechPhraseInfoBuilder(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseInfoBuilder {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RestorePhraseFromMemory(&self, phraseinmemory: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<ISpeechPhraseInfo> {
        let mut result__ = ::windows::core::zeroed::<ISpeechPhraseInfo>();
        (::windows::core::Interface::vtable(self).RestorePhraseFromMemory)(::windows::core::Interface::as_raw(self), phraseinmemory, &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechPhraseInfoBuilder, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechPhraseInfoBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechPhraseInfoBuilder {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechPhraseInfoBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechPhraseInfoBuilder").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechPhraseInfoBuilder {
    type Vtable = ISpeechPhraseInfoBuilder_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechPhraseInfoBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechPhraseInfoBuilder {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b151836_df3a_4e0a_846c_d2adc9334333);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechPhraseInfoBuilder_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RestorePhraseFromMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phraseinmemory: *const super::super::System::Com::VARIANT, phraseinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RestorePhraseFromMemory: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechPhraseProperties(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseProperties {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<ISpeechPhraseProperty> {
        let mut result__ = ::windows::core::zeroed::<ISpeechPhraseProperty>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechPhraseProperties, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechPhraseProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechPhraseProperties {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechPhraseProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechPhraseProperties").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechPhraseProperties {
    type Vtable = ISpeechPhraseProperties_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechPhraseProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechPhraseProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08166b47_102e_4b23_a599_bdb98dbfd1f4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechPhraseProperties_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, property: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechPhraseProperty(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseProperty {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Id)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).Value)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FirstElement(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).FirstElement)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NumberOfElements(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).NumberOfElements)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EngineConfidence(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).EngineConfidence)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Confidence(&self) -> ::windows::core::Result<SpeechEngineConfidence> {
        let mut result__ = ::windows::core::zeroed::<SpeechEngineConfidence>();
        (::windows::core::Interface::vtable(self).Confidence)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<ISpeechPhraseProperty> {
        let mut result__ = ::windows::core::zeroed::<ISpeechPhraseProperty>();
        (::windows::core::Interface::vtable(self).Parent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Children(&self) -> ::windows::core::Result<ISpeechPhraseProperties> {
        let mut result__ = ::windows::core::zeroed::<ISpeechPhraseProperties>();
        (::windows::core::Interface::vtable(self).Children)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechPhraseProperty, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechPhraseProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechPhraseProperty {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechPhraseProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechPhraseProperty").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechPhraseProperty {
    type Vtable = ISpeechPhraseProperty_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechPhraseProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechPhraseProperty {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce563d48_961e_4732_a2e1_378a42b430be);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechPhraseProperty_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    pub FirstElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, firstelement: *mut i32) -> ::windows::core::HRESULT,
    pub NumberOfElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows::core::HRESULT,
    pub EngineConfidence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, confidence: *mut f32) -> ::windows::core::HRESULT,
    pub Confidence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, confidence: *mut SpeechEngineConfidence) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentproperty: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, children: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Children: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechPhraseReplacement(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseReplacement {
    pub unsafe fn DisplayAttributes(&self) -> ::windows::core::Result<SpeechDisplayAttributes> {
        let mut result__ = ::windows::core::zeroed::<SpeechDisplayAttributes>();
        (::windows::core::Interface::vtable(self).DisplayAttributes)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Text(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Text)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FirstElement(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).FirstElement)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NumberOfElements(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).NumberOfElements)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechPhraseReplacement, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechPhraseReplacement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechPhraseReplacement {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechPhraseReplacement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechPhraseReplacement").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechPhraseReplacement {
    type Vtable = ISpeechPhraseReplacement_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechPhraseReplacement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechPhraseReplacement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2890a410_53a7_4fb5_94ec_06d4998e3d02);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechPhraseReplacement_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub DisplayAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayattributes: *mut SpeechDisplayAttributes) -> ::windows::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub FirstElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, firstelement: *mut i32) -> ::windows::core::HRESULT,
    pub NumberOfElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechPhraseReplacements(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseReplacements {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<ISpeechPhraseReplacement> {
        let mut result__ = ::windows::core::zeroed::<ISpeechPhraseReplacement>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechPhraseReplacements, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechPhraseReplacements {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechPhraseReplacements {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechPhraseReplacements {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechPhraseReplacements").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechPhraseReplacements {
    type Vtable = ISpeechPhraseReplacements_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechPhraseReplacements {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechPhraseReplacements {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38bc662f_2257_4525_959e_2069d2596c05);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechPhraseReplacements_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, reps: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechPhraseRule(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseRule {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Id(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Id)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn FirstElement(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).FirstElement)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NumberOfElements(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).NumberOfElements)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parent(&self) -> ::windows::core::Result<ISpeechPhraseRule> {
        let mut result__ = ::windows::core::zeroed::<ISpeechPhraseRule>();
        (::windows::core::Interface::vtable(self).Parent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Children(&self) -> ::windows::core::Result<ISpeechPhraseRules> {
        let mut result__ = ::windows::core::zeroed::<ISpeechPhraseRules>();
        (::windows::core::Interface::vtable(self).Children)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Confidence(&self) -> ::windows::core::Result<SpeechEngineConfidence> {
        let mut result__ = ::windows::core::zeroed::<SpeechEngineConfidence>();
        (::windows::core::Interface::vtable(self).Confidence)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EngineConfidence(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::windows::core::zeroed::<f32>();
        (::windows::core::Interface::vtable(self).EngineConfidence)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechPhraseRule, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechPhraseRule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechPhraseRule {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechPhraseRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechPhraseRule").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechPhraseRule {
    type Vtable = ISpeechPhraseRule_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechPhraseRule {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechPhraseRule {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7bfe112_a4a0_48d9_b602_c313843f6964);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechPhraseRule_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT,
    pub FirstElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, firstelement: *mut i32) -> ::windows::core::HRESULT,
    pub NumberOfElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Parent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, children: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Children: usize,
    pub Confidence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, actualconfidence: *mut SpeechEngineConfidence) -> ::windows::core::HRESULT,
    pub EngineConfidence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, engineconfidence: *mut f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechPhraseRules(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechPhraseRules {
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<ISpeechPhraseRule> {
        let mut result__ = ::windows::core::zeroed::<ISpeechPhraseRule>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechPhraseRules, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechPhraseRules {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechPhraseRules {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechPhraseRules {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechPhraseRules").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechPhraseRules {
    type Vtable = ISpeechPhraseRules_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechPhraseRules {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechPhraseRules {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9047d593_01dd_4b72_81a3_e4a0ca69f407);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechPhraseRules_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, rule: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechRecoContext(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecoContext {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recognizer(&self) -> ::windows::core::Result<ISpeechRecognizer> {
        let mut result__ = ::windows::core::zeroed::<ISpeechRecognizer>();
        (::windows::core::Interface::vtable(self).Recognizer)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AudioInputInterferenceStatus(&self) -> ::windows::core::Result<SpeechInterference> {
        let mut result__ = ::windows::core::zeroed::<SpeechInterference>();
        (::windows::core::Interface::vtable(self).AudioInputInterferenceStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RequestedUIType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).RequestedUIType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_Voice<P0>(&self, voice: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechVoice>,
    {
        (::windows::core::Interface::vtable(self).putref_Voice)(::windows::core::Interface::as_raw(self), voice.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Voice(&self) -> ::windows::core::Result<ISpeechVoice> {
        let mut result__ = ::windows::core::zeroed::<ISpeechVoice>();
        (::windows::core::Interface::vtable(self).Voice)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowVoiceFormatMatchingOnNextSet<P0>(&self, allow: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetAllowVoiceFormatMatchingOnNextSet)(::windows::core::Interface::as_raw(self), allow.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowVoiceFormatMatchingOnNextSet(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).AllowVoiceFormatMatchingOnNextSet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetVoicePurgeEvent(&self, eventinterest: SpeechRecoEvents) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVoicePurgeEvent)(::windows::core::Interface::as_raw(self), eventinterest).ok()
    }
    pub unsafe fn VoicePurgeEvent(&self) -> ::windows::core::Result<SpeechRecoEvents> {
        let mut result__ = ::windows::core::zeroed::<SpeechRecoEvents>();
        (::windows::core::Interface::vtable(self).VoicePurgeEvent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEventInterests(&self, eventinterest: SpeechRecoEvents) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventInterests)(::windows::core::Interface::as_raw(self), eventinterest).ok()
    }
    pub unsafe fn EventInterests(&self) -> ::windows::core::Result<SpeechRecoEvents> {
        let mut result__ = ::windows::core::zeroed::<SpeechRecoEvents>();
        (::windows::core::Interface::vtable(self).EventInterests)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCmdMaxAlternates(&self, maxalternates: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCmdMaxAlternates)(::windows::core::Interface::as_raw(self), maxalternates).ok()
    }
    pub unsafe fn CmdMaxAlternates(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).CmdMaxAlternates)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetState(&self, state: SpeechRecoContextState) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetState)(::windows::core::Interface::as_raw(self), state).ok()
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<SpeechRecoContextState> {
        let mut result__ = ::windows::core::zeroed::<SpeechRecoContextState>();
        (::windows::core::Interface::vtable(self).State)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRetainedAudio(&self, option: SpeechRetainedAudioOptions) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRetainedAudio)(::windows::core::Interface::as_raw(self), option).ok()
    }
    pub unsafe fn RetainedAudio(&self) -> ::windows::core::Result<SpeechRetainedAudioOptions> {
        let mut result__ = ::windows::core::zeroed::<SpeechRetainedAudioOptions>();
        (::windows::core::Interface::vtable(self).RetainedAudio)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_RetainedAudioFormat<P0>(&self, format: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechAudioFormat>,
    {
        (::windows::core::Interface::vtable(self).putref_RetainedAudioFormat)(::windows::core::Interface::as_raw(self), format.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RetainedAudioFormat(&self) -> ::windows::core::Result<ISpeechAudioFormat> {
        let mut result__ = ::windows::core::zeroed::<ISpeechAudioFormat>();
        (::windows::core::Interface::vtable(self).RetainedAudioFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Pause)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resume)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateGrammar(&self, grammarid: super::super::System::Com::VARIANT) -> ::windows::core::Result<ISpeechRecoGrammar> {
        let mut result__ = ::windows::core::zeroed::<ISpeechRecoGrammar>();
        (::windows::core::Interface::vtable(self).CreateGrammar)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(grammarid), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateResultFromMemory(&self, resultblock: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<ISpeechRecoResult> {
        let mut result__ = ::windows::core::zeroed::<ISpeechRecoResult>();
        (::windows::core::Interface::vtable(self).CreateResultFromMemory)(::windows::core::Interface::as_raw(self), resultblock, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Bookmark(&self, options: SpeechBookmarkOptions, streampos: super::super::System::Com::VARIANT, bookmarkid: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Bookmark)(::windows::core::Interface::as_raw(self), options, ::core::mem::transmute(streampos), ::core::mem::transmute(bookmarkid)).ok()
    }
    pub unsafe fn SetAdaptationData<P0>(&self, adaptationstring: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetAdaptationData)(::windows::core::Interface::as_raw(self), adaptationstring.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechRecoContext, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechRecoContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechRecoContext {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechRecoContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechRecoContext").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechRecoContext {
    type Vtable = ISpeechRecoContext_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechRecoContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechRecoContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x580aa49d_7e1e_4809_b8e2_57da806104b8);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecoContext_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Recognizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recognizer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recognizer: usize,
    pub AudioInputInterferenceStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interference: *mut SpeechInterference) -> ::windows::core::HRESULT,
    pub RequestedUIType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uitype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Voice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, voice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Voice: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Voice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, voice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Voice: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowVoiceFormatMatchingOnNextSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowVoiceFormatMatchingOnNextSet: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowVoiceFormatMatchingOnNextSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pallow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowVoiceFormatMatchingOnNextSet: usize,
    pub SetVoicePurgeEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventinterest: SpeechRecoEvents) -> ::windows::core::HRESULT,
    pub VoicePurgeEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventinterest: *mut SpeechRecoEvents) -> ::windows::core::HRESULT,
    pub SetEventInterests: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventinterest: SpeechRecoEvents) -> ::windows::core::HRESULT,
    pub EventInterests: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventinterest: *mut SpeechRecoEvents) -> ::windows::core::HRESULT,
    pub SetCmdMaxAlternates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxalternates: i32) -> ::windows::core::HRESULT,
    pub CmdMaxAlternates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxalternates: *mut i32) -> ::windows::core::HRESULT,
    pub SetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: SpeechRecoContextState) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: *mut SpeechRecoContextState) -> ::windows::core::HRESULT,
    pub SetRetainedAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, option: SpeechRetainedAudioOptions) -> ::windows::core::HRESULT,
    pub RetainedAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, option: *mut SpeechRetainedAudioOptions) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_RetainedAudioFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_RetainedAudioFormat: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RetainedAudioFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RetainedAudioFormat: usize,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateGrammar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grammarid: super::super::System::Com::VARIANT, grammar: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateGrammar: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CreateResultFromMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resultblock: *const super::super::System::Com::VARIANT, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CreateResultFromMemory: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Bookmark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: SpeechBookmarkOptions, streampos: super::super::System::Com::VARIANT, bookmarkid: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Bookmark: usize,
    pub SetAdaptationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adaptationstring: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechRecoGrammar(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecoGrammar {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Id(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).Id)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RecoContext(&self) -> ::windows::core::Result<ISpeechRecoContext> {
        let mut result__ = ::windows::core::zeroed::<ISpeechRecoContext>();
        (::windows::core::Interface::vtable(self).RecoContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetState(&self, state: SpeechGrammarState) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetState)(::windows::core::Interface::as_raw(self), state).ok()
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<SpeechGrammarState> {
        let mut result__ = ::windows::core::zeroed::<SpeechGrammarState>();
        (::windows::core::Interface::vtable(self).State)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Rules(&self) -> ::windows::core::Result<ISpeechGrammarRules> {
        let mut result__ = ::windows::core::zeroed::<ISpeechGrammarRules>();
        (::windows::core::Interface::vtable(self).Rules)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Reset(&self, newlanguage: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self), newlanguage).ok()
    }
    pub unsafe fn CmdLoadFromFile<P0>(&self, filename: P0, loadoption: SpeechLoadOption) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).CmdLoadFromFile)(::windows::core::Interface::as_raw(self), filename.into_param().abi(), loadoption).ok()
    }
    pub unsafe fn CmdLoadFromObject<P0, P1>(&self, classid: P0, grammarname: P1, loadoption: SpeechLoadOption) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).CmdLoadFromObject)(::windows::core::Interface::as_raw(self), classid.into_param().abi(), grammarname.into_param().abi(), loadoption).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CmdLoadFromResource(&self, hmodule: i32, resourcename: super::super::System::Com::VARIANT, resourcetype: super::super::System::Com::VARIANT, languageid: i32, loadoption: SpeechLoadOption) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CmdLoadFromResource)(::windows::core::Interface::as_raw(self), hmodule, ::core::mem::transmute(resourcename), ::core::mem::transmute(resourcetype), languageid, loadoption).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CmdLoadFromMemory(&self, grammardata: super::super::System::Com::VARIANT, loadoption: SpeechLoadOption) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CmdLoadFromMemory)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(grammardata), loadoption).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CmdLoadFromProprietaryGrammar<P0, P1>(&self, proprietaryguid: P0, proprietarystring: P1, proprietarydata: super::super::System::Com::VARIANT, loadoption: SpeechLoadOption) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).CmdLoadFromProprietaryGrammar)(::windows::core::Interface::as_raw(self), proprietaryguid.into_param().abi(), proprietarystring.into_param().abi(), ::core::mem::transmute(proprietarydata), loadoption).ok()
    }
    pub unsafe fn CmdSetRuleState<P0>(&self, name: P0, state: SpeechRuleState) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).CmdSetRuleState)(::windows::core::Interface::as_raw(self), name.into_param().abi(), state).ok()
    }
    pub unsafe fn CmdSetRuleIdState(&self, ruleid: i32, state: SpeechRuleState) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CmdSetRuleIdState)(::windows::core::Interface::as_raw(self), ruleid, state).ok()
    }
    pub unsafe fn DictationLoad<P0>(&self, topicname: P0, loadoption: SpeechLoadOption) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DictationLoad)(::windows::core::Interface::as_raw(self), topicname.into_param().abi(), loadoption).ok()
    }
    pub unsafe fn DictationUnload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DictationUnload)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn DictationSetState(&self, state: SpeechRuleState) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DictationSetState)(::windows::core::Interface::as_raw(self), state).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetWordSequenceData<P0, P1>(&self, text: P0, textlength: i32, info: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<ISpeechTextSelectionInformation>,
    {
        (::windows::core::Interface::vtable(self).SetWordSequenceData)(::windows::core::Interface::as_raw(self), text.into_param().abi(), textlength, info.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetTextSelection<P0>(&self, info: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechTextSelectionInformation>,
    {
        (::windows::core::Interface::vtable(self).SetTextSelection)(::windows::core::Interface::as_raw(self), info.into_param().abi()).ok()
    }
    pub unsafe fn IsPronounceable<P0>(&self, word: P0) -> ::windows::core::Result<SpeechWordPronounceable>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<SpeechWordPronounceable>();
        (::windows::core::Interface::vtable(self).IsPronounceable)(::windows::core::Interface::as_raw(self), word.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechRecoGrammar, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechRecoGrammar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechRecoGrammar {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechRecoGrammar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechRecoGrammar").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechRecoGrammar {
    type Vtable = ISpeechRecoGrammar_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechRecoGrammar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechRecoGrammar {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6d6f79f_2158_4e50_b5bc_9a9ccd852a09);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecoGrammar_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Id: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub RecoContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recocontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RecoContext: usize,
    pub SetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: SpeechGrammarState) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: *mut SpeechGrammarState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Rules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rules: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Rules: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newlanguage: i32) -> ::windows::core::HRESULT,
    pub CmdLoadFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows::core::BSTR>, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT,
    pub CmdLoadFromObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classid: ::std::mem::MaybeUninit<::windows::core::BSTR>, grammarname: ::std::mem::MaybeUninit<::windows::core::BSTR>, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CmdLoadFromResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hmodule: i32, resourcename: super::super::System::Com::VARIANT, resourcetype: super::super::System::Com::VARIANT, languageid: i32, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CmdLoadFromResource: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CmdLoadFromMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, grammardata: super::super::System::Com::VARIANT, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CmdLoadFromMemory: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CmdLoadFromProprietaryGrammar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, proprietaryguid: ::std::mem::MaybeUninit<::windows::core::BSTR>, proprietarystring: ::std::mem::MaybeUninit<::windows::core::BSTR>, proprietarydata: super::super::System::Com::VARIANT, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CmdLoadFromProprietaryGrammar: usize,
    pub CmdSetRuleState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, state: SpeechRuleState) -> ::windows::core::HRESULT,
    pub CmdSetRuleIdState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ruleid: i32, state: SpeechRuleState) -> ::windows::core::HRESULT,
    pub DictationLoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, topicname: ::std::mem::MaybeUninit<::windows::core::BSTR>, loadoption: SpeechLoadOption) -> ::windows::core::HRESULT,
    pub DictationUnload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DictationSetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: SpeechRuleState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetWordSequenceData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows::core::BSTR>, textlength: i32, info: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetWordSequenceData: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetTextSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, info: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetTextSelection: usize,
    pub IsPronounceable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, word: ::std::mem::MaybeUninit<::windows::core::BSTR>, wordpronounceable: *mut SpeechWordPronounceable) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechRecoResult(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecoResult {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RecoContext(&self) -> ::windows::core::Result<ISpeechRecoContext> {
        let mut result__ = ::windows::core::zeroed::<ISpeechRecoContext>();
        (::windows::core::Interface::vtable(self).RecoContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Times(&self) -> ::windows::core::Result<ISpeechRecoResultTimes> {
        let mut result__ = ::windows::core::zeroed::<ISpeechRecoResultTimes>();
        (::windows::core::Interface::vtable(self).Times)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AudioFormat<P0>(&self, format: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechAudioFormat>,
    {
        (::windows::core::Interface::vtable(self).putref_AudioFormat)(::windows::core::Interface::as_raw(self), format.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AudioFormat(&self) -> ::windows::core::Result<ISpeechAudioFormat> {
        let mut result__ = ::windows::core::zeroed::<ISpeechAudioFormat>();
        (::windows::core::Interface::vtable(self).AudioFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PhraseInfo(&self) -> ::windows::core::Result<ISpeechPhraseInfo> {
        let mut result__ = ::windows::core::zeroed::<ISpeechPhraseInfo>();
        (::windows::core::Interface::vtable(self).PhraseInfo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Alternates(&self, requestcount: i32, startelement: i32, elements: i32) -> ::windows::core::Result<ISpeechPhraseAlternates> {
        let mut result__ = ::windows::core::zeroed::<ISpeechPhraseAlternates>();
        (::windows::core::Interface::vtable(self).Alternates)(::windows::core::Interface::as_raw(self), requestcount, startelement, elements, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Audio(&self, startelement: i32, elements: i32) -> ::windows::core::Result<ISpeechMemoryStream> {
        let mut result__ = ::windows::core::zeroed::<ISpeechMemoryStream>();
        (::windows::core::Interface::vtable(self).Audio)(::windows::core::Interface::as_raw(self), startelement, elements, &mut result__).from_abi(result__)
    }
    pub unsafe fn SpeakAudio(&self, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).SpeakAudio)(::windows::core::Interface::as_raw(self), startelement, elements, flags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SaveToMemory(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).SaveToMemory)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DiscardResultInfo(&self, valuetypes: SpeechDiscardType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DiscardResultInfo)(::windows::core::Interface::as_raw(self), valuetypes).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechRecoResult, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechRecoResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechRecoResult {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechRecoResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechRecoResult").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechRecoResult {
    type Vtable = ISpeechRecoResult_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechRecoResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechRecoResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed2879cf_ced9_4ee6_a534_de0191d5468d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecoResult_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RecoContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recocontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RecoContext: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Times: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, times: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Times: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AudioFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AudioFormat: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioFormat: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PhraseInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phraseinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PhraseInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Alternates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestcount: i32, startelement: i32, elements: i32, alternates: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Alternates: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Audio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Audio: usize,
    pub SpeakAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SaveToMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resultblock: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SaveToMemory: usize,
    pub DiscardResultInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuetypes: SpeechDiscardType) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechRecoResult2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecoResult2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RecoContext(&self) -> ::windows::core::Result<ISpeechRecoContext> {
        let mut result__ = ::windows::core::zeroed::<ISpeechRecoContext>();
        (::windows::core::Interface::vtable(self).base__.RecoContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Times(&self) -> ::windows::core::Result<ISpeechRecoResultTimes> {
        let mut result__ = ::windows::core::zeroed::<ISpeechRecoResultTimes>();
        (::windows::core::Interface::vtable(self).base__.Times)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AudioFormat<P0>(&self, format: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechAudioFormat>,
    {
        (::windows::core::Interface::vtable(self).base__.putref_AudioFormat)(::windows::core::Interface::as_raw(self), format.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AudioFormat(&self) -> ::windows::core::Result<ISpeechAudioFormat> {
        let mut result__ = ::windows::core::zeroed::<ISpeechAudioFormat>();
        (::windows::core::Interface::vtable(self).base__.AudioFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PhraseInfo(&self) -> ::windows::core::Result<ISpeechPhraseInfo> {
        let mut result__ = ::windows::core::zeroed::<ISpeechPhraseInfo>();
        (::windows::core::Interface::vtable(self).base__.PhraseInfo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Alternates(&self, requestcount: i32, startelement: i32, elements: i32) -> ::windows::core::Result<ISpeechPhraseAlternates> {
        let mut result__ = ::windows::core::zeroed::<ISpeechPhraseAlternates>();
        (::windows::core::Interface::vtable(self).base__.Alternates)(::windows::core::Interface::as_raw(self), requestcount, startelement, elements, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Audio(&self, startelement: i32, elements: i32) -> ::windows::core::Result<ISpeechMemoryStream> {
        let mut result__ = ::windows::core::zeroed::<ISpeechMemoryStream>();
        (::windows::core::Interface::vtable(self).base__.Audio)(::windows::core::Interface::as_raw(self), startelement, elements, &mut result__).from_abi(result__)
    }
    pub unsafe fn SpeakAudio(&self, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.SpeakAudio)(::windows::core::Interface::as_raw(self), startelement, elements, flags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SaveToMemory(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.SaveToMemory)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DiscardResultInfo(&self, valuetypes: SpeechDiscardType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DiscardResultInfo)(::windows::core::Interface::as_raw(self), valuetypes).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTextFeedback<P0, P1>(&self, feedback: P0, wassuccessful: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetTextFeedback)(::windows::core::Interface::as_raw(self), feedback.into_param().abi(), wassuccessful.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechRecoResult2, ::windows::core::IUnknown, super::super::System::Com::IDispatch, ISpeechRecoResult);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechRecoResult2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechRecoResult2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechRecoResult2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechRecoResult2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechRecoResult2 {
    type Vtable = ISpeechRecoResult2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechRecoResult2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechRecoResult2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e0a246d_d3c8_45de_8657_04290c458c3c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecoResult2_Vtbl {
    pub base__: ISpeechRecoResult_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTextFeedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedback: ::std::mem::MaybeUninit<::windows::core::BSTR>, wassuccessful: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTextFeedback: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechRecoResultDispatch(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecoResultDispatch {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RecoContext(&self) -> ::windows::core::Result<ISpeechRecoContext> {
        let mut result__ = ::windows::core::zeroed::<ISpeechRecoContext>();
        (::windows::core::Interface::vtable(self).RecoContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Times(&self) -> ::windows::core::Result<ISpeechRecoResultTimes> {
        let mut result__ = ::windows::core::zeroed::<ISpeechRecoResultTimes>();
        (::windows::core::Interface::vtable(self).Times)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AudioFormat<P0>(&self, format: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechAudioFormat>,
    {
        (::windows::core::Interface::vtable(self).putref_AudioFormat)(::windows::core::Interface::as_raw(self), format.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AudioFormat(&self) -> ::windows::core::Result<ISpeechAudioFormat> {
        let mut result__ = ::windows::core::zeroed::<ISpeechAudioFormat>();
        (::windows::core::Interface::vtable(self).AudioFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PhraseInfo(&self) -> ::windows::core::Result<ISpeechPhraseInfo> {
        let mut result__ = ::windows::core::zeroed::<ISpeechPhraseInfo>();
        (::windows::core::Interface::vtable(self).PhraseInfo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Alternates(&self, requestcount: i32, startelement: i32, elements: i32) -> ::windows::core::Result<ISpeechPhraseAlternates> {
        let mut result__ = ::windows::core::zeroed::<ISpeechPhraseAlternates>();
        (::windows::core::Interface::vtable(self).Alternates)(::windows::core::Interface::as_raw(self), requestcount, startelement, elements, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Audio(&self, startelement: i32, elements: i32) -> ::windows::core::Result<ISpeechMemoryStream> {
        let mut result__ = ::windows::core::zeroed::<ISpeechMemoryStream>();
        (::windows::core::Interface::vtable(self).Audio)(::windows::core::Interface::as_raw(self), startelement, elements, &mut result__).from_abi(result__)
    }
    pub unsafe fn SpeakAudio(&self, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).SpeakAudio)(::windows::core::Interface::as_raw(self), startelement, elements, flags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SaveToMemory(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).SaveToMemory)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DiscardResultInfo(&self, valuetypes: SpeechDiscardType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DiscardResultInfo)(::windows::core::Interface::as_raw(self), valuetypes).ok()
    }
    pub unsafe fn GetXMLResult(&self, options: SPXMLRESULTOPTIONS) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetXMLResult)(::windows::core::Interface::as_raw(self), options, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetXMLErrorInfo(&self, linenumber: *mut i32, scriptline: *mut ::windows::core::BSTR, source: *mut ::windows::core::BSTR, description: *mut ::windows::core::BSTR, resultcode: *mut ::windows::core::HRESULT, iserror: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetXMLErrorInfo)(::windows::core::Interface::as_raw(self), linenumber, ::core::mem::transmute(scriptline), ::core::mem::transmute(source), ::core::mem::transmute(description), resultcode, iserror).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTextFeedback<P0, P1>(&self, feedback: P0, wassuccessful: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetTextFeedback)(::windows::core::Interface::as_raw(self), feedback.into_param().abi(), wassuccessful.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechRecoResultDispatch, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechRecoResultDispatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechRecoResultDispatch {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechRecoResultDispatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechRecoResultDispatch").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechRecoResultDispatch {
    type Vtable = ISpeechRecoResultDispatch_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechRecoResultDispatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechRecoResultDispatch {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d60eb64_aced_40a6_bbf3_4e557f71dee2);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecoResultDispatch_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub RecoContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recocontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RecoContext: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Times: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, times: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Times: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AudioFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AudioFormat: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioFormat: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub PhraseInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phraseinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    PhraseInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Alternates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestcount: i32, startelement: i32, elements: i32, alternates: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Alternates: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Audio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Audio: usize,
    pub SpeakAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SaveToMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resultblock: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SaveToMemory: usize,
    pub DiscardResultInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, valuetypes: SpeechDiscardType) -> ::windows::core::HRESULT,
    pub GetXMLResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: SPXMLRESULTOPTIONS, presult: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetXMLErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linenumber: *mut i32, scriptline: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, source: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, description: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, resultcode: *mut ::windows::core::HRESULT, iserror: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetXMLErrorInfo: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTextFeedback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedback: ::std::mem::MaybeUninit<::windows::core::BSTR>, wassuccessful: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTextFeedback: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechRecoResultTimes(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecoResultTimes {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn StreamTime(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).StreamTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Length(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).Length)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TickCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).TickCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn OffsetFromStart(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).OffsetFromStart)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechRecoResultTimes, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechRecoResultTimes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechRecoResultTimes {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechRecoResultTimes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechRecoResultTimes").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechRecoResultTimes {
    type Vtable = ISpeechRecoResultTimes_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechRecoResultTimes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechRecoResultTimes {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62b3b8fb_f6e7_41be_bdcb_056b1c29efc0);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecoResultTimes_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub StreamTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, time: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    StreamTime: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Length: usize,
    pub TickCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tickcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub OffsetFromStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetfromstart: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    OffsetFromStart: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechRecognizer(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecognizer {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_Recognizer<P0>(&self, recognizer: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechObjectToken>,
    {
        (::windows::core::Interface::vtable(self).putref_Recognizer)(::windows::core::Interface::as_raw(self), recognizer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Recognizer(&self) -> ::windows::core::Result<ISpeechObjectToken> {
        let mut result__ = ::windows::core::zeroed::<ISpeechObjectToken>();
        (::windows::core::Interface::vtable(self).Recognizer)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowAudioInputFormatChangesOnNextSet<P0>(&self, allow: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetAllowAudioInputFormatChangesOnNextSet)(::windows::core::Interface::as_raw(self), allow.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowAudioInputFormatChangesOnNextSet(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).AllowAudioInputFormatChangesOnNextSet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AudioInput<P0>(&self, audioinput: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechObjectToken>,
    {
        (::windows::core::Interface::vtable(self).putref_AudioInput)(::windows::core::Interface::as_raw(self), audioinput.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AudioInput(&self) -> ::windows::core::Result<ISpeechObjectToken> {
        let mut result__ = ::windows::core::zeroed::<ISpeechObjectToken>();
        (::windows::core::Interface::vtable(self).AudioInput)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AudioInputStream<P0>(&self, audioinputstream: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechBaseStream>,
    {
        (::windows::core::Interface::vtable(self).putref_AudioInputStream)(::windows::core::Interface::as_raw(self), audioinputstream.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AudioInputStream(&self) -> ::windows::core::Result<ISpeechBaseStream> {
        let mut result__ = ::windows::core::zeroed::<ISpeechBaseStream>();
        (::windows::core::Interface::vtable(self).AudioInputStream)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsShared(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsShared)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetState(&self, state: SpeechRecognizerState) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetState)(::windows::core::Interface::as_raw(self), state).ok()
    }
    pub unsafe fn State(&self) -> ::windows::core::Result<SpeechRecognizerState> {
        let mut result__ = ::windows::core::zeroed::<SpeechRecognizerState>();
        (::windows::core::Interface::vtable(self).State)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Status(&self) -> ::windows::core::Result<ISpeechRecognizerStatus> {
        let mut result__ = ::windows::core::zeroed::<ISpeechRecognizerStatus>();
        (::windows::core::Interface::vtable(self).Status)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_Profile<P0>(&self, profile: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechObjectToken>,
    {
        (::windows::core::Interface::vtable(self).putref_Profile)(::windows::core::Interface::as_raw(self), profile.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Profile(&self) -> ::windows::core::Result<ISpeechObjectToken> {
        let mut result__ = ::windows::core::zeroed::<ISpeechObjectToken>();
        (::windows::core::Interface::vtable(self).Profile)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EmulateRecognition(&self, textelements: super::super::System::Com::VARIANT, elementdisplayattributes: *const super::super::System::Com::VARIANT, languageid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EmulateRecognition)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(textelements), elementdisplayattributes, languageid).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateRecoContext(&self) -> ::windows::core::Result<ISpeechRecoContext> {
        let mut result__ = ::windows::core::zeroed::<ISpeechRecoContext>();
        (::windows::core::Interface::vtable(self).CreateRecoContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetFormat(&self, r#type: SpeechFormatType) -> ::windows::core::Result<ISpeechAudioFormat> {
        let mut result__ = ::windows::core::zeroed::<ISpeechAudioFormat>();
        (::windows::core::Interface::vtable(self).GetFormat)(::windows::core::Interface::as_raw(self), r#type, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPropertyNumber<P0>(&self, name: P0, value: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).SetPropertyNumber)(::windows::core::Interface::as_raw(self), name.into_param().abi(), value, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyNumber<P0>(&self, name: P0, value: *mut i32, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).GetPropertyNumber)(::windows::core::Interface::as_raw(self), name.into_param().abi(), value, supported).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPropertyString<P0, P1>(&self, name: P0, value: P1) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).SetPropertyString)(::windows::core::Interface::as_raw(self), name.into_param().abi(), value.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyString<P0>(&self, name: P0, value: *mut ::windows::core::BSTR, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).GetPropertyString)(::windows::core::Interface::as_raw(self), name.into_param().abi(), ::core::mem::transmute(value), supported).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn IsUISupported<P0>(&self, typeofui: P0, extradata: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsUISupported)(::windows::core::Interface::as_raw(self), typeofui.into_param().abi(), extradata, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DisplayUI<P0, P1>(&self, hwndparent: i32, title: P0, typeofui: P1, extradata: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DisplayUI)(::windows::core::Interface::as_raw(self), hwndparent, title.into_param().abi(), typeofui.into_param().abi(), extradata).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRecognizers<P0, P1>(&self, requiredattributes: P0, optionalattributes: P1) -> ::windows::core::Result<ISpeechObjectTokens>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISpeechObjectTokens>();
        (::windows::core::Interface::vtable(self).GetRecognizers)(::windows::core::Interface::as_raw(self), requiredattributes.into_param().abi(), optionalattributes.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAudioInputs<P0, P1>(&self, requiredattributes: P0, optionalattributes: P1) -> ::windows::core::Result<ISpeechObjectTokens>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISpeechObjectTokens>();
        (::windows::core::Interface::vtable(self).GetAudioInputs)(::windows::core::Interface::as_raw(self), requiredattributes.into_param().abi(), optionalattributes.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProfiles<P0, P1>(&self, requiredattributes: P0, optionalattributes: P1) -> ::windows::core::Result<ISpeechObjectTokens>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISpeechObjectTokens>();
        (::windows::core::Interface::vtable(self).GetProfiles)(::windows::core::Interface::as_raw(self), requiredattributes.into_param().abi(), optionalattributes.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechRecognizer, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechRecognizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechRecognizer {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechRecognizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechRecognizer").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechRecognizer {
    type Vtable = ISpeechRecognizer_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechRecognizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechRecognizer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d5f1c0c_bd75_4b08_9478_3b11fea2586c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizer_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Recognizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recognizer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Recognizer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Recognizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recognizer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Recognizer: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowAudioInputFormatChangesOnNextSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowAudioInputFormatChangesOnNextSet: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowAudioInputFormatChangesOnNextSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowAudioInputFormatChangesOnNextSet: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AudioInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audioinput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AudioInput: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audioinput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioInput: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AudioInputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audioinputstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AudioInputStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioInputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audioinputstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioInputStream: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsShared: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shared: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsShared: usize,
    pub SetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: SpeechRecognizerState) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: *mut SpeechRecognizerState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Status: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Profile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Profile: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Profile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Profile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EmulateRecognition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textelements: super::super::System::Com::VARIANT, elementdisplayattributes: *const super::super::System::Com::VARIANT, languageid: i32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EmulateRecognition: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateRecoContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateRecoContext: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: SpeechFormatType, format: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetFormat: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPropertyNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: i32, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPropertyNumber: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPropertyNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: *mut i32, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPropertyNumber: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPropertyString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: ::std::mem::MaybeUninit<::windows::core::BSTR>, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPropertyString: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPropertyString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, value: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPropertyString: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub IsUISupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typeofui: ::std::mem::MaybeUninit<::windows::core::BSTR>, extradata: *const super::super::System::Com::VARIANT, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    IsUISupported: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DisplayUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: i32, title: ::std::mem::MaybeUninit<::windows::core::BSTR>, typeofui: ::std::mem::MaybeUninit<::windows::core::BSTR>, extradata: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DisplayUI: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRecognizers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requiredattributes: ::std::mem::MaybeUninit<::windows::core::BSTR>, optionalattributes: ::std::mem::MaybeUninit<::windows::core::BSTR>, objecttokens: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRecognizers: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAudioInputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requiredattributes: ::std::mem::MaybeUninit<::windows::core::BSTR>, optionalattributes: ::std::mem::MaybeUninit<::windows::core::BSTR>, objecttokens: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAudioInputs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requiredattributes: ::std::mem::MaybeUninit<::windows::core::BSTR>, optionalattributes: ::std::mem::MaybeUninit<::windows::core::BSTR>, objecttokens: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetProfiles: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechRecognizerStatus(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechRecognizerStatus {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AudioStatus(&self) -> ::windows::core::Result<ISpeechAudioStatus> {
        let mut result__ = ::windows::core::zeroed::<ISpeechAudioStatus>();
        (::windows::core::Interface::vtable(self).AudioStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CurrentStreamPosition(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).CurrentStreamPosition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CurrentStreamNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).CurrentStreamNumber)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NumberOfActiveRules(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).NumberOfActiveRules)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ClsidEngine(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).ClsidEngine)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SupportedLanguages(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).SupportedLanguages)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechRecognizerStatus, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechRecognizerStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechRecognizerStatus {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechRecognizerStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechRecognizerStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechRecognizerStatus {
    type Vtable = ISpeechRecognizerStatus_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechRecognizerStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechRecognizerStatus {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbff9e781_53ec_484e_bb8a_0e1b5551e35c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechRecognizerStatus_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiostatus: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioStatus: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CurrentStreamPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcurrentstreampos: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CurrentStreamPosition: usize,
    pub CurrentStreamNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamnumber: *mut i32) -> ::windows::core::HRESULT,
    pub NumberOfActiveRules: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numberofactiverules: *mut i32) -> ::windows::core::HRESULT,
    pub ClsidEngine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsidengine: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SupportedLanguages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedlanguages: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SupportedLanguages: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechResourceLoader(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechResourceLoader {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LoadResource<P0, P1>(&self, bstrresourceuri: P0, falwaysreload: P1, pstream: *mut ::core::option::Option<::windows::core::IUnknown>, pbstrmimetype: ::core::option::Option<*mut ::windows::core::BSTR>, pfmodified: *mut super::super::Foundation::VARIANT_BOOL, pbstrredirecturl: ::core::option::Option<*mut ::windows::core::BSTR>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).LoadResource)(::windows::core::Interface::as_raw(self), bstrresourceuri.into_param().abi(), falwaysreload.into_param().abi(), ::core::mem::transmute(pstream), ::core::mem::transmute(pbstrmimetype.unwrap_or(::std::ptr::null_mut())), pfmodified, ::core::mem::transmute(pbstrredirecturl.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetLocalCopy<P0>(&self, bstrresourceuri: P0, pbstrlocalpath: *mut ::windows::core::BSTR, pbstrmimetype: ::core::option::Option<*mut ::windows::core::BSTR>, pbstrredirecturl: ::core::option::Option<*mut ::windows::core::BSTR>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).GetLocalCopy)(::windows::core::Interface::as_raw(self), bstrresourceuri.into_param().abi(), ::core::mem::transmute(pbstrlocalpath), ::core::mem::transmute(pbstrmimetype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pbstrredirecturl.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ReleaseLocalCopy<P0>(&self, pbstrlocalpath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).ReleaseLocalCopy)(::windows::core::Interface::as_raw(self), pbstrlocalpath.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechResourceLoader, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechResourceLoader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechResourceLoader {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechResourceLoader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechResourceLoader").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechResourceLoader {
    type Vtable = ISpeechResourceLoader_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechResourceLoader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechResourceLoader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9ac5783_fcd0_4b21_b119_b4f8da8fd2c3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechResourceLoader_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub LoadResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrresourceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, falwaysreload: super::super::Foundation::VARIANT_BOOL, pstream: *mut *mut ::core::ffi::c_void, pbstrmimetype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pfmodified: *mut super::super::Foundation::VARIANT_BOOL, pbstrredirecturl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    LoadResource: usize,
    pub GetLocalCopy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrresourceuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrlocalpath: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrmimetype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrredirecturl: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ReleaseLocalCopy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlocalpath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechTextSelectionInformation(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechTextSelectionInformation {
    pub unsafe fn SetActiveOffset(&self, activeoffset: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetActiveOffset)(::windows::core::Interface::as_raw(self), activeoffset).ok()
    }
    pub unsafe fn ActiveOffset(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).ActiveOffset)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetActiveLength(&self, activelength: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetActiveLength)(::windows::core::Interface::as_raw(self), activelength).ok()
    }
    pub unsafe fn ActiveLength(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).ActiveLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSelectionOffset(&self, selectionoffset: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSelectionOffset)(::windows::core::Interface::as_raw(self), selectionoffset).ok()
    }
    pub unsafe fn SelectionOffset(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).SelectionOffset)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSelectionLength(&self, selectionlength: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSelectionLength)(::windows::core::Interface::as_raw(self), selectionlength).ok()
    }
    pub unsafe fn SelectionLength(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).SelectionLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechTextSelectionInformation, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechTextSelectionInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechTextSelectionInformation {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechTextSelectionInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechTextSelectionInformation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechTextSelectionInformation {
    type Vtable = ISpeechTextSelectionInformation_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechTextSelectionInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechTextSelectionInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b9c7e7a_6eee_4ded_9092_11657279adbe);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechTextSelectionInformation_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub SetActiveOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activeoffset: i32) -> ::windows::core::HRESULT,
    pub ActiveOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activeoffset: *mut i32) -> ::windows::core::HRESULT,
    pub SetActiveLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activelength: i32) -> ::windows::core::HRESULT,
    pub ActiveLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activelength: *mut i32) -> ::windows::core::HRESULT,
    pub SetSelectionOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selectionoffset: i32) -> ::windows::core::HRESULT,
    pub SelectionOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selectionoffset: *mut i32) -> ::windows::core::HRESULT,
    pub SetSelectionLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selectionlength: i32) -> ::windows::core::HRESULT,
    pub SelectionLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selectionlength: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechVoice(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechVoice {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Status(&self) -> ::windows::core::Result<ISpeechVoiceStatus> {
        let mut result__ = ::windows::core::zeroed::<ISpeechVoiceStatus>();
        (::windows::core::Interface::vtable(self).Status)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Voice(&self) -> ::windows::core::Result<ISpeechObjectToken> {
        let mut result__ = ::windows::core::zeroed::<ISpeechObjectToken>();
        (::windows::core::Interface::vtable(self).Voice)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_Voice<P0>(&self, voice: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechObjectToken>,
    {
        (::windows::core::Interface::vtable(self).putref_Voice)(::windows::core::Interface::as_raw(self), voice.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AudioOutput(&self) -> ::windows::core::Result<ISpeechObjectToken> {
        let mut result__ = ::windows::core::zeroed::<ISpeechObjectToken>();
        (::windows::core::Interface::vtable(self).AudioOutput)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AudioOutput<P0>(&self, audiooutput: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechObjectToken>,
    {
        (::windows::core::Interface::vtable(self).putref_AudioOutput)(::windows::core::Interface::as_raw(self), audiooutput.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AudioOutputStream(&self) -> ::windows::core::Result<ISpeechBaseStream> {
        let mut result__ = ::windows::core::zeroed::<ISpeechBaseStream>();
        (::windows::core::Interface::vtable(self).AudioOutputStream)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AudioOutputStream<P0>(&self, audiooutputstream: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechBaseStream>,
    {
        (::windows::core::Interface::vtable(self).putref_AudioOutputStream)(::windows::core::Interface::as_raw(self), audiooutputstream.into_param().abi()).ok()
    }
    pub unsafe fn Rate(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Rate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRate(&self, rate: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRate)(::windows::core::Interface::as_raw(self), rate).ok()
    }
    pub unsafe fn Volume(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Volume)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetVolume(&self, volume: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVolume)(::windows::core::Interface::as_raw(self), volume).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowAudioOutputFormatChangesOnNextSet<P0>(&self, allow: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetAllowAudioOutputFormatChangesOnNextSet)(::windows::core::Interface::as_raw(self), allow.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowAudioOutputFormatChangesOnNextSet(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).AllowAudioOutputFormatChangesOnNextSet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EventInterests(&self) -> ::windows::core::Result<SpeechVoiceEvents> {
        let mut result__ = ::windows::core::zeroed::<SpeechVoiceEvents>();
        (::windows::core::Interface::vtable(self).EventInterests)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEventInterests(&self, eventinterestflags: SpeechVoiceEvents) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventInterests)(::windows::core::Interface::as_raw(self), eventinterestflags).ok()
    }
    pub unsafe fn SetPriority(&self, priority: SpeechVoicePriority) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPriority)(::windows::core::Interface::as_raw(self), priority).ok()
    }
    pub unsafe fn Priority(&self) -> ::windows::core::Result<SpeechVoicePriority> {
        let mut result__ = ::windows::core::zeroed::<SpeechVoicePriority>();
        (::windows::core::Interface::vtable(self).Priority)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAlertBoundary(&self, boundary: SpeechVoiceEvents) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAlertBoundary)(::windows::core::Interface::as_raw(self), boundary).ok()
    }
    pub unsafe fn AlertBoundary(&self) -> ::windows::core::Result<SpeechVoiceEvents> {
        let mut result__ = ::windows::core::zeroed::<SpeechVoiceEvents>();
        (::windows::core::Interface::vtable(self).AlertBoundary)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSynchronousSpeakTimeout(&self, mstimeout: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSynchronousSpeakTimeout)(::windows::core::Interface::as_raw(self), mstimeout).ok()
    }
    pub unsafe fn SynchronousSpeakTimeout(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).SynchronousSpeakTimeout)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Speak<P0>(&self, text: P0, flags: SpeechVoiceSpeakFlags) -> ::windows::core::Result<i32>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Speak)(::windows::core::Interface::as_raw(self), text.into_param().abi(), flags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SpeakStream<P0>(&self, stream: P0, flags: SpeechVoiceSpeakFlags) -> ::windows::core::Result<i32>
    where
        P0: ::windows::core::IntoParam<ISpeechBaseStream>,
    {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).SpeakStream)(::windows::core::Interface::as_raw(self), stream.into_param().abi(), flags, &mut result__).from_abi(result__)
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Pause)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Resume(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resume)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Skip<P0>(&self, r#type: P0, numitems: i32) -> ::windows::core::Result<i32>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), r#type.into_param().abi(), numitems, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetVoices<P0, P1>(&self, requiredattributes: P0, optionalattributes: P1) -> ::windows::core::Result<ISpeechObjectTokens>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISpeechObjectTokens>();
        (::windows::core::Interface::vtable(self).GetVoices)(::windows::core::Interface::as_raw(self), requiredattributes.into_param().abi(), optionalattributes.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAudioOutputs<P0, P1>(&self, requiredattributes: P0, optionalattributes: P1) -> ::windows::core::Result<ISpeechObjectTokens>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<ISpeechObjectTokens>();
        (::windows::core::Interface::vtable(self).GetAudioOutputs)(::windows::core::Interface::as_raw(self), requiredattributes.into_param().abi(), optionalattributes.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WaitUntilDone(&self, mstimeout: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).WaitUntilDone)(::windows::core::Interface::as_raw(self), mstimeout, &mut result__).from_abi(result__)
    }
    pub unsafe fn SpeakCompleteEvent(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).SpeakCompleteEvent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn IsUISupported<P0>(&self, typeofui: P0, extradata: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).IsUISupported)(::windows::core::Interface::as_raw(self), typeofui.into_param().abi(), extradata, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DisplayUI<P0, P1>(&self, hwndparent: i32, title: P0, typeofui: P1, extradata: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).DisplayUI)(::windows::core::Interface::as_raw(self), hwndparent, title.into_param().abi(), typeofui.into_param().abi(), extradata).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechVoice, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechVoice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechVoice {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechVoice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechVoice").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechVoice {
    type Vtable = ISpeechVoice_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechVoice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechVoice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x269316d8_57bd_11d2_9eee_00c04f797396);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechVoice_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Status: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Voice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, voice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Voice: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_Voice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, voice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_Voice: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiooutput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioOutput: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AudioOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiooutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AudioOutput: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AudioOutputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiooutputstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AudioOutputStream: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub putref_AudioOutputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audiooutputstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    putref_AudioOutputStream: usize,
    pub Rate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rate: *mut i32) -> ::windows::core::HRESULT,
    pub SetRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rate: i32) -> ::windows::core::HRESULT,
    pub Volume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volume: *mut i32) -> ::windows::core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, volume: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowAudioOutputFormatChangesOnNextSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowAudioOutputFormatChangesOnNextSet: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowAudioOutputFormatChangesOnNextSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowAudioOutputFormatChangesOnNextSet: usize,
    pub EventInterests: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventinterestflags: *mut SpeechVoiceEvents) -> ::windows::core::HRESULT,
    pub SetEventInterests: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventinterestflags: SpeechVoiceEvents) -> ::windows::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, priority: SpeechVoicePriority) -> ::windows::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, priority: *mut SpeechVoicePriority) -> ::windows::core::HRESULT,
    pub SetAlertBoundary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boundary: SpeechVoiceEvents) -> ::windows::core::HRESULT,
    pub AlertBoundary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boundary: *mut SpeechVoiceEvents) -> ::windows::core::HRESULT,
    pub SetSynchronousSpeakTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mstimeout: i32) -> ::windows::core::HRESULT,
    pub SynchronousSpeakTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mstimeout: *mut i32) -> ::windows::core::HRESULT,
    pub Speak: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows::core::BSTR>, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SpeakStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SpeakStream: usize,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Resume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: ::std::mem::MaybeUninit<::windows::core::BSTR>, numitems: i32, numskipped: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetVoices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requiredattributes: ::std::mem::MaybeUninit<::windows::core::BSTR>, optionalattributes: ::std::mem::MaybeUninit<::windows::core::BSTR>, objecttokens: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetVoices: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetAudioOutputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requiredattributes: ::std::mem::MaybeUninit<::windows::core::BSTR>, optionalattributes: ::std::mem::MaybeUninit<::windows::core::BSTR>, objecttokens: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetAudioOutputs: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WaitUntilDone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mstimeout: i32, done: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WaitUntilDone: usize,
    pub SpeakCompleteEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub IsUISupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typeofui: ::std::mem::MaybeUninit<::windows::core::BSTR>, extradata: *const super::super::System::Com::VARIANT, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    IsUISupported: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DisplayUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: i32, title: ::std::mem::MaybeUninit<::windows::core::BSTR>, typeofui: ::std::mem::MaybeUninit<::windows::core::BSTR>, extradata: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DisplayUI: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechVoiceStatus(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechVoiceStatus {
    pub unsafe fn CurrentStreamNumber(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).CurrentStreamNumber)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastStreamNumberQueued(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).LastStreamNumberQueued)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastHResult(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).LastHResult)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RunningState(&self) -> ::windows::core::Result<SpeechRunState> {
        let mut result__ = ::windows::core::zeroed::<SpeechRunState>();
        (::windows::core::Interface::vtable(self).RunningState)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InputWordPosition(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).InputWordPosition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InputWordLength(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).InputWordLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InputSentencePosition(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).InputSentencePosition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InputSentenceLength(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).InputSentenceLength)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastBookmark(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).LastBookmark)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LastBookmarkId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).LastBookmarkId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PhonemeId(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::windows::core::zeroed::<i16>();
        (::windows::core::Interface::vtable(self).PhonemeId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn VisemeId(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::windows::core::zeroed::<i16>();
        (::windows::core::Interface::vtable(self).VisemeId)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechVoiceStatus, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechVoiceStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechVoiceStatus {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechVoiceStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechVoiceStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechVoiceStatus {
    type Vtable = ISpeechVoiceStatus_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechVoiceStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechVoiceStatus {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8be47b07_57f6_11d2_9eee_00c04f797396);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechVoiceStatus_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub CurrentStreamNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamnumber: *mut i32) -> ::windows::core::HRESULT,
    pub LastStreamNumberQueued: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamnumber: *mut i32) -> ::windows::core::HRESULT,
    pub LastHResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hresult: *mut i32) -> ::windows::core::HRESULT,
    pub RunningState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: *mut SpeechRunState) -> ::windows::core::HRESULT,
    pub InputWordPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: *mut i32) -> ::windows::core::HRESULT,
    pub InputWordLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT,
    pub InputSentencePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: *mut i32) -> ::windows::core::HRESULT,
    pub InputSentenceLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT,
    pub LastBookmark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bookmark: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub LastBookmarkId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bookmarkid: *mut i32) -> ::windows::core::HRESULT,
    pub PhonemeId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phoneid: *mut i16) -> ::windows::core::HRESULT,
    pub VisemeId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visemeid: *mut i16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechWaveFormatEx(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechWaveFormatEx {
    pub unsafe fn FormatTag(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::windows::core::zeroed::<i16>();
        (::windows::core::Interface::vtable(self).FormatTag)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFormatTag(&self, formattag: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFormatTag)(::windows::core::Interface::as_raw(self), formattag).ok()
    }
    pub unsafe fn Channels(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::windows::core::zeroed::<i16>();
        (::windows::core::Interface::vtable(self).Channels)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetChannels(&self, channels: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetChannels)(::windows::core::Interface::as_raw(self), channels).ok()
    }
    pub unsafe fn SamplesPerSec(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).SamplesPerSec)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSamplesPerSec(&self, samplespersec: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSamplesPerSec)(::windows::core::Interface::as_raw(self), samplespersec).ok()
    }
    pub unsafe fn AvgBytesPerSec(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).AvgBytesPerSec)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAvgBytesPerSec(&self, avgbytespersec: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAvgBytesPerSec)(::windows::core::Interface::as_raw(self), avgbytespersec).ok()
    }
    pub unsafe fn BlockAlign(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::windows::core::zeroed::<i16>();
        (::windows::core::Interface::vtable(self).BlockAlign)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBlockAlign(&self, blockalign: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlockAlign)(::windows::core::Interface::as_raw(self), blockalign).ok()
    }
    pub unsafe fn BitsPerSample(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::windows::core::zeroed::<i16>();
        (::windows::core::Interface::vtable(self).BitsPerSample)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBitsPerSample(&self, bitspersample: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBitsPerSample)(::windows::core::Interface::as_raw(self), bitspersample).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExtraData(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).ExtraData)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetExtraData(&self, extradata: super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetExtraData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(extradata)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechWaveFormatEx, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechWaveFormatEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechWaveFormatEx {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechWaveFormatEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechWaveFormatEx").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechWaveFormatEx {
    type Vtable = ISpeechWaveFormatEx_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechWaveFormatEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechWaveFormatEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a1ef0d5_1581_4741_88e4_209a49f11a10);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechWaveFormatEx_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
    pub FormatTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formattag: *mut i16) -> ::windows::core::HRESULT,
    pub SetFormatTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formattag: i16) -> ::windows::core::HRESULT,
    pub Channels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channels: *mut i16) -> ::windows::core::HRESULT,
    pub SetChannels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, channels: i16) -> ::windows::core::HRESULT,
    pub SamplesPerSec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, samplespersec: *mut i32) -> ::windows::core::HRESULT,
    pub SetSamplesPerSec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, samplespersec: i32) -> ::windows::core::HRESULT,
    pub AvgBytesPerSec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, avgbytespersec: *mut i32) -> ::windows::core::HRESULT,
    pub SetAvgBytesPerSec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, avgbytespersec: i32) -> ::windows::core::HRESULT,
    pub BlockAlign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blockalign: *mut i16) -> ::windows::core::HRESULT,
    pub SetBlockAlign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blockalign: i16) -> ::windows::core::HRESULT,
    pub BitsPerSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitspersample: *mut i16) -> ::windows::core::HRESULT,
    pub SetBitsPerSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitspersample: i16) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ExtraData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extradata: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ExtraData: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetExtraData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extradata: super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetExtraData: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISpeechXMLRecoResult(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISpeechXMLRecoResult {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RecoContext(&self) -> ::windows::core::Result<ISpeechRecoContext> {
        let mut result__ = ::windows::core::zeroed::<ISpeechRecoContext>();
        (::windows::core::Interface::vtable(self).base__.RecoContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Times(&self) -> ::windows::core::Result<ISpeechRecoResultTimes> {
        let mut result__ = ::windows::core::zeroed::<ISpeechRecoResultTimes>();
        (::windows::core::Interface::vtable(self).base__.Times)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn putref_AudioFormat<P0>(&self, format: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISpeechAudioFormat>,
    {
        (::windows::core::Interface::vtable(self).base__.putref_AudioFormat)(::windows::core::Interface::as_raw(self), format.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AudioFormat(&self) -> ::windows::core::Result<ISpeechAudioFormat> {
        let mut result__ = ::windows::core::zeroed::<ISpeechAudioFormat>();
        (::windows::core::Interface::vtable(self).base__.AudioFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PhraseInfo(&self) -> ::windows::core::Result<ISpeechPhraseInfo> {
        let mut result__ = ::windows::core::zeroed::<ISpeechPhraseInfo>();
        (::windows::core::Interface::vtable(self).base__.PhraseInfo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Alternates(&self, requestcount: i32, startelement: i32, elements: i32) -> ::windows::core::Result<ISpeechPhraseAlternates> {
        let mut result__ = ::windows::core::zeroed::<ISpeechPhraseAlternates>();
        (::windows::core::Interface::vtable(self).base__.Alternates)(::windows::core::Interface::as_raw(self), requestcount, startelement, elements, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Audio(&self, startelement: i32, elements: i32) -> ::windows::core::Result<ISpeechMemoryStream> {
        let mut result__ = ::windows::core::zeroed::<ISpeechMemoryStream>();
        (::windows::core::Interface::vtable(self).base__.Audio)(::windows::core::Interface::as_raw(self), startelement, elements, &mut result__).from_abi(result__)
    }
    pub unsafe fn SpeakAudio(&self, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.SpeakAudio)(::windows::core::Interface::as_raw(self), startelement, elements, flags, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SaveToMemory(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).base__.SaveToMemory)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DiscardResultInfo(&self, valuetypes: SpeechDiscardType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DiscardResultInfo)(::windows::core::Interface::as_raw(self), valuetypes).ok()
    }
    pub unsafe fn GetXMLResult(&self, options: SPXMLRESULTOPTIONS) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetXMLResult)(::windows::core::Interface::as_raw(self), options, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetXMLErrorInfo(&self, linenumber: *mut i32, scriptline: *mut ::windows::core::BSTR, source: *mut ::windows::core::BSTR, description: *mut ::windows::core::BSTR, resultcode: *mut i32, iserror: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetXMLErrorInfo)(::windows::core::Interface::as_raw(self), linenumber, ::core::mem::transmute(scriptline), ::core::mem::transmute(source), ::core::mem::transmute(description), resultcode, iserror).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ISpeechXMLRecoResult, ::windows::core::IUnknown, super::super::System::Com::IDispatch, ISpeechRecoResult);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISpeechXMLRecoResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISpeechXMLRecoResult {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISpeechXMLRecoResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpeechXMLRecoResult").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISpeechXMLRecoResult {
    type Vtable = ISpeechXMLRecoResult_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISpeechXMLRecoResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ISpeechXMLRecoResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaaec54af_8f85_4924_944d_b79d39d72e19);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechXMLRecoResult_Vtbl {
    pub base__: ISpeechRecoResult_Vtbl,
    pub GetXMLResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: SPXMLRESULTOPTIONS, presult: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetXMLErrorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linenumber: *mut i32, scriptline: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, source: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, description: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, resultcode: *mut i32, iserror: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetXMLErrorInfo: usize,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct _ISpeechRecoContextEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl _ISpeechRecoContextEvents {}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(_ISpeechRecoContextEvents, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for _ISpeechRecoContextEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for _ISpeechRecoContextEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for _ISpeechRecoContextEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_ISpeechRecoContextEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for _ISpeechRecoContextEvents {
    type Vtable = _ISpeechRecoContextEvents_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for _ISpeechRecoContextEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for _ISpeechRecoContextEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b8fcb42_0e9d_4f00_a048_7b04d6179d3d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct _ISpeechRecoContextEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct _ISpeechVoiceEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl _ISpeechVoiceEvents {}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(_ISpeechVoiceEvents, ::windows::core::IUnknown, super::super::System::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for _ISpeechVoiceEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for _ISpeechVoiceEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for _ISpeechVoiceEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_ISpeechVoiceEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for _ISpeechVoiceEvents {
    type Vtable = _ISpeechVoiceEvents_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for _ISpeechVoiceEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for _ISpeechVoiceEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa372acd1_3bef_4bbd_8ffb_cb3e2b416af8);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct _ISpeechVoiceEvents_Vtbl {
    pub base__: super::super::System::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DEFAULT_WEIGHT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAPI_ERROR_BASE: u32 = 20480u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPALTERNATESCLSID: ::windows::core::PCWSTR = ::windows::core::w!("AlternatesCLSID");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCAT_APPLEXICONS: ::windows::core::PCWSTR = ::windows::core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AppLexicons");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCAT_AUDIOIN: ::windows::core::PCWSTR = ::windows::core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioInput");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCAT_AUDIOOUT: ::windows::core::PCWSTR = ::windows::core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioOutput");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCAT_PHONECONVERTERS: ::windows::core::PCWSTR = ::windows::core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\PhoneConverters");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCAT_RECOGNIZERS: ::windows::core::PCWSTR = ::windows::core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\Recognizers");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCAT_RECOPROFILES: ::windows::core::PCWSTR = ::windows::core::w!("HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech\\RecoProfiles");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCAT_TEXTNORMALIZERS: ::windows::core::PCWSTR = ::windows::core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\TextNormalizers");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCAT_VOICES: ::windows::core::PCWSTR = ::windows::core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\Voices");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCURRENT_USER_LEXICON_TOKEN_ID: ::windows::core::PCWSTR = ::windows::core::w!("HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech\\CurrentUserLexicon");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCURRENT_USER_SHORTCUT_TOKEN_ID: ::windows::core::PCWSTR = ::windows::core::w!("HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech\\CurrentUserShortcut");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDICTATION: ::windows::core::PCWSTR = ::windows::core::w!("*");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDUI_AddRemoveWord: ::windows::core::PCWSTR = ::windows::core::w!("AddRemoveWord");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDUI_AudioProperties: ::windows::core::PCWSTR = ::windows::core::w!("AudioProperties");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDUI_AudioVolume: ::windows::core::PCWSTR = ::windows::core::w!("AudioVolume");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDUI_EngineProperties: ::windows::core::PCWSTR = ::windows::core::w!("EngineProperties");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDUI_MicTraining: ::windows::core::PCWSTR = ::windows::core::w!("MicTraining");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDUI_RecoProfileProperties: ::windows::core::PCWSTR = ::windows::core::w!("RecoProfileProperties");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDUI_ShareData: ::windows::core::PCWSTR = ::windows::core::w!("ShareData");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDUI_Tutorial: ::windows::core::PCWSTR = ::windows::core::w!("Tutorial");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDUI_UserEnrollment: ::windows::core::PCWSTR = ::windows::core::w!("UserEnrollment");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDUI_UserTraining: ::windows::core::PCWSTR = ::windows::core::w!("UserTraining");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPINFDICTATION: ::windows::core::PCWSTR = ::windows::core::w!("*+");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPMMSYS_AUDIO_IN_TOKEN_ID: ::windows::core::PCWSTR = ::windows::core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioInput\\TokenEnums\\MMAudioIn\\");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPMMSYS_AUDIO_OUT_TOKEN_ID: ::windows::core::PCWSTR = ::windows::core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\AudioOutput\\TokenEnums\\MMAudioOut\\");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPROP_ADAPTATION_ON: ::windows::core::PCWSTR = ::windows::core::w!("AdaptationOn");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPROP_COMPLEX_RESPONSE_SPEED: ::windows::core::PCWSTR = ::windows::core::w!("ComplexResponseSpeed");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPROP_HIGH_CONFIDENCE_THRESHOLD: ::windows::core::PCWSTR = ::windows::core::w!("HighConfidenceThreshold");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPROP_LOW_CONFIDENCE_THRESHOLD: ::windows::core::PCWSTR = ::windows::core::w!("LowConfidenceThreshold");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPROP_NORMAL_CONFIDENCE_THRESHOLD: ::windows::core::PCWSTR = ::windows::core::w!("NormalConfidenceThreshold");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPROP_PERSISTED_BACKGROUND_ADAPTATION: ::windows::core::PCWSTR = ::windows::core::w!("PersistedBackgroundAdaptation");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPROP_PERSISTED_LANGUAGE_MODEL_ADAPTATION: ::windows::core::PCWSTR = ::windows::core::w!("PersistedLanguageModelAdaptation");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPROP_RESOURCE_USAGE: ::windows::core::PCWSTR = ::windows::core::w!("ResourceUsage");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPROP_RESPONSE_SPEED: ::windows::core::PCWSTR = ::windows::core::w!("ResponseSpeed");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPROP_UX_IS_LISTENING: ::windows::core::PCWSTR = ::windows::core::w!("UXIsListening");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRECOEXTENSION: ::windows::core::PCWSTR = ::windows::core::w!("RecoExtension");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPREG_LOCAL_MACHINE_ROOT: ::windows::core::PCWSTR = ::windows::core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPREG_SAFE_USER_TOKENS: ::windows::core::PCWSTR = ::windows::core::w!("HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Speech\\UserTokens");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPREG_USER_ROOT: ::windows::core::PCWSTR = ::windows::core::w!("HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Speech");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRP_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPTOKENKEY_ATTRIBUTES: ::windows::core::PCWSTR = ::windows::core::w!("Attributes");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPTOKENKEY_AUDIO_LATENCY_TRUNCATE: ::windows::core::PCWSTR = ::windows::core::w!("LatencyTruncateThreshold");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPTOKENKEY_AUDIO_LATENCY_UPDATE_INTERVAL: ::windows::core::PCWSTR = ::windows::core::w!("LatencyUpdateInterval");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPTOKENKEY_AUDIO_LATENCY_WARNING: ::windows::core::PCWSTR = ::windows::core::w!("LatencyWarningThreshold");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPTOKENKEY_FILES: ::windows::core::PCWSTR = ::windows::core::w!("Files");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPTOKENKEY_RETAINEDAUDIO: ::windows::core::PCWSTR = ::windows::core::w!("SecondsPerRetainedAudioEvent");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPTOKENKEY_UI: ::windows::core::PCWSTR = ::windows::core::w!("UI");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPTOKENVALUE_CLSID: ::windows::core::PCWSTR = ::windows::core::w!("CLSID");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPTOPIC_SPELLING: ::windows::core::PCWSTR = ::windows::core::w!("Spelling");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVOICECATEGORY_TTSRATE: ::windows::core::PCWSTR = ::windows::core::w!("DefaultTTSRate");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPWILDCARD: ::windows::core::PCWSTR = ::windows::core::w!("...");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_EMULATE_RESULT: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_LOW_CONFIDENCE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_MAX_LANGIDS: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_MAX_PRON_LENGTH: u32 = 384u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_MAX_WORD_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_NORMAL_CONFIDENCE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_STREAMPOS_ASAP: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_STREAMPOS_REALTIME: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SR_LOCALIZED_DESCRIPTION: ::windows::core::PCWSTR = ::windows::core::w!("Description");
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpAudioFormat: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ef96870_e160_4792_820d_48cf0649e4ec);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpCompressedLexicon: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90903716_2f42_11d3_9c26_00c04f8ef87c);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpCustomStream: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8dbef13f_1948_4aa8_8cf0_048eebed95d8);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpFileStream: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x947812b3_2ae1_4644_ba86_9e90ded7ec91);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpInProcRecoContext: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73ad6842_ace0_45e8_a4dd_8795881a2c2a);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpInprocRecognizer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41b89b6b_9399_11d2_9623_00c04f8ee628);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpLexicon: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0655e396_25d0_11d3_9c26_00c04f8ef87c);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpMMAudioEnum: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab1890a0_e91f_11d2_bb91_00c04f8ee6c0);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpMMAudioIn: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf3d2e50_53f2_11d2_960c_00c04f8ee628);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpMMAudioOut: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8c680eb_3d32_11d2_9ee7_00c04f797396);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpMemoryStream: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5fb7ef7d_dff4_468a_b6b7_2fcbd188f994);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpNotifyTranslator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2ae5372_5d40_11d2_960e_00c04f8ee628);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpNullPhoneConverter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x455f24e9_7396_4a16_9715_7c0fdbe3efe3);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpObjectToken: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef411752_3736_4cb4_9c8c_8ef4ccb58efe);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpObjectTokenCategory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa910187f_0c7a_45ac_92cc_59edafb77b53);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpPhoneConverter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9185f743_1143_4c28_86b5_bff14f20e5c8);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpPhoneticAlphabetConverter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f414126_dfe3_4629_99ee_797978317ead);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpPhraseInfoBuilder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc23fc28d_c55f_4720_8b32_91f73c2bd5d1);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpResourceManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96749373_3391_11d2_9ee3_00c04f797396);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpSharedRecoContext: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47206204_5eca_11d2_960f_00c04f8ee628);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpSharedRecognizer: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3bee4890_4fe9_4a37_8c1e_5e7e12791c1f);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpShortcut: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d722f1a_9fcf_4e62_96d8_6df8f01a26aa);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpStream: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x715d9c59_4442_11d2_9605_00c04f8ee628);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpStreamFormatConverter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7013943a_e2ec_11d2_a086_00c04f8ef9b5);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpTextSelectionInformation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f92030a_cbfd_4ab8_a164_ff5985547ff6);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpUnCompressedLexicon: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9e37c15_df92_4727_85d6_72e5eeb6995a);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpVoice: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96749377_3391_11d2_9ee3_00c04f797396);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpWaveFormatEx: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc79a574c_63be_44b9_801f_283f87f898be);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SpeechAllElements: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const Speech_Default_Weight: f32 = 1f32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const Speech_Max_Pron_Length: i32 = 384i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const Speech_Max_Word_Length: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const Speech_StreamPos_Asap: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const Speech_StreamPos_RealTime: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPIDSPRG(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGId: DISPIDSPRG = DISPIDSPRG(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGRecoContext: DISPIDSPRG = DISPIDSPRG(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGState: DISPIDSPRG = DISPIDSPRG(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGRules: DISPIDSPRG = DISPIDSPRG(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGReset: DISPIDSPRG = DISPIDSPRG(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGCommit: DISPIDSPRG = DISPIDSPRG(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGCmdLoadFromFile: DISPIDSPRG = DISPIDSPRG(7i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGCmdLoadFromObject: DISPIDSPRG = DISPIDSPRG(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGCmdLoadFromResource: DISPIDSPRG = DISPIDSPRG(9i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGCmdLoadFromMemory: DISPIDSPRG = DISPIDSPRG(10i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGCmdLoadFromProprietaryGrammar: DISPIDSPRG = DISPIDSPRG(11i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGCmdSetRuleState: DISPIDSPRG = DISPIDSPRG(12i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGCmdSetRuleIdState: DISPIDSPRG = DISPIDSPRG(13i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGDictationLoad: DISPIDSPRG = DISPIDSPRG(14i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGDictationUnload: DISPIDSPRG = DISPIDSPRG(15i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGDictationSetState: DISPIDSPRG = DISPIDSPRG(16i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGSetWordSequenceData: DISPIDSPRG = DISPIDSPRG(17i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGSetTextSelection: DISPIDSPRG = DISPIDSPRG(18i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGIsPronounceable: DISPIDSPRG = DISPIDSPRG(19i32);
impl ::core::marker::Copy for DISPIDSPRG {}
impl ::core::clone::Clone for DISPIDSPRG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPIDSPRG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPIDSPRG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPIDSPRG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPIDSPRG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPIDSPTSI(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPIDSPTSI_ActiveOffset: DISPIDSPTSI = DISPIDSPTSI(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPIDSPTSI_ActiveLength: DISPIDSPTSI = DISPIDSPTSI(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPIDSPTSI_SelectionOffset: DISPIDSPTSI = DISPIDSPTSI(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPIDSPTSI_SelectionLength: DISPIDSPTSI = DISPIDSPTSI(4i32);
impl ::core::marker::Copy for DISPIDSPTSI {}
impl ::core::clone::Clone for DISPIDSPTSI {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPIDSPTSI {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPIDSPTSI {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPIDSPTSI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPIDSPTSI").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechAudio(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SAStatus: DISPID_SpeechAudio = DISPID_SpeechAudio(200i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SABufferInfo: DISPID_SpeechAudio = DISPID_SpeechAudio(201i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SADefaultFormat: DISPID_SpeechAudio = DISPID_SpeechAudio(202i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SAVolume: DISPID_SpeechAudio = DISPID_SpeechAudio(203i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SABufferNotifySize: DISPID_SpeechAudio = DISPID_SpeechAudio(204i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SAEventHandle: DISPID_SpeechAudio = DISPID_SpeechAudio(205i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SASetState: DISPID_SpeechAudio = DISPID_SpeechAudio(206i32);
impl ::core::marker::Copy for DISPID_SpeechAudio {}
impl ::core::clone::Clone for DISPID_SpeechAudio {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechAudio {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechAudio {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechAudio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechAudio").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechAudioBufferInfo(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SABIMinNotification: DISPID_SpeechAudioBufferInfo = DISPID_SpeechAudioBufferInfo(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SABIBufferSize: DISPID_SpeechAudioBufferInfo = DISPID_SpeechAudioBufferInfo(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SABIEventBias: DISPID_SpeechAudioBufferInfo = DISPID_SpeechAudioBufferInfo(3i32);
impl ::core::marker::Copy for DISPID_SpeechAudioBufferInfo {}
impl ::core::clone::Clone for DISPID_SpeechAudioBufferInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechAudioBufferInfo {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechAudioBufferInfo {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechAudioBufferInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechAudioBufferInfo").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechAudioFormat(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SAFType: DISPID_SpeechAudioFormat = DISPID_SpeechAudioFormat(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SAFGuid: DISPID_SpeechAudioFormat = DISPID_SpeechAudioFormat(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SAFGetWaveFormatEx: DISPID_SpeechAudioFormat = DISPID_SpeechAudioFormat(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SAFSetWaveFormatEx: DISPID_SpeechAudioFormat = DISPID_SpeechAudioFormat(4i32);
impl ::core::marker::Copy for DISPID_SpeechAudioFormat {}
impl ::core::clone::Clone for DISPID_SpeechAudioFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechAudioFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechAudioFormat {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechAudioFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechAudioFormat").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechAudioStatus(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SASFreeBufferSpace: DISPID_SpeechAudioStatus = DISPID_SpeechAudioStatus(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SASNonBlockingIO: DISPID_SpeechAudioStatus = DISPID_SpeechAudioStatus(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SASState: DISPID_SpeechAudioStatus = DISPID_SpeechAudioStatus(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SASCurrentSeekPosition: DISPID_SpeechAudioStatus = DISPID_SpeechAudioStatus(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SASCurrentDevicePosition: DISPID_SpeechAudioStatus = DISPID_SpeechAudioStatus(5i32);
impl ::core::marker::Copy for DISPID_SpeechAudioStatus {}
impl ::core::clone::Clone for DISPID_SpeechAudioStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechAudioStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechAudioStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechAudioStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechAudioStatus").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechBaseStream(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SBSFormat: DISPID_SpeechBaseStream = DISPID_SpeechBaseStream(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SBSRead: DISPID_SpeechBaseStream = DISPID_SpeechBaseStream(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SBSWrite: DISPID_SpeechBaseStream = DISPID_SpeechBaseStream(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SBSSeek: DISPID_SpeechBaseStream = DISPID_SpeechBaseStream(4i32);
impl ::core::marker::Copy for DISPID_SpeechBaseStream {}
impl ::core::clone::Clone for DISPID_SpeechBaseStream {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechBaseStream {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechBaseStream {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechBaseStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechBaseStream").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechCustomStream(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SCSBaseStream: DISPID_SpeechCustomStream = DISPID_SpeechCustomStream(100i32);
impl ::core::marker::Copy for DISPID_SpeechCustomStream {}
impl ::core::clone::Clone for DISPID_SpeechCustomStream {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechCustomStream {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechCustomStream {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechCustomStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechCustomStream").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechDataKey(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKSetBinaryValue: DISPID_SpeechDataKey = DISPID_SpeechDataKey(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKGetBinaryValue: DISPID_SpeechDataKey = DISPID_SpeechDataKey(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKSetStringValue: DISPID_SpeechDataKey = DISPID_SpeechDataKey(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKGetStringValue: DISPID_SpeechDataKey = DISPID_SpeechDataKey(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKSetLongValue: DISPID_SpeechDataKey = DISPID_SpeechDataKey(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKGetlongValue: DISPID_SpeechDataKey = DISPID_SpeechDataKey(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKOpenKey: DISPID_SpeechDataKey = DISPID_SpeechDataKey(7i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKCreateKey: DISPID_SpeechDataKey = DISPID_SpeechDataKey(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKDeleteKey: DISPID_SpeechDataKey = DISPID_SpeechDataKey(9i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKDeleteValue: DISPID_SpeechDataKey = DISPID_SpeechDataKey(10i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKEnumKeys: DISPID_SpeechDataKey = DISPID_SpeechDataKey(11i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SDKEnumValues: DISPID_SpeechDataKey = DISPID_SpeechDataKey(12i32);
impl ::core::marker::Copy for DISPID_SpeechDataKey {}
impl ::core::clone::Clone for DISPID_SpeechDataKey {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechDataKey {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechDataKey {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechDataKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechDataKey").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechFileStream(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SFSOpen: DISPID_SpeechFileStream = DISPID_SpeechFileStream(100i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SFSClose: DISPID_SpeechFileStream = DISPID_SpeechFileStream(101i32);
impl ::core::marker::Copy for DISPID_SpeechFileStream {}
impl ::core::clone::Clone for DISPID_SpeechFileStream {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechFileStream {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechFileStream {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechFileStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechFileStream").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechGrammarRule(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRAttributes: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRInitialState: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRName: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRId: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRClear: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRAddResource: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRAddState: DISPID_SpeechGrammarRule = DISPID_SpeechGrammarRule(7i32);
impl ::core::marker::Copy for DISPID_SpeechGrammarRule {}
impl ::core::clone::Clone for DISPID_SpeechGrammarRule {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechGrammarRule {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechGrammarRule {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechGrammarRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechGrammarRule").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechGrammarRuleState(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSRule: DISPID_SpeechGrammarRuleState = DISPID_SpeechGrammarRuleState(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTransitions: DISPID_SpeechGrammarRuleState = DISPID_SpeechGrammarRuleState(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSAddWordTransition: DISPID_SpeechGrammarRuleState = DISPID_SpeechGrammarRuleState(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSAddRuleTransition: DISPID_SpeechGrammarRuleState = DISPID_SpeechGrammarRuleState(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSAddSpecialTransition: DISPID_SpeechGrammarRuleState = DISPID_SpeechGrammarRuleState(5i32);
impl ::core::marker::Copy for DISPID_SpeechGrammarRuleState {}
impl ::core::clone::Clone for DISPID_SpeechGrammarRuleState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechGrammarRuleState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechGrammarRuleState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechGrammarRuleState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechGrammarRuleState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechGrammarRuleStateTransition(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTType: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTText: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTRule: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTWeight: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTPropertyName: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTPropertyId: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTPropertyValue: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(7i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTNextState: DISPID_SpeechGrammarRuleStateTransition = DISPID_SpeechGrammarRuleStateTransition(8i32);
impl ::core::marker::Copy for DISPID_SpeechGrammarRuleStateTransition {}
impl ::core::clone::Clone for DISPID_SpeechGrammarRuleStateTransition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechGrammarRuleStateTransition {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechGrammarRuleStateTransition {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechGrammarRuleStateTransition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechGrammarRuleStateTransition").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechGrammarRuleStateTransitions(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTsCount: DISPID_SpeechGrammarRuleStateTransitions = DISPID_SpeechGrammarRuleStateTransitions(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTsItem: DISPID_SpeechGrammarRuleStateTransitions = DISPID_SpeechGrammarRuleStateTransitions(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRSTs_NewEnum: DISPID_SpeechGrammarRuleStateTransitions = DISPID_SpeechGrammarRuleStateTransitions(-4i32);
impl ::core::marker::Copy for DISPID_SpeechGrammarRuleStateTransitions {}
impl ::core::clone::Clone for DISPID_SpeechGrammarRuleStateTransitions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechGrammarRuleStateTransitions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechGrammarRuleStateTransitions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechGrammarRuleStateTransitions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechGrammarRuleStateTransitions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechGrammarRules(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRsCount: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRsDynamic: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRsAdd: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRsCommit: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRsCommitAndSave: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRsFindRule: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRsItem: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SGRs_NewEnum: DISPID_SpeechGrammarRules = DISPID_SpeechGrammarRules(-4i32);
impl ::core::marker::Copy for DISPID_SpeechGrammarRules {}
impl ::core::clone::Clone for DISPID_SpeechGrammarRules {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechGrammarRules {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechGrammarRules {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechGrammarRules {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechGrammarRules").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechLexicon(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLGenerationId: DISPID_SpeechLexicon = DISPID_SpeechLexicon(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLGetWords: DISPID_SpeechLexicon = DISPID_SpeechLexicon(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLAddPronunciation: DISPID_SpeechLexicon = DISPID_SpeechLexicon(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLAddPronunciationByPhoneIds: DISPID_SpeechLexicon = DISPID_SpeechLexicon(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLRemovePronunciation: DISPID_SpeechLexicon = DISPID_SpeechLexicon(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLRemovePronunciationByPhoneIds: DISPID_SpeechLexicon = DISPID_SpeechLexicon(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLGetPronunciations: DISPID_SpeechLexicon = DISPID_SpeechLexicon(7i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLGetGenerationChange: DISPID_SpeechLexicon = DISPID_SpeechLexicon(8i32);
impl ::core::marker::Copy for DISPID_SpeechLexicon {}
impl ::core::clone::Clone for DISPID_SpeechLexicon {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechLexicon {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechLexicon {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechLexicon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechLexicon").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechLexiconProns(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLPsCount: DISPID_SpeechLexiconProns = DISPID_SpeechLexiconProns(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLPsItem: DISPID_SpeechLexiconProns = DISPID_SpeechLexiconProns(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLPs_NewEnum: DISPID_SpeechLexiconProns = DISPID_SpeechLexiconProns(-4i32);
impl ::core::marker::Copy for DISPID_SpeechLexiconProns {}
impl ::core::clone::Clone for DISPID_SpeechLexiconProns {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechLexiconProns {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechLexiconProns {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechLexiconProns {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechLexiconProns").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechLexiconPronunciation(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLPType: DISPID_SpeechLexiconPronunciation = DISPID_SpeechLexiconPronunciation(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLPLangId: DISPID_SpeechLexiconPronunciation = DISPID_SpeechLexiconPronunciation(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLPPartOfSpeech: DISPID_SpeechLexiconPronunciation = DISPID_SpeechLexiconPronunciation(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLPPhoneIds: DISPID_SpeechLexiconPronunciation = DISPID_SpeechLexiconPronunciation(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLPSymbolic: DISPID_SpeechLexiconPronunciation = DISPID_SpeechLexiconPronunciation(5i32);
impl ::core::marker::Copy for DISPID_SpeechLexiconPronunciation {}
impl ::core::clone::Clone for DISPID_SpeechLexiconPronunciation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechLexiconPronunciation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechLexiconPronunciation {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechLexiconPronunciation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechLexiconPronunciation").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechLexiconWord(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLWLangId: DISPID_SpeechLexiconWord = DISPID_SpeechLexiconWord(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLWType: DISPID_SpeechLexiconWord = DISPID_SpeechLexiconWord(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLWWord: DISPID_SpeechLexiconWord = DISPID_SpeechLexiconWord(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLWPronunciations: DISPID_SpeechLexiconWord = DISPID_SpeechLexiconWord(4i32);
impl ::core::marker::Copy for DISPID_SpeechLexiconWord {}
impl ::core::clone::Clone for DISPID_SpeechLexiconWord {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechLexiconWord {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechLexiconWord {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechLexiconWord {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechLexiconWord").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechLexiconWords(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLWsCount: DISPID_SpeechLexiconWords = DISPID_SpeechLexiconWords(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLWsItem: DISPID_SpeechLexiconWords = DISPID_SpeechLexiconWords(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SLWs_NewEnum: DISPID_SpeechLexiconWords = DISPID_SpeechLexiconWords(-4i32);
impl ::core::marker::Copy for DISPID_SpeechLexiconWords {}
impl ::core::clone::Clone for DISPID_SpeechLexiconWords {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechLexiconWords {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechLexiconWords {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechLexiconWords {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechLexiconWords").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechMMSysAudio(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SMSADeviceId: DISPID_SpeechMMSysAudio = DISPID_SpeechMMSysAudio(300i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SMSALineId: DISPID_SpeechMMSysAudio = DISPID_SpeechMMSysAudio(301i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SMSAMMHandle: DISPID_SpeechMMSysAudio = DISPID_SpeechMMSysAudio(302i32);
impl ::core::marker::Copy for DISPID_SpeechMMSysAudio {}
impl ::core::clone::Clone for DISPID_SpeechMMSysAudio {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechMMSysAudio {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechMMSysAudio {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechMMSysAudio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechMMSysAudio").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechMemoryStream(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SMSSetData: DISPID_SpeechMemoryStream = DISPID_SpeechMemoryStream(100i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SMSGetData: DISPID_SpeechMemoryStream = DISPID_SpeechMemoryStream(101i32);
impl ::core::marker::Copy for DISPID_SpeechMemoryStream {}
impl ::core::clone::Clone for DISPID_SpeechMemoryStream {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechMemoryStream {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechMemoryStream {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechMemoryStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechMemoryStream").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechObjectToken(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTId: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTDataKey: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTCategory: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTGetDescription: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTSetId: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTGetAttribute: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTCreateInstance: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(7i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTRemove: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTGetStorageFileName: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(9i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTRemoveStorageFileName: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(10i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTIsUISupported: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(11i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTDisplayUI: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(12i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTMatchesAttributes: DISPID_SpeechObjectToken = DISPID_SpeechObjectToken(13i32);
impl ::core::marker::Copy for DISPID_SpeechObjectToken {}
impl ::core::clone::Clone for DISPID_SpeechObjectToken {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechObjectToken {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechObjectToken {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechObjectToken {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechObjectToken").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechObjectTokenCategory(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTCId: DISPID_SpeechObjectTokenCategory = DISPID_SpeechObjectTokenCategory(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTCDefault: DISPID_SpeechObjectTokenCategory = DISPID_SpeechObjectTokenCategory(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTCSetId: DISPID_SpeechObjectTokenCategory = DISPID_SpeechObjectTokenCategory(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTCGetDataKey: DISPID_SpeechObjectTokenCategory = DISPID_SpeechObjectTokenCategory(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTCEnumerateTokens: DISPID_SpeechObjectTokenCategory = DISPID_SpeechObjectTokenCategory(5i32);
impl ::core::marker::Copy for DISPID_SpeechObjectTokenCategory {}
impl ::core::clone::Clone for DISPID_SpeechObjectTokenCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechObjectTokenCategory {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechObjectTokenCategory {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechObjectTokenCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechObjectTokenCategory").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechObjectTokens(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTsCount: DISPID_SpeechObjectTokens = DISPID_SpeechObjectTokens(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTsItem: DISPID_SpeechObjectTokens = DISPID_SpeechObjectTokens(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SOTs_NewEnum: DISPID_SpeechObjectTokens = DISPID_SpeechObjectTokens(-4i32);
impl ::core::marker::Copy for DISPID_SpeechObjectTokens {}
impl ::core::clone::Clone for DISPID_SpeechObjectTokens {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechObjectTokens {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechObjectTokens {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechObjectTokens {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechObjectTokens").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechPhoneConverter(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPCLangId: DISPID_SpeechPhoneConverter = DISPID_SpeechPhoneConverter(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPCPhoneToId: DISPID_SpeechPhoneConverter = DISPID_SpeechPhoneConverter(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPCIdToPhone: DISPID_SpeechPhoneConverter = DISPID_SpeechPhoneConverter(3i32);
impl ::core::marker::Copy for DISPID_SpeechPhoneConverter {}
impl ::core::clone::Clone for DISPID_SpeechPhoneConverter {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechPhoneConverter {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechPhoneConverter {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechPhoneConverter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhoneConverter").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechPhraseAlternate(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPARecoResult: DISPID_SpeechPhraseAlternate = DISPID_SpeechPhraseAlternate(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPAStartElementInResult: DISPID_SpeechPhraseAlternate = DISPID_SpeechPhraseAlternate(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPANumberOfElementsInResult: DISPID_SpeechPhraseAlternate = DISPID_SpeechPhraseAlternate(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPAPhraseInfo: DISPID_SpeechPhraseAlternate = DISPID_SpeechPhraseAlternate(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPACommit: DISPID_SpeechPhraseAlternate = DISPID_SpeechPhraseAlternate(5i32);
impl ::core::marker::Copy for DISPID_SpeechPhraseAlternate {}
impl ::core::clone::Clone for DISPID_SpeechPhraseAlternate {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechPhraseAlternate {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechPhraseAlternate {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechPhraseAlternate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseAlternate").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechPhraseAlternates(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPAsCount: DISPID_SpeechPhraseAlternates = DISPID_SpeechPhraseAlternates(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPAsItem: DISPID_SpeechPhraseAlternates = DISPID_SpeechPhraseAlternates(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPAs_NewEnum: DISPID_SpeechPhraseAlternates = DISPID_SpeechPhraseAlternates(-4i32);
impl ::core::marker::Copy for DISPID_SpeechPhraseAlternates {}
impl ::core::clone::Clone for DISPID_SpeechPhraseAlternates {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechPhraseAlternates {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechPhraseAlternates {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechPhraseAlternates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseAlternates").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechPhraseBuilder(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPBRestorePhraseFromMemory: DISPID_SpeechPhraseBuilder = DISPID_SpeechPhraseBuilder(1i32);
impl ::core::marker::Copy for DISPID_SpeechPhraseBuilder {}
impl ::core::clone::Clone for DISPID_SpeechPhraseBuilder {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechPhraseBuilder {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechPhraseBuilder {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechPhraseBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseBuilder").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechPhraseElement(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEAudioTimeOffset: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEAudioSizeTime: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEAudioStreamOffset: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEAudioSizeBytes: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPERetainedStreamOffset: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPERetainedSizeBytes: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEDisplayText: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(7i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPELexicalForm: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEPronunciation: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(9i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEDisplayAttributes: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(10i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPERequiredConfidence: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(11i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEActualConfidence: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(12i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEEngineConfidence: DISPID_SpeechPhraseElement = DISPID_SpeechPhraseElement(13i32);
impl ::core::marker::Copy for DISPID_SpeechPhraseElement {}
impl ::core::clone::Clone for DISPID_SpeechPhraseElement {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechPhraseElement {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechPhraseElement {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechPhraseElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseElement").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechPhraseElements(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEsCount: DISPID_SpeechPhraseElements = DISPID_SpeechPhraseElements(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEsItem: DISPID_SpeechPhraseElements = DISPID_SpeechPhraseElements(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPEs_NewEnum: DISPID_SpeechPhraseElements = DISPID_SpeechPhraseElements(-4i32);
impl ::core::marker::Copy for DISPID_SpeechPhraseElements {}
impl ::core::clone::Clone for DISPID_SpeechPhraseElements {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechPhraseElements {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechPhraseElements {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechPhraseElements {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseElements").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechPhraseInfo(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPILanguageId: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIGrammarId: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIStartTime: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIAudioStreamPosition: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIAudioSizeBytes: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIRetainedSizeBytes: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIAudioSizeTime: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(7i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIRule: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIProperties: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(9i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIElements: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(10i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIReplacements: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(11i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIEngineId: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(12i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIEnginePrivateData: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(13i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPISaveToMemory: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(14i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIGetText: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(15i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPIGetDisplayAttributes: DISPID_SpeechPhraseInfo = DISPID_SpeechPhraseInfo(16i32);
impl ::core::marker::Copy for DISPID_SpeechPhraseInfo {}
impl ::core::clone::Clone for DISPID_SpeechPhraseInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechPhraseInfo {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechPhraseInfo {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechPhraseInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseInfo").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechPhraseProperties(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPsCount: DISPID_SpeechPhraseProperties = DISPID_SpeechPhraseProperties(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPsItem: DISPID_SpeechPhraseProperties = DISPID_SpeechPhraseProperties(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPs_NewEnum: DISPID_SpeechPhraseProperties = DISPID_SpeechPhraseProperties(-4i32);
impl ::core::marker::Copy for DISPID_SpeechPhraseProperties {}
impl ::core::clone::Clone for DISPID_SpeechPhraseProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechPhraseProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechPhraseProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechPhraseProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseProperties").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechPhraseProperty(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPName: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPId: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPValue: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPFirstElement: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPNumberOfElements: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPEngineConfidence: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPConfidence: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(7i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPParent: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPPChildren: DISPID_SpeechPhraseProperty = DISPID_SpeechPhraseProperty(9i32);
impl ::core::marker::Copy for DISPID_SpeechPhraseProperty {}
impl ::core::clone::Clone for DISPID_SpeechPhraseProperty {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechPhraseProperty {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechPhraseProperty {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechPhraseProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseProperty").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechPhraseReplacement(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRDisplayAttributes: DISPID_SpeechPhraseReplacement = DISPID_SpeechPhraseReplacement(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRText: DISPID_SpeechPhraseReplacement = DISPID_SpeechPhraseReplacement(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRFirstElement: DISPID_SpeechPhraseReplacement = DISPID_SpeechPhraseReplacement(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRNumberOfElements: DISPID_SpeechPhraseReplacement = DISPID_SpeechPhraseReplacement(4i32);
impl ::core::marker::Copy for DISPID_SpeechPhraseReplacement {}
impl ::core::clone::Clone for DISPID_SpeechPhraseReplacement {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechPhraseReplacement {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechPhraseReplacement {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechPhraseReplacement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseReplacement").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechPhraseReplacements(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRsCount: DISPID_SpeechPhraseReplacements = DISPID_SpeechPhraseReplacements(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRsItem: DISPID_SpeechPhraseReplacements = DISPID_SpeechPhraseReplacements(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRs_NewEnum: DISPID_SpeechPhraseReplacements = DISPID_SpeechPhraseReplacements(-4i32);
impl ::core::marker::Copy for DISPID_SpeechPhraseReplacements {}
impl ::core::clone::Clone for DISPID_SpeechPhraseReplacements {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechPhraseReplacements {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechPhraseReplacements {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechPhraseReplacements {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseReplacements").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechPhraseRule(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRuleName: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRuleId: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRuleFirstElement: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRuleNumberOfElements: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRuleParent: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRuleChildren: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRuleConfidence: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(7i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRuleEngineConfidence: DISPID_SpeechPhraseRule = DISPID_SpeechPhraseRule(8i32);
impl ::core::marker::Copy for DISPID_SpeechPhraseRule {}
impl ::core::clone::Clone for DISPID_SpeechPhraseRule {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechPhraseRule {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechPhraseRule {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechPhraseRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseRule").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechPhraseRules(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRulesCount: DISPID_SpeechPhraseRules = DISPID_SpeechPhraseRules(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRulesItem: DISPID_SpeechPhraseRules = DISPID_SpeechPhraseRules(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SPRules_NewEnum: DISPID_SpeechPhraseRules = DISPID_SpeechPhraseRules(-4i32);
impl ::core::marker::Copy for DISPID_SpeechPhraseRules {}
impl ::core::clone::Clone for DISPID_SpeechPhraseRules {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechPhraseRules {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechPhraseRules {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechPhraseRules {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechPhraseRules").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechRecoContext(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCRecognizer: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCAudioInInterferenceStatus: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCRequestedUIType: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCVoice: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRAllowVoiceFormatMatchingOnNextSet: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCVoicePurgeEvent: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEventInterests: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(7i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCCmdMaxAlternates: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCState: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(9i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCRetainedAudio: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(10i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCRetainedAudioFormat: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(11i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCPause: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(12i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCResume: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(13i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCCreateGrammar: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(14i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCCreateResultFromMemory: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(15i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCBookmark: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(16i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCSetAdaptationData: DISPID_SpeechRecoContext = DISPID_SpeechRecoContext(17i32);
impl ::core::marker::Copy for DISPID_SpeechRecoContext {}
impl ::core::clone::Clone for DISPID_SpeechRecoContext {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechRecoContext {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechRecoContext {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechRecoContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechRecoContext").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechRecoContextEvents(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEStartStream: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEEndStream: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEBookmark: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCESoundStart: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCESoundEnd: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEPhraseStart: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCERecognition: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(7i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEHypothesis: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEPropertyNumberChange: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(9i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEPropertyStringChange: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(10i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEFalseRecognition: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(11i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEInterference: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(12i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCERequestUI: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(13i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCERecognizerStateChange: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(14i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEAdaptation: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(15i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCERecognitionForOtherContext: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(16i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEAudioLevel: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(17i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCEEnginePrivate: DISPID_SpeechRecoContextEvents = DISPID_SpeechRecoContextEvents(18i32);
impl ::core::marker::Copy for DISPID_SpeechRecoContextEvents {}
impl ::core::clone::Clone for DISPID_SpeechRecoContextEvents {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechRecoContextEvents {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechRecoContextEvents {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechRecoContextEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechRecoContextEvents").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechRecoResult(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRRecoContext: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRTimes: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRAudioFormat: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRPhraseInfo: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRAlternates: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRAudio: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRSpeakAudio: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(7i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRSaveToMemory: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRDiscardResultInfo: DISPID_SpeechRecoResult = DISPID_SpeechRecoResult(9i32);
impl ::core::marker::Copy for DISPID_SpeechRecoResult {}
impl ::core::clone::Clone for DISPID_SpeechRecoResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechRecoResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechRecoResult {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechRecoResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechRecoResult").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechRecoResult2(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRSetTextFeedback: DISPID_SpeechRecoResult2 = DISPID_SpeechRecoResult2(12i32);
impl ::core::marker::Copy for DISPID_SpeechRecoResult2 {}
impl ::core::clone::Clone for DISPID_SpeechRecoResult2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechRecoResult2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechRecoResult2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechRecoResult2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechRecoResult2").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechRecoResultTimes(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRTStreamTime: DISPID_SpeechRecoResultTimes = DISPID_SpeechRecoResultTimes(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRTLength: DISPID_SpeechRecoResultTimes = DISPID_SpeechRecoResultTimes(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRTTickCount: DISPID_SpeechRecoResultTimes = DISPID_SpeechRecoResultTimes(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRTOffsetFromStart: DISPID_SpeechRecoResultTimes = DISPID_SpeechRecoResultTimes(4i32);
impl ::core::marker::Copy for DISPID_SpeechRecoResultTimes {}
impl ::core::clone::Clone for DISPID_SpeechRecoResultTimes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechRecoResultTimes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechRecoResultTimes {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechRecoResultTimes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechRecoResultTimes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechRecognizer(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRecognizer: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRAllowAudioInputFormatChangesOnNextSet: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRAudioInput: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRAudioInputStream: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRIsShared: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRState: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRStatus: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(7i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRProfile: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SREmulateRecognition: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(9i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRCreateRecoContext: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(10i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGetFormat: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(11i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRSetPropertyNumber: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(12i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGetPropertyNumber: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(13i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRSetPropertyString: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(14i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGetPropertyString: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(15i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRIsUISupported: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(16i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRDisplayUI: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(17i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRGetRecognizers: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(18i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVGetAudioInputs: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(19i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVGetProfiles: DISPID_SpeechRecognizer = DISPID_SpeechRecognizer(20i32);
impl ::core::marker::Copy for DISPID_SpeechRecognizer {}
impl ::core::clone::Clone for DISPID_SpeechRecognizer {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechRecognizer {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechRecognizer {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechRecognizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechRecognizer").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechRecognizerStatus(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRSAudioStatus: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRSCurrentStreamPosition: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRSCurrentStreamNumber: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRSNumberOfActiveRules: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRSClsidEngine: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRSSupportedLanguages: DISPID_SpeechRecognizerStatus = DISPID_SpeechRecognizerStatus(6i32);
impl ::core::marker::Copy for DISPID_SpeechRecognizerStatus {}
impl ::core::clone::Clone for DISPID_SpeechRecognizerStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechRecognizerStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechRecognizerStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechRecognizerStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechRecognizerStatus").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechVoice(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVStatus: DISPID_SpeechVoice = DISPID_SpeechVoice(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVVoice: DISPID_SpeechVoice = DISPID_SpeechVoice(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVAudioOutput: DISPID_SpeechVoice = DISPID_SpeechVoice(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVAudioOutputStream: DISPID_SpeechVoice = DISPID_SpeechVoice(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVRate: DISPID_SpeechVoice = DISPID_SpeechVoice(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVVolume: DISPID_SpeechVoice = DISPID_SpeechVoice(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVAllowAudioOuputFormatChangesOnNextSet: DISPID_SpeechVoice = DISPID_SpeechVoice(7i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVEventInterests: DISPID_SpeechVoice = DISPID_SpeechVoice(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVPriority: DISPID_SpeechVoice = DISPID_SpeechVoice(9i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVAlertBoundary: DISPID_SpeechVoice = DISPID_SpeechVoice(10i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSyncronousSpeakTimeout: DISPID_SpeechVoice = DISPID_SpeechVoice(11i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSpeak: DISPID_SpeechVoice = DISPID_SpeechVoice(12i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSpeakStream: DISPID_SpeechVoice = DISPID_SpeechVoice(13i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVPause: DISPID_SpeechVoice = DISPID_SpeechVoice(14i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVResume: DISPID_SpeechVoice = DISPID_SpeechVoice(15i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSkip: DISPID_SpeechVoice = DISPID_SpeechVoice(16i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVGetVoices: DISPID_SpeechVoice = DISPID_SpeechVoice(17i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVGetAudioOutputs: DISPID_SpeechVoice = DISPID_SpeechVoice(18i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVWaitUntilDone: DISPID_SpeechVoice = DISPID_SpeechVoice(19i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSpeakCompleteEvent: DISPID_SpeechVoice = DISPID_SpeechVoice(20i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVIsUISupported: DISPID_SpeechVoice = DISPID_SpeechVoice(21i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVDisplayUI: DISPID_SpeechVoice = DISPID_SpeechVoice(22i32);
impl ::core::marker::Copy for DISPID_SpeechVoice {}
impl ::core::clone::Clone for DISPID_SpeechVoice {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechVoice {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechVoice {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechVoice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechVoice").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechVoiceEvent(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVEStreamStart: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVEStreamEnd: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVEVoiceChange: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVEBookmark: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVEWord: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVEPhoneme: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVESentenceBoundary: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(7i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVEViseme: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVEAudioLevel: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(9i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVEEnginePrivate: DISPID_SpeechVoiceEvent = DISPID_SpeechVoiceEvent(10i32);
impl ::core::marker::Copy for DISPID_SpeechVoiceEvent {}
impl ::core::clone::Clone for DISPID_SpeechVoiceEvent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechVoiceEvent {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechVoiceEvent {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechVoiceEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechVoiceEvent").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechVoiceStatus(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSCurrentStreamNumber: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSLastStreamNumberQueued: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSLastResult: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSRunningState: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSInputWordPosition: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSInputWordLength: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSInputSentencePosition: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(7i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSInputSentenceLength: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSLastBookmark: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(9i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSLastBookmarkId: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(10i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSPhonemeId: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(11i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SVSVisemeId: DISPID_SpeechVoiceStatus = DISPID_SpeechVoiceStatus(12i32);
impl ::core::marker::Copy for DISPID_SpeechVoiceStatus {}
impl ::core::clone::Clone for DISPID_SpeechVoiceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechVoiceStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechVoiceStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechVoiceStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechVoiceStatus").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechWaveFormatEx(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SWFEFormatTag: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SWFEChannels: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SWFESamplesPerSec: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SWFEAvgBytesPerSec: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SWFEBlockAlign: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SWFEBitsPerSample: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SWFEExtraData: DISPID_SpeechWaveFormatEx = DISPID_SpeechWaveFormatEx(7i32);
impl ::core::marker::Copy for DISPID_SpeechWaveFormatEx {}
impl ::core::clone::Clone for DISPID_SpeechWaveFormatEx {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechWaveFormatEx {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechWaveFormatEx {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechWaveFormatEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechWaveFormatEx").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISPID_SpeechXMLRecoResult(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRGetXMLResult: DISPID_SpeechXMLRecoResult = DISPID_SpeechXMLRecoResult(10i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const DISPID_SRRGetXMLErrorInfo: DISPID_SpeechXMLRecoResult = DISPID_SpeechXMLRecoResult(11i32);
impl ::core::marker::Copy for DISPID_SpeechXMLRecoResult {}
impl ::core::clone::Clone for DISPID_SpeechXMLRecoResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISPID_SpeechXMLRecoResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DISPID_SpeechXMLRecoResult {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DISPID_SpeechXMLRecoResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPID_SpeechXMLRecoResult").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PHONETICALPHABET(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const PA_Ipa: PHONETICALPHABET = PHONETICALPHABET(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const PA_Ups: PHONETICALPHABET = PHONETICALPHABET(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const PA_Sapi: PHONETICALPHABET = PHONETICALPHABET(2i32);
impl ::core::marker::Copy for PHONETICALPHABET {}
impl ::core::clone::Clone for PHONETICALPHABET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PHONETICALPHABET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PHONETICALPHABET {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PHONETICALPHABET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PHONETICALPHABET").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPADAPTATIONRELEVANCE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAR_Unknown: SPADAPTATIONRELEVANCE = SPADAPTATIONRELEVANCE(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAR_Low: SPADAPTATIONRELEVANCE = SPADAPTATIONRELEVANCE(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAR_Medium: SPADAPTATIONRELEVANCE = SPADAPTATIONRELEVANCE(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAR_High: SPADAPTATIONRELEVANCE = SPADAPTATIONRELEVANCE(3i32);
impl ::core::marker::Copy for SPADAPTATIONRELEVANCE {}
impl ::core::clone::Clone for SPADAPTATIONRELEVANCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPADAPTATIONRELEVANCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPADAPTATIONRELEVANCE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPADAPTATIONRELEVANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPADAPTATIONRELEVANCE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPADAPTATIONSETTINGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPADS_Default: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPADS_CurrentRecognizer: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPADS_RecoProfile: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPADS_Immediate: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPADS_Reset: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPADS_HighVolumeDataSource: SPADAPTATIONSETTINGS = SPADAPTATIONSETTINGS(16i32);
impl ::core::marker::Copy for SPADAPTATIONSETTINGS {}
impl ::core::clone::Clone for SPADAPTATIONSETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPADAPTATIONSETTINGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPADAPTATIONSETTINGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPADAPTATIONSETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPADAPTATIONSETTINGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPAUDIOOPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAO_NONE: SPAUDIOOPTIONS = SPAUDIOOPTIONS(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAO_RETAIN_AUDIO: SPAUDIOOPTIONS = SPAUDIOOPTIONS(1i32);
impl ::core::marker::Copy for SPAUDIOOPTIONS {}
impl ::core::clone::Clone for SPAUDIOOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPAUDIOOPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPAUDIOOPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPAUDIOOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPAUDIOOPTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPAUDIOSTATE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAS_CLOSED: SPAUDIOSTATE = SPAUDIOSTATE(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAS_STOP: SPAUDIOSTATE = SPAUDIOSTATE(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAS_PAUSE: SPAUDIOSTATE = SPAUDIOSTATE(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAS_RUN: SPAUDIOSTATE = SPAUDIOSTATE(3i32);
impl ::core::marker::Copy for SPAUDIOSTATE {}
impl ::core::clone::Clone for SPAUDIOSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPAUDIOSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPAUDIOSTATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPAUDIOSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPAUDIOSTATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPBOOKMARKOPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPBO_NONE: SPBOOKMARKOPTIONS = SPBOOKMARKOPTIONS(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPBO_PAUSE: SPBOOKMARKOPTIONS = SPBOOKMARKOPTIONS(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPBO_AHEAD: SPBOOKMARKOPTIONS = SPBOOKMARKOPTIONS(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPBO_TIME_UNITS: SPBOOKMARKOPTIONS = SPBOOKMARKOPTIONS(4i32);
impl ::core::marker::Copy for SPBOOKMARKOPTIONS {}
impl ::core::clone::Clone for SPBOOKMARKOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPBOOKMARKOPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPBOOKMARKOPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPBOOKMARKOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPBOOKMARKOPTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPCFGRULEATTRIBUTES(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRAF_TopLevel: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRAF_Active: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRAF_Export: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRAF_Import: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRAF_Interpreter: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(16i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRAF_Dynamic: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(32i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRAF_Root: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(64i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRAF_AutoPause: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(65536i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRAF_UserDelimited: SPCFGRULEATTRIBUTES = SPCFGRULEATTRIBUTES(131072i32);
impl ::core::marker::Copy for SPCFGRULEATTRIBUTES {}
impl ::core::clone::Clone for SPCFGRULEATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPCFGRULEATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPCFGRULEATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPCFGRULEATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPCFGRULEATTRIBUTES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPCOMMITFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCF_NONE: SPCOMMITFLAGS = SPCOMMITFLAGS(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCF_ADD_TO_USER_LEXICON: SPCOMMITFLAGS = SPCOMMITFLAGS(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCF_DEFINITE_CORRECTION: SPCOMMITFLAGS = SPCOMMITFLAGS(2i32);
impl ::core::marker::Copy for SPCOMMITFLAGS {}
impl ::core::clone::Clone for SPCOMMITFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPCOMMITFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPCOMMITFLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPCOMMITFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPCOMMITFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPCONTEXTSTATE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCS_DISABLED: SPCONTEXTSTATE = SPCONTEXTSTATE(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPCS_ENABLED: SPCONTEXTSTATE = SPCONTEXTSTATE(1i32);
impl ::core::marker::Copy for SPCONTEXTSTATE {}
impl ::core::clone::Clone for SPCONTEXTSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPCONTEXTSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPCONTEXTSTATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPCONTEXTSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPCONTEXTSTATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPDATAKEYLOCATION(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDKL_DefaultLocation: SPDATAKEYLOCATION = SPDATAKEYLOCATION(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDKL_CurrentUser: SPDATAKEYLOCATION = SPDATAKEYLOCATION(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDKL_LocalMachine: SPDATAKEYLOCATION = SPDATAKEYLOCATION(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDKL_CurrentConfig: SPDATAKEYLOCATION = SPDATAKEYLOCATION(5i32);
impl ::core::marker::Copy for SPDATAKEYLOCATION {}
impl ::core::clone::Clone for SPDATAKEYLOCATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPDATAKEYLOCATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPDATAKEYLOCATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPDATAKEYLOCATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPDATAKEYLOCATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPDISPLAYATTRIBUTES(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAF_ONE_TRAILING_SPACE: SPDISPLAYATTRIBUTES = SPDISPLAYATTRIBUTES(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAF_TWO_TRAILING_SPACES: SPDISPLAYATTRIBUTES = SPDISPLAYATTRIBUTES(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAF_CONSUME_LEADING_SPACES: SPDISPLAYATTRIBUTES = SPDISPLAYATTRIBUTES(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAF_BUFFER_POSITION: SPDISPLAYATTRIBUTES = SPDISPLAYATTRIBUTES(16i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAF_ALL: SPDISPLAYATTRIBUTES = SPDISPLAYATTRIBUTES(31i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPAF_USER_SPECIFIED: SPDISPLAYATTRIBUTES = SPDISPLAYATTRIBUTES(128i32);
impl ::core::marker::Copy for SPDISPLAYATTRIBUTES {}
impl ::core::clone::Clone for SPDISPLAYATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPDISPLAYATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPDISPLAYATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPDISPLAYATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPDISPLAYATTRIBUTES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPEAKFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_DEFAULT: SPEAKFLAGS = SPEAKFLAGS(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_ASYNC: SPEAKFLAGS = SPEAKFLAGS(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_PURGEBEFORESPEAK: SPEAKFLAGS = SPEAKFLAGS(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_IS_FILENAME: SPEAKFLAGS = SPEAKFLAGS(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_IS_XML: SPEAKFLAGS = SPEAKFLAGS(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_IS_NOT_XML: SPEAKFLAGS = SPEAKFLAGS(16i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_PERSIST_XML: SPEAKFLAGS = SPEAKFLAGS(32i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_NLP_SPEAK_PUNC: SPEAKFLAGS = SPEAKFLAGS(64i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_PARSE_SAPI: SPEAKFLAGS = SPEAKFLAGS(128i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_PARSE_SSML: SPEAKFLAGS = SPEAKFLAGS(256i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_PARSE_AUTODETECT: SPEAKFLAGS = SPEAKFLAGS(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_NLP_MASK: SPEAKFLAGS = SPEAKFLAGS(64i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_PARSE_MASK: SPEAKFLAGS = SPEAKFLAGS(384i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_VOICE_MASK: SPEAKFLAGS = SPEAKFLAGS(511i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPF_UNUSED_FLAGS: SPEAKFLAGS = SPEAKFLAGS(-512i32);
impl ::core::marker::Copy for SPEAKFLAGS {}
impl ::core::clone::Clone for SPEAKFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPEAKFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPEAKFLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPEAKFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPEAKFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPENDSRSTREAMFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPESF_NONE: SPENDSRSTREAMFLAGS = SPENDSRSTREAMFLAGS(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPESF_STREAM_RELEASED: SPENDSRSTREAMFLAGS = SPENDSRSTREAMFLAGS(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPESF_EMULATED: SPENDSRSTREAMFLAGS = SPENDSRSTREAMFLAGS(2i32);
impl ::core::marker::Copy for SPENDSRSTREAMFLAGS {}
impl ::core::clone::Clone for SPENDSRSTREAMFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPENDSRSTREAMFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPENDSRSTREAMFLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPENDSRSTREAMFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPENDSRSTREAMFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPEVENTENUM(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_UNDEFINED: SPEVENTENUM = SPEVENTENUM(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_START_INPUT_STREAM: SPEVENTENUM = SPEVENTENUM(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_END_INPUT_STREAM: SPEVENTENUM = SPEVENTENUM(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_VOICE_CHANGE: SPEVENTENUM = SPEVENTENUM(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_TTS_BOOKMARK: SPEVENTENUM = SPEVENTENUM(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_WORD_BOUNDARY: SPEVENTENUM = SPEVENTENUM(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_PHONEME: SPEVENTENUM = SPEVENTENUM(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_SENTENCE_BOUNDARY: SPEVENTENUM = SPEVENTENUM(7i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_VISEME: SPEVENTENUM = SPEVENTENUM(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_TTS_AUDIO_LEVEL: SPEVENTENUM = SPEVENTENUM(9i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_TTS_PRIVATE: SPEVENTENUM = SPEVENTENUM(15i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_MIN_TTS: SPEVENTENUM = SPEVENTENUM(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_MAX_TTS: SPEVENTENUM = SPEVENTENUM(15i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_END_SR_STREAM: SPEVENTENUM = SPEVENTENUM(34i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_SOUND_START: SPEVENTENUM = SPEVENTENUM(35i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_SOUND_END: SPEVENTENUM = SPEVENTENUM(36i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_PHRASE_START: SPEVENTENUM = SPEVENTENUM(37i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_RECOGNITION: SPEVENTENUM = SPEVENTENUM(38i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_HYPOTHESIS: SPEVENTENUM = SPEVENTENUM(39i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_SR_BOOKMARK: SPEVENTENUM = SPEVENTENUM(40i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_PROPERTY_NUM_CHANGE: SPEVENTENUM = SPEVENTENUM(41i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_PROPERTY_STRING_CHANGE: SPEVENTENUM = SPEVENTENUM(42i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_FALSE_RECOGNITION: SPEVENTENUM = SPEVENTENUM(43i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_INTERFERENCE: SPEVENTENUM = SPEVENTENUM(44i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_REQUEST_UI: SPEVENTENUM = SPEVENTENUM(45i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_RECO_STATE_CHANGE: SPEVENTENUM = SPEVENTENUM(46i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_ADAPTATION: SPEVENTENUM = SPEVENTENUM(47i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_START_SR_STREAM: SPEVENTENUM = SPEVENTENUM(48i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_RECO_OTHER_CONTEXT: SPEVENTENUM = SPEVENTENUM(49i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_SR_AUDIO_LEVEL: SPEVENTENUM = SPEVENTENUM(50i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_SR_RETAINEDAUDIO: SPEVENTENUM = SPEVENTENUM(51i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_SR_PRIVATE: SPEVENTENUM = SPEVENTENUM(52i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_RESERVED4: SPEVENTENUM = SPEVENTENUM(53i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_RESERVED5: SPEVENTENUM = SPEVENTENUM(54i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_RESERVED6: SPEVENTENUM = SPEVENTENUM(55i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_MIN_SR: SPEVENTENUM = SPEVENTENUM(34i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_MAX_SR: SPEVENTENUM = SPEVENTENUM(55i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_RESERVED1: SPEVENTENUM = SPEVENTENUM(30i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_RESERVED2: SPEVENTENUM = SPEVENTENUM(33i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPEI_RESERVED3: SPEVENTENUM = SPEVENTENUM(63i32);
impl ::core::marker::Copy for SPEVENTENUM {}
impl ::core::clone::Clone for SPEVENTENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPEVENTENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPEVENTENUM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPEVENTENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPEVENTENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPEVENTLPARAMTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPET_LPARAM_IS_UNDEFINED: SPEVENTLPARAMTYPE = SPEVENTLPARAMTYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPET_LPARAM_IS_TOKEN: SPEVENTLPARAMTYPE = SPEVENTLPARAMTYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPET_LPARAM_IS_OBJECT: SPEVENTLPARAMTYPE = SPEVENTLPARAMTYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPET_LPARAM_IS_POINTER: SPEVENTLPARAMTYPE = SPEVENTLPARAMTYPE(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPET_LPARAM_IS_STRING: SPEVENTLPARAMTYPE = SPEVENTLPARAMTYPE(4i32);
impl ::core::marker::Copy for SPEVENTLPARAMTYPE {}
impl ::core::clone::Clone for SPEVENTLPARAMTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPEVENTLPARAMTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPEVENTLPARAMTYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPEVENTLPARAMTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPEVENTLPARAMTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPFILEMODE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPFM_OPEN_READONLY: SPFILEMODE = SPFILEMODE(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPFM_OPEN_READWRITE: SPFILEMODE = SPFILEMODE(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPFM_CREATE: SPFILEMODE = SPFILEMODE(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPFM_CREATE_ALWAYS: SPFILEMODE = SPFILEMODE(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPFM_NUM_MODES: SPFILEMODE = SPFILEMODE(4i32);
impl ::core::marker::Copy for SPFILEMODE {}
impl ::core::clone::Clone for SPFILEMODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPFILEMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPFILEMODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPFILEMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPFILEMODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPGRAMMAROPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_SAPI: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_SRGS: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_UPS: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_SRGS_MS_SCRIPT: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_SRGS_W3C_SCRIPT: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(256i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_SRGS_STG_SCRIPT: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(512i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_SRGS_SCRIPT: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(778i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_FILE: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(16i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_HTTP: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(32i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_RES: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(64i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_OBJECT: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(128i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_DEFAULT: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(1019i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGO_ALL: SPGRAMMAROPTIONS = SPGRAMMAROPTIONS(1023i32);
impl ::core::marker::Copy for SPGRAMMAROPTIONS {}
impl ::core::clone::Clone for SPGRAMMAROPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPGRAMMAROPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPGRAMMAROPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPGRAMMAROPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPGRAMMAROPTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPGRAMMARSTATE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGS_DISABLED: SPGRAMMARSTATE = SPGRAMMARSTATE(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGS_ENABLED: SPGRAMMARSTATE = SPGRAMMARSTATE(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPGS_EXCLUSIVE: SPGRAMMARSTATE = SPGRAMMARSTATE(3i32);
impl ::core::marker::Copy for SPGRAMMARSTATE {}
impl ::core::clone::Clone for SPGRAMMARSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPGRAMMARSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPGRAMMARSTATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPGRAMMARSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPGRAMMARSTATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPGRAMMARWORDTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPWT_DISPLAY: SPGRAMMARWORDTYPE = SPGRAMMARWORDTYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPWT_LEXICAL: SPGRAMMARWORDTYPE = SPGRAMMARWORDTYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPWT_PRONUNCIATION: SPGRAMMARWORDTYPE = SPGRAMMARWORDTYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPWT_LEXICAL_NO_SPECIAL_CHARS: SPGRAMMARWORDTYPE = SPGRAMMARWORDTYPE(3i32);
impl ::core::marker::Copy for SPGRAMMARWORDTYPE {}
impl ::core::clone::Clone for SPGRAMMARWORDTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPGRAMMARWORDTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPGRAMMARWORDTYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPGRAMMARWORDTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPGRAMMARWORDTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPINTERFERENCE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPINTERFERENCE_NONE: SPINTERFERENCE = SPINTERFERENCE(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPINTERFERENCE_NOISE: SPINTERFERENCE = SPINTERFERENCE(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPINTERFERENCE_NOSIGNAL: SPINTERFERENCE = SPINTERFERENCE(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPINTERFERENCE_TOOLOUD: SPINTERFERENCE = SPINTERFERENCE(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPINTERFERENCE_TOOQUIET: SPINTERFERENCE = SPINTERFERENCE(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPINTERFERENCE_TOOFAST: SPINTERFERENCE = SPINTERFERENCE(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPINTERFERENCE_TOOSLOW: SPINTERFERENCE = SPINTERFERENCE(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPINTERFERENCE_LATENCY_WARNING: SPINTERFERENCE = SPINTERFERENCE(7i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPINTERFERENCE_LATENCY_TRUNCATE_BEGIN: SPINTERFERENCE = SPINTERFERENCE(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPINTERFERENCE_LATENCY_TRUNCATE_END: SPINTERFERENCE = SPINTERFERENCE(9i32);
impl ::core::marker::Copy for SPINTERFERENCE {}
impl ::core::clone::Clone for SPINTERFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPINTERFERENCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPINTERFERENCE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPINTERFERENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPINTERFERENCE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPLEXICONTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_USER: SPLEXICONTYPE = SPLEXICONTYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_APP: SPLEXICONTYPE = SPLEXICONTYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_VENDORLEXICON: SPLEXICONTYPE = SPLEXICONTYPE(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_LETTERTOSOUND: SPLEXICONTYPE = SPLEXICONTYPE(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_MORPHOLOGY: SPLEXICONTYPE = SPLEXICONTYPE(16i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_RESERVED4: SPLEXICONTYPE = SPLEXICONTYPE(32i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_USER_SHORTCUT: SPLEXICONTYPE = SPLEXICONTYPE(64i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_RESERVED6: SPLEXICONTYPE = SPLEXICONTYPE(128i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_RESERVED7: SPLEXICONTYPE = SPLEXICONTYPE(256i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_RESERVED8: SPLEXICONTYPE = SPLEXICONTYPE(512i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_RESERVED9: SPLEXICONTYPE = SPLEXICONTYPE(1024i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_RESERVED10: SPLEXICONTYPE = SPLEXICONTYPE(2048i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE1: SPLEXICONTYPE = SPLEXICONTYPE(4096i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE2: SPLEXICONTYPE = SPLEXICONTYPE(8192i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE3: SPLEXICONTYPE = SPLEXICONTYPE(16384i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE4: SPLEXICONTYPE = SPLEXICONTYPE(32768i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE5: SPLEXICONTYPE = SPLEXICONTYPE(65536i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE6: SPLEXICONTYPE = SPLEXICONTYPE(131072i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE7: SPLEXICONTYPE = SPLEXICONTYPE(262144i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE8: SPLEXICONTYPE = SPLEXICONTYPE(524288i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE9: SPLEXICONTYPE = SPLEXICONTYPE(1048576i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE10: SPLEXICONTYPE = SPLEXICONTYPE(2097152i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE11: SPLEXICONTYPE = SPLEXICONTYPE(4194304i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE12: SPLEXICONTYPE = SPLEXICONTYPE(8388608i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE13: SPLEXICONTYPE = SPLEXICONTYPE(16777216i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE14: SPLEXICONTYPE = SPLEXICONTYPE(33554432i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE15: SPLEXICONTYPE = SPLEXICONTYPE(67108864i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE16: SPLEXICONTYPE = SPLEXICONTYPE(134217728i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE17: SPLEXICONTYPE = SPLEXICONTYPE(268435456i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE18: SPLEXICONTYPE = SPLEXICONTYPE(536870912i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE19: SPLEXICONTYPE = SPLEXICONTYPE(1073741824i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eLEXTYPE_PRIVATE20: SPLEXICONTYPE = SPLEXICONTYPE(-2147483648i32);
impl ::core::marker::Copy for SPLEXICONTYPE {}
impl ::core::clone::Clone for SPLEXICONTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPLEXICONTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPLEXICONTYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPLEXICONTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPLEXICONTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPLOADOPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPLO_STATIC: SPLOADOPTIONS = SPLOADOPTIONS(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPLO_DYNAMIC: SPLOADOPTIONS = SPLOADOPTIONS(1i32);
impl ::core::marker::Copy for SPLOADOPTIONS {}
impl ::core::clone::Clone for SPLOADOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPLOADOPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPLOADOPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPLOADOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPLOADOPTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPMATCHINGMODE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const AllWords: SPMATCHINGMODE = SPMATCHINGMODE(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const Subsequence: SPMATCHINGMODE = SPMATCHINGMODE(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const OrderedSubset: SPMATCHINGMODE = SPMATCHINGMODE(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SubsequenceContentRequired: SPMATCHINGMODE = SPMATCHINGMODE(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const OrderedSubsetContentRequired: SPMATCHINGMODE = SPMATCHINGMODE(7i32);
impl ::core::marker::Copy for SPMATCHINGMODE {}
impl ::core::clone::Clone for SPMATCHINGMODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPMATCHINGMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPMATCHINGMODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPMATCHINGMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPMATCHINGMODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPPARTOFSPEECH(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_NotOverriden: SPPARTOFSPEECH = SPPARTOFSPEECH(-1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_Unknown: SPPARTOFSPEECH = SPPARTOFSPEECH(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_Noun: SPPARTOFSPEECH = SPPARTOFSPEECH(4096i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_Verb: SPPARTOFSPEECH = SPPARTOFSPEECH(8192i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_Modifier: SPPARTOFSPEECH = SPPARTOFSPEECH(12288i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_Function: SPPARTOFSPEECH = SPPARTOFSPEECH(16384i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_Interjection: SPPARTOFSPEECH = SPPARTOFSPEECH(20480i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_Noncontent: SPPARTOFSPEECH = SPPARTOFSPEECH(24576i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_LMA: SPPARTOFSPEECH = SPPARTOFSPEECH(28672i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_SuppressWord: SPPARTOFSPEECH = SPPARTOFSPEECH(61440i32);
impl ::core::marker::Copy for SPPARTOFSPEECH {}
impl ::core::clone::Clone for SPPARTOFSPEECH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPPARTOFSPEECH {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPPARTOFSPEECH {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPPARTOFSPEECH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPPARTOFSPEECH").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPPHRASEPROPERTYUNIONTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPPUT_UNUSED: SPPHRASEPROPERTYUNIONTYPE = SPPHRASEPROPERTYUNIONTYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPPUT_ARRAY_INDEX: SPPHRASEPROPERTYUNIONTYPE = SPPHRASEPROPERTYUNIONTYPE(1i32);
impl ::core::marker::Copy for SPPHRASEPROPERTYUNIONTYPE {}
impl ::core::clone::Clone for SPPHRASEPROPERTYUNIONTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPPHRASEPROPERTYUNIONTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPPHRASEPROPERTYUNIONTYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPPHRASEPROPERTYUNIONTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPPHRASEPROPERTYUNIONTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPPHRASERNG(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPR_ALL_ELEMENTS: SPPHRASERNG = SPPHRASERNG(-1i32);
impl ::core::marker::Copy for SPPHRASERNG {}
impl ::core::clone::Clone for SPPHRASERNG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPPHRASERNG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPPHRASERNG {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPPHRASERNG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPPHRASERNG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPPRONUNCIATIONFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const ePRONFLAG_USED: SPPRONUNCIATIONFLAGS = SPPRONUNCIATIONFLAGS(1i32);
impl ::core::marker::Copy for SPPRONUNCIATIONFLAGS {}
impl ::core::clone::Clone for SPPRONUNCIATIONFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPPRONUNCIATIONFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPPRONUNCIATIONFLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPPRONUNCIATIONFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPPRONUNCIATIONFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPRECOEVENTFLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPREF_AutoPause: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPREF_Emulated: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPREF_SMLTimeout: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPREF_ExtendableParse: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPREF_ReSent: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(16i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPREF_Hypothesis: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(32i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPREF_FalseRecognition: SPRECOEVENTFLAGS = SPRECOEVENTFLAGS(64i32);
impl ::core::marker::Copy for SPRECOEVENTFLAGS {}
impl ::core::clone::Clone for SPRECOEVENTFLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPRECOEVENTFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPRECOEVENTFLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPRECOEVENTFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPRECOEVENTFLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPRECOSTATE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRST_INACTIVE: SPRECOSTATE = SPRECOSTATE(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRST_ACTIVE: SPRECOSTATE = SPRECOSTATE(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRST_ACTIVE_ALWAYS: SPRECOSTATE = SPRECOSTATE(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRST_INACTIVE_WITH_PURGE: SPRECOSTATE = SPRECOSTATE(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRST_NUM_STATES: SPRECOSTATE = SPRECOSTATE(4i32);
impl ::core::marker::Copy for SPRECOSTATE {}
impl ::core::clone::Clone for SPRECOSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPRECOSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPRECOSTATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPRECOSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPRECOSTATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPRULESTATE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRS_INACTIVE: SPRULESTATE = SPRULESTATE(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRS_ACTIVE: SPRULESTATE = SPRULESTATE(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRS_ACTIVE_WITH_AUTO_PAUSE: SPRULESTATE = SPRULESTATE(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRS_ACTIVE_USER_DELIMITED: SPRULESTATE = SPRULESTATE(4i32);
impl ::core::marker::Copy for SPRULESTATE {}
impl ::core::clone::Clone for SPRULESTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPRULESTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPRULESTATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPRULESTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPRULESTATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPRUNSTATE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRS_DONE: SPRUNSTATE = SPRUNSTATE(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPRS_IS_SPEAKING: SPRUNSTATE = SPRUNSTATE(2i32);
impl ::core::marker::Copy for SPRUNSTATE {}
impl ::core::clone::Clone for SPRUNSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPRUNSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPRUNSTATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPRUNSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPRUNSTATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPSEMANTICFORMAT(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSMF_SAPI_PROPERTIES: SPSEMANTICFORMAT = SPSEMANTICFORMAT(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSMF_SRGS_SEMANTICINTERPRETATION_MS: SPSEMANTICFORMAT = SPSEMANTICFORMAT(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSMF_SRGS_SAPIPROPERTIES: SPSEMANTICFORMAT = SPSEMANTICFORMAT(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSMF_UPS: SPSEMANTICFORMAT = SPSEMANTICFORMAT(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSMF_SRGS_SEMANTICINTERPRETATION_W3C: SPSEMANTICFORMAT = SPSEMANTICFORMAT(8i32);
impl ::core::marker::Copy for SPSEMANTICFORMAT {}
impl ::core::clone::Clone for SPSEMANTICFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPSEMANTICFORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPSEMANTICFORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPSEMANTICFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPSEMANTICFORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPSHORTCUTTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSHT_NotOverriden: SPSHORTCUTTYPE = SPSHORTCUTTYPE(-1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSHT_Unknown: SPSHORTCUTTYPE = SPSHORTCUTTYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSHT_EMAIL: SPSHORTCUTTYPE = SPSHORTCUTTYPE(4096i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSHT_OTHER: SPSHORTCUTTYPE = SPSHORTCUTTYPE(8192i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_RESERVED1: SPSHORTCUTTYPE = SPSHORTCUTTYPE(12288i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_RESERVED2: SPSHORTCUTTYPE = SPSHORTCUTTYPE(16384i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_RESERVED3: SPSHORTCUTTYPE = SPSHORTCUTTYPE(20480i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPPS_RESERVED4: SPSHORTCUTTYPE = SPSHORTCUTTYPE(61440i32);
impl ::core::marker::Copy for SPSHORTCUTTYPE {}
impl ::core::clone::Clone for SPSHORTCUTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPSHORTCUTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPSHORTCUTTYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPSHORTCUTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPSHORTCUTTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPSTREAMFORMAT(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_Default: SPSTREAMFORMAT = SPSTREAMFORMAT(-1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_NoAssignedFormat: SPSTREAMFORMAT = SPSTREAMFORMAT(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_Text: SPSTREAMFORMAT = SPSTREAMFORMAT(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_NonStandardFormat: SPSTREAMFORMAT = SPSTREAMFORMAT(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_ExtendedAudioFormat: SPSTREAMFORMAT = SPSTREAMFORMAT(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_8kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_8kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_8kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_8kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(7i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_11kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_11kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(9i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_11kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(10i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_11kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(11i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_12kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(12i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_12kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(13i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_12kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(14i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_12kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(15i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_16kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(16i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_16kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(17i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_16kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(18i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_16kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(19i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_22kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(20i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_22kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(21i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_22kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(22i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_22kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(23i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_24kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(24i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_24kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(25i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_24kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(26i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_24kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(27i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_32kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(28i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_32kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(29i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_32kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(30i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_32kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(31i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_44kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(32i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_44kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(33i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_44kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(34i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_44kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(35i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_48kHz8BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(36i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_48kHz8BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(37i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_48kHz16BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(38i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_48kHz16BitStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(39i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_TrueSpeech_8kHz1BitMono: SPSTREAMFORMAT = SPSTREAMFORMAT(40i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_ALaw_8kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(41i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_ALaw_8kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(42i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_ALaw_11kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(43i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_ALaw_11kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(44i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_ALaw_22kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(45i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_ALaw_22kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(46i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_ALaw_44kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(47i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_ALaw_44kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(48i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_uLaw_8kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(49i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_uLaw_8kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(50i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_uLaw_11kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(51i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_uLaw_11kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(52i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_uLaw_22kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(53i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_uLaw_22kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(54i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_uLaw_44kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(55i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_CCITT_uLaw_44kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(56i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_ADPCM_8kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(57i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_ADPCM_8kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(58i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_ADPCM_11kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(59i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_ADPCM_11kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(60i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_ADPCM_22kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(61i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_ADPCM_22kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(62i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_ADPCM_44kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(63i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_ADPCM_44kHzStereo: SPSTREAMFORMAT = SPSTREAMFORMAT(64i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_GSM610_8kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(65i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_GSM610_11kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(66i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_GSM610_22kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(67i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_GSM610_44kHzMono: SPSTREAMFORMAT = SPSTREAMFORMAT(68i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSF_NUM_FORMATS: SPSTREAMFORMAT = SPSTREAMFORMAT(69i32);
impl ::core::marker::Copy for SPSTREAMFORMAT {}
impl ::core::clone::Clone for SPSTREAMFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPSTREAMFORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPSTREAMFORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPSTREAMFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPSTREAMFORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPSTREAMFORMATTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPWF_INPUT: SPSTREAMFORMATTYPE = SPSTREAMFORMATTYPE(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPWF_SRENGINE: SPSTREAMFORMATTYPE = SPSTREAMFORMATTYPE(1i32);
impl ::core::marker::Copy for SPSTREAMFORMATTYPE {}
impl ::core::clone::Clone for SPSTREAMFORMATTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPSTREAMFORMATTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPSTREAMFORMATTYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPSTREAMFORMATTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPSTREAMFORMATTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPVACTIONS(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVA_Speak: SPVACTIONS = SPVACTIONS(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVA_Silence: SPVACTIONS = SPVACTIONS(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVA_Pronounce: SPVACTIONS = SPVACTIONS(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVA_Bookmark: SPVACTIONS = SPVACTIONS(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVA_SpellOut: SPVACTIONS = SPVACTIONS(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVA_Section: SPVACTIONS = SPVACTIONS(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVA_ParseUnknownTag: SPVACTIONS = SPVACTIONS(6i32);
impl ::core::marker::Copy for SPVACTIONS {}
impl ::core::clone::Clone for SPVACTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPVACTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPVACTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPVACTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPVACTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPVALUETYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDF_PROPERTY: SPVALUETYPE = SPVALUETYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDF_REPLACEMENT: SPVALUETYPE = SPVALUETYPE(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDF_RULE: SPVALUETYPE = SPVALUETYPE(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDF_DISPLAYTEXT: SPVALUETYPE = SPVALUETYPE(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDF_LEXICALFORM: SPVALUETYPE = SPVALUETYPE(16i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDF_PRONUNCIATION: SPVALUETYPE = SPVALUETYPE(32i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDF_AUDIO: SPVALUETYPE = SPVALUETYPE(64i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDF_ALTERNATES: SPVALUETYPE = SPVALUETYPE(128i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPDF_ALL: SPVALUETYPE = SPVALUETYPE(255i32);
impl ::core::marker::Copy for SPVALUETYPE {}
impl ::core::clone::Clone for SPVALUETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPVALUETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPVALUETYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPVALUETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPVALUETYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPVFEATURE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVFEATURE_STRESSED: SPVFEATURE = SPVFEATURE(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVFEATURE_EMPHASIS: SPVFEATURE = SPVFEATURE(2i32);
impl ::core::marker::Copy for SPVFEATURE {}
impl ::core::clone::Clone for SPVFEATURE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPVFEATURE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPVFEATURE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPVFEATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPVFEATURE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPVISEMES(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_0: SPVISEMES = SPVISEMES(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_1: SPVISEMES = SPVISEMES(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_2: SPVISEMES = SPVISEMES(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_3: SPVISEMES = SPVISEMES(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_4: SPVISEMES = SPVISEMES(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_5: SPVISEMES = SPVISEMES(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_6: SPVISEMES = SPVISEMES(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_7: SPVISEMES = SPVISEMES(7i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_8: SPVISEMES = SPVISEMES(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_9: SPVISEMES = SPVISEMES(9i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_10: SPVISEMES = SPVISEMES(10i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_11: SPVISEMES = SPVISEMES(11i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_12: SPVISEMES = SPVISEMES(12i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_13: SPVISEMES = SPVISEMES(13i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_14: SPVISEMES = SPVISEMES(14i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_15: SPVISEMES = SPVISEMES(15i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_16: SPVISEMES = SPVISEMES(16i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_17: SPVISEMES = SPVISEMES(17i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_18: SPVISEMES = SPVISEMES(18i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_19: SPVISEMES = SPVISEMES(19i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_20: SPVISEMES = SPVISEMES(20i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SP_VISEME_21: SPVISEMES = SPVISEMES(21i32);
impl ::core::marker::Copy for SPVISEMES {}
impl ::core::clone::Clone for SPVISEMES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPVISEMES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPVISEMES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPVISEMES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPVISEMES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPVLIMITS(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPMIN_VOLUME: SPVLIMITS = SPVLIMITS(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPMAX_VOLUME: SPVLIMITS = SPVLIMITS(100i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPMIN_RATE: SPVLIMITS = SPVLIMITS(-10i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPMAX_RATE: SPVLIMITS = SPVLIMITS(10i32);
impl ::core::marker::Copy for SPVLIMITS {}
impl ::core::clone::Clone for SPVLIMITS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPVLIMITS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPVLIMITS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPVLIMITS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPVLIMITS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPVPRIORITY(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVPRI_NORMAL: SPVPRIORITY = SPVPRIORITY(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVPRI_ALERT: SPVPRIORITY = SPVPRIORITY(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPVPRI_OVER: SPVPRIORITY = SPVPRIORITY(2i32);
impl ::core::marker::Copy for SPVPRIORITY {}
impl ::core::clone::Clone for SPVPRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPVPRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPVPRIORITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPVPRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPVPRIORITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPWORDPRONOUNCEABLE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPWP_UNKNOWN_WORD_UNPRONOUNCEABLE: SPWORDPRONOUNCEABLE = SPWORDPRONOUNCEABLE(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPWP_UNKNOWN_WORD_PRONOUNCEABLE: SPWORDPRONOUNCEABLE = SPWORDPRONOUNCEABLE(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPWP_KNOWN_WORD_PRONOUNCEABLE: SPWORDPRONOUNCEABLE = SPWORDPRONOUNCEABLE(2i32);
impl ::core::marker::Copy for SPWORDPRONOUNCEABLE {}
impl ::core::clone::Clone for SPWORDPRONOUNCEABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPWORDPRONOUNCEABLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPWORDPRONOUNCEABLE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPWORDPRONOUNCEABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPWORDPRONOUNCEABLE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPWORDTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eWORDTYPE_ADDED: SPWORDTYPE = SPWORDTYPE(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const eWORDTYPE_DELETED: SPWORDTYPE = SPWORDTYPE(2i32);
impl ::core::marker::Copy for SPWORDTYPE {}
impl ::core::clone::Clone for SPWORDTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPWORDTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPWORDTYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPWORDTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPWORDTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SPXMLRESULTOPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPXRO_SML: SPXMLRESULTOPTIONS = SPXMLRESULTOPTIONS(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPXRO_Alternates_SML: SPXMLRESULTOPTIONS = SPXMLRESULTOPTIONS(1i32);
impl ::core::marker::Copy for SPXMLRESULTOPTIONS {}
impl ::core::clone::Clone for SPXMLRESULTOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SPXMLRESULTOPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SPXMLRESULTOPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SPXMLRESULTOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SPXMLRESULTOPTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechAudioFormatType(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTDefault: SpeechAudioFormatType = SpeechAudioFormatType(-1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTNoAssignedFormat: SpeechAudioFormatType = SpeechAudioFormatType(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTText: SpeechAudioFormatType = SpeechAudioFormatType(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTNonStandardFormat: SpeechAudioFormatType = SpeechAudioFormatType(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTExtendedAudioFormat: SpeechAudioFormatType = SpeechAudioFormatType(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT8kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT8kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT8kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT8kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(7i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT11kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT11kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(9i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT11kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(10i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT11kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(11i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT12kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(12i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT12kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(13i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT12kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(14i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT12kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(15i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT16kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(16i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT16kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(17i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT16kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(18i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT16kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(19i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT22kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(20i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT22kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(21i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT22kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(22i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT22kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(23i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT24kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(24i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT24kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(25i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT24kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(26i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT24kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(27i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT32kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(28i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT32kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(29i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT32kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(30i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT32kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(31i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT44kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(32i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT44kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(33i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT44kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(34i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT44kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(35i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT48kHz8BitMono: SpeechAudioFormatType = SpeechAudioFormatType(36i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT48kHz8BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(37i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT48kHz16BitMono: SpeechAudioFormatType = SpeechAudioFormatType(38i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFT48kHz16BitStereo: SpeechAudioFormatType = SpeechAudioFormatType(39i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTTrueSpeech_8kHz1BitMono: SpeechAudioFormatType = SpeechAudioFormatType(40i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_ALaw_8kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(41i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_ALaw_8kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(42i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_ALaw_11kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(43i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_ALaw_11kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(44i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_ALaw_22kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(45i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_ALaw_22kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(46i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_ALaw_44kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(47i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_ALaw_44kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(48i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_uLaw_8kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(49i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_uLaw_8kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(50i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_uLaw_11kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(51i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_uLaw_11kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(52i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_uLaw_22kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(53i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_uLaw_22kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(54i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_uLaw_44kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(55i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTCCITT_uLaw_44kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(56i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTADPCM_8kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(57i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTADPCM_8kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(58i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTADPCM_11kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(59i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTADPCM_11kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(60i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTADPCM_22kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(61i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTADPCM_22kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(62i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTADPCM_44kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(63i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTADPCM_44kHzStereo: SpeechAudioFormatType = SpeechAudioFormatType(64i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTGSM610_8kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(65i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTGSM610_11kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(66i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTGSM610_22kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(67i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SAFTGSM610_44kHzMono: SpeechAudioFormatType = SpeechAudioFormatType(68i32);
impl ::core::marker::Copy for SpeechAudioFormatType {}
impl ::core::clone::Clone for SpeechAudioFormatType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechAudioFormatType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechAudioFormatType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechAudioFormatType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechAudioFormatType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechAudioState(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SASClosed: SpeechAudioState = SpeechAudioState(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SASStop: SpeechAudioState = SpeechAudioState(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SASPause: SpeechAudioState = SpeechAudioState(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SASRun: SpeechAudioState = SpeechAudioState(3i32);
impl ::core::marker::Copy for SpeechAudioState {}
impl ::core::clone::Clone for SpeechAudioState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechAudioState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechAudioState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechAudioState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechAudioState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechBookmarkOptions(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SBONone: SpeechBookmarkOptions = SpeechBookmarkOptions(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SBOPause: SpeechBookmarkOptions = SpeechBookmarkOptions(1i32);
impl ::core::marker::Copy for SpeechBookmarkOptions {}
impl ::core::clone::Clone for SpeechBookmarkOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechBookmarkOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechBookmarkOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechBookmarkOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechBookmarkOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechDataKeyLocation(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDKLDefaultLocation: SpeechDataKeyLocation = SpeechDataKeyLocation(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDKLCurrentUser: SpeechDataKeyLocation = SpeechDataKeyLocation(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDKLLocalMachine: SpeechDataKeyLocation = SpeechDataKeyLocation(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDKLCurrentConfig: SpeechDataKeyLocation = SpeechDataKeyLocation(5i32);
impl ::core::marker::Copy for SpeechDataKeyLocation {}
impl ::core::clone::Clone for SpeechDataKeyLocation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechDataKeyLocation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechDataKeyLocation {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechDataKeyLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechDataKeyLocation").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechDiscardType(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDTProperty: SpeechDiscardType = SpeechDiscardType(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDTReplacement: SpeechDiscardType = SpeechDiscardType(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDTRule: SpeechDiscardType = SpeechDiscardType(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDTDisplayText: SpeechDiscardType = SpeechDiscardType(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDTLexicalForm: SpeechDiscardType = SpeechDiscardType(16i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDTPronunciation: SpeechDiscardType = SpeechDiscardType(32i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDTAudio: SpeechDiscardType = SpeechDiscardType(64i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDTAlternates: SpeechDiscardType = SpeechDiscardType(128i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDTAll: SpeechDiscardType = SpeechDiscardType(255i32);
impl ::core::marker::Copy for SpeechDiscardType {}
impl ::core::clone::Clone for SpeechDiscardType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechDiscardType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechDiscardType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechDiscardType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechDiscardType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechDisplayAttributes(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDA_No_Trailing_Space: SpeechDisplayAttributes = SpeechDisplayAttributes(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDA_One_Trailing_Space: SpeechDisplayAttributes = SpeechDisplayAttributes(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDA_Two_Trailing_Spaces: SpeechDisplayAttributes = SpeechDisplayAttributes(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SDA_Consume_Leading_Spaces: SpeechDisplayAttributes = SpeechDisplayAttributes(8i32);
impl ::core::marker::Copy for SpeechDisplayAttributes {}
impl ::core::clone::Clone for SpeechDisplayAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechDisplayAttributes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechDisplayAttributes {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechDisplayAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechDisplayAttributes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechEmulationCompareFlags(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SECFIgnoreCase: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SECFIgnoreKanaType: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(65536i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SECFIgnoreWidth: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(131072i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SECFNoSpecialChars: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(536870912i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SECFEmulateResult: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(1073741824i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SECFDefault: SpeechEmulationCompareFlags = SpeechEmulationCompareFlags(196609i32);
impl ::core::marker::Copy for SpeechEmulationCompareFlags {}
impl ::core::clone::Clone for SpeechEmulationCompareFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechEmulationCompareFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechEmulationCompareFlags {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechEmulationCompareFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechEmulationCompareFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechEngineConfidence(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SECLowConfidence: SpeechEngineConfidence = SpeechEngineConfidence(-1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SECNormalConfidence: SpeechEngineConfidence = SpeechEngineConfidence(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SECHighConfidence: SpeechEngineConfidence = SpeechEngineConfidence(1i32);
impl ::core::marker::Copy for SpeechEngineConfidence {}
impl ::core::clone::Clone for SpeechEngineConfidence {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechEngineConfidence {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechEngineConfidence {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechEngineConfidence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechEngineConfidence").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechFormatType(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SFTInput: SpeechFormatType = SpeechFormatType(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SFTSREngine: SpeechFormatType = SpeechFormatType(1i32);
impl ::core::marker::Copy for SpeechFormatType {}
impl ::core::clone::Clone for SpeechFormatType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechFormatType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechFormatType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechFormatType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechFormatType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechGrammarRuleStateTransitionType(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGRSTTEpsilon: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGRSTTWord: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGRSTTRule: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGRSTTDictation: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGRSTTWildcard: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGRSTTTextBuffer: SpeechGrammarRuleStateTransitionType = SpeechGrammarRuleStateTransitionType(5i32);
impl ::core::marker::Copy for SpeechGrammarRuleStateTransitionType {}
impl ::core::clone::Clone for SpeechGrammarRuleStateTransitionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechGrammarRuleStateTransitionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechGrammarRuleStateTransitionType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechGrammarRuleStateTransitionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechGrammarRuleStateTransitionType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechGrammarState(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGSEnabled: SpeechGrammarState = SpeechGrammarState(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGSDisabled: SpeechGrammarState = SpeechGrammarState(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGSExclusive: SpeechGrammarState = SpeechGrammarState(3i32);
impl ::core::marker::Copy for SpeechGrammarState {}
impl ::core::clone::Clone for SpeechGrammarState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechGrammarState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechGrammarState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechGrammarState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechGrammarState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechGrammarWordType(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGDisplay: SpeechGrammarWordType = SpeechGrammarWordType(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGLexical: SpeechGrammarWordType = SpeechGrammarWordType(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGPronounciation: SpeechGrammarWordType = SpeechGrammarWordType(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGLexicalNoSpecialChars: SpeechGrammarWordType = SpeechGrammarWordType(3i32);
impl ::core::marker::Copy for SpeechGrammarWordType {}
impl ::core::clone::Clone for SpeechGrammarWordType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechGrammarWordType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechGrammarWordType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechGrammarWordType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechGrammarWordType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechInterference(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SINone: SpeechInterference = SpeechInterference(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SINoise: SpeechInterference = SpeechInterference(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SINoSignal: SpeechInterference = SpeechInterference(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SITooLoud: SpeechInterference = SpeechInterference(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SITooQuiet: SpeechInterference = SpeechInterference(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SITooFast: SpeechInterference = SpeechInterference(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SITooSlow: SpeechInterference = SpeechInterference(6i32);
impl ::core::marker::Copy for SpeechInterference {}
impl ::core::clone::Clone for SpeechInterference {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechInterference {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechInterference {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechInterference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechInterference").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechLexiconType(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SLTUser: SpeechLexiconType = SpeechLexiconType(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SLTApp: SpeechLexiconType = SpeechLexiconType(2i32);
impl ::core::marker::Copy for SpeechLexiconType {}
impl ::core::clone::Clone for SpeechLexiconType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechLexiconType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechLexiconType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechLexiconType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechLexiconType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechLoadOption(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SLOStatic: SpeechLoadOption = SpeechLoadOption(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SLODynamic: SpeechLoadOption = SpeechLoadOption(1i32);
impl ::core::marker::Copy for SpeechLoadOption {}
impl ::core::clone::Clone for SpeechLoadOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechLoadOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechLoadOption {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechLoadOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechLoadOption").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechPartOfSpeech(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSNotOverriden: SpeechPartOfSpeech = SpeechPartOfSpeech(-1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSUnknown: SpeechPartOfSpeech = SpeechPartOfSpeech(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSNoun: SpeechPartOfSpeech = SpeechPartOfSpeech(4096i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSVerb: SpeechPartOfSpeech = SpeechPartOfSpeech(8192i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSModifier: SpeechPartOfSpeech = SpeechPartOfSpeech(12288i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSFunction: SpeechPartOfSpeech = SpeechPartOfSpeech(16384i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSInterjection: SpeechPartOfSpeech = SpeechPartOfSpeech(20480i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSLMA: SpeechPartOfSpeech = SpeechPartOfSpeech(28672i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SPSSuppressWord: SpeechPartOfSpeech = SpeechPartOfSpeech(61440i32);
impl ::core::marker::Copy for SpeechPartOfSpeech {}
impl ::core::clone::Clone for SpeechPartOfSpeech {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechPartOfSpeech {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechPartOfSpeech {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechPartOfSpeech {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechPartOfSpeech").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechRecoContextState(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRCS_Disabled: SpeechRecoContextState = SpeechRecoContextState(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRCS_Enabled: SpeechRecoContextState = SpeechRecoContextState(1i32);
impl ::core::marker::Copy for SpeechRecoContextState {}
impl ::core::clone::Clone for SpeechRecoContextState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechRecoContextState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechRecoContextState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechRecoContextState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecoContextState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechRecoEvents(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREStreamEnd: SpeechRecoEvents = SpeechRecoEvents(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRESoundStart: SpeechRecoEvents = SpeechRecoEvents(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRESoundEnd: SpeechRecoEvents = SpeechRecoEvents(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREPhraseStart: SpeechRecoEvents = SpeechRecoEvents(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRERecognition: SpeechRecoEvents = SpeechRecoEvents(16i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREHypothesis: SpeechRecoEvents = SpeechRecoEvents(32i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREBookmark: SpeechRecoEvents = SpeechRecoEvents(64i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREPropertyNumChange: SpeechRecoEvents = SpeechRecoEvents(128i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREPropertyStringChange: SpeechRecoEvents = SpeechRecoEvents(256i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREFalseRecognition: SpeechRecoEvents = SpeechRecoEvents(512i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREInterference: SpeechRecoEvents = SpeechRecoEvents(1024i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRERequestUI: SpeechRecoEvents = SpeechRecoEvents(2048i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREStateChange: SpeechRecoEvents = SpeechRecoEvents(4096i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREAdaptation: SpeechRecoEvents = SpeechRecoEvents(8192i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREStreamStart: SpeechRecoEvents = SpeechRecoEvents(16384i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRERecoOtherContext: SpeechRecoEvents = SpeechRecoEvents(32768i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREAudioLevel: SpeechRecoEvents = SpeechRecoEvents(65536i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREPrivate: SpeechRecoEvents = SpeechRecoEvents(262144i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SREAllEvents: SpeechRecoEvents = SpeechRecoEvents(393215i32);
impl ::core::marker::Copy for SpeechRecoEvents {}
impl ::core::clone::Clone for SpeechRecoEvents {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechRecoEvents {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechRecoEvents {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechRecoEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecoEvents").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechRecognitionType(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRTStandard: SpeechRecognitionType = SpeechRecognitionType(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRTAutopause: SpeechRecognitionType = SpeechRecognitionType(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRTEmulated: SpeechRecognitionType = SpeechRecognitionType(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRTSMLTimeout: SpeechRecognitionType = SpeechRecognitionType(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRTExtendableParse: SpeechRecognitionType = SpeechRecognitionType(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRTReSent: SpeechRecognitionType = SpeechRecognitionType(16i32);
impl ::core::marker::Copy for SpeechRecognitionType {}
impl ::core::clone::Clone for SpeechRecognitionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechRecognitionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechRecognitionType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechRecognitionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognitionType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechRecognizerState(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRSInactive: SpeechRecognizerState = SpeechRecognizerState(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRSActive: SpeechRecognizerState = SpeechRecognizerState(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRSActiveAlways: SpeechRecognizerState = SpeechRecognizerState(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRSInactiveWithPurge: SpeechRecognizerState = SpeechRecognizerState(3i32);
impl ::core::marker::Copy for SpeechRecognizerState {}
impl ::core::clone::Clone for SpeechRecognizerState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechRecognizerState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechRecognizerState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechRecognizerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRecognizerState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechRetainedAudioOptions(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRAONone: SpeechRetainedAudioOptions = SpeechRetainedAudioOptions(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRAORetainAudio: SpeechRetainedAudioOptions = SpeechRetainedAudioOptions(1i32);
impl ::core::marker::Copy for SpeechRetainedAudioOptions {}
impl ::core::clone::Clone for SpeechRetainedAudioOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechRetainedAudioOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechRetainedAudioOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechRetainedAudioOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRetainedAudioOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechRuleAttributes(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRATopLevel: SpeechRuleAttributes = SpeechRuleAttributes(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRADefaultToActive: SpeechRuleAttributes = SpeechRuleAttributes(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRAExport: SpeechRuleAttributes = SpeechRuleAttributes(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRAImport: SpeechRuleAttributes = SpeechRuleAttributes(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRAInterpreter: SpeechRuleAttributes = SpeechRuleAttributes(16i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRADynamic: SpeechRuleAttributes = SpeechRuleAttributes(32i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRARoot: SpeechRuleAttributes = SpeechRuleAttributes(64i32);
impl ::core::marker::Copy for SpeechRuleAttributes {}
impl ::core::clone::Clone for SpeechRuleAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechRuleAttributes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechRuleAttributes {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechRuleAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRuleAttributes").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechRuleState(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGDSInactive: SpeechRuleState = SpeechRuleState(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGDSActive: SpeechRuleState = SpeechRuleState(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGDSActiveWithAutoPause: SpeechRuleState = SpeechRuleState(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SGDSActiveUserDelimited: SpeechRuleState = SpeechRuleState(4i32);
impl ::core::marker::Copy for SpeechRuleState {}
impl ::core::clone::Clone for SpeechRuleState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechRuleState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechRuleState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechRuleState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRuleState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechRunState(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRSEDone: SpeechRunState = SpeechRunState(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SRSEIsSpeaking: SpeechRunState = SpeechRunState(2i32);
impl ::core::marker::Copy for SpeechRunState {}
impl ::core::clone::Clone for SpeechRunState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechRunState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechRunState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechRunState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechRunState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechSpecialTransitionType(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SSTTWildcard: SpeechSpecialTransitionType = SpeechSpecialTransitionType(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SSTTDictation: SpeechSpecialTransitionType = SpeechSpecialTransitionType(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SSTTTextBuffer: SpeechSpecialTransitionType = SpeechSpecialTransitionType(3i32);
impl ::core::marker::Copy for SpeechSpecialTransitionType {}
impl ::core::clone::Clone for SpeechSpecialTransitionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechSpecialTransitionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechSpecialTransitionType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechSpecialTransitionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechSpecialTransitionType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechStreamFileMode(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SSFMOpenForRead: SpeechStreamFileMode = SpeechStreamFileMode(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SSFMOpenReadWrite: SpeechStreamFileMode = SpeechStreamFileMode(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SSFMCreate: SpeechStreamFileMode = SpeechStreamFileMode(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SSFMCreateForWrite: SpeechStreamFileMode = SpeechStreamFileMode(3i32);
impl ::core::marker::Copy for SpeechStreamFileMode {}
impl ::core::clone::Clone for SpeechStreamFileMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechStreamFileMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechStreamFileMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechStreamFileMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechStreamFileMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechStreamSeekPositionType(pub u32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SSSPTRelativeToStart: SpeechStreamSeekPositionType = SpeechStreamSeekPositionType(0u32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SSSPTRelativeToCurrentPosition: SpeechStreamSeekPositionType = SpeechStreamSeekPositionType(1u32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SSSPTRelativeToEnd: SpeechStreamSeekPositionType = SpeechStreamSeekPositionType(2u32);
impl ::core::marker::Copy for SpeechStreamSeekPositionType {}
impl ::core::clone::Clone for SpeechStreamSeekPositionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechStreamSeekPositionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechStreamSeekPositionType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechStreamSeekPositionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechStreamSeekPositionType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechTokenContext(pub u32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const STCInprocServer: SpeechTokenContext = SpeechTokenContext(1u32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const STCInprocHandler: SpeechTokenContext = SpeechTokenContext(2u32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const STCLocalServer: SpeechTokenContext = SpeechTokenContext(4u32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const STCRemoteServer: SpeechTokenContext = SpeechTokenContext(16u32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const STCAll: SpeechTokenContext = SpeechTokenContext(23u32);
impl ::core::marker::Copy for SpeechTokenContext {}
impl ::core::clone::Clone for SpeechTokenContext {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechTokenContext {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechTokenContext {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechTokenContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechTokenContext").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechTokenShellFolder(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const STSF_AppData: SpeechTokenShellFolder = SpeechTokenShellFolder(26i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const STSF_LocalAppData: SpeechTokenShellFolder = SpeechTokenShellFolder(28i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const STSF_CommonAppData: SpeechTokenShellFolder = SpeechTokenShellFolder(35i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const STSF_FlagCreate: SpeechTokenShellFolder = SpeechTokenShellFolder(32768i32);
impl ::core::marker::Copy for SpeechTokenShellFolder {}
impl ::core::clone::Clone for SpeechTokenShellFolder {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechTokenShellFolder {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechTokenShellFolder {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechTokenShellFolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechTokenShellFolder").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechVisemeFeature(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVF_None: SpeechVisemeFeature = SpeechVisemeFeature(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVF_Stressed: SpeechVisemeFeature = SpeechVisemeFeature(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVF_Emphasis: SpeechVisemeFeature = SpeechVisemeFeature(2i32);
impl ::core::marker::Copy for SpeechVisemeFeature {}
impl ::core::clone::Clone for SpeechVisemeFeature {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechVisemeFeature {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechVisemeFeature {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechVisemeFeature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechVisemeFeature").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechVisemeType(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_0: SpeechVisemeType = SpeechVisemeType(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_1: SpeechVisemeType = SpeechVisemeType(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_2: SpeechVisemeType = SpeechVisemeType(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_3: SpeechVisemeType = SpeechVisemeType(3i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_4: SpeechVisemeType = SpeechVisemeType(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_5: SpeechVisemeType = SpeechVisemeType(5i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_6: SpeechVisemeType = SpeechVisemeType(6i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_7: SpeechVisemeType = SpeechVisemeType(7i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_8: SpeechVisemeType = SpeechVisemeType(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_9: SpeechVisemeType = SpeechVisemeType(9i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_10: SpeechVisemeType = SpeechVisemeType(10i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_11: SpeechVisemeType = SpeechVisemeType(11i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_12: SpeechVisemeType = SpeechVisemeType(12i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_13: SpeechVisemeType = SpeechVisemeType(13i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_14: SpeechVisemeType = SpeechVisemeType(14i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_15: SpeechVisemeType = SpeechVisemeType(15i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_16: SpeechVisemeType = SpeechVisemeType(16i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_17: SpeechVisemeType = SpeechVisemeType(17i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_18: SpeechVisemeType = SpeechVisemeType(18i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_19: SpeechVisemeType = SpeechVisemeType(19i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_20: SpeechVisemeType = SpeechVisemeType(20i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVP_21: SpeechVisemeType = SpeechVisemeType(21i32);
impl ::core::marker::Copy for SpeechVisemeType {}
impl ::core::clone::Clone for SpeechVisemeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechVisemeType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechVisemeType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechVisemeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechVisemeType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechVoiceEvents(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVEStartInputStream: SpeechVoiceEvents = SpeechVoiceEvents(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVEEndInputStream: SpeechVoiceEvents = SpeechVoiceEvents(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVEVoiceChange: SpeechVoiceEvents = SpeechVoiceEvents(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVEBookmark: SpeechVoiceEvents = SpeechVoiceEvents(16i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVEWordBoundary: SpeechVoiceEvents = SpeechVoiceEvents(32i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVEPhoneme: SpeechVoiceEvents = SpeechVoiceEvents(64i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVESentenceBoundary: SpeechVoiceEvents = SpeechVoiceEvents(128i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVEViseme: SpeechVoiceEvents = SpeechVoiceEvents(256i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVEAudioLevel: SpeechVoiceEvents = SpeechVoiceEvents(512i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVEPrivate: SpeechVoiceEvents = SpeechVoiceEvents(32768i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVEAllEvents: SpeechVoiceEvents = SpeechVoiceEvents(33790i32);
impl ::core::marker::Copy for SpeechVoiceEvents {}
impl ::core::clone::Clone for SpeechVoiceEvents {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechVoiceEvents {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechVoiceEvents {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechVoiceEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechVoiceEvents").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechVoicePriority(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVPNormal: SpeechVoicePriority = SpeechVoicePriority(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVPAlert: SpeechVoicePriority = SpeechVoicePriority(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVPOver: SpeechVoicePriority = SpeechVoicePriority(2i32);
impl ::core::marker::Copy for SpeechVoicePriority {}
impl ::core::clone::Clone for SpeechVoicePriority {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechVoicePriority {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechVoicePriority {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechVoicePriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechVoicePriority").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechVoiceSpeakFlags(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFDefault: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFlagsAsync: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFPurgeBeforeSpeak: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(2i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFIsFilename: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(4i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFIsXML: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(8i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFIsNotXML: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(16i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFPersistXML: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(32i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFNLPSpeakPunc: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(64i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFParseSapi: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(128i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFParseSsml: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(256i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFParseAutodetect: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFNLPMask: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(64i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFParseMask: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(384i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFVoiceMask: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(511i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SVSFUnusedFlags: SpeechVoiceSpeakFlags = SpeechVoiceSpeakFlags(-512i32);
impl ::core::marker::Copy for SpeechVoiceSpeakFlags {}
impl ::core::clone::Clone for SpeechVoiceSpeakFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechVoiceSpeakFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechVoiceSpeakFlags {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechVoiceSpeakFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechVoiceSpeakFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechWordPronounceable(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SWPUnknownWordUnpronounceable: SpeechWordPronounceable = SpeechWordPronounceable(0i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SWPUnknownWordPronounceable: SpeechWordPronounceable = SpeechWordPronounceable(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SWPKnownWordPronounceable: SpeechWordPronounceable = SpeechWordPronounceable(2i32);
impl ::core::marker::Copy for SpeechWordPronounceable {}
impl ::core::clone::Clone for SpeechWordPronounceable {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechWordPronounceable {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechWordPronounceable {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechWordPronounceable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechWordPronounceable").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SpeechWordType(pub i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SWTAdded: SpeechWordType = SpeechWordType(1i32);
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub const SWTDeleted: SpeechWordType = SpeechWordType(2i32);
impl ::core::marker::Copy for SpeechWordType {}
impl ::core::clone::Clone for SpeechWordType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SpeechWordType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SpeechWordType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SpeechWordType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechWordType").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPAUDIOBUFFERINFO {
    pub ulMsMinNotification: u32,
    pub ulMsBufferSize: u32,
    pub ulMsEventBias: u32,
}
impl ::core::marker::Copy for SPAUDIOBUFFERINFO {}
impl ::core::clone::Clone for SPAUDIOBUFFERINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPAUDIOBUFFERINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPAUDIOBUFFERINFO").field("ulMsMinNotification", &self.ulMsMinNotification).field("ulMsBufferSize", &self.ulMsBufferSize).field("ulMsEventBias", &self.ulMsEventBias).finish()
    }
}
impl ::windows::core::TypeKind for SPAUDIOBUFFERINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPAUDIOBUFFERINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ulMsMinNotification == other.ulMsMinNotification && self.ulMsBufferSize == other.ulMsBufferSize && self.ulMsEventBias == other.ulMsEventBias
    }
}
impl ::core::cmp::Eq for SPAUDIOBUFFERINFO {}
impl ::core::default::Default for SPAUDIOBUFFERINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPAUDIOSTATUS {
    pub cbFreeBuffSpace: i32,
    pub cbNonBlockingIO: u32,
    pub State: SPAUDIOSTATE,
    pub CurSeekPos: u64,
    pub CurDevicePos: u64,
    pub dwAudioLevel: u32,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for SPAUDIOSTATUS {}
impl ::core::clone::Clone for SPAUDIOSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPAUDIOSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPAUDIOSTATUS").field("cbFreeBuffSpace", &self.cbFreeBuffSpace).field("cbNonBlockingIO", &self.cbNonBlockingIO).field("State", &self.State).field("CurSeekPos", &self.CurSeekPos).field("CurDevicePos", &self.CurDevicePos).field("dwAudioLevel", &self.dwAudioLevel).field("dwReserved2", &self.dwReserved2).finish()
    }
}
impl ::windows::core::TypeKind for SPAUDIOSTATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPAUDIOSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.cbFreeBuffSpace == other.cbFreeBuffSpace && self.cbNonBlockingIO == other.cbNonBlockingIO && self.State == other.State && self.CurSeekPos == other.CurSeekPos && self.CurDevicePos == other.CurDevicePos && self.dwAudioLevel == other.dwAudioLevel && self.dwReserved2 == other.dwReserved2
    }
}
impl ::core::cmp::Eq for SPAUDIOSTATUS {}
impl ::core::default::Default for SPAUDIOSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPBINARYGRAMMAR {
    pub ulTotalSerializedSize: u32,
}
impl ::core::marker::Copy for SPBINARYGRAMMAR {}
impl ::core::clone::Clone for SPBINARYGRAMMAR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPBINARYGRAMMAR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPBINARYGRAMMAR").field("ulTotalSerializedSize", &self.ulTotalSerializedSize).finish()
    }
}
impl ::windows::core::TypeKind for SPBINARYGRAMMAR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPBINARYGRAMMAR {
    fn eq(&self, other: &Self) -> bool {
        self.ulTotalSerializedSize == other.ulTotalSerializedSize
    }
}
impl ::core::cmp::Eq for SPBINARYGRAMMAR {}
impl ::core::default::Default for SPBINARYGRAMMAR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPDISPLAYPHRASE {
    pub ulNumTokens: u32,
    pub pTokens: *mut SPDISPLAYTOKEN,
}
impl ::core::marker::Copy for SPDISPLAYPHRASE {}
impl ::core::clone::Clone for SPDISPLAYPHRASE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPDISPLAYPHRASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPDISPLAYPHRASE").field("ulNumTokens", &self.ulNumTokens).field("pTokens", &self.pTokens).finish()
    }
}
impl ::windows::core::TypeKind for SPDISPLAYPHRASE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPDISPLAYPHRASE {
    fn eq(&self, other: &Self) -> bool {
        self.ulNumTokens == other.ulNumTokens && self.pTokens == other.pTokens
    }
}
impl ::core::cmp::Eq for SPDISPLAYPHRASE {}
impl ::core::default::Default for SPDISPLAYPHRASE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPDISPLAYTOKEN {
    pub pszLexical: ::windows::core::PCWSTR,
    pub pszDisplay: ::windows::core::PCWSTR,
    pub bDisplayAttributes: u8,
}
impl ::core::marker::Copy for SPDISPLAYTOKEN {}
impl ::core::clone::Clone for SPDISPLAYTOKEN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPDISPLAYTOKEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPDISPLAYTOKEN").field("pszLexical", &self.pszLexical).field("pszDisplay", &self.pszDisplay).field("bDisplayAttributes", &self.bDisplayAttributes).finish()
    }
}
impl ::windows::core::TypeKind for SPDISPLAYTOKEN {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPDISPLAYTOKEN {
    fn eq(&self, other: &Self) -> bool {
        self.pszLexical == other.pszLexical && self.pszDisplay == other.pszDisplay && self.bDisplayAttributes == other.bDisplayAttributes
    }
}
impl ::core::cmp::Eq for SPDISPLAYTOKEN {}
impl ::core::default::Default for SPDISPLAYTOKEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SPEVENT {
    pub _bitfield: i32,
    pub ulStreamNum: u32,
    pub ullAudioStreamOffset: u64,
    pub wParam: super::super::Foundation::WPARAM,
    pub lParam: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPEVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SPEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPEVENT").field("_bitfield", &self._bitfield).field("ulStreamNum", &self.ulStreamNum).field("ullAudioStreamOffset", &self.ullAudioStreamOffset).field("wParam", &self.wParam).field("lParam", &self.lParam).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SPEVENT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SPEVENT {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.ulStreamNum == other.ulStreamNum && self.ullAudioStreamOffset == other.ullAudioStreamOffset && self.wParam == other.wParam && self.lParam == other.lParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SPEVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SPEVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SPEVENTEX {
    pub _bitfield: i32,
    pub ulStreamNum: u32,
    pub ullAudioStreamOffset: u64,
    pub wParam: super::super::Foundation::WPARAM,
    pub lParam: super::super::Foundation::LPARAM,
    pub ullAudioTimeOffset: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPEVENTEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPEVENTEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SPEVENTEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPEVENTEX").field("_bitfield", &self._bitfield).field("ulStreamNum", &self.ulStreamNum).field("ullAudioStreamOffset", &self.ullAudioStreamOffset).field("wParam", &self.wParam).field("lParam", &self.lParam).field("ullAudioTimeOffset", &self.ullAudioTimeOffset).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SPEVENTEX {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SPEVENTEX {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.ulStreamNum == other.ulStreamNum && self.ullAudioStreamOffset == other.ullAudioStreamOffset && self.wParam == other.wParam && self.lParam == other.lParam && self.ullAudioTimeOffset == other.ullAudioTimeOffset
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SPEVENTEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SPEVENTEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPEVENTSOURCEINFO {
    pub ullEventInterest: u64,
    pub ullQueuedInterest: u64,
    pub ulCount: u32,
}
impl ::core::marker::Copy for SPEVENTSOURCEINFO {}
impl ::core::clone::Clone for SPEVENTSOURCEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPEVENTSOURCEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPEVENTSOURCEINFO").field("ullEventInterest", &self.ullEventInterest).field("ullQueuedInterest", &self.ullQueuedInterest).field("ulCount", &self.ulCount).finish()
    }
}
impl ::windows::core::TypeKind for SPEVENTSOURCEINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPEVENTSOURCEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ullEventInterest == other.ullEventInterest && self.ullQueuedInterest == other.ullQueuedInterest && self.ulCount == other.ulCount
    }
}
impl ::core::cmp::Eq for SPEVENTSOURCEINFO {}
impl ::core::default::Default for SPEVENTSOURCEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPNORMALIZATIONLIST {
    pub ulSize: u32,
    pub ppszzNormalizedList: *mut *mut u16,
}
impl ::core::marker::Copy for SPNORMALIZATIONLIST {}
impl ::core::clone::Clone for SPNORMALIZATIONLIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPNORMALIZATIONLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPNORMALIZATIONLIST").field("ulSize", &self.ulSize).field("ppszzNormalizedList", &self.ppszzNormalizedList).finish()
    }
}
impl ::windows::core::TypeKind for SPNORMALIZATIONLIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPNORMALIZATIONLIST {
    fn eq(&self, other: &Self) -> bool {
        self.ulSize == other.ulSize && self.ppszzNormalizedList == other.ppszzNormalizedList
    }
}
impl ::core::cmp::Eq for SPNORMALIZATIONLIST {}
impl ::core::default::Default for SPNORMALIZATIONLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct SPPHRASE {
    pub Base: SPPHRASE_50,
    pub pSML: ::windows::core::PWSTR,
    pub pSemanticErrorInfo: *mut SPSEMANTICERRORINFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for SPPHRASE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for SPPHRASE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for SPPHRASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPPHRASE").field("Base", &self.Base).field("pSML", &self.pSML).field("pSemanticErrorInfo", &self.pSemanticErrorInfo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::TypeKind for SPPHRASE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for SPPHRASE {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base && self.pSML == other.pSML && self.pSemanticErrorInfo == other.pSemanticErrorInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for SPPHRASE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for SPPHRASE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPPHRASEELEMENT {
    pub ulAudioTimeOffset: u32,
    pub ulAudioSizeTime: u32,
    pub ulAudioStreamOffset: u32,
    pub ulAudioSizeBytes: u32,
    pub ulRetainedStreamOffset: u32,
    pub ulRetainedSizeBytes: u32,
    pub pszDisplayText: ::windows::core::PCWSTR,
    pub pszLexicalForm: ::windows::core::PCWSTR,
    pub pszPronunciation: *const u16,
    pub bDisplayAttributes: u8,
    pub RequiredConfidence: i8,
    pub ActualConfidence: i8,
    pub Reserved: u8,
    pub SREngineConfidence: f32,
}
impl ::core::marker::Copy for SPPHRASEELEMENT {}
impl ::core::clone::Clone for SPPHRASEELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPPHRASEELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPPHRASEELEMENT")
            .field("ulAudioTimeOffset", &self.ulAudioTimeOffset)
            .field("ulAudioSizeTime", &self.ulAudioSizeTime)
            .field("ulAudioStreamOffset", &self.ulAudioStreamOffset)
            .field("ulAudioSizeBytes", &self.ulAudioSizeBytes)
            .field("ulRetainedStreamOffset", &self.ulRetainedStreamOffset)
            .field("ulRetainedSizeBytes", &self.ulRetainedSizeBytes)
            .field("pszDisplayText", &self.pszDisplayText)
            .field("pszLexicalForm", &self.pszLexicalForm)
            .field("pszPronunciation", &self.pszPronunciation)
            .field("bDisplayAttributes", &self.bDisplayAttributes)
            .field("RequiredConfidence", &self.RequiredConfidence)
            .field("ActualConfidence", &self.ActualConfidence)
            .field("Reserved", &self.Reserved)
            .field("SREngineConfidence", &self.SREngineConfidence)
            .finish()
    }
}
impl ::windows::core::TypeKind for SPPHRASEELEMENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPPHRASEELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.ulAudioTimeOffset == other.ulAudioTimeOffset && self.ulAudioSizeTime == other.ulAudioSizeTime && self.ulAudioStreamOffset == other.ulAudioStreamOffset && self.ulAudioSizeBytes == other.ulAudioSizeBytes && self.ulRetainedStreamOffset == other.ulRetainedStreamOffset && self.ulRetainedSizeBytes == other.ulRetainedSizeBytes && self.pszDisplayText == other.pszDisplayText && self.pszLexicalForm == other.pszLexicalForm && self.pszPronunciation == other.pszPronunciation && self.bDisplayAttributes == other.bDisplayAttributes && self.RequiredConfidence == other.RequiredConfidence && self.ActualConfidence == other.ActualConfidence && self.Reserved == other.Reserved && self.SREngineConfidence == other.SREngineConfidence
    }
}
impl ::core::cmp::Eq for SPPHRASEELEMENT {}
impl ::core::default::Default for SPPHRASEELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct SPPHRASEPROPERTY {
    pub pszName: ::windows::core::PCWSTR,
    pub Anonymous: SPPHRASEPROPERTY_0,
    pub pszValue: ::windows::core::PCWSTR,
    pub vValue: super::super::System::Com::VARIANT,
    pub ulFirstElement: u32,
    pub ulCountOfElements: u32,
    pub pNextSibling: *const SPPHRASEPROPERTY,
    pub pFirstChild: *const SPPHRASEPROPERTY,
    pub SREngineConfidence: f32,
    pub Confidence: i8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for SPPHRASEPROPERTY {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::TypeKind for SPPHRASEPROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for SPPHRASEPROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub union SPPHRASEPROPERTY_0 {
    pub ulId: u32,
    pub Anonymous: SPPHRASEPROPERTY_0_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for SPPHRASEPROPERTY_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for SPPHRASEPROPERTY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::TypeKind for SPPHRASEPROPERTY_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for SPPHRASEPROPERTY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct SPPHRASEPROPERTY_0_0 {
    pub bType: u8,
    pub bReserved: u8,
    pub usArrayIndex: u16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for SPPHRASEPROPERTY_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for SPPHRASEPROPERTY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for SPPHRASEPROPERTY_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPPHRASEPROPERTY_0_0").field("bType", &self.bType).field("bReserved", &self.bReserved).field("usArrayIndex", &self.usArrayIndex).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::TypeKind for SPPHRASEPROPERTY_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for SPPHRASEPROPERTY_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.bType == other.bType && self.bReserved == other.bReserved && self.usArrayIndex == other.usArrayIndex
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for SPPHRASEPROPERTY_0_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for SPPHRASEPROPERTY_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPPHRASEREPLACEMENT {
    pub bDisplayAttributes: u8,
    pub pszReplacementText: ::windows::core::PCWSTR,
    pub ulFirstElement: u32,
    pub ulCountOfElements: u32,
}
impl ::core::marker::Copy for SPPHRASEREPLACEMENT {}
impl ::core::clone::Clone for SPPHRASEREPLACEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPPHRASEREPLACEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPPHRASEREPLACEMENT").field("bDisplayAttributes", &self.bDisplayAttributes).field("pszReplacementText", &self.pszReplacementText).field("ulFirstElement", &self.ulFirstElement).field("ulCountOfElements", &self.ulCountOfElements).finish()
    }
}
impl ::windows::core::TypeKind for SPPHRASEREPLACEMENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPPHRASEREPLACEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.bDisplayAttributes == other.bDisplayAttributes && self.pszReplacementText == other.pszReplacementText && self.ulFirstElement == other.ulFirstElement && self.ulCountOfElements == other.ulCountOfElements
    }
}
impl ::core::cmp::Eq for SPPHRASEREPLACEMENT {}
impl ::core::default::Default for SPPHRASEREPLACEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPPHRASERULE {
    pub pszName: ::windows::core::PCWSTR,
    pub ulId: u32,
    pub ulFirstElement: u32,
    pub ulCountOfElements: u32,
    pub pNextSibling: *const SPPHRASERULE,
    pub pFirstChild: *const SPPHRASERULE,
    pub SREngineConfidence: f32,
    pub Confidence: i8,
}
impl ::core::marker::Copy for SPPHRASERULE {}
impl ::core::clone::Clone for SPPHRASERULE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPPHRASERULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPPHRASERULE").field("pszName", &self.pszName).field("ulId", &self.ulId).field("ulFirstElement", &self.ulFirstElement).field("ulCountOfElements", &self.ulCountOfElements).field("pNextSibling", &self.pNextSibling).field("pFirstChild", &self.pFirstChild).field("SREngineConfidence", &self.SREngineConfidence).field("Confidence", &self.Confidence).finish()
    }
}
impl ::windows::core::TypeKind for SPPHRASERULE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPPHRASERULE {
    fn eq(&self, other: &Self) -> bool {
        self.pszName == other.pszName && self.ulId == other.ulId && self.ulFirstElement == other.ulFirstElement && self.ulCountOfElements == other.ulCountOfElements && self.pNextSibling == other.pNextSibling && self.pFirstChild == other.pFirstChild && self.SREngineConfidence == other.SREngineConfidence && self.Confidence == other.Confidence
    }
}
impl ::core::cmp::Eq for SPPHRASERULE {}
impl ::core::default::Default for SPPHRASERULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct SPPHRASE_50 {
    pub cbSize: u32,
    pub LangID: u16,
    pub wHomophoneGroupId: u16,
    pub ullGrammarID: u64,
    pub ftStartTime: u64,
    pub ullAudioStreamPosition: u64,
    pub ulAudioSizeBytes: u32,
    pub ulRetainedSizeBytes: u32,
    pub ulAudioSizeTime: u32,
    pub Rule: SPPHRASERULE,
    pub pProperties: *const SPPHRASEPROPERTY,
    pub pElements: *const SPPHRASEELEMENT,
    pub cReplacements: u32,
    pub pReplacements: *const SPPHRASEREPLACEMENT,
    pub SREngineID: ::windows::core::GUID,
    pub ulSREnginePrivateDataSize: u32,
    pub pSREnginePrivateData: *const u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::marker::Copy for SPPHRASE_50 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for SPPHRASE_50 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::fmt::Debug for SPPHRASE_50 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPPHRASE_50")
            .field("cbSize", &self.cbSize)
            .field("LangID", &self.LangID)
            .field("wHomophoneGroupId", &self.wHomophoneGroupId)
            .field("ullGrammarID", &self.ullGrammarID)
            .field("ftStartTime", &self.ftStartTime)
            .field("ullAudioStreamPosition", &self.ullAudioStreamPosition)
            .field("ulAudioSizeBytes", &self.ulAudioSizeBytes)
            .field("ulRetainedSizeBytes", &self.ulRetainedSizeBytes)
            .field("ulAudioSizeTime", &self.ulAudioSizeTime)
            .field("Rule", &self.Rule)
            .field("pProperties", &self.pProperties)
            .field("pElements", &self.pElements)
            .field("cReplacements", &self.cReplacements)
            .field("pReplacements", &self.pReplacements)
            .field("SREngineID", &self.SREngineID)
            .field("ulSREnginePrivateDataSize", &self.ulSREnginePrivateDataSize)
            .field("pSREnginePrivateData", &self.pSREnginePrivateData)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::TypeKind for SPPHRASE_50 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::PartialEq for SPPHRASE_50 {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.LangID == other.LangID && self.wHomophoneGroupId == other.wHomophoneGroupId && self.ullGrammarID == other.ullGrammarID && self.ftStartTime == other.ftStartTime && self.ullAudioStreamPosition == other.ullAudioStreamPosition && self.ulAudioSizeBytes == other.ulAudioSizeBytes && self.ulRetainedSizeBytes == other.ulRetainedSizeBytes && self.ulAudioSizeTime == other.ulAudioSizeTime && self.Rule == other.Rule && self.pProperties == other.pProperties && self.pElements == other.pElements && self.cReplacements == other.cReplacements && self.pReplacements == other.pReplacements && self.SREngineID == other.SREngineID && self.ulSREnginePrivateDataSize == other.ulSREnginePrivateDataSize && self.pSREnginePrivateData == other.pSREnginePrivateData
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::cmp::Eq for SPPHRASE_50 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for SPPHRASE_50 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct SPPROPERTYINFO {
    pub pszName: ::windows::core::PCWSTR,
    pub ulId: u32,
    pub pszValue: ::windows::core::PCWSTR,
    pub vValue: super::super::System::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for SPPROPERTYINFO {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::TypeKind for SPPROPERTYINFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for SPPROPERTYINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPRECOCONTEXTSTATUS {
    pub eInterference: SPINTERFERENCE,
    pub szRequestTypeOfUI: [u16; 255],
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for SPRECOCONTEXTSTATUS {}
impl ::core::clone::Clone for SPRECOCONTEXTSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPRECOCONTEXTSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPRECOCONTEXTSTATUS").field("eInterference", &self.eInterference).field("szRequestTypeOfUI", &self.szRequestTypeOfUI).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).finish()
    }
}
impl ::windows::core::TypeKind for SPRECOCONTEXTSTATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPRECOCONTEXTSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.eInterference == other.eInterference && self.szRequestTypeOfUI == other.szRequestTypeOfUI && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2
    }
}
impl ::core::cmp::Eq for SPRECOCONTEXTSTATUS {}
impl ::core::default::Default for SPRECOCONTEXTSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPRECOGNIZERSTATUS {
    pub AudioStatus: SPAUDIOSTATUS,
    pub ullRecognitionStreamPos: u64,
    pub ulStreamNumber: u32,
    pub ulNumActive: u32,
    pub clsidEngine: ::windows::core::GUID,
    pub cLangIDs: u32,
    pub aLangID: [u16; 20],
    pub ullRecognitionStreamTime: u64,
}
impl ::core::marker::Copy for SPRECOGNIZERSTATUS {}
impl ::core::clone::Clone for SPRECOGNIZERSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPRECOGNIZERSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPRECOGNIZERSTATUS").field("AudioStatus", &self.AudioStatus).field("ullRecognitionStreamPos", &self.ullRecognitionStreamPos).field("ulStreamNumber", &self.ulStreamNumber).field("ulNumActive", &self.ulNumActive).field("clsidEngine", &self.clsidEngine).field("cLangIDs", &self.cLangIDs).field("aLangID", &self.aLangID).field("ullRecognitionStreamTime", &self.ullRecognitionStreamTime).finish()
    }
}
impl ::windows::core::TypeKind for SPRECOGNIZERSTATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPRECOGNIZERSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.AudioStatus == other.AudioStatus && self.ullRecognitionStreamPos == other.ullRecognitionStreamPos && self.ulStreamNumber == other.ulStreamNumber && self.ulNumActive == other.ulNumActive && self.clsidEngine == other.clsidEngine && self.cLangIDs == other.cLangIDs && self.aLangID == other.aLangID && self.ullRecognitionStreamTime == other.ullRecognitionStreamTime
    }
}
impl ::core::cmp::Eq for SPRECOGNIZERSTATUS {}
impl ::core::default::Default for SPRECOGNIZERSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SPRECORESULTTIMES {
    pub ftStreamTime: super::super::Foundation::FILETIME,
    pub ullLength: u64,
    pub dwTickCount: u32,
    pub ullStart: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SPRECORESULTTIMES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SPRECORESULTTIMES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SPRECORESULTTIMES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPRECORESULTTIMES").field("ftStreamTime", &self.ftStreamTime).field("ullLength", &self.ullLength).field("dwTickCount", &self.dwTickCount).field("ullStart", &self.ullStart).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SPRECORESULTTIMES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SPRECORESULTTIMES {
    fn eq(&self, other: &Self) -> bool {
        self.ftStreamTime == other.ftStreamTime && self.ullLength == other.ullLength && self.dwTickCount == other.dwTickCount && self.ullStart == other.ullStart
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SPRECORESULTTIMES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SPRECORESULTTIMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPRULE {
    pub pszRuleName: ::windows::core::PCWSTR,
    pub ulRuleId: u32,
    pub dwAttributes: u32,
}
impl ::core::marker::Copy for SPRULE {}
impl ::core::clone::Clone for SPRULE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPRULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPRULE").field("pszRuleName", &self.pszRuleName).field("ulRuleId", &self.ulRuleId).field("dwAttributes", &self.dwAttributes).finish()
    }
}
impl ::windows::core::TypeKind for SPRULE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPRULE {
    fn eq(&self, other: &Self) -> bool {
        self.pszRuleName == other.pszRuleName && self.ulRuleId == other.ulRuleId && self.dwAttributes == other.dwAttributes
    }
}
impl ::core::cmp::Eq for SPRULE {}
impl ::core::default::Default for SPRULE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPSEMANTICERRORINFO {
    pub ulLineNumber: u32,
    pub pszScriptLine: ::windows::core::PWSTR,
    pub pszSource: ::windows::core::PWSTR,
    pub pszDescription: ::windows::core::PWSTR,
    pub hrResultCode: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for SPSEMANTICERRORINFO {}
impl ::core::clone::Clone for SPSEMANTICERRORINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPSEMANTICERRORINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPSEMANTICERRORINFO").field("ulLineNumber", &self.ulLineNumber).field("pszScriptLine", &self.pszScriptLine).field("pszSource", &self.pszSource).field("pszDescription", &self.pszDescription).field("hrResultCode", &self.hrResultCode).finish()
    }
}
impl ::windows::core::TypeKind for SPSEMANTICERRORINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPSEMANTICERRORINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ulLineNumber == other.ulLineNumber && self.pszScriptLine == other.pszScriptLine && self.pszSource == other.pszSource && self.pszDescription == other.pszDescription && self.hrResultCode == other.hrResultCode
    }
}
impl ::core::cmp::Eq for SPSEMANTICERRORINFO {}
impl ::core::default::Default for SPSEMANTICERRORINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPSERIALIZEDEVENT {
    pub _bitfield: i32,
    pub ulStreamNum: u32,
    pub ullAudioStreamOffset: u64,
    pub SerializedwParam: u32,
    pub SerializedlParam: i32,
}
impl ::core::marker::Copy for SPSERIALIZEDEVENT {}
impl ::core::clone::Clone for SPSERIALIZEDEVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPSERIALIZEDEVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPSERIALIZEDEVENT").field("_bitfield", &self._bitfield).field("ulStreamNum", &self.ulStreamNum).field("ullAudioStreamOffset", &self.ullAudioStreamOffset).field("SerializedwParam", &self.SerializedwParam).field("SerializedlParam", &self.SerializedlParam).finish()
    }
}
impl ::windows::core::TypeKind for SPSERIALIZEDEVENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPSERIALIZEDEVENT {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.ulStreamNum == other.ulStreamNum && self.ullAudioStreamOffset == other.ullAudioStreamOffset && self.SerializedwParam == other.SerializedwParam && self.SerializedlParam == other.SerializedlParam
    }
}
impl ::core::cmp::Eq for SPSERIALIZEDEVENT {}
impl ::core::default::Default for SPSERIALIZEDEVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPSERIALIZEDEVENT64 {
    pub _bitfield: i32,
    pub ulStreamNum: u32,
    pub ullAudioStreamOffset: u64,
    pub SerializedwParam: u64,
    pub SerializedlParam: i64,
}
impl ::core::marker::Copy for SPSERIALIZEDEVENT64 {}
impl ::core::clone::Clone for SPSERIALIZEDEVENT64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPSERIALIZEDEVENT64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPSERIALIZEDEVENT64").field("_bitfield", &self._bitfield).field("ulStreamNum", &self.ulStreamNum).field("ullAudioStreamOffset", &self.ullAudioStreamOffset).field("SerializedwParam", &self.SerializedwParam).field("SerializedlParam", &self.SerializedlParam).finish()
    }
}
impl ::windows::core::TypeKind for SPSERIALIZEDEVENT64 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPSERIALIZEDEVENT64 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.ulStreamNum == other.ulStreamNum && self.ullAudioStreamOffset == other.ullAudioStreamOffset && self.SerializedwParam == other.SerializedwParam && self.SerializedlParam == other.SerializedlParam
    }
}
impl ::core::cmp::Eq for SPSERIALIZEDEVENT64 {}
impl ::core::default::Default for SPSERIALIZEDEVENT64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPSERIALIZEDPHRASE {
    pub ulSerializedSize: u32,
}
impl ::core::marker::Copy for SPSERIALIZEDPHRASE {}
impl ::core::clone::Clone for SPSERIALIZEDPHRASE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPSERIALIZEDPHRASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPSERIALIZEDPHRASE").field("ulSerializedSize", &self.ulSerializedSize).finish()
    }
}
impl ::windows::core::TypeKind for SPSERIALIZEDPHRASE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPSERIALIZEDPHRASE {
    fn eq(&self, other: &Self) -> bool {
        self.ulSerializedSize == other.ulSerializedSize
    }
}
impl ::core::cmp::Eq for SPSERIALIZEDPHRASE {}
impl ::core::default::Default for SPSERIALIZEDPHRASE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPSERIALIZEDRESULT {
    pub ulSerializedSize: u32,
}
impl ::core::marker::Copy for SPSERIALIZEDRESULT {}
impl ::core::clone::Clone for SPSERIALIZEDRESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPSERIALIZEDRESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPSERIALIZEDRESULT").field("ulSerializedSize", &self.ulSerializedSize).finish()
    }
}
impl ::windows::core::TypeKind for SPSERIALIZEDRESULT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPSERIALIZEDRESULT {
    fn eq(&self, other: &Self) -> bool {
        self.ulSerializedSize == other.ulSerializedSize
    }
}
impl ::core::cmp::Eq for SPSERIALIZEDRESULT {}
impl ::core::default::Default for SPSERIALIZEDRESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPSHORTCUTPAIR {
    pub pNextSHORTCUTPAIR: *mut SPSHORTCUTPAIR,
    pub LangID: u16,
    pub shType: SPSHORTCUTTYPE,
    pub pszDisplay: ::windows::core::PWSTR,
    pub pszSpoken: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for SPSHORTCUTPAIR {}
impl ::core::clone::Clone for SPSHORTCUTPAIR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPSHORTCUTPAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPSHORTCUTPAIR").field("pNextSHORTCUTPAIR", &self.pNextSHORTCUTPAIR).field("LangID", &self.LangID).field("shType", &self.shType).field("pszDisplay", &self.pszDisplay).field("pszSpoken", &self.pszSpoken).finish()
    }
}
impl ::windows::core::TypeKind for SPSHORTCUTPAIR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPSHORTCUTPAIR {
    fn eq(&self, other: &Self) -> bool {
        self.pNextSHORTCUTPAIR == other.pNextSHORTCUTPAIR && self.LangID == other.LangID && self.shType == other.shType && self.pszDisplay == other.pszDisplay && self.pszSpoken == other.pszSpoken
    }
}
impl ::core::cmp::Eq for SPSHORTCUTPAIR {}
impl ::core::default::Default for SPSHORTCUTPAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPSHORTCUTPAIRLIST {
    pub ulSize: u32,
    pub pvBuffer: *mut u8,
    pub pFirstShortcutPair: *mut SPSHORTCUTPAIR,
}
impl ::core::marker::Copy for SPSHORTCUTPAIRLIST {}
impl ::core::clone::Clone for SPSHORTCUTPAIRLIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPSHORTCUTPAIRLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPSHORTCUTPAIRLIST").field("ulSize", &self.ulSize).field("pvBuffer", &self.pvBuffer).field("pFirstShortcutPair", &self.pFirstShortcutPair).finish()
    }
}
impl ::windows::core::TypeKind for SPSHORTCUTPAIRLIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPSHORTCUTPAIRLIST {
    fn eq(&self, other: &Self) -> bool {
        self.ulSize == other.ulSize && self.pvBuffer == other.pvBuffer && self.pFirstShortcutPair == other.pFirstShortcutPair
    }
}
impl ::core::cmp::Eq for SPSHORTCUTPAIRLIST {}
impl ::core::default::Default for SPSHORTCUTPAIRLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPSTATEHANDLE__ {
    pub unused: i32,
}
impl ::core::marker::Copy for SPSTATEHANDLE__ {}
impl ::core::clone::Clone for SPSTATEHANDLE__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPSTATEHANDLE__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPSTATEHANDLE__").field("unused", &self.unused).finish()
    }
}
impl ::windows::core::TypeKind for SPSTATEHANDLE__ {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPSTATEHANDLE__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for SPSTATEHANDLE__ {}
impl ::core::default::Default for SPSTATEHANDLE__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPTEXTSELECTIONINFO {
    pub ulStartActiveOffset: u32,
    pub cchActiveChars: u32,
    pub ulStartSelection: u32,
    pub cchSelection: u32,
}
impl ::core::marker::Copy for SPTEXTSELECTIONINFO {}
impl ::core::clone::Clone for SPTEXTSELECTIONINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPTEXTSELECTIONINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPTEXTSELECTIONINFO").field("ulStartActiveOffset", &self.ulStartActiveOffset).field("cchActiveChars", &self.cchActiveChars).field("ulStartSelection", &self.ulStartSelection).field("cchSelection", &self.cchSelection).finish()
    }
}
impl ::windows::core::TypeKind for SPTEXTSELECTIONINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPTEXTSELECTIONINFO {
    fn eq(&self, other: &Self) -> bool {
        self.ulStartActiveOffset == other.ulStartActiveOffset && self.cchActiveChars == other.cchActiveChars && self.ulStartSelection == other.ulStartSelection && self.cchSelection == other.cchSelection
    }
}
impl ::core::cmp::Eq for SPTEXTSELECTIONINFO {}
impl ::core::default::Default for SPTEXTSELECTIONINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPVCONTEXT {
    pub pCategory: ::windows::core::PCWSTR,
    pub pBefore: ::windows::core::PCWSTR,
    pub pAfter: ::windows::core::PCWSTR,
}
impl ::core::marker::Copy for SPVCONTEXT {}
impl ::core::clone::Clone for SPVCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPVCONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPVCONTEXT").field("pCategory", &self.pCategory).field("pBefore", &self.pBefore).field("pAfter", &self.pAfter).finish()
    }
}
impl ::windows::core::TypeKind for SPVCONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPVCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.pCategory == other.pCategory && self.pBefore == other.pBefore && self.pAfter == other.pAfter
    }
}
impl ::core::cmp::Eq for SPVCONTEXT {}
impl ::core::default::Default for SPVCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPVOICESTATUS {
    pub ulCurrentStream: u32,
    pub ulLastStreamQueued: u32,
    pub hrLastResult: ::windows::core::HRESULT,
    pub dwRunningState: u32,
    pub ulInputWordPos: u32,
    pub ulInputWordLen: u32,
    pub ulInputSentPos: u32,
    pub ulInputSentLen: u32,
    pub lBookmarkId: i32,
    pub PhonemeId: u16,
    pub VisemeId: SPVISEMES,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for SPVOICESTATUS {}
impl ::core::clone::Clone for SPVOICESTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPVOICESTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPVOICESTATUS")
            .field("ulCurrentStream", &self.ulCurrentStream)
            .field("ulLastStreamQueued", &self.ulLastStreamQueued)
            .field("hrLastResult", &self.hrLastResult)
            .field("dwRunningState", &self.dwRunningState)
            .field("ulInputWordPos", &self.ulInputWordPos)
            .field("ulInputWordLen", &self.ulInputWordLen)
            .field("ulInputSentPos", &self.ulInputSentPos)
            .field("ulInputSentLen", &self.ulInputSentLen)
            .field("lBookmarkId", &self.lBookmarkId)
            .field("PhonemeId", &self.PhonemeId)
            .field("VisemeId", &self.VisemeId)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .finish()
    }
}
impl ::windows::core::TypeKind for SPVOICESTATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPVOICESTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.ulCurrentStream == other.ulCurrentStream && self.ulLastStreamQueued == other.ulLastStreamQueued && self.hrLastResult == other.hrLastResult && self.dwRunningState == other.dwRunningState && self.ulInputWordPos == other.ulInputWordPos && self.ulInputWordLen == other.ulInputWordLen && self.ulInputSentPos == other.ulInputSentPos && self.ulInputSentLen == other.ulInputSentLen && self.lBookmarkId == other.lBookmarkId && self.PhonemeId == other.PhonemeId && self.VisemeId == other.VisemeId && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2
    }
}
impl ::core::cmp::Eq for SPVOICESTATUS {}
impl ::core::default::Default for SPVOICESTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPVPITCH {
    pub MiddleAdj: i32,
    pub RangeAdj: i32,
}
impl ::core::marker::Copy for SPVPITCH {}
impl ::core::clone::Clone for SPVPITCH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPVPITCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPVPITCH").field("MiddleAdj", &self.MiddleAdj).field("RangeAdj", &self.RangeAdj).finish()
    }
}
impl ::windows::core::TypeKind for SPVPITCH {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPVPITCH {
    fn eq(&self, other: &Self) -> bool {
        self.MiddleAdj == other.MiddleAdj && self.RangeAdj == other.RangeAdj
    }
}
impl ::core::cmp::Eq for SPVPITCH {}
impl ::core::default::Default for SPVPITCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPVSTATE {
    pub eAction: SPVACTIONS,
    pub LangID: u16,
    pub wReserved: u16,
    pub EmphAdj: i32,
    pub RateAdj: i32,
    pub Volume: u32,
    pub PitchAdj: SPVPITCH,
    pub SilenceMSecs: u32,
    pub pPhoneIds: *mut u16,
    pub ePartOfSpeech: SPPARTOFSPEECH,
    pub Context: SPVCONTEXT,
}
impl ::core::marker::Copy for SPVSTATE {}
impl ::core::clone::Clone for SPVSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPVSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPVSTATE").field("eAction", &self.eAction).field("LangID", &self.LangID).field("wReserved", &self.wReserved).field("EmphAdj", &self.EmphAdj).field("RateAdj", &self.RateAdj).field("Volume", &self.Volume).field("PitchAdj", &self.PitchAdj).field("SilenceMSecs", &self.SilenceMSecs).field("pPhoneIds", &self.pPhoneIds).field("ePartOfSpeech", &self.ePartOfSpeech).field("Context", &self.Context).finish()
    }
}
impl ::windows::core::TypeKind for SPVSTATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPVSTATE {
    fn eq(&self, other: &Self) -> bool {
        self.eAction == other.eAction && self.LangID == other.LangID && self.wReserved == other.wReserved && self.EmphAdj == other.EmphAdj && self.RateAdj == other.RateAdj && self.Volume == other.Volume && self.PitchAdj == other.PitchAdj && self.SilenceMSecs == other.SilenceMSecs && self.pPhoneIds == other.pPhoneIds && self.ePartOfSpeech == other.ePartOfSpeech && self.Context == other.Context
    }
}
impl ::core::cmp::Eq for SPVSTATE {}
impl ::core::default::Default for SPVSTATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPWORD {
    pub pNextWord: *mut SPWORD,
    pub LangID: u16,
    pub wReserved: u16,
    pub eWordType: SPWORDTYPE,
    pub pszWord: ::windows::core::PWSTR,
    pub pFirstWordPronunciation: *mut SPWORDPRONUNCIATION,
}
impl ::core::marker::Copy for SPWORD {}
impl ::core::clone::Clone for SPWORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPWORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPWORD").field("pNextWord", &self.pNextWord).field("LangID", &self.LangID).field("wReserved", &self.wReserved).field("eWordType", &self.eWordType).field("pszWord", &self.pszWord).field("pFirstWordPronunciation", &self.pFirstWordPronunciation).finish()
    }
}
impl ::windows::core::TypeKind for SPWORD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPWORD {
    fn eq(&self, other: &Self) -> bool {
        self.pNextWord == other.pNextWord && self.LangID == other.LangID && self.wReserved == other.wReserved && self.eWordType == other.eWordType && self.pszWord == other.pszWord && self.pFirstWordPronunciation == other.pFirstWordPronunciation
    }
}
impl ::core::cmp::Eq for SPWORD {}
impl ::core::default::Default for SPWORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPWORDLIST {
    pub ulSize: u32,
    pub pvBuffer: *mut u8,
    pub pFirstWord: *mut SPWORD,
}
impl ::core::marker::Copy for SPWORDLIST {}
impl ::core::clone::Clone for SPWORDLIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPWORDLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPWORDLIST").field("ulSize", &self.ulSize).field("pvBuffer", &self.pvBuffer).field("pFirstWord", &self.pFirstWord).finish()
    }
}
impl ::windows::core::TypeKind for SPWORDLIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPWORDLIST {
    fn eq(&self, other: &Self) -> bool {
        self.ulSize == other.ulSize && self.pvBuffer == other.pvBuffer && self.pFirstWord == other.pFirstWord
    }
}
impl ::core::cmp::Eq for SPWORDLIST {}
impl ::core::default::Default for SPWORDLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPWORDPRONUNCIATION {
    pub pNextWordPronunciation: *mut SPWORDPRONUNCIATION,
    pub eLexiconType: SPLEXICONTYPE,
    pub LangID: u16,
    pub wPronunciationFlags: u16,
    pub ePartOfSpeech: SPPARTOFSPEECH,
    pub szPronunciation: [u16; 1],
}
impl ::core::marker::Copy for SPWORDPRONUNCIATION {}
impl ::core::clone::Clone for SPWORDPRONUNCIATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPWORDPRONUNCIATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPWORDPRONUNCIATION").field("pNextWordPronunciation", &self.pNextWordPronunciation).field("eLexiconType", &self.eLexiconType).field("LangID", &self.LangID).field("wPronunciationFlags", &self.wPronunciationFlags).field("ePartOfSpeech", &self.ePartOfSpeech).field("szPronunciation", &self.szPronunciation).finish()
    }
}
impl ::windows::core::TypeKind for SPWORDPRONUNCIATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPWORDPRONUNCIATION {
    fn eq(&self, other: &Self) -> bool {
        self.pNextWordPronunciation == other.pNextWordPronunciation && self.eLexiconType == other.eLexiconType && self.LangID == other.LangID && self.wPronunciationFlags == other.wPronunciationFlags && self.ePartOfSpeech == other.ePartOfSpeech && self.szPronunciation == other.szPronunciation
    }
}
impl ::core::cmp::Eq for SPWORDPRONUNCIATION {}
impl ::core::default::Default for SPWORDPRONUNCIATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Speech\"`*"]
pub struct SPWORDPRONUNCIATIONLIST {
    pub ulSize: u32,
    pub pvBuffer: *mut u8,
    pub pFirstWordPronunciation: *mut SPWORDPRONUNCIATION,
}
impl ::core::marker::Copy for SPWORDPRONUNCIATIONLIST {}
impl ::core::clone::Clone for SPWORDPRONUNCIATIONLIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SPWORDPRONUNCIATIONLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SPWORDPRONUNCIATIONLIST").field("ulSize", &self.ulSize).field("pvBuffer", &self.pvBuffer).field("pFirstWordPronunciation", &self.pFirstWordPronunciation).finish()
    }
}
impl ::windows::core::TypeKind for SPWORDPRONUNCIATIONLIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SPWORDPRONUNCIATIONLIST {
    fn eq(&self, other: &Self) -> bool {
        self.ulSize == other.ulSize && self.pvBuffer == other.pvBuffer && self.pFirstWordPronunciation == other.pFirstWordPronunciation
    }
}
impl ::core::cmp::Eq for SPWORDPRONUNCIATIONLIST {}
impl ::core::default::Default for SPWORDPRONUNCIATIONLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type SPNOTIFYCALLBACK = ::core::option::Option<unsafe extern "system" fn(wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
