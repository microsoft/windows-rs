#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"implement\"`*"]
pub trait IEnumSpObjectTokens_Impl: Sized {
    fn Next(&self, celt: u32, pelt: *mut ::core::option::Option<ISpObjectToken>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumSpObjectTokens>;
    fn Item(&self, index: u32) -> ::windows_core::Result<ISpObjectToken>;
    fn GetCount(&self, pcount: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEnumSpObjectTokens {}
impl IEnumSpObjectTokens_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSpObjectTokens_Impl, const OFFSET: isize>() -> IEnumSpObjectTokens_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, pelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pptoken: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptoken, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCount(::core::mem::transmute_copy(&pcount)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumSpObjectTokens as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_Media_Audio\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
pub trait ISpAudio_Impl: Sized + ISpStreamFormat_Impl {
    fn SetState(&self, newstate: SPAUDIOSTATE, ullreserved: u64) -> ::windows_core::Result<()>;
    fn SetFormat(&self, rguidfmtid: *const ::windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn GetStatus(&self, pstatus: *mut SPAUDIOSTATUS) -> ::windows_core::Result<()>;
    fn SetBufferInfo(&self, pbuffinfo: *const SPAUDIOBUFFERINFO) -> ::windows_core::Result<()>;
    fn GetBufferInfo(&self, pbuffinfo: *mut SPAUDIOBUFFERINFO) -> ::windows_core::Result<()>;
    fn GetDefaultFormat(&self, pformatid: *mut ::windows_core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn EventHandle(&self) -> super::super::Foundation::HANDLE;
    fn GetVolumeLevel(&self, plevel: *mut u32) -> ::windows_core::Result<()>;
    fn SetVolumeLevel(&self, level: u32) -> ::windows_core::Result<()>;
    fn GetBufferNotifySize(&self, pcbsize: *mut u32) -> ::windows_core::Result<()>;
    fn SetBufferNotifySize(&self, cbsize: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for ISpAudio {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ISpAudio_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: isize>() -> ISpAudio_Vtbl {
        unsafe extern "system" fn SetState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: SPAUDIOSTATE, ullreserved: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetState(::core::mem::transmute_copy(&newstate), ::core::mem::transmute_copy(&ullreserved)).into()
        }
        unsafe extern "system" fn SetFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidfmtid: *const ::windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFormat(::core::mem::transmute_copy(&rguidfmtid), ::core::mem::transmute_copy(&pwaveformatex)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut SPAUDIOSTATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStatus(::core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn SetBufferInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffinfo: *const SPAUDIOBUFFERINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBufferInfo(::core::mem::transmute_copy(&pbuffinfo)).into()
        }
        unsafe extern "system" fn GetBufferInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffinfo: *mut SPAUDIOBUFFERINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBufferInfo(::core::mem::transmute_copy(&pbuffinfo)).into()
        }
        unsafe extern "system" fn GetDefaultFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatid: *mut ::windows_core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDefaultFormat(::core::mem::transmute_copy(&pformatid), ::core::mem::transmute_copy(&ppcomemwaveformatex)).into()
        }
        unsafe extern "system" fn EventHandle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EventHandle()
        }
        unsafe extern "system" fn GetVolumeLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plevel: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVolumeLevel(::core::mem::transmute_copy(&plevel)).into()
        }
        unsafe extern "system" fn SetVolumeLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVolumeLevel(::core::mem::transmute_copy(&level)).into()
        }
        unsafe extern "system" fn GetBufferNotifySize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBufferNotifySize(::core::mem::transmute_copy(&pcbsize)).into()
        }
        unsafe extern "system" fn SetBufferNotifySize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbsize: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBufferNotifySize(::core::mem::transmute_copy(&cbsize)).into()
        }
        Self {
            base__: ISpStreamFormat_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetState: SetState::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            SetBufferInfo: SetBufferInfo::<Identity, Impl, OFFSET>,
            GetBufferInfo: GetBufferInfo::<Identity, Impl, OFFSET>,
            GetDefaultFormat: GetDefaultFormat::<Identity, Impl, OFFSET>,
            EventHandle: EventHandle::<Identity, Impl, OFFSET>,
            GetVolumeLevel: GetVolumeLevel::<Identity, Impl, OFFSET>,
            SetVolumeLevel: SetVolumeLevel::<Identity, Impl, OFFSET>,
            GetBufferNotifySize: GetBufferNotifySize::<Identity, Impl, OFFSET>,
            SetBufferNotifySize: SetBufferNotifySize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpAudio as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::ISequentialStream as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IStream as ::windows_core::ComInterface>::IID || iid == &<ISpStreamFormat as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"implement\"`*"]
pub trait ISpContainerLexicon_Impl: Sized + ISpLexicon_Impl {
    fn AddLexicon(&self, paddlexicon: ::core::option::Option<&ISpLexicon>, dwflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISpContainerLexicon {}
impl ISpContainerLexicon_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpContainerLexicon_Impl, const OFFSET: isize>() -> ISpContainerLexicon_Vtbl {
        unsafe extern "system" fn AddLexicon<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpContainerLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddlexicon: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddLexicon(::windows_core::from_raw_borrowed(&paddlexicon), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base__: ISpLexicon_Vtbl::new::<Identity, Impl, OFFSET>(), AddLexicon: AddLexicon::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpContainerLexicon as ::windows_core::ComInterface>::IID || iid == &<ISpLexicon as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"implement\"`*"]
pub trait ISpDataKey_Impl: Sized {
    fn SetData(&self, pszvaluename: &::windows_core::PCWSTR, cbdata: u32, pdata: *const u8) -> ::windows_core::Result<()>;
    fn GetData(&self, pszvaluename: &::windows_core::PCWSTR, pcbdata: *mut u32, pdata: *mut u8) -> ::windows_core::Result<()>;
    fn SetStringValue(&self, pszvaluename: &::windows_core::PCWSTR, pszvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetStringValue(&self, pszvaluename: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetDWORD(&self, pszvaluename: &::windows_core::PCWSTR, dwvalue: u32) -> ::windows_core::Result<()>;
    fn GetDWORD(&self, pszvaluename: &::windows_core::PCWSTR, pdwvalue: *mut u32) -> ::windows_core::Result<()>;
    fn OpenKey(&self, pszsubkeyname: &::windows_core::PCWSTR) -> ::windows_core::Result<ISpDataKey>;
    fn CreateKey(&self, pszsubkey: &::windows_core::PCWSTR) -> ::windows_core::Result<ISpDataKey>;
    fn DeleteKey(&self, pszsubkey: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn DeleteValue(&self, pszvaluename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn EnumKeys(&self, index: u32) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn EnumValues(&self, index: u32) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::RuntimeName for ISpDataKey {}
impl ISpDataKey_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: isize>() -> ISpDataKey_Vtbl {
        unsafe extern "system" fn SetData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: ::windows_core::PCWSTR, cbdata: u32, pdata: *const u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetData(::core::mem::transmute(&pszvaluename), ::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: ::windows_core::PCWSTR, pcbdata: *mut u32, pdata: *mut u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetData(::core::mem::transmute(&pszvaluename), ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetStringValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: ::windows_core::PCWSTR, pszvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStringValue(::core::mem::transmute(&pszvaluename), ::core::mem::transmute(&pszvalue)).into()
        }
        unsafe extern "system" fn GetStringValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: ::windows_core::PCWSTR, ppszvalue: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStringValue(::core::mem::transmute(&pszvaluename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDWORD<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: ::windows_core::PCWSTR, dwvalue: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDWORD(::core::mem::transmute(&pszvaluename), ::core::mem::transmute_copy(&dwvalue)).into()
        }
        unsafe extern "system" fn GetDWORD<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: ::windows_core::PCWSTR, pdwvalue: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDWORD(::core::mem::transmute(&pszvaluename), ::core::mem::transmute_copy(&pdwvalue)).into()
        }
        unsafe extern "system" fn OpenKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubkeyname: ::windows_core::PCWSTR, ppsubkey: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenKey(::core::mem::transmute(&pszsubkeyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsubkey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubkey: ::windows_core::PCWSTR, ppsubkey: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateKey(::core::mem::transmute(&pszsubkey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsubkey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubkey: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteKey(::core::mem::transmute(&pszsubkey)).into()
        }
        unsafe extern "system" fn DeleteValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszvaluename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteValue(::core::mem::transmute(&pszvaluename)).into()
        }
        unsafe extern "system" fn EnumKeys<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppszsubkeyname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumKeys(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszsubkeyname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumValues<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppszvaluename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumValues(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszvaluename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetData: SetData::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            SetStringValue: SetStringValue::<Identity, Impl, OFFSET>,
            GetStringValue: GetStringValue::<Identity, Impl, OFFSET>,
            SetDWORD: SetDWORD::<Identity, Impl, OFFSET>,
            GetDWORD: GetDWORD::<Identity, Impl, OFFSET>,
            OpenKey: OpenKey::<Identity, Impl, OFFSET>,
            CreateKey: CreateKey::<Identity, Impl, OFFSET>,
            DeleteKey: DeleteKey::<Identity, Impl, OFFSET>,
            DeleteValue: DeleteValue::<Identity, Impl, OFFSET>,
            EnumKeys: EnumKeys::<Identity, Impl, OFFSET>,
            EnumValues: EnumValues::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpDataKey as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"implement\"`*"]
pub trait ISpDisplayAlternates_Impl: Sized {
    fn GetDisplayAlternates(&self, pphrase: *const SPDISPLAYPHRASE, crequestcount: u32, ppcomemphrases: *mut *mut SPDISPLAYPHRASE, pcphrasesreturned: *mut u32) -> ::windows_core::Result<()>;
    fn SetFullStopTrailSpace(&self, ultrailspace: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISpDisplayAlternates {}
impl ISpDisplayAlternates_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpDisplayAlternates_Impl, const OFFSET: isize>() -> ISpDisplayAlternates_Vtbl {
        unsafe extern "system" fn GetDisplayAlternates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpDisplayAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphrase: *const SPDISPLAYPHRASE, crequestcount: u32, ppcomemphrases: *mut *mut SPDISPLAYPHRASE, pcphrasesreturned: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplayAlternates(::core::mem::transmute_copy(&pphrase), ::core::mem::transmute_copy(&crequestcount), ::core::mem::transmute_copy(&ppcomemphrases), ::core::mem::transmute_copy(&pcphrasesreturned)).into()
        }
        unsafe extern "system" fn SetFullStopTrailSpace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpDisplayAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ultrailspace: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFullStopTrailSpace(::core::mem::transmute_copy(&ultrailspace)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDisplayAlternates: GetDisplayAlternates::<Identity, Impl, OFFSET>,
            SetFullStopTrailSpace: SetFullStopTrailSpace::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpDisplayAlternates as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"implement\"`*"]
pub trait ISpEnginePronunciation_Impl: Sized {
    fn Normalize(&self, pszword: &::windows_core::PCWSTR, pszleftcontext: &::windows_core::PCWSTR, pszrightcontext: &::windows_core::PCWSTR, langid: u16, pnormalizationlist: *mut SPNORMALIZATIONLIST) -> ::windows_core::Result<()>;
    fn GetPronunciations(&self, pszword: &::windows_core::PCWSTR, pszleftcontext: &::windows_core::PCWSTR, pszrightcontext: &::windows_core::PCWSTR, langid: u16, penginepronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISpEnginePronunciation {}
impl ISpEnginePronunciation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpEnginePronunciation_Impl, const OFFSET: isize>() -> ISpEnginePronunciation_Vtbl {
        unsafe extern "system" fn Normalize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpEnginePronunciation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: ::windows_core::PCWSTR, pszleftcontext: ::windows_core::PCWSTR, pszrightcontext: ::windows_core::PCWSTR, langid: u16, pnormalizationlist: *mut SPNORMALIZATIONLIST) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Normalize(::core::mem::transmute(&pszword), ::core::mem::transmute(&pszleftcontext), ::core::mem::transmute(&pszrightcontext), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&pnormalizationlist)).into()
        }
        unsafe extern "system" fn GetPronunciations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpEnginePronunciation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: ::windows_core::PCWSTR, pszleftcontext: ::windows_core::PCWSTR, pszrightcontext: ::windows_core::PCWSTR, langid: u16, penginepronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPronunciations(::core::mem::transmute(&pszword), ::core::mem::transmute(&pszleftcontext), ::core::mem::transmute(&pszrightcontext), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&penginepronunciationlist)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Normalize: Normalize::<Identity, Impl, OFFSET>,
            GetPronunciations: GetPronunciations::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpEnginePronunciation as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpEventSink_Impl: Sized {
    fn AddEvents(&self, peventarray: *const SPEVENT, ulcount: u32) -> ::windows_core::Result<()>;
    fn GetEventInterest(&self, pulleventinterest: *mut u64) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ISpEventSink {}
#[cfg(feature = "Win32_Foundation")]
impl ISpEventSink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpEventSink_Impl, const OFFSET: isize>() -> ISpEventSink_Vtbl {
        unsafe extern "system" fn AddEvents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peventarray: *const SPEVENT, ulcount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddEvents(::core::mem::transmute_copy(&peventarray), ::core::mem::transmute_copy(&ulcount)).into()
        }
        unsafe extern "system" fn GetEventInterest<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpEventSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulleventinterest: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEventInterest(::core::mem::transmute_copy(&pulleventinterest)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddEvents: AddEvents::<Identity, Impl, OFFSET>,
            GetEventInterest: GetEventInterest::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpEventSink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpEventSource_Impl: Sized + ISpNotifySource_Impl {
    fn SetInterest(&self, ulleventinterest: u64, ullqueuedinterest: u64) -> ::windows_core::Result<()>;
    fn GetEvents(&self, ulcount: u32, peventarray: *mut SPEVENT, pulfetched: *mut u32) -> ::windows_core::Result<()>;
    fn GetInfo(&self, pinfo: *mut SPEVENTSOURCEINFO) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ISpEventSource {}
#[cfg(feature = "Win32_Foundation")]
impl ISpEventSource_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpEventSource_Impl, const OFFSET: isize>() -> ISpEventSource_Vtbl {
        unsafe extern "system" fn SetInterest<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpEventSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulleventinterest: u64, ullqueuedinterest: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInterest(::core::mem::transmute_copy(&ulleventinterest), ::core::mem::transmute_copy(&ullqueuedinterest)).into()
        }
        unsafe extern "system" fn GetEvents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpEventSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, peventarray: *mut SPEVENT, pulfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEvents(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&peventarray), ::core::mem::transmute_copy(&pulfetched)).into()
        }
        unsafe extern "system" fn GetInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpEventSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut SPEVENTSOURCEINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInfo(::core::mem::transmute_copy(&pinfo)).into()
        }
        Self {
            base__: ISpNotifySource_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetInterest: SetInterest::<Identity, Impl, OFFSET>,
            GetEvents: GetEvents::<Identity, Impl, OFFSET>,
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpEventSource as ::windows_core::ComInterface>::IID || iid == &<ISpNotifySource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpEventSource2_Impl: Sized + ISpEventSource_Impl {
    fn GetEventsEx(&self, ulcount: u32, peventarray: *mut SPEVENTEX, pulfetched: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ISpEventSource2 {}
#[cfg(feature = "Win32_Foundation")]
impl ISpEventSource2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpEventSource2_Impl, const OFFSET: isize>() -> ISpEventSource2_Vtbl {
        unsafe extern "system" fn GetEventsEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpEventSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulcount: u32, peventarray: *mut SPEVENTEX, pulfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEventsEx(::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&peventarray), ::core::mem::transmute_copy(&pulfetched)).into()
        }
        Self { base__: ISpEventSource_Vtbl::new::<Identity, Impl, OFFSET>(), GetEventsEx: GetEventsEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpEventSource2 as ::windows_core::ComInterface>::IID || iid == &<ISpNotifySource as ::windows_core::ComInterface>::IID || iid == &<ISpEventSource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpGrammarBuilder_Impl: Sized {
    fn ResetGrammar(&self, newlanguage: u16) -> ::windows_core::Result<()>;
    fn GetRule(&self, pszrulename: &::windows_core::PCWSTR, dwruleid: u32, dwattributes: u32, fcreateifnotexist: super::super::Foundation::BOOL, phinitialstate: *mut SPSTATEHANDLE) -> ::windows_core::Result<()>;
    fn ClearRule(&self, hstate: SPSTATEHANDLE) -> ::windows_core::Result<()>;
    fn CreateNewState(&self, hstate: SPSTATEHANDLE, phstate: *mut SPSTATEHANDLE) -> ::windows_core::Result<()>;
    fn AddWordTransition(&self, hfromstate: SPSTATEHANDLE, htostate: SPSTATEHANDLE, psz: &::windows_core::PCWSTR, pszseparators: &::windows_core::PCWSTR, ewordtype: SPGRAMMARWORDTYPE, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows_core::Result<()>;
    fn AddRuleTransition(&self, hfromstate: SPSTATEHANDLE, htostate: SPSTATEHANDLE, hrule: SPSTATEHANDLE, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows_core::Result<()>;
    fn AddResource(&self, hrulestate: SPSTATEHANDLE, pszresourcename: &::windows_core::PCWSTR, pszresourcevalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Commit(&self, dwreserved: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpGrammarBuilder {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpGrammarBuilder_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>() -> ISpGrammarBuilder_Vtbl {
        unsafe extern "system" fn ResetGrammar<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newlanguage: u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResetGrammar(::core::mem::transmute_copy(&newlanguage)).into()
        }
        unsafe extern "system" fn GetRule<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrulename: ::windows_core::PCWSTR, dwruleid: u32, dwattributes: u32, fcreateifnotexist: super::super::Foundation::BOOL, phinitialstate: *mut SPSTATEHANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRule(::core::mem::transmute(&pszrulename), ::core::mem::transmute_copy(&dwruleid), ::core::mem::transmute_copy(&dwattributes), ::core::mem::transmute_copy(&fcreateifnotexist), ::core::mem::transmute_copy(&phinitialstate)).into()
        }
        unsafe extern "system" fn ClearRule<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hstate: SPSTATEHANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearRule(::core::mem::transmute_copy(&hstate)).into()
        }
        unsafe extern "system" fn CreateNewState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hstate: SPSTATEHANDLE, phstate: *mut SPSTATEHANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateNewState(::core::mem::transmute_copy(&hstate), ::core::mem::transmute_copy(&phstate)).into()
        }
        unsafe extern "system" fn AddWordTransition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfromstate: SPSTATEHANDLE, htostate: SPSTATEHANDLE, psz: ::windows_core::PCWSTR, pszseparators: ::windows_core::PCWSTR, ewordtype: SPGRAMMARWORDTYPE, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddWordTransition(::core::mem::transmute_copy(&hfromstate), ::core::mem::transmute_copy(&htostate), ::core::mem::transmute(&psz), ::core::mem::transmute(&pszseparators), ::core::mem::transmute_copy(&ewordtype), ::core::mem::transmute_copy(&weight), ::core::mem::transmute_copy(&ppropinfo)).into()
        }
        unsafe extern "system" fn AddRuleTransition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfromstate: SPSTATEHANDLE, htostate: SPSTATEHANDLE, hrule: SPSTATEHANDLE, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRuleTransition(::core::mem::transmute_copy(&hfromstate), ::core::mem::transmute_copy(&htostate), ::core::mem::transmute_copy(&hrule), ::core::mem::transmute_copy(&weight), ::core::mem::transmute_copy(&ppropinfo)).into()
        }
        unsafe extern "system" fn AddResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrulestate: SPSTATEHANDLE, pszresourcename: ::windows_core::PCWSTR, pszresourcevalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddResource(::core::mem::transmute_copy(&hrulestate), ::core::mem::transmute(&pszresourcename), ::core::mem::transmute(&pszresourcevalue)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit(::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ResetGrammar: ResetGrammar::<Identity, Impl, OFFSET>,
            GetRule: GetRule::<Identity, Impl, OFFSET>,
            ClearRule: ClearRule::<Identity, Impl, OFFSET>,
            CreateNewState: CreateNewState::<Identity, Impl, OFFSET>,
            AddWordTransition: AddWordTransition::<Identity, Impl, OFFSET>,
            AddRuleTransition: AddRuleTransition::<Identity, Impl, OFFSET>,
            AddResource: AddResource::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpGrammarBuilder as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"implement\"`*"]
pub trait ISpGrammarBuilder2_Impl: Sized {
    fn AddTextSubset(&self, hfromstate: SPSTATEHANDLE, htostate: SPSTATEHANDLE, psz: &::windows_core::PCWSTR, ematchmode: SPMATCHINGMODE) -> ::windows_core::Result<()>;
    fn SetPhoneticAlphabet(&self, phoneticalphabet: PHONETICALPHABET) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISpGrammarBuilder2 {}
impl ISpGrammarBuilder2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpGrammarBuilder2_Impl, const OFFSET: isize>() -> ISpGrammarBuilder2_Vtbl {
        unsafe extern "system" fn AddTextSubset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpGrammarBuilder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hfromstate: SPSTATEHANDLE, htostate: SPSTATEHANDLE, psz: ::windows_core::PCWSTR, ematchmode: SPMATCHINGMODE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddTextSubset(::core::mem::transmute_copy(&hfromstate), ::core::mem::transmute_copy(&htostate), ::core::mem::transmute(&psz), ::core::mem::transmute_copy(&ematchmode)).into()
        }
        unsafe extern "system" fn SetPhoneticAlphabet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpGrammarBuilder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phoneticalphabet: PHONETICALPHABET) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPhoneticAlphabet(::core::mem::transmute_copy(&phoneticalphabet)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddTextSubset: AddTextSubset::<Identity, Impl, OFFSET>,
            SetPhoneticAlphabet: SetPhoneticAlphabet::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpGrammarBuilder2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"implement\"`*"]
pub trait ISpLexicon_Impl: Sized {
    fn GetPronunciations(&self, pszword: &::windows_core::PCWSTR, langid: u16, dwflags: u32, pwordpronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows_core::Result<()>;
    fn AddPronunciation(&self, pszword: &::windows_core::PCWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> ::windows_core::Result<()>;
    fn RemovePronunciation(&self, pszword: &::windows_core::PCWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> ::windows_core::Result<()>;
    fn GetGeneration(&self, pdwgeneration: *mut u32) -> ::windows_core::Result<()>;
    fn GetGenerationChange(&self, dwflags: u32, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows_core::Result<()>;
    fn GetWords(&self, dwflags: u32, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISpLexicon {}
impl ISpLexicon_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpLexicon_Impl, const OFFSET: isize>() -> ISpLexicon_Vtbl {
        unsafe extern "system" fn GetPronunciations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: ::windows_core::PCWSTR, langid: u16, dwflags: u32, pwordpronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPronunciations(::core::mem::transmute(&pszword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pwordpronunciationlist)).into()
        }
        unsafe extern "system" fn AddPronunciation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: ::windows_core::PCWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPronunciation(::core::mem::transmute(&pszword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&epartofspeech), ::core::mem::transmute_copy(&pszpronunciation)).into()
        }
        unsafe extern "system" fn RemovePronunciation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: ::windows_core::PCWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePronunciation(::core::mem::transmute(&pszword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&epartofspeech), ::core::mem::transmute_copy(&pszpronunciation)).into()
        }
        unsafe extern "system" fn GetGeneration<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGeneration(::core::mem::transmute_copy(&pdwgeneration)).into()
        }
        unsafe extern "system" fn GetGenerationChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGenerationChange(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pwordlist)).into()
        }
        unsafe extern "system" fn GetWords<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWords(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pdwcookie), ::core::mem::transmute_copy(&pwordlist)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPronunciations: GetPronunciations::<Identity, Impl, OFFSET>,
            AddPronunciation: AddPronunciation::<Identity, Impl, OFFSET>,
            RemovePronunciation: RemovePronunciation::<Identity, Impl, OFFSET>,
            GetGeneration: GetGeneration::<Identity, Impl, OFFSET>,
            GetGenerationChange: GetGenerationChange::<Identity, Impl, OFFSET>,
            GetWords: GetWords::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpLexicon as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_Media_Audio\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
pub trait ISpMMSysAudio_Impl: Sized + ISpAudio_Impl {
    fn GetDeviceId(&self, pudeviceid: *mut u32) -> ::windows_core::Result<()>;
    fn SetDeviceId(&self, udeviceid: u32) -> ::windows_core::Result<()>;
    fn GetMMHandle(&self, phandle: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetLineId(&self, pulineid: *mut u32) -> ::windows_core::Result<()>;
    fn SetLineId(&self, ulineid: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for ISpMMSysAudio {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ISpMMSysAudio_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpMMSysAudio_Impl, const OFFSET: isize>() -> ISpMMSysAudio_Vtbl {
        unsafe extern "system" fn GetDeviceId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pudeviceid: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceId(::core::mem::transmute_copy(&pudeviceid)).into()
        }
        unsafe extern "system" fn SetDeviceId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, udeviceid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDeviceId(::core::mem::transmute_copy(&udeviceid)).into()
        }
        unsafe extern "system" fn GetMMHandle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phandle: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMMHandle(::core::mem::transmute_copy(&phandle)).into()
        }
        unsafe extern "system" fn GetLineId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulineid: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLineId(::core::mem::transmute_copy(&pulineid)).into()
        }
        unsafe extern "system" fn SetLineId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulineid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLineId(::core::mem::transmute_copy(&ulineid)).into()
        }
        Self {
            base__: ISpAudio_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDeviceId: GetDeviceId::<Identity, Impl, OFFSET>,
            SetDeviceId: SetDeviceId::<Identity, Impl, OFFSET>,
            GetMMHandle: GetMMHandle::<Identity, Impl, OFFSET>,
            GetLineId: GetLineId::<Identity, Impl, OFFSET>,
            SetLineId: SetLineId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpMMSysAudio as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::ISequentialStream as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IStream as ::windows_core::ComInterface>::IID || iid == &<ISpStreamFormat as ::windows_core::ComInterface>::IID || iid == &<ISpAudio as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpNotifyCallback_Impl: Sized {
    fn NotifyCallback(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpNotifyCallback_Vtbl {
    pub const fn new<Impl: ISpNotifyCallback_Impl>() -> ISpNotifyCallback_Vtbl {
        unsafe extern "system" fn NotifyCallback<Impl: ISpNotifyCallback_Impl>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            this.NotifyCallback(::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        Self { NotifyCallback: NotifyCallback::<Impl> }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Foundation")]
struct ISpNotifyCallback_ImplVtbl<T: ISpNotifyCallback_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Foundation")]
impl<T: ISpNotifyCallback_Impl> ISpNotifyCallback_ImplVtbl<T> {
    const VTABLE: ISpNotifyCallback_Vtbl = ISpNotifyCallback_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Foundation")]
impl ISpNotifyCallback {
    pub fn new<'a, T: ISpNotifyCallback_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ISpNotifyCallback_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::mem::ManuallyDrop::new(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::std::mem::transmute(&this.vtable)) }
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"implement\"`*"]
pub trait ISpNotifySink_Impl: Sized {
    fn Notify(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISpNotifySink {}
impl ISpNotifySink_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpNotifySink_Impl, const OFFSET: isize>() -> ISpNotifySink_Vtbl {
        unsafe extern "system" fn Notify<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpNotifySink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Notify().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Notify: Notify::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpNotifySink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpNotifySource_Impl: Sized {
    fn SetNotifySink(&self, pnotifysink: ::core::option::Option<&ISpNotifySink>) -> ::windows_core::Result<()>;
    fn SetNotifyWindowMessage(&self, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn SetNotifyCallbackFunction(&self, pfncallback: *mut SPNOTIFYCALLBACK, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn SetNotifyCallbackInterface(&self, pspcallback: ::core::option::Option<&ISpNotifyCallback>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn SetNotifyWin32Event(&self) -> ::windows_core::Result<()>;
    fn WaitForNotifyEvent(&self, dwmilliseconds: u32) -> ::windows_core::Result<()>;
    fn GetNotifyEventHandle(&self) -> super::super::Foundation::HANDLE;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ISpNotifySource {}
#[cfg(feature = "Win32_Foundation")]
impl ISpNotifySource_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpNotifySource_Impl, const OFFSET: isize>() -> ISpNotifySource_Vtbl {
        unsafe extern "system" fn SetNotifySink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotifysink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNotifySink(::windows_core::from_raw_borrowed(&pnotifysink)).into()
        }
        unsafe extern "system" fn SetNotifyWindowMessage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNotifyWindowMessage(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn SetNotifyCallbackFunction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfncallback: *mut SPNOTIFYCALLBACK, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNotifyCallbackFunction(::core::mem::transmute_copy(&pfncallback), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn SetNotifyCallbackInterface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pspcallback: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNotifyCallbackInterface(::windows_core::from_raw_borrowed(&pspcallback), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn SetNotifyWin32Event<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNotifyWin32Event().into()
        }
        unsafe extern "system" fn WaitForNotifyEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WaitForNotifyEvent(::core::mem::transmute_copy(&dwmilliseconds)).into()
        }
        unsafe extern "system" fn GetNotifyEventHandle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNotifyEventHandle()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetNotifySink: SetNotifySink::<Identity, Impl, OFFSET>,
            SetNotifyWindowMessage: SetNotifyWindowMessage::<Identity, Impl, OFFSET>,
            SetNotifyCallbackFunction: SetNotifyCallbackFunction::<Identity, Impl, OFFSET>,
            SetNotifyCallbackInterface: SetNotifyCallbackInterface::<Identity, Impl, OFFSET>,
            SetNotifyWin32Event: SetNotifyWin32Event::<Identity, Impl, OFFSET>,
            WaitForNotifyEvent: WaitForNotifyEvent::<Identity, Impl, OFFSET>,
            GetNotifyEventHandle: GetNotifyEventHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpNotifySource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpNotifyTranslator_Impl: Sized + ISpNotifySink_Impl {
    fn InitWindowMessage(&self, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn InitCallback(&self, pfncallback: *mut SPNOTIFYCALLBACK, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn InitSpNotifyCallback(&self, pspcallback: ::core::option::Option<&ISpNotifyCallback>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn InitWin32Event(&self, hevent: super::super::Foundation::HANDLE, fclosehandleonrelease: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Wait(&self, dwmilliseconds: u32) -> ::windows_core::Result<()>;
    fn GetEventHandle(&self) -> super::super::Foundation::HANDLE;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ISpNotifyTranslator {}
#[cfg(feature = "Win32_Foundation")]
impl ISpNotifyTranslator_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpNotifyTranslator_Impl, const OFFSET: isize>() -> ISpNotifyTranslator_Vtbl {
        unsafe extern "system" fn InitWindowMessage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitWindowMessage(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn InitCallback<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfncallback: *mut SPNOTIFYCALLBACK, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitCallback(::core::mem::transmute_copy(&pfncallback), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn InitSpNotifyCallback<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pspcallback: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitSpNotifyCallback(::windows_core::from_raw_borrowed(&pspcallback), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn InitWin32Event<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, fclosehandleonrelease: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitWin32Event(::core::mem::transmute_copy(&hevent), ::core::mem::transmute_copy(&fclosehandleonrelease)).into()
        }
        unsafe extern "system" fn Wait<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmilliseconds: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Wait(::core::mem::transmute_copy(&dwmilliseconds)).into()
        }
        unsafe extern "system" fn GetEventHandle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEventHandle()
        }
        Self {
            base__: ISpNotifySink_Vtbl::new::<Identity, Impl, OFFSET>(),
            InitWindowMessage: InitWindowMessage::<Identity, Impl, OFFSET>,
            InitCallback: InitCallback::<Identity, Impl, OFFSET>,
            InitSpNotifyCallback: InitSpNotifyCallback::<Identity, Impl, OFFSET>,
            InitWin32Event: InitWin32Event::<Identity, Impl, OFFSET>,
            Wait: Wait::<Identity, Impl, OFFSET>,
            GetEventHandle: GetEventHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpNotifyTranslator as ::windows_core::ComInterface>::IID || iid == &<ISpNotifySink as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpObjectToken_Impl: Sized + ISpDataKey_Impl {
    fn SetId(&self, pszcategoryid: &::windows_core::PCWSTR, psztokenid: &::windows_core::PCWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetId(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetCategory(&self) -> ::windows_core::Result<ISpObjectTokenCategory>;
    fn CreateInstance(&self, punkouter: ::core::option::Option<&::windows_core::IUnknown>, dwclscontext: u32, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetStorageFileName(&self, clsidcaller: *const ::windows_core::GUID, pszvaluename: &::windows_core::PCWSTR, pszfilenamespecifier: &::windows_core::PCWSTR, nfolder: u32) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn RemoveStorageFileName(&self, clsidcaller: *const ::windows_core::GUID, pszkeyname: &::windows_core::PCWSTR, fdeletefile: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Remove(&self, pclsidcaller: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn IsUISupported(&self, psztypeofui: &::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: ::core::option::Option<&::windows_core::IUnknown>, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn DisplayUI(&self, hwndparent: super::super::Foundation::HWND, psztitle: &::windows_core::PCWSTR, psztypeofui: &::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn MatchesAttributes(&self, pszattributes: &::windows_core::PCWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ISpObjectToken {}
#[cfg(feature = "Win32_Foundation")]
impl ISpObjectToken_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpObjectToken_Impl, const OFFSET: isize>() -> ISpObjectToken_Vtbl {
        unsafe extern "system" fn SetId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcategoryid: ::windows_core::PCWSTR, psztokenid: ::windows_core::PCWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetId(::core::mem::transmute(&pszcategoryid), ::core::mem::transmute(&psztokenid), ::core::mem::transmute_copy(&fcreateifnotexist)).into()
        }
        unsafe extern "system" fn GetId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemtokenid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszcomemtokenid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptokencategory: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCategory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptokencategory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateInstance(::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&dwclscontext), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobject)).into()
        }
        unsafe extern "system" fn GetStorageFileName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidcaller: *const ::windows_core::GUID, pszvaluename: ::windows_core::PCWSTR, pszfilenamespecifier: ::windows_core::PCWSTR, nfolder: u32, ppszfilepath: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStorageFileName(::core::mem::transmute_copy(&clsidcaller), ::core::mem::transmute(&pszvaluename), ::core::mem::transmute(&pszfilenamespecifier), ::core::mem::transmute_copy(&nfolder)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszfilepath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStorageFileName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidcaller: *const ::windows_core::GUID, pszkeyname: ::windows_core::PCWSTR, fdeletefile: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveStorageFileName(::core::mem::transmute_copy(&clsidcaller), ::core::mem::transmute(&pszkeyname), ::core::mem::transmute_copy(&fdeletefile)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsidcaller: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute_copy(&pclsidcaller)).into()
        }
        unsafe extern "system" fn IsUISupported<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztypeofui: ::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: *mut ::core::ffi::c_void, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsUISupported(::core::mem::transmute(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata), ::windows_core::from_raw_borrowed(&punkobject), ::core::mem::transmute_copy(&pfsupported)).into()
        }
        unsafe extern "system" fn DisplayUI<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, psztitle: ::windows_core::PCWSTR, psztypeofui: ::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, punkobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisplayUI(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&psztitle), ::core::mem::transmute(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata), ::windows_core::from_raw_borrowed(&punkobject)).into()
        }
        unsafe extern "system" fn MatchesAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszattributes: ::windows_core::PCWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MatchesAttributes(::core::mem::transmute(&pszattributes), ::core::mem::transmute_copy(&pfmatches)).into()
        }
        Self {
            base__: ISpDataKey_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetId: SetId::<Identity, Impl, OFFSET>,
            GetId: GetId::<Identity, Impl, OFFSET>,
            GetCategory: GetCategory::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            GetStorageFileName: GetStorageFileName::<Identity, Impl, OFFSET>,
            RemoveStorageFileName: RemoveStorageFileName::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            IsUISupported: IsUISupported::<Identity, Impl, OFFSET>,
            DisplayUI: DisplayUI::<Identity, Impl, OFFSET>,
            MatchesAttributes: MatchesAttributes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpObjectToken as ::windows_core::ComInterface>::IID || iid == &<ISpDataKey as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpObjectTokenCategory_Impl: Sized + ISpDataKey_Impl {
    fn SetId(&self, pszcategoryid: &::windows_core::PCWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetId(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetDataKey(&self, spdkl: SPDATAKEYLOCATION) -> ::windows_core::Result<ISpDataKey>;
    fn EnumTokens(&self, pzsreqattribs: &::windows_core::PCWSTR, pszoptattribs: &::windows_core::PCWSTR) -> ::windows_core::Result<IEnumSpObjectTokens>;
    fn SetDefaultTokenId(&self, psztokenid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetDefaultTokenId(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ISpObjectTokenCategory {}
#[cfg(feature = "Win32_Foundation")]
impl ISpObjectTokenCategory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpObjectTokenCategory_Impl, const OFFSET: isize>() -> ISpObjectTokenCategory_Vtbl {
        unsafe extern "system" fn SetId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcategoryid: ::windows_core::PCWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetId(::core::mem::transmute(&pszcategoryid), ::core::mem::transmute_copy(&fcreateifnotexist)).into()
        }
        unsafe extern "system" fn GetId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemcategoryid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszcomemcategoryid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, spdkl: SPDATAKEYLOCATION, ppdatakey: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDataKey(::core::mem::transmute_copy(&spdkl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdatakey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumTokens<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pzsreqattribs: ::windows_core::PCWSTR, pszoptattribs: ::windows_core::PCWSTR, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumTokens(::core::mem::transmute(&pzsreqattribs), ::core::mem::transmute(&pszoptattribs)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultTokenId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztokenid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultTokenId(::core::mem::transmute(&psztokenid)).into()
        }
        unsafe extern "system" fn GetDefaultTokenId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemtokenid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefaultTokenId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszcomemtokenid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISpDataKey_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetId: SetId::<Identity, Impl, OFFSET>,
            GetId: GetId::<Identity, Impl, OFFSET>,
            GetDataKey: GetDataKey::<Identity, Impl, OFFSET>,
            EnumTokens: EnumTokens::<Identity, Impl, OFFSET>,
            SetDefaultTokenId: SetDefaultTokenId::<Identity, Impl, OFFSET>,
            GetDefaultTokenId: GetDefaultTokenId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpObjectTokenCategory as ::windows_core::ComInterface>::IID || iid == &<ISpDataKey as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpObjectTokenInit_Impl: Sized + ISpObjectToken_Impl {
    fn InitFromDataKey(&self, pszcategoryid: &::windows_core::PCWSTR, psztokenid: &::windows_core::PCWSTR, pdatakey: ::core::option::Option<&ISpDataKey>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ISpObjectTokenInit {}
#[cfg(feature = "Win32_Foundation")]
impl ISpObjectTokenInit_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpObjectTokenInit_Impl, const OFFSET: isize>() -> ISpObjectTokenInit_Vtbl {
        unsafe extern "system" fn InitFromDataKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpObjectTokenInit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcategoryid: ::windows_core::PCWSTR, psztokenid: ::windows_core::PCWSTR, pdatakey: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitFromDataKey(::core::mem::transmute(&pszcategoryid), ::core::mem::transmute(&psztokenid), ::windows_core::from_raw_borrowed(&pdatakey)).into()
        }
        Self { base__: ISpObjectToken_Vtbl::new::<Identity, Impl, OFFSET>(), InitFromDataKey: InitFromDataKey::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpObjectTokenInit as ::windows_core::ComInterface>::IID || iid == &<ISpDataKey as ::windows_core::ComInterface>::IID || iid == &<ISpObjectToken as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"implement\"`*"]
pub trait ISpObjectWithToken_Impl: Sized {
    fn SetObjectToken(&self, ptoken: ::core::option::Option<&ISpObjectToken>) -> ::windows_core::Result<()>;
    fn GetObjectToken(&self) -> ::windows_core::Result<ISpObjectToken>;
}
impl ::windows_core::RuntimeName for ISpObjectWithToken {}
impl ISpObjectWithToken_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpObjectWithToken_Impl, const OFFSET: isize>() -> ISpObjectWithToken_Vtbl {
        unsafe extern "system" fn SetObjectToken<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpObjectWithToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptoken: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetObjectToken(::windows_core::from_raw_borrowed(&ptoken)).into()
        }
        unsafe extern "system" fn GetObjectToken<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpObjectWithToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptoken: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetObjectToken() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptoken, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetObjectToken: SetObjectToken::<Identity, Impl, OFFSET>,
            GetObjectToken: GetObjectToken::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpObjectWithToken as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"implement\"`*"]
pub trait ISpPhoneConverter_Impl: Sized + ISpObjectWithToken_Impl {
    fn PhoneToId(&self, pszphone: &::windows_core::PCWSTR) -> ::windows_core::Result<u16>;
    fn IdToPhone(&self, pid: *const u16, pszphone: ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISpPhoneConverter {}
impl ISpPhoneConverter_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhoneConverter_Impl, const OFFSET: isize>() -> ISpPhoneConverter_Vtbl {
        unsafe extern "system" fn PhoneToId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhoneConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszphone: ::windows_core::PCWSTR, pid: *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PhoneToId(::core::mem::transmute(&pszphone)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IdToPhone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhoneConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *const u16, pszphone: ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IdToPhone(::core::mem::transmute_copy(&pid), ::core::mem::transmute_copy(&pszphone)).into()
        }
        Self {
            base__: ISpObjectWithToken_Vtbl::new::<Identity, Impl, OFFSET>(),
            PhoneToId: PhoneToId::<Identity, Impl, OFFSET>,
            IdToPhone: IdToPhone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpPhoneConverter as ::windows_core::ComInterface>::IID || iid == &<ISpObjectWithToken as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpPhoneticAlphabetConverter_Impl: Sized {
    fn GetLangId(&self) -> ::windows_core::Result<u16>;
    fn SetLangId(&self, langid: u16) -> ::windows_core::Result<()>;
    fn SAPI2UPS(&self, pszsapiid: *const u16, pszupsid: *mut u16, cmaxlength: u32) -> ::windows_core::Result<()>;
    fn UPS2SAPI(&self, pszupsid: *const u16, pszsapiid: *mut u16, cmaxlength: u32) -> ::windows_core::Result<()>;
    fn GetMaxConvertLength(&self, csrclength: u32, bsapi2ups: super::super::Foundation::BOOL) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ISpPhoneticAlphabetConverter {}
#[cfg(feature = "Win32_Foundation")]
impl ISpPhoneticAlphabetConverter_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhoneticAlphabetConverter_Impl, const OFFSET: isize>() -> ISpPhoneticAlphabetConverter_Vtbl {
        unsafe extern "system" fn GetLangId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhoneticAlphabetConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plangid: *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLangId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plangid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLangId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhoneticAlphabetConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLangId(::core::mem::transmute_copy(&langid)).into()
        }
        unsafe extern "system" fn SAPI2UPS<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhoneticAlphabetConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsapiid: *const u16, pszupsid: *mut u16, cmaxlength: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SAPI2UPS(::core::mem::transmute_copy(&pszsapiid), ::core::mem::transmute_copy(&pszupsid), ::core::mem::transmute_copy(&cmaxlength)).into()
        }
        unsafe extern "system" fn UPS2SAPI<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhoneticAlphabetConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszupsid: *const u16, pszsapiid: *mut u16, cmaxlength: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UPS2SAPI(::core::mem::transmute_copy(&pszupsid), ::core::mem::transmute_copy(&pszsapiid), ::core::mem::transmute_copy(&cmaxlength)).into()
        }
        unsafe extern "system" fn GetMaxConvertLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhoneticAlphabetConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, csrclength: u32, bsapi2ups: super::super::Foundation::BOOL, pcmaxdestlength: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxConvertLength(::core::mem::transmute_copy(&csrclength), ::core::mem::transmute_copy(&bsapi2ups)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcmaxdestlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLangId: GetLangId::<Identity, Impl, OFFSET>,
            SetLangId: SetLangId::<Identity, Impl, OFFSET>,
            SAPI2UPS: SAPI2UPS::<Identity, Impl, OFFSET>,
            UPS2SAPI: UPS2SAPI::<Identity, Impl, OFFSET>,
            GetMaxConvertLength: GetMaxConvertLength::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpPhoneticAlphabetConverter as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpPhoneticAlphabetSelection_Impl: Sized {
    fn IsAlphabetUPS(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetAlphabetToUPS(&self, fforceups: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ISpPhoneticAlphabetSelection {}
#[cfg(feature = "Win32_Foundation")]
impl ISpPhoneticAlphabetSelection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhoneticAlphabetSelection_Impl, const OFFSET: isize>() -> ISpPhoneticAlphabetSelection_Vtbl {
        unsafe extern "system" fn IsAlphabetUPS<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhoneticAlphabetSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisups: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsAlphabetUPS() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisups, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlphabetToUPS<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhoneticAlphabetSelection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fforceups: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAlphabetToUPS(::core::mem::transmute_copy(&fforceups)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsAlphabetUPS: IsAlphabetUPS::<Identity, Impl, OFFSET>,
            SetAlphabetToUPS: SetAlphabetToUPS::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpPhoneticAlphabetSelection as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpPhrase_Impl: Sized {
    fn GetPhrase(&self) -> ::windows_core::Result<*mut SPPHRASE>;
    fn GetSerializedPhrase(&self) -> ::windows_core::Result<*mut SPSERIALIZEDPHRASE>;
    fn GetText(&self, ulstart: u32, ulcount: u32, fusetextreplacements: super::super::Foundation::BOOL, ppszcomemtext: *mut ::windows_core::PWSTR, pbdisplayattributes: *mut u8) -> ::windows_core::Result<()>;
    fn Discard(&self, dwvaluetypes: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpPhrase {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpPhrase_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhrase_Impl, const OFFSET: isize>() -> ISpPhrase_Vtbl {
        unsafe extern "system" fn GetPhrase<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhrase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomemphrase: *mut *mut SPPHRASE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPhrase() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcomemphrase, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerializedPhrase<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhrase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomemphrase: *mut *mut SPSERIALIZEDPHRASE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSerializedPhrase() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcomemphrase, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhrase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstart: u32, ulcount: u32, fusetextreplacements: super::super::Foundation::BOOL, ppszcomemtext: *mut ::windows_core::PWSTR, pbdisplayattributes: *mut u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetText(::core::mem::transmute_copy(&ulstart), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&fusetextreplacements), ::core::mem::transmute_copy(&ppszcomemtext), ::core::mem::transmute_copy(&pbdisplayattributes)).into()
        }
        unsafe extern "system" fn Discard<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhrase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwvaluetypes: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Discard(::core::mem::transmute_copy(&dwvaluetypes)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPhrase: GetPhrase::<Identity, Impl, OFFSET>,
            GetSerializedPhrase: GetSerializedPhrase::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            Discard: Discard::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpPhrase as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpPhrase2_Impl: Sized + ISpPhrase_Impl {
    fn GetXMLResult(&self, ppszcomemxmlresult: *mut ::windows_core::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows_core::Result<()>;
    fn GetXMLErrorInfo(&self, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows_core::Result<()>;
    fn GetAudio(&self, ulstartelement: u32, celements: u32) -> ::windows_core::Result<ISpStreamFormat>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpPhrase2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpPhrase2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhrase2_Impl, const OFFSET: isize>() -> ISpPhrase2_Vtbl {
        unsafe extern "system" fn GetXMLResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhrase2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemxmlresult: *mut ::windows_core::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetXMLResult(::core::mem::transmute_copy(&ppszcomemxmlresult), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn GetXMLErrorInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhrase2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetXMLErrorInfo(::core::mem::transmute_copy(&psemanticerrorinfo)).into()
        }
        unsafe extern "system" fn GetAudio<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhrase2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAudio(::core::mem::transmute_copy(&ulstartelement), ::core::mem::transmute_copy(&celements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISpPhrase_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetXMLResult: GetXMLResult::<Identity, Impl, OFFSET>,
            GetXMLErrorInfo: GetXMLErrorInfo::<Identity, Impl, OFFSET>,
            GetAudio: GetAudio::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpPhrase2 as ::windows_core::ComInterface>::IID || iid == &<ISpPhrase as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpPhraseAlt_Impl: Sized + ISpPhrase_Impl {
    fn GetAltInfo(&self, ppparent: *mut ::core::option::Option<ISpPhrase>, pulstartelementinparent: *mut u32, pcelementsinparent: *mut u32, pcelementsinalt: *mut u32) -> ::windows_core::Result<()>;
    fn Commit(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpPhraseAlt {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpPhraseAlt_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhraseAlt_Impl, const OFFSET: isize>() -> ISpPhraseAlt_Vtbl {
        unsafe extern "system" fn GetAltInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhraseAlt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparent: *mut *mut ::core::ffi::c_void, pulstartelementinparent: *mut u32, pcelementsinparent: *mut u32, pcelementsinalt: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAltInfo(::core::mem::transmute_copy(&ppparent), ::core::mem::transmute_copy(&pulstartelementinparent), ::core::mem::transmute_copy(&pcelementsinparent), ::core::mem::transmute_copy(&pcelementsinalt)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpPhraseAlt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit().into()
        }
        Self {
            base__: ISpPhrase_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetAltInfo: GetAltInfo::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpPhraseAlt as ::windows_core::ComInterface>::IID || iid == &<ISpPhrase as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"implement\"`*"]
pub trait ISpProperties_Impl: Sized {
    fn SetPropertyNum(&self, pname: &::windows_core::PCWSTR, lvalue: i32) -> ::windows_core::Result<()>;
    fn GetPropertyNum(&self, pname: &::windows_core::PCWSTR, plvalue: *mut i32) -> ::windows_core::Result<()>;
    fn SetPropertyString(&self, pname: &::windows_core::PCWSTR, pvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetPropertyString(&self, pname: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::RuntimeName for ISpProperties {}
impl ISpProperties_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpProperties_Impl, const OFFSET: isize>() -> ISpProperties_Vtbl {
        unsafe extern "system" fn SetPropertyNum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: ::windows_core::PCWSTR, lvalue: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPropertyNum(::core::mem::transmute(&pname), ::core::mem::transmute_copy(&lvalue)).into()
        }
        unsafe extern "system" fn GetPropertyNum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: ::windows_core::PCWSTR, plvalue: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyNum(::core::mem::transmute(&pname), ::core::mem::transmute_copy(&plvalue)).into()
        }
        unsafe extern "system" fn SetPropertyString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: ::windows_core::PCWSTR, pvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPropertyString(::core::mem::transmute(&pname), ::core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn GetPropertyString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pname: ::windows_core::PCWSTR, ppcomemvalue: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertyString(::core::mem::transmute(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcomemvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPropertyNum: SetPropertyNum::<Identity, Impl, OFFSET>,
            GetPropertyNum: GetPropertyNum::<Identity, Impl, OFFSET>,
            SetPropertyString: SetPropertyString::<Identity, Impl, OFFSET>,
            GetPropertyString: GetPropertyString::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpProperties as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_Media_Audio\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio"))]
pub trait ISpRecoContext_Impl: Sized + ISpEventSource_Impl {
    fn GetRecognizer(&self) -> ::windows_core::Result<ISpRecognizer>;
    fn CreateGrammar(&self, ullgrammarid: u64) -> ::windows_core::Result<ISpRecoGrammar>;
    fn GetStatus(&self, pstatus: *mut SPRECOCONTEXTSTATUS) -> ::windows_core::Result<()>;
    fn GetMaxAlternates(&self, pcalternates: *mut u32) -> ::windows_core::Result<()>;
    fn SetMaxAlternates(&self, calternates: u32) -> ::windows_core::Result<()>;
    fn SetAudioOptions(&self, options: SPAUDIOOPTIONS, paudioformatid: *const ::windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn GetAudioOptions(&self, poptions: *mut SPAUDIOOPTIONS, paudioformatid: *mut ::windows_core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn DeserializeResult(&self, pserializedresult: *const SPSERIALIZEDRESULT) -> ::windows_core::Result<ISpRecoResult>;
    fn Bookmark(&self, options: SPBOOKMARKOPTIONS, ullstreamposition: u64, lparamevent: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn SetAdaptationData(&self, padaptationdata: &::windows_core::PCWSTR, cch: u32) -> ::windows_core::Result<()>;
    fn Pause(&self, dwreserved: u32) -> ::windows_core::Result<()>;
    fn Resume(&self, dwreserved: u32) -> ::windows_core::Result<()>;
    fn SetVoice(&self, pvoice: ::core::option::Option<&ISpVoice>, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetVoice(&self) -> ::windows_core::Result<ISpVoice>;
    fn SetVoicePurgeEvent(&self, ulleventinterest: u64) -> ::windows_core::Result<()>;
    fn GetVoicePurgeEvent(&self, pulleventinterest: *mut u64) -> ::windows_core::Result<()>;
    fn SetContextState(&self, econtextstate: SPCONTEXTSTATE) -> ::windows_core::Result<()>;
    fn GetContextState(&self, pecontextstate: *mut SPCONTEXTSTATE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio"))]
impl ::windows_core::RuntimeName for ISpRecoContext {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio"))]
impl ISpRecoContext_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: isize>() -> ISpRecoContext_Vtbl {
        unsafe extern "system" fn GetRecognizer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprecognizer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRecognizer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprecognizer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGrammar<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullgrammarid: u64, ppgrammar: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateGrammar(::core::mem::transmute_copy(&ullgrammarid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgrammar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut SPRECOCONTEXTSTATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStatus(::core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn GetMaxAlternates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcalternates: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMaxAlternates(::core::mem::transmute_copy(&pcalternates)).into()
        }
        unsafe extern "system" fn SetMaxAlternates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, calternates: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxAlternates(::core::mem::transmute_copy(&calternates)).into()
        }
        unsafe extern "system" fn SetAudioOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: SPAUDIOOPTIONS, paudioformatid: *const ::windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAudioOptions(::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&paudioformatid), ::core::mem::transmute_copy(&pwaveformatex)).into()
        }
        unsafe extern "system" fn GetAudioOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poptions: *mut SPAUDIOOPTIONS, paudioformatid: *mut ::windows_core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAudioOptions(::core::mem::transmute_copy(&poptions), ::core::mem::transmute_copy(&paudioformatid), ::core::mem::transmute_copy(&ppcomemwfex)).into()
        }
        unsafe extern "system" fn DeserializeResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserializedresult: *const SPSERIALIZEDRESULT, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeserializeResult(::core::mem::transmute_copy(&pserializedresult)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bookmark<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: SPBOOKMARKOPTIONS, ullstreamposition: u64, lparamevent: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Bookmark(::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&ullstreamposition), ::core::mem::transmute_copy(&lparamevent)).into()
        }
        unsafe extern "system" fn SetAdaptationData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padaptationdata: ::windows_core::PCWSTR, cch: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAdaptationData(::core::mem::transmute(&padaptationdata), ::core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Pause(::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resume(::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SetVoice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvoice: *mut ::core::ffi::c_void, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVoice(::windows_core::from_raw_borrowed(&pvoice), ::core::mem::transmute_copy(&fallowformatchanges)).into()
        }
        unsafe extern "system" fn GetVoice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvoice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVoice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvoice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVoicePurgeEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulleventinterest: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVoicePurgeEvent(::core::mem::transmute_copy(&ulleventinterest)).into()
        }
        unsafe extern "system" fn GetVoicePurgeEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulleventinterest: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVoicePurgeEvent(::core::mem::transmute_copy(&pulleventinterest)).into()
        }
        unsafe extern "system" fn SetContextState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, econtextstate: SPCONTEXTSTATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContextState(::core::mem::transmute_copy(&econtextstate)).into()
        }
        unsafe extern "system" fn GetContextState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pecontextstate: *mut SPCONTEXTSTATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetContextState(::core::mem::transmute_copy(&pecontextstate)).into()
        }
        Self {
            base__: ISpEventSource_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetRecognizer: GetRecognizer::<Identity, Impl, OFFSET>,
            CreateGrammar: CreateGrammar::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetMaxAlternates: GetMaxAlternates::<Identity, Impl, OFFSET>,
            SetMaxAlternates: SetMaxAlternates::<Identity, Impl, OFFSET>,
            SetAudioOptions: SetAudioOptions::<Identity, Impl, OFFSET>,
            GetAudioOptions: GetAudioOptions::<Identity, Impl, OFFSET>,
            DeserializeResult: DeserializeResult::<Identity, Impl, OFFSET>,
            Bookmark: Bookmark::<Identity, Impl, OFFSET>,
            SetAdaptationData: SetAdaptationData::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            SetVoice: SetVoice::<Identity, Impl, OFFSET>,
            GetVoice: GetVoice::<Identity, Impl, OFFSET>,
            SetVoicePurgeEvent: SetVoicePurgeEvent::<Identity, Impl, OFFSET>,
            GetVoicePurgeEvent: GetVoicePurgeEvent::<Identity, Impl, OFFSET>,
            SetContextState: SetContextState::<Identity, Impl, OFFSET>,
            GetContextState: GetContextState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpRecoContext as ::windows_core::ComInterface>::IID || iid == &<ISpNotifySource as ::windows_core::ComInterface>::IID || iid == &<ISpEventSource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"implement\"`*"]
pub trait ISpRecoContext2_Impl: Sized {
    fn SetGrammarOptions(&self, egrammaroptions: u32) -> ::windows_core::Result<()>;
    fn GetGrammarOptions(&self, pegrammaroptions: *mut u32) -> ::windows_core::Result<()>;
    fn SetAdaptationData2(&self, padaptationdata: &::windows_core::PCWSTR, cch: u32, ptopicname: &::windows_core::PCWSTR, eadaptationsettings: u32, erelevance: SPADAPTATIONRELEVANCE) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISpRecoContext2 {}
impl ISpRecoContext2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoContext2_Impl, const OFFSET: isize>() -> ISpRecoContext2_Vtbl {
        unsafe extern "system" fn SetGrammarOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, egrammaroptions: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGrammarOptions(::core::mem::transmute_copy(&egrammaroptions)).into()
        }
        unsafe extern "system" fn GetGrammarOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pegrammaroptions: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGrammarOptions(::core::mem::transmute_copy(&pegrammaroptions)).into()
        }
        unsafe extern "system" fn SetAdaptationData2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padaptationdata: ::windows_core::PCWSTR, cch: u32, ptopicname: ::windows_core::PCWSTR, eadaptationsettings: u32, erelevance: SPADAPTATIONRELEVANCE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAdaptationData2(::core::mem::transmute(&padaptationdata), ::core::mem::transmute_copy(&cch), ::core::mem::transmute(&ptopicname), ::core::mem::transmute_copy(&eadaptationsettings), ::core::mem::transmute_copy(&erelevance)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGrammarOptions: SetGrammarOptions::<Identity, Impl, OFFSET>,
            GetGrammarOptions: GetGrammarOptions::<Identity, Impl, OFFSET>,
            SetAdaptationData2: SetAdaptationData2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpRecoContext2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpRecoGrammar_Impl: Sized + ISpGrammarBuilder_Impl {
    fn GetGrammarId(&self, pullgrammarid: *mut u64) -> ::windows_core::Result<()>;
    fn GetRecoContext(&self) -> ::windows_core::Result<ISpRecoContext>;
    fn LoadCmdFromFile(&self, pszfilename: &::windows_core::PCWSTR, options: SPLOADOPTIONS) -> ::windows_core::Result<()>;
    fn LoadCmdFromObject(&self, rcid: *const ::windows_core::GUID, pszgrammarname: &::windows_core::PCWSTR, options: SPLOADOPTIONS) -> ::windows_core::Result<()>;
    fn LoadCmdFromResource(&self, hmodule: super::super::Foundation::HMODULE, pszresourcename: &::windows_core::PCWSTR, pszresourcetype: &::windows_core::PCWSTR, wlanguage: u16, options: SPLOADOPTIONS) -> ::windows_core::Result<()>;
    fn LoadCmdFromMemory(&self, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS) -> ::windows_core::Result<()>;
    fn LoadCmdFromProprietaryGrammar(&self, rguidparam: *const ::windows_core::GUID, pszstringparam: &::windows_core::PCWSTR, pvdataprarm: *const ::core::ffi::c_void, cbdatasize: u32, options: SPLOADOPTIONS) -> ::windows_core::Result<()>;
    fn SetRuleState(&self, pszname: &::windows_core::PCWSTR, preserved: *mut ::core::ffi::c_void, newstate: SPRULESTATE) -> ::windows_core::Result<()>;
    fn SetRuleIdState(&self, ulruleid: u32, newstate: SPRULESTATE) -> ::windows_core::Result<()>;
    fn LoadDictation(&self, psztopicname: &::windows_core::PCWSTR, options: SPLOADOPTIONS) -> ::windows_core::Result<()>;
    fn UnloadDictation(&self) -> ::windows_core::Result<()>;
    fn SetDictationState(&self, newstate: SPRULESTATE) -> ::windows_core::Result<()>;
    fn SetWordSequenceData(&self, ptext: &::windows_core::PCWSTR, cchtext: u32, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows_core::Result<()>;
    fn SetTextSelection(&self, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows_core::Result<()>;
    fn IsPronounceable(&self, pszword: &::windows_core::PCWSTR, pwordpronounceable: *mut SPWORDPRONOUNCEABLE) -> ::windows_core::Result<()>;
    fn SetGrammarState(&self, egrammarstate: SPGRAMMARSTATE) -> ::windows_core::Result<()>;
    fn SaveCmd(&self, pstream: ::core::option::Option<&super::super::System::Com::IStream>, ppszcomemerrortext: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetGrammarState(&self, pegrammarstate: *mut SPGRAMMARSTATE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpRecoGrammar {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpRecoGrammar_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>() -> ISpRecoGrammar_Vtbl {
        unsafe extern "system" fn GetGrammarId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullgrammarid: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGrammarId(::core::mem::transmute_copy(&pullgrammarid)).into()
        }
        unsafe extern "system" fn GetRecoContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprecoctxt: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRecoContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprecoctxt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadCmdFromFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: ::windows_core::PCWSTR, options: SPLOADOPTIONS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadCmdFromFile(::core::mem::transmute(&pszfilename), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn LoadCmdFromObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rcid: *const ::windows_core::GUID, pszgrammarname: ::windows_core::PCWSTR, options: SPLOADOPTIONS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadCmdFromObject(::core::mem::transmute_copy(&rcid), ::core::mem::transmute(&pszgrammarname), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn LoadCmdFromResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmodule: super::super::Foundation::HMODULE, pszresourcename: ::windows_core::PCWSTR, pszresourcetype: ::windows_core::PCWSTR, wlanguage: u16, options: SPLOADOPTIONS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadCmdFromResource(::core::mem::transmute_copy(&hmodule), ::core::mem::transmute(&pszresourcename), ::core::mem::transmute(&pszresourcetype), ::core::mem::transmute_copy(&wlanguage), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn LoadCmdFromMemory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadCmdFromMemory(::core::mem::transmute_copy(&pgrammar), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn LoadCmdFromProprietaryGrammar<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidparam: *const ::windows_core::GUID, pszstringparam: ::windows_core::PCWSTR, pvdataprarm: *const ::core::ffi::c_void, cbdatasize: u32, options: SPLOADOPTIONS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadCmdFromProprietaryGrammar(::core::mem::transmute_copy(&rguidparam), ::core::mem::transmute(&pszstringparam), ::core::mem::transmute_copy(&pvdataprarm), ::core::mem::transmute_copy(&cbdatasize), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn SetRuleState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, preserved: *mut ::core::ffi::c_void, newstate: SPRULESTATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRuleState(::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&preserved), ::core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn SetRuleIdState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulruleid: u32, newstate: SPRULESTATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRuleIdState(::core::mem::transmute_copy(&ulruleid), ::core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn LoadDictation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztopicname: ::windows_core::PCWSTR, options: SPLOADOPTIONS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadDictation(::core::mem::transmute(&psztopicname), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn UnloadDictation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnloadDictation().into()
        }
        unsafe extern "system" fn SetDictationState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: SPRULESTATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDictationState(::core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn SetWordSequenceData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptext: ::windows_core::PCWSTR, cchtext: u32, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWordSequenceData(::core::mem::transmute(&ptext), ::core::mem::transmute_copy(&cchtext), ::core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn SetTextSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const SPTEXTSELECTIONINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTextSelection(::core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn IsPronounceable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszword: ::windows_core::PCWSTR, pwordpronounceable: *mut SPWORDPRONOUNCEABLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsPronounceable(::core::mem::transmute(&pszword), ::core::mem::transmute_copy(&pwordpronounceable)).into()
        }
        unsafe extern "system" fn SetGrammarState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, egrammarstate: SPGRAMMARSTATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGrammarState(::core::mem::transmute_copy(&egrammarstate)).into()
        }
        unsafe extern "system" fn SaveCmd<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, ppszcomemerrortext: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveCmd(::windows_core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&ppszcomemerrortext)).into()
        }
        unsafe extern "system" fn GetGrammarState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pegrammarstate: *mut SPGRAMMARSTATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGrammarState(::core::mem::transmute_copy(&pegrammarstate)).into()
        }
        Self {
            base__: ISpGrammarBuilder_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetGrammarId: GetGrammarId::<Identity, Impl, OFFSET>,
            GetRecoContext: GetRecoContext::<Identity, Impl, OFFSET>,
            LoadCmdFromFile: LoadCmdFromFile::<Identity, Impl, OFFSET>,
            LoadCmdFromObject: LoadCmdFromObject::<Identity, Impl, OFFSET>,
            LoadCmdFromResource: LoadCmdFromResource::<Identity, Impl, OFFSET>,
            LoadCmdFromMemory: LoadCmdFromMemory::<Identity, Impl, OFFSET>,
            LoadCmdFromProprietaryGrammar: LoadCmdFromProprietaryGrammar::<Identity, Impl, OFFSET>,
            SetRuleState: SetRuleState::<Identity, Impl, OFFSET>,
            SetRuleIdState: SetRuleIdState::<Identity, Impl, OFFSET>,
            LoadDictation: LoadDictation::<Identity, Impl, OFFSET>,
            UnloadDictation: UnloadDictation::<Identity, Impl, OFFSET>,
            SetDictationState: SetDictationState::<Identity, Impl, OFFSET>,
            SetWordSequenceData: SetWordSequenceData::<Identity, Impl, OFFSET>,
            SetTextSelection: SetTextSelection::<Identity, Impl, OFFSET>,
            IsPronounceable: IsPronounceable::<Identity, Impl, OFFSET>,
            SetGrammarState: SetGrammarState::<Identity, Impl, OFFSET>,
            SaveCmd: SaveCmd::<Identity, Impl, OFFSET>,
            GetGrammarState: GetGrammarState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpRecoGrammar as ::windows_core::ComInterface>::IID || iid == &<ISpGrammarBuilder as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_System_Com_Urlmon\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com_Urlmon")]
pub trait ISpRecoGrammar2_Impl: Sized {
    fn GetRules(&self, ppcomemrules: *mut *mut SPRULE, punumrules: *mut u32) -> ::windows_core::Result<()>;
    fn LoadCmdFromFile2(&self, pszfilename: &::windows_core::PCWSTR, options: SPLOADOPTIONS, pszsharinguri: &::windows_core::PCWSTR, pszbaseuri: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn LoadCmdFromMemory2(&self, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS, pszsharinguri: &::windows_core::PCWSTR, pszbaseuri: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetRulePriority(&self, pszrulename: &::windows_core::PCWSTR, ulruleid: u32, nrulepriority: i32) -> ::windows_core::Result<()>;
    fn SetRuleWeight(&self, pszrulename: &::windows_core::PCWSTR, ulruleid: u32, flweight: f32) -> ::windows_core::Result<()>;
    fn SetDictationWeight(&self, flweight: f32) -> ::windows_core::Result<()>;
    fn SetGrammarLoader(&self, ploader: ::core::option::Option<&ISpeechResourceLoader>) -> ::windows_core::Result<()>;
    fn SetSMLSecurityManager(&self, psmlsecuritymanager: ::core::option::Option<&super::super::System::Com::Urlmon::IInternetSecurityManager>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_Urlmon")]
impl ::windows_core::RuntimeName for ISpRecoGrammar2 {}
#[cfg(feature = "Win32_System_Com_Urlmon")]
impl ISpRecoGrammar2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>() -> ISpRecoGrammar2_Vtbl {
        unsafe extern "system" fn GetRules<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomemrules: *mut *mut SPRULE, punumrules: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRules(::core::mem::transmute_copy(&ppcomemrules), ::core::mem::transmute_copy(&punumrules)).into()
        }
        unsafe extern "system" fn LoadCmdFromFile2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: ::windows_core::PCWSTR, options: SPLOADOPTIONS, pszsharinguri: ::windows_core::PCWSTR, pszbaseuri: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadCmdFromFile2(::core::mem::transmute(&pszfilename), ::core::mem::transmute_copy(&options), ::core::mem::transmute(&pszsharinguri), ::core::mem::transmute(&pszbaseuri)).into()
        }
        unsafe extern "system" fn LoadCmdFromMemory2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS, pszsharinguri: ::windows_core::PCWSTR, pszbaseuri: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadCmdFromMemory2(::core::mem::transmute_copy(&pgrammar), ::core::mem::transmute_copy(&options), ::core::mem::transmute(&pszsharinguri), ::core::mem::transmute(&pszbaseuri)).into()
        }
        unsafe extern "system" fn SetRulePriority<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrulename: ::windows_core::PCWSTR, ulruleid: u32, nrulepriority: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRulePriority(::core::mem::transmute(&pszrulename), ::core::mem::transmute_copy(&ulruleid), ::core::mem::transmute_copy(&nrulepriority)).into()
        }
        unsafe extern "system" fn SetRuleWeight<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszrulename: ::windows_core::PCWSTR, ulruleid: u32, flweight: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRuleWeight(::core::mem::transmute(&pszrulename), ::core::mem::transmute_copy(&ulruleid), ::core::mem::transmute_copy(&flweight)).into()
        }
        unsafe extern "system" fn SetDictationWeight<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flweight: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDictationWeight(::core::mem::transmute_copy(&flweight)).into()
        }
        unsafe extern "system" fn SetGrammarLoader<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ploader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGrammarLoader(::windows_core::from_raw_borrowed(&ploader)).into()
        }
        unsafe extern "system" fn SetSMLSecurityManager<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psmlsecuritymanager: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSMLSecurityManager(::windows_core::from_raw_borrowed(&psmlsecuritymanager)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRules: GetRules::<Identity, Impl, OFFSET>,
            LoadCmdFromFile2: LoadCmdFromFile2::<Identity, Impl, OFFSET>,
            LoadCmdFromMemory2: LoadCmdFromMemory2::<Identity, Impl, OFFSET>,
            SetRulePriority: SetRulePriority::<Identity, Impl, OFFSET>,
            SetRuleWeight: SetRuleWeight::<Identity, Impl, OFFSET>,
            SetDictationWeight: SetDictationWeight::<Identity, Impl, OFFSET>,
            SetGrammarLoader: SetGrammarLoader::<Identity, Impl, OFFSET>,
            SetSMLSecurityManager: SetSMLSecurityManager::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpRecoGrammar2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_Media_Audio\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpRecoResult_Impl: Sized + ISpPhrase_Impl {
    fn GetResultTimes(&self, ptimes: *mut SPRECORESULTTIMES) -> ::windows_core::Result<()>;
    fn GetAlternates(&self, ulstartelement: u32, celements: u32, ulrequestcount: u32, ppphrases: *mut ::core::option::Option<ISpPhraseAlt>, pcphrasesreturned: *mut u32) -> ::windows_core::Result<()>;
    fn GetAudio(&self, ulstartelement: u32, celements: u32) -> ::windows_core::Result<ISpStreamFormat>;
    fn SpeakAudio(&self, ulstartelement: u32, celements: u32, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows_core::Result<()>;
    fn Serialize(&self, ppcomemserializedresult: *mut *mut SPSERIALIZEDRESULT) -> ::windows_core::Result<()>;
    fn ScaleAudio(&self, paudioformatid: *const ::windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn GetRecoContext(&self) -> ::windows_core::Result<ISpRecoContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpRecoResult {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpRecoResult_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoResult_Impl, const OFFSET: isize>() -> ISpRecoResult_Vtbl {
        unsafe extern "system" fn GetResultTimes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimes: *mut SPRECORESULTTIMES) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetResultTimes(::core::mem::transmute_copy(&ptimes)).into()
        }
        unsafe extern "system" fn GetAlternates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, ulrequestcount: u32, ppphrases: *mut *mut ::core::ffi::c_void, pcphrasesreturned: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAlternates(::core::mem::transmute_copy(&ulstartelement), ::core::mem::transmute_copy(&celements), ::core::mem::transmute_copy(&ulrequestcount), ::core::mem::transmute_copy(&ppphrases), ::core::mem::transmute_copy(&pcphrasesreturned)).into()
        }
        unsafe extern "system" fn GetAudio<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAudio(::core::mem::transmute_copy(&ulstartelement), ::core::mem::transmute_copy(&celements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakAudio<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SpeakAudio(::core::mem::transmute_copy(&ulstartelement), ::core::mem::transmute_copy(&celements), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pulstreamnumber)).into()
        }
        unsafe extern "system" fn Serialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomemserializedresult: *mut *mut SPSERIALIZEDRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Serialize(::core::mem::transmute_copy(&ppcomemserializedresult)).into()
        }
        unsafe extern "system" fn ScaleAudio<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paudioformatid: *const ::windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ScaleAudio(::core::mem::transmute_copy(&paudioformatid), ::core::mem::transmute_copy(&pwaveformatex)).into()
        }
        unsafe extern "system" fn GetRecoContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprecocontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRecoContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprecocontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISpPhrase_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetResultTimes: GetResultTimes::<Identity, Impl, OFFSET>,
            GetAlternates: GetAlternates::<Identity, Impl, OFFSET>,
            GetAudio: GetAudio::<Identity, Impl, OFFSET>,
            SpeakAudio: SpeakAudio::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
            ScaleAudio: ScaleAudio::<Identity, Impl, OFFSET>,
            GetRecoContext: GetRecoContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpRecoResult as ::windows_core::ComInterface>::IID || iid == &<ISpPhrase as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_Media_Audio\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpRecoResult2_Impl: Sized + ISpRecoResult_Impl {
    fn CommitAlternate(&self, pphrasealt: ::core::option::Option<&ISpPhraseAlt>) -> ::windows_core::Result<ISpRecoResult>;
    fn CommitText(&self, ulstartelement: u32, celements: u32, pszcorrecteddata: &::windows_core::PCWSTR, ecommitflags: u32) -> ::windows_core::Result<()>;
    fn SetTextFeedback(&self, pszfeedback: &::windows_core::PCWSTR, fsuccessful: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpRecoResult2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpRecoResult2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoResult2_Impl, const OFFSET: isize>() -> ISpRecoResult2_Vtbl {
        unsafe extern "system" fn CommitAlternate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphrasealt: *mut ::core::ffi::c_void, ppnewresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommitAlternate(::windows_core::from_raw_borrowed(&pphrasealt)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulstartelement: u32, celements: u32, pszcorrecteddata: ::windows_core::PCWSTR, ecommitflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CommitText(::core::mem::transmute_copy(&ulstartelement), ::core::mem::transmute_copy(&celements), ::core::mem::transmute(&pszcorrecteddata), ::core::mem::transmute_copy(&ecommitflags)).into()
        }
        unsafe extern "system" fn SetTextFeedback<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecoResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfeedback: ::windows_core::PCWSTR, fsuccessful: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTextFeedback(::core::mem::transmute(&pszfeedback), ::core::mem::transmute_copy(&fsuccessful)).into()
        }
        Self {
            base__: ISpRecoResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            CommitAlternate: CommitAlternate::<Identity, Impl, OFFSET>,
            CommitText: CommitText::<Identity, Impl, OFFSET>,
            SetTextFeedback: SetTextFeedback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpRecoResult2 as ::windows_core::ComInterface>::IID || iid == &<ISpPhrase as ::windows_core::ComInterface>::IID || iid == &<ISpRecoResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_Media_Audio\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
pub trait ISpRecognizer_Impl: Sized + ISpProperties_Impl {
    fn SetRecognizer(&self, precognizer: ::core::option::Option<&ISpObjectToken>) -> ::windows_core::Result<()>;
    fn GetRecognizer(&self) -> ::windows_core::Result<ISpObjectToken>;
    fn SetInput(&self, punkinput: ::core::option::Option<&::windows_core::IUnknown>, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetInputObjectToken(&self) -> ::windows_core::Result<ISpObjectToken>;
    fn GetInputStream(&self) -> ::windows_core::Result<ISpStreamFormat>;
    fn CreateRecoContext(&self) -> ::windows_core::Result<ISpRecoContext>;
    fn GetRecoProfile(&self) -> ::windows_core::Result<ISpObjectToken>;
    fn SetRecoProfile(&self, ptoken: ::core::option::Option<&ISpObjectToken>) -> ::windows_core::Result<()>;
    fn IsSharedInstance(&self) -> ::windows_core::Result<()>;
    fn GetRecoState(&self, pstate: *mut SPRECOSTATE) -> ::windows_core::Result<()>;
    fn SetRecoState(&self, newstate: SPRECOSTATE) -> ::windows_core::Result<()>;
    fn GetStatus(&self, pstatus: *mut SPRECOGNIZERSTATUS) -> ::windows_core::Result<()>;
    fn GetFormat(&self, waveformattype: SPSTREAMFORMATTYPE, pformatid: *mut ::windows_core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn IsUISupported(&self, psztypeofui: &::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn DisplayUI(&self, hwndparent: super::super::Foundation::HWND, psztitle: &::windows_core::PCWSTR, psztypeofui: &::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows_core::Result<()>;
    fn EmulateRecognition(&self, pphrase: ::core::option::Option<&ISpPhrase>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for ISpRecognizer {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ISpRecognizer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: isize>() -> ISpRecognizer_Vtbl {
        unsafe extern "system" fn SetRecognizer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precognizer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRecognizer(::windows_core::from_raw_borrowed(&precognizer)).into()
        }
        unsafe extern "system" fn GetRecognizer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprecognizer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRecognizer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprecognizer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInput<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkinput: *mut ::core::ffi::c_void, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInput(::windows_core::from_raw_borrowed(&punkinput), ::core::mem::transmute_copy(&fallowformatchanges)).into()
        }
        unsafe extern "system" fn GetInputObjectToken<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptoken: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputObjectToken() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptoken, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRecoContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnewctxt: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRecoContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewctxt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecoProfile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptoken: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRecoProfile() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptoken, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecoProfile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptoken: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRecoProfile(::windows_core::from_raw_borrowed(&ptoken)).into()
        }
        unsafe extern "system" fn IsSharedInstance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsSharedInstance().into()
        }
        unsafe extern "system" fn GetRecoState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut SPRECOSTATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRecoState(::core::mem::transmute_copy(&pstate)).into()
        }
        unsafe extern "system" fn SetRecoState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: SPRECOSTATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRecoState(::core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut SPRECOGNIZERSTATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStatus(::core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn GetFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, waveformattype: SPSTREAMFORMATTYPE, pformatid: *mut ::windows_core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFormat(::core::mem::transmute_copy(&waveformattype), ::core::mem::transmute_copy(&pformatid), ::core::mem::transmute_copy(&ppcomemwfex)).into()
        }
        unsafe extern "system" fn IsUISupported<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztypeofui: ::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsUISupported(::core::mem::transmute(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata), ::core::mem::transmute_copy(&pfsupported)).into()
        }
        unsafe extern "system" fn DisplayUI<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, psztitle: ::windows_core::PCWSTR, psztypeofui: ::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisplayUI(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&psztitle), ::core::mem::transmute(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata)).into()
        }
        unsafe extern "system" fn EmulateRecognition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphrase: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EmulateRecognition(::windows_core::from_raw_borrowed(&pphrase)).into()
        }
        Self {
            base__: ISpProperties_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetRecognizer: SetRecognizer::<Identity, Impl, OFFSET>,
            GetRecognizer: GetRecognizer::<Identity, Impl, OFFSET>,
            SetInput: SetInput::<Identity, Impl, OFFSET>,
            GetInputObjectToken: GetInputObjectToken::<Identity, Impl, OFFSET>,
            GetInputStream: GetInputStream::<Identity, Impl, OFFSET>,
            CreateRecoContext: CreateRecoContext::<Identity, Impl, OFFSET>,
            GetRecoProfile: GetRecoProfile::<Identity, Impl, OFFSET>,
            SetRecoProfile: SetRecoProfile::<Identity, Impl, OFFSET>,
            IsSharedInstance: IsSharedInstance::<Identity, Impl, OFFSET>,
            GetRecoState: GetRecoState::<Identity, Impl, OFFSET>,
            SetRecoState: SetRecoState::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetFormat: GetFormat::<Identity, Impl, OFFSET>,
            IsUISupported: IsUISupported::<Identity, Impl, OFFSET>,
            DisplayUI: DisplayUI::<Identity, Impl, OFFSET>,
            EmulateRecognition: EmulateRecognition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpRecognizer as ::windows_core::ComInterface>::IID || iid == &<ISpProperties as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpRecognizer2_Impl: Sized {
    fn EmulateRecognitionEx(&self, pphrase: ::core::option::Option<&ISpPhrase>, dwcompareflags: u32) -> ::windows_core::Result<()>;
    fn SetTrainingState(&self, fdoingtraining: super::super::Foundation::BOOL, fadaptfromtrainingdata: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ResetAcousticModelAdaptation(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ISpRecognizer2 {}
#[cfg(feature = "Win32_Foundation")]
impl ISpRecognizer2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecognizer2_Impl, const OFFSET: isize>() -> ISpRecognizer2_Vtbl {
        unsafe extern "system" fn EmulateRecognitionEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecognizer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphrase: *mut ::core::ffi::c_void, dwcompareflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EmulateRecognitionEx(::windows_core::from_raw_borrowed(&pphrase), ::core::mem::transmute_copy(&dwcompareflags)).into()
        }
        unsafe extern "system" fn SetTrainingState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecognizer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdoingtraining: super::super::Foundation::BOOL, fadaptfromtrainingdata: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTrainingState(::core::mem::transmute_copy(&fdoingtraining), ::core::mem::transmute_copy(&fadaptfromtrainingdata)).into()
        }
        unsafe extern "system" fn ResetAcousticModelAdaptation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRecognizer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResetAcousticModelAdaptation().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EmulateRecognitionEx: EmulateRecognitionEx::<Identity, Impl, OFFSET>,
            SetTrainingState: SetTrainingState::<Identity, Impl, OFFSET>,
            ResetAcousticModelAdaptation: ResetAcousticModelAdaptation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpRecognizer2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait ISpRegDataKey_Impl: Sized + ISpDataKey_Impl {
    fn SetKey(&self, hkey: super::super::System::Registry::HKEY, freadonly: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::windows_core::RuntimeName for ISpRegDataKey {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ISpRegDataKey_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRegDataKey_Impl, const OFFSET: isize>() -> ISpRegDataKey_Vtbl {
        unsafe extern "system" fn SetKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpRegDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkey: super::super::System::Registry::HKEY, freadonly: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetKey(::core::mem::transmute_copy(&hkey), ::core::mem::transmute_copy(&freadonly)).into()
        }
        Self { base__: ISpDataKey_Vtbl::new::<Identity, Impl, OFFSET>(), SetKey: SetKey::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpRegDataKey as ::windows_core::ComInterface>::IID || iid == &<ISpDataKey as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISpResourceManager_Impl: Sized + super::super::System::Com::IServiceProvider_Impl {
    fn SetObject(&self, guidserviceid: *const ::windows_core::GUID, punkobject: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetObject(&self, guidserviceid: *const ::windows_core::GUID, objectclsid: *const ::windows_core::GUID, objectiid: *const ::windows_core::GUID, freleasewhenlastexternalrefreleased: super::super::Foundation::BOOL, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for ISpResourceManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISpResourceManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpResourceManager_Impl, const OFFSET: isize>() -> ISpResourceManager_Vtbl {
        unsafe extern "system" fn SetObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpResourceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidserviceid: *const ::windows_core::GUID, punkobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetObject(::core::mem::transmute_copy(&guidserviceid), ::windows_core::from_raw_borrowed(&punkobject)).into()
        }
        unsafe extern "system" fn GetObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpResourceManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidserviceid: *const ::windows_core::GUID, objectclsid: *const ::windows_core::GUID, objectiid: *const ::windows_core::GUID, freleasewhenlastexternalrefreleased: super::super::Foundation::BOOL, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObject(::core::mem::transmute_copy(&guidserviceid), ::core::mem::transmute_copy(&objectclsid), ::core::mem::transmute_copy(&objectiid), ::core::mem::transmute_copy(&freleasewhenlastexternalrefreleased), ::core::mem::transmute_copy(&ppobject)).into()
        }
        Self {
            base__: super::super::System::Com::IServiceProvider_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetObject: SetObject::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpResourceManager as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IServiceProvider as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"implement\"`*"]
pub trait ISpSerializeState_Impl: Sized {
    fn GetSerializedState(&self, ppbdata: *mut *mut u8, pulsize: *mut u32, dwreserved: u32) -> ::windows_core::Result<()>;
    fn SetSerializedState(&self, pbdata: *const u8, ulsize: u32, dwreserved: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISpSerializeState {}
impl ISpSerializeState_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpSerializeState_Impl, const OFFSET: isize>() -> ISpSerializeState_Vtbl {
        unsafe extern "system" fn GetSerializedState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpSerializeState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbdata: *mut *mut u8, pulsize: *mut u32, dwreserved: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSerializedState(::core::mem::transmute_copy(&ppbdata), ::core::mem::transmute_copy(&pulsize), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SetSerializedState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpSerializeState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, ulsize: u32, dwreserved: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSerializedState(::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&ulsize), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSerializedState: GetSerializedState::<Identity, Impl, OFFSET>,
            SetSerializedState: SetSerializedState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpSerializeState as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"implement\"`*"]
pub trait ISpShortcut_Impl: Sized {
    fn AddShortcut(&self, pszdisplay: &::windows_core::PCWSTR, langid: u16, pszspoken: &::windows_core::PCWSTR, shtype: SPSHORTCUTTYPE) -> ::windows_core::Result<()>;
    fn RemoveShortcut(&self, pszdisplay: &::windows_core::PCWSTR, langid: u16, pszspoken: &::windows_core::PCWSTR, shtype: SPSHORTCUTTYPE) -> ::windows_core::Result<()>;
    fn GetShortcuts(&self, langid: u16, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows_core::Result<()>;
    fn GetGeneration(&self) -> ::windows_core::Result<u32>;
    fn GetWordsFromGenerationChange(&self, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows_core::Result<()>;
    fn GetWords(&self, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows_core::Result<()>;
    fn GetShortcutsForGeneration(&self, pdwgeneration: *mut u32, pdwcookie: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows_core::Result<()>;
    fn GetGenerationChange(&self, pdwgeneration: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISpShortcut {}
impl ISpShortcut_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpShortcut_Impl, const OFFSET: isize>() -> ISpShortcut_Vtbl {
        unsafe extern "system" fn AddShortcut<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdisplay: ::windows_core::PCWSTR, langid: u16, pszspoken: ::windows_core::PCWSTR, shtype: SPSHORTCUTTYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddShortcut(::core::mem::transmute(&pszdisplay), ::core::mem::transmute_copy(&langid), ::core::mem::transmute(&pszspoken), ::core::mem::transmute_copy(&shtype)).into()
        }
        unsafe extern "system" fn RemoveShortcut<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdisplay: ::windows_core::PCWSTR, langid: u16, pszspoken: ::windows_core::PCWSTR, shtype: SPSHORTCUTTYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveShortcut(::core::mem::transmute(&pszdisplay), ::core::mem::transmute_copy(&langid), ::core::mem::transmute(&pszspoken), ::core::mem::transmute_copy(&shtype)).into()
        }
        unsafe extern "system" fn GetShortcuts<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: u16, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetShortcuts(::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&pshortcutpairlist)).into()
        }
        unsafe extern "system" fn GetGeneration<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGeneration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwgeneration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWordsFromGenerationChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWordsFromGenerationChange(::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pwordlist)).into()
        }
        unsafe extern "system" fn GetWords<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWords(::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pdwcookie), ::core::mem::transmute_copy(&pwordlist)).into()
        }
        unsafe extern "system" fn GetShortcutsForGeneration<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pdwcookie: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetShortcutsForGeneration(::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pdwcookie), ::core::mem::transmute_copy(&pshortcutpairlist)).into()
        }
        unsafe extern "system" fn GetGenerationChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpShortcut_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwgeneration: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGenerationChange(::core::mem::transmute_copy(&pdwgeneration), ::core::mem::transmute_copy(&pshortcutpairlist)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddShortcut: AddShortcut::<Identity, Impl, OFFSET>,
            RemoveShortcut: RemoveShortcut::<Identity, Impl, OFFSET>,
            GetShortcuts: GetShortcuts::<Identity, Impl, OFFSET>,
            GetGeneration: GetGeneration::<Identity, Impl, OFFSET>,
            GetWordsFromGenerationChange: GetWordsFromGenerationChange::<Identity, Impl, OFFSET>,
            GetWords: GetWords::<Identity, Impl, OFFSET>,
            GetShortcutsForGeneration: GetShortcutsForGeneration::<Identity, Impl, OFFSET>,
            GetGenerationChange: GetGenerationChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpShortcut as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_Media_Audio\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
pub trait ISpStream_Impl: Sized + ISpStreamFormat_Impl {
    fn SetBaseStream(&self, pstream: ::core::option::Option<&super::super::System::Com::IStream>, rguidformat: *const ::windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn GetBaseStream(&self) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn BindToFile(&self, pszfilename: &::windows_core::PCWSTR, emode: SPFILEMODE, pformatid: *const ::windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX, ulleventinterest: u64) -> ::windows_core::Result<()>;
    fn Close(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for ISpStream {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ISpStream_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpStream_Impl, const OFFSET: isize>() -> ISpStream_Vtbl {
        unsafe extern "system" fn SetBaseStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, rguidformat: *const ::windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBaseStream(::windows_core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&rguidformat), ::core::mem::transmute_copy(&pwaveformatex)).into()
        }
        unsafe extern "system" fn GetBaseStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBaseStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: ::windows_core::PCWSTR, emode: SPFILEMODE, pformatid: *const ::windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX, ulleventinterest: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindToFile(::core::mem::transmute(&pszfilename), ::core::mem::transmute_copy(&emode), ::core::mem::transmute_copy(&pformatid), ::core::mem::transmute_copy(&pwaveformatex), ::core::mem::transmute_copy(&ulleventinterest)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        Self {
            base__: ISpStreamFormat_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetBaseStream: SetBaseStream::<Identity, Impl, OFFSET>,
            GetBaseStream: GetBaseStream::<Identity, Impl, OFFSET>,
            BindToFile: BindToFile::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpStream as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::ISequentialStream as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IStream as ::windows_core::ComInterface>::IID || iid == &<ISpStreamFormat as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_Media_Audio\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
pub trait ISpStreamFormat_Impl: Sized + super::super::System::Com::IStream_Impl {
    fn GetFormat(&self, pguidformatid: *const ::windows_core::GUID) -> ::windows_core::Result<*mut super::Audio::WAVEFORMATEX>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for ISpStreamFormat {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ISpStreamFormat_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpStreamFormat_Impl, const OFFSET: isize>() -> ISpStreamFormat_Vtbl {
        unsafe extern "system" fn GetFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpStreamFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidformatid: *const ::windows_core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFormat(::core::mem::transmute_copy(&pguidformatid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcomemwaveformatex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IStream_Vtbl::new::<Identity, Impl, OFFSET>(), GetFormat: GetFormat::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpStreamFormat as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::ISequentialStream as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IStream as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_Media_Audio\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
pub trait ISpStreamFormatConverter_Impl: Sized + ISpStreamFormat_Impl {
    fn SetBaseStream(&self, pstream: ::core::option::Option<&ISpStreamFormat>, fsetformattobasestreamformat: super::super::Foundation::BOOL, fwritetobasestream: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetBaseStream(&self) -> ::windows_core::Result<ISpStreamFormat>;
    fn SetFormat(&self, rguidformatidofconvertedstream: *const ::windows_core::GUID, pwaveformatexofconvertedstream: *const super::Audio::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn ResetSeekPosition(&self) -> ::windows_core::Result<()>;
    fn ScaleConvertedToBaseOffset(&self, ulloffsetconvertedstream: u64) -> ::windows_core::Result<u64>;
    fn ScaleBaseToConvertedOffset(&self, ulloffsetbasestream: u64) -> ::windows_core::Result<u64>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for ISpStreamFormatConverter {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ISpStreamFormatConverter_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpStreamFormatConverter_Impl, const OFFSET: isize>() -> ISpStreamFormatConverter_Vtbl {
        unsafe extern "system" fn SetBaseStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, fsetformattobasestreamformat: super::super::Foundation::BOOL, fwritetobasestream: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBaseStream(::windows_core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&fsetformattobasestreamformat), ::core::mem::transmute_copy(&fwritetobasestream)).into()
        }
        unsafe extern "system" fn GetBaseStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBaseStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidformatidofconvertedstream: *const ::windows_core::GUID, pwaveformatexofconvertedstream: *const super::Audio::WAVEFORMATEX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFormat(::core::mem::transmute_copy(&rguidformatidofconvertedstream), ::core::mem::transmute_copy(&pwaveformatexofconvertedstream)).into()
        }
        unsafe extern "system" fn ResetSeekPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResetSeekPosition().into()
        }
        unsafe extern "system" fn ScaleConvertedToBaseOffset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulloffsetconvertedstream: u64, pulloffsetbasestream: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ScaleConvertedToBaseOffset(::core::mem::transmute_copy(&ulloffsetconvertedstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulloffsetbasestream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleBaseToConvertedOffset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulloffsetbasestream: u64, pulloffsetconvertedstream: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ScaleBaseToConvertedOffset(::core::mem::transmute_copy(&ulloffsetbasestream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulloffsetconvertedstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISpStreamFormat_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetBaseStream: SetBaseStream::<Identity, Impl, OFFSET>,
            GetBaseStream: GetBaseStream::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
            ResetSeekPosition: ResetSeekPosition::<Identity, Impl, OFFSET>,
            ScaleConvertedToBaseOffset: ScaleConvertedToBaseOffset::<Identity, Impl, OFFSET>,
            ScaleBaseToConvertedOffset: ScaleBaseToConvertedOffset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpStreamFormatConverter as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::ISequentialStream as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IStream as ::windows_core::ComInterface>::IID || iid == &<ISpStreamFormat as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"implement\"`*"]
pub trait ISpTranscript_Impl: Sized {
    fn GetTranscript(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn AppendTranscript(&self, psztranscript: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISpTranscript {}
impl ISpTranscript_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpTranscript_Impl, const OFFSET: isize>() -> ISpTranscript_Vtbl {
        unsafe extern "system" fn GetTranscript<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpTranscript_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsztranscript: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTranscript() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsztranscript, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppendTranscript<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpTranscript_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztranscript: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AppendTranscript(::core::mem::transmute(&psztranscript)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTranscript: GetTranscript::<Identity, Impl, OFFSET>,
            AppendTranscript: AppendTranscript::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpTranscript as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISpVoice_Impl: Sized + ISpEventSource_Impl {
    fn SetOutput(&self, punkoutput: ::core::option::Option<&::windows_core::IUnknown>, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetOutputObjectToken(&self) -> ::windows_core::Result<ISpObjectToken>;
    fn GetOutputStream(&self) -> ::windows_core::Result<ISpStreamFormat>;
    fn Pause(&self) -> ::windows_core::Result<()>;
    fn Resume(&self) -> ::windows_core::Result<()>;
    fn SetVoice(&self, ptoken: ::core::option::Option<&ISpObjectToken>) -> ::windows_core::Result<()>;
    fn GetVoice(&self) -> ::windows_core::Result<ISpObjectToken>;
    fn Speak(&self, pwcs: &::windows_core::PCWSTR, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows_core::Result<()>;
    fn SpeakStream(&self, pstream: ::core::option::Option<&super::super::System::Com::IStream>, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows_core::Result<()>;
    fn GetStatus(&self, pstatus: *mut SPVOICESTATUS, ppszlastbookmark: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn Skip(&self, pitemtype: &::windows_core::PCWSTR, lnumitems: i32, pulnumskipped: *mut u32) -> ::windows_core::Result<()>;
    fn SetPriority(&self, epriority: SPVPRIORITY) -> ::windows_core::Result<()>;
    fn GetPriority(&self, pepriority: *mut SPVPRIORITY) -> ::windows_core::Result<()>;
    fn SetAlertBoundary(&self, eboundary: SPEVENTENUM) -> ::windows_core::Result<()>;
    fn GetAlertBoundary(&self, peboundary: *mut SPEVENTENUM) -> ::windows_core::Result<()>;
    fn SetRate(&self, rateadjust: i32) -> ::windows_core::Result<()>;
    fn GetRate(&self, prateadjust: *mut i32) -> ::windows_core::Result<()>;
    fn SetVolume(&self, usvolume: u16) -> ::windows_core::Result<()>;
    fn GetVolume(&self, pusvolume: *mut u16) -> ::windows_core::Result<()>;
    fn WaitUntilDone(&self, mstimeout: u32) -> ::windows_core::Result<()>;
    fn SetSyncSpeakTimeout(&self, mstimeout: u32) -> ::windows_core::Result<()>;
    fn GetSyncSpeakTimeout(&self, pmstimeout: *mut u32) -> ::windows_core::Result<()>;
    fn SpeakCompleteEvent(&self) -> super::super::Foundation::HANDLE;
    fn IsUISupported(&self, psztypeofui: &::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn DisplayUI(&self, hwndparent: super::super::Foundation::HWND, psztitle: &::windows_core::PCWSTR, psztypeofui: &::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for ISpVoice {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ISpVoice_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>() -> ISpVoice_Vtbl {
        unsafe extern "system" fn SetOutput<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkoutput: *mut ::core::ffi::c_void, fallowformatchanges: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOutput(::windows_core::from_raw_borrowed(&punkoutput), ::core::mem::transmute_copy(&fallowformatchanges)).into()
        }
        unsafe extern "system" fn GetOutputObjectToken<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobjecttoken: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputObjectToken() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobjecttoken, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resume().into()
        }
        unsafe extern "system" fn SetVoice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptoken: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVoice(::windows_core::from_raw_borrowed(&ptoken)).into()
        }
        unsafe extern "system" fn GetVoice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptoken: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVoice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptoken, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Speak<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcs: ::windows_core::PCWSTR, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Speak(::core::mem::transmute(&pwcs), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pulstreamnumber)).into()
        }
        unsafe extern "system" fn SpeakStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, dwflags: u32, pulstreamnumber: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SpeakStream(::windows_core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pulstreamnumber)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut SPVOICESTATUS, ppszlastbookmark: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStatus(::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&ppszlastbookmark)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitemtype: ::windows_core::PCWSTR, lnumitems: i32, pulnumskipped: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute(&pitemtype), ::core::mem::transmute_copy(&lnumitems), ::core::mem::transmute_copy(&pulnumskipped)).into()
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, epriority: SPVPRIORITY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPriority(::core::mem::transmute_copy(&epriority)).into()
        }
        unsafe extern "system" fn GetPriority<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pepriority: *mut SPVPRIORITY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPriority(::core::mem::transmute_copy(&pepriority)).into()
        }
        unsafe extern "system" fn SetAlertBoundary<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eboundary: SPEVENTENUM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAlertBoundary(::core::mem::transmute_copy(&eboundary)).into()
        }
        unsafe extern "system" fn GetAlertBoundary<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peboundary: *mut SPEVENTENUM) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAlertBoundary(::core::mem::transmute_copy(&peboundary)).into()
        }
        unsafe extern "system" fn SetRate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rateadjust: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRate(::core::mem::transmute_copy(&rateadjust)).into()
        }
        unsafe extern "system" fn GetRate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prateadjust: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRate(::core::mem::transmute_copy(&prateadjust)).into()
        }
        unsafe extern "system" fn SetVolume<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usvolume: u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVolume(::core::mem::transmute_copy(&usvolume)).into()
        }
        unsafe extern "system" fn GetVolume<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusvolume: *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVolume(::core::mem::transmute_copy(&pusvolume)).into()
        }
        unsafe extern "system" fn WaitUntilDone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mstimeout: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WaitUntilDone(::core::mem::transmute_copy(&mstimeout)).into()
        }
        unsafe extern "system" fn SetSyncSpeakTimeout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mstimeout: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSyncSpeakTimeout(::core::mem::transmute_copy(&mstimeout)).into()
        }
        unsafe extern "system" fn GetSyncSpeakTimeout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmstimeout: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSyncSpeakTimeout(::core::mem::transmute_copy(&pmstimeout)).into()
        }
        unsafe extern "system" fn SpeakCompleteEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SpeakCompleteEvent()
        }
        unsafe extern "system" fn IsUISupported<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztypeofui: ::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsUISupported(::core::mem::transmute(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata), ::core::mem::transmute_copy(&pfsupported)).into()
        }
        unsafe extern "system" fn DisplayUI<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, psztitle: ::windows_core::PCWSTR, psztypeofui: ::windows_core::PCWSTR, pvextradata: *mut ::core::ffi::c_void, cbextradata: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisplayUI(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&psztitle), ::core::mem::transmute(&psztypeofui), ::core::mem::transmute_copy(&pvextradata), ::core::mem::transmute_copy(&cbextradata)).into()
        }
        Self {
            base__: ISpEventSource_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetOutput: SetOutput::<Identity, Impl, OFFSET>,
            GetOutputObjectToken: GetOutputObjectToken::<Identity, Impl, OFFSET>,
            GetOutputStream: GetOutputStream::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            SetVoice: SetVoice::<Identity, Impl, OFFSET>,
            GetVoice: GetVoice::<Identity, Impl, OFFSET>,
            Speak: Speak::<Identity, Impl, OFFSET>,
            SpeakStream: SpeakStream::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            GetPriority: GetPriority::<Identity, Impl, OFFSET>,
            SetAlertBoundary: SetAlertBoundary::<Identity, Impl, OFFSET>,
            GetAlertBoundary: GetAlertBoundary::<Identity, Impl, OFFSET>,
            SetRate: SetRate::<Identity, Impl, OFFSET>,
            GetRate: GetRate::<Identity, Impl, OFFSET>,
            SetVolume: SetVolume::<Identity, Impl, OFFSET>,
            GetVolume: GetVolume::<Identity, Impl, OFFSET>,
            WaitUntilDone: WaitUntilDone::<Identity, Impl, OFFSET>,
            SetSyncSpeakTimeout: SetSyncSpeakTimeout::<Identity, Impl, OFFSET>,
            GetSyncSpeakTimeout: GetSyncSpeakTimeout::<Identity, Impl, OFFSET>,
            SpeakCompleteEvent: SpeakCompleteEvent::<Identity, Impl, OFFSET>,
            IsUISupported: IsUISupported::<Identity, Impl, OFFSET>,
            DisplayUI: DisplayUI::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpVoice as ::windows_core::ComInterface>::IID || iid == &<ISpNotifySource as ::windows_core::ComInterface>::IID || iid == &<ISpEventSource as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_Media_Audio\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpXMLRecoResult_Impl: Sized + ISpRecoResult_Impl {
    fn GetXMLResult(&self, ppszcomemxmlresult: *mut ::windows_core::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows_core::Result<()>;
    fn GetXMLErrorInfo(&self, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpXMLRecoResult {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpXMLRecoResult_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpXMLRecoResult_Impl, const OFFSET: isize>() -> ISpXMLRecoResult_Vtbl {
        unsafe extern "system" fn GetXMLResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpXMLRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemxmlresult: *mut ::windows_core::PWSTR, options: SPXMLRESULTOPTIONS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetXMLResult(::core::mem::transmute_copy(&ppszcomemxmlresult), ::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn GetXMLErrorInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpXMLRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetXMLErrorInfo(::core::mem::transmute_copy(&psemanticerrorinfo)).into()
        }
        Self {
            base__: ISpRecoResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetXMLResult: GetXMLResult::<Identity, Impl, OFFSET>,
            GetXMLErrorInfo: GetXMLErrorInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpXMLRecoResult as ::windows_core::ComInterface>::IID || iid == &<ISpPhrase as ::windows_core::ComInterface>::IID || iid == &<ISpRecoResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechAudio_Impl: Sized + ISpeechBaseStream_Impl {
    fn Status(&self) -> ::windows_core::Result<ISpeechAudioStatus>;
    fn BufferInfo(&self) -> ::windows_core::Result<ISpeechAudioBufferInfo>;
    fn DefaultFormat(&self) -> ::windows_core::Result<ISpeechAudioFormat>;
    fn Volume(&self) -> ::windows_core::Result<i32>;
    fn SetVolume(&self, volume: i32) -> ::windows_core::Result<()>;
    fn BufferNotifySize(&self) -> ::windows_core::Result<i32>;
    fn SetBufferNotifySize(&self, buffernotifysize: i32) -> ::windows_core::Result<()>;
    fn EventHandle(&self) -> ::windows_core::Result<i32>;
    fn SetState(&self, state: SpeechAudioState) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechAudio {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechAudio_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudio_Impl, const OFFSET: isize>() -> ISpeechAudio_Vtbl {
        unsafe extern "system" fn Status<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bufferinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BufferInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bufferinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamformat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DefaultFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(streamformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Volume<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Volume() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(volume, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVolume(::core::mem::transmute_copy(&volume)).into()
        }
        unsafe extern "system" fn BufferNotifySize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffernotifysize: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BufferNotifySize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(buffernotifysize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferNotifySize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffernotifysize: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBufferNotifySize(::core::mem::transmute_copy(&buffernotifysize)).into()
        }
        unsafe extern "system" fn EventHandle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandle: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventHandle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eventhandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: SpeechAudioState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetState(::core::mem::transmute_copy(&state)).into()
        }
        Self {
            base__: ISpeechBaseStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            Status: Status::<Identity, Impl, OFFSET>,
            BufferInfo: BufferInfo::<Identity, Impl, OFFSET>,
            DefaultFormat: DefaultFormat::<Identity, Impl, OFFSET>,
            Volume: Volume::<Identity, Impl, OFFSET>,
            SetVolume: SetVolume::<Identity, Impl, OFFSET>,
            BufferNotifySize: BufferNotifySize::<Identity, Impl, OFFSET>,
            SetBufferNotifySize: SetBufferNotifySize::<Identity, Impl, OFFSET>,
            EventHandle: EventHandle::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechAudio as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<ISpeechBaseStream as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechAudioBufferInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn MinNotification(&self) -> ::windows_core::Result<i32>;
    fn SetMinNotification(&self, minnotification: i32) -> ::windows_core::Result<()>;
    fn BufferSize(&self) -> ::windows_core::Result<i32>;
    fn SetBufferSize(&self, buffersize: i32) -> ::windows_core::Result<()>;
    fn EventBias(&self) -> ::windows_core::Result<i32>;
    fn SetEventBias(&self, eventbias: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechAudioBufferInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechAudioBufferInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>() -> ISpeechAudioBufferInfo_Vtbl {
        unsafe extern "system" fn MinNotification<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minnotification: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MinNotification() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minnotification, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinNotification<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minnotification: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMinNotification(::core::mem::transmute_copy(&minnotification)).into()
        }
        unsafe extern "system" fn BufferSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffersize: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BufferSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(buffersize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffersize: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBufferSize(::core::mem::transmute_copy(&buffersize)).into()
        }
        unsafe extern "system" fn EventBias<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventbias: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventBias() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eventbias, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventBias<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventbias: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventBias(::core::mem::transmute_copy(&eventbias)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            MinNotification: MinNotification::<Identity, Impl, OFFSET>,
            SetMinNotification: SetMinNotification::<Identity, Impl, OFFSET>,
            BufferSize: BufferSize::<Identity, Impl, OFFSET>,
            SetBufferSize: SetBufferSize::<Identity, Impl, OFFSET>,
            EventBias: EventBias::<Identity, Impl, OFFSET>,
            SetEventBias: SetEventBias::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechAudioBufferInfo as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechAudioFormat_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&self) -> ::windows_core::Result<SpeechAudioFormatType>;
    fn SetType(&self, audioformat: SpeechAudioFormatType) -> ::windows_core::Result<()>;
    fn Guid(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetGuid(&self, guid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetWaveFormatEx(&self) -> ::windows_core::Result<ISpeechWaveFormatEx>;
    fn SetWaveFormatEx(&self, speechwaveformatex: ::core::option::Option<&ISpeechWaveFormatEx>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechAudioFormat {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechAudioFormat_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudioFormat_Impl, const OFFSET: isize>() -> ISpeechAudioFormat_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioformat: *mut SpeechAudioFormatType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audioformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioformat: SpeechAudioFormatType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetType(::core::mem::transmute_copy(&audioformat)).into()
        }
        unsafe extern "system" fn Guid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Guid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(guid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGuid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGuid(::core::mem::transmute(&guid)).into()
        }
        unsafe extern "system" fn GetWaveFormatEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, speechwaveformatex: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWaveFormatEx() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(speechwaveformatex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWaveFormatEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, speechwaveformatex: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWaveFormatEx(::windows_core::from_raw_borrowed(&speechwaveformatex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            Guid: Guid::<Identity, Impl, OFFSET>,
            SetGuid: SetGuid::<Identity, Impl, OFFSET>,
            GetWaveFormatEx: GetWaveFormatEx::<Identity, Impl, OFFSET>,
            SetWaveFormatEx: SetWaveFormatEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechAudioFormat as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechAudioStatus_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn FreeBufferSpace(&self) -> ::windows_core::Result<i32>;
    fn NonBlockingIO(&self) -> ::windows_core::Result<i32>;
    fn State(&self) -> ::windows_core::Result<SpeechAudioState>;
    fn CurrentSeekPosition(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn CurrentDevicePosition(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechAudioStatus {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechAudioStatus_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudioStatus_Impl, const OFFSET: isize>() -> ISpeechAudioStatus_Vtbl {
        unsafe extern "system" fn FreeBufferSpace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudioStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, freebufferspace: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FreeBufferSpace() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(freebufferspace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NonBlockingIO<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudioStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nonblockingio: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NonBlockingIO() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nonblockingio, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudioStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut SpeechAudioState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSeekPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudioStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentseekposition: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentSeekPosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentseekposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDevicePosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechAudioStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentdeviceposition: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentDevicePosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentdeviceposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            FreeBufferSpace: FreeBufferSpace::<Identity, Impl, OFFSET>,
            NonBlockingIO: NonBlockingIO::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            CurrentSeekPosition: CurrentSeekPosition::<Identity, Impl, OFFSET>,
            CurrentDevicePosition: CurrentDevicePosition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechAudioStatus as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechBaseStream_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Format(&self) -> ::windows_core::Result<ISpeechAudioFormat>;
    fn putref_Format(&self, audioformat: ::core::option::Option<&ISpeechAudioFormat>) -> ::windows_core::Result<()>;
    fn Read(&self, buffer: *mut super::super::System::Variant::VARIANT, numberofbytes: i32, bytesread: *mut i32) -> ::windows_core::Result<()>;
    fn Write(&self, buffer: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<i32>;
    fn Seek(&self, position: &super::super::System::Variant::VARIANT, origin: SpeechStreamSeekPositionType) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechBaseStream {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechBaseStream_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechBaseStream_Impl, const OFFSET: isize>() -> ISpeechBaseStream_Vtbl {
        unsafe extern "system" fn Format<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechBaseStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioformat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Format() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audioformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Format<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechBaseStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioformat: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_Format(::windows_core::from_raw_borrowed(&audioformat)).into()
        }
        unsafe extern "system" fn Read<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechBaseStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: *mut super::super::System::Variant::VARIANT, numberofbytes: i32, bytesread: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Read(::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&numberofbytes), ::core::mem::transmute_copy(&bytesread)).into()
        }
        unsafe extern "system" fn Write<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechBaseStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: super::super::System::Variant::VARIANT, byteswritten: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Write(::core::mem::transmute(&buffer)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(byteswritten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Seek<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechBaseStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: super::super::System::Variant::VARIANT, origin: SpeechStreamSeekPositionType, newposition: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Seek(::core::mem::transmute(&position), ::core::mem::transmute_copy(&origin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Format: Format::<Identity, Impl, OFFSET>,
            putref_Format: putref_Format::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechBaseStream as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechCustomStream_Impl: Sized + ISpeechBaseStream_Impl {
    fn BaseStream(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn putref_BaseStream(&self, punkstream: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechCustomStream {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechCustomStream_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechCustomStream_Impl, const OFFSET: isize>() -> ISpeechCustomStream_Vtbl {
        unsafe extern "system" fn BaseStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechCustomStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunkstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BaseStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_BaseStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechCustomStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_BaseStream(::windows_core::from_raw_borrowed(&punkstream)).into()
        }
        Self {
            base__: ISpeechBaseStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            BaseStream: BaseStream::<Identity, Impl, OFFSET>,
            putref_BaseStream: putref_BaseStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechCustomStream as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<ISpeechBaseStream as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechDataKey_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetBinaryValue(&self, valuename: &::windows_core::BSTR, value: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetBinaryValue(&self, valuename: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetStringValue(&self, valuename: &::windows_core::BSTR, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetStringValue(&self, valuename: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLongValue(&self, valuename: &::windows_core::BSTR, value: i32) -> ::windows_core::Result<()>;
    fn GetLongValue(&self, valuename: &::windows_core::BSTR) -> ::windows_core::Result<i32>;
    fn OpenKey(&self, subkeyname: &::windows_core::BSTR) -> ::windows_core::Result<ISpeechDataKey>;
    fn CreateKey(&self, subkeyname: &::windows_core::BSTR) -> ::windows_core::Result<ISpeechDataKey>;
    fn DeleteKey(&self, subkeyname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DeleteValue(&self, valuename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn EnumKeys(&self, index: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn EnumValues(&self, index: i32) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechDataKey {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechDataKey_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: isize>() -> ISpeechDataKey_Vtbl {
        unsafe extern "system" fn SetBinaryValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBinaryValue(::core::mem::transmute(&valuename), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GetBinaryValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBinaryValue(::core::mem::transmute(&valuename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStringValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStringValue(::core::mem::transmute(&valuename), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GetStringValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStringValue(::core::mem::transmute(&valuename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLongValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLongValue(::core::mem::transmute(&valuename), ::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetLongValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLongValue(::core::mem::transmute(&valuename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subkeyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, subkey: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenKey(::core::mem::transmute(&subkeyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(subkey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subkeyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, subkey: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateKey(::core::mem::transmute(&subkeyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(subkey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subkeyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteKey(::core::mem::transmute(&subkeyname)).into()
        }
        unsafe extern "system" fn DeleteValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteValue(::core::mem::transmute(&valuename)).into()
        }
        unsafe extern "system" fn EnumKeys<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, subkeyname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumKeys(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(subkeyname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumValues<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, valuename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumValues(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(valuename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetBinaryValue: SetBinaryValue::<Identity, Impl, OFFSET>,
            GetBinaryValue: GetBinaryValue::<Identity, Impl, OFFSET>,
            SetStringValue: SetStringValue::<Identity, Impl, OFFSET>,
            GetStringValue: GetStringValue::<Identity, Impl, OFFSET>,
            SetLongValue: SetLongValue::<Identity, Impl, OFFSET>,
            GetLongValue: GetLongValue::<Identity, Impl, OFFSET>,
            OpenKey: OpenKey::<Identity, Impl, OFFSET>,
            CreateKey: CreateKey::<Identity, Impl, OFFSET>,
            DeleteKey: DeleteKey::<Identity, Impl, OFFSET>,
            DeleteValue: DeleteValue::<Identity, Impl, OFFSET>,
            EnumKeys: EnumKeys::<Identity, Impl, OFFSET>,
            EnumValues: EnumValues::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechDataKey as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechFileStream_Impl: Sized + ISpeechBaseStream_Impl {
    fn Open(&self, filename: &::windows_core::BSTR, filemode: SpeechStreamFileMode, doevents: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Close(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechFileStream {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechFileStream_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechFileStream_Impl, const OFFSET: isize>() -> ISpeechFileStream_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechFileStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows_core::BSTR>, filemode: SpeechStreamFileMode, doevents: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open(::core::mem::transmute(&filename), ::core::mem::transmute_copy(&filemode), ::core::mem::transmute_copy(&doevents)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechFileStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        Self { base__: ISpeechBaseStream_Vtbl::new::<Identity, Impl, OFFSET>(), Open: Open::<Identity, Impl, OFFSET>, Close: Close::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechFileStream as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<ISpeechBaseStream as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechGrammarRule_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Attributes(&self) -> ::windows_core::Result<SpeechRuleAttributes>;
    fn InitialState(&self) -> ::windows_core::Result<ISpeechGrammarRuleState>;
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Id(&self) -> ::windows_core::Result<i32>;
    fn Clear(&self) -> ::windows_core::Result<()>;
    fn AddResource(&self, resourcename: &::windows_core::BSTR, resourcevalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddState(&self) -> ::windows_core::Result<ISpeechGrammarRuleState>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechGrammarRule {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechGrammarRule_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRule_Impl, const OFFSET: isize>() -> ISpeechGrammarRule_Vtbl {
        unsafe extern "system" fn Attributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: *mut SpeechRuleAttributes) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Attributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitialState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitialState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        unsafe extern "system" fn AddResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcename: ::std::mem::MaybeUninit<::windows_core::BSTR>, resourcevalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddResource(::core::mem::transmute(&resourcename), ::core::mem::transmute(&resourcevalue)).into()
        }
        unsafe extern "system" fn AddState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Attributes: Attributes::<Identity, Impl, OFFSET>,
            InitialState: InitialState::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            AddResource: AddResource::<Identity, Impl, OFFSET>,
            AddState: AddState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechGrammarRule as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechGrammarRuleState_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Rule(&self) -> ::windows_core::Result<ISpeechGrammarRule>;
    fn Transitions(&self) -> ::windows_core::Result<ISpeechGrammarRuleStateTransitions>;
    fn AddWordTransition(&self, deststate: ::core::option::Option<&ISpeechGrammarRuleState>, words: &::windows_core::BSTR, separators: &::windows_core::BSTR, r#type: SpeechGrammarWordType, propertyname: &::windows_core::BSTR, propertyid: i32, propertyvalue: *const super::super::System::Variant::VARIANT, weight: f32) -> ::windows_core::Result<()>;
    fn AddRuleTransition(&self, destinationstate: ::core::option::Option<&ISpeechGrammarRuleState>, rule: ::core::option::Option<&ISpeechGrammarRule>, propertyname: &::windows_core::BSTR, propertyid: i32, propertyvalue: *const super::super::System::Variant::VARIANT, weight: f32) -> ::windows_core::Result<()>;
    fn AddSpecialTransition(&self, destinationstate: ::core::option::Option<&ISpeechGrammarRuleState>, r#type: SpeechSpecialTransitionType, propertyname: &::windows_core::BSTR, propertyid: i32, propertyvalue: *const super::super::System::Variant::VARIANT, weight: f32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechGrammarRuleState {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechGrammarRuleState_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRuleState_Impl, const OFFSET: isize>() -> ISpeechGrammarRuleState_Vtbl {
        unsafe extern "system" fn Rule<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRuleState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Rule() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transitions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRuleState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transitions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Transitions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transitions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWordTransition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRuleState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deststate: *mut ::core::ffi::c_void, words: ::std::mem::MaybeUninit<::windows_core::BSTR>, separators: ::std::mem::MaybeUninit<::windows_core::BSTR>, r#type: SpeechGrammarWordType, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyid: i32, propertyvalue: *const super::super::System::Variant::VARIANT, weight: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddWordTransition(::windows_core::from_raw_borrowed(&deststate), ::core::mem::transmute(&words), ::core::mem::transmute(&separators), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute(&propertyname), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&propertyvalue), ::core::mem::transmute_copy(&weight)).into()
        }
        unsafe extern "system" fn AddRuleTransition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRuleState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationstate: *mut ::core::ffi::c_void, rule: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyid: i32, propertyvalue: *const super::super::System::Variant::VARIANT, weight: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRuleTransition(::windows_core::from_raw_borrowed(&destinationstate), ::windows_core::from_raw_borrowed(&rule), ::core::mem::transmute(&propertyname), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&propertyvalue), ::core::mem::transmute_copy(&weight)).into()
        }
        unsafe extern "system" fn AddSpecialTransition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRuleState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, destinationstate: *mut ::core::ffi::c_void, r#type: SpeechSpecialTransitionType, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyid: i32, propertyvalue: *const super::super::System::Variant::VARIANT, weight: f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddSpecialTransition(::windows_core::from_raw_borrowed(&destinationstate), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute(&propertyname), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&propertyvalue), ::core::mem::transmute_copy(&weight)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Rule: Rule::<Identity, Impl, OFFSET>,
            Transitions: Transitions::<Identity, Impl, OFFSET>,
            AddWordTransition: AddWordTransition::<Identity, Impl, OFFSET>,
            AddRuleTransition: AddRuleTransition::<Identity, Impl, OFFSET>,
            AddSpecialTransition: AddSpecialTransition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechGrammarRuleState as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechGrammarRuleStateTransition_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&self) -> ::windows_core::Result<SpeechGrammarRuleStateTransitionType>;
    fn Text(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Rule(&self) -> ::windows_core::Result<ISpeechGrammarRule>;
    fn Weight(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PropertyName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn PropertyId(&self) -> ::windows_core::Result<i32>;
    fn PropertyValue(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn NextState(&self) -> ::windows_core::Result<ISpeechGrammarRuleState>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechGrammarRuleStateTransition {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechGrammarRuleStateTransition_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>() -> ISpeechGrammarRuleStateTransition_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: *mut SpeechGrammarRuleStateTransitionType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Text() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(text, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rule<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Rule() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Weight<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weight: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Weight() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(weight, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropertyName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyid: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropertyId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropertyValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NextState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nextstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            Text: Text::<Identity, Impl, OFFSET>,
            Rule: Rule::<Identity, Impl, OFFSET>,
            Weight: Weight::<Identity, Impl, OFFSET>,
            PropertyName: PropertyName::<Identity, Impl, OFFSET>,
            PropertyId: PropertyId::<Identity, Impl, OFFSET>,
            PropertyValue: PropertyValue::<Identity, Impl, OFFSET>,
            NextState: NextState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechGrammarRuleStateTransition as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechGrammarRuleStateTransitions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn Item(&self, index: i32) -> ::windows_core::Result<ISpeechGrammarRuleStateTransition>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechGrammarRuleStateTransitions {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechGrammarRuleStateTransitions_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransitions_Impl, const OFFSET: isize>() -> ISpeechGrammarRuleStateTransitions_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, transition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRuleStateTransitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechGrammarRuleStateTransitions as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechGrammarRules_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn FindRule(&self, rulenameorid: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISpeechGrammarRule>;
    fn Item(&self, index: i32) -> ::windows_core::Result<ISpeechGrammarRule>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Dynamic(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Add(&self, rulename: &::windows_core::BSTR, attributes: SpeechRuleAttributes, ruleid: i32) -> ::windows_core::Result<ISpeechGrammarRule>;
    fn Commit(&self) -> ::windows_core::Result<()>;
    fn CommitAndSave(&self, errortext: *mut ::windows_core::BSTR, savestream: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechGrammarRules {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechGrammarRules_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>() -> ISpeechGrammarRules_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindRule<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rulenameorid: super::super::System::Variant::VARIANT, rule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindRule(::core::mem::transmute(&rulenameorid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, rule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dynamic<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dynamic: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Dynamic() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dynamic, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rulename: ::std::mem::MaybeUninit<::windows_core::BSTR>, attributes: SpeechRuleAttributes, ruleid: i32, rule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Add(::core::mem::transmute(&rulename), ::core::mem::transmute_copy(&attributes), ::core::mem::transmute_copy(&ruleid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit().into()
        }
        unsafe extern "system" fn CommitAndSave<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errortext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, savestream: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CommitAndSave(::core::mem::transmute_copy(&errortext), ::core::mem::transmute_copy(&savestream)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            FindRule: FindRule::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Dynamic: Dynamic::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            CommitAndSave: CommitAndSave::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechGrammarRules as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechLexicon_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GenerationId(&self) -> ::windows_core::Result<i32>;
    fn GetWords(&self, flags: SpeechLexiconType, generationid: *mut i32, words: *mut ::core::option::Option<ISpeechLexiconWords>) -> ::windows_core::Result<()>;
    fn AddPronunciation(&self, bstrword: &::windows_core::BSTR, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddPronunciationByPhoneIds(&self, bstrword: &::windows_core::BSTR, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn RemovePronunciation(&self, bstrword: &::windows_core::BSTR, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RemovePronunciationByPhoneIds(&self, bstrword: &::windows_core::BSTR, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetPronunciations(&self, bstrword: &::windows_core::BSTR, langid: i32, typeflags: SpeechLexiconType) -> ::windows_core::Result<ISpeechLexiconPronunciations>;
    fn GetGenerationChange(&self, generationid: *mut i32, ppwords: *mut ::core::option::Option<ISpeechLexiconWords>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechLexicon {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechLexicon_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexicon_Impl, const OFFSET: isize>() -> ISpeechLexicon_Vtbl {
        unsafe extern "system" fn GenerationId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generationid: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GenerationId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(generationid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWords<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: SpeechLexiconType, generationid: *mut i32, words: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWords(::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&generationid), ::core::mem::transmute_copy(&words)).into()
        }
        unsafe extern "system" fn AddPronunciation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrword: ::std::mem::MaybeUninit<::windows_core::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPronunciation(::core::mem::transmute(&bstrword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&partofspeech), ::core::mem::transmute(&bstrpronunciation)).into()
        }
        unsafe extern "system" fn AddPronunciationByPhoneIds<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrword: ::std::mem::MaybeUninit<::windows_core::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPronunciationByPhoneIds(::core::mem::transmute(&bstrword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&partofspeech), ::core::mem::transmute_copy(&phoneids)).into()
        }
        unsafe extern "system" fn RemovePronunciation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrword: ::std::mem::MaybeUninit<::windows_core::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePronunciation(::core::mem::transmute(&bstrword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&partofspeech), ::core::mem::transmute(&bstrpronunciation)).into()
        }
        unsafe extern "system" fn RemovePronunciationByPhoneIds<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrword: ::std::mem::MaybeUninit<::windows_core::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePronunciationByPhoneIds(::core::mem::transmute(&bstrword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&partofspeech), ::core::mem::transmute_copy(&phoneids)).into()
        }
        unsafe extern "system" fn GetPronunciations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrword: ::std::mem::MaybeUninit<::windows_core::BSTR>, langid: i32, typeflags: SpeechLexiconType, pppronunciations: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPronunciations(::core::mem::transmute(&bstrword), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&typeflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppronunciations, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGenerationChange<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, generationid: *mut i32, ppwords: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGenerationChange(::core::mem::transmute_copy(&generationid), ::core::mem::transmute_copy(&ppwords)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GenerationId: GenerationId::<Identity, Impl, OFFSET>,
            GetWords: GetWords::<Identity, Impl, OFFSET>,
            AddPronunciation: AddPronunciation::<Identity, Impl, OFFSET>,
            AddPronunciationByPhoneIds: AddPronunciationByPhoneIds::<Identity, Impl, OFFSET>,
            RemovePronunciation: RemovePronunciation::<Identity, Impl, OFFSET>,
            RemovePronunciationByPhoneIds: RemovePronunciationByPhoneIds::<Identity, Impl, OFFSET>,
            GetPronunciations: GetPronunciations::<Identity, Impl, OFFSET>,
            GetGenerationChange: GetGenerationChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechLexicon as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechLexiconPronunciation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&self) -> ::windows_core::Result<SpeechLexiconType>;
    fn LangId(&self) -> ::windows_core::Result<i32>;
    fn PartOfSpeech(&self) -> ::windows_core::Result<SpeechPartOfSpeech>;
    fn PhoneIds(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Symbolic(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechLexiconPronunciation {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechLexiconPronunciation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexiconPronunciation_Impl, const OFFSET: isize>() -> ISpeechLexiconPronunciation_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexiconPronunciation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lexicontype: *mut SpeechLexiconType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lexicontype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LangId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexiconPronunciation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LangId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(langid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartOfSpeech<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexiconPronunciation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partofspeech: *mut SpeechPartOfSpeech) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PartOfSpeech() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partofspeech, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneIds<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexiconPronunciation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phoneids: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PhoneIds() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phoneids, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Symbolic<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexiconPronunciation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, symbolic: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Symbolic() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(symbolic, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            LangId: LangId::<Identity, Impl, OFFSET>,
            PartOfSpeech: PartOfSpeech::<Identity, Impl, OFFSET>,
            PhoneIds: PhoneIds::<Identity, Impl, OFFSET>,
            Symbolic: Symbolic::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechLexiconPronunciation as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechLexiconPronunciations_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn Item(&self, index: i32) -> ::windows_core::Result<ISpeechLexiconPronunciation>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechLexiconPronunciations {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechLexiconPronunciations_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexiconPronunciations_Impl, const OFFSET: isize>() -> ISpeechLexiconPronunciations_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexiconPronunciations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexiconPronunciations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pronunciation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pronunciation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexiconPronunciations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechLexiconPronunciations as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechLexiconWord_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn LangId(&self) -> ::windows_core::Result<i32>;
    fn Type(&self) -> ::windows_core::Result<SpeechWordType>;
    fn Word(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Pronunciations(&self) -> ::windows_core::Result<ISpeechLexiconPronunciations>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechLexiconWord {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechLexiconWord_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexiconWord_Impl, const OFFSET: isize>() -> ISpeechLexiconWord_Vtbl {
        unsafe extern "system" fn LangId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexiconWord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, langid: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LangId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(langid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexiconWord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wordtype: *mut SpeechWordType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(wordtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Word<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexiconWord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, word: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Word() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(word, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pronunciations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexiconWord_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pronunciations: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Pronunciations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pronunciations, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            LangId: LangId::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Word: Word::<Identity, Impl, OFFSET>,
            Pronunciations: Pronunciations::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechLexiconWord as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechLexiconWords_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn Item(&self, index: i32) -> ::windows_core::Result<ISpeechLexiconWord>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechLexiconWords {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechLexiconWords_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexiconWords_Impl, const OFFSET: isize>() -> ISpeechLexiconWords_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexiconWords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexiconWords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, word: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(word, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechLexiconWords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechLexiconWords as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechMMSysAudio_Impl: Sized + ISpeechAudio_Impl {
    fn DeviceId(&self) -> ::windows_core::Result<i32>;
    fn SetDeviceId(&self, deviceid: i32) -> ::windows_core::Result<()>;
    fn LineId(&self) -> ::windows_core::Result<i32>;
    fn SetLineId(&self, lineid: i32) -> ::windows_core::Result<()>;
    fn MMHandle(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechMMSysAudio {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechMMSysAudio_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechMMSysAudio_Impl, const OFFSET: isize>() -> ISpeechMMSysAudio_Vtbl {
        unsafe extern "system" fn DeviceId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(deviceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDeviceId(::core::mem::transmute_copy(&deviceid)).into()
        }
        unsafe extern "system" fn LineId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineid: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LineId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lineid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lineid: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLineId(::core::mem::transmute_copy(&lineid)).into()
        }
        unsafe extern "system" fn MMHandle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechMMSysAudio_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MMHandle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(handle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISpeechAudio_Vtbl::new::<Identity, Impl, OFFSET>(),
            DeviceId: DeviceId::<Identity, Impl, OFFSET>,
            SetDeviceId: SetDeviceId::<Identity, Impl, OFFSET>,
            LineId: LineId::<Identity, Impl, OFFSET>,
            SetLineId: SetLineId::<Identity, Impl, OFFSET>,
            MMHandle: MMHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechMMSysAudio as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<ISpeechBaseStream as ::windows_core::ComInterface>::IID || iid == &<ISpeechAudio as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechMemoryStream_Impl: Sized + ISpeechBaseStream_Impl {
    fn SetData(&self, data: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetData(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechMemoryStream {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechMemoryStream_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechMemoryStream_Impl, const OFFSET: isize>() -> ISpeechMemoryStream_Vtbl {
        unsafe extern "system" fn SetData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechMemoryStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetData(::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn GetData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechMemoryStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdata: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISpeechBaseStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetData: SetData::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechMemoryStream as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<ISpeechBaseStream as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechObjectToken_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DataKey(&self) -> ::windows_core::Result<ISpeechDataKey>;
    fn Category(&self) -> ::windows_core::Result<ISpeechObjectTokenCategory>;
    fn GetDescription(&self, locale: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetId(&self, id: &::windows_core::BSTR, categoryid: &::windows_core::BSTR, createifnotexist: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn GetAttribute(&self, attributename: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CreateInstance(&self, punkouter: ::core::option::Option<&::windows_core::IUnknown>, clscontext: SpeechTokenContext) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Remove(&self, objectstorageclsid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetStorageFileName(&self, objectstorageclsid: &::windows_core::BSTR, keyname: &::windows_core::BSTR, filename: &::windows_core::BSTR, folder: SpeechTokenShellFolder) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RemoveStorageFileName(&self, objectstorageclsid: &::windows_core::BSTR, keyname: &::windows_core::BSTR, deletefile: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn IsUISupported(&self, typeofui: &::windows_core::BSTR, extradata: *const super::super::System::Variant::VARIANT, object: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn DisplayUI(&self, hwnd: i32, title: &::windows_core::BSTR, typeofui: &::windows_core::BSTR, extradata: *const super::super::System::Variant::VARIANT, object: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn MatchesAttributes(&self, attributes: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechObjectToken {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechObjectToken_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>() -> ISpeechObjectToken_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objectid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datakey: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DataKey() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(datakey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Category<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Category() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(category, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, locale: i32, description: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDescription(::core::mem::transmute_copy(&locale)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(description, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows_core::BSTR>, categoryid: ::std::mem::MaybeUninit<::windows_core::BSTR>, createifnotexist: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetId(::core::mem::transmute(&id), ::core::mem::transmute(&categoryid), ::core::mem::transmute_copy(&createifnotexist)).into()
        }
        unsafe extern "system" fn GetAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributename: ::std::mem::MaybeUninit<::windows_core::BSTR>, attributevalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAttribute(::core::mem::transmute(&attributename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributevalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, clscontext: SpeechTokenContext, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInstance(::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&clscontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(object, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectstorageclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&objectstorageclsid)).into()
        }
        unsafe extern "system" fn GetStorageFileName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectstorageclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>, keyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, filename: ::std::mem::MaybeUninit<::windows_core::BSTR>, folder: SpeechTokenShellFolder, filepath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStorageFileName(::core::mem::transmute(&objectstorageclsid), ::core::mem::transmute(&keyname), ::core::mem::transmute(&filename), ::core::mem::transmute_copy(&folder)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filepath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStorageFileName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectstorageclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>, keyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, deletefile: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveStorageFileName(::core::mem::transmute(&objectstorageclsid), ::core::mem::transmute(&keyname), ::core::mem::transmute_copy(&deletefile)).into()
        }
        unsafe extern "system" fn IsUISupported<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typeofui: ::std::mem::MaybeUninit<::windows_core::BSTR>, extradata: *const super::super::System::Variant::VARIANT, object: *mut ::core::ffi::c_void, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsUISupported(::core::mem::transmute(&typeofui), ::core::mem::transmute_copy(&extradata), ::windows_core::from_raw_borrowed(&object)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: i32, title: ::std::mem::MaybeUninit<::windows_core::BSTR>, typeofui: ::std::mem::MaybeUninit<::windows_core::BSTR>, extradata: *const super::super::System::Variant::VARIANT, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisplayUI(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&title), ::core::mem::transmute(&typeofui), ::core::mem::transmute_copy(&extradata), ::windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn MatchesAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, matches: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MatchesAttributes(::core::mem::transmute(&attributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matches, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            DataKey: DataKey::<Identity, Impl, OFFSET>,
            Category: Category::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
            GetAttribute: GetAttribute::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            GetStorageFileName: GetStorageFileName::<Identity, Impl, OFFSET>,
            RemoveStorageFileName: RemoveStorageFileName::<Identity, Impl, OFFSET>,
            IsUISupported: IsUISupported::<Identity, Impl, OFFSET>,
            DisplayUI: DisplayUI::<Identity, Impl, OFFSET>,
            MatchesAttributes: MatchesAttributes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechObjectToken as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechObjectTokenCategory_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDefault(&self, tokenid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Default(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetId(&self, id: &::windows_core::BSTR, createifnotexist: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn GetDataKey(&self, location: SpeechDataKeyLocation) -> ::windows_core::Result<ISpeechDataKey>;
    fn EnumerateTokens(&self, requiredattributes: &::windows_core::BSTR, optionalattributes: &::windows_core::BSTR) -> ::windows_core::Result<ISpeechObjectTokens>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechObjectTokenCategory {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechObjectTokenCategory_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>() -> ISpeechObjectTokenCategory_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefault<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tokenid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefault(::core::mem::transmute(&tokenid)).into()
        }
        unsafe extern "system" fn Default<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tokenid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Default() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tokenid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows_core::BSTR>, createifnotexist: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetId(::core::mem::transmute(&id), ::core::mem::transmute_copy(&createifnotexist)).into()
        }
        unsafe extern "system" fn GetDataKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: SpeechDataKeyLocation, datakey: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDataKey(::core::mem::transmute_copy(&location)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(datakey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateTokens<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionalattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, tokens: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerateTokens(::core::mem::transmute(&requiredattributes), ::core::mem::transmute(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tokens, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            SetDefault: SetDefault::<Identity, Impl, OFFSET>,
            Default: Default::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
            GetDataKey: GetDataKey::<Identity, Impl, OFFSET>,
            EnumerateTokens: EnumerateTokens::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechObjectTokenCategory as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechObjectTokens_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn Item(&self, index: i32) -> ::windows_core::Result<ISpeechObjectToken>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechObjectTokens {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechObjectTokens_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectTokens_Impl, const OFFSET: isize>() -> ISpeechObjectTokens_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, token: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(token, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechObjectTokens_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechObjectTokens as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhoneConverter_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn LanguageId(&self) -> ::windows_core::Result<i32>;
    fn SetLanguageId(&self, languageid: i32) -> ::windows_core::Result<()>;
    fn PhoneToId(&self, phonemes: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn IdToPhone(&self, idarray: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechPhoneConverter {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhoneConverter_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhoneConverter_Impl, const OFFSET: isize>() -> ISpeechPhoneConverter_Vtbl {
        unsafe extern "system" fn LanguageId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhoneConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageid: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LanguageId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(languageid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguageId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhoneConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageid: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLanguageId(::core::mem::transmute_copy(&languageid)).into()
        }
        unsafe extern "system" fn PhoneToId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhoneConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phonemes: ::std::mem::MaybeUninit<::windows_core::BSTR>, idarray: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PhoneToId(::core::mem::transmute(&phonemes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(idarray, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IdToPhone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhoneConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, idarray: super::super::System::Variant::VARIANT, phonemes: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IdToPhone(::core::mem::transmute(&idarray)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phonemes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            LanguageId: LanguageId::<Identity, Impl, OFFSET>,
            SetLanguageId: SetLanguageId::<Identity, Impl, OFFSET>,
            PhoneToId: PhoneToId::<Identity, Impl, OFFSET>,
            IdToPhone: IdToPhone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechPhoneConverter as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseAlternate_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RecoResult(&self) -> ::windows_core::Result<ISpeechRecoResult>;
    fn StartElementInResult(&self) -> ::windows_core::Result<i32>;
    fn NumberOfElementsInResult(&self) -> ::windows_core::Result<i32>;
    fn PhraseInfo(&self) -> ::windows_core::Result<ISpeechPhraseInfo>;
    fn Commit(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechPhraseAlternate {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseAlternate_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseAlternate_Impl, const OFFSET: isize>() -> ISpeechPhraseAlternate_Vtbl {
        unsafe extern "system" fn RecoResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recoresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RecoResult() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(recoresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartElementInResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartElementInResult() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(startelement, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfElementsInResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NumberOfElementsInResult() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(numberofelements, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhraseInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phraseinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PhraseInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phraseinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseAlternate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RecoResult: RecoResult::<Identity, Impl, OFFSET>,
            StartElementInResult: StartElementInResult::<Identity, Impl, OFFSET>,
            NumberOfElementsInResult: NumberOfElementsInResult::<Identity, Impl, OFFSET>,
            PhraseInfo: PhraseInfo::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseAlternate as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseAlternates_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn Item(&self, index: i32) -> ::windows_core::Result<ISpeechPhraseAlternate>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechPhraseAlternates {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseAlternates_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseAlternates_Impl, const OFFSET: isize>() -> ISpeechPhraseAlternates_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, phrasealternate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrasealternate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseAlternates_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseAlternates as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseElement_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AudioTimeOffset(&self) -> ::windows_core::Result<i32>;
    fn AudioSizeTime(&self) -> ::windows_core::Result<i32>;
    fn AudioStreamOffset(&self) -> ::windows_core::Result<i32>;
    fn AudioSizeBytes(&self) -> ::windows_core::Result<i32>;
    fn RetainedStreamOffset(&self) -> ::windows_core::Result<i32>;
    fn RetainedSizeBytes(&self) -> ::windows_core::Result<i32>;
    fn DisplayText(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LexicalForm(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Pronunciation(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn DisplayAttributes(&self) -> ::windows_core::Result<SpeechDisplayAttributes>;
    fn RequiredConfidence(&self) -> ::windows_core::Result<SpeechEngineConfidence>;
    fn ActualConfidence(&self) -> ::windows_core::Result<SpeechEngineConfidence>;
    fn EngineConfidence(&self) -> ::windows_core::Result<f32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechPhraseElement {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseElement_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>() -> ISpeechPhraseElement_Vtbl {
        unsafe extern "system" fn AudioTimeOffset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiotimeoffset: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AudioTimeOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audiotimeoffset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioSizeTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiosizetime: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AudioSizeTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audiosizetime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioStreamOffset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiostreamoffset: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AudioStreamOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audiostreamoffset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioSizeBytes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiosizebytes: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AudioSizeBytes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audiosizebytes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainedStreamOffset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retainedstreamoffset: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RetainedStreamOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retainedstreamoffset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainedSizeBytes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retainedsizebytes: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RetainedSizeBytes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retainedsizebytes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displaytext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(displaytext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LexicalForm<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lexicalform: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LexicalForm() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lexicalform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pronunciation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pronunciation: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Pronunciation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pronunciation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayattributes: *mut SpeechDisplayAttributes) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(displayattributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequiredConfidence<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredconfidence: *mut SpeechEngineConfidence) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RequiredConfidence() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requiredconfidence, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActualConfidence<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actualconfidence: *mut SpeechEngineConfidence) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActualConfidence() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(actualconfidence, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EngineConfidence<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, engineconfidence: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EngineConfidence() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(engineconfidence, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AudioTimeOffset: AudioTimeOffset::<Identity, Impl, OFFSET>,
            AudioSizeTime: AudioSizeTime::<Identity, Impl, OFFSET>,
            AudioStreamOffset: AudioStreamOffset::<Identity, Impl, OFFSET>,
            AudioSizeBytes: AudioSizeBytes::<Identity, Impl, OFFSET>,
            RetainedStreamOffset: RetainedStreamOffset::<Identity, Impl, OFFSET>,
            RetainedSizeBytes: RetainedSizeBytes::<Identity, Impl, OFFSET>,
            DisplayText: DisplayText::<Identity, Impl, OFFSET>,
            LexicalForm: LexicalForm::<Identity, Impl, OFFSET>,
            Pronunciation: Pronunciation::<Identity, Impl, OFFSET>,
            DisplayAttributes: DisplayAttributes::<Identity, Impl, OFFSET>,
            RequiredConfidence: RequiredConfidence::<Identity, Impl, OFFSET>,
            ActualConfidence: ActualConfidence::<Identity, Impl, OFFSET>,
            EngineConfidence: EngineConfidence::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseElement as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseElements_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn Item(&self, index: i32) -> ::windows_core::Result<ISpeechPhraseElement>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechPhraseElements {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseElements_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseElements_Impl, const OFFSET: isize>() -> ISpeechPhraseElements_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseElements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseElements as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn LanguageId(&self) -> ::windows_core::Result<i32>;
    fn GrammarId(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn StartTime(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AudioStreamPosition(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AudioSizeBytes(&self) -> ::windows_core::Result<i32>;
    fn RetainedSizeBytes(&self) -> ::windows_core::Result<i32>;
    fn AudioSizeTime(&self) -> ::windows_core::Result<i32>;
    fn Rule(&self) -> ::windows_core::Result<ISpeechPhraseRule>;
    fn Properties(&self) -> ::windows_core::Result<ISpeechPhraseProperties>;
    fn Elements(&self) -> ::windows_core::Result<ISpeechPhraseElements>;
    fn Replacements(&self) -> ::windows_core::Result<ISpeechPhraseReplacements>;
    fn EngineId(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn EnginePrivateData(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SaveToMemory(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetText(&self, startelement: i32, elements: i32, usereplacements: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDisplayAttributes(&self, startelement: i32, elements: i32, usereplacements: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<SpeechDisplayAttributes>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechPhraseInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>() -> ISpeechPhraseInfo_Vtbl {
        unsafe extern "system" fn LanguageId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageid: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LanguageId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(languageid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GrammarId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grammarid: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GrammarId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(grammarid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(starttime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioStreamPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiostreamposition: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AudioStreamPosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audiostreamposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioSizeBytes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paudiosizebytes: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AudioSizeBytes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(paudiosizebytes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainedSizeBytes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retainedsizebytes: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RetainedSizeBytes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retainedsizebytes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioSizeTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiosizetime: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AudioSizeTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audiosizetime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rule<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Rule() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, properties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Properties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(properties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Elements<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, elements: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Elements() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(elements, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Replacements<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, replacements: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Replacements() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(replacements, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EngineId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, engineidguid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EngineId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(engineidguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnginePrivateData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, privatedata: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnginePrivateData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(privatedata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveToMemory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phraseblock: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SaveToMemory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phraseblock, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, usereplacements: super::super::Foundation::VARIANT_BOOL, text: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetText(::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements), ::core::mem::transmute_copy(&usereplacements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(text, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, usereplacements: super::super::Foundation::VARIANT_BOOL, displayattributes: *mut SpeechDisplayAttributes) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDisplayAttributes(::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements), ::core::mem::transmute_copy(&usereplacements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(displayattributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            LanguageId: LanguageId::<Identity, Impl, OFFSET>,
            GrammarId: GrammarId::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            AudioStreamPosition: AudioStreamPosition::<Identity, Impl, OFFSET>,
            AudioSizeBytes: AudioSizeBytes::<Identity, Impl, OFFSET>,
            RetainedSizeBytes: RetainedSizeBytes::<Identity, Impl, OFFSET>,
            AudioSizeTime: AudioSizeTime::<Identity, Impl, OFFSET>,
            Rule: Rule::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Elements: Elements::<Identity, Impl, OFFSET>,
            Replacements: Replacements::<Identity, Impl, OFFSET>,
            EngineId: EngineId::<Identity, Impl, OFFSET>,
            EnginePrivateData: EnginePrivateData::<Identity, Impl, OFFSET>,
            SaveToMemory: SaveToMemory::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            GetDisplayAttributes: GetDisplayAttributes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseInfo as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseInfoBuilder_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RestorePhraseFromMemory(&self, phraseinmemory: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISpeechPhraseInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechPhraseInfoBuilder {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseInfoBuilder_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseInfoBuilder_Impl, const OFFSET: isize>() -> ISpeechPhraseInfoBuilder_Vtbl {
        unsafe extern "system" fn RestorePhraseFromMemory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseInfoBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phraseinmemory: *const super::super::System::Variant::VARIANT, phraseinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RestorePhraseFromMemory(::core::mem::transmute_copy(&phraseinmemory)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phraseinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RestorePhraseFromMemory: RestorePhraseFromMemory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseInfoBuilder as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseProperties_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn Item(&self, index: i32) -> ::windows_core::Result<ISpeechPhraseProperty>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechPhraseProperties {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseProperties_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseProperties_Impl, const OFFSET: isize>() -> ISpeechPhraseProperties_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, property: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(property, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseProperties as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseProperty_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Id(&self) -> ::windows_core::Result<i32>;
    fn Value(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn FirstElement(&self) -> ::windows_core::Result<i32>;
    fn NumberOfElements(&self) -> ::windows_core::Result<i32>;
    fn EngineConfidence(&self) -> ::windows_core::Result<f32>;
    fn Confidence(&self) -> ::windows_core::Result<SpeechEngineConfidence>;
    fn Parent(&self) -> ::windows_core::Result<ISpeechPhraseProperty>;
    fn Children(&self) -> ::windows_core::Result<ISpeechPhraseProperties>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechPhraseProperty {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseProperty_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>() -> ISpeechPhraseProperty_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, firstelement: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FirstElement() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(firstelement, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfElements<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NumberOfElements() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(numberofelements, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EngineConfidence<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confidence: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EngineConfidence() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(confidence, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Confidence<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, confidence: *mut SpeechEngineConfidence) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Confidence() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(confidence, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentproperty: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Parent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parentproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, children: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Children() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(children, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            FirstElement: FirstElement::<Identity, Impl, OFFSET>,
            NumberOfElements: NumberOfElements::<Identity, Impl, OFFSET>,
            EngineConfidence: EngineConfidence::<Identity, Impl, OFFSET>,
            Confidence: Confidence::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            Children: Children::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseProperty as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseReplacement_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DisplayAttributes(&self) -> ::windows_core::Result<SpeechDisplayAttributes>;
    fn Text(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn FirstElement(&self) -> ::windows_core::Result<i32>;
    fn NumberOfElements(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechPhraseReplacement {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseReplacement_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseReplacement_Impl, const OFFSET: isize>() -> ISpeechPhraseReplacement_Vtbl {
        unsafe extern "system" fn DisplayAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseReplacement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayattributes: *mut SpeechDisplayAttributes) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(displayattributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseReplacement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Text() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(text, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseReplacement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, firstelement: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FirstElement() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(firstelement, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfElements<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseReplacement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NumberOfElements() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(numberofelements, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            DisplayAttributes: DisplayAttributes::<Identity, Impl, OFFSET>,
            Text: Text::<Identity, Impl, OFFSET>,
            FirstElement: FirstElement::<Identity, Impl, OFFSET>,
            NumberOfElements: NumberOfElements::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseReplacement as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseReplacements_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn Item(&self, index: i32) -> ::windows_core::Result<ISpeechPhraseReplacement>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechPhraseReplacements {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseReplacements_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseReplacements_Impl, const OFFSET: isize>() -> ISpeechPhraseReplacements_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseReplacements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseReplacements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, reps: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reps, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseReplacements_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseReplacements as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseRule_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Id(&self) -> ::windows_core::Result<i32>;
    fn FirstElement(&self) -> ::windows_core::Result<i32>;
    fn NumberOfElements(&self) -> ::windows_core::Result<i32>;
    fn Parent(&self) -> ::windows_core::Result<ISpeechPhraseRule>;
    fn Children(&self) -> ::windows_core::Result<ISpeechPhraseRules>;
    fn Confidence(&self) -> ::windows_core::Result<SpeechEngineConfidence>;
    fn EngineConfidence(&self) -> ::windows_core::Result<f32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechPhraseRule {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseRule_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>() -> ISpeechPhraseRule_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstElement<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, firstelement: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FirstElement() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(firstelement, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfElements<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberofelements: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NumberOfElements() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(numberofelements, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Parent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, children: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Children() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(children, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Confidence<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actualconfidence: *mut SpeechEngineConfidence) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Confidence() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(actualconfidence, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EngineConfidence<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, engineconfidence: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EngineConfidence() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(engineconfidence, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            FirstElement: FirstElement::<Identity, Impl, OFFSET>,
            NumberOfElements: NumberOfElements::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            Children: Children::<Identity, Impl, OFFSET>,
            Confidence: Confidence::<Identity, Impl, OFFSET>,
            EngineConfidence: EngineConfidence::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseRule as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseRules_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn Item(&self, index: i32) -> ::windows_core::Result<ISpeechPhraseRule>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechPhraseRules {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseRules_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseRules_Impl, const OFFSET: isize>() -> ISpeechPhraseRules_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, rule: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechPhraseRules_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseRules as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecoContext_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Recognizer(&self) -> ::windows_core::Result<ISpeechRecognizer>;
    fn AudioInputInterferenceStatus(&self) -> ::windows_core::Result<SpeechInterference>;
    fn RequestedUIType(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn putref_Voice(&self, voice: ::core::option::Option<&ISpeechVoice>) -> ::windows_core::Result<()>;
    fn Voice(&self) -> ::windows_core::Result<ISpeechVoice>;
    fn SetAllowVoiceFormatMatchingOnNextSet(&self, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AllowVoiceFormatMatchingOnNextSet(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetVoicePurgeEvent(&self, eventinterest: SpeechRecoEvents) -> ::windows_core::Result<()>;
    fn VoicePurgeEvent(&self) -> ::windows_core::Result<SpeechRecoEvents>;
    fn SetEventInterests(&self, eventinterest: SpeechRecoEvents) -> ::windows_core::Result<()>;
    fn EventInterests(&self) -> ::windows_core::Result<SpeechRecoEvents>;
    fn SetCmdMaxAlternates(&self, maxalternates: i32) -> ::windows_core::Result<()>;
    fn CmdMaxAlternates(&self) -> ::windows_core::Result<i32>;
    fn SetState(&self, state: SpeechRecoContextState) -> ::windows_core::Result<()>;
    fn State(&self) -> ::windows_core::Result<SpeechRecoContextState>;
    fn SetRetainedAudio(&self, option: SpeechRetainedAudioOptions) -> ::windows_core::Result<()>;
    fn RetainedAudio(&self) -> ::windows_core::Result<SpeechRetainedAudioOptions>;
    fn putref_RetainedAudioFormat(&self, format: ::core::option::Option<&ISpeechAudioFormat>) -> ::windows_core::Result<()>;
    fn RetainedAudioFormat(&self) -> ::windows_core::Result<ISpeechAudioFormat>;
    fn Pause(&self) -> ::windows_core::Result<()>;
    fn Resume(&self) -> ::windows_core::Result<()>;
    fn CreateGrammar(&self, grammarid: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISpeechRecoGrammar>;
    fn CreateResultFromMemory(&self, resultblock: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<ISpeechRecoResult>;
    fn Bookmark(&self, options: SpeechBookmarkOptions, streampos: &super::super::System::Variant::VARIANT, bookmarkid: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetAdaptationData(&self, adaptationstring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechRecoContext {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechRecoContext_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>() -> ISpeechRecoContext_Vtbl {
        unsafe extern "system" fn Recognizer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Recognizer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(recognizer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioInputInterferenceStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interference: *mut SpeechInterference) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AudioInputInterferenceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(interference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedUIType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uitype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RequestedUIType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uitype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Voice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_Voice(::windows_core::from_raw_borrowed(&voice)).into()
        }
        unsafe extern "system" fn Voice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Voice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(voice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowVoiceFormatMatchingOnNextSet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllowVoiceFormatMatchingOnNextSet(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowVoiceFormatMatchingOnNextSet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pallow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AllowVoiceFormatMatchingOnNextSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pallow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVoicePurgeEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterest: SpeechRecoEvents) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVoicePurgeEvent(::core::mem::transmute_copy(&eventinterest)).into()
        }
        unsafe extern "system" fn VoicePurgeEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterest: *mut SpeechRecoEvents) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VoicePurgeEvent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eventinterest, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventInterests<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterest: SpeechRecoEvents) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventInterests(::core::mem::transmute_copy(&eventinterest)).into()
        }
        unsafe extern "system" fn EventInterests<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterest: *mut SpeechRecoEvents) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventInterests() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eventinterest, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCmdMaxAlternates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxalternates: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCmdMaxAlternates(::core::mem::transmute_copy(&maxalternates)).into()
        }
        unsafe extern "system" fn CmdMaxAlternates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxalternates: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CmdMaxAlternates() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxalternates, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: SpeechRecoContextState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn State<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut SpeechRecoContextState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRetainedAudio<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: SpeechRetainedAudioOptions) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRetainedAudio(::core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn RetainedAudio<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: *mut SpeechRetainedAudioOptions) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RetainedAudio() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(option, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_RetainedAudioFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_RetainedAudioFormat(::windows_core::from_raw_borrowed(&format)).into()
        }
        unsafe extern "system" fn RetainedAudioFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RetainedAudioFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(format, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resume().into()
        }
        unsafe extern "system" fn CreateGrammar<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grammarid: super::super::System::Variant::VARIANT, grammar: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateGrammar(::core::mem::transmute(&grammarid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(grammar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResultFromMemory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultblock: *const super::super::System::Variant::VARIANT, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateResultFromMemory(::core::mem::transmute_copy(&resultblock)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bookmark<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: SpeechBookmarkOptions, streampos: super::super::System::Variant::VARIANT, bookmarkid: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Bookmark(::core::mem::transmute_copy(&options), ::core::mem::transmute(&streampos), ::core::mem::transmute(&bookmarkid)).into()
        }
        unsafe extern "system" fn SetAdaptationData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adaptationstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAdaptationData(::core::mem::transmute(&adaptationstring)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Recognizer: Recognizer::<Identity, Impl, OFFSET>,
            AudioInputInterferenceStatus: AudioInputInterferenceStatus::<Identity, Impl, OFFSET>,
            RequestedUIType: RequestedUIType::<Identity, Impl, OFFSET>,
            putref_Voice: putref_Voice::<Identity, Impl, OFFSET>,
            Voice: Voice::<Identity, Impl, OFFSET>,
            SetAllowVoiceFormatMatchingOnNextSet: SetAllowVoiceFormatMatchingOnNextSet::<Identity, Impl, OFFSET>,
            AllowVoiceFormatMatchingOnNextSet: AllowVoiceFormatMatchingOnNextSet::<Identity, Impl, OFFSET>,
            SetVoicePurgeEvent: SetVoicePurgeEvent::<Identity, Impl, OFFSET>,
            VoicePurgeEvent: VoicePurgeEvent::<Identity, Impl, OFFSET>,
            SetEventInterests: SetEventInterests::<Identity, Impl, OFFSET>,
            EventInterests: EventInterests::<Identity, Impl, OFFSET>,
            SetCmdMaxAlternates: SetCmdMaxAlternates::<Identity, Impl, OFFSET>,
            CmdMaxAlternates: CmdMaxAlternates::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            SetRetainedAudio: SetRetainedAudio::<Identity, Impl, OFFSET>,
            RetainedAudio: RetainedAudio::<Identity, Impl, OFFSET>,
            putref_RetainedAudioFormat: putref_RetainedAudioFormat::<Identity, Impl, OFFSET>,
            RetainedAudioFormat: RetainedAudioFormat::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            CreateGrammar: CreateGrammar::<Identity, Impl, OFFSET>,
            CreateResultFromMemory: CreateResultFromMemory::<Identity, Impl, OFFSET>,
            Bookmark: Bookmark::<Identity, Impl, OFFSET>,
            SetAdaptationData: SetAdaptationData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechRecoContext as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecoGrammar_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn RecoContext(&self) -> ::windows_core::Result<ISpeechRecoContext>;
    fn SetState(&self, state: SpeechGrammarState) -> ::windows_core::Result<()>;
    fn State(&self) -> ::windows_core::Result<SpeechGrammarState>;
    fn Rules(&self) -> ::windows_core::Result<ISpeechGrammarRules>;
    fn Reset(&self, newlanguage: i32) -> ::windows_core::Result<()>;
    fn CmdLoadFromFile(&self, filename: &::windows_core::BSTR, loadoption: SpeechLoadOption) -> ::windows_core::Result<()>;
    fn CmdLoadFromObject(&self, classid: &::windows_core::BSTR, grammarname: &::windows_core::BSTR, loadoption: SpeechLoadOption) -> ::windows_core::Result<()>;
    fn CmdLoadFromResource(&self, hmodule: i32, resourcename: &super::super::System::Variant::VARIANT, resourcetype: &super::super::System::Variant::VARIANT, languageid: i32, loadoption: SpeechLoadOption) -> ::windows_core::Result<()>;
    fn CmdLoadFromMemory(&self, grammardata: &super::super::System::Variant::VARIANT, loadoption: SpeechLoadOption) -> ::windows_core::Result<()>;
    fn CmdLoadFromProprietaryGrammar(&self, proprietaryguid: &::windows_core::BSTR, proprietarystring: &::windows_core::BSTR, proprietarydata: &super::super::System::Variant::VARIANT, loadoption: SpeechLoadOption) -> ::windows_core::Result<()>;
    fn CmdSetRuleState(&self, name: &::windows_core::BSTR, state: SpeechRuleState) -> ::windows_core::Result<()>;
    fn CmdSetRuleIdState(&self, ruleid: i32, state: SpeechRuleState) -> ::windows_core::Result<()>;
    fn DictationLoad(&self, topicname: &::windows_core::BSTR, loadoption: SpeechLoadOption) -> ::windows_core::Result<()>;
    fn DictationUnload(&self) -> ::windows_core::Result<()>;
    fn DictationSetState(&self, state: SpeechRuleState) -> ::windows_core::Result<()>;
    fn SetWordSequenceData(&self, text: &::windows_core::BSTR, textlength: i32, info: ::core::option::Option<&ISpeechTextSelectionInformation>) -> ::windows_core::Result<()>;
    fn SetTextSelection(&self, info: ::core::option::Option<&ISpeechTextSelectionInformation>) -> ::windows_core::Result<()>;
    fn IsPronounceable(&self, word: &::windows_core::BSTR) -> ::windows_core::Result<SpeechWordPronounceable>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechRecoGrammar {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechRecoGrammar_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>() -> ISpeechRecoGrammar_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecoContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recocontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RecoContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(recocontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: SpeechGrammarState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn State<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut SpeechGrammarState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rules<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rules: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Rules() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rules, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newlanguage: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset(::core::mem::transmute_copy(&newlanguage)).into()
        }
        unsafe extern "system" fn CmdLoadFromFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows_core::BSTR>, loadoption: SpeechLoadOption) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CmdLoadFromFile(::core::mem::transmute(&filename), ::core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn CmdLoadFromObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classid: ::std::mem::MaybeUninit<::windows_core::BSTR>, grammarname: ::std::mem::MaybeUninit<::windows_core::BSTR>, loadoption: SpeechLoadOption) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CmdLoadFromObject(::core::mem::transmute(&classid), ::core::mem::transmute(&grammarname), ::core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn CmdLoadFromResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hmodule: i32, resourcename: super::super::System::Variant::VARIANT, resourcetype: super::super::System::Variant::VARIANT, languageid: i32, loadoption: SpeechLoadOption) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CmdLoadFromResource(::core::mem::transmute_copy(&hmodule), ::core::mem::transmute(&resourcename), ::core::mem::transmute(&resourcetype), ::core::mem::transmute_copy(&languageid), ::core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn CmdLoadFromMemory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grammardata: super::super::System::Variant::VARIANT, loadoption: SpeechLoadOption) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CmdLoadFromMemory(::core::mem::transmute(&grammardata), ::core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn CmdLoadFromProprietaryGrammar<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, proprietaryguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, proprietarystring: ::std::mem::MaybeUninit<::windows_core::BSTR>, proprietarydata: super::super::System::Variant::VARIANT, loadoption: SpeechLoadOption) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CmdLoadFromProprietaryGrammar(::core::mem::transmute(&proprietaryguid), ::core::mem::transmute(&proprietarystring), ::core::mem::transmute(&proprietarydata), ::core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn CmdSetRuleState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, state: SpeechRuleState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CmdSetRuleState(::core::mem::transmute(&name), ::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn CmdSetRuleIdState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ruleid: i32, state: SpeechRuleState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CmdSetRuleIdState(::core::mem::transmute_copy(&ruleid), ::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn DictationLoad<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topicname: ::std::mem::MaybeUninit<::windows_core::BSTR>, loadoption: SpeechLoadOption) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DictationLoad(::core::mem::transmute(&topicname), ::core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn DictationUnload<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DictationUnload().into()
        }
        unsafe extern "system" fn DictationSetState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: SpeechRuleState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DictationSetState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn SetWordSequenceData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::BSTR>, textlength: i32, info: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWordSequenceData(::core::mem::transmute(&text), ::core::mem::transmute_copy(&textlength), ::windows_core::from_raw_borrowed(&info)).into()
        }
        unsafe extern "system" fn SetTextSelection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, info: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTextSelection(::windows_core::from_raw_borrowed(&info)).into()
        }
        unsafe extern "system" fn IsPronounceable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, word: ::std::mem::MaybeUninit<::windows_core::BSTR>, wordpronounceable: *mut SpeechWordPronounceable) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPronounceable(::core::mem::transmute(&word)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(wordpronounceable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            RecoContext: RecoContext::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Rules: Rules::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            CmdLoadFromFile: CmdLoadFromFile::<Identity, Impl, OFFSET>,
            CmdLoadFromObject: CmdLoadFromObject::<Identity, Impl, OFFSET>,
            CmdLoadFromResource: CmdLoadFromResource::<Identity, Impl, OFFSET>,
            CmdLoadFromMemory: CmdLoadFromMemory::<Identity, Impl, OFFSET>,
            CmdLoadFromProprietaryGrammar: CmdLoadFromProprietaryGrammar::<Identity, Impl, OFFSET>,
            CmdSetRuleState: CmdSetRuleState::<Identity, Impl, OFFSET>,
            CmdSetRuleIdState: CmdSetRuleIdState::<Identity, Impl, OFFSET>,
            DictationLoad: DictationLoad::<Identity, Impl, OFFSET>,
            DictationUnload: DictationUnload::<Identity, Impl, OFFSET>,
            DictationSetState: DictationSetState::<Identity, Impl, OFFSET>,
            SetWordSequenceData: SetWordSequenceData::<Identity, Impl, OFFSET>,
            SetTextSelection: SetTextSelection::<Identity, Impl, OFFSET>,
            IsPronounceable: IsPronounceable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechRecoGrammar as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecoResult_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RecoContext(&self) -> ::windows_core::Result<ISpeechRecoContext>;
    fn Times(&self) -> ::windows_core::Result<ISpeechRecoResultTimes>;
    fn putref_AudioFormat(&self, format: ::core::option::Option<&ISpeechAudioFormat>) -> ::windows_core::Result<()>;
    fn AudioFormat(&self) -> ::windows_core::Result<ISpeechAudioFormat>;
    fn PhraseInfo(&self) -> ::windows_core::Result<ISpeechPhraseInfo>;
    fn Alternates(&self, requestcount: i32, startelement: i32, elements: i32) -> ::windows_core::Result<ISpeechPhraseAlternates>;
    fn Audio(&self, startelement: i32, elements: i32) -> ::windows_core::Result<ISpeechMemoryStream>;
    fn SpeakAudio(&self, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags) -> ::windows_core::Result<i32>;
    fn SaveToMemory(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn DiscardResultInfo(&self, valuetypes: SpeechDiscardType) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechRecoResult {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechRecoResult_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResult_Impl, const OFFSET: isize>() -> ISpeechRecoResult_Vtbl {
        unsafe extern "system" fn RecoContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recocontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RecoContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(recocontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Times<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, times: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Times() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(times, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_AudioFormat(::windows_core::from_raw_borrowed(&format)).into()
        }
        unsafe extern "system" fn AudioFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AudioFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(format, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhraseInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phraseinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PhraseInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phraseinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Alternates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestcount: i32, startelement: i32, elements: i32, alternates: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Alternates(::core::mem::transmute_copy(&requestcount), ::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(alternates, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Audio<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Audio(::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakAudio<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SpeakAudio(::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(streamnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveToMemory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultblock: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SaveToMemory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultblock, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscardResultInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuetypes: SpeechDiscardType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DiscardResultInfo(::core::mem::transmute_copy(&valuetypes)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RecoContext: RecoContext::<Identity, Impl, OFFSET>,
            Times: Times::<Identity, Impl, OFFSET>,
            putref_AudioFormat: putref_AudioFormat::<Identity, Impl, OFFSET>,
            AudioFormat: AudioFormat::<Identity, Impl, OFFSET>,
            PhraseInfo: PhraseInfo::<Identity, Impl, OFFSET>,
            Alternates: Alternates::<Identity, Impl, OFFSET>,
            Audio: Audio::<Identity, Impl, OFFSET>,
            SpeakAudio: SpeakAudio::<Identity, Impl, OFFSET>,
            SaveToMemory: SaveToMemory::<Identity, Impl, OFFSET>,
            DiscardResultInfo: DiscardResultInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechRecoResult as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecoResult2_Impl: Sized + ISpeechRecoResult_Impl {
    fn SetTextFeedback(&self, feedback: &::windows_core::BSTR, wassuccessful: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechRecoResult2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechRecoResult2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResult2_Impl, const OFFSET: isize>() -> ISpeechRecoResult2_Vtbl {
        unsafe extern "system" fn SetTextFeedback<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedback: ::std::mem::MaybeUninit<::windows_core::BSTR>, wassuccessful: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTextFeedback(::core::mem::transmute(&feedback), ::core::mem::transmute_copy(&wassuccessful)).into()
        }
        Self { base__: ISpeechRecoResult_Vtbl::new::<Identity, Impl, OFFSET>(), SetTextFeedback: SetTextFeedback::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechRecoResult2 as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<ISpeechRecoResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecoResultDispatch_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RecoContext(&self) -> ::windows_core::Result<ISpeechRecoContext>;
    fn Times(&self) -> ::windows_core::Result<ISpeechRecoResultTimes>;
    fn putref_AudioFormat(&self, format: ::core::option::Option<&ISpeechAudioFormat>) -> ::windows_core::Result<()>;
    fn AudioFormat(&self) -> ::windows_core::Result<ISpeechAudioFormat>;
    fn PhraseInfo(&self) -> ::windows_core::Result<ISpeechPhraseInfo>;
    fn Alternates(&self, requestcount: i32, startelement: i32, elements: i32) -> ::windows_core::Result<ISpeechPhraseAlternates>;
    fn Audio(&self, startelement: i32, elements: i32) -> ::windows_core::Result<ISpeechMemoryStream>;
    fn SpeakAudio(&self, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags) -> ::windows_core::Result<i32>;
    fn SaveToMemory(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn DiscardResultInfo(&self, valuetypes: SpeechDiscardType) -> ::windows_core::Result<()>;
    fn GetXMLResult(&self, options: SPXMLRESULTOPTIONS) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetXMLErrorInfo(&self, linenumber: *mut i32, scriptline: *mut ::windows_core::BSTR, source: *mut ::windows_core::BSTR, description: *mut ::windows_core::BSTR, resultcode: *mut ::windows_core::HRESULT, iserror: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetTextFeedback(&self, feedback: &::windows_core::BSTR, wassuccessful: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechRecoResultDispatch {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechRecoResultDispatch_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>() -> ISpeechRecoResultDispatch_Vtbl {
        unsafe extern "system" fn RecoContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recocontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RecoContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(recocontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Times<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, times: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Times() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(times, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_AudioFormat(::windows_core::from_raw_borrowed(&format)).into()
        }
        unsafe extern "system" fn AudioFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AudioFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(format, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhraseInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phraseinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PhraseInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phraseinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Alternates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestcount: i32, startelement: i32, elements: i32, alternates: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Alternates(::core::mem::transmute_copy(&requestcount), ::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(alternates, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Audio<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Audio(::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakAudio<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SpeakAudio(::core::mem::transmute_copy(&startelement), ::core::mem::transmute_copy(&elements), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(streamnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveToMemory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultblock: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SaveToMemory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultblock, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscardResultInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, valuetypes: SpeechDiscardType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DiscardResultInfo(::core::mem::transmute_copy(&valuetypes)).into()
        }
        unsafe extern "system" fn GetXMLResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: SPXMLRESULTOPTIONS, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetXMLResult(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXMLErrorInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linenumber: *mut i32, scriptline: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, source: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, description: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, resultcode: *mut ::windows_core::HRESULT, iserror: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetXMLErrorInfo(::core::mem::transmute_copy(&linenumber), ::core::mem::transmute_copy(&scriptline), ::core::mem::transmute_copy(&source), ::core::mem::transmute_copy(&description), ::core::mem::transmute_copy(&resultcode), ::core::mem::transmute_copy(&iserror)).into()
        }
        unsafe extern "system" fn SetTextFeedback<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feedback: ::std::mem::MaybeUninit<::windows_core::BSTR>, wassuccessful: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTextFeedback(::core::mem::transmute(&feedback), ::core::mem::transmute_copy(&wassuccessful)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            RecoContext: RecoContext::<Identity, Impl, OFFSET>,
            Times: Times::<Identity, Impl, OFFSET>,
            putref_AudioFormat: putref_AudioFormat::<Identity, Impl, OFFSET>,
            AudioFormat: AudioFormat::<Identity, Impl, OFFSET>,
            PhraseInfo: PhraseInfo::<Identity, Impl, OFFSET>,
            Alternates: Alternates::<Identity, Impl, OFFSET>,
            Audio: Audio::<Identity, Impl, OFFSET>,
            SpeakAudio: SpeakAudio::<Identity, Impl, OFFSET>,
            SaveToMemory: SaveToMemory::<Identity, Impl, OFFSET>,
            DiscardResultInfo: DiscardResultInfo::<Identity, Impl, OFFSET>,
            GetXMLResult: GetXMLResult::<Identity, Impl, OFFSET>,
            GetXMLErrorInfo: GetXMLErrorInfo::<Identity, Impl, OFFSET>,
            SetTextFeedback: SetTextFeedback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechRecoResultDispatch as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecoResultTimes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StreamTime(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Length(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn TickCount(&self) -> ::windows_core::Result<i32>;
    fn OffsetFromStart(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechRecoResultTimes {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechRecoResultTimes_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResultTimes_Impl, const OFFSET: isize>() -> ISpeechRecoResultTimes_Vtbl {
        unsafe extern "system" fn StreamTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResultTimes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StreamTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(time, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResultTimes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Length() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResultTimes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tickcount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TickCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tickcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OffsetFromStart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecoResultTimes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offsetfromstart: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OffsetFromStart() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(offsetfromstart, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StreamTime: StreamTime::<Identity, Impl, OFFSET>,
            Length: Length::<Identity, Impl, OFFSET>,
            TickCount: TickCount::<Identity, Impl, OFFSET>,
            OffsetFromStart: OffsetFromStart::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechRecoResultTimes as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecognizer_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn putref_Recognizer(&self, recognizer: ::core::option::Option<&ISpeechObjectToken>) -> ::windows_core::Result<()>;
    fn Recognizer(&self) -> ::windows_core::Result<ISpeechObjectToken>;
    fn SetAllowAudioInputFormatChangesOnNextSet(&self, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AllowAudioInputFormatChangesOnNextSet(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn putref_AudioInput(&self, audioinput: ::core::option::Option<&ISpeechObjectToken>) -> ::windows_core::Result<()>;
    fn AudioInput(&self) -> ::windows_core::Result<ISpeechObjectToken>;
    fn putref_AudioInputStream(&self, audioinputstream: ::core::option::Option<&ISpeechBaseStream>) -> ::windows_core::Result<()>;
    fn AudioInputStream(&self) -> ::windows_core::Result<ISpeechBaseStream>;
    fn IsShared(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetState(&self, state: SpeechRecognizerState) -> ::windows_core::Result<()>;
    fn State(&self) -> ::windows_core::Result<SpeechRecognizerState>;
    fn Status(&self) -> ::windows_core::Result<ISpeechRecognizerStatus>;
    fn putref_Profile(&self, profile: ::core::option::Option<&ISpeechObjectToken>) -> ::windows_core::Result<()>;
    fn Profile(&self) -> ::windows_core::Result<ISpeechObjectToken>;
    fn EmulateRecognition(&self, textelements: &super::super::System::Variant::VARIANT, elementdisplayattributes: *const super::super::System::Variant::VARIANT, languageid: i32) -> ::windows_core::Result<()>;
    fn CreateRecoContext(&self) -> ::windows_core::Result<ISpeechRecoContext>;
    fn GetFormat(&self, r#type: SpeechFormatType) -> ::windows_core::Result<ISpeechAudioFormat>;
    fn SetPropertyNumber(&self, name: &::windows_core::BSTR, value: i32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetPropertyNumber(&self, name: &::windows_core::BSTR, value: *mut i32, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetPropertyString(&self, name: &::windows_core::BSTR, value: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetPropertyString(&self, name: &::windows_core::BSTR, value: *mut ::windows_core::BSTR, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn IsUISupported(&self, typeofui: &::windows_core::BSTR, extradata: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn DisplayUI(&self, hwndparent: i32, title: &::windows_core::BSTR, typeofui: &::windows_core::BSTR, extradata: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetRecognizers(&self, requiredattributes: &::windows_core::BSTR, optionalattributes: &::windows_core::BSTR) -> ::windows_core::Result<ISpeechObjectTokens>;
    fn GetAudioInputs(&self, requiredattributes: &::windows_core::BSTR, optionalattributes: &::windows_core::BSTR) -> ::windows_core::Result<ISpeechObjectTokens>;
    fn GetProfiles(&self, requiredattributes: &::windows_core::BSTR, optionalattributes: &::windows_core::BSTR) -> ::windows_core::Result<ISpeechObjectTokens>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechRecognizer {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechRecognizer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>() -> ISpeechRecognizer_Vtbl {
        unsafe extern "system" fn putref_Recognizer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_Recognizer(::windows_core::from_raw_borrowed(&recognizer)).into()
        }
        unsafe extern "system" fn Recognizer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recognizer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Recognizer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(recognizer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowAudioInputFormatChangesOnNextSet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllowAudioInputFormatChangesOnNextSet(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowAudioInputFormatChangesOnNextSet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AllowAudioInputFormatChangesOnNextSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(allow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioInput<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioinput: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_AudioInput(::windows_core::from_raw_borrowed(&audioinput)).into()
        }
        unsafe extern "system" fn AudioInput<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioinput: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AudioInput() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audioinput, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioInputStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioinputstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_AudioInputStream(::windows_core::from_raw_borrowed(&audioinputstream)).into()
        }
        unsafe extern "system" fn AudioInputStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioinputstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AudioInputStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audioinputstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsShared<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shared: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsShared() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(shared, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: SpeechRecognizerState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetState(::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn State<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut SpeechRecognizerState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Profile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_Profile(::windows_core::from_raw_borrowed(&profile)).into()
        }
        unsafe extern "system" fn Profile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Profile() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(profile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EmulateRecognition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, textelements: super::super::System::Variant::VARIANT, elementdisplayattributes: *const super::super::System::Variant::VARIANT, languageid: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EmulateRecognition(::core::mem::transmute(&textelements), ::core::mem::transmute_copy(&elementdisplayattributes), ::core::mem::transmute_copy(&languageid)).into()
        }
        unsafe extern "system" fn CreateRecoContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRecoContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: SpeechFormatType, format: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFormat(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(format, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: i32, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetPropertyNumber(::core::mem::transmute(&name), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: *mut i32, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyNumber(::core::mem::transmute(&name), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&supported)).into()
        }
        unsafe extern "system" fn SetPropertyString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: ::std::mem::MaybeUninit<::windows_core::BSTR>, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetPropertyString(::core::mem::transmute(&name), ::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyString(::core::mem::transmute(&name), ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&supported)).into()
        }
        unsafe extern "system" fn IsUISupported<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typeofui: ::std::mem::MaybeUninit<::windows_core::BSTR>, extradata: *const super::super::System::Variant::VARIANT, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsUISupported(::core::mem::transmute(&typeofui), ::core::mem::transmute_copy(&extradata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: i32, title: ::std::mem::MaybeUninit<::windows_core::BSTR>, typeofui: ::std::mem::MaybeUninit<::windows_core::BSTR>, extradata: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisplayUI(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&title), ::core::mem::transmute(&typeofui), ::core::mem::transmute_copy(&extradata)).into()
        }
        unsafe extern "system" fn GetRecognizers<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionalattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, objecttokens: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRecognizers(::core::mem::transmute(&requiredattributes), ::core::mem::transmute(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objecttokens, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioInputs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionalattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, objecttokens: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAudioInputs(::core::mem::transmute(&requiredattributes), ::core::mem::transmute(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objecttokens, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfiles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionalattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, objecttokens: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProfiles(::core::mem::transmute(&requiredattributes), ::core::mem::transmute(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objecttokens, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            putref_Recognizer: putref_Recognizer::<Identity, Impl, OFFSET>,
            Recognizer: Recognizer::<Identity, Impl, OFFSET>,
            SetAllowAudioInputFormatChangesOnNextSet: SetAllowAudioInputFormatChangesOnNextSet::<Identity, Impl, OFFSET>,
            AllowAudioInputFormatChangesOnNextSet: AllowAudioInputFormatChangesOnNextSet::<Identity, Impl, OFFSET>,
            putref_AudioInput: putref_AudioInput::<Identity, Impl, OFFSET>,
            AudioInput: AudioInput::<Identity, Impl, OFFSET>,
            putref_AudioInputStream: putref_AudioInputStream::<Identity, Impl, OFFSET>,
            AudioInputStream: AudioInputStream::<Identity, Impl, OFFSET>,
            IsShared: IsShared::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            putref_Profile: putref_Profile::<Identity, Impl, OFFSET>,
            Profile: Profile::<Identity, Impl, OFFSET>,
            EmulateRecognition: EmulateRecognition::<Identity, Impl, OFFSET>,
            CreateRecoContext: CreateRecoContext::<Identity, Impl, OFFSET>,
            GetFormat: GetFormat::<Identity, Impl, OFFSET>,
            SetPropertyNumber: SetPropertyNumber::<Identity, Impl, OFFSET>,
            GetPropertyNumber: GetPropertyNumber::<Identity, Impl, OFFSET>,
            SetPropertyString: SetPropertyString::<Identity, Impl, OFFSET>,
            GetPropertyString: GetPropertyString::<Identity, Impl, OFFSET>,
            IsUISupported: IsUISupported::<Identity, Impl, OFFSET>,
            DisplayUI: DisplayUI::<Identity, Impl, OFFSET>,
            GetRecognizers: GetRecognizers::<Identity, Impl, OFFSET>,
            GetAudioInputs: GetAudioInputs::<Identity, Impl, OFFSET>,
            GetProfiles: GetProfiles::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechRecognizer as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecognizerStatus_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AudioStatus(&self) -> ::windows_core::Result<ISpeechAudioStatus>;
    fn CurrentStreamPosition(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn CurrentStreamNumber(&self) -> ::windows_core::Result<i32>;
    fn NumberOfActiveRules(&self) -> ::windows_core::Result<i32>;
    fn ClsidEngine(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SupportedLanguages(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechRecognizerStatus {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechRecognizerStatus_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizerStatus_Impl, const OFFSET: isize>() -> ISpeechRecognizerStatus_Vtbl {
        unsafe extern "system" fn AudioStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiostatus: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AudioStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audiostatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentStreamPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcurrentstreampos: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentStreamPosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcurrentstreampos, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentStreamNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentStreamNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(streamnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfActiveRules<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numberofactiverules: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NumberOfActiveRules() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(numberofactiverules, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClsidEngine<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidengine: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClsidEngine() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(clsidengine, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedLanguages<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedlanguages: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportedLanguages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supportedlanguages, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AudioStatus: AudioStatus::<Identity, Impl, OFFSET>,
            CurrentStreamPosition: CurrentStreamPosition::<Identity, Impl, OFFSET>,
            CurrentStreamNumber: CurrentStreamNumber::<Identity, Impl, OFFSET>,
            NumberOfActiveRules: NumberOfActiveRules::<Identity, Impl, OFFSET>,
            ClsidEngine: ClsidEngine::<Identity, Impl, OFFSET>,
            SupportedLanguages: SupportedLanguages::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechRecognizerStatus as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechResourceLoader_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn LoadResource(&self, bstrresourceuri: &::windows_core::BSTR, falwaysreload: super::super::Foundation::VARIANT_BOOL, pstream: *mut ::core::option::Option<::windows_core::IUnknown>, pbstrmimetype: *mut ::windows_core::BSTR, pfmodified: *mut super::super::Foundation::VARIANT_BOOL, pbstrredirecturl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetLocalCopy(&self, bstrresourceuri: &::windows_core::BSTR, pbstrlocalpath: *mut ::windows_core::BSTR, pbstrmimetype: *mut ::windows_core::BSTR, pbstrredirecturl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ReleaseLocalCopy(&self, pbstrlocalpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechResourceLoader {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechResourceLoader_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechResourceLoader_Impl, const OFFSET: isize>() -> ISpeechResourceLoader_Vtbl {
        unsafe extern "system" fn LoadResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechResourceLoader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, falwaysreload: super::super::Foundation::VARIANT_BOOL, pstream: *mut *mut ::core::ffi::c_void, pbstrmimetype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pfmodified: *mut super::super::Foundation::VARIANT_BOOL, pbstrredirecturl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadResource(::core::mem::transmute(&bstrresourceuri), ::core::mem::transmute_copy(&falwaysreload), ::core::mem::transmute_copy(&pstream), ::core::mem::transmute_copy(&pbstrmimetype), ::core::mem::transmute_copy(&pfmodified), ::core::mem::transmute_copy(&pbstrredirecturl)).into()
        }
        unsafe extern "system" fn GetLocalCopy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechResourceLoader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrresourceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrlocalpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrmimetype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrredirecturl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLocalCopy(::core::mem::transmute(&bstrresourceuri), ::core::mem::transmute_copy(&pbstrlocalpath), ::core::mem::transmute_copy(&pbstrmimetype), ::core::mem::transmute_copy(&pbstrredirecturl)).into()
        }
        unsafe extern "system" fn ReleaseLocalCopy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechResourceLoader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrlocalpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseLocalCopy(::core::mem::transmute(&pbstrlocalpath)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            LoadResource: LoadResource::<Identity, Impl, OFFSET>,
            GetLocalCopy: GetLocalCopy::<Identity, Impl, OFFSET>,
            ReleaseLocalCopy: ReleaseLocalCopy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechResourceLoader as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechTextSelectionInformation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetActiveOffset(&self, activeoffset: i32) -> ::windows_core::Result<()>;
    fn ActiveOffset(&self) -> ::windows_core::Result<i32>;
    fn SetActiveLength(&self, activelength: i32) -> ::windows_core::Result<()>;
    fn ActiveLength(&self) -> ::windows_core::Result<i32>;
    fn SetSelectionOffset(&self, selectionoffset: i32) -> ::windows_core::Result<()>;
    fn SelectionOffset(&self) -> ::windows_core::Result<i32>;
    fn SetSelectionLength(&self, selectionlength: i32) -> ::windows_core::Result<()>;
    fn SelectionLength(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechTextSelectionInformation {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechTextSelectionInformation_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>() -> ISpeechTextSelectionInformation_Vtbl {
        unsafe extern "system" fn SetActiveOffset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activeoffset: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetActiveOffset(::core::mem::transmute_copy(&activeoffset)).into()
        }
        unsafe extern "system" fn ActiveOffset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activeoffset: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActiveOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(activeoffset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activelength: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetActiveLength(::core::mem::transmute_copy(&activelength)).into()
        }
        unsafe extern "system" fn ActiveLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activelength: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActiveLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(activelength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionOffset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionoffset: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelectionOffset(::core::mem::transmute_copy(&selectionoffset)).into()
        }
        unsafe extern "system" fn SelectionOffset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionoffset: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelectionOffset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(selectionoffset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionlength: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelectionLength(::core::mem::transmute_copy(&selectionlength)).into()
        }
        unsafe extern "system" fn SelectionLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectionlength: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelectionLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(selectionlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetActiveOffset: SetActiveOffset::<Identity, Impl, OFFSET>,
            ActiveOffset: ActiveOffset::<Identity, Impl, OFFSET>,
            SetActiveLength: SetActiveLength::<Identity, Impl, OFFSET>,
            ActiveLength: ActiveLength::<Identity, Impl, OFFSET>,
            SetSelectionOffset: SetSelectionOffset::<Identity, Impl, OFFSET>,
            SelectionOffset: SelectionOffset::<Identity, Impl, OFFSET>,
            SetSelectionLength: SetSelectionLength::<Identity, Impl, OFFSET>,
            SelectionLength: SelectionLength::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechTextSelectionInformation as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechVoice_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Status(&self) -> ::windows_core::Result<ISpeechVoiceStatus>;
    fn Voice(&self) -> ::windows_core::Result<ISpeechObjectToken>;
    fn putref_Voice(&self, voice: ::core::option::Option<&ISpeechObjectToken>) -> ::windows_core::Result<()>;
    fn AudioOutput(&self) -> ::windows_core::Result<ISpeechObjectToken>;
    fn putref_AudioOutput(&self, audiooutput: ::core::option::Option<&ISpeechObjectToken>) -> ::windows_core::Result<()>;
    fn AudioOutputStream(&self) -> ::windows_core::Result<ISpeechBaseStream>;
    fn putref_AudioOutputStream(&self, audiooutputstream: ::core::option::Option<&ISpeechBaseStream>) -> ::windows_core::Result<()>;
    fn Rate(&self) -> ::windows_core::Result<i32>;
    fn SetRate(&self, rate: i32) -> ::windows_core::Result<()>;
    fn Volume(&self) -> ::windows_core::Result<i32>;
    fn SetVolume(&self, volume: i32) -> ::windows_core::Result<()>;
    fn SetAllowAudioOutputFormatChangesOnNextSet(&self, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AllowAudioOutputFormatChangesOnNextSet(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EventInterests(&self) -> ::windows_core::Result<SpeechVoiceEvents>;
    fn SetEventInterests(&self, eventinterestflags: SpeechVoiceEvents) -> ::windows_core::Result<()>;
    fn SetPriority(&self, priority: SpeechVoicePriority) -> ::windows_core::Result<()>;
    fn Priority(&self) -> ::windows_core::Result<SpeechVoicePriority>;
    fn SetAlertBoundary(&self, boundary: SpeechVoiceEvents) -> ::windows_core::Result<()>;
    fn AlertBoundary(&self) -> ::windows_core::Result<SpeechVoiceEvents>;
    fn SetSynchronousSpeakTimeout(&self, mstimeout: i32) -> ::windows_core::Result<()>;
    fn SynchronousSpeakTimeout(&self) -> ::windows_core::Result<i32>;
    fn Speak(&self, text: &::windows_core::BSTR, flags: SpeechVoiceSpeakFlags) -> ::windows_core::Result<i32>;
    fn SpeakStream(&self, stream: ::core::option::Option<&ISpeechBaseStream>, flags: SpeechVoiceSpeakFlags) -> ::windows_core::Result<i32>;
    fn Pause(&self) -> ::windows_core::Result<()>;
    fn Resume(&self) -> ::windows_core::Result<()>;
    fn Skip(&self, r#type: &::windows_core::BSTR, numitems: i32) -> ::windows_core::Result<i32>;
    fn GetVoices(&self, requiredattributes: &::windows_core::BSTR, optionalattributes: &::windows_core::BSTR) -> ::windows_core::Result<ISpeechObjectTokens>;
    fn GetAudioOutputs(&self, requiredattributes: &::windows_core::BSTR, optionalattributes: &::windows_core::BSTR) -> ::windows_core::Result<ISpeechObjectTokens>;
    fn WaitUntilDone(&self, mstimeout: i32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SpeakCompleteEvent(&self) -> ::windows_core::Result<i32>;
    fn IsUISupported(&self, typeofui: &::windows_core::BSTR, extradata: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn DisplayUI(&self, hwndparent: i32, title: &::windows_core::BSTR, typeofui: &::windows_core::BSTR, extradata: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechVoice {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechVoice_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>() -> ISpeechVoice_Vtbl {
        unsafe extern "system" fn Status<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Voice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Voice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(voice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Voice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, voice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_Voice(::windows_core::from_raw_borrowed(&voice)).into()
        }
        unsafe extern "system" fn AudioOutput<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiooutput: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AudioOutput() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audiooutput, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioOutput<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiooutput: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_AudioOutput(::windows_core::from_raw_borrowed(&audiooutput)).into()
        }
        unsafe extern "system" fn AudioOutputStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiooutputstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AudioOutputStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audiooutputstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioOutputStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiooutputstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.putref_AudioOutputStream(::windows_core::from_raw_borrowed(&audiooutputstream)).into()
        }
        unsafe extern "system" fn Rate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rate: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Rate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rate: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRate(::core::mem::transmute_copy(&rate)).into()
        }
        unsafe extern "system" fn Volume<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Volume() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(volume, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVolume(::core::mem::transmute_copy(&volume)).into()
        }
        unsafe extern "system" fn SetAllowAudioOutputFormatChangesOnNextSet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllowAudioOutputFormatChangesOnNextSet(::core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowAudioOutputFormatChangesOnNextSet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AllowAudioOutputFormatChangesOnNextSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(allow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventInterests<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterestflags: *mut SpeechVoiceEvents) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventInterests() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eventinterestflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventInterests<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventinterestflags: SpeechVoiceEvents) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventInterests(::core::mem::transmute_copy(&eventinterestflags)).into()
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: SpeechVoicePriority) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPriority(::core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn Priority<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: *mut SpeechVoicePriority) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Priority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(priority, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlertBoundary<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundary: SpeechVoiceEvents) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAlertBoundary(::core::mem::transmute_copy(&boundary)).into()
        }
        unsafe extern "system" fn AlertBoundary<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundary: *mut SpeechVoiceEvents) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AlertBoundary() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(boundary, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSynchronousSpeakTimeout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mstimeout: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSynchronousSpeakTimeout(::core::mem::transmute_copy(&mstimeout)).into()
        }
        unsafe extern "system" fn SynchronousSpeakTimeout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mstimeout: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SynchronousSpeakTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mstimeout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Speak<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::BSTR>, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Speak(::core::mem::transmute(&text), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(streamnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SpeakStream(::windows_core::from_raw_borrowed(&stream), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(streamnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resume().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ::std::mem::MaybeUninit<::windows_core::BSTR>, numitems: i32, numskipped: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Skip(::core::mem::transmute(&r#type), ::core::mem::transmute_copy(&numitems)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(numskipped, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVoices<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionalattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, objecttokens: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVoices(::core::mem::transmute(&requiredattributes), ::core::mem::transmute(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objecttokens, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioOutputs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requiredattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, optionalattributes: ::std::mem::MaybeUninit<::windows_core::BSTR>, objecttokens: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAudioOutputs(::core::mem::transmute(&requiredattributes), ::core::mem::transmute(&optionalattributes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(objecttokens, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitUntilDone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mstimeout: i32, done: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WaitUntilDone(::core::mem::transmute_copy(&mstimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(done, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakCompleteEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SpeakCompleteEvent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(handle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUISupported<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, typeofui: ::std::mem::MaybeUninit<::windows_core::BSTR>, extradata: *const super::super::System::Variant::VARIANT, supported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsUISupported(::core::mem::transmute(&typeofui), ::core::mem::transmute_copy(&extradata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: i32, title: ::std::mem::MaybeUninit<::windows_core::BSTR>, typeofui: ::std::mem::MaybeUninit<::windows_core::BSTR>, extradata: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisplayUI(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute(&title), ::core::mem::transmute(&typeofui), ::core::mem::transmute_copy(&extradata)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Status: Status::<Identity, Impl, OFFSET>,
            Voice: Voice::<Identity, Impl, OFFSET>,
            putref_Voice: putref_Voice::<Identity, Impl, OFFSET>,
            AudioOutput: AudioOutput::<Identity, Impl, OFFSET>,
            putref_AudioOutput: putref_AudioOutput::<Identity, Impl, OFFSET>,
            AudioOutputStream: AudioOutputStream::<Identity, Impl, OFFSET>,
            putref_AudioOutputStream: putref_AudioOutputStream::<Identity, Impl, OFFSET>,
            Rate: Rate::<Identity, Impl, OFFSET>,
            SetRate: SetRate::<Identity, Impl, OFFSET>,
            Volume: Volume::<Identity, Impl, OFFSET>,
            SetVolume: SetVolume::<Identity, Impl, OFFSET>,
            SetAllowAudioOutputFormatChangesOnNextSet: SetAllowAudioOutputFormatChangesOnNextSet::<Identity, Impl, OFFSET>,
            AllowAudioOutputFormatChangesOnNextSet: AllowAudioOutputFormatChangesOnNextSet::<Identity, Impl, OFFSET>,
            EventInterests: EventInterests::<Identity, Impl, OFFSET>,
            SetEventInterests: SetEventInterests::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetAlertBoundary: SetAlertBoundary::<Identity, Impl, OFFSET>,
            AlertBoundary: AlertBoundary::<Identity, Impl, OFFSET>,
            SetSynchronousSpeakTimeout: SetSynchronousSpeakTimeout::<Identity, Impl, OFFSET>,
            SynchronousSpeakTimeout: SynchronousSpeakTimeout::<Identity, Impl, OFFSET>,
            Speak: Speak::<Identity, Impl, OFFSET>,
            SpeakStream: SpeakStream::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            GetVoices: GetVoices::<Identity, Impl, OFFSET>,
            GetAudioOutputs: GetAudioOutputs::<Identity, Impl, OFFSET>,
            WaitUntilDone: WaitUntilDone::<Identity, Impl, OFFSET>,
            SpeakCompleteEvent: SpeakCompleteEvent::<Identity, Impl, OFFSET>,
            IsUISupported: IsUISupported::<Identity, Impl, OFFSET>,
            DisplayUI: DisplayUI::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechVoice as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechVoiceStatus_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CurrentStreamNumber(&self) -> ::windows_core::Result<i32>;
    fn LastStreamNumberQueued(&self) -> ::windows_core::Result<i32>;
    fn LastHResult(&self) -> ::windows_core::Result<i32>;
    fn RunningState(&self) -> ::windows_core::Result<SpeechRunState>;
    fn InputWordPosition(&self) -> ::windows_core::Result<i32>;
    fn InputWordLength(&self) -> ::windows_core::Result<i32>;
    fn InputSentencePosition(&self) -> ::windows_core::Result<i32>;
    fn InputSentenceLength(&self) -> ::windows_core::Result<i32>;
    fn LastBookmark(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LastBookmarkId(&self) -> ::windows_core::Result<i32>;
    fn PhonemeId(&self) -> ::windows_core::Result<i16>;
    fn VisemeId(&self) -> ::windows_core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechVoiceStatus {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechVoiceStatus_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>() -> ISpeechVoiceStatus_Vtbl {
        unsafe extern "system" fn CurrentStreamNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentStreamNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(streamnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastStreamNumberQueued<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamnumber: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastStreamNumberQueued() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(streamnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastHResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastHResult() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunningState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: *mut SpeechRunState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RunningState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputWordPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InputWordPosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(position, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputWordLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InputWordLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputSentencePosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InputSentencePosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(position, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputSentenceLength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InputSentenceLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBookmark<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bookmark: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastBookmark() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bookmark, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBookmarkId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bookmarkid: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastBookmarkId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bookmarkid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhonemeId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phoneid: *mut i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PhonemeId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phoneid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VisemeId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visemeid: *mut i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VisemeId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(visemeid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentStreamNumber: CurrentStreamNumber::<Identity, Impl, OFFSET>,
            LastStreamNumberQueued: LastStreamNumberQueued::<Identity, Impl, OFFSET>,
            LastHResult: LastHResult::<Identity, Impl, OFFSET>,
            RunningState: RunningState::<Identity, Impl, OFFSET>,
            InputWordPosition: InputWordPosition::<Identity, Impl, OFFSET>,
            InputWordLength: InputWordLength::<Identity, Impl, OFFSET>,
            InputSentencePosition: InputSentencePosition::<Identity, Impl, OFFSET>,
            InputSentenceLength: InputSentenceLength::<Identity, Impl, OFFSET>,
            LastBookmark: LastBookmark::<Identity, Impl, OFFSET>,
            LastBookmarkId: LastBookmarkId::<Identity, Impl, OFFSET>,
            PhonemeId: PhonemeId::<Identity, Impl, OFFSET>,
            VisemeId: VisemeId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechVoiceStatus as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechWaveFormatEx_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn FormatTag(&self) -> ::windows_core::Result<i16>;
    fn SetFormatTag(&self, formattag: i16) -> ::windows_core::Result<()>;
    fn Channels(&self) -> ::windows_core::Result<i16>;
    fn SetChannels(&self, channels: i16) -> ::windows_core::Result<()>;
    fn SamplesPerSec(&self) -> ::windows_core::Result<i32>;
    fn SetSamplesPerSec(&self, samplespersec: i32) -> ::windows_core::Result<()>;
    fn AvgBytesPerSec(&self) -> ::windows_core::Result<i32>;
    fn SetAvgBytesPerSec(&self, avgbytespersec: i32) -> ::windows_core::Result<()>;
    fn BlockAlign(&self) -> ::windows_core::Result<i16>;
    fn SetBlockAlign(&self, blockalign: i16) -> ::windows_core::Result<()>;
    fn BitsPerSample(&self) -> ::windows_core::Result<i16>;
    fn SetBitsPerSample(&self, bitspersample: i16) -> ::windows_core::Result<()>;
    fn ExtraData(&self) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetExtraData(&self, extradata: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechWaveFormatEx {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechWaveFormatEx_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>() -> ISpeechWaveFormatEx_Vtbl {
        unsafe extern "system" fn FormatTag<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formattag: *mut i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FormatTag() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(formattag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatTag<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, formattag: i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFormatTag(::core::mem::transmute_copy(&formattag)).into()
        }
        unsafe extern "system" fn Channels<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channels: *mut i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Channels() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(channels, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannels<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channels: i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetChannels(::core::mem::transmute_copy(&channels)).into()
        }
        unsafe extern "system" fn SamplesPerSec<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, samplespersec: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SamplesPerSec() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(samplespersec, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSamplesPerSec<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, samplespersec: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSamplesPerSec(::core::mem::transmute_copy(&samplespersec)).into()
        }
        unsafe extern "system" fn AvgBytesPerSec<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, avgbytespersec: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AvgBytesPerSec() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(avgbytespersec, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAvgBytesPerSec<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, avgbytespersec: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAvgBytesPerSec(::core::mem::transmute_copy(&avgbytespersec)).into()
        }
        unsafe extern "system" fn BlockAlign<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockalign: *mut i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BlockAlign() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(blockalign, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlockAlign<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blockalign: i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBlockAlign(::core::mem::transmute_copy(&blockalign)).into()
        }
        unsafe extern "system" fn BitsPerSample<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitspersample: *mut i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BitsPerSample() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bitspersample, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitsPerSample<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitspersample: i16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBitsPerSample(::core::mem::transmute_copy(&bitspersample)).into()
        }
        unsafe extern "system" fn ExtraData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extradata: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExtraData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(extradata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtraData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extradata: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExtraData(::core::mem::transmute(&extradata)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            FormatTag: FormatTag::<Identity, Impl, OFFSET>,
            SetFormatTag: SetFormatTag::<Identity, Impl, OFFSET>,
            Channels: Channels::<Identity, Impl, OFFSET>,
            SetChannels: SetChannels::<Identity, Impl, OFFSET>,
            SamplesPerSec: SamplesPerSec::<Identity, Impl, OFFSET>,
            SetSamplesPerSec: SetSamplesPerSec::<Identity, Impl, OFFSET>,
            AvgBytesPerSec: AvgBytesPerSec::<Identity, Impl, OFFSET>,
            SetAvgBytesPerSec: SetAvgBytesPerSec::<Identity, Impl, OFFSET>,
            BlockAlign: BlockAlign::<Identity, Impl, OFFSET>,
            SetBlockAlign: SetBlockAlign::<Identity, Impl, OFFSET>,
            BitsPerSample: BitsPerSample::<Identity, Impl, OFFSET>,
            SetBitsPerSample: SetBitsPerSample::<Identity, Impl, OFFSET>,
            ExtraData: ExtraData::<Identity, Impl, OFFSET>,
            SetExtraData: SetExtraData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechWaveFormatEx as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechXMLRecoResult_Impl: Sized + ISpeechRecoResult_Impl {
    fn GetXMLResult(&self, options: SPXMLRESULTOPTIONS) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetXMLErrorInfo(&self, linenumber: *mut i32, scriptline: *mut ::windows_core::BSTR, source: *mut ::windows_core::BSTR, description: *mut ::windows_core::BSTR, resultcode: *mut i32, iserror: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISpeechXMLRecoResult {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechXMLRecoResult_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechXMLRecoResult_Impl, const OFFSET: isize>() -> ISpeechXMLRecoResult_Vtbl {
        unsafe extern "system" fn GetXMLResult<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechXMLRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: SPXMLRESULTOPTIONS, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetXMLResult(::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXMLErrorInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISpeechXMLRecoResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linenumber: *mut i32, scriptline: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, source: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, description: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, resultcode: *mut i32, iserror: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetXMLErrorInfo(::core::mem::transmute_copy(&linenumber), ::core::mem::transmute_copy(&scriptline), ::core::mem::transmute_copy(&source), ::core::mem::transmute_copy(&description), ::core::mem::transmute_copy(&resultcode), ::core::mem::transmute_copy(&iserror)).into()
        }
        Self {
            base__: ISpeechRecoResult_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetXMLResult: GetXMLResult::<Identity, Impl, OFFSET>,
            GetXMLErrorInfo: GetXMLErrorInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISpeechXMLRecoResult as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<ISpeechRecoResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _ISpeechRecoContextEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for _ISpeechRecoContextEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl _ISpeechRecoContextEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _ISpeechRecoContextEvents_Impl, const OFFSET: isize>() -> _ISpeechRecoContextEvents_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<_ISpeechRecoContextEvents as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Speech\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _ISpeechVoiceEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for _ISpeechVoiceEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl _ISpeechVoiceEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: _ISpeechVoiceEvents_Impl, const OFFSET: isize>() -> _ISpeechVoiceEvents_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<_ISpeechVoiceEvents as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
