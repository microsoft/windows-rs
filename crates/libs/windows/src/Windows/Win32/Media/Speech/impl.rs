pub trait IEnumSpObjectTokens_Impl: Sized + windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, pelt: *mut Option<ISpObjectToken>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSpObjectTokens>;
    fn Item(&self, index: u32) -> windows_core::Result<ISpObjectToken>;
    fn GetCount(&self, pcount: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumSpObjectTokens {}
impl IEnumSpObjectTokens_Vtbl {
    pub const fn new<Identity: IEnumSpObjectTokens_Impl, const OFFSET: isize>() -> IEnumSpObjectTokens_Vtbl {
        unsafe extern "system" fn Next<Identity: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, pelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSpObjectTokens_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&pelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSpObjectTokens_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSpObjectTokens_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumSpObjectTokens_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pptoken: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumSpObjectTokens_Impl::Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pptoken.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumSpObjectTokens_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumSpObjectTokens_Impl::GetCount(this, core::mem::transmute_copy(&pcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSpObjectTokens as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
pub trait ISpAudio_Impl: Sized + ISpStreamFormat_Impl {
    fn SetState(&self, newstate: SPAUDIOSTATE, ullreserved: u64) -> windows_core::Result<()>;
    fn SetFormat(&self, rguidfmtid: *const windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> windows_core::Result<()>;
    fn GetStatus(&self, pstatus: *mut SPAUDIOSTATUS) -> windows_core::Result<()>;
    fn SetBufferInfo(&self, pbuffinfo: *const SPAUDIOBUFFERINFO) -> windows_core::Result<()>;
    fn GetBufferInfo(&self, pbuffinfo: *mut SPAUDIOBUFFERINFO) -> windows_core::Result<()>;
    fn GetDefaultFormat(&self, pformatid: *mut windows_core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> windows_core::Result<()>;
    fn EventHandle(&self) -> super::super::Foundation::HANDLE;
    fn GetVolumeLevel(&self, plevel: *mut u32) -> windows_core::Result<()>;
    fn SetVolumeLevel(&self, level: u32) -> windows_core::Result<()>;
    fn GetBufferNotifySize(&self, pcbsize: *mut u32) -> windows_core::Result<()>;
    fn SetBufferNotifySize(&self, cbsize: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ISpAudio {}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ISpAudio_Vtbl {
    pub const fn new<Identity: ISpAudio_Impl, const OFFSET: isize>() -> ISpAudio_Vtbl {
        unsafe extern "system" fn SetState<Identity: ISpAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstate: SPAUDIOSTATE, ullreserved: u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpAudio_Impl::SetState(this, core::mem::transmute_copy(&newstate), core::mem::transmute_copy(&ullreserved)).into()
        }
        unsafe extern "system" fn SetFormat<Identity: ISpAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidfmtid: *const windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpAudio_Impl::SetFormat(this, core::mem::transmute_copy(&rguidfmtid), core::mem::transmute_copy(&pwaveformatex)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ISpAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut SPAUDIOSTATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpAudio_Impl::GetStatus(this, core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn SetBufferInfo<Identity: ISpAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffinfo: *const SPAUDIOBUFFERINFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpAudio_Impl::SetBufferInfo(this, core::mem::transmute_copy(&pbuffinfo)).into()
        }
        unsafe extern "system" fn GetBufferInfo<Identity: ISpAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffinfo: *mut SPAUDIOBUFFERINFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpAudio_Impl::GetBufferInfo(this, core::mem::transmute_copy(&pbuffinfo)).into()
        }
        unsafe extern "system" fn GetDefaultFormat<Identity: ISpAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatid: *mut windows_core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpAudio_Impl::GetDefaultFormat(this, core::mem::transmute_copy(&pformatid), core::mem::transmute_copy(&ppcomemwaveformatex)).into()
        }
        unsafe extern "system" fn EventHandle<Identity: ISpAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpAudio_Impl::EventHandle(this)
        }
        unsafe extern "system" fn GetVolumeLevel<Identity: ISpAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plevel: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpAudio_Impl::GetVolumeLevel(this, core::mem::transmute_copy(&plevel)).into()
        }
        unsafe extern "system" fn SetVolumeLevel<Identity: ISpAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpAudio_Impl::SetVolumeLevel(this, core::mem::transmute_copy(&level)).into()
        }
        unsafe extern "system" fn GetBufferNotifySize<Identity: ISpAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpAudio_Impl::GetBufferNotifySize(this, core::mem::transmute_copy(&pcbsize)).into()
        }
        unsafe extern "system" fn SetBufferNotifySize<Identity: ISpAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsize: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpAudio_Impl::SetBufferNotifySize(this, core::mem::transmute_copy(&cbsize)).into()
        }
        Self {
            base__: ISpStreamFormat_Vtbl::new::<Identity, OFFSET>(),
            SetState: SetState::<Identity, OFFSET>,
            SetFormat: SetFormat::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            SetBufferInfo: SetBufferInfo::<Identity, OFFSET>,
            GetBufferInfo: GetBufferInfo::<Identity, OFFSET>,
            GetDefaultFormat: GetDefaultFormat::<Identity, OFFSET>,
            EventHandle: EventHandle::<Identity, OFFSET>,
            GetVolumeLevel: GetVolumeLevel::<Identity, OFFSET>,
            SetVolumeLevel: SetVolumeLevel::<Identity, OFFSET>,
            GetBufferNotifySize: GetBufferNotifySize::<Identity, OFFSET>,
            SetBufferNotifySize: SetBufferNotifySize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpAudio as windows_core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IStream as windows_core::Interface>::IID || iid == &<ISpStreamFormat as windows_core::Interface>::IID
    }
}
pub trait ISpCFGInterpreter_Impl: Sized + windows_core::IUnknownImpl {
    fn InitGrammar(&self, pszgrammarname: &windows_core::PCWSTR, pvgrammardata: *const *const core::ffi::c_void) -> windows_core::Result<()>;
    fn Interpret(&self, pphrase: Option<&ISpPhraseBuilder>, ulfirstelement: u32, ulcountofelements: u32, psite: Option<&ISpCFGInterpreterSite>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpCFGInterpreter {}
impl ISpCFGInterpreter_Vtbl {
    pub const fn new<Identity: ISpCFGInterpreter_Impl, const OFFSET: isize>() -> ISpCFGInterpreter_Vtbl {
        unsafe extern "system" fn InitGrammar<Identity: ISpCFGInterpreter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszgrammarname: windows_core::PCWSTR, pvgrammardata: *const *const core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpCFGInterpreter_Impl::InitGrammar(this, core::mem::transmute(&pszgrammarname), core::mem::transmute_copy(&pvgrammardata)).into()
        }
        unsafe extern "system" fn Interpret<Identity: ISpCFGInterpreter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphrase: *mut core::ffi::c_void, ulfirstelement: u32, ulcountofelements: u32, psite: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpCFGInterpreter_Impl::Interpret(this, windows_core::from_raw_borrowed(&pphrase), core::mem::transmute_copy(&ulfirstelement), core::mem::transmute_copy(&ulcountofelements), windows_core::from_raw_borrowed(&psite)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitGrammar: InitGrammar::<Identity, OFFSET>,
            Interpret: Interpret::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpCFGInterpreter as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpCFGInterpreterSite_Impl: Sized + windows_core::IUnknownImpl {
    fn AddTextReplacement(&self, preplace: *const SPPHRASEREPLACEMENT) -> windows_core::Result<()>;
    fn AddProperty(&self, pproperty: *const SPPHRASEPROPERTY) -> windows_core::Result<()>;
    fn GetResourceValue(&self, pszresourcename: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpCFGInterpreterSite {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpCFGInterpreterSite_Vtbl {
    pub const fn new<Identity: ISpCFGInterpreterSite_Impl, const OFFSET: isize>() -> ISpCFGInterpreterSite_Vtbl {
        unsafe extern "system" fn AddTextReplacement<Identity: ISpCFGInterpreterSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preplace: *const SPPHRASEREPLACEMENT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpCFGInterpreterSite_Impl::AddTextReplacement(this, core::mem::transmute_copy(&preplace)).into()
        }
        unsafe extern "system" fn AddProperty<Identity: ISpCFGInterpreterSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproperty: *const SPPHRASEPROPERTY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpCFGInterpreterSite_Impl::AddProperty(this, core::mem::transmute_copy(&pproperty)).into()
        }
        unsafe extern "system" fn GetResourceValue<Identity: ISpCFGInterpreterSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszresourcename: windows_core::PCWSTR, ppcomemresource: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpCFGInterpreterSite_Impl::GetResourceValue(this, core::mem::transmute(&pszresourcename)) {
                Ok(ok__) => {
                    ppcomemresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddTextReplacement: AddTextReplacement::<Identity, OFFSET>,
            AddProperty: AddProperty::<Identity, OFFSET>,
            GetResourceValue: GetResourceValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpCFGInterpreterSite as windows_core::Interface>::IID
    }
}
pub trait ISpContainerLexicon_Impl: Sized + ISpLexicon_Impl {
    fn AddLexicon(&self, paddlexicon: Option<&ISpLexicon>, dwflags: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpContainerLexicon {}
impl ISpContainerLexicon_Vtbl {
    pub const fn new<Identity: ISpContainerLexicon_Impl, const OFFSET: isize>() -> ISpContainerLexicon_Vtbl {
        unsafe extern "system" fn AddLexicon<Identity: ISpContainerLexicon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddlexicon: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpContainerLexicon_Impl::AddLexicon(this, windows_core::from_raw_borrowed(&paddlexicon), core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base__: ISpLexicon_Vtbl::new::<Identity, OFFSET>(), AddLexicon: AddLexicon::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpContainerLexicon as windows_core::Interface>::IID || iid == &<ISpLexicon as windows_core::Interface>::IID
    }
}
pub trait ISpDataKey_Impl: Sized + windows_core::IUnknownImpl {
    fn SetData(&self, pszvaluename: &windows_core::PCWSTR, cbdata: u32, pdata: *const u8) -> windows_core::Result<()>;
    fn GetData(&self, pszvaluename: &windows_core::PCWSTR, pcbdata: *mut u32, pdata: *mut u8) -> windows_core::Result<()>;
    fn SetStringValue(&self, pszvaluename: &windows_core::PCWSTR, pszvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetStringValue(&self, pszvaluename: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
    fn SetDWORD(&self, pszvaluename: &windows_core::PCWSTR, dwvalue: u32) -> windows_core::Result<()>;
    fn GetDWORD(&self, pszvaluename: &windows_core::PCWSTR, pdwvalue: *mut u32) -> windows_core::Result<()>;
    fn OpenKey(&self, pszsubkeyname: &windows_core::PCWSTR) -> windows_core::Result<ISpDataKey>;
    fn CreateKey(&self, pszsubkey: &windows_core::PCWSTR) -> windows_core::Result<ISpDataKey>;
    fn DeleteKey(&self, pszsubkey: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DeleteValue(&self, pszvaluename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnumKeys(&self, index: u32) -> windows_core::Result<windows_core::PWSTR>;
    fn EnumValues(&self, index: u32) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for ISpDataKey {}
impl ISpDataKey_Vtbl {
    pub const fn new<Identity: ISpDataKey_Impl, const OFFSET: isize>() -> ISpDataKey_Vtbl {
        unsafe extern "system" fn SetData<Identity: ISpDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszvaluename: windows_core::PCWSTR, cbdata: u32, pdata: *const u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpDataKey_Impl::SetData(this, core::mem::transmute(&pszvaluename), core::mem::transmute_copy(&cbdata), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetData<Identity: ISpDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszvaluename: windows_core::PCWSTR, pcbdata: *mut u32, pdata: *mut u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpDataKey_Impl::GetData(this, core::mem::transmute(&pszvaluename), core::mem::transmute_copy(&pcbdata), core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetStringValue<Identity: ISpDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszvaluename: windows_core::PCWSTR, pszvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpDataKey_Impl::SetStringValue(this, core::mem::transmute(&pszvaluename), core::mem::transmute(&pszvalue)).into()
        }
        unsafe extern "system" fn GetStringValue<Identity: ISpDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszvaluename: windows_core::PCWSTR, ppszvalue: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpDataKey_Impl::GetStringValue(this, core::mem::transmute(&pszvaluename)) {
                Ok(ok__) => {
                    ppszvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDWORD<Identity: ISpDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszvaluename: windows_core::PCWSTR, dwvalue: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpDataKey_Impl::SetDWORD(this, core::mem::transmute(&pszvaluename), core::mem::transmute_copy(&dwvalue)).into()
        }
        unsafe extern "system" fn GetDWORD<Identity: ISpDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszvaluename: windows_core::PCWSTR, pdwvalue: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpDataKey_Impl::GetDWORD(this, core::mem::transmute(&pszvaluename), core::mem::transmute_copy(&pdwvalue)).into()
        }
        unsafe extern "system" fn OpenKey<Identity: ISpDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubkeyname: windows_core::PCWSTR, ppsubkey: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpDataKey_Impl::OpenKey(this, core::mem::transmute(&pszsubkeyname)) {
                Ok(ok__) => {
                    ppsubkey.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateKey<Identity: ISpDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubkey: windows_core::PCWSTR, ppsubkey: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpDataKey_Impl::CreateKey(this, core::mem::transmute(&pszsubkey)) {
                Ok(ok__) => {
                    ppsubkey.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteKey<Identity: ISpDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubkey: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpDataKey_Impl::DeleteKey(this, core::mem::transmute(&pszsubkey)).into()
        }
        unsafe extern "system" fn DeleteValue<Identity: ISpDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszvaluename: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpDataKey_Impl::DeleteValue(this, core::mem::transmute(&pszvaluename)).into()
        }
        unsafe extern "system" fn EnumKeys<Identity: ISpDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ppszsubkeyname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpDataKey_Impl::EnumKeys(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    ppszsubkeyname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumValues<Identity: ISpDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ppszvaluename: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpDataKey_Impl::EnumValues(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    ppszvaluename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetData: SetData::<Identity, OFFSET>,
            GetData: GetData::<Identity, OFFSET>,
            SetStringValue: SetStringValue::<Identity, OFFSET>,
            GetStringValue: GetStringValue::<Identity, OFFSET>,
            SetDWORD: SetDWORD::<Identity, OFFSET>,
            GetDWORD: GetDWORD::<Identity, OFFSET>,
            OpenKey: OpenKey::<Identity, OFFSET>,
            CreateKey: CreateKey::<Identity, OFFSET>,
            DeleteKey: DeleteKey::<Identity, OFFSET>,
            DeleteValue: DeleteValue::<Identity, OFFSET>,
            EnumKeys: EnumKeys::<Identity, OFFSET>,
            EnumValues: EnumValues::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpDataKey as windows_core::Interface>::IID
    }
}
pub trait ISpDisplayAlternates_Impl: Sized + windows_core::IUnknownImpl {
    fn GetDisplayAlternates(&self, pphrase: *const SPDISPLAYPHRASE, crequestcount: u32, ppcomemphrases: *mut *mut SPDISPLAYPHRASE, pcphrasesreturned: *mut u32) -> windows_core::Result<()>;
    fn SetFullStopTrailSpace(&self, ultrailspace: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpDisplayAlternates {}
impl ISpDisplayAlternates_Vtbl {
    pub const fn new<Identity: ISpDisplayAlternates_Impl, const OFFSET: isize>() -> ISpDisplayAlternates_Vtbl {
        unsafe extern "system" fn GetDisplayAlternates<Identity: ISpDisplayAlternates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphrase: *const SPDISPLAYPHRASE, crequestcount: u32, ppcomemphrases: *mut *mut SPDISPLAYPHRASE, pcphrasesreturned: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpDisplayAlternates_Impl::GetDisplayAlternates(this, core::mem::transmute_copy(&pphrase), core::mem::transmute_copy(&crequestcount), core::mem::transmute_copy(&ppcomemphrases), core::mem::transmute_copy(&pcphrasesreturned)).into()
        }
        unsafe extern "system" fn SetFullStopTrailSpace<Identity: ISpDisplayAlternates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ultrailspace: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpDisplayAlternates_Impl::SetFullStopTrailSpace(this, core::mem::transmute_copy(&ultrailspace)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDisplayAlternates: GetDisplayAlternates::<Identity, OFFSET>,
            SetFullStopTrailSpace: SetFullStopTrailSpace::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpDisplayAlternates as windows_core::Interface>::IID
    }
}
pub trait ISpEnginePronunciation_Impl: Sized + windows_core::IUnknownImpl {
    fn Normalize(&self, pszword: &windows_core::PCWSTR, pszleftcontext: &windows_core::PCWSTR, pszrightcontext: &windows_core::PCWSTR, langid: u16, pnormalizationlist: *mut SPNORMALIZATIONLIST) -> windows_core::Result<()>;
    fn GetPronunciations(&self, pszword: &windows_core::PCWSTR, pszleftcontext: &windows_core::PCWSTR, pszrightcontext: &windows_core::PCWSTR, langid: u16, penginepronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpEnginePronunciation {}
impl ISpEnginePronunciation_Vtbl {
    pub const fn new<Identity: ISpEnginePronunciation_Impl, const OFFSET: isize>() -> ISpEnginePronunciation_Vtbl {
        unsafe extern "system" fn Normalize<Identity: ISpEnginePronunciation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszword: windows_core::PCWSTR, pszleftcontext: windows_core::PCWSTR, pszrightcontext: windows_core::PCWSTR, langid: u16, pnormalizationlist: *mut SPNORMALIZATIONLIST) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpEnginePronunciation_Impl::Normalize(this, core::mem::transmute(&pszword), core::mem::transmute(&pszleftcontext), core::mem::transmute(&pszrightcontext), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&pnormalizationlist)).into()
        }
        unsafe extern "system" fn GetPronunciations<Identity: ISpEnginePronunciation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszword: windows_core::PCWSTR, pszleftcontext: windows_core::PCWSTR, pszrightcontext: windows_core::PCWSTR, langid: u16, penginepronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpEnginePronunciation_Impl::GetPronunciations(this, core::mem::transmute(&pszword), core::mem::transmute(&pszleftcontext), core::mem::transmute(&pszrightcontext), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&penginepronunciationlist)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Normalize: Normalize::<Identity, OFFSET>,
            GetPronunciations: GetPronunciations::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpEnginePronunciation as windows_core::Interface>::IID
    }
}
pub trait ISpErrorLog_Impl: Sized + windows_core::IUnknownImpl {
    fn AddError(&self, llinenumber: i32, hr: windows_core::HRESULT, pszdescription: &windows_core::PCWSTR, pszhelpfile: &windows_core::PCWSTR, dwhelpcontext: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpErrorLog {}
impl ISpErrorLog_Vtbl {
    pub const fn new<Identity: ISpErrorLog_Impl, const OFFSET: isize>() -> ISpErrorLog_Vtbl {
        unsafe extern "system" fn AddError<Identity: ISpErrorLog_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, llinenumber: i32, hr: windows_core::HRESULT, pszdescription: windows_core::PCWSTR, pszhelpfile: windows_core::PCWSTR, dwhelpcontext: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpErrorLog_Impl::AddError(this, core::mem::transmute_copy(&llinenumber), core::mem::transmute_copy(&hr), core::mem::transmute(&pszdescription), core::mem::transmute(&pszhelpfile), core::mem::transmute_copy(&dwhelpcontext)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AddError: AddError::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpErrorLog as windows_core::Interface>::IID
    }
}
pub trait ISpEventSink_Impl: Sized + windows_core::IUnknownImpl {
    fn AddEvents(&self, peventarray: *const SPEVENT, ulcount: u32) -> windows_core::Result<()>;
    fn GetEventInterest(&self, pulleventinterest: *mut u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpEventSink {}
impl ISpEventSink_Vtbl {
    pub const fn new<Identity: ISpEventSink_Impl, const OFFSET: isize>() -> ISpEventSink_Vtbl {
        unsafe extern "system" fn AddEvents<Identity: ISpEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventarray: *const SPEVENT, ulcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpEventSink_Impl::AddEvents(this, core::mem::transmute_copy(&peventarray), core::mem::transmute_copy(&ulcount)).into()
        }
        unsafe extern "system" fn GetEventInterest<Identity: ISpEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulleventinterest: *mut u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpEventSink_Impl::GetEventInterest(this, core::mem::transmute_copy(&pulleventinterest)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddEvents: AddEvents::<Identity, OFFSET>,
            GetEventInterest: GetEventInterest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpEventSink as windows_core::Interface>::IID
    }
}
pub trait ISpEventSource_Impl: Sized + ISpNotifySource_Impl {
    fn SetInterest(&self, ulleventinterest: u64, ullqueuedinterest: u64) -> windows_core::Result<()>;
    fn GetEvents(&self, ulcount: u32, peventarray: *mut SPEVENT, pulfetched: *mut u32) -> windows_core::Result<()>;
    fn GetInfo(&self, pinfo: *mut SPEVENTSOURCEINFO) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpEventSource {}
impl ISpEventSource_Vtbl {
    pub const fn new<Identity: ISpEventSource_Impl, const OFFSET: isize>() -> ISpEventSource_Vtbl {
        unsafe extern "system" fn SetInterest<Identity: ISpEventSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulleventinterest: u64, ullqueuedinterest: u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpEventSource_Impl::SetInterest(this, core::mem::transmute_copy(&ulleventinterest), core::mem::transmute_copy(&ullqueuedinterest)).into()
        }
        unsafe extern "system" fn GetEvents<Identity: ISpEventSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, peventarray: *mut SPEVENT, pulfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpEventSource_Impl::GetEvents(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&peventarray), core::mem::transmute_copy(&pulfetched)).into()
        }
        unsafe extern "system" fn GetInfo<Identity: ISpEventSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *mut SPEVENTSOURCEINFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpEventSource_Impl::GetInfo(this, core::mem::transmute_copy(&pinfo)).into()
        }
        Self {
            base__: ISpNotifySource_Vtbl::new::<Identity, OFFSET>(),
            SetInterest: SetInterest::<Identity, OFFSET>,
            GetEvents: GetEvents::<Identity, OFFSET>,
            GetInfo: GetInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpEventSource as windows_core::Interface>::IID || iid == &<ISpNotifySource as windows_core::Interface>::IID
    }
}
pub trait ISpEventSource2_Impl: Sized + ISpEventSource_Impl {
    fn GetEventsEx(&self, ulcount: u32, peventarray: *mut SPEVENTEX, pulfetched: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpEventSource2 {}
impl ISpEventSource2_Vtbl {
    pub const fn new<Identity: ISpEventSource2_Impl, const OFFSET: isize>() -> ISpEventSource2_Vtbl {
        unsafe extern "system" fn GetEventsEx<Identity: ISpEventSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulcount: u32, peventarray: *mut SPEVENTEX, pulfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpEventSource2_Impl::GetEventsEx(this, core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&peventarray), core::mem::transmute_copy(&pulfetched)).into()
        }
        Self { base__: ISpEventSource_Vtbl::new::<Identity, OFFSET>(), GetEventsEx: GetEventsEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpEventSource2 as windows_core::Interface>::IID || iid == &<ISpNotifySource as windows_core::Interface>::IID || iid == &<ISpEventSource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpGramCompBackend_Impl: Sized + ISpGrammarBuilder_Impl {
    fn SetSaveObjects(&self, pstream: Option<&super::super::System::Com::IStream>, perrorlog: Option<&ISpErrorLog>) -> windows_core::Result<()>;
    fn InitFromBinaryGrammar(&self, pbinarydata: *const SPBINARYGRAMMAR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpGramCompBackend {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpGramCompBackend_Vtbl {
    pub const fn new<Identity: ISpGramCompBackend_Impl, const OFFSET: isize>() -> ISpGramCompBackend_Vtbl {
        unsafe extern "system" fn SetSaveObjects<Identity: ISpGramCompBackend_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, perrorlog: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpGramCompBackend_Impl::SetSaveObjects(this, windows_core::from_raw_borrowed(&pstream), windows_core::from_raw_borrowed(&perrorlog)).into()
        }
        unsafe extern "system" fn InitFromBinaryGrammar<Identity: ISpGramCompBackend_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbinarydata: *const SPBINARYGRAMMAR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpGramCompBackend_Impl::InitFromBinaryGrammar(this, core::mem::transmute_copy(&pbinarydata)).into()
        }
        Self {
            base__: ISpGrammarBuilder_Vtbl::new::<Identity, OFFSET>(),
            SetSaveObjects: SetSaveObjects::<Identity, OFFSET>,
            InitFromBinaryGrammar: InitFromBinaryGrammar::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpGramCompBackend as windows_core::Interface>::IID || iid == &<ISpGrammarBuilder as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpGrammarBuilder_Impl: Sized + windows_core::IUnknownImpl {
    fn ResetGrammar(&self, newlanguage: u16) -> windows_core::Result<()>;
    fn GetRule(&self, pszrulename: &windows_core::PCWSTR, dwruleid: u32, dwattributes: u32, fcreateifnotexist: super::super::Foundation::BOOL, phinitialstate: *mut SPSTATEHANDLE) -> windows_core::Result<()>;
    fn ClearRule(&self, hstate: SPSTATEHANDLE) -> windows_core::Result<()>;
    fn CreateNewState(&self, hstate: SPSTATEHANDLE, phstate: *mut SPSTATEHANDLE) -> windows_core::Result<()>;
    fn AddWordTransition(&self, hfromstate: SPSTATEHANDLE, htostate: SPSTATEHANDLE, psz: &windows_core::PCWSTR, pszseparators: &windows_core::PCWSTR, ewordtype: SPGRAMMARWORDTYPE, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> windows_core::Result<()>;
    fn AddRuleTransition(&self, hfromstate: SPSTATEHANDLE, htostate: SPSTATEHANDLE, hrule: SPSTATEHANDLE, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> windows_core::Result<()>;
    fn AddResource(&self, hrulestate: SPSTATEHANDLE, pszresourcename: &windows_core::PCWSTR, pszresourcevalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Commit(&self, dwreserved: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpGrammarBuilder {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpGrammarBuilder_Vtbl {
    pub const fn new<Identity: ISpGrammarBuilder_Impl, const OFFSET: isize>() -> ISpGrammarBuilder_Vtbl {
        unsafe extern "system" fn ResetGrammar<Identity: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newlanguage: u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpGrammarBuilder_Impl::ResetGrammar(this, core::mem::transmute_copy(&newlanguage)).into()
        }
        unsafe extern "system" fn GetRule<Identity: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszrulename: windows_core::PCWSTR, dwruleid: u32, dwattributes: u32, fcreateifnotexist: super::super::Foundation::BOOL, phinitialstate: *mut SPSTATEHANDLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpGrammarBuilder_Impl::GetRule(this, core::mem::transmute(&pszrulename), core::mem::transmute_copy(&dwruleid), core::mem::transmute_copy(&dwattributes), core::mem::transmute_copy(&fcreateifnotexist), core::mem::transmute_copy(&phinitialstate)).into()
        }
        unsafe extern "system" fn ClearRule<Identity: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hstate: SPSTATEHANDLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpGrammarBuilder_Impl::ClearRule(this, core::mem::transmute_copy(&hstate)).into()
        }
        unsafe extern "system" fn CreateNewState<Identity: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hstate: SPSTATEHANDLE, phstate: *mut SPSTATEHANDLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpGrammarBuilder_Impl::CreateNewState(this, core::mem::transmute_copy(&hstate), core::mem::transmute_copy(&phstate)).into()
        }
        unsafe extern "system" fn AddWordTransition<Identity: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfromstate: SPSTATEHANDLE, htostate: SPSTATEHANDLE, psz: windows_core::PCWSTR, pszseparators: windows_core::PCWSTR, ewordtype: SPGRAMMARWORDTYPE, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpGrammarBuilder_Impl::AddWordTransition(this, core::mem::transmute_copy(&hfromstate), core::mem::transmute_copy(&htostate), core::mem::transmute(&psz), core::mem::transmute(&pszseparators), core::mem::transmute_copy(&ewordtype), core::mem::transmute_copy(&weight), core::mem::transmute_copy(&ppropinfo)).into()
        }
        unsafe extern "system" fn AddRuleTransition<Identity: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfromstate: SPSTATEHANDLE, htostate: SPSTATEHANDLE, hrule: SPSTATEHANDLE, weight: f32, ppropinfo: *const SPPROPERTYINFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpGrammarBuilder_Impl::AddRuleTransition(this, core::mem::transmute_copy(&hfromstate), core::mem::transmute_copy(&htostate), core::mem::transmute_copy(&hrule), core::mem::transmute_copy(&weight), core::mem::transmute_copy(&ppropinfo)).into()
        }
        unsafe extern "system" fn AddResource<Identity: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrulestate: SPSTATEHANDLE, pszresourcename: windows_core::PCWSTR, pszresourcevalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpGrammarBuilder_Impl::AddResource(this, core::mem::transmute_copy(&hrulestate), core::mem::transmute(&pszresourcename), core::mem::transmute(&pszresourcevalue)).into()
        }
        unsafe extern "system" fn Commit<Identity: ISpGrammarBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpGrammarBuilder_Impl::Commit(this, core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ResetGrammar: ResetGrammar::<Identity, OFFSET>,
            GetRule: GetRule::<Identity, OFFSET>,
            ClearRule: ClearRule::<Identity, OFFSET>,
            CreateNewState: CreateNewState::<Identity, OFFSET>,
            AddWordTransition: AddWordTransition::<Identity, OFFSET>,
            AddRuleTransition: AddRuleTransition::<Identity, OFFSET>,
            AddResource: AddResource::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpGrammarBuilder as windows_core::Interface>::IID
    }
}
pub trait ISpGrammarBuilder2_Impl: Sized + windows_core::IUnknownImpl {
    fn AddTextSubset(&self, hfromstate: SPSTATEHANDLE, htostate: SPSTATEHANDLE, psz: &windows_core::PCWSTR, ematchmode: SPMATCHINGMODE) -> windows_core::Result<()>;
    fn SetPhoneticAlphabet(&self, phoneticalphabet: PHONETICALPHABET) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpGrammarBuilder2 {}
impl ISpGrammarBuilder2_Vtbl {
    pub const fn new<Identity: ISpGrammarBuilder2_Impl, const OFFSET: isize>() -> ISpGrammarBuilder2_Vtbl {
        unsafe extern "system" fn AddTextSubset<Identity: ISpGrammarBuilder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hfromstate: SPSTATEHANDLE, htostate: SPSTATEHANDLE, psz: windows_core::PCWSTR, ematchmode: SPMATCHINGMODE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpGrammarBuilder2_Impl::AddTextSubset(this, core::mem::transmute_copy(&hfromstate), core::mem::transmute_copy(&htostate), core::mem::transmute(&psz), core::mem::transmute_copy(&ematchmode)).into()
        }
        unsafe extern "system" fn SetPhoneticAlphabet<Identity: ISpGrammarBuilder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phoneticalphabet: PHONETICALPHABET) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpGrammarBuilder2_Impl::SetPhoneticAlphabet(this, core::mem::transmute_copy(&phoneticalphabet)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddTextSubset: AddTextSubset::<Identity, OFFSET>,
            SetPhoneticAlphabet: SetPhoneticAlphabet::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpGrammarBuilder2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpGrammarCompiler_Impl: Sized + windows_core::IUnknownImpl {
    fn CompileStream(&self, psource: Option<&super::super::System::Com::IStream>, pdest: Option<&super::super::System::Com::IStream>, pheader: Option<&super::super::System::Com::IStream>, preserved: Option<&windows_core::IUnknown>, perrorlog: Option<&ISpErrorLog>, dwflags: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISpGrammarCompiler {}
#[cfg(feature = "Win32_System_Com")]
impl ISpGrammarCompiler_Vtbl {
    pub const fn new<Identity: ISpGrammarCompiler_Impl, const OFFSET: isize>() -> ISpGrammarCompiler_Vtbl {
        unsafe extern "system" fn CompileStream<Identity: ISpGrammarCompiler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psource: *mut core::ffi::c_void, pdest: *mut core::ffi::c_void, pheader: *mut core::ffi::c_void, preserved: *mut core::ffi::c_void, perrorlog: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpGrammarCompiler_Impl::CompileStream(this, windows_core::from_raw_borrowed(&psource), windows_core::from_raw_borrowed(&pdest), windows_core::from_raw_borrowed(&pheader), windows_core::from_raw_borrowed(&preserved), windows_core::from_raw_borrowed(&perrorlog), core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CompileStream: CompileStream::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpGrammarCompiler as windows_core::Interface>::IID
    }
}
pub trait ISpITNProcessor_Impl: Sized + windows_core::IUnknownImpl {
    fn LoadITNGrammar(&self, pszclsid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ITNPhrase(&self, pphrase: Option<&ISpPhraseBuilder>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpITNProcessor {}
impl ISpITNProcessor_Vtbl {
    pub const fn new<Identity: ISpITNProcessor_Impl, const OFFSET: isize>() -> ISpITNProcessor_Vtbl {
        unsafe extern "system" fn LoadITNGrammar<Identity: ISpITNProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszclsid: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpITNProcessor_Impl::LoadITNGrammar(this, core::mem::transmute(&pszclsid)).into()
        }
        unsafe extern "system" fn ITNPhrase<Identity: ISpITNProcessor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphrase: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpITNProcessor_Impl::ITNPhrase(this, windows_core::from_raw_borrowed(&pphrase)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LoadITNGrammar: LoadITNGrammar::<Identity, OFFSET>,
            ITNPhrase: ITNPhrase::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpITNProcessor as windows_core::Interface>::IID
    }
}
pub trait ISpLexicon_Impl: Sized + windows_core::IUnknownImpl {
    fn GetPronunciations(&self, pszword: &windows_core::PCWSTR, langid: u16, dwflags: u32, pwordpronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> windows_core::Result<()>;
    fn AddPronunciation(&self, pszword: &windows_core::PCWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> windows_core::Result<()>;
    fn RemovePronunciation(&self, pszword: &windows_core::PCWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> windows_core::Result<()>;
    fn GetGeneration(&self, pdwgeneration: *mut u32) -> windows_core::Result<()>;
    fn GetGenerationChange(&self, dwflags: u32, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> windows_core::Result<()>;
    fn GetWords(&self, dwflags: u32, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpLexicon {}
impl ISpLexicon_Vtbl {
    pub const fn new<Identity: ISpLexicon_Impl, const OFFSET: isize>() -> ISpLexicon_Vtbl {
        unsafe extern "system" fn GetPronunciations<Identity: ISpLexicon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszword: windows_core::PCWSTR, langid: u16, dwflags: u32, pwordpronunciationlist: *mut SPWORDPRONUNCIATIONLIST) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpLexicon_Impl::GetPronunciations(this, core::mem::transmute(&pszword), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pwordpronunciationlist)).into()
        }
        unsafe extern "system" fn AddPronunciation<Identity: ISpLexicon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszword: windows_core::PCWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpLexicon_Impl::AddPronunciation(this, core::mem::transmute(&pszword), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&epartofspeech), core::mem::transmute_copy(&pszpronunciation)).into()
        }
        unsafe extern "system" fn RemovePronunciation<Identity: ISpLexicon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszword: windows_core::PCWSTR, langid: u16, epartofspeech: SPPARTOFSPEECH, pszpronunciation: *const u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpLexicon_Impl::RemovePronunciation(this, core::mem::transmute(&pszword), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&epartofspeech), core::mem::transmute_copy(&pszpronunciation)).into()
        }
        unsafe extern "system" fn GetGeneration<Identity: ISpLexicon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwgeneration: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpLexicon_Impl::GetGeneration(this, core::mem::transmute_copy(&pdwgeneration)).into()
        }
        unsafe extern "system" fn GetGenerationChange<Identity: ISpLexicon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpLexicon_Impl::GetGenerationChange(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pdwgeneration), core::mem::transmute_copy(&pwordlist)).into()
        }
        unsafe extern "system" fn GetWords<Identity: ISpLexicon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpLexicon_Impl::GetWords(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pdwgeneration), core::mem::transmute_copy(&pdwcookie), core::mem::transmute_copy(&pwordlist)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPronunciations: GetPronunciations::<Identity, OFFSET>,
            AddPronunciation: AddPronunciation::<Identity, OFFSET>,
            RemovePronunciation: RemovePronunciation::<Identity, OFFSET>,
            GetGeneration: GetGeneration::<Identity, OFFSET>,
            GetGenerationChange: GetGenerationChange::<Identity, OFFSET>,
            GetWords: GetWords::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpLexicon as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
pub trait ISpMMSysAudio_Impl: Sized + ISpAudio_Impl {
    fn GetDeviceId(&self, pudeviceid: *mut u32) -> windows_core::Result<()>;
    fn SetDeviceId(&self, udeviceid: u32) -> windows_core::Result<()>;
    fn GetMMHandle(&self, phandle: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetLineId(&self, pulineid: *mut u32) -> windows_core::Result<()>;
    fn SetLineId(&self, ulineid: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ISpMMSysAudio {}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ISpMMSysAudio_Vtbl {
    pub const fn new<Identity: ISpMMSysAudio_Impl, const OFFSET: isize>() -> ISpMMSysAudio_Vtbl {
        unsafe extern "system" fn GetDeviceId<Identity: ISpMMSysAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pudeviceid: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpMMSysAudio_Impl::GetDeviceId(this, core::mem::transmute_copy(&pudeviceid)).into()
        }
        unsafe extern "system" fn SetDeviceId<Identity: ISpMMSysAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, udeviceid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpMMSysAudio_Impl::SetDeviceId(this, core::mem::transmute_copy(&udeviceid)).into()
        }
        unsafe extern "system" fn GetMMHandle<Identity: ISpMMSysAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phandle: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpMMSysAudio_Impl::GetMMHandle(this, core::mem::transmute_copy(&phandle)).into()
        }
        unsafe extern "system" fn GetLineId<Identity: ISpMMSysAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulineid: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpMMSysAudio_Impl::GetLineId(this, core::mem::transmute_copy(&pulineid)).into()
        }
        unsafe extern "system" fn SetLineId<Identity: ISpMMSysAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulineid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpMMSysAudio_Impl::SetLineId(this, core::mem::transmute_copy(&ulineid)).into()
        }
        Self {
            base__: ISpAudio_Vtbl::new::<Identity, OFFSET>(),
            GetDeviceId: GetDeviceId::<Identity, OFFSET>,
            SetDeviceId: SetDeviceId::<Identity, OFFSET>,
            GetMMHandle: GetMMHandle::<Identity, OFFSET>,
            GetLineId: GetLineId::<Identity, OFFSET>,
            SetLineId: SetLineId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpMMSysAudio as windows_core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IStream as windows_core::Interface>::IID || iid == &<ISpStreamFormat as windows_core::Interface>::IID || iid == &<ISpAudio as windows_core::Interface>::IID
    }
}
pub trait ISpNotifyCallback_Impl: Sized {
    fn NotifyCallback(&self, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
}
impl ISpNotifyCallback_Vtbl {
    pub const fn new<Impl: ISpNotifyCallback_Impl>() -> ISpNotifyCallback_Vtbl {
        unsafe extern "system" fn NotifyCallback<Impl: ISpNotifyCallback_Impl>(this: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ISpNotifyCallback_Impl::NotifyCallback(this, core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
        }
        Self { NotifyCallback: NotifyCallback::<Impl> }
    }
}
#[doc(hidden)]
struct ISpNotifyCallback_ImplVtbl<T: ISpNotifyCallback_Impl>(std::marker::PhantomData<T>);
impl<T: ISpNotifyCallback_Impl> ISpNotifyCallback_ImplVtbl<T> {
    const VTABLE: ISpNotifyCallback_Vtbl = ISpNotifyCallback_Vtbl::new::<T>();
}
impl ISpNotifyCallback {
    pub fn new<'a, T: ISpNotifyCallback_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ISpNotifyCallback_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait ISpNotifySink_Impl: Sized + windows_core::IUnknownImpl {
    fn Notify(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpNotifySink {}
impl ISpNotifySink_Vtbl {
    pub const fn new<Identity: ISpNotifySink_Impl, const OFFSET: isize>() -> ISpNotifySink_Vtbl {
        unsafe extern "system" fn Notify<Identity: ISpNotifySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpNotifySink_Impl::Notify(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Notify: Notify::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpNotifySink as windows_core::Interface>::IID
    }
}
pub trait ISpNotifySource_Impl: Sized + windows_core::IUnknownImpl {
    fn SetNotifySink(&self, pnotifysink: Option<&ISpNotifySink>) -> windows_core::Result<()>;
    fn SetNotifyWindowMessage(&self, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn SetNotifyCallbackFunction(&self, pfncallback: *mut SPNOTIFYCALLBACK, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn SetNotifyCallbackInterface(&self, pspcallback: Option<&ISpNotifyCallback>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn SetNotifyWin32Event(&self) -> windows_core::Result<()>;
    fn WaitForNotifyEvent(&self, dwmilliseconds: u32) -> windows_core::Result<()>;
    fn GetNotifyEventHandle(&self) -> super::super::Foundation::HANDLE;
}
impl windows_core::RuntimeName for ISpNotifySource {}
impl ISpNotifySource_Vtbl {
    pub const fn new<Identity: ISpNotifySource_Impl, const OFFSET: isize>() -> ISpNotifySource_Vtbl {
        unsafe extern "system" fn SetNotifySink<Identity: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnotifysink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpNotifySource_Impl::SetNotifySink(this, windows_core::from_raw_borrowed(&pnotifysink)).into()
        }
        unsafe extern "system" fn SetNotifyWindowMessage<Identity: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpNotifySource_Impl::SetNotifyWindowMessage(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn SetNotifyCallbackFunction<Identity: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfncallback: *mut SPNOTIFYCALLBACK, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpNotifySource_Impl::SetNotifyCallbackFunction(this, core::mem::transmute_copy(&pfncallback), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn SetNotifyCallbackInterface<Identity: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pspcallback: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpNotifySource_Impl::SetNotifyCallbackInterface(this, windows_core::from_raw_borrowed(&pspcallback), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn SetNotifyWin32Event<Identity: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpNotifySource_Impl::SetNotifyWin32Event(this).into()
        }
        unsafe extern "system" fn WaitForNotifyEvent<Identity: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmilliseconds: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpNotifySource_Impl::WaitForNotifyEvent(this, core::mem::transmute_copy(&dwmilliseconds)).into()
        }
        unsafe extern "system" fn GetNotifyEventHandle<Identity: ISpNotifySource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpNotifySource_Impl::GetNotifyEventHandle(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetNotifySink: SetNotifySink::<Identity, OFFSET>,
            SetNotifyWindowMessage: SetNotifyWindowMessage::<Identity, OFFSET>,
            SetNotifyCallbackFunction: SetNotifyCallbackFunction::<Identity, OFFSET>,
            SetNotifyCallbackInterface: SetNotifyCallbackInterface::<Identity, OFFSET>,
            SetNotifyWin32Event: SetNotifyWin32Event::<Identity, OFFSET>,
            WaitForNotifyEvent: WaitForNotifyEvent::<Identity, OFFSET>,
            GetNotifyEventHandle: GetNotifyEventHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpNotifySource as windows_core::Interface>::IID
    }
}
pub trait ISpNotifyTranslator_Impl: Sized + ISpNotifySink_Impl {
    fn InitWindowMessage(&self, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn InitCallback(&self, pfncallback: *mut SPNOTIFYCALLBACK, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn InitSpNotifyCallback(&self, pspcallback: Option<&ISpNotifyCallback>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn InitWin32Event(&self, hevent: super::super::Foundation::HANDLE, fclosehandleonrelease: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Wait(&self, dwmilliseconds: u32) -> windows_core::Result<()>;
    fn GetEventHandle(&self) -> super::super::Foundation::HANDLE;
}
impl windows_core::RuntimeName for ISpNotifyTranslator {}
impl ISpNotifyTranslator_Vtbl {
    pub const fn new<Identity: ISpNotifyTranslator_Impl, const OFFSET: isize>() -> ISpNotifyTranslator_Vtbl {
        unsafe extern "system" fn InitWindowMessage<Identity: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpNotifyTranslator_Impl::InitWindowMessage(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn InitCallback<Identity: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfncallback: *mut SPNOTIFYCALLBACK, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpNotifyTranslator_Impl::InitCallback(this, core::mem::transmute_copy(&pfncallback), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn InitSpNotifyCallback<Identity: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pspcallback: *mut core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpNotifyTranslator_Impl::InitSpNotifyCallback(this, windows_core::from_raw_borrowed(&pspcallback), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)).into()
        }
        unsafe extern "system" fn InitWin32Event<Identity: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::super::Foundation::HANDLE, fclosehandleonrelease: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpNotifyTranslator_Impl::InitWin32Event(this, core::mem::transmute_copy(&hevent), core::mem::transmute_copy(&fclosehandleonrelease)).into()
        }
        unsafe extern "system" fn Wait<Identity: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmilliseconds: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpNotifyTranslator_Impl::Wait(this, core::mem::transmute_copy(&dwmilliseconds)).into()
        }
        unsafe extern "system" fn GetEventHandle<Identity: ISpNotifyTranslator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpNotifyTranslator_Impl::GetEventHandle(this)
        }
        Self {
            base__: ISpNotifySink_Vtbl::new::<Identity, OFFSET>(),
            InitWindowMessage: InitWindowMessage::<Identity, OFFSET>,
            InitCallback: InitCallback::<Identity, OFFSET>,
            InitSpNotifyCallback: InitSpNotifyCallback::<Identity, OFFSET>,
            InitWin32Event: InitWin32Event::<Identity, OFFSET>,
            Wait: Wait::<Identity, OFFSET>,
            GetEventHandle: GetEventHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpNotifyTranslator as windows_core::Interface>::IID || iid == &<ISpNotifySink as windows_core::Interface>::IID
    }
}
pub trait ISpObjectToken_Impl: Sized + ISpDataKey_Impl {
    fn SetId(&self, pszcategoryid: &windows_core::PCWSTR, psztokenid: &windows_core::PCWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetCategory(&self) -> windows_core::Result<ISpObjectTokenCategory>;
    fn CreateInstance(&self, punkouter: Option<&windows_core::IUnknown>, dwclscontext: u32, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetStorageFileName(&self, clsidcaller: *const windows_core::GUID, pszvaluename: &windows_core::PCWSTR, pszfilenamespecifier: &windows_core::PCWSTR, nfolder: u32) -> windows_core::Result<windows_core::PWSTR>;
    fn RemoveStorageFileName(&self, clsidcaller: *const windows_core::GUID, pszkeyname: &windows_core::PCWSTR, fdeletefile: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Remove(&self, pclsidcaller: *const windows_core::GUID) -> windows_core::Result<()>;
    fn IsUISupported(&self, psztypeofui: &windows_core::PCWSTR, pvextradata: *mut core::ffi::c_void, cbextradata: u32, punkobject: Option<&windows_core::IUnknown>, pfsupported: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn DisplayUI(&self, hwndparent: super::super::Foundation::HWND, psztitle: &windows_core::PCWSTR, psztypeofui: &windows_core::PCWSTR, pvextradata: *mut core::ffi::c_void, cbextradata: u32, punkobject: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn MatchesAttributes(&self, pszattributes: &windows_core::PCWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpObjectToken {}
impl ISpObjectToken_Vtbl {
    pub const fn new<Identity: ISpObjectToken_Impl, const OFFSET: isize>() -> ISpObjectToken_Vtbl {
        unsafe extern "system" fn SetId<Identity: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcategoryid: windows_core::PCWSTR, psztokenid: windows_core::PCWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpObjectToken_Impl::SetId(this, core::mem::transmute(&pszcategoryid), core::mem::transmute(&psztokenid), core::mem::transmute_copy(&fcreateifnotexist)).into()
        }
        unsafe extern "system" fn GetId<Identity: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszcomemtokenid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpObjectToken_Impl::GetId(this) {
                Ok(ok__) => {
                    ppszcomemtokenid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Identity: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptokencategory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpObjectToken_Impl::GetCategory(this) {
                Ok(ok__) => {
                    pptokencategory.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, dwclscontext: u32, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpObjectToken_Impl::CreateInstance(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&dwclscontext), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
        }
        unsafe extern "system" fn GetStorageFileName<Identity: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsidcaller: *const windows_core::GUID, pszvaluename: windows_core::PCWSTR, pszfilenamespecifier: windows_core::PCWSTR, nfolder: u32, ppszfilepath: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpObjectToken_Impl::GetStorageFileName(this, core::mem::transmute_copy(&clsidcaller), core::mem::transmute(&pszvaluename), core::mem::transmute(&pszfilenamespecifier), core::mem::transmute_copy(&nfolder)) {
                Ok(ok__) => {
                    ppszfilepath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStorageFileName<Identity: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsidcaller: *const windows_core::GUID, pszkeyname: windows_core::PCWSTR, fdeletefile: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpObjectToken_Impl::RemoveStorageFileName(this, core::mem::transmute_copy(&clsidcaller), core::mem::transmute(&pszkeyname), core::mem::transmute_copy(&fdeletefile)).into()
        }
        unsafe extern "system" fn Remove<Identity: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclsidcaller: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpObjectToken_Impl::Remove(this, core::mem::transmute_copy(&pclsidcaller)).into()
        }
        unsafe extern "system" fn IsUISupported<Identity: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztypeofui: windows_core::PCWSTR, pvextradata: *mut core::ffi::c_void, cbextradata: u32, punkobject: *mut core::ffi::c_void, pfsupported: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpObjectToken_Impl::IsUISupported(this, core::mem::transmute(&psztypeofui), core::mem::transmute_copy(&pvextradata), core::mem::transmute_copy(&cbextradata), windows_core::from_raw_borrowed(&punkobject), core::mem::transmute_copy(&pfsupported)).into()
        }
        unsafe extern "system" fn DisplayUI<Identity: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, psztitle: windows_core::PCWSTR, psztypeofui: windows_core::PCWSTR, pvextradata: *mut core::ffi::c_void, cbextradata: u32, punkobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpObjectToken_Impl::DisplayUI(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute(&psztitle), core::mem::transmute(&psztypeofui), core::mem::transmute_copy(&pvextradata), core::mem::transmute_copy(&cbextradata), windows_core::from_raw_borrowed(&punkobject)).into()
        }
        unsafe extern "system" fn MatchesAttributes<Identity: ISpObjectToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszattributes: windows_core::PCWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpObjectToken_Impl::MatchesAttributes(this, core::mem::transmute(&pszattributes), core::mem::transmute_copy(&pfmatches)).into()
        }
        Self {
            base__: ISpDataKey_Vtbl::new::<Identity, OFFSET>(),
            SetId: SetId::<Identity, OFFSET>,
            GetId: GetId::<Identity, OFFSET>,
            GetCategory: GetCategory::<Identity, OFFSET>,
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            GetStorageFileName: GetStorageFileName::<Identity, OFFSET>,
            RemoveStorageFileName: RemoveStorageFileName::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            IsUISupported: IsUISupported::<Identity, OFFSET>,
            DisplayUI: DisplayUI::<Identity, OFFSET>,
            MatchesAttributes: MatchesAttributes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpObjectToken as windows_core::Interface>::IID || iid == &<ISpDataKey as windows_core::Interface>::IID
    }
}
pub trait ISpObjectTokenCategory_Impl: Sized + ISpDataKey_Impl {
    fn SetId(&self, pszcategoryid: &windows_core::PCWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetDataKey(&self, spdkl: SPDATAKEYLOCATION) -> windows_core::Result<ISpDataKey>;
    fn EnumTokens(&self, pzsreqattribs: &windows_core::PCWSTR, pszoptattribs: &windows_core::PCWSTR) -> windows_core::Result<IEnumSpObjectTokens>;
    fn SetDefaultTokenId(&self, psztokenid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDefaultTokenId(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for ISpObjectTokenCategory {}
impl ISpObjectTokenCategory_Vtbl {
    pub const fn new<Identity: ISpObjectTokenCategory_Impl, const OFFSET: isize>() -> ISpObjectTokenCategory_Vtbl {
        unsafe extern "system" fn SetId<Identity: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcategoryid: windows_core::PCWSTR, fcreateifnotexist: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpObjectTokenCategory_Impl::SetId(this, core::mem::transmute(&pszcategoryid), core::mem::transmute_copy(&fcreateifnotexist)).into()
        }
        unsafe extern "system" fn GetId<Identity: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszcomemcategoryid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpObjectTokenCategory_Impl::GetId(this) {
                Ok(ok__) => {
                    ppszcomemcategoryid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataKey<Identity: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, spdkl: SPDATAKEYLOCATION, ppdatakey: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpObjectTokenCategory_Impl::GetDataKey(this, core::mem::transmute_copy(&spdkl)) {
                Ok(ok__) => {
                    ppdatakey.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumTokens<Identity: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pzsreqattribs: windows_core::PCWSTR, pszoptattribs: windows_core::PCWSTR, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpObjectTokenCategory_Impl::EnumTokens(this, core::mem::transmute(&pzsreqattribs), core::mem::transmute(&pszoptattribs)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultTokenId<Identity: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztokenid: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpObjectTokenCategory_Impl::SetDefaultTokenId(this, core::mem::transmute(&psztokenid)).into()
        }
        unsafe extern "system" fn GetDefaultTokenId<Identity: ISpObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszcomemtokenid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpObjectTokenCategory_Impl::GetDefaultTokenId(this) {
                Ok(ok__) => {
                    ppszcomemtokenid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISpDataKey_Vtbl::new::<Identity, OFFSET>(),
            SetId: SetId::<Identity, OFFSET>,
            GetId: GetId::<Identity, OFFSET>,
            GetDataKey: GetDataKey::<Identity, OFFSET>,
            EnumTokens: EnumTokens::<Identity, OFFSET>,
            SetDefaultTokenId: SetDefaultTokenId::<Identity, OFFSET>,
            GetDefaultTokenId: GetDefaultTokenId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpObjectTokenCategory as windows_core::Interface>::IID || iid == &<ISpDataKey as windows_core::Interface>::IID
    }
}
pub trait ISpObjectTokenEnumBuilder_Impl: Sized + IEnumSpObjectTokens_Impl {
    fn SetAttribs(&self, pszreqattribs: &windows_core::PCWSTR, pszoptattribs: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddTokens(&self, ctokens: u32, ptoken: *const Option<ISpObjectToken>) -> windows_core::Result<()>;
    fn AddTokensFromDataKey(&self, pdatakey: Option<&ISpDataKey>, pszsubkey: &windows_core::PCWSTR, pszcategoryid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddTokensFromTokenEnum(&self, ptokenenum: Option<&IEnumSpObjectTokens>) -> windows_core::Result<()>;
    fn Sort(&self, psztokenidtolistfirst: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpObjectTokenEnumBuilder {}
impl ISpObjectTokenEnumBuilder_Vtbl {
    pub const fn new<Identity: ISpObjectTokenEnumBuilder_Impl, const OFFSET: isize>() -> ISpObjectTokenEnumBuilder_Vtbl {
        unsafe extern "system" fn SetAttribs<Identity: ISpObjectTokenEnumBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszreqattribs: windows_core::PCWSTR, pszoptattribs: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpObjectTokenEnumBuilder_Impl::SetAttribs(this, core::mem::transmute(&pszreqattribs), core::mem::transmute(&pszoptattribs)).into()
        }
        unsafe extern "system" fn AddTokens<Identity: ISpObjectTokenEnumBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ctokens: u32, ptoken: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpObjectTokenEnumBuilder_Impl::AddTokens(this, core::mem::transmute_copy(&ctokens), core::mem::transmute_copy(&ptoken)).into()
        }
        unsafe extern "system" fn AddTokensFromDataKey<Identity: ISpObjectTokenEnumBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdatakey: *mut core::ffi::c_void, pszsubkey: windows_core::PCWSTR, pszcategoryid: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpObjectTokenEnumBuilder_Impl::AddTokensFromDataKey(this, windows_core::from_raw_borrowed(&pdatakey), core::mem::transmute(&pszsubkey), core::mem::transmute(&pszcategoryid)).into()
        }
        unsafe extern "system" fn AddTokensFromTokenEnum<Identity: ISpObjectTokenEnumBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptokenenum: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpObjectTokenEnumBuilder_Impl::AddTokensFromTokenEnum(this, windows_core::from_raw_borrowed(&ptokenenum)).into()
        }
        unsafe extern "system" fn Sort<Identity: ISpObjectTokenEnumBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztokenidtolistfirst: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpObjectTokenEnumBuilder_Impl::Sort(this, core::mem::transmute(&psztokenidtolistfirst)).into()
        }
        Self {
            base__: IEnumSpObjectTokens_Vtbl::new::<Identity, OFFSET>(),
            SetAttribs: SetAttribs::<Identity, OFFSET>,
            AddTokens: AddTokens::<Identity, OFFSET>,
            AddTokensFromDataKey: AddTokensFromDataKey::<Identity, OFFSET>,
            AddTokensFromTokenEnum: AddTokensFromTokenEnum::<Identity, OFFSET>,
            Sort: Sort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpObjectTokenEnumBuilder as windows_core::Interface>::IID || iid == &<IEnumSpObjectTokens as windows_core::Interface>::IID
    }
}
pub trait ISpObjectTokenInit_Impl: Sized + ISpObjectToken_Impl {
    fn InitFromDataKey(&self, pszcategoryid: &windows_core::PCWSTR, psztokenid: &windows_core::PCWSTR, pdatakey: Option<&ISpDataKey>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpObjectTokenInit {}
impl ISpObjectTokenInit_Vtbl {
    pub const fn new<Identity: ISpObjectTokenInit_Impl, const OFFSET: isize>() -> ISpObjectTokenInit_Vtbl {
        unsafe extern "system" fn InitFromDataKey<Identity: ISpObjectTokenInit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcategoryid: windows_core::PCWSTR, psztokenid: windows_core::PCWSTR, pdatakey: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpObjectTokenInit_Impl::InitFromDataKey(this, core::mem::transmute(&pszcategoryid), core::mem::transmute(&psztokenid), windows_core::from_raw_borrowed(&pdatakey)).into()
        }
        Self { base__: ISpObjectToken_Vtbl::new::<Identity, OFFSET>(), InitFromDataKey: InitFromDataKey::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpObjectTokenInit as windows_core::Interface>::IID || iid == &<ISpDataKey as windows_core::Interface>::IID || iid == &<ISpObjectToken as windows_core::Interface>::IID
    }
}
pub trait ISpObjectWithToken_Impl: Sized + windows_core::IUnknownImpl {
    fn SetObjectToken(&self, ptoken: Option<&ISpObjectToken>) -> windows_core::Result<()>;
    fn GetObjectToken(&self) -> windows_core::Result<ISpObjectToken>;
}
impl windows_core::RuntimeName for ISpObjectWithToken {}
impl ISpObjectWithToken_Vtbl {
    pub const fn new<Identity: ISpObjectWithToken_Impl, const OFFSET: isize>() -> ISpObjectWithToken_Vtbl {
        unsafe extern "system" fn SetObjectToken<Identity: ISpObjectWithToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptoken: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpObjectWithToken_Impl::SetObjectToken(this, windows_core::from_raw_borrowed(&ptoken)).into()
        }
        unsafe extern "system" fn GetObjectToken<Identity: ISpObjectWithToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptoken: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpObjectWithToken_Impl::GetObjectToken(this) {
                Ok(ok__) => {
                    pptoken.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetObjectToken: SetObjectToken::<Identity, OFFSET>,
            GetObjectToken: GetObjectToken::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpObjectWithToken as windows_core::Interface>::IID
    }
}
pub trait ISpPhoneConverter_Impl: Sized + ISpObjectWithToken_Impl {
    fn PhoneToId(&self, pszphone: &windows_core::PCWSTR) -> windows_core::Result<u16>;
    fn IdToPhone(&self, pid: *const u16, pszphone: windows_core::PWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpPhoneConverter {}
impl ISpPhoneConverter_Vtbl {
    pub const fn new<Identity: ISpPhoneConverter_Impl, const OFFSET: isize>() -> ISpPhoneConverter_Vtbl {
        unsafe extern "system" fn PhoneToId<Identity: ISpPhoneConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszphone: windows_core::PCWSTR, pid: *mut u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpPhoneConverter_Impl::PhoneToId(this, core::mem::transmute(&pszphone)) {
                Ok(ok__) => {
                    pid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IdToPhone<Identity: ISpPhoneConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *const u16, pszphone: windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpPhoneConverter_Impl::IdToPhone(this, core::mem::transmute_copy(&pid), core::mem::transmute_copy(&pszphone)).into()
        }
        Self { base__: ISpObjectWithToken_Vtbl::new::<Identity, OFFSET>(), PhoneToId: PhoneToId::<Identity, OFFSET>, IdToPhone: IdToPhone::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpPhoneConverter as windows_core::Interface>::IID || iid == &<ISpObjectWithToken as windows_core::Interface>::IID
    }
}
pub trait ISpPhoneticAlphabetConverter_Impl: Sized + windows_core::IUnknownImpl {
    fn GetLangId(&self) -> windows_core::Result<u16>;
    fn SetLangId(&self, langid: u16) -> windows_core::Result<()>;
    fn SAPI2UPS(&self, pszsapiid: *const u16, pszupsid: *mut u16, cmaxlength: u32) -> windows_core::Result<()>;
    fn UPS2SAPI(&self, pszupsid: *const u16, pszsapiid: *mut u16, cmaxlength: u32) -> windows_core::Result<()>;
    fn GetMaxConvertLength(&self, csrclength: u32, bsapi2ups: super::super::Foundation::BOOL) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for ISpPhoneticAlphabetConverter {}
impl ISpPhoneticAlphabetConverter_Vtbl {
    pub const fn new<Identity: ISpPhoneticAlphabetConverter_Impl, const OFFSET: isize>() -> ISpPhoneticAlphabetConverter_Vtbl {
        unsafe extern "system" fn GetLangId<Identity: ISpPhoneticAlphabetConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plangid: *mut u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpPhoneticAlphabetConverter_Impl::GetLangId(this) {
                Ok(ok__) => {
                    plangid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLangId<Identity: ISpPhoneticAlphabetConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpPhoneticAlphabetConverter_Impl::SetLangId(this, core::mem::transmute_copy(&langid)).into()
        }
        unsafe extern "system" fn SAPI2UPS<Identity: ISpPhoneticAlphabetConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsapiid: *const u16, pszupsid: *mut u16, cmaxlength: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpPhoneticAlphabetConverter_Impl::SAPI2UPS(this, core::mem::transmute_copy(&pszsapiid), core::mem::transmute_copy(&pszupsid), core::mem::transmute_copy(&cmaxlength)).into()
        }
        unsafe extern "system" fn UPS2SAPI<Identity: ISpPhoneticAlphabetConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszupsid: *const u16, pszsapiid: *mut u16, cmaxlength: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpPhoneticAlphabetConverter_Impl::UPS2SAPI(this, core::mem::transmute_copy(&pszupsid), core::mem::transmute_copy(&pszsapiid), core::mem::transmute_copy(&cmaxlength)).into()
        }
        unsafe extern "system" fn GetMaxConvertLength<Identity: ISpPhoneticAlphabetConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, csrclength: u32, bsapi2ups: super::super::Foundation::BOOL, pcmaxdestlength: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpPhoneticAlphabetConverter_Impl::GetMaxConvertLength(this, core::mem::transmute_copy(&csrclength), core::mem::transmute_copy(&bsapi2ups)) {
                Ok(ok__) => {
                    pcmaxdestlength.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLangId: GetLangId::<Identity, OFFSET>,
            SetLangId: SetLangId::<Identity, OFFSET>,
            SAPI2UPS: SAPI2UPS::<Identity, OFFSET>,
            UPS2SAPI: UPS2SAPI::<Identity, OFFSET>,
            GetMaxConvertLength: GetMaxConvertLength::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpPhoneticAlphabetConverter as windows_core::Interface>::IID
    }
}
pub trait ISpPhoneticAlphabetSelection_Impl: Sized + windows_core::IUnknownImpl {
    fn IsAlphabetUPS(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn SetAlphabetToUPS(&self, fforceups: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpPhoneticAlphabetSelection {}
impl ISpPhoneticAlphabetSelection_Vtbl {
    pub const fn new<Identity: ISpPhoneticAlphabetSelection_Impl, const OFFSET: isize>() -> ISpPhoneticAlphabetSelection_Vtbl {
        unsafe extern "system" fn IsAlphabetUPS<Identity: ISpPhoneticAlphabetSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisups: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpPhoneticAlphabetSelection_Impl::IsAlphabetUPS(this) {
                Ok(ok__) => {
                    pfisups.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlphabetToUPS<Identity: ISpPhoneticAlphabetSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fforceups: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpPhoneticAlphabetSelection_Impl::SetAlphabetToUPS(this, core::mem::transmute_copy(&fforceups)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsAlphabetUPS: IsAlphabetUPS::<Identity, OFFSET>,
            SetAlphabetToUPS: SetAlphabetToUPS::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpPhoneticAlphabetSelection as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpPhrase_Impl: Sized + windows_core::IUnknownImpl {
    fn GetPhrase(&self) -> windows_core::Result<*mut SPPHRASE>;
    fn GetSerializedPhrase(&self) -> windows_core::Result<*mut SPSERIALIZEDPHRASE>;
    fn GetText(&self, ulstart: u32, ulcount: u32, fusetextreplacements: super::super::Foundation::BOOL, ppszcomemtext: *mut windows_core::PWSTR, pbdisplayattributes: *mut u8) -> windows_core::Result<()>;
    fn Discard(&self, dwvaluetypes: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpPhrase {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpPhrase_Vtbl {
    pub const fn new<Identity: ISpPhrase_Impl, const OFFSET: isize>() -> ISpPhrase_Vtbl {
        unsafe extern "system" fn GetPhrase<Identity: ISpPhrase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcomemphrase: *mut *mut SPPHRASE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpPhrase_Impl::GetPhrase(this) {
                Ok(ok__) => {
                    ppcomemphrase.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSerializedPhrase<Identity: ISpPhrase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcomemphrase: *mut *mut SPSERIALIZEDPHRASE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpPhrase_Impl::GetSerializedPhrase(this) {
                Ok(ok__) => {
                    ppcomemphrase.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Identity: ISpPhrase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulstart: u32, ulcount: u32, fusetextreplacements: super::super::Foundation::BOOL, ppszcomemtext: *mut windows_core::PWSTR, pbdisplayattributes: *mut u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpPhrase_Impl::GetText(this, core::mem::transmute_copy(&ulstart), core::mem::transmute_copy(&ulcount), core::mem::transmute_copy(&fusetextreplacements), core::mem::transmute_copy(&ppszcomemtext), core::mem::transmute_copy(&pbdisplayattributes)).into()
        }
        unsafe extern "system" fn Discard<Identity: ISpPhrase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwvaluetypes: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpPhrase_Impl::Discard(this, core::mem::transmute_copy(&dwvaluetypes)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPhrase: GetPhrase::<Identity, OFFSET>,
            GetSerializedPhrase: GetSerializedPhrase::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            Discard: Discard::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpPhrase as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpPhrase2_Impl: Sized + ISpPhrase_Impl {
    fn GetXMLResult(&self, ppszcomemxmlresult: *mut windows_core::PWSTR, options: SPXMLRESULTOPTIONS) -> windows_core::Result<()>;
    fn GetXMLErrorInfo(&self, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> windows_core::Result<()>;
    fn GetAudio(&self, ulstartelement: u32, celements: u32) -> windows_core::Result<ISpStreamFormat>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpPhrase2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpPhrase2_Vtbl {
    pub const fn new<Identity: ISpPhrase2_Impl, const OFFSET: isize>() -> ISpPhrase2_Vtbl {
        unsafe extern "system" fn GetXMLResult<Identity: ISpPhrase2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszcomemxmlresult: *mut windows_core::PWSTR, options: SPXMLRESULTOPTIONS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpPhrase2_Impl::GetXMLResult(this, core::mem::transmute_copy(&ppszcomemxmlresult), core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn GetXMLErrorInfo<Identity: ISpPhrase2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpPhrase2_Impl::GetXMLErrorInfo(this, core::mem::transmute_copy(&psemanticerrorinfo)).into()
        }
        unsafe extern "system" fn GetAudio<Identity: ISpPhrase2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulstartelement: u32, celements: u32, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpPhrase2_Impl::GetAudio(this, core::mem::transmute_copy(&ulstartelement), core::mem::transmute_copy(&celements)) {
                Ok(ok__) => {
                    ppstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISpPhrase_Vtbl::new::<Identity, OFFSET>(),
            GetXMLResult: GetXMLResult::<Identity, OFFSET>,
            GetXMLErrorInfo: GetXMLErrorInfo::<Identity, OFFSET>,
            GetAudio: GetAudio::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpPhrase2 as windows_core::Interface>::IID || iid == &<ISpPhrase as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpPhraseAlt_Impl: Sized + ISpPhrase_Impl {
    fn GetAltInfo(&self, ppparent: *mut Option<ISpPhrase>, pulstartelementinparent: *mut u32, pcelementsinparent: *mut u32, pcelementsinalt: *mut u32) -> windows_core::Result<()>;
    fn Commit(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpPhraseAlt {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpPhraseAlt_Vtbl {
    pub const fn new<Identity: ISpPhraseAlt_Impl, const OFFSET: isize>() -> ISpPhraseAlt_Vtbl {
        unsafe extern "system" fn GetAltInfo<Identity: ISpPhraseAlt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparent: *mut *mut core::ffi::c_void, pulstartelementinparent: *mut u32, pcelementsinparent: *mut u32, pcelementsinalt: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpPhraseAlt_Impl::GetAltInfo(this, core::mem::transmute_copy(&ppparent), core::mem::transmute_copy(&pulstartelementinparent), core::mem::transmute_copy(&pcelementsinparent), core::mem::transmute_copy(&pcelementsinalt)).into()
        }
        unsafe extern "system" fn Commit<Identity: ISpPhraseAlt_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpPhraseAlt_Impl::Commit(this).into()
        }
        Self { base__: ISpPhrase_Vtbl::new::<Identity, OFFSET>(), GetAltInfo: GetAltInfo::<Identity, OFFSET>, Commit: Commit::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpPhraseAlt as windows_core::Interface>::IID || iid == &<ISpPhrase as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpPhraseBuilder_Impl: Sized + ISpPhrase_Impl {
    fn InitFromPhrase(&self, pphrase: *const SPPHRASE) -> windows_core::Result<()>;
    fn InitFromSerializedPhrase(&self, pphrase: *const SPSERIALIZEDPHRASE) -> windows_core::Result<()>;
    fn AddElements(&self, celements: u32, pelement: *const SPPHRASEELEMENT) -> windows_core::Result<()>;
    fn AddRules(&self, hparent: SPPHRASERULEHANDLE, prule: *const SPPHRASERULE) -> windows_core::Result<SPPHRASERULEHANDLE>;
    fn AddProperties(&self, hparent: SPPHRASEPROPERTYHANDLE, pproperty: *const SPPHRASEPROPERTY) -> windows_core::Result<SPPHRASEPROPERTYHANDLE>;
    fn AddReplacements(&self, creplacements: u32, preplacements: *const SPPHRASEREPLACEMENT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpPhraseBuilder {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpPhraseBuilder_Vtbl {
    pub const fn new<Identity: ISpPhraseBuilder_Impl, const OFFSET: isize>() -> ISpPhraseBuilder_Vtbl {
        unsafe extern "system" fn InitFromPhrase<Identity: ISpPhraseBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphrase: *const SPPHRASE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpPhraseBuilder_Impl::InitFromPhrase(this, core::mem::transmute_copy(&pphrase)).into()
        }
        unsafe extern "system" fn InitFromSerializedPhrase<Identity: ISpPhraseBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphrase: *const SPSERIALIZEDPHRASE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpPhraseBuilder_Impl::InitFromSerializedPhrase(this, core::mem::transmute_copy(&pphrase)).into()
        }
        unsafe extern "system" fn AddElements<Identity: ISpPhraseBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celements: u32, pelement: *const SPPHRASEELEMENT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpPhraseBuilder_Impl::AddElements(this, core::mem::transmute_copy(&celements), core::mem::transmute_copy(&pelement)).into()
        }
        unsafe extern "system" fn AddRules<Identity: ISpPhraseBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hparent: SPPHRASERULEHANDLE, prule: *const SPPHRASERULE, phnewrule: *mut SPPHRASERULEHANDLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpPhraseBuilder_Impl::AddRules(this, core::mem::transmute_copy(&hparent), core::mem::transmute_copy(&prule)) {
                Ok(ok__) => {
                    phnewrule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddProperties<Identity: ISpPhraseBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hparent: SPPHRASEPROPERTYHANDLE, pproperty: *const SPPHRASEPROPERTY, phnewproperty: *mut SPPHRASEPROPERTYHANDLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpPhraseBuilder_Impl::AddProperties(this, core::mem::transmute_copy(&hparent), core::mem::transmute_copy(&pproperty)) {
                Ok(ok__) => {
                    phnewproperty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddReplacements<Identity: ISpPhraseBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, creplacements: u32, preplacements: *const SPPHRASEREPLACEMENT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpPhraseBuilder_Impl::AddReplacements(this, core::mem::transmute_copy(&creplacements), core::mem::transmute_copy(&preplacements)).into()
        }
        Self {
            base__: ISpPhrase_Vtbl::new::<Identity, OFFSET>(),
            InitFromPhrase: InitFromPhrase::<Identity, OFFSET>,
            InitFromSerializedPhrase: InitFromSerializedPhrase::<Identity, OFFSET>,
            AddElements: AddElements::<Identity, OFFSET>,
            AddRules: AddRules::<Identity, OFFSET>,
            AddProperties: AddProperties::<Identity, OFFSET>,
            AddReplacements: AddReplacements::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpPhraseBuilder as windows_core::Interface>::IID || iid == &<ISpPhrase as windows_core::Interface>::IID
    }
}
pub trait ISpPrivateEngineCallEx_Impl: Sized + windows_core::IUnknownImpl {
    fn CallEngineSynchronize(&self, pinframe: *const core::ffi::c_void, ulinframesize: u32, ppcomemoutframe: *mut *mut core::ffi::c_void, puloutframesize: *mut u32) -> windows_core::Result<()>;
    fn CallEngineImmediate(&self, pinframe: *const core::ffi::c_void, ulinframesize: u32, ppcomemoutframe: *mut *mut core::ffi::c_void, puloutframesize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpPrivateEngineCallEx {}
impl ISpPrivateEngineCallEx_Vtbl {
    pub const fn new<Identity: ISpPrivateEngineCallEx_Impl, const OFFSET: isize>() -> ISpPrivateEngineCallEx_Vtbl {
        unsafe extern "system" fn CallEngineSynchronize<Identity: ISpPrivateEngineCallEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinframe: *const core::ffi::c_void, ulinframesize: u32, ppcomemoutframe: *mut *mut core::ffi::c_void, puloutframesize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpPrivateEngineCallEx_Impl::CallEngineSynchronize(this, core::mem::transmute_copy(&pinframe), core::mem::transmute_copy(&ulinframesize), core::mem::transmute_copy(&ppcomemoutframe), core::mem::transmute_copy(&puloutframesize)).into()
        }
        unsafe extern "system" fn CallEngineImmediate<Identity: ISpPrivateEngineCallEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinframe: *const core::ffi::c_void, ulinframesize: u32, ppcomemoutframe: *mut *mut core::ffi::c_void, puloutframesize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpPrivateEngineCallEx_Impl::CallEngineImmediate(this, core::mem::transmute_copy(&pinframe), core::mem::transmute_copy(&ulinframesize), core::mem::transmute_copy(&ppcomemoutframe), core::mem::transmute_copy(&puloutframesize)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CallEngineSynchronize: CallEngineSynchronize::<Identity, OFFSET>,
            CallEngineImmediate: CallEngineImmediate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpPrivateEngineCallEx as windows_core::Interface>::IID
    }
}
pub trait ISpProperties_Impl: Sized + windows_core::IUnknownImpl {
    fn SetPropertyNum(&self, pname: &windows_core::PCWSTR, lvalue: i32) -> windows_core::Result<()>;
    fn GetPropertyNum(&self, pname: &windows_core::PCWSTR, plvalue: *mut i32) -> windows_core::Result<()>;
    fn SetPropertyString(&self, pname: &windows_core::PCWSTR, pvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPropertyString(&self, pname: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
}
impl windows_core::RuntimeName for ISpProperties {}
impl ISpProperties_Vtbl {
    pub const fn new<Identity: ISpProperties_Impl, const OFFSET: isize>() -> ISpProperties_Vtbl {
        unsafe extern "system" fn SetPropertyNum<Identity: ISpProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: windows_core::PCWSTR, lvalue: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpProperties_Impl::SetPropertyNum(this, core::mem::transmute(&pname), core::mem::transmute_copy(&lvalue)).into()
        }
        unsafe extern "system" fn GetPropertyNum<Identity: ISpProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: windows_core::PCWSTR, plvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpProperties_Impl::GetPropertyNum(this, core::mem::transmute(&pname), core::mem::transmute_copy(&plvalue)).into()
        }
        unsafe extern "system" fn SetPropertyString<Identity: ISpProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: windows_core::PCWSTR, pvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpProperties_Impl::SetPropertyString(this, core::mem::transmute(&pname), core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn GetPropertyString<Identity: ISpProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: windows_core::PCWSTR, ppcomemvalue: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpProperties_Impl::GetPropertyString(this, core::mem::transmute(&pname)) {
                Ok(ok__) => {
                    ppcomemvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPropertyNum: SetPropertyNum::<Identity, OFFSET>,
            GetPropertyNum: GetPropertyNum::<Identity, OFFSET>,
            SetPropertyString: SetPropertyString::<Identity, OFFSET>,
            GetPropertyString: GetPropertyString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpProperties as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait ISpRecoContext_Impl: Sized + ISpEventSource_Impl {
    fn GetRecognizer(&self) -> windows_core::Result<ISpRecognizer>;
    fn CreateGrammar(&self, ullgrammarid: u64) -> windows_core::Result<ISpRecoGrammar>;
    fn GetStatus(&self, pstatus: *mut SPRECOCONTEXTSTATUS) -> windows_core::Result<()>;
    fn GetMaxAlternates(&self, pcalternates: *mut u32) -> windows_core::Result<()>;
    fn SetMaxAlternates(&self, calternates: u32) -> windows_core::Result<()>;
    fn SetAudioOptions(&self, options: SPAUDIOOPTIONS, paudioformatid: *const windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> windows_core::Result<()>;
    fn GetAudioOptions(&self, poptions: *mut SPAUDIOOPTIONS, paudioformatid: *mut windows_core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> windows_core::Result<()>;
    fn DeserializeResult(&self, pserializedresult: *const SPSERIALIZEDRESULT) -> windows_core::Result<ISpRecoResult>;
    fn Bookmark(&self, options: SPBOOKMARKOPTIONS, ullstreamposition: u64, lparamevent: super::super::Foundation::LPARAM) -> windows_core::Result<()>;
    fn SetAdaptationData(&self, padaptationdata: &windows_core::PCWSTR, cch: u32) -> windows_core::Result<()>;
    fn Pause(&self, dwreserved: u32) -> windows_core::Result<()>;
    fn Resume(&self, dwreserved: u32) -> windows_core::Result<()>;
    fn SetVoice(&self, pvoice: Option<&ISpVoice>, fallowformatchanges: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetVoice(&self) -> windows_core::Result<ISpVoice>;
    fn SetVoicePurgeEvent(&self, ulleventinterest: u64) -> windows_core::Result<()>;
    fn GetVoicePurgeEvent(&self, pulleventinterest: *mut u64) -> windows_core::Result<()>;
    fn SetContextState(&self, econtextstate: SPCONTEXTSTATE) -> windows_core::Result<()>;
    fn GetContextState(&self, pecontextstate: *mut SPCONTEXTSTATE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::RuntimeName for ISpRecoContext {}
#[cfg(feature = "Win32_Media_Audio")]
impl ISpRecoContext_Vtbl {
    pub const fn new<Identity: ISpRecoContext_Impl, const OFFSET: isize>() -> ISpRecoContext_Vtbl {
        unsafe extern "system" fn GetRecognizer<Identity: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprecognizer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpRecoContext_Impl::GetRecognizer(this) {
                Ok(ok__) => {
                    pprecognizer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGrammar<Identity: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullgrammarid: u64, ppgrammar: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpRecoContext_Impl::CreateGrammar(this, core::mem::transmute_copy(&ullgrammarid)) {
                Ok(ok__) => {
                    ppgrammar.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut SPRECOCONTEXTSTATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoContext_Impl::GetStatus(this, core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn GetMaxAlternates<Identity: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcalternates: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoContext_Impl::GetMaxAlternates(this, core::mem::transmute_copy(&pcalternates)).into()
        }
        unsafe extern "system" fn SetMaxAlternates<Identity: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, calternates: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoContext_Impl::SetMaxAlternates(this, core::mem::transmute_copy(&calternates)).into()
        }
        unsafe extern "system" fn SetAudioOptions<Identity: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: SPAUDIOOPTIONS, paudioformatid: *const windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoContext_Impl::SetAudioOptions(this, core::mem::transmute_copy(&options), core::mem::transmute_copy(&paudioformatid), core::mem::transmute_copy(&pwaveformatex)).into()
        }
        unsafe extern "system" fn GetAudioOptions<Identity: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poptions: *mut SPAUDIOOPTIONS, paudioformatid: *mut windows_core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoContext_Impl::GetAudioOptions(this, core::mem::transmute_copy(&poptions), core::mem::transmute_copy(&paudioformatid), core::mem::transmute_copy(&ppcomemwfex)).into()
        }
        unsafe extern "system" fn DeserializeResult<Identity: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserializedresult: *const SPSERIALIZEDRESULT, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpRecoContext_Impl::DeserializeResult(this, core::mem::transmute_copy(&pserializedresult)) {
                Ok(ok__) => {
                    ppresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bookmark<Identity: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: SPBOOKMARKOPTIONS, ullstreamposition: u64, lparamevent: super::super::Foundation::LPARAM) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoContext_Impl::Bookmark(this, core::mem::transmute_copy(&options), core::mem::transmute_copy(&ullstreamposition), core::mem::transmute_copy(&lparamevent)).into()
        }
        unsafe extern "system" fn SetAdaptationData<Identity: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, padaptationdata: windows_core::PCWSTR, cch: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoContext_Impl::SetAdaptationData(this, core::mem::transmute(&padaptationdata), core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn Pause<Identity: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoContext_Impl::Pause(this, core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn Resume<Identity: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoContext_Impl::Resume(this, core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SetVoice<Identity: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvoice: *mut core::ffi::c_void, fallowformatchanges: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoContext_Impl::SetVoice(this, windows_core::from_raw_borrowed(&pvoice), core::mem::transmute_copy(&fallowformatchanges)).into()
        }
        unsafe extern "system" fn GetVoice<Identity: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvoice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpRecoContext_Impl::GetVoice(this) {
                Ok(ok__) => {
                    ppvoice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVoicePurgeEvent<Identity: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulleventinterest: u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoContext_Impl::SetVoicePurgeEvent(this, core::mem::transmute_copy(&ulleventinterest)).into()
        }
        unsafe extern "system" fn GetVoicePurgeEvent<Identity: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pulleventinterest: *mut u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoContext_Impl::GetVoicePurgeEvent(this, core::mem::transmute_copy(&pulleventinterest)).into()
        }
        unsafe extern "system" fn SetContextState<Identity: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, econtextstate: SPCONTEXTSTATE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoContext_Impl::SetContextState(this, core::mem::transmute_copy(&econtextstate)).into()
        }
        unsafe extern "system" fn GetContextState<Identity: ISpRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pecontextstate: *mut SPCONTEXTSTATE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoContext_Impl::GetContextState(this, core::mem::transmute_copy(&pecontextstate)).into()
        }
        Self {
            base__: ISpEventSource_Vtbl::new::<Identity, OFFSET>(),
            GetRecognizer: GetRecognizer::<Identity, OFFSET>,
            CreateGrammar: CreateGrammar::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetMaxAlternates: GetMaxAlternates::<Identity, OFFSET>,
            SetMaxAlternates: SetMaxAlternates::<Identity, OFFSET>,
            SetAudioOptions: SetAudioOptions::<Identity, OFFSET>,
            GetAudioOptions: GetAudioOptions::<Identity, OFFSET>,
            DeserializeResult: DeserializeResult::<Identity, OFFSET>,
            Bookmark: Bookmark::<Identity, OFFSET>,
            SetAdaptationData: SetAdaptationData::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            SetVoice: SetVoice::<Identity, OFFSET>,
            GetVoice: GetVoice::<Identity, OFFSET>,
            SetVoicePurgeEvent: SetVoicePurgeEvent::<Identity, OFFSET>,
            GetVoicePurgeEvent: GetVoicePurgeEvent::<Identity, OFFSET>,
            SetContextState: SetContextState::<Identity, OFFSET>,
            GetContextState: GetContextState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpRecoContext as windows_core::Interface>::IID || iid == &<ISpNotifySource as windows_core::Interface>::IID || iid == &<ISpEventSource as windows_core::Interface>::IID
    }
}
pub trait ISpRecoContext2_Impl: Sized + windows_core::IUnknownImpl {
    fn SetGrammarOptions(&self, egrammaroptions: u32) -> windows_core::Result<()>;
    fn GetGrammarOptions(&self, pegrammaroptions: *mut u32) -> windows_core::Result<()>;
    fn SetAdaptationData2(&self, padaptationdata: &windows_core::PCWSTR, cch: u32, ptopicname: &windows_core::PCWSTR, eadaptationsettings: u32, erelevance: SPADAPTATIONRELEVANCE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpRecoContext2 {}
impl ISpRecoContext2_Vtbl {
    pub const fn new<Identity: ISpRecoContext2_Impl, const OFFSET: isize>() -> ISpRecoContext2_Vtbl {
        unsafe extern "system" fn SetGrammarOptions<Identity: ISpRecoContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, egrammaroptions: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoContext2_Impl::SetGrammarOptions(this, core::mem::transmute_copy(&egrammaroptions)).into()
        }
        unsafe extern "system" fn GetGrammarOptions<Identity: ISpRecoContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pegrammaroptions: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoContext2_Impl::GetGrammarOptions(this, core::mem::transmute_copy(&pegrammaroptions)).into()
        }
        unsafe extern "system" fn SetAdaptationData2<Identity: ISpRecoContext2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, padaptationdata: windows_core::PCWSTR, cch: u32, ptopicname: windows_core::PCWSTR, eadaptationsettings: u32, erelevance: SPADAPTATIONRELEVANCE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoContext2_Impl::SetAdaptationData2(this, core::mem::transmute(&padaptationdata), core::mem::transmute_copy(&cch), core::mem::transmute(&ptopicname), core::mem::transmute_copy(&eadaptationsettings), core::mem::transmute_copy(&erelevance)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGrammarOptions: SetGrammarOptions::<Identity, OFFSET>,
            GetGrammarOptions: GetGrammarOptions::<Identity, OFFSET>,
            SetAdaptationData2: SetAdaptationData2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpRecoContext2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpRecoGrammar_Impl: Sized + ISpGrammarBuilder_Impl {
    fn GetGrammarId(&self, pullgrammarid: *mut u64) -> windows_core::Result<()>;
    fn GetRecoContext(&self) -> windows_core::Result<ISpRecoContext>;
    fn LoadCmdFromFile(&self, pszfilename: &windows_core::PCWSTR, options: SPLOADOPTIONS) -> windows_core::Result<()>;
    fn LoadCmdFromObject(&self, rcid: *const windows_core::GUID, pszgrammarname: &windows_core::PCWSTR, options: SPLOADOPTIONS) -> windows_core::Result<()>;
    fn LoadCmdFromResource(&self, hmodule: super::super::Foundation::HMODULE, pszresourcename: &windows_core::PCWSTR, pszresourcetype: &windows_core::PCWSTR, wlanguage: u16, options: SPLOADOPTIONS) -> windows_core::Result<()>;
    fn LoadCmdFromMemory(&self, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS) -> windows_core::Result<()>;
    fn LoadCmdFromProprietaryGrammar(&self, rguidparam: *const windows_core::GUID, pszstringparam: &windows_core::PCWSTR, pvdataprarm: *const core::ffi::c_void, cbdatasize: u32, options: SPLOADOPTIONS) -> windows_core::Result<()>;
    fn SetRuleState(&self, pszname: &windows_core::PCWSTR, preserved: *mut core::ffi::c_void, newstate: SPRULESTATE) -> windows_core::Result<()>;
    fn SetRuleIdState(&self, ulruleid: u32, newstate: SPRULESTATE) -> windows_core::Result<()>;
    fn LoadDictation(&self, psztopicname: &windows_core::PCWSTR, options: SPLOADOPTIONS) -> windows_core::Result<()>;
    fn UnloadDictation(&self) -> windows_core::Result<()>;
    fn SetDictationState(&self, newstate: SPRULESTATE) -> windows_core::Result<()>;
    fn SetWordSequenceData(&self, ptext: &windows_core::PCWSTR, cchtext: u32, pinfo: *const SPTEXTSELECTIONINFO) -> windows_core::Result<()>;
    fn SetTextSelection(&self, pinfo: *const SPTEXTSELECTIONINFO) -> windows_core::Result<()>;
    fn IsPronounceable(&self, pszword: &windows_core::PCWSTR, pwordpronounceable: *mut SPWORDPRONOUNCEABLE) -> windows_core::Result<()>;
    fn SetGrammarState(&self, egrammarstate: SPGRAMMARSTATE) -> windows_core::Result<()>;
    fn SaveCmd(&self, pstream: Option<&super::super::System::Com::IStream>, ppszcomemerrortext: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetGrammarState(&self, pegrammarstate: *mut SPGRAMMARSTATE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpRecoGrammar {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpRecoGrammar_Vtbl {
    pub const fn new<Identity: ISpRecoGrammar_Impl, const OFFSET: isize>() -> ISpRecoGrammar_Vtbl {
        unsafe extern "system" fn GetGrammarId<Identity: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pullgrammarid: *mut u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar_Impl::GetGrammarId(this, core::mem::transmute_copy(&pullgrammarid)).into()
        }
        unsafe extern "system" fn GetRecoContext<Identity: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprecoctxt: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpRecoGrammar_Impl::GetRecoContext(this) {
                Ok(ok__) => {
                    pprecoctxt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadCmdFromFile<Identity: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilename: windows_core::PCWSTR, options: SPLOADOPTIONS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar_Impl::LoadCmdFromFile(this, core::mem::transmute(&pszfilename), core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn LoadCmdFromObject<Identity: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rcid: *const windows_core::GUID, pszgrammarname: windows_core::PCWSTR, options: SPLOADOPTIONS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar_Impl::LoadCmdFromObject(this, core::mem::transmute_copy(&rcid), core::mem::transmute(&pszgrammarname), core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn LoadCmdFromResource<Identity: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmodule: super::super::Foundation::HMODULE, pszresourcename: windows_core::PCWSTR, pszresourcetype: windows_core::PCWSTR, wlanguage: u16, options: SPLOADOPTIONS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar_Impl::LoadCmdFromResource(this, core::mem::transmute_copy(&hmodule), core::mem::transmute(&pszresourcename), core::mem::transmute(&pszresourcetype), core::mem::transmute_copy(&wlanguage), core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn LoadCmdFromMemory<Identity: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar_Impl::LoadCmdFromMemory(this, core::mem::transmute_copy(&pgrammar), core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn LoadCmdFromProprietaryGrammar<Identity: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidparam: *const windows_core::GUID, pszstringparam: windows_core::PCWSTR, pvdataprarm: *const core::ffi::c_void, cbdatasize: u32, options: SPLOADOPTIONS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar_Impl::LoadCmdFromProprietaryGrammar(this, core::mem::transmute_copy(&rguidparam), core::mem::transmute(&pszstringparam), core::mem::transmute_copy(&pvdataprarm), core::mem::transmute_copy(&cbdatasize), core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn SetRuleState<Identity: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR, preserved: *mut core::ffi::c_void, newstate: SPRULESTATE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar_Impl::SetRuleState(this, core::mem::transmute(&pszname), core::mem::transmute_copy(&preserved), core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn SetRuleIdState<Identity: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulruleid: u32, newstate: SPRULESTATE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar_Impl::SetRuleIdState(this, core::mem::transmute_copy(&ulruleid), core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn LoadDictation<Identity: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztopicname: windows_core::PCWSTR, options: SPLOADOPTIONS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar_Impl::LoadDictation(this, core::mem::transmute(&psztopicname), core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn UnloadDictation<Identity: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar_Impl::UnloadDictation(this).into()
        }
        unsafe extern "system" fn SetDictationState<Identity: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstate: SPRULESTATE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar_Impl::SetDictationState(this, core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn SetWordSequenceData<Identity: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptext: windows_core::PCWSTR, cchtext: u32, pinfo: *const SPTEXTSELECTIONINFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar_Impl::SetWordSequenceData(this, core::mem::transmute(&ptext), core::mem::transmute_copy(&cchtext), core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn SetTextSelection<Identity: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const SPTEXTSELECTIONINFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar_Impl::SetTextSelection(this, core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn IsPronounceable<Identity: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszword: windows_core::PCWSTR, pwordpronounceable: *mut SPWORDPRONOUNCEABLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar_Impl::IsPronounceable(this, core::mem::transmute(&pszword), core::mem::transmute_copy(&pwordpronounceable)).into()
        }
        unsafe extern "system" fn SetGrammarState<Identity: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, egrammarstate: SPGRAMMARSTATE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar_Impl::SetGrammarState(this, core::mem::transmute_copy(&egrammarstate)).into()
        }
        unsafe extern "system" fn SaveCmd<Identity: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, ppszcomemerrortext: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar_Impl::SaveCmd(this, windows_core::from_raw_borrowed(&pstream), core::mem::transmute_copy(&ppszcomemerrortext)).into()
        }
        unsafe extern "system" fn GetGrammarState<Identity: ISpRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pegrammarstate: *mut SPGRAMMARSTATE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar_Impl::GetGrammarState(this, core::mem::transmute_copy(&pegrammarstate)).into()
        }
        Self {
            base__: ISpGrammarBuilder_Vtbl::new::<Identity, OFFSET>(),
            GetGrammarId: GetGrammarId::<Identity, OFFSET>,
            GetRecoContext: GetRecoContext::<Identity, OFFSET>,
            LoadCmdFromFile: LoadCmdFromFile::<Identity, OFFSET>,
            LoadCmdFromObject: LoadCmdFromObject::<Identity, OFFSET>,
            LoadCmdFromResource: LoadCmdFromResource::<Identity, OFFSET>,
            LoadCmdFromMemory: LoadCmdFromMemory::<Identity, OFFSET>,
            LoadCmdFromProprietaryGrammar: LoadCmdFromProprietaryGrammar::<Identity, OFFSET>,
            SetRuleState: SetRuleState::<Identity, OFFSET>,
            SetRuleIdState: SetRuleIdState::<Identity, OFFSET>,
            LoadDictation: LoadDictation::<Identity, OFFSET>,
            UnloadDictation: UnloadDictation::<Identity, OFFSET>,
            SetDictationState: SetDictationState::<Identity, OFFSET>,
            SetWordSequenceData: SetWordSequenceData::<Identity, OFFSET>,
            SetTextSelection: SetTextSelection::<Identity, OFFSET>,
            IsPronounceable: IsPronounceable::<Identity, OFFSET>,
            SetGrammarState: SetGrammarState::<Identity, OFFSET>,
            SaveCmd: SaveCmd::<Identity, OFFSET>,
            GetGrammarState: GetGrammarState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpRecoGrammar as windows_core::Interface>::IID || iid == &<ISpGrammarBuilder as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_Urlmon")]
pub trait ISpRecoGrammar2_Impl: Sized + windows_core::IUnknownImpl {
    fn GetRules(&self, ppcomemrules: *mut *mut SPRULE, punumrules: *mut u32) -> windows_core::Result<()>;
    fn LoadCmdFromFile2(&self, pszfilename: &windows_core::PCWSTR, options: SPLOADOPTIONS, pszsharinguri: &windows_core::PCWSTR, pszbaseuri: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn LoadCmdFromMemory2(&self, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS, pszsharinguri: &windows_core::PCWSTR, pszbaseuri: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetRulePriority(&self, pszrulename: &windows_core::PCWSTR, ulruleid: u32, nrulepriority: i32) -> windows_core::Result<()>;
    fn SetRuleWeight(&self, pszrulename: &windows_core::PCWSTR, ulruleid: u32, flweight: f32) -> windows_core::Result<()>;
    fn SetDictationWeight(&self, flweight: f32) -> windows_core::Result<()>;
    fn SetGrammarLoader(&self, ploader: Option<&ISpeechResourceLoader>) -> windows_core::Result<()>;
    fn SetSMLSecurityManager(&self, psmlsecuritymanager: Option<&super::super::System::Com::Urlmon::IInternetSecurityManager>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_Urlmon")]
impl windows_core::RuntimeName for ISpRecoGrammar2 {}
#[cfg(feature = "Win32_System_Com_Urlmon")]
impl ISpRecoGrammar2_Vtbl {
    pub const fn new<Identity: ISpRecoGrammar2_Impl, const OFFSET: isize>() -> ISpRecoGrammar2_Vtbl {
        unsafe extern "system" fn GetRules<Identity: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcomemrules: *mut *mut SPRULE, punumrules: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar2_Impl::GetRules(this, core::mem::transmute_copy(&ppcomemrules), core::mem::transmute_copy(&punumrules)).into()
        }
        unsafe extern "system" fn LoadCmdFromFile2<Identity: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilename: windows_core::PCWSTR, options: SPLOADOPTIONS, pszsharinguri: windows_core::PCWSTR, pszbaseuri: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar2_Impl::LoadCmdFromFile2(this, core::mem::transmute(&pszfilename), core::mem::transmute_copy(&options), core::mem::transmute(&pszsharinguri), core::mem::transmute(&pszbaseuri)).into()
        }
        unsafe extern "system" fn LoadCmdFromMemory2<Identity: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgrammar: *const SPBINARYGRAMMAR, options: SPLOADOPTIONS, pszsharinguri: windows_core::PCWSTR, pszbaseuri: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar2_Impl::LoadCmdFromMemory2(this, core::mem::transmute_copy(&pgrammar), core::mem::transmute_copy(&options), core::mem::transmute(&pszsharinguri), core::mem::transmute(&pszbaseuri)).into()
        }
        unsafe extern "system" fn SetRulePriority<Identity: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszrulename: windows_core::PCWSTR, ulruleid: u32, nrulepriority: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar2_Impl::SetRulePriority(this, core::mem::transmute(&pszrulename), core::mem::transmute_copy(&ulruleid), core::mem::transmute_copy(&nrulepriority)).into()
        }
        unsafe extern "system" fn SetRuleWeight<Identity: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszrulename: windows_core::PCWSTR, ulruleid: u32, flweight: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar2_Impl::SetRuleWeight(this, core::mem::transmute(&pszrulename), core::mem::transmute_copy(&ulruleid), core::mem::transmute_copy(&flweight)).into()
        }
        unsafe extern "system" fn SetDictationWeight<Identity: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flweight: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar2_Impl::SetDictationWeight(this, core::mem::transmute_copy(&flweight)).into()
        }
        unsafe extern "system" fn SetGrammarLoader<Identity: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ploader: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar2_Impl::SetGrammarLoader(this, windows_core::from_raw_borrowed(&ploader)).into()
        }
        unsafe extern "system" fn SetSMLSecurityManager<Identity: ISpRecoGrammar2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psmlsecuritymanager: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoGrammar2_Impl::SetSMLSecurityManager(this, windows_core::from_raw_borrowed(&psmlsecuritymanager)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRules: GetRules::<Identity, OFFSET>,
            LoadCmdFromFile2: LoadCmdFromFile2::<Identity, OFFSET>,
            LoadCmdFromMemory2: LoadCmdFromMemory2::<Identity, OFFSET>,
            SetRulePriority: SetRulePriority::<Identity, OFFSET>,
            SetRuleWeight: SetRuleWeight::<Identity, OFFSET>,
            SetDictationWeight: SetDictationWeight::<Identity, OFFSET>,
            SetGrammarLoader: SetGrammarLoader::<Identity, OFFSET>,
            SetSMLSecurityManager: SetSMLSecurityManager::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpRecoGrammar2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpRecoResult_Impl: Sized + ISpPhrase_Impl {
    fn GetResultTimes(&self, ptimes: *mut SPRECORESULTTIMES) -> windows_core::Result<()>;
    fn GetAlternates(&self, ulstartelement: u32, celements: u32, ulrequestcount: u32, ppphrases: *mut Option<ISpPhraseAlt>, pcphrasesreturned: *mut u32) -> windows_core::Result<()>;
    fn GetAudio(&self, ulstartelement: u32, celements: u32) -> windows_core::Result<ISpStreamFormat>;
    fn SpeakAudio(&self, ulstartelement: u32, celements: u32, dwflags: u32, pulstreamnumber: *mut u32) -> windows_core::Result<()>;
    fn Serialize(&self, ppcomemserializedresult: *mut *mut SPSERIALIZEDRESULT) -> windows_core::Result<()>;
    fn ScaleAudio(&self, paudioformatid: *const windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> windows_core::Result<()>;
    fn GetRecoContext(&self) -> windows_core::Result<ISpRecoContext>;
}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpRecoResult {}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpRecoResult_Vtbl {
    pub const fn new<Identity: ISpRecoResult_Impl, const OFFSET: isize>() -> ISpRecoResult_Vtbl {
        unsafe extern "system" fn GetResultTimes<Identity: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptimes: *mut SPRECORESULTTIMES) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoResult_Impl::GetResultTimes(this, core::mem::transmute_copy(&ptimes)).into()
        }
        unsafe extern "system" fn GetAlternates<Identity: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulstartelement: u32, celements: u32, ulrequestcount: u32, ppphrases: *mut *mut core::ffi::c_void, pcphrasesreturned: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoResult_Impl::GetAlternates(this, core::mem::transmute_copy(&ulstartelement), core::mem::transmute_copy(&celements), core::mem::transmute_copy(&ulrequestcount), core::mem::transmute_copy(&ppphrases), core::mem::transmute_copy(&pcphrasesreturned)).into()
        }
        unsafe extern "system" fn GetAudio<Identity: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulstartelement: u32, celements: u32, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpRecoResult_Impl::GetAudio(this, core::mem::transmute_copy(&ulstartelement), core::mem::transmute_copy(&celements)) {
                Ok(ok__) => {
                    ppstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakAudio<Identity: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulstartelement: u32, celements: u32, dwflags: u32, pulstreamnumber: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoResult_Impl::SpeakAudio(this, core::mem::transmute_copy(&ulstartelement), core::mem::transmute_copy(&celements), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pulstreamnumber)).into()
        }
        unsafe extern "system" fn Serialize<Identity: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcomemserializedresult: *mut *mut SPSERIALIZEDRESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoResult_Impl::Serialize(this, core::mem::transmute_copy(&ppcomemserializedresult)).into()
        }
        unsafe extern "system" fn ScaleAudio<Identity: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paudioformatid: *const windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoResult_Impl::ScaleAudio(this, core::mem::transmute_copy(&paudioformatid), core::mem::transmute_copy(&pwaveformatex)).into()
        }
        unsafe extern "system" fn GetRecoContext<Identity: ISpRecoResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprecocontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpRecoResult_Impl::GetRecoContext(this) {
                Ok(ok__) => {
                    pprecocontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISpPhrase_Vtbl::new::<Identity, OFFSET>(),
            GetResultTimes: GetResultTimes::<Identity, OFFSET>,
            GetAlternates: GetAlternates::<Identity, OFFSET>,
            GetAudio: GetAudio::<Identity, OFFSET>,
            SpeakAudio: SpeakAudio::<Identity, OFFSET>,
            Serialize: Serialize::<Identity, OFFSET>,
            ScaleAudio: ScaleAudio::<Identity, OFFSET>,
            GetRecoContext: GetRecoContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpRecoResult as windows_core::Interface>::IID || iid == &<ISpPhrase as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpRecoResult2_Impl: Sized + ISpRecoResult_Impl {
    fn CommitAlternate(&self, pphrasealt: Option<&ISpPhraseAlt>) -> windows_core::Result<ISpRecoResult>;
    fn CommitText(&self, ulstartelement: u32, celements: u32, pszcorrecteddata: &windows_core::PCWSTR, ecommitflags: u32) -> windows_core::Result<()>;
    fn SetTextFeedback(&self, pszfeedback: &windows_core::PCWSTR, fsuccessful: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpRecoResult2 {}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpRecoResult2_Vtbl {
    pub const fn new<Identity: ISpRecoResult2_Impl, const OFFSET: isize>() -> ISpRecoResult2_Vtbl {
        unsafe extern "system" fn CommitAlternate<Identity: ISpRecoResult2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphrasealt: *mut core::ffi::c_void, ppnewresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpRecoResult2_Impl::CommitAlternate(this, windows_core::from_raw_borrowed(&pphrasealt)) {
                Ok(ok__) => {
                    ppnewresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitText<Identity: ISpRecoResult2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulstartelement: u32, celements: u32, pszcorrecteddata: windows_core::PCWSTR, ecommitflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoResult2_Impl::CommitText(this, core::mem::transmute_copy(&ulstartelement), core::mem::transmute_copy(&celements), core::mem::transmute(&pszcorrecteddata), core::mem::transmute_copy(&ecommitflags)).into()
        }
        unsafe extern "system" fn SetTextFeedback<Identity: ISpRecoResult2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfeedback: windows_core::PCWSTR, fsuccessful: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecoResult2_Impl::SetTextFeedback(this, core::mem::transmute(&pszfeedback), core::mem::transmute_copy(&fsuccessful)).into()
        }
        Self {
            base__: ISpRecoResult_Vtbl::new::<Identity, OFFSET>(),
            CommitAlternate: CommitAlternate::<Identity, OFFSET>,
            CommitText: CommitText::<Identity, OFFSET>,
            SetTextFeedback: SetTextFeedback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpRecoResult2 as windows_core::Interface>::IID || iid == &<ISpPhrase as windows_core::Interface>::IID || iid == &<ISpRecoResult as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
pub trait ISpRecognizer_Impl: Sized + ISpProperties_Impl {
    fn SetRecognizer(&self, precognizer: Option<&ISpObjectToken>) -> windows_core::Result<()>;
    fn GetRecognizer(&self) -> windows_core::Result<ISpObjectToken>;
    fn SetInput(&self, punkinput: Option<&windows_core::IUnknown>, fallowformatchanges: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetInputObjectToken(&self) -> windows_core::Result<ISpObjectToken>;
    fn GetInputStream(&self) -> windows_core::Result<ISpStreamFormat>;
    fn CreateRecoContext(&self) -> windows_core::Result<ISpRecoContext>;
    fn GetRecoProfile(&self) -> windows_core::Result<ISpObjectToken>;
    fn SetRecoProfile(&self, ptoken: Option<&ISpObjectToken>) -> windows_core::Result<()>;
    fn IsSharedInstance(&self) -> windows_core::Result<()>;
    fn GetRecoState(&self, pstate: *mut SPRECOSTATE) -> windows_core::Result<()>;
    fn SetRecoState(&self, newstate: SPRECOSTATE) -> windows_core::Result<()>;
    fn GetStatus(&self, pstatus: *mut SPRECOGNIZERSTATUS) -> windows_core::Result<()>;
    fn GetFormat(&self, waveformattype: SPSTREAMFORMATTYPE, pformatid: *mut windows_core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> windows_core::Result<()>;
    fn IsUISupported(&self, psztypeofui: &windows_core::PCWSTR, pvextradata: *mut core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn DisplayUI(&self, hwndparent: super::super::Foundation::HWND, psztitle: &windows_core::PCWSTR, psztypeofui: &windows_core::PCWSTR, pvextradata: *mut core::ffi::c_void, cbextradata: u32) -> windows_core::Result<()>;
    fn EmulateRecognition(&self, pphrase: Option<&ISpPhrase>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ISpRecognizer {}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ISpRecognizer_Vtbl {
    pub const fn new<Identity: ISpRecognizer_Impl, const OFFSET: isize>() -> ISpRecognizer_Vtbl {
        unsafe extern "system" fn SetRecognizer<Identity: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, precognizer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecognizer_Impl::SetRecognizer(this, windows_core::from_raw_borrowed(&precognizer)).into()
        }
        unsafe extern "system" fn GetRecognizer<Identity: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprecognizer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpRecognizer_Impl::GetRecognizer(this) {
                Ok(ok__) => {
                    pprecognizer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInput<Identity: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkinput: *mut core::ffi::c_void, fallowformatchanges: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecognizer_Impl::SetInput(this, windows_core::from_raw_borrowed(&punkinput), core::mem::transmute_copy(&fallowformatchanges)).into()
        }
        unsafe extern "system" fn GetInputObjectToken<Identity: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptoken: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpRecognizer_Impl::GetInputObjectToken(this) {
                Ok(ok__) => {
                    pptoken.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputStream<Identity: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpRecognizer_Impl::GetInputStream(this) {
                Ok(ok__) => {
                    ppstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRecoContext<Identity: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnewctxt: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpRecognizer_Impl::CreateRecoContext(this) {
                Ok(ok__) => {
                    ppnewctxt.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecoProfile<Identity: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptoken: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpRecognizer_Impl::GetRecoProfile(this) {
                Ok(ok__) => {
                    pptoken.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecoProfile<Identity: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptoken: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecognizer_Impl::SetRecoProfile(this, windows_core::from_raw_borrowed(&ptoken)).into()
        }
        unsafe extern "system" fn IsSharedInstance<Identity: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecognizer_Impl::IsSharedInstance(this).into()
        }
        unsafe extern "system" fn GetRecoState<Identity: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut SPRECOSTATE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecognizer_Impl::GetRecoState(this, core::mem::transmute_copy(&pstate)).into()
        }
        unsafe extern "system" fn SetRecoState<Identity: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstate: SPRECOSTATE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecognizer_Impl::SetRecoState(this, core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut SPRECOGNIZERSTATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecognizer_Impl::GetStatus(this, core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn GetFormat<Identity: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, waveformattype: SPSTREAMFORMATTYPE, pformatid: *mut windows_core::GUID, ppcomemwfex: *mut *mut super::Audio::WAVEFORMATEX) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecognizer_Impl::GetFormat(this, core::mem::transmute_copy(&waveformattype), core::mem::transmute_copy(&pformatid), core::mem::transmute_copy(&ppcomemwfex)).into()
        }
        unsafe extern "system" fn IsUISupported<Identity: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztypeofui: windows_core::PCWSTR, pvextradata: *mut core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecognizer_Impl::IsUISupported(this, core::mem::transmute(&psztypeofui), core::mem::transmute_copy(&pvextradata), core::mem::transmute_copy(&cbextradata), core::mem::transmute_copy(&pfsupported)).into()
        }
        unsafe extern "system" fn DisplayUI<Identity: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, psztitle: windows_core::PCWSTR, psztypeofui: windows_core::PCWSTR, pvextradata: *mut core::ffi::c_void, cbextradata: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecognizer_Impl::DisplayUI(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute(&psztitle), core::mem::transmute(&psztypeofui), core::mem::transmute_copy(&pvextradata), core::mem::transmute_copy(&cbextradata)).into()
        }
        unsafe extern "system" fn EmulateRecognition<Identity: ISpRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphrase: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecognizer_Impl::EmulateRecognition(this, windows_core::from_raw_borrowed(&pphrase)).into()
        }
        Self {
            base__: ISpProperties_Vtbl::new::<Identity, OFFSET>(),
            SetRecognizer: SetRecognizer::<Identity, OFFSET>,
            GetRecognizer: GetRecognizer::<Identity, OFFSET>,
            SetInput: SetInput::<Identity, OFFSET>,
            GetInputObjectToken: GetInputObjectToken::<Identity, OFFSET>,
            GetInputStream: GetInputStream::<Identity, OFFSET>,
            CreateRecoContext: CreateRecoContext::<Identity, OFFSET>,
            GetRecoProfile: GetRecoProfile::<Identity, OFFSET>,
            SetRecoProfile: SetRecoProfile::<Identity, OFFSET>,
            IsSharedInstance: IsSharedInstance::<Identity, OFFSET>,
            GetRecoState: GetRecoState::<Identity, OFFSET>,
            SetRecoState: SetRecoState::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetFormat: GetFormat::<Identity, OFFSET>,
            IsUISupported: IsUISupported::<Identity, OFFSET>,
            DisplayUI: DisplayUI::<Identity, OFFSET>,
            EmulateRecognition: EmulateRecognition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpRecognizer as windows_core::Interface>::IID || iid == &<ISpProperties as windows_core::Interface>::IID
    }
}
pub trait ISpRecognizer2_Impl: Sized + windows_core::IUnknownImpl {
    fn EmulateRecognitionEx(&self, pphrase: Option<&ISpPhrase>, dwcompareflags: u32) -> windows_core::Result<()>;
    fn SetTrainingState(&self, fdoingtraining: super::super::Foundation::BOOL, fadaptfromtrainingdata: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ResetAcousticModelAdaptation(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpRecognizer2 {}
impl ISpRecognizer2_Vtbl {
    pub const fn new<Identity: ISpRecognizer2_Impl, const OFFSET: isize>() -> ISpRecognizer2_Vtbl {
        unsafe extern "system" fn EmulateRecognitionEx<Identity: ISpRecognizer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphrase: *mut core::ffi::c_void, dwcompareflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecognizer2_Impl::EmulateRecognitionEx(this, windows_core::from_raw_borrowed(&pphrase), core::mem::transmute_copy(&dwcompareflags)).into()
        }
        unsafe extern "system" fn SetTrainingState<Identity: ISpRecognizer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdoingtraining: super::super::Foundation::BOOL, fadaptfromtrainingdata: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecognizer2_Impl::SetTrainingState(this, core::mem::transmute_copy(&fdoingtraining), core::mem::transmute_copy(&fadaptfromtrainingdata)).into()
        }
        unsafe extern "system" fn ResetAcousticModelAdaptation<Identity: ISpRecognizer2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRecognizer2_Impl::ResetAcousticModelAdaptation(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EmulateRecognitionEx: EmulateRecognitionEx::<Identity, OFFSET>,
            SetTrainingState: SetTrainingState::<Identity, OFFSET>,
            ResetAcousticModelAdaptation: ResetAcousticModelAdaptation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpRecognizer2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Registry")]
pub trait ISpRegDataKey_Impl: Sized + ISpDataKey_Impl {
    fn SetKey(&self, hkey: super::super::System::Registry::HKEY, freadonly: super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::RuntimeName for ISpRegDataKey {}
#[cfg(feature = "Win32_System_Registry")]
impl ISpRegDataKey_Vtbl {
    pub const fn new<Identity: ISpRegDataKey_Impl, const OFFSET: isize>() -> ISpRegDataKey_Vtbl {
        unsafe extern "system" fn SetKey<Identity: ISpRegDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hkey: super::super::System::Registry::HKEY, freadonly: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpRegDataKey_Impl::SetKey(this, core::mem::transmute_copy(&hkey), core::mem::transmute_copy(&freadonly)).into()
        }
        Self { base__: ISpDataKey_Vtbl::new::<Identity, OFFSET>(), SetKey: SetKey::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpRegDataKey as windows_core::Interface>::IID || iid == &<ISpDataKey as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpResourceManager_Impl: Sized + super::super::System::Com::IServiceProvider_Impl {
    fn SetObject(&self, guidserviceid: *const windows_core::GUID, punkobject: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetObject(&self, guidserviceid: *const windows_core::GUID, objectclsid: *const windows_core::GUID, objectiid: *const windows_core::GUID, freleasewhenlastexternalrefreleased: super::super::Foundation::BOOL, ppobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISpResourceManager {}
#[cfg(feature = "Win32_System_Com")]
impl ISpResourceManager_Vtbl {
    pub const fn new<Identity: ISpResourceManager_Impl, const OFFSET: isize>() -> ISpResourceManager_Vtbl {
        unsafe extern "system" fn SetObject<Identity: ISpResourceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidserviceid: *const windows_core::GUID, punkobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpResourceManager_Impl::SetObject(this, core::mem::transmute_copy(&guidserviceid), windows_core::from_raw_borrowed(&punkobject)).into()
        }
        unsafe extern "system" fn GetObject<Identity: ISpResourceManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidserviceid: *const windows_core::GUID, objectclsid: *const windows_core::GUID, objectiid: *const windows_core::GUID, freleasewhenlastexternalrefreleased: super::super::Foundation::BOOL, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpResourceManager_Impl::GetObject(this, core::mem::transmute_copy(&guidserviceid), core::mem::transmute_copy(&objectclsid), core::mem::transmute_copy(&objectiid), core::mem::transmute_copy(&freleasewhenlastexternalrefreleased), core::mem::transmute_copy(&ppobject)).into()
        }
        Self {
            base__: super::super::System::Com::IServiceProvider_Vtbl::new::<Identity, OFFSET>(),
            SetObject: SetObject::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpResourceManager as windows_core::Interface>::IID || iid == &<super::super::System::Com::IServiceProvider as windows_core::Interface>::IID
    }
}
pub trait ISpSRAlternates_Impl: Sized + windows_core::IUnknownImpl {
    fn GetAlternates(&self, paltrequest: *const SPPHRASEALTREQUEST, ppalts: *mut *mut SPPHRASEALT, pcalts: *mut u32) -> windows_core::Result<()>;
    fn Commit(&self, paltrequest: *const SPPHRASEALTREQUEST, palt: *const SPPHRASEALT, ppvresultextra: *mut *mut core::ffi::c_void, pcbresultextra: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpSRAlternates {}
impl ISpSRAlternates_Vtbl {
    pub const fn new<Identity: ISpSRAlternates_Impl, const OFFSET: isize>() -> ISpSRAlternates_Vtbl {
        unsafe extern "system" fn GetAlternates<Identity: ISpSRAlternates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paltrequest: *const SPPHRASEALTREQUEST, ppalts: *mut *mut SPPHRASEALT, pcalts: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSRAlternates_Impl::GetAlternates(this, core::mem::transmute_copy(&paltrequest), core::mem::transmute_copy(&ppalts), core::mem::transmute_copy(&pcalts)).into()
        }
        unsafe extern "system" fn Commit<Identity: ISpSRAlternates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paltrequest: *const SPPHRASEALTREQUEST, palt: *const SPPHRASEALT, ppvresultextra: *mut *mut core::ffi::c_void, pcbresultextra: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSRAlternates_Impl::Commit(this, core::mem::transmute_copy(&paltrequest), core::mem::transmute_copy(&palt), core::mem::transmute_copy(&ppvresultextra), core::mem::transmute_copy(&pcbresultextra)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAlternates: GetAlternates::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpSRAlternates as windows_core::Interface>::IID
    }
}
pub trait ISpSRAlternates2_Impl: Sized + ISpSRAlternates_Impl {
    fn CommitText(&self, paltrequest: *const SPPHRASEALTREQUEST, pcsznewtext: &windows_core::PCWSTR, commitflags: SPCOMMITFLAGS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpSRAlternates2 {}
impl ISpSRAlternates2_Vtbl {
    pub const fn new<Identity: ISpSRAlternates2_Impl, const OFFSET: isize>() -> ISpSRAlternates2_Vtbl {
        unsafe extern "system" fn CommitText<Identity: ISpSRAlternates2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paltrequest: *const SPPHRASEALTREQUEST, pcsznewtext: windows_core::PCWSTR, commitflags: SPCOMMITFLAGS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSRAlternates2_Impl::CommitText(this, core::mem::transmute_copy(&paltrequest), core::mem::transmute(&pcsznewtext), core::mem::transmute_copy(&commitflags)).into()
        }
        Self { base__: ISpSRAlternates_Vtbl::new::<Identity, OFFSET>(), CommitText: CommitText::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpSRAlternates2 as windows_core::Interface>::IID || iid == &<ISpSRAlternates as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait ISpSREngine_Impl: Sized + windows_core::IUnknownImpl {
    fn SetSite(&self, psite: Option<&ISpSREngineSite>) -> windows_core::Result<()>;
    fn GetInputAudioFormat(&self, pguidsourceformatid: *const windows_core::GUID, psourcewaveformatex: *const super::Audio::WAVEFORMATEX, pguiddesiredformatid: *mut windows_core::GUID, ppcomemdesiredwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> windows_core::Result<()>;
    fn RecognizeStream(&self, rguidfmtid: *const windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX, hrequestsync: super::super::Foundation::HANDLE, hdataavailable: super::super::Foundation::HANDLE, hexit: super::super::Foundation::HANDLE, fnewaudiostream: super::super::Foundation::BOOL, frealtimeaudio: super::super::Foundation::BOOL, paudioobjecttoken: Option<&ISpObjectToken>) -> windows_core::Result<()>;
    fn SetRecoProfile(&self, pprofile: Option<&ISpObjectToken>) -> windows_core::Result<()>;
    fn OnCreateGrammar(&self, pvenginerecocontext: *const core::ffi::c_void, hsapigrammar: SPGRAMMARHANDLE, ppvenginegrammarcontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn OnDeleteGrammar(&self, pvenginegrammar: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn LoadProprietaryGrammar(&self, pvenginegrammar: *const core::ffi::c_void, rguidparam: *const windows_core::GUID, pszstringparam: &windows_core::PCWSTR, pvdataparam: *const core::ffi::c_void, uldatasize: u32, options: SPLOADOPTIONS) -> windows_core::Result<()>;
    fn UnloadProprietaryGrammar(&self, pvenginegrammar: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetProprietaryRuleState(&self, pvenginegrammar: *const core::ffi::c_void, pszname: &windows_core::PCWSTR, preserved: *const core::ffi::c_void, newstate: SPRULESTATE) -> windows_core::Result<u32>;
    fn SetProprietaryRuleIdState(&self, pvenginegrammar: *const core::ffi::c_void, dwruleid: u32, newstate: SPRULESTATE) -> windows_core::Result<()>;
    fn LoadSLM(&self, pvenginegrammar: *const core::ffi::c_void, psztopicname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn UnloadSLM(&self, pvenginegrammar: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetSLMState(&self, pvenginegrammar: *const core::ffi::c_void, newstate: SPRULESTATE) -> windows_core::Result<()>;
    fn SetWordSequenceData(&self, pvenginegrammar: *const core::ffi::c_void, ptext: &windows_core::PCWSTR, cchtext: u32, pinfo: *const SPTEXTSELECTIONINFO) -> windows_core::Result<()>;
    fn SetTextSelection(&self, pvenginegrammar: *const core::ffi::c_void, pinfo: *const SPTEXTSELECTIONINFO) -> windows_core::Result<()>;
    fn IsPronounceable(&self, pvenginegrammar: *const core::ffi::c_void, pszword: &windows_core::PCWSTR) -> windows_core::Result<SPWORDPRONOUNCEABLE>;
    fn OnCreateRecoContext(&self, hsapirecocontext: SPRECOCONTEXTHANDLE, ppvenginecontext: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn OnDeleteRecoContext(&self, pvenginecontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn PrivateCall(&self, pvenginecontext: *const core::ffi::c_void, pcallframe: *mut core::ffi::c_void, ulcallframesize: u32) -> windows_core::Result<()>;
    fn SetAdaptationData(&self, pvenginecontext: *const core::ffi::c_void, padaptationdata: &windows_core::PCWSTR, cch: u32) -> windows_core::Result<()>;
    fn SetPropertyNum(&self, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: &windows_core::PCWSTR, lvalue: i32) -> windows_core::Result<()>;
    fn GetPropertyNum(&self, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: &windows_core::PCWSTR) -> windows_core::Result<i32>;
    fn SetPropertyString(&self, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: &windows_core::PCWSTR, pvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPropertyString(&self, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
    fn SetGrammarState(&self, pvenginegrammar: *const core::ffi::c_void, egrammarstate: SPGRAMMARSTATE) -> windows_core::Result<()>;
    fn WordNotify(&self, action: SPCFGNOTIFY, cwords: u32, pwords: *const SPWORDENTRY) -> windows_core::Result<()>;
    fn RuleNotify(&self, action: SPCFGNOTIFY, crules: u32, prules: *const SPRULEENTRY) -> windows_core::Result<()>;
    fn PrivateCallEx(&self, pvenginecontext: *const core::ffi::c_void, pincallframe: *const core::ffi::c_void, ulincallframesize: u32, ppvcomemresponse: *mut *mut core::ffi::c_void, pulresponsesize: *mut u32) -> windows_core::Result<()>;
    fn SetContextState(&self, pvenginecontext: *const core::ffi::c_void, econtextstate: SPCONTEXTSTATE) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::RuntimeName for ISpSREngine {}
#[cfg(feature = "Win32_Media_Audio")]
impl ISpSREngine_Vtbl {
    pub const fn new<Identity: ISpSREngine_Impl, const OFFSET: isize>() -> ISpSREngine_Vtbl {
        unsafe extern "system" fn SetSite<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psite: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::SetSite(this, windows_core::from_raw_borrowed(&psite)).into()
        }
        unsafe extern "system" fn GetInputAudioFormat<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidsourceformatid: *const windows_core::GUID, psourcewaveformatex: *const super::Audio::WAVEFORMATEX, pguiddesiredformatid: *mut windows_core::GUID, ppcomemdesiredwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::GetInputAudioFormat(this, core::mem::transmute_copy(&pguidsourceformatid), core::mem::transmute_copy(&psourcewaveformatex), core::mem::transmute_copy(&pguiddesiredformatid), core::mem::transmute_copy(&ppcomemdesiredwaveformatex)).into()
        }
        unsafe extern "system" fn RecognizeStream<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidfmtid: *const windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX, hrequestsync: super::super::Foundation::HANDLE, hdataavailable: super::super::Foundation::HANDLE, hexit: super::super::Foundation::HANDLE, fnewaudiostream: super::super::Foundation::BOOL, frealtimeaudio: super::super::Foundation::BOOL, paudioobjecttoken: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::RecognizeStream(this, core::mem::transmute_copy(&rguidfmtid), core::mem::transmute_copy(&pwaveformatex), core::mem::transmute_copy(&hrequestsync), core::mem::transmute_copy(&hdataavailable), core::mem::transmute_copy(&hexit), core::mem::transmute_copy(&fnewaudiostream), core::mem::transmute_copy(&frealtimeaudio), windows_core::from_raw_borrowed(&paudioobjecttoken)).into()
        }
        unsafe extern "system" fn SetRecoProfile<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprofile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::SetRecoProfile(this, windows_core::from_raw_borrowed(&pprofile)).into()
        }
        unsafe extern "system" fn OnCreateGrammar<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginerecocontext: *const core::ffi::c_void, hsapigrammar: SPGRAMMARHANDLE, ppvenginegrammarcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::OnCreateGrammar(this, core::mem::transmute_copy(&pvenginerecocontext), core::mem::transmute_copy(&hsapigrammar), core::mem::transmute_copy(&ppvenginegrammarcontext)).into()
        }
        unsafe extern "system" fn OnDeleteGrammar<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::OnDeleteGrammar(this, core::mem::transmute_copy(&pvenginegrammar)).into()
        }
        unsafe extern "system" fn LoadProprietaryGrammar<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, rguidparam: *const windows_core::GUID, pszstringparam: windows_core::PCWSTR, pvdataparam: *const core::ffi::c_void, uldatasize: u32, options: SPLOADOPTIONS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::LoadProprietaryGrammar(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute_copy(&rguidparam), core::mem::transmute(&pszstringparam), core::mem::transmute_copy(&pvdataparam), core::mem::transmute_copy(&uldatasize), core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn UnloadProprietaryGrammar<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::UnloadProprietaryGrammar(this, core::mem::transmute_copy(&pvenginegrammar)).into()
        }
        unsafe extern "system" fn SetProprietaryRuleState<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, pszname: windows_core::PCWSTR, preserved: *const core::ffi::c_void, newstate: SPRULESTATE, pcruleschanged: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpSREngine_Impl::SetProprietaryRuleState(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute(&pszname), core::mem::transmute_copy(&preserved), core::mem::transmute_copy(&newstate)) {
                Ok(ok__) => {
                    pcruleschanged.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProprietaryRuleIdState<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, dwruleid: u32, newstate: SPRULESTATE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::SetProprietaryRuleIdState(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute_copy(&dwruleid), core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn LoadSLM<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, psztopicname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::LoadSLM(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute(&psztopicname)).into()
        }
        unsafe extern "system" fn UnloadSLM<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::UnloadSLM(this, core::mem::transmute_copy(&pvenginegrammar)).into()
        }
        unsafe extern "system" fn SetSLMState<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, newstate: SPRULESTATE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::SetSLMState(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn SetWordSequenceData<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, ptext: windows_core::PCWSTR, cchtext: u32, pinfo: *const SPTEXTSELECTIONINFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::SetWordSequenceData(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute(&ptext), core::mem::transmute_copy(&cchtext), core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn SetTextSelection<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, pinfo: *const SPTEXTSELECTIONINFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::SetTextSelection(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn IsPronounceable<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, pszword: windows_core::PCWSTR, pwordpronounceable: *mut SPWORDPRONOUNCEABLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpSREngine_Impl::IsPronounceable(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute(&pszword)) {
                Ok(ok__) => {
                    pwordpronounceable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnCreateRecoContext<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hsapirecocontext: SPRECOCONTEXTHANDLE, ppvenginecontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::OnCreateRecoContext(this, core::mem::transmute_copy(&hsapirecocontext), core::mem::transmute_copy(&ppvenginecontext)).into()
        }
        unsafe extern "system" fn OnDeleteRecoContext<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginecontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::OnDeleteRecoContext(this, core::mem::transmute_copy(&pvenginecontext)).into()
        }
        unsafe extern "system" fn PrivateCall<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginecontext: *const core::ffi::c_void, pcallframe: *mut core::ffi::c_void, ulcallframesize: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::PrivateCall(this, core::mem::transmute_copy(&pvenginecontext), core::mem::transmute_copy(&pcallframe), core::mem::transmute_copy(&ulcallframesize)).into()
        }
        unsafe extern "system" fn SetAdaptationData<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginecontext: *const core::ffi::c_void, padaptationdata: windows_core::PCWSTR, cch: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::SetAdaptationData(this, core::mem::transmute_copy(&pvenginecontext), core::mem::transmute(&padaptationdata), core::mem::transmute_copy(&cch)).into()
        }
        unsafe extern "system" fn SetPropertyNum<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: windows_core::PCWSTR, lvalue: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::SetPropertyNum(this, core::mem::transmute_copy(&esrc), core::mem::transmute_copy(&pvsrcobj), core::mem::transmute(&pname), core::mem::transmute_copy(&lvalue)).into()
        }
        unsafe extern "system" fn GetPropertyNum<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: windows_core::PCWSTR, lvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpSREngine_Impl::GetPropertyNum(this, core::mem::transmute_copy(&esrc), core::mem::transmute_copy(&pvsrcobj), core::mem::transmute(&pname)) {
                Ok(ok__) => {
                    lvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyString<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: windows_core::PCWSTR, pvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::SetPropertyString(this, core::mem::transmute_copy(&esrc), core::mem::transmute_copy(&pvsrcobj), core::mem::transmute(&pname), core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn GetPropertyString<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, esrc: SPPROPSRC, pvsrcobj: *const core::ffi::c_void, pname: windows_core::PCWSTR, ppcomemvalue: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpSREngine_Impl::GetPropertyString(this, core::mem::transmute_copy(&esrc), core::mem::transmute_copy(&pvsrcobj), core::mem::transmute(&pname)) {
                Ok(ok__) => {
                    ppcomemvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGrammarState<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, egrammarstate: SPGRAMMARSTATE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::SetGrammarState(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute_copy(&egrammarstate)).into()
        }
        unsafe extern "system" fn WordNotify<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: SPCFGNOTIFY, cwords: u32, pwords: *const SPWORDENTRY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::WordNotify(this, core::mem::transmute_copy(&action), core::mem::transmute_copy(&cwords), core::mem::transmute_copy(&pwords)).into()
        }
        unsafe extern "system" fn RuleNotify<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: SPCFGNOTIFY, crules: u32, prules: *const SPRULEENTRY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::RuleNotify(this, core::mem::transmute_copy(&action), core::mem::transmute_copy(&crules), core::mem::transmute_copy(&prules)).into()
        }
        unsafe extern "system" fn PrivateCallEx<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginecontext: *const core::ffi::c_void, pincallframe: *const core::ffi::c_void, ulincallframesize: u32, ppvcomemresponse: *mut *mut core::ffi::c_void, pulresponsesize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::PrivateCallEx(this, core::mem::transmute_copy(&pvenginecontext), core::mem::transmute_copy(&pincallframe), core::mem::transmute_copy(&ulincallframesize), core::mem::transmute_copy(&ppvcomemresponse), core::mem::transmute_copy(&pulresponsesize)).into()
        }
        unsafe extern "system" fn SetContextState<Identity: ISpSREngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginecontext: *const core::ffi::c_void, econtextstate: SPCONTEXTSTATE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine_Impl::SetContextState(this, core::mem::transmute_copy(&pvenginecontext), core::mem::transmute_copy(&econtextstate)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSite: SetSite::<Identity, OFFSET>,
            GetInputAudioFormat: GetInputAudioFormat::<Identity, OFFSET>,
            RecognizeStream: RecognizeStream::<Identity, OFFSET>,
            SetRecoProfile: SetRecoProfile::<Identity, OFFSET>,
            OnCreateGrammar: OnCreateGrammar::<Identity, OFFSET>,
            OnDeleteGrammar: OnDeleteGrammar::<Identity, OFFSET>,
            LoadProprietaryGrammar: LoadProprietaryGrammar::<Identity, OFFSET>,
            UnloadProprietaryGrammar: UnloadProprietaryGrammar::<Identity, OFFSET>,
            SetProprietaryRuleState: SetProprietaryRuleState::<Identity, OFFSET>,
            SetProprietaryRuleIdState: SetProprietaryRuleIdState::<Identity, OFFSET>,
            LoadSLM: LoadSLM::<Identity, OFFSET>,
            UnloadSLM: UnloadSLM::<Identity, OFFSET>,
            SetSLMState: SetSLMState::<Identity, OFFSET>,
            SetWordSequenceData: SetWordSequenceData::<Identity, OFFSET>,
            SetTextSelection: SetTextSelection::<Identity, OFFSET>,
            IsPronounceable: IsPronounceable::<Identity, OFFSET>,
            OnCreateRecoContext: OnCreateRecoContext::<Identity, OFFSET>,
            OnDeleteRecoContext: OnDeleteRecoContext::<Identity, OFFSET>,
            PrivateCall: PrivateCall::<Identity, OFFSET>,
            SetAdaptationData: SetAdaptationData::<Identity, OFFSET>,
            SetPropertyNum: SetPropertyNum::<Identity, OFFSET>,
            GetPropertyNum: GetPropertyNum::<Identity, OFFSET>,
            SetPropertyString: SetPropertyString::<Identity, OFFSET>,
            GetPropertyString: GetPropertyString::<Identity, OFFSET>,
            SetGrammarState: SetGrammarState::<Identity, OFFSET>,
            WordNotify: WordNotify::<Identity, OFFSET>,
            RuleNotify: RuleNotify::<Identity, OFFSET>,
            PrivateCallEx: PrivateCallEx::<Identity, OFFSET>,
            SetContextState: SetContextState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpSREngine as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait ISpSREngine2_Impl: Sized + ISpSREngine_Impl {
    fn PrivateCallImmediate(&self, pvenginecontext: *const core::ffi::c_void, pincallframe: *const core::ffi::c_void, ulincallframesize: u32, ppvcomemresponse: *mut *mut core::ffi::c_void, pulresponsesize: *mut u32) -> windows_core::Result<()>;
    fn SetAdaptationData2(&self, pvenginecontext: *const core::ffi::c_void, padaptationdata: &windows_core::PCWSTR, cch: u32, ptopicname: &windows_core::PCWSTR, esettings: SPADAPTATIONSETTINGS, erelevance: SPADAPTATIONRELEVANCE) -> windows_core::Result<()>;
    fn SetGrammarPrefix(&self, pvenginegrammar: *const core::ffi::c_void, pszprefix: &windows_core::PCWSTR, fisprefixrequired: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetRulePriority(&self, hrule: SPRULEHANDLE, pvclientrulecontext: *const core::ffi::c_void, nrulepriority: i32) -> windows_core::Result<()>;
    fn EmulateRecognition(&self, pphrase: Option<&ISpPhrase>, dwcompareflags: u32) -> windows_core::Result<()>;
    fn SetSLMWeight(&self, pvenginegrammar: *const core::ffi::c_void, flweight: f32) -> windows_core::Result<()>;
    fn SetRuleWeight(&self, hrule: SPRULEHANDLE, pvclientrulecontext: *const core::ffi::c_void, flweight: f32) -> windows_core::Result<()>;
    fn SetTrainingState(&self, fdoingtraining: super::super::Foundation::BOOL, fadaptfromtrainingdata: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ResetAcousticModelAdaptation(&self) -> windows_core::Result<()>;
    fn OnLoadCFG(&self, pvenginegrammar: *const core::ffi::c_void, pgrammardata: *const SPBINARYGRAMMAR, ulgrammarid: u32) -> windows_core::Result<()>;
    fn OnUnloadCFG(&self, pvenginegrammar: *const core::ffi::c_void, ulgrammarid: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::RuntimeName for ISpSREngine2 {}
#[cfg(feature = "Win32_Media_Audio")]
impl ISpSREngine2_Vtbl {
    pub const fn new<Identity: ISpSREngine2_Impl, const OFFSET: isize>() -> ISpSREngine2_Vtbl {
        unsafe extern "system" fn PrivateCallImmediate<Identity: ISpSREngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginecontext: *const core::ffi::c_void, pincallframe: *const core::ffi::c_void, ulincallframesize: u32, ppvcomemresponse: *mut *mut core::ffi::c_void, pulresponsesize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine2_Impl::PrivateCallImmediate(this, core::mem::transmute_copy(&pvenginecontext), core::mem::transmute_copy(&pincallframe), core::mem::transmute_copy(&ulincallframesize), core::mem::transmute_copy(&ppvcomemresponse), core::mem::transmute_copy(&pulresponsesize)).into()
        }
        unsafe extern "system" fn SetAdaptationData2<Identity: ISpSREngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginecontext: *const core::ffi::c_void, padaptationdata: windows_core::PCWSTR, cch: u32, ptopicname: windows_core::PCWSTR, esettings: SPADAPTATIONSETTINGS, erelevance: SPADAPTATIONRELEVANCE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine2_Impl::SetAdaptationData2(this, core::mem::transmute_copy(&pvenginecontext), core::mem::transmute(&padaptationdata), core::mem::transmute_copy(&cch), core::mem::transmute(&ptopicname), core::mem::transmute_copy(&esettings), core::mem::transmute_copy(&erelevance)).into()
        }
        unsafe extern "system" fn SetGrammarPrefix<Identity: ISpSREngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, pszprefix: windows_core::PCWSTR, fisprefixrequired: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine2_Impl::SetGrammarPrefix(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute(&pszprefix), core::mem::transmute_copy(&fisprefixrequired)).into()
        }
        unsafe extern "system" fn SetRulePriority<Identity: ISpSREngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrule: SPRULEHANDLE, pvclientrulecontext: *const core::ffi::c_void, nrulepriority: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine2_Impl::SetRulePriority(this, core::mem::transmute_copy(&hrule), core::mem::transmute_copy(&pvclientrulecontext), core::mem::transmute_copy(&nrulepriority)).into()
        }
        unsafe extern "system" fn EmulateRecognition<Identity: ISpSREngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphrase: *mut core::ffi::c_void, dwcompareflags: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine2_Impl::EmulateRecognition(this, windows_core::from_raw_borrowed(&pphrase), core::mem::transmute_copy(&dwcompareflags)).into()
        }
        unsafe extern "system" fn SetSLMWeight<Identity: ISpSREngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, flweight: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine2_Impl::SetSLMWeight(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute_copy(&flweight)).into()
        }
        unsafe extern "system" fn SetRuleWeight<Identity: ISpSREngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrule: SPRULEHANDLE, pvclientrulecontext: *const core::ffi::c_void, flweight: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine2_Impl::SetRuleWeight(this, core::mem::transmute_copy(&hrule), core::mem::transmute_copy(&pvclientrulecontext), core::mem::transmute_copy(&flweight)).into()
        }
        unsafe extern "system" fn SetTrainingState<Identity: ISpSREngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdoingtraining: super::super::Foundation::BOOL, fadaptfromtrainingdata: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine2_Impl::SetTrainingState(this, core::mem::transmute_copy(&fdoingtraining), core::mem::transmute_copy(&fadaptfromtrainingdata)).into()
        }
        unsafe extern "system" fn ResetAcousticModelAdaptation<Identity: ISpSREngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine2_Impl::ResetAcousticModelAdaptation(this).into()
        }
        unsafe extern "system" fn OnLoadCFG<Identity: ISpSREngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, pgrammardata: *const SPBINARYGRAMMAR, ulgrammarid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine2_Impl::OnLoadCFG(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute_copy(&pgrammardata), core::mem::transmute_copy(&ulgrammarid)).into()
        }
        unsafe extern "system" fn OnUnloadCFG<Identity: ISpSREngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvenginegrammar: *const core::ffi::c_void, ulgrammarid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngine2_Impl::OnUnloadCFG(this, core::mem::transmute_copy(&pvenginegrammar), core::mem::transmute_copy(&ulgrammarid)).into()
        }
        Self {
            base__: ISpSREngine_Vtbl::new::<Identity, OFFSET>(),
            PrivateCallImmediate: PrivateCallImmediate::<Identity, OFFSET>,
            SetAdaptationData2: SetAdaptationData2::<Identity, OFFSET>,
            SetGrammarPrefix: SetGrammarPrefix::<Identity, OFFSET>,
            SetRulePriority: SetRulePriority::<Identity, OFFSET>,
            EmulateRecognition: EmulateRecognition::<Identity, OFFSET>,
            SetSLMWeight: SetSLMWeight::<Identity, OFFSET>,
            SetRuleWeight: SetRuleWeight::<Identity, OFFSET>,
            SetTrainingState: SetTrainingState::<Identity, OFFSET>,
            ResetAcousticModelAdaptation: ResetAcousticModelAdaptation::<Identity, OFFSET>,
            OnLoadCFG: OnLoadCFG::<Identity, OFFSET>,
            OnUnloadCFG: OnUnloadCFG::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpSREngine2 as windows_core::Interface>::IID || iid == &<ISpSREngine as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpSREngineSite_Impl: Sized + windows_core::IUnknownImpl {
    fn Read(&self, pv: *const core::ffi::c_void, cb: u32) -> windows_core::Result<u32>;
    fn DataAvailable(&self) -> windows_core::Result<u32>;
    fn SetBufferNotifySize(&self, cbsize: u32) -> windows_core::Result<()>;
    fn ParseFromTransitions(&self, pparseinfo: *const SPPARSEINFO) -> windows_core::Result<ISpPhraseBuilder>;
    fn Recognition(&self, presultinfo: *const SPRECORESULTINFO) -> windows_core::Result<()>;
    fn AddEvent(&self, pevent: *const SPEVENT, hsapirecocontext: SPRECOCONTEXTHANDLE) -> windows_core::Result<()>;
    fn Synchronize(&self, ullprocessedthrupos: u64) -> windows_core::Result<()>;
    fn GetWordInfo(&self, pwordentry: *mut SPWORDENTRY, options: SPWORDINFOOPT) -> windows_core::Result<()>;
    fn SetWordClientContext(&self, hword: SPWORDHANDLE, pvclientcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetRuleInfo(&self, pruleentry: *mut SPRULEENTRY, options: SPRULEINFOOPT) -> windows_core::Result<()>;
    fn SetRuleClientContext(&self, hrule: SPRULEHANDLE, pvclientcontext: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetStateInfo(&self, hstate: SPSTATEHANDLE, pstateinfo: *mut SPSTATEINFO) -> windows_core::Result<()>;
    fn GetResource(&self, hrule: SPRULEHANDLE, pszresourcename: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
    fn GetTransitionProperty(&self, id: SPTRANSITIONID) -> windows_core::Result<*mut SPTRANSITIONPROPERTY>;
    fn IsAlternate(&self, hrule: SPRULEHANDLE, haltrule: SPRULEHANDLE) -> windows_core::Result<()>;
    fn GetMaxAlternates(&self, hrule: SPRULEHANDLE) -> windows_core::Result<u32>;
    fn GetContextMaxAlternates(&self, hcontext: SPRECOCONTEXTHANDLE) -> windows_core::Result<u32>;
    fn UpdateRecoPos(&self, ullcurrentrecopos: u64) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpSREngineSite {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpSREngineSite_Vtbl {
    pub const fn new<Identity: ISpSREngineSite_Impl, const OFFSET: isize>() -> ISpSREngineSite_Vtbl {
        unsafe extern "system" fn Read<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void, cb: u32, pcbread: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpSREngineSite_Impl::Read(this, core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb)) {
                Ok(ok__) => {
                    pcbread.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataAvailable<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcb: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpSREngineSite_Impl::DataAvailable(this) {
                Ok(ok__) => {
                    pcb.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferNotifySize<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsize: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngineSite_Impl::SetBufferNotifySize(this, core::mem::transmute_copy(&cbsize)).into()
        }
        unsafe extern "system" fn ParseFromTransitions<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparseinfo: *const SPPARSEINFO, ppnewphrase: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpSREngineSite_Impl::ParseFromTransitions(this, core::mem::transmute_copy(&pparseinfo)) {
                Ok(ok__) => {
                    ppnewphrase.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recognition<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presultinfo: *const SPRECORESULTINFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngineSite_Impl::Recognition(this, core::mem::transmute_copy(&presultinfo)).into()
        }
        unsafe extern "system" fn AddEvent<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *const SPEVENT, hsapirecocontext: SPRECOCONTEXTHANDLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngineSite_Impl::AddEvent(this, core::mem::transmute_copy(&pevent), core::mem::transmute_copy(&hsapirecocontext)).into()
        }
        unsafe extern "system" fn Synchronize<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullprocessedthrupos: u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngineSite_Impl::Synchronize(this, core::mem::transmute_copy(&ullprocessedthrupos)).into()
        }
        unsafe extern "system" fn GetWordInfo<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwordentry: *mut SPWORDENTRY, options: SPWORDINFOOPT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngineSite_Impl::GetWordInfo(this, core::mem::transmute_copy(&pwordentry), core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn SetWordClientContext<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hword: SPWORDHANDLE, pvclientcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngineSite_Impl::SetWordClientContext(this, core::mem::transmute_copy(&hword), core::mem::transmute_copy(&pvclientcontext)).into()
        }
        unsafe extern "system" fn GetRuleInfo<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pruleentry: *mut SPRULEENTRY, options: SPRULEINFOOPT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngineSite_Impl::GetRuleInfo(this, core::mem::transmute_copy(&pruleentry), core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn SetRuleClientContext<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrule: SPRULEHANDLE, pvclientcontext: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngineSite_Impl::SetRuleClientContext(this, core::mem::transmute_copy(&hrule), core::mem::transmute_copy(&pvclientcontext)).into()
        }
        unsafe extern "system" fn GetStateInfo<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hstate: SPSTATEHANDLE, pstateinfo: *mut SPSTATEINFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngineSite_Impl::GetStateInfo(this, core::mem::transmute_copy(&hstate), core::mem::transmute_copy(&pstateinfo)).into()
        }
        unsafe extern "system" fn GetResource<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrule: SPRULEHANDLE, pszresourcename: windows_core::PCWSTR, ppcomemresource: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpSREngineSite_Impl::GetResource(this, core::mem::transmute_copy(&hrule), core::mem::transmute(&pszresourcename)) {
                Ok(ok__) => {
                    ppcomemresource.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransitionProperty<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: SPTRANSITIONID, ppcomemproperty: *mut *mut SPTRANSITIONPROPERTY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpSREngineSite_Impl::GetTransitionProperty(this, core::mem::transmute_copy(&id)) {
                Ok(ok__) => {
                    ppcomemproperty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAlternate<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrule: SPRULEHANDLE, haltrule: SPRULEHANDLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngineSite_Impl::IsAlternate(this, core::mem::transmute_copy(&hrule), core::mem::transmute_copy(&haltrule)).into()
        }
        unsafe extern "system" fn GetMaxAlternates<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrule: SPRULEHANDLE, pulnumalts: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpSREngineSite_Impl::GetMaxAlternates(this, core::mem::transmute_copy(&hrule)) {
                Ok(ok__) => {
                    pulnumalts.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextMaxAlternates<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hcontext: SPRECOCONTEXTHANDLE, pulnumalts: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpSREngineSite_Impl::GetContextMaxAlternates(this, core::mem::transmute_copy(&hcontext)) {
                Ok(ok__) => {
                    pulnumalts.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateRecoPos<Identity: ISpSREngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullcurrentrecopos: u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngineSite_Impl::UpdateRecoPos(this, core::mem::transmute_copy(&ullcurrentrecopos)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Read: Read::<Identity, OFFSET>,
            DataAvailable: DataAvailable::<Identity, OFFSET>,
            SetBufferNotifySize: SetBufferNotifySize::<Identity, OFFSET>,
            ParseFromTransitions: ParseFromTransitions::<Identity, OFFSET>,
            Recognition: Recognition::<Identity, OFFSET>,
            AddEvent: AddEvent::<Identity, OFFSET>,
            Synchronize: Synchronize::<Identity, OFFSET>,
            GetWordInfo: GetWordInfo::<Identity, OFFSET>,
            SetWordClientContext: SetWordClientContext::<Identity, OFFSET>,
            GetRuleInfo: GetRuleInfo::<Identity, OFFSET>,
            SetRuleClientContext: SetRuleClientContext::<Identity, OFFSET>,
            GetStateInfo: GetStateInfo::<Identity, OFFSET>,
            GetResource: GetResource::<Identity, OFFSET>,
            GetTransitionProperty: GetTransitionProperty::<Identity, OFFSET>,
            IsAlternate: IsAlternate::<Identity, OFFSET>,
            GetMaxAlternates: GetMaxAlternates::<Identity, OFFSET>,
            GetContextMaxAlternates: GetContextMaxAlternates::<Identity, OFFSET>,
            UpdateRecoPos: UpdateRecoPos::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpSREngineSite as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpSREngineSite2_Impl: Sized + ISpSREngineSite_Impl {
    fn AddEventEx(&self, pevent: *const SPEVENTEX, hsapirecocontext: SPRECOCONTEXTHANDLE) -> windows_core::Result<()>;
    fn UpdateRecoPosEx(&self, ullcurrentrecopos: u64, ullcurrentrecotime: u64) -> windows_core::Result<()>;
    fn GetRuleTransition(&self, ulgrammarid: u32, ruleindex: u32, ptrans: *mut SPTRANSITIONENTRY) -> windows_core::Result<()>;
    fn RecognitionEx(&self, presultinfo: *const SPRECORESULTINFOEX) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpSREngineSite2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpSREngineSite2_Vtbl {
    pub const fn new<Identity: ISpSREngineSite2_Impl, const OFFSET: isize>() -> ISpSREngineSite2_Vtbl {
        unsafe extern "system" fn AddEventEx<Identity: ISpSREngineSite2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *const SPEVENTEX, hsapirecocontext: SPRECOCONTEXTHANDLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngineSite2_Impl::AddEventEx(this, core::mem::transmute_copy(&pevent), core::mem::transmute_copy(&hsapirecocontext)).into()
        }
        unsafe extern "system" fn UpdateRecoPosEx<Identity: ISpSREngineSite2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullcurrentrecopos: u64, ullcurrentrecotime: u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngineSite2_Impl::UpdateRecoPosEx(this, core::mem::transmute_copy(&ullcurrentrecopos), core::mem::transmute_copy(&ullcurrentrecotime)).into()
        }
        unsafe extern "system" fn GetRuleTransition<Identity: ISpSREngineSite2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulgrammarid: u32, ruleindex: u32, ptrans: *mut SPTRANSITIONENTRY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngineSite2_Impl::GetRuleTransition(this, core::mem::transmute_copy(&ulgrammarid), core::mem::transmute_copy(&ruleindex), core::mem::transmute_copy(&ptrans)).into()
        }
        unsafe extern "system" fn RecognitionEx<Identity: ISpSREngineSite2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presultinfo: *const SPRECORESULTINFOEX) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSREngineSite2_Impl::RecognitionEx(this, core::mem::transmute_copy(&presultinfo)).into()
        }
        Self {
            base__: ISpSREngineSite_Vtbl::new::<Identity, OFFSET>(),
            AddEventEx: AddEventEx::<Identity, OFFSET>,
            UpdateRecoPosEx: UpdateRecoPosEx::<Identity, OFFSET>,
            GetRuleTransition: GetRuleTransition::<Identity, OFFSET>,
            RecognitionEx: RecognitionEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpSREngineSite2 as windows_core::Interface>::IID || iid == &<ISpSREngineSite as windows_core::Interface>::IID
    }
}
pub trait ISpSerializeState_Impl: Sized + windows_core::IUnknownImpl {
    fn GetSerializedState(&self, ppbdata: *mut *mut u8, pulsize: *mut u32, dwreserved: u32) -> windows_core::Result<()>;
    fn SetSerializedState(&self, pbdata: *const u8, ulsize: u32, dwreserved: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpSerializeState {}
impl ISpSerializeState_Vtbl {
    pub const fn new<Identity: ISpSerializeState_Impl, const OFFSET: isize>() -> ISpSerializeState_Vtbl {
        unsafe extern "system" fn GetSerializedState<Identity: ISpSerializeState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbdata: *mut *mut u8, pulsize: *mut u32, dwreserved: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSerializeState_Impl::GetSerializedState(this, core::mem::transmute_copy(&ppbdata), core::mem::transmute_copy(&pulsize), core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn SetSerializedState<Identity: ISpSerializeState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdata: *const u8, ulsize: u32, dwreserved: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpSerializeState_Impl::SetSerializedState(this, core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&ulsize), core::mem::transmute_copy(&dwreserved)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSerializedState: GetSerializedState::<Identity, OFFSET>,
            SetSerializedState: SetSerializedState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpSerializeState as windows_core::Interface>::IID
    }
}
pub trait ISpShortcut_Impl: Sized + windows_core::IUnknownImpl {
    fn AddShortcut(&self, pszdisplay: &windows_core::PCWSTR, langid: u16, pszspoken: &windows_core::PCWSTR, shtype: SPSHORTCUTTYPE) -> windows_core::Result<()>;
    fn RemoveShortcut(&self, pszdisplay: &windows_core::PCWSTR, langid: u16, pszspoken: &windows_core::PCWSTR, shtype: SPSHORTCUTTYPE) -> windows_core::Result<()>;
    fn GetShortcuts(&self, langid: u16, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> windows_core::Result<()>;
    fn GetGeneration(&self) -> windows_core::Result<u32>;
    fn GetWordsFromGenerationChange(&self, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> windows_core::Result<()>;
    fn GetWords(&self, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> windows_core::Result<()>;
    fn GetShortcutsForGeneration(&self, pdwgeneration: *mut u32, pdwcookie: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> windows_core::Result<()>;
    fn GetGenerationChange(&self, pdwgeneration: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpShortcut {}
impl ISpShortcut_Vtbl {
    pub const fn new<Identity: ISpShortcut_Impl, const OFFSET: isize>() -> ISpShortcut_Vtbl {
        unsafe extern "system" fn AddShortcut<Identity: ISpShortcut_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdisplay: windows_core::PCWSTR, langid: u16, pszspoken: windows_core::PCWSTR, shtype: SPSHORTCUTTYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpShortcut_Impl::AddShortcut(this, core::mem::transmute(&pszdisplay), core::mem::transmute_copy(&langid), core::mem::transmute(&pszspoken), core::mem::transmute_copy(&shtype)).into()
        }
        unsafe extern "system" fn RemoveShortcut<Identity: ISpShortcut_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdisplay: windows_core::PCWSTR, langid: u16, pszspoken: windows_core::PCWSTR, shtype: SPSHORTCUTTYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpShortcut_Impl::RemoveShortcut(this, core::mem::transmute(&pszdisplay), core::mem::transmute_copy(&langid), core::mem::transmute(&pszspoken), core::mem::transmute_copy(&shtype)).into()
        }
        unsafe extern "system" fn GetShortcuts<Identity: ISpShortcut_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: u16, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpShortcut_Impl::GetShortcuts(this, core::mem::transmute_copy(&langid), core::mem::transmute_copy(&pshortcutpairlist)).into()
        }
        unsafe extern "system" fn GetGeneration<Identity: ISpShortcut_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwgeneration: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpShortcut_Impl::GetGeneration(this) {
                Ok(ok__) => {
                    pdwgeneration.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWordsFromGenerationChange<Identity: ISpShortcut_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwgeneration: *mut u32, pwordlist: *mut SPWORDLIST) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpShortcut_Impl::GetWordsFromGenerationChange(this, core::mem::transmute_copy(&pdwgeneration), core::mem::transmute_copy(&pwordlist)).into()
        }
        unsafe extern "system" fn GetWords<Identity: ISpShortcut_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwgeneration: *mut u32, pdwcookie: *mut u32, pwordlist: *mut SPWORDLIST) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpShortcut_Impl::GetWords(this, core::mem::transmute_copy(&pdwgeneration), core::mem::transmute_copy(&pdwcookie), core::mem::transmute_copy(&pwordlist)).into()
        }
        unsafe extern "system" fn GetShortcutsForGeneration<Identity: ISpShortcut_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwgeneration: *mut u32, pdwcookie: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpShortcut_Impl::GetShortcutsForGeneration(this, core::mem::transmute_copy(&pdwgeneration), core::mem::transmute_copy(&pdwcookie), core::mem::transmute_copy(&pshortcutpairlist)).into()
        }
        unsafe extern "system" fn GetGenerationChange<Identity: ISpShortcut_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwgeneration: *mut u32, pshortcutpairlist: *mut SPSHORTCUTPAIRLIST) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpShortcut_Impl::GetGenerationChange(this, core::mem::transmute_copy(&pdwgeneration), core::mem::transmute_copy(&pshortcutpairlist)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddShortcut: AddShortcut::<Identity, OFFSET>,
            RemoveShortcut: RemoveShortcut::<Identity, OFFSET>,
            GetShortcuts: GetShortcuts::<Identity, OFFSET>,
            GetGeneration: GetGeneration::<Identity, OFFSET>,
            GetWordsFromGenerationChange: GetWordsFromGenerationChange::<Identity, OFFSET>,
            GetWords: GetWords::<Identity, OFFSET>,
            GetShortcutsForGeneration: GetShortcutsForGeneration::<Identity, OFFSET>,
            GetGenerationChange: GetGenerationChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpShortcut as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
pub trait ISpStream_Impl: Sized + ISpStreamFormat_Impl {
    fn SetBaseStream(&self, pstream: Option<&super::super::System::Com::IStream>, rguidformat: *const windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> windows_core::Result<()>;
    fn GetBaseStream(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn BindToFile(&self, pszfilename: &windows_core::PCWSTR, emode: SPFILEMODE, pformatid: *const windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX, ulleventinterest: u64) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ISpStream {}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ISpStream_Vtbl {
    pub const fn new<Identity: ISpStream_Impl, const OFFSET: isize>() -> ISpStream_Vtbl {
        unsafe extern "system" fn SetBaseStream<Identity: ISpStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, rguidformat: *const windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpStream_Impl::SetBaseStream(this, windows_core::from_raw_borrowed(&pstream), core::mem::transmute_copy(&rguidformat), core::mem::transmute_copy(&pwaveformatex)).into()
        }
        unsafe extern "system" fn GetBaseStream<Identity: ISpStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpStream_Impl::GetBaseStream(this) {
                Ok(ok__) => {
                    ppstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToFile<Identity: ISpStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilename: windows_core::PCWSTR, emode: SPFILEMODE, pformatid: *const windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX, ulleventinterest: u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpStream_Impl::BindToFile(this, core::mem::transmute(&pszfilename), core::mem::transmute_copy(&emode), core::mem::transmute_copy(&pformatid), core::mem::transmute_copy(&pwaveformatex), core::mem::transmute_copy(&ulleventinterest)).into()
        }
        unsafe extern "system" fn Close<Identity: ISpStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpStream_Impl::Close(this).into()
        }
        Self {
            base__: ISpStreamFormat_Vtbl::new::<Identity, OFFSET>(),
            SetBaseStream: SetBaseStream::<Identity, OFFSET>,
            GetBaseStream: GetBaseStream::<Identity, OFFSET>,
            BindToFile: BindToFile::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IStream as windows_core::Interface>::IID || iid == &<ISpStreamFormat as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
pub trait ISpStreamFormat_Impl: Sized + super::super::System::Com::IStream_Impl {
    fn GetFormat(&self, pguidformatid: *const windows_core::GUID) -> windows_core::Result<*mut super::Audio::WAVEFORMATEX>;
}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ISpStreamFormat {}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ISpStreamFormat_Vtbl {
    pub const fn new<Identity: ISpStreamFormat_Impl, const OFFSET: isize>() -> ISpStreamFormat_Vtbl {
        unsafe extern "system" fn GetFormat<Identity: ISpStreamFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidformatid: *const windows_core::GUID, ppcomemwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpStreamFormat_Impl::GetFormat(this, core::mem::transmute_copy(&pguidformatid)) {
                Ok(ok__) => {
                    ppcomemwaveformatex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IStream_Vtbl::new::<Identity, OFFSET>(), GetFormat: GetFormat::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpStreamFormat as windows_core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IStream as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
pub trait ISpStreamFormatConverter_Impl: Sized + ISpStreamFormat_Impl {
    fn SetBaseStream(&self, pstream: Option<&ISpStreamFormat>, fsetformattobasestreamformat: super::super::Foundation::BOOL, fwritetobasestream: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetBaseStream(&self) -> windows_core::Result<ISpStreamFormat>;
    fn SetFormat(&self, rguidformatidofconvertedstream: *const windows_core::GUID, pwaveformatexofconvertedstream: *const super::Audio::WAVEFORMATEX) -> windows_core::Result<()>;
    fn ResetSeekPosition(&self) -> windows_core::Result<()>;
    fn ScaleConvertedToBaseOffset(&self, ulloffsetconvertedstream: u64) -> windows_core::Result<u64>;
    fn ScaleBaseToConvertedOffset(&self, ulloffsetbasestream: u64) -> windows_core::Result<u64>;
}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl windows_core::RuntimeName for ISpStreamFormatConverter {}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com"))]
impl ISpStreamFormatConverter_Vtbl {
    pub const fn new<Identity: ISpStreamFormatConverter_Impl, const OFFSET: isize>() -> ISpStreamFormatConverter_Vtbl {
        unsafe extern "system" fn SetBaseStream<Identity: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, fsetformattobasestreamformat: super::super::Foundation::BOOL, fwritetobasestream: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpStreamFormatConverter_Impl::SetBaseStream(this, windows_core::from_raw_borrowed(&pstream), core::mem::transmute_copy(&fsetformattobasestreamformat), core::mem::transmute_copy(&fwritetobasestream)).into()
        }
        unsafe extern "system" fn GetBaseStream<Identity: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpStreamFormatConverter_Impl::GetBaseStream(this) {
                Ok(ok__) => {
                    ppstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Identity: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidformatidofconvertedstream: *const windows_core::GUID, pwaveformatexofconvertedstream: *const super::Audio::WAVEFORMATEX) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpStreamFormatConverter_Impl::SetFormat(this, core::mem::transmute_copy(&rguidformatidofconvertedstream), core::mem::transmute_copy(&pwaveformatexofconvertedstream)).into()
        }
        unsafe extern "system" fn ResetSeekPosition<Identity: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpStreamFormatConverter_Impl::ResetSeekPosition(this).into()
        }
        unsafe extern "system" fn ScaleConvertedToBaseOffset<Identity: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulloffsetconvertedstream: u64, pulloffsetbasestream: *mut u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpStreamFormatConverter_Impl::ScaleConvertedToBaseOffset(this, core::mem::transmute_copy(&ulloffsetconvertedstream)) {
                Ok(ok__) => {
                    pulloffsetbasestream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScaleBaseToConvertedOffset<Identity: ISpStreamFormatConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulloffsetbasestream: u64, pulloffsetconvertedstream: *mut u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpStreamFormatConverter_Impl::ScaleBaseToConvertedOffset(this, core::mem::transmute_copy(&ulloffsetbasestream)) {
                Ok(ok__) => {
                    pulloffsetconvertedstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISpStreamFormat_Vtbl::new::<Identity, OFFSET>(),
            SetBaseStream: SetBaseStream::<Identity, OFFSET>,
            GetBaseStream: GetBaseStream::<Identity, OFFSET>,
            SetFormat: SetFormat::<Identity, OFFSET>,
            ResetSeekPosition: ResetSeekPosition::<Identity, OFFSET>,
            ScaleConvertedToBaseOffset: ScaleConvertedToBaseOffset::<Identity, OFFSET>,
            ScaleBaseToConvertedOffset: ScaleBaseToConvertedOffset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpStreamFormatConverter as windows_core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IStream as windows_core::Interface>::IID || iid == &<ISpStreamFormat as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_Audio")]
pub trait ISpTTSEngine_Impl: Sized + windows_core::IUnknownImpl {
    fn Speak(&self, dwspeakflags: u32, rguidformatid: *const windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX, ptextfraglist: *const SPVTEXTFRAG, poutputsite: Option<&ISpTTSEngineSite>) -> windows_core::Result<()>;
    fn GetOutputFormat(&self, ptargetfmtid: *const windows_core::GUID, ptargetwaveformatex: *const super::Audio::WAVEFORMATEX, poutputformatid: *mut windows_core::GUID, ppcomemoutputwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_Audio")]
impl windows_core::RuntimeName for ISpTTSEngine {}
#[cfg(feature = "Win32_Media_Audio")]
impl ISpTTSEngine_Vtbl {
    pub const fn new<Identity: ISpTTSEngine_Impl, const OFFSET: isize>() -> ISpTTSEngine_Vtbl {
        unsafe extern "system" fn Speak<Identity: ISpTTSEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwspeakflags: u32, rguidformatid: *const windows_core::GUID, pwaveformatex: *const super::Audio::WAVEFORMATEX, ptextfraglist: *const SPVTEXTFRAG, poutputsite: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpTTSEngine_Impl::Speak(this, core::mem::transmute_copy(&dwspeakflags), core::mem::transmute_copy(&rguidformatid), core::mem::transmute_copy(&pwaveformatex), core::mem::transmute_copy(&ptextfraglist), windows_core::from_raw_borrowed(&poutputsite)).into()
        }
        unsafe extern "system" fn GetOutputFormat<Identity: ISpTTSEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptargetfmtid: *const windows_core::GUID, ptargetwaveformatex: *const super::Audio::WAVEFORMATEX, poutputformatid: *mut windows_core::GUID, ppcomemoutputwaveformatex: *mut *mut super::Audio::WAVEFORMATEX) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpTTSEngine_Impl::GetOutputFormat(this, core::mem::transmute_copy(&ptargetfmtid), core::mem::transmute_copy(&ptargetwaveformatex), core::mem::transmute_copy(&poutputformatid), core::mem::transmute_copy(&ppcomemoutputwaveformatex)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Speak: Speak::<Identity, OFFSET>,
            GetOutputFormat: GetOutputFormat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpTTSEngine as windows_core::Interface>::IID
    }
}
pub trait ISpTTSEngineSite_Impl: Sized + ISpEventSink_Impl {
    fn GetActions(&self) -> u32;
    fn Write(&self, pbuff: *const core::ffi::c_void, cb: u32) -> windows_core::Result<u32>;
    fn GetRate(&self) -> windows_core::Result<i32>;
    fn GetVolume(&self) -> windows_core::Result<u16>;
    fn GetSkipInfo(&self, petype: *mut SPVSKIPTYPE, plnumitems: *mut i32) -> windows_core::Result<()>;
    fn CompleteSkip(&self, ulnumskipped: i32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpTTSEngineSite {}
impl ISpTTSEngineSite_Vtbl {
    pub const fn new<Identity: ISpTTSEngineSite_Impl, const OFFSET: isize>() -> ISpTTSEngineSite_Vtbl {
        unsafe extern "system" fn GetActions<Identity: ISpTTSEngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpTTSEngineSite_Impl::GetActions(this)
        }
        unsafe extern "system" fn Write<Identity: ISpTTSEngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuff: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpTTSEngineSite_Impl::Write(this, core::mem::transmute_copy(&pbuff), core::mem::transmute_copy(&cb)) {
                Ok(ok__) => {
                    pcbwritten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRate<Identity: ISpTTSEngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prateadjust: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpTTSEngineSite_Impl::GetRate(this) {
                Ok(ok__) => {
                    prateadjust.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVolume<Identity: ISpTTSEngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pusvolume: *mut u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpTTSEngineSite_Impl::GetVolume(this) {
                Ok(ok__) => {
                    pusvolume.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSkipInfo<Identity: ISpTTSEngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, petype: *mut SPVSKIPTYPE, plnumitems: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpTTSEngineSite_Impl::GetSkipInfo(this, core::mem::transmute_copy(&petype), core::mem::transmute_copy(&plnumitems)).into()
        }
        unsafe extern "system" fn CompleteSkip<Identity: ISpTTSEngineSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulnumskipped: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpTTSEngineSite_Impl::CompleteSkip(this, core::mem::transmute_copy(&ulnumskipped)).into()
        }
        Self {
            base__: ISpEventSink_Vtbl::new::<Identity, OFFSET>(),
            GetActions: GetActions::<Identity, OFFSET>,
            Write: Write::<Identity, OFFSET>,
            GetRate: GetRate::<Identity, OFFSET>,
            GetVolume: GetVolume::<Identity, OFFSET>,
            GetSkipInfo: GetSkipInfo::<Identity, OFFSET>,
            CompleteSkip: CompleteSkip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpTTSEngineSite as windows_core::Interface>::IID || iid == &<ISpEventSink as windows_core::Interface>::IID
    }
}
pub trait ISpTask_Impl: Sized {
    fn Execute(&self, pvtaskdata: *mut core::ffi::c_void, pfcontinueprocessing: *const i32) -> windows_core::Result<()>;
}
impl ISpTask_Vtbl {
    pub const fn new<Impl: ISpTask_Impl>() -> ISpTask_Vtbl {
        unsafe extern "system" fn Execute<Impl: ISpTask_Impl>(this: *mut core::ffi::c_void, pvtaskdata: *mut core::ffi::c_void, pfcontinueprocessing: *const i32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ISpTask_Impl::Execute(this, core::mem::transmute_copy(&pvtaskdata), core::mem::transmute_copy(&pfcontinueprocessing)).into()
        }
        Self { Execute: Execute::<Impl> }
    }
}
#[doc(hidden)]
struct ISpTask_ImplVtbl<T: ISpTask_Impl>(std::marker::PhantomData<T>);
impl<T: ISpTask_Impl> ISpTask_ImplVtbl<T> {
    const VTABLE: ISpTask_Vtbl = ISpTask_Vtbl::new::<T>();
}
impl ISpTask {
    pub fn new<'a, T: ISpTask_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ISpTask_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait ISpTaskManager_Impl: Sized + windows_core::IUnknownImpl {
    fn SetThreadPoolInfo(&self, ppoolinfo: *const SPTMTHREADINFO) -> windows_core::Result<()>;
    fn GetThreadPoolInfo(&self) -> windows_core::Result<SPTMTHREADINFO>;
    fn QueueTask(&self, ptask: Option<&ISpTask>, pvtaskdata: *const core::ffi::c_void, hcompevent: super::super::Foundation::HANDLE, pdwgroupid: *mut u32, ptaskid: *mut u32) -> windows_core::Result<()>;
    fn CreateReoccurringTask(&self, ptask: Option<&ISpTask>, pvtaskdata: *const core::ffi::c_void, hcompevent: super::super::Foundation::HANDLE) -> windows_core::Result<ISpNotifySink>;
    fn CreateThreadControl(&self, ptask: Option<&ISpThreadTask>, pvtaskdata: *const core::ffi::c_void, npriority: i32) -> windows_core::Result<ISpThreadControl>;
    fn TerminateTask(&self, dwtaskid: u32, ulwaitperiod: u32) -> windows_core::Result<()>;
    fn TerminateTaskGroup(&self, dwgroupid: u32, ulwaitperiod: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpTaskManager {}
impl ISpTaskManager_Vtbl {
    pub const fn new<Identity: ISpTaskManager_Impl, const OFFSET: isize>() -> ISpTaskManager_Vtbl {
        unsafe extern "system" fn SetThreadPoolInfo<Identity: ISpTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoolinfo: *const SPTMTHREADINFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpTaskManager_Impl::SetThreadPoolInfo(this, core::mem::transmute_copy(&ppoolinfo)).into()
        }
        unsafe extern "system" fn GetThreadPoolInfo<Identity: ISpTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppoolinfo: *mut SPTMTHREADINFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpTaskManager_Impl::GetThreadPoolInfo(this) {
                Ok(ok__) => {
                    ppoolinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueueTask<Identity: ISpTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptask: *mut core::ffi::c_void, pvtaskdata: *const core::ffi::c_void, hcompevent: super::super::Foundation::HANDLE, pdwgroupid: *mut u32, ptaskid: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpTaskManager_Impl::QueueTask(this, windows_core::from_raw_borrowed(&ptask), core::mem::transmute_copy(&pvtaskdata), core::mem::transmute_copy(&hcompevent), core::mem::transmute_copy(&pdwgroupid), core::mem::transmute_copy(&ptaskid)).into()
        }
        unsafe extern "system" fn CreateReoccurringTask<Identity: ISpTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptask: *mut core::ffi::c_void, pvtaskdata: *const core::ffi::c_void, hcompevent: super::super::Foundation::HANDLE, pptaskctrl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpTaskManager_Impl::CreateReoccurringTask(this, windows_core::from_raw_borrowed(&ptask), core::mem::transmute_copy(&pvtaskdata), core::mem::transmute_copy(&hcompevent)) {
                Ok(ok__) => {
                    pptaskctrl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateThreadControl<Identity: ISpTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptask: *mut core::ffi::c_void, pvtaskdata: *const core::ffi::c_void, npriority: i32, pptaskctrl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpTaskManager_Impl::CreateThreadControl(this, windows_core::from_raw_borrowed(&ptask), core::mem::transmute_copy(&pvtaskdata), core::mem::transmute_copy(&npriority)) {
                Ok(ok__) => {
                    pptaskctrl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminateTask<Identity: ISpTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtaskid: u32, ulwaitperiod: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpTaskManager_Impl::TerminateTask(this, core::mem::transmute_copy(&dwtaskid), core::mem::transmute_copy(&ulwaitperiod)).into()
        }
        unsafe extern "system" fn TerminateTaskGroup<Identity: ISpTaskManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwgroupid: u32, ulwaitperiod: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpTaskManager_Impl::TerminateTaskGroup(this, core::mem::transmute_copy(&dwgroupid), core::mem::transmute_copy(&ulwaitperiod)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetThreadPoolInfo: SetThreadPoolInfo::<Identity, OFFSET>,
            GetThreadPoolInfo: GetThreadPoolInfo::<Identity, OFFSET>,
            QueueTask: QueueTask::<Identity, OFFSET>,
            CreateReoccurringTask: CreateReoccurringTask::<Identity, OFFSET>,
            CreateThreadControl: CreateThreadControl::<Identity, OFFSET>,
            TerminateTask: TerminateTask::<Identity, OFFSET>,
            TerminateTaskGroup: TerminateTaskGroup::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpTaskManager as windows_core::Interface>::IID
    }
}
pub trait ISpThreadControl_Impl: Sized + ISpNotifySink_Impl {
    fn StartThread(&self, dwflags: u32) -> windows_core::Result<super::super::Foundation::HWND>;
    fn WaitForThreadDone(&self, fforcestop: super::super::Foundation::BOOL, phrthreadresult: *mut windows_core::HRESULT, mstimeout: u32) -> windows_core::Result<()>;
    fn TerminateThread(&self) -> windows_core::Result<()>;
    fn ThreadHandle(&self) -> super::super::Foundation::HANDLE;
    fn ThreadId(&self) -> u32;
    fn NotifyEvent(&self) -> super::super::Foundation::HANDLE;
    fn WindowHandle(&self) -> super::super::Foundation::HWND;
    fn ThreadCompleteEvent(&self) -> super::super::Foundation::HANDLE;
    fn ExitThreadEvent(&self) -> super::super::Foundation::HANDLE;
}
impl windows_core::RuntimeName for ISpThreadControl {}
impl ISpThreadControl_Vtbl {
    pub const fn new<Identity: ISpThreadControl_Impl, const OFFSET: isize>() -> ISpThreadControl_Vtbl {
        unsafe extern "system" fn StartThread<Identity: ISpThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, phwnd: *mut super::super::Foundation::HWND) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpThreadControl_Impl::StartThread(this, core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    phwnd.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitForThreadDone<Identity: ISpThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fforcestop: super::super::Foundation::BOOL, phrthreadresult: *mut windows_core::HRESULT, mstimeout: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpThreadControl_Impl::WaitForThreadDone(this, core::mem::transmute_copy(&fforcestop), core::mem::transmute_copy(&phrthreadresult), core::mem::transmute_copy(&mstimeout)).into()
        }
        unsafe extern "system" fn TerminateThread<Identity: ISpThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpThreadControl_Impl::TerminateThread(this).into()
        }
        unsafe extern "system" fn ThreadHandle<Identity: ISpThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpThreadControl_Impl::ThreadHandle(this)
        }
        unsafe extern "system" fn ThreadId<Identity: ISpThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpThreadControl_Impl::ThreadId(this)
        }
        unsafe extern "system" fn NotifyEvent<Identity: ISpThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpThreadControl_Impl::NotifyEvent(this)
        }
        unsafe extern "system" fn WindowHandle<Identity: ISpThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::HWND {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpThreadControl_Impl::WindowHandle(this)
        }
        unsafe extern "system" fn ThreadCompleteEvent<Identity: ISpThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpThreadControl_Impl::ThreadCompleteEvent(this)
        }
        unsafe extern "system" fn ExitThreadEvent<Identity: ISpThreadControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpThreadControl_Impl::ExitThreadEvent(this)
        }
        Self {
            base__: ISpNotifySink_Vtbl::new::<Identity, OFFSET>(),
            StartThread: StartThread::<Identity, OFFSET>,
            WaitForThreadDone: WaitForThreadDone::<Identity, OFFSET>,
            TerminateThread: TerminateThread::<Identity, OFFSET>,
            ThreadHandle: ThreadHandle::<Identity, OFFSET>,
            ThreadId: ThreadId::<Identity, OFFSET>,
            NotifyEvent: NotifyEvent::<Identity, OFFSET>,
            WindowHandle: WindowHandle::<Identity, OFFSET>,
            ThreadCompleteEvent: ThreadCompleteEvent::<Identity, OFFSET>,
            ExitThreadEvent: ExitThreadEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpThreadControl as windows_core::Interface>::IID || iid == &<ISpNotifySink as windows_core::Interface>::IID
    }
}
pub trait ISpThreadTask_Impl: Sized {
    fn InitThread(&self, pvtaskdata: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn ThreadProc(&self, pvtaskdata: *mut core::ffi::c_void, hexitthreadevent: super::super::Foundation::HANDLE, hnotifyevent: super::super::Foundation::HANDLE, hwndworker: super::super::Foundation::HWND, pfcontinueprocessing: *const i32) -> windows_core::Result<()>;
    fn WindowMessage(&self, pvtaskdata: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
}
impl ISpThreadTask_Vtbl {
    pub const fn new<Impl: ISpThreadTask_Impl>() -> ISpThreadTask_Vtbl {
        unsafe extern "system" fn InitThread<Impl: ISpThreadTask_Impl>(this: *mut core::ffi::c_void, pvtaskdata: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ISpThreadTask_Impl::InitThread(this, core::mem::transmute_copy(&pvtaskdata), core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn ThreadProc<Impl: ISpThreadTask_Impl>(this: *mut core::ffi::c_void, pvtaskdata: *mut core::ffi::c_void, hexitthreadevent: super::super::Foundation::HANDLE, hnotifyevent: super::super::Foundation::HANDLE, hwndworker: super::super::Foundation::HWND, pfcontinueprocessing: *const i32) -> windows_core::HRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ISpThreadTask_Impl::ThreadProc(this, core::mem::transmute_copy(&pvtaskdata), core::mem::transmute_copy(&hexitthreadevent), core::mem::transmute_copy(&hnotifyevent), core::mem::transmute_copy(&hwndworker), core::mem::transmute_copy(&pfcontinueprocessing)).into()
        }
        unsafe extern "system" fn WindowMessage<Impl: ISpThreadTask_Impl>(this: *mut core::ffi::c_void, pvtaskdata: *mut core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT {
            let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            ISpThreadTask_Impl::WindowMessage(this, core::mem::transmute_copy(&pvtaskdata), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&msg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam))
        }
        Self { InitThread: InitThread::<Impl>, ThreadProc: ThreadProc::<Impl>, WindowMessage: WindowMessage::<Impl> }
    }
}
#[doc(hidden)]
struct ISpThreadTask_ImplVtbl<T: ISpThreadTask_Impl>(std::marker::PhantomData<T>);
impl<T: ISpThreadTask_Impl> ISpThreadTask_ImplVtbl<T> {
    const VTABLE: ISpThreadTask_Vtbl = ISpThreadTask_Vtbl::new::<T>();
}
impl ISpThreadTask {
    pub fn new<'a, T: ISpThreadTask_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &ISpThreadTask_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub trait ISpTokenUI_Impl: Sized + windows_core::IUnknownImpl {
    fn IsUISupported(&self, psztypeofui: &windows_core::PCWSTR, pvextradata: *const core::ffi::c_void, cbextradata: u32, punkobject: Option<&windows_core::IUnknown>) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn DisplayUI(&self, hwndparent: super::super::Foundation::HWND, psztitle: &windows_core::PCWSTR, psztypeofui: &windows_core::PCWSTR, pvextradata: *const core::ffi::c_void, cbextradata: u32, ptoken: Option<&ISpObjectToken>, punkobject: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpTokenUI {}
impl ISpTokenUI_Vtbl {
    pub const fn new<Identity: ISpTokenUI_Impl, const OFFSET: isize>() -> ISpTokenUI_Vtbl {
        unsafe extern "system" fn IsUISupported<Identity: ISpTokenUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztypeofui: windows_core::PCWSTR, pvextradata: *const core::ffi::c_void, cbextradata: u32, punkobject: *mut core::ffi::c_void, pfsupported: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpTokenUI_Impl::IsUISupported(this, core::mem::transmute(&psztypeofui), core::mem::transmute_copy(&pvextradata), core::mem::transmute_copy(&cbextradata), windows_core::from_raw_borrowed(&punkobject)) {
                Ok(ok__) => {
                    pfsupported.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Identity: ISpTokenUI_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, psztitle: windows_core::PCWSTR, psztypeofui: windows_core::PCWSTR, pvextradata: *const core::ffi::c_void, cbextradata: u32, ptoken: *mut core::ffi::c_void, punkobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpTokenUI_Impl::DisplayUI(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute(&psztitle), core::mem::transmute(&psztypeofui), core::mem::transmute_copy(&pvextradata), core::mem::transmute_copy(&cbextradata), windows_core::from_raw_borrowed(&ptoken), windows_core::from_raw_borrowed(&punkobject)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsUISupported: IsUISupported::<Identity, OFFSET>,
            DisplayUI: DisplayUI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpTokenUI as windows_core::Interface>::IID
    }
}
pub trait ISpTranscript_Impl: Sized + windows_core::IUnknownImpl {
    fn GetTranscript(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn AppendTranscript(&self, psztranscript: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISpTranscript {}
impl ISpTranscript_Vtbl {
    pub const fn new<Identity: ISpTranscript_Impl, const OFFSET: isize>() -> ISpTranscript_Vtbl {
        unsafe extern "system" fn GetTranscript<Identity: ISpTranscript_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsztranscript: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpTranscript_Impl::GetTranscript(this) {
                Ok(ok__) => {
                    ppsztranscript.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppendTranscript<Identity: ISpTranscript_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztranscript: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpTranscript_Impl::AppendTranscript(this, core::mem::transmute(&psztranscript)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTranscript: GetTranscript::<Identity, OFFSET>,
            AppendTranscript: AppendTranscript::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpTranscript as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISpVoice_Impl: Sized + ISpEventSource_Impl {
    fn SetOutput(&self, punkoutput: Option<&windows_core::IUnknown>, fallowformatchanges: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetOutputObjectToken(&self) -> windows_core::Result<ISpObjectToken>;
    fn GetOutputStream(&self) -> windows_core::Result<ISpStreamFormat>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn SetVoice(&self, ptoken: Option<&ISpObjectToken>) -> windows_core::Result<()>;
    fn GetVoice(&self) -> windows_core::Result<ISpObjectToken>;
    fn Speak(&self, pwcs: &windows_core::PCWSTR, dwflags: u32, pulstreamnumber: *mut u32) -> windows_core::Result<()>;
    fn SpeakStream(&self, pstream: Option<&super::super::System::Com::IStream>, dwflags: u32, pulstreamnumber: *mut u32) -> windows_core::Result<()>;
    fn GetStatus(&self, pstatus: *mut SPVOICESTATUS, ppszlastbookmark: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn Skip(&self, pitemtype: &windows_core::PCWSTR, lnumitems: i32, pulnumskipped: *mut u32) -> windows_core::Result<()>;
    fn SetPriority(&self, epriority: SPVPRIORITY) -> windows_core::Result<()>;
    fn GetPriority(&self, pepriority: *mut SPVPRIORITY) -> windows_core::Result<()>;
    fn SetAlertBoundary(&self, eboundary: SPEVENTENUM) -> windows_core::Result<()>;
    fn GetAlertBoundary(&self, peboundary: *mut SPEVENTENUM) -> windows_core::Result<()>;
    fn SetRate(&self, rateadjust: i32) -> windows_core::Result<()>;
    fn GetRate(&self, prateadjust: *mut i32) -> windows_core::Result<()>;
    fn SetVolume(&self, usvolume: u16) -> windows_core::Result<()>;
    fn GetVolume(&self, pusvolume: *mut u16) -> windows_core::Result<()>;
    fn WaitUntilDone(&self, mstimeout: u32) -> windows_core::Result<()>;
    fn SetSyncSpeakTimeout(&self, mstimeout: u32) -> windows_core::Result<()>;
    fn GetSyncSpeakTimeout(&self, pmstimeout: *mut u32) -> windows_core::Result<()>;
    fn SpeakCompleteEvent(&self) -> super::super::Foundation::HANDLE;
    fn IsUISupported(&self, psztypeofui: &windows_core::PCWSTR, pvextradata: *mut core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn DisplayUI(&self, hwndparent: super::super::Foundation::HWND, psztitle: &windows_core::PCWSTR, psztypeofui: &windows_core::PCWSTR, pvextradata: *mut core::ffi::c_void, cbextradata: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for ISpVoice {}
#[cfg(feature = "Win32_System_Com")]
impl ISpVoice_Vtbl {
    pub const fn new<Identity: ISpVoice_Impl, const OFFSET: isize>() -> ISpVoice_Vtbl {
        unsafe extern "system" fn SetOutput<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkoutput: *mut core::ffi::c_void, fallowformatchanges: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpVoice_Impl::SetOutput(this, windows_core::from_raw_borrowed(&punkoutput), core::mem::transmute_copy(&fallowformatchanges)).into()
        }
        unsafe extern "system" fn GetOutputObjectToken<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppobjecttoken: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpVoice_Impl::GetOutputObjectToken(this) {
                Ok(ok__) => {
                    ppobjecttoken.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputStream<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpVoice_Impl::GetOutputStream(this) {
                Ok(ok__) => {
                    ppstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpVoice_Impl::Pause(this).into()
        }
        unsafe extern "system" fn Resume<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpVoice_Impl::Resume(this).into()
        }
        unsafe extern "system" fn SetVoice<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptoken: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpVoice_Impl::SetVoice(this, windows_core::from_raw_borrowed(&ptoken)).into()
        }
        unsafe extern "system" fn GetVoice<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptoken: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpVoice_Impl::GetVoice(this) {
                Ok(ok__) => {
                    pptoken.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Speak<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcs: windows_core::PCWSTR, dwflags: u32, pulstreamnumber: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpVoice_Impl::Speak(this, core::mem::transmute(&pwcs), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pulstreamnumber)).into()
        }
        unsafe extern "system" fn SpeakStream<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, dwflags: u32, pulstreamnumber: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpVoice_Impl::SpeakStream(this, windows_core::from_raw_borrowed(&pstream), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pulstreamnumber)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut SPVOICESTATUS, ppszlastbookmark: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpVoice_Impl::GetStatus(this, core::mem::transmute_copy(&pstatus), core::mem::transmute_copy(&ppszlastbookmark)).into()
        }
        unsafe extern "system" fn Skip<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemtype: windows_core::PCWSTR, lnumitems: i32, pulnumskipped: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpVoice_Impl::Skip(this, core::mem::transmute(&pitemtype), core::mem::transmute_copy(&lnumitems), core::mem::transmute_copy(&pulnumskipped)).into()
        }
        unsafe extern "system" fn SetPriority<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, epriority: SPVPRIORITY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpVoice_Impl::SetPriority(this, core::mem::transmute_copy(&epriority)).into()
        }
        unsafe extern "system" fn GetPriority<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pepriority: *mut SPVPRIORITY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpVoice_Impl::GetPriority(this, core::mem::transmute_copy(&pepriority)).into()
        }
        unsafe extern "system" fn SetAlertBoundary<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eboundary: SPEVENTENUM) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpVoice_Impl::SetAlertBoundary(this, core::mem::transmute_copy(&eboundary)).into()
        }
        unsafe extern "system" fn GetAlertBoundary<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peboundary: *mut SPEVENTENUM) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpVoice_Impl::GetAlertBoundary(this, core::mem::transmute_copy(&peboundary)).into()
        }
        unsafe extern "system" fn SetRate<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rateadjust: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpVoice_Impl::SetRate(this, core::mem::transmute_copy(&rateadjust)).into()
        }
        unsafe extern "system" fn GetRate<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prateadjust: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpVoice_Impl::GetRate(this, core::mem::transmute_copy(&prateadjust)).into()
        }
        unsafe extern "system" fn SetVolume<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usvolume: u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpVoice_Impl::SetVolume(this, core::mem::transmute_copy(&usvolume)).into()
        }
        unsafe extern "system" fn GetVolume<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pusvolume: *mut u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpVoice_Impl::GetVolume(this, core::mem::transmute_copy(&pusvolume)).into()
        }
        unsafe extern "system" fn WaitUntilDone<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mstimeout: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpVoice_Impl::WaitUntilDone(this, core::mem::transmute_copy(&mstimeout)).into()
        }
        unsafe extern "system" fn SetSyncSpeakTimeout<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mstimeout: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpVoice_Impl::SetSyncSpeakTimeout(this, core::mem::transmute_copy(&mstimeout)).into()
        }
        unsafe extern "system" fn GetSyncSpeakTimeout<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmstimeout: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpVoice_Impl::GetSyncSpeakTimeout(this, core::mem::transmute_copy(&pmstimeout)).into()
        }
        unsafe extern "system" fn SpeakCompleteEvent<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpVoice_Impl::SpeakCompleteEvent(this)
        }
        unsafe extern "system" fn IsUISupported<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztypeofui: windows_core::PCWSTR, pvextradata: *mut core::ffi::c_void, cbextradata: u32, pfsupported: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpVoice_Impl::IsUISupported(this, core::mem::transmute(&psztypeofui), core::mem::transmute_copy(&pvextradata), core::mem::transmute_copy(&cbextradata), core::mem::transmute_copy(&pfsupported)).into()
        }
        unsafe extern "system" fn DisplayUI<Identity: ISpVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: super::super::Foundation::HWND, psztitle: windows_core::PCWSTR, psztypeofui: windows_core::PCWSTR, pvextradata: *mut core::ffi::c_void, cbextradata: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpVoice_Impl::DisplayUI(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute(&psztitle), core::mem::transmute(&psztypeofui), core::mem::transmute_copy(&pvextradata), core::mem::transmute_copy(&cbextradata)).into()
        }
        Self {
            base__: ISpEventSource_Vtbl::new::<Identity, OFFSET>(),
            SetOutput: SetOutput::<Identity, OFFSET>,
            GetOutputObjectToken: GetOutputObjectToken::<Identity, OFFSET>,
            GetOutputStream: GetOutputStream::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            SetVoice: SetVoice::<Identity, OFFSET>,
            GetVoice: GetVoice::<Identity, OFFSET>,
            Speak: Speak::<Identity, OFFSET>,
            SpeakStream: SpeakStream::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            GetPriority: GetPriority::<Identity, OFFSET>,
            SetAlertBoundary: SetAlertBoundary::<Identity, OFFSET>,
            GetAlertBoundary: GetAlertBoundary::<Identity, OFFSET>,
            SetRate: SetRate::<Identity, OFFSET>,
            GetRate: GetRate::<Identity, OFFSET>,
            SetVolume: SetVolume::<Identity, OFFSET>,
            GetVolume: GetVolume::<Identity, OFFSET>,
            WaitUntilDone: WaitUntilDone::<Identity, OFFSET>,
            SetSyncSpeakTimeout: SetSyncSpeakTimeout::<Identity, OFFSET>,
            GetSyncSpeakTimeout: GetSyncSpeakTimeout::<Identity, OFFSET>,
            SpeakCompleteEvent: SpeakCompleteEvent::<Identity, OFFSET>,
            IsUISupported: IsUISupported::<Identity, OFFSET>,
            DisplayUI: DisplayUI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpVoice as windows_core::Interface>::IID || iid == &<ISpNotifySource as windows_core::Interface>::IID || iid == &<ISpEventSource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpXMLRecoResult_Impl: Sized + ISpRecoResult_Impl {
    fn GetXMLResult(&self, ppszcomemxmlresult: *mut windows_core::PWSTR, options: SPXMLRESULTOPTIONS) -> windows_core::Result<()>;
    fn GetXMLErrorInfo(&self, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpXMLRecoResult {}
#[cfg(all(feature = "Win32_Media_Audio", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpXMLRecoResult_Vtbl {
    pub const fn new<Identity: ISpXMLRecoResult_Impl, const OFFSET: isize>() -> ISpXMLRecoResult_Vtbl {
        unsafe extern "system" fn GetXMLResult<Identity: ISpXMLRecoResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszcomemxmlresult: *mut windows_core::PWSTR, options: SPXMLRESULTOPTIONS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpXMLRecoResult_Impl::GetXMLResult(this, core::mem::transmute_copy(&ppszcomemxmlresult), core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn GetXMLErrorInfo<Identity: ISpXMLRecoResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psemanticerrorinfo: *mut SPSEMANTICERRORINFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpXMLRecoResult_Impl::GetXMLErrorInfo(this, core::mem::transmute_copy(&psemanticerrorinfo)).into()
        }
        Self {
            base__: ISpRecoResult_Vtbl::new::<Identity, OFFSET>(),
            GetXMLResult: GetXMLResult::<Identity, OFFSET>,
            GetXMLErrorInfo: GetXMLErrorInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpXMLRecoResult as windows_core::Interface>::IID || iid == &<ISpPhrase as windows_core::Interface>::IID || iid == &<ISpRecoResult as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechAudio_Impl: Sized + ISpeechBaseStream_Impl {
    fn Status(&self) -> windows_core::Result<ISpeechAudioStatus>;
    fn BufferInfo(&self) -> windows_core::Result<ISpeechAudioBufferInfo>;
    fn DefaultFormat(&self) -> windows_core::Result<ISpeechAudioFormat>;
    fn Volume(&self) -> windows_core::Result<i32>;
    fn SetVolume(&self, volume: i32) -> windows_core::Result<()>;
    fn BufferNotifySize(&self) -> windows_core::Result<i32>;
    fn SetBufferNotifySize(&self, buffernotifysize: i32) -> windows_core::Result<()>;
    fn EventHandle(&self) -> windows_core::Result<i32>;
    fn SetState(&self, state: SpeechAudioState) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechAudio {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechAudio_Vtbl {
    pub const fn new<Identity: ISpeechAudio_Impl, const OFFSET: isize>() -> ISpeechAudio_Vtbl {
        unsafe extern "system" fn Status<Identity: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechAudio_Impl::Status(this) {
                Ok(ok__) => {
                    status.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferInfo<Identity: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bufferinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechAudio_Impl::BufferInfo(this) {
                Ok(ok__) => {
                    bufferinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultFormat<Identity: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamformat: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechAudio_Impl::DefaultFormat(this) {
                Ok(ok__) => {
                    streamformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Volume<Identity: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, volume: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechAudio_Impl::Volume(this) {
                Ok(ok__) => {
                    volume.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Identity: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, volume: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechAudio_Impl::SetVolume(this, core::mem::transmute_copy(&volume)).into()
        }
        unsafe extern "system" fn BufferNotifySize<Identity: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffernotifysize: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechAudio_Impl::BufferNotifySize(this) {
                Ok(ok__) => {
                    buffernotifysize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferNotifySize<Identity: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffernotifysize: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechAudio_Impl::SetBufferNotifySize(this, core::mem::transmute_copy(&buffernotifysize)).into()
        }
        unsafe extern "system" fn EventHandle<Identity: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventhandle: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechAudio_Impl::EventHandle(this) {
                Ok(ok__) => {
                    eventhandle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: ISpeechAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: SpeechAudioState) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechAudio_Impl::SetState(this, core::mem::transmute_copy(&state)).into()
        }
        Self {
            base__: ISpeechBaseStream_Vtbl::new::<Identity, OFFSET>(),
            Status: Status::<Identity, OFFSET>,
            BufferInfo: BufferInfo::<Identity, OFFSET>,
            DefaultFormat: DefaultFormat::<Identity, OFFSET>,
            Volume: Volume::<Identity, OFFSET>,
            SetVolume: SetVolume::<Identity, OFFSET>,
            BufferNotifySize: BufferNotifySize::<Identity, OFFSET>,
            SetBufferNotifySize: SetBufferNotifySize::<Identity, OFFSET>,
            EventHandle: EventHandle::<Identity, OFFSET>,
            SetState: SetState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechAudio as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISpeechBaseStream as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechAudioBufferInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn MinNotification(&self) -> windows_core::Result<i32>;
    fn SetMinNotification(&self, minnotification: i32) -> windows_core::Result<()>;
    fn BufferSize(&self) -> windows_core::Result<i32>;
    fn SetBufferSize(&self, buffersize: i32) -> windows_core::Result<()>;
    fn EventBias(&self) -> windows_core::Result<i32>;
    fn SetEventBias(&self, eventbias: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechAudioBufferInfo {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechAudioBufferInfo_Vtbl {
    pub const fn new<Identity: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>() -> ISpeechAudioBufferInfo_Vtbl {
        unsafe extern "system" fn MinNotification<Identity: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minnotification: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechAudioBufferInfo_Impl::MinNotification(this) {
                Ok(ok__) => {
                    minnotification.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinNotification<Identity: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, minnotification: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechAudioBufferInfo_Impl::SetMinNotification(this, core::mem::transmute_copy(&minnotification)).into()
        }
        unsafe extern "system" fn BufferSize<Identity: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffersize: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechAudioBufferInfo_Impl::BufferSize(this) {
                Ok(ok__) => {
                    buffersize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferSize<Identity: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffersize: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechAudioBufferInfo_Impl::SetBufferSize(this, core::mem::transmute_copy(&buffersize)).into()
        }
        unsafe extern "system" fn EventBias<Identity: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventbias: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechAudioBufferInfo_Impl::EventBias(this) {
                Ok(ok__) => {
                    eventbias.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventBias<Identity: ISpeechAudioBufferInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventbias: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechAudioBufferInfo_Impl::SetEventBias(this, core::mem::transmute_copy(&eventbias)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            MinNotification: MinNotification::<Identity, OFFSET>,
            SetMinNotification: SetMinNotification::<Identity, OFFSET>,
            BufferSize: BufferSize::<Identity, OFFSET>,
            SetBufferSize: SetBufferSize::<Identity, OFFSET>,
            EventBias: EventBias::<Identity, OFFSET>,
            SetEventBias: SetEventBias::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechAudioBufferInfo as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechAudioFormat_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&self) -> windows_core::Result<SpeechAudioFormatType>;
    fn SetType(&self, audioformat: SpeechAudioFormatType) -> windows_core::Result<()>;
    fn Guid(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetGuid(&self, guid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetWaveFormatEx(&self) -> windows_core::Result<ISpeechWaveFormatEx>;
    fn SetWaveFormatEx(&self, speechwaveformatex: Option<&ISpeechWaveFormatEx>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechAudioFormat {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechAudioFormat_Vtbl {
    pub const fn new<Identity: ISpeechAudioFormat_Impl, const OFFSET: isize>() -> ISpeechAudioFormat_Vtbl {
        unsafe extern "system" fn Type<Identity: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audioformat: *mut SpeechAudioFormatType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechAudioFormat_Impl::Type(this) {
                Ok(ok__) => {
                    audioformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audioformat: SpeechAudioFormatType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechAudioFormat_Impl::SetType(this, core::mem::transmute_copy(&audioformat)).into()
        }
        unsafe extern "system" fn Guid<Identity: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechAudioFormat_Impl::Guid(this) {
                Ok(ok__) => {
                    guid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGuid<Identity: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechAudioFormat_Impl::SetGuid(this, core::mem::transmute(&guid)).into()
        }
        unsafe extern "system" fn GetWaveFormatEx<Identity: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, speechwaveformatex: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechAudioFormat_Impl::GetWaveFormatEx(this) {
                Ok(ok__) => {
                    speechwaveformatex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWaveFormatEx<Identity: ISpeechAudioFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, speechwaveformatex: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechAudioFormat_Impl::SetWaveFormatEx(this, windows_core::from_raw_borrowed(&speechwaveformatex)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Type: Type::<Identity, OFFSET>,
            SetType: SetType::<Identity, OFFSET>,
            Guid: Guid::<Identity, OFFSET>,
            SetGuid: SetGuid::<Identity, OFFSET>,
            GetWaveFormatEx: GetWaveFormatEx::<Identity, OFFSET>,
            SetWaveFormatEx: SetWaveFormatEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechAudioFormat as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechAudioStatus_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn FreeBufferSpace(&self) -> windows_core::Result<i32>;
    fn NonBlockingIO(&self) -> windows_core::Result<i32>;
    fn State(&self) -> windows_core::Result<SpeechAudioState>;
    fn CurrentSeekPosition(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn CurrentDevicePosition(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechAudioStatus {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechAudioStatus_Vtbl {
    pub const fn new<Identity: ISpeechAudioStatus_Impl, const OFFSET: isize>() -> ISpeechAudioStatus_Vtbl {
        unsafe extern "system" fn FreeBufferSpace<Identity: ISpeechAudioStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, freebufferspace: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechAudioStatus_Impl::FreeBufferSpace(this) {
                Ok(ok__) => {
                    freebufferspace.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NonBlockingIO<Identity: ISpeechAudioStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nonblockingio: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechAudioStatus_Impl::NonBlockingIO(this) {
                Ok(ok__) => {
                    nonblockingio.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Identity: ISpeechAudioStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut SpeechAudioState) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechAudioStatus_Impl::State(this) {
                Ok(ok__) => {
                    state.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentSeekPosition<Identity: ISpeechAudioStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentseekposition: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechAudioStatus_Impl::CurrentSeekPosition(this) {
                Ok(ok__) => {
                    currentseekposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentDevicePosition<Identity: ISpeechAudioStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentdeviceposition: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechAudioStatus_Impl::CurrentDevicePosition(this) {
                Ok(ok__) => {
                    currentdeviceposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            FreeBufferSpace: FreeBufferSpace::<Identity, OFFSET>,
            NonBlockingIO: NonBlockingIO::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            CurrentSeekPosition: CurrentSeekPosition::<Identity, OFFSET>,
            CurrentDevicePosition: CurrentDevicePosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechAudioStatus as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechBaseStream_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Format(&self) -> windows_core::Result<ISpeechAudioFormat>;
    fn putref_Format(&self, audioformat: Option<&ISpeechAudioFormat>) -> windows_core::Result<()>;
    fn Read(&self, buffer: *mut super::super::System::Variant::VARIANT, numberofbytes: i32, bytesread: *mut i32) -> windows_core::Result<()>;
    fn Write(&self, buffer: &super::super::System::Variant::VARIANT) -> windows_core::Result<i32>;
    fn Seek(&self, position: &super::super::System::Variant::VARIANT, origin: SpeechStreamSeekPositionType) -> windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechBaseStream {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechBaseStream_Vtbl {
    pub const fn new<Identity: ISpeechBaseStream_Impl, const OFFSET: isize>() -> ISpeechBaseStream_Vtbl {
        unsafe extern "system" fn Format<Identity: ISpeechBaseStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audioformat: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechBaseStream_Impl::Format(this) {
                Ok(ok__) => {
                    audioformat.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Format<Identity: ISpeechBaseStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audioformat: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechBaseStream_Impl::putref_Format(this, windows_core::from_raw_borrowed(&audioformat)).into()
        }
        unsafe extern "system" fn Read<Identity: ISpeechBaseStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>, numberofbytes: i32, bytesread: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechBaseStream_Impl::Read(this, core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&numberofbytes), core::mem::transmute_copy(&bytesread)).into()
        }
        unsafe extern "system" fn Write<Identity: ISpeechBaseStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: core::mem::MaybeUninit<super::super::System::Variant::VARIANT>, byteswritten: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechBaseStream_Impl::Write(this, core::mem::transmute(&buffer)) {
                Ok(ok__) => {
                    byteswritten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Seek<Identity: ISpeechBaseStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, position: core::mem::MaybeUninit<super::super::System::Variant::VARIANT>, origin: SpeechStreamSeekPositionType, newposition: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechBaseStream_Impl::Seek(this, core::mem::transmute(&position), core::mem::transmute_copy(&origin)) {
                Ok(ok__) => {
                    newposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Format: Format::<Identity, OFFSET>,
            putref_Format: putref_Format::<Identity, OFFSET>,
            Read: Read::<Identity, OFFSET>,
            Write: Write::<Identity, OFFSET>,
            Seek: Seek::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechBaseStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechCustomStream_Impl: Sized + ISpeechBaseStream_Impl {
    fn BaseStream(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn putref_BaseStream(&self, punkstream: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechCustomStream {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechCustomStream_Vtbl {
    pub const fn new<Identity: ISpeechCustomStream_Impl, const OFFSET: isize>() -> ISpeechCustomStream_Vtbl {
        unsafe extern "system" fn BaseStream<Identity: ISpeechCustomStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunkstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechCustomStream_Impl::BaseStream(this) {
                Ok(ok__) => {
                    ppunkstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_BaseStream<Identity: ISpeechCustomStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechCustomStream_Impl::putref_BaseStream(this, windows_core::from_raw_borrowed(&punkstream)).into()
        }
        Self {
            base__: ISpeechBaseStream_Vtbl::new::<Identity, OFFSET>(),
            BaseStream: BaseStream::<Identity, OFFSET>,
            putref_BaseStream: putref_BaseStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechCustomStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISpeechBaseStream as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechDataKey_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetBinaryValue(&self, valuename: &windows_core::BSTR, value: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn GetBinaryValue(&self, valuename: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetStringValue(&self, valuename: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetStringValue(&self, valuename: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn SetLongValue(&self, valuename: &windows_core::BSTR, value: i32) -> windows_core::Result<()>;
    fn GetLongValue(&self, valuename: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn OpenKey(&self, subkeyname: &windows_core::BSTR) -> windows_core::Result<ISpeechDataKey>;
    fn CreateKey(&self, subkeyname: &windows_core::BSTR) -> windows_core::Result<ISpeechDataKey>;
    fn DeleteKey(&self, subkeyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DeleteValue(&self, valuename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EnumKeys(&self, index: i32) -> windows_core::Result<windows_core::BSTR>;
    fn EnumValues(&self, index: i32) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechDataKey {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechDataKey_Vtbl {
    pub const fn new<Identity: ISpeechDataKey_Impl, const OFFSET: isize>() -> ISpeechDataKey_Vtbl {
        unsafe extern "system" fn SetBinaryValue<Identity: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, valuename: core::mem::MaybeUninit<windows_core::BSTR>, value: core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechDataKey_Impl::SetBinaryValue(this, core::mem::transmute(&valuename), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GetBinaryValue<Identity: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, valuename: core::mem::MaybeUninit<windows_core::BSTR>, value: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechDataKey_Impl::GetBinaryValue(this, core::mem::transmute(&valuename)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStringValue<Identity: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, valuename: core::mem::MaybeUninit<windows_core::BSTR>, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechDataKey_Impl::SetStringValue(this, core::mem::transmute(&valuename), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn GetStringValue<Identity: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, valuename: core::mem::MaybeUninit<windows_core::BSTR>, value: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechDataKey_Impl::GetStringValue(this, core::mem::transmute(&valuename)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLongValue<Identity: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, valuename: core::mem::MaybeUninit<windows_core::BSTR>, value: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechDataKey_Impl::SetLongValue(this, core::mem::transmute(&valuename), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetLongValue<Identity: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, valuename: core::mem::MaybeUninit<windows_core::BSTR>, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechDataKey_Impl::GetLongValue(this, core::mem::transmute(&valuename)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenKey<Identity: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subkeyname: core::mem::MaybeUninit<windows_core::BSTR>, subkey: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechDataKey_Impl::OpenKey(this, core::mem::transmute(&subkeyname)) {
                Ok(ok__) => {
                    subkey.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateKey<Identity: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subkeyname: core::mem::MaybeUninit<windows_core::BSTR>, subkey: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechDataKey_Impl::CreateKey(this, core::mem::transmute(&subkeyname)) {
                Ok(ok__) => {
                    subkey.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteKey<Identity: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subkeyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechDataKey_Impl::DeleteKey(this, core::mem::transmute(&subkeyname)).into()
        }
        unsafe extern "system" fn DeleteValue<Identity: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, valuename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechDataKey_Impl::DeleteValue(this, core::mem::transmute(&valuename)).into()
        }
        unsafe extern "system" fn EnumKeys<Identity: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, subkeyname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechDataKey_Impl::EnumKeys(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    subkeyname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumValues<Identity: ISpeechDataKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, valuename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechDataKey_Impl::EnumValues(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    valuename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetBinaryValue: SetBinaryValue::<Identity, OFFSET>,
            GetBinaryValue: GetBinaryValue::<Identity, OFFSET>,
            SetStringValue: SetStringValue::<Identity, OFFSET>,
            GetStringValue: GetStringValue::<Identity, OFFSET>,
            SetLongValue: SetLongValue::<Identity, OFFSET>,
            GetLongValue: GetLongValue::<Identity, OFFSET>,
            OpenKey: OpenKey::<Identity, OFFSET>,
            CreateKey: CreateKey::<Identity, OFFSET>,
            DeleteKey: DeleteKey::<Identity, OFFSET>,
            DeleteValue: DeleteValue::<Identity, OFFSET>,
            EnumKeys: EnumKeys::<Identity, OFFSET>,
            EnumValues: EnumValues::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechDataKey as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechFileStream_Impl: Sized + ISpeechBaseStream_Impl {
    fn Open(&self, filename: &windows_core::BSTR, filemode: SpeechStreamFileMode, doevents: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechFileStream {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechFileStream_Vtbl {
    pub const fn new<Identity: ISpeechFileStream_Impl, const OFFSET: isize>() -> ISpeechFileStream_Vtbl {
        unsafe extern "system" fn Open<Identity: ISpeechFileStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: core::mem::MaybeUninit<windows_core::BSTR>, filemode: SpeechStreamFileMode, doevents: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechFileStream_Impl::Open(this, core::mem::transmute(&filename), core::mem::transmute_copy(&filemode), core::mem::transmute_copy(&doevents)).into()
        }
        unsafe extern "system" fn Close<Identity: ISpeechFileStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechFileStream_Impl::Close(this).into()
        }
        Self { base__: ISpeechBaseStream_Vtbl::new::<Identity, OFFSET>(), Open: Open::<Identity, OFFSET>, Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechFileStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISpeechBaseStream as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechGrammarRule_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Attributes(&self) -> windows_core::Result<SpeechRuleAttributes>;
    fn InitialState(&self) -> windows_core::Result<ISpeechGrammarRuleState>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Id(&self) -> windows_core::Result<i32>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddResource(&self, resourcename: &windows_core::BSTR, resourcevalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddState(&self) -> windows_core::Result<ISpeechGrammarRuleState>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechGrammarRule {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechGrammarRule_Vtbl {
    pub const fn new<Identity: ISpeechGrammarRule_Impl, const OFFSET: isize>() -> ISpeechGrammarRule_Vtbl {
        unsafe extern "system" fn Attributes<Identity: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributes: *mut SpeechRuleAttributes) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRule_Impl::Attributes(this) {
                Ok(ok__) => {
                    attributes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitialState<Identity: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRule_Impl::InitialState(this) {
                Ok(ok__) => {
                    state.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRule_Impl::Name(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRule_Impl::Id(this) {
                Ok(ok__) => {
                    id.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Identity: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechGrammarRule_Impl::Clear(this).into()
        }
        unsafe extern "system" fn AddResource<Identity: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcename: core::mem::MaybeUninit<windows_core::BSTR>, resourcevalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechGrammarRule_Impl::AddResource(this, core::mem::transmute(&resourcename), core::mem::transmute(&resourcevalue)).into()
        }
        unsafe extern "system" fn AddState<Identity: ISpeechGrammarRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRule_Impl::AddState(this) {
                Ok(ok__) => {
                    state.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Attributes: Attributes::<Identity, OFFSET>,
            InitialState: InitialState::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddResource: AddResource::<Identity, OFFSET>,
            AddState: AddState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechGrammarRule as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechGrammarRuleState_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Rule(&self) -> windows_core::Result<ISpeechGrammarRule>;
    fn Transitions(&self) -> windows_core::Result<ISpeechGrammarRuleStateTransitions>;
    fn AddWordTransition(&self, deststate: Option<&ISpeechGrammarRuleState>, words: &windows_core::BSTR, separators: &windows_core::BSTR, r#type: SpeechGrammarWordType, propertyname: &windows_core::BSTR, propertyid: i32, propertyvalue: *const super::super::System::Variant::VARIANT, weight: f32) -> windows_core::Result<()>;
    fn AddRuleTransition(&self, destinationstate: Option<&ISpeechGrammarRuleState>, rule: Option<&ISpeechGrammarRule>, propertyname: &windows_core::BSTR, propertyid: i32, propertyvalue: *const super::super::System::Variant::VARIANT, weight: f32) -> windows_core::Result<()>;
    fn AddSpecialTransition(&self, destinationstate: Option<&ISpeechGrammarRuleState>, r#type: SpeechSpecialTransitionType, propertyname: &windows_core::BSTR, propertyid: i32, propertyvalue: *const super::super::System::Variant::VARIANT, weight: f32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechGrammarRuleState {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechGrammarRuleState_Vtbl {
    pub const fn new<Identity: ISpeechGrammarRuleState_Impl, const OFFSET: isize>() -> ISpeechGrammarRuleState_Vtbl {
        unsafe extern "system" fn Rule<Identity: ISpeechGrammarRuleState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRuleState_Impl::Rule(this) {
                Ok(ok__) => {
                    rule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transitions<Identity: ISpeechGrammarRuleState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transitions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRuleState_Impl::Transitions(this) {
                Ok(ok__) => {
                    transitions.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddWordTransition<Identity: ISpeechGrammarRuleState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deststate: *mut core::ffi::c_void, words: core::mem::MaybeUninit<windows_core::BSTR>, separators: core::mem::MaybeUninit<windows_core::BSTR>, r#type: SpeechGrammarWordType, propertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyid: i32, propertyvalue: *const core::mem::MaybeUninit<super::super::System::Variant::VARIANT>, weight: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechGrammarRuleState_Impl::AddWordTransition(this, windows_core::from_raw_borrowed(&deststate), core::mem::transmute(&words), core::mem::transmute(&separators), core::mem::transmute_copy(&r#type), core::mem::transmute(&propertyname), core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&propertyvalue), core::mem::transmute_copy(&weight)).into()
        }
        unsafe extern "system" fn AddRuleTransition<Identity: ISpeechGrammarRuleState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destinationstate: *mut core::ffi::c_void, rule: *mut core::ffi::c_void, propertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyid: i32, propertyvalue: *const core::mem::MaybeUninit<super::super::System::Variant::VARIANT>, weight: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechGrammarRuleState_Impl::AddRuleTransition(this, windows_core::from_raw_borrowed(&destinationstate), windows_core::from_raw_borrowed(&rule), core::mem::transmute(&propertyname), core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&propertyvalue), core::mem::transmute_copy(&weight)).into()
        }
        unsafe extern "system" fn AddSpecialTransition<Identity: ISpeechGrammarRuleState_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destinationstate: *mut core::ffi::c_void, r#type: SpeechSpecialTransitionType, propertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyid: i32, propertyvalue: *const core::mem::MaybeUninit<super::super::System::Variant::VARIANT>, weight: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechGrammarRuleState_Impl::AddSpecialTransition(this, windows_core::from_raw_borrowed(&destinationstate), core::mem::transmute_copy(&r#type), core::mem::transmute(&propertyname), core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&propertyvalue), core::mem::transmute_copy(&weight)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Rule: Rule::<Identity, OFFSET>,
            Transitions: Transitions::<Identity, OFFSET>,
            AddWordTransition: AddWordTransition::<Identity, OFFSET>,
            AddRuleTransition: AddRuleTransition::<Identity, OFFSET>,
            AddSpecialTransition: AddSpecialTransition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechGrammarRuleState as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechGrammarRuleStateTransition_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&self) -> windows_core::Result<SpeechGrammarRuleStateTransitionType>;
    fn Text(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Rule(&self) -> windows_core::Result<ISpeechGrammarRule>;
    fn Weight(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PropertyName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn PropertyId(&self) -> windows_core::Result<i32>;
    fn PropertyValue(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn NextState(&self) -> windows_core::Result<ISpeechGrammarRuleState>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechGrammarRuleStateTransition {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechGrammarRuleStateTransition_Vtbl {
    pub const fn new<Identity: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>() -> ISpeechGrammarRuleStateTransition_Vtbl {
        unsafe extern "system" fn Type<Identity: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut SpeechGrammarRuleStateTransitionType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRuleStateTransition_Impl::Type(this) {
                Ok(ok__) => {
                    r#type.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Identity: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRuleStateTransition_Impl::Text(this) {
                Ok(ok__) => {
                    text.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rule<Identity: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRuleStateTransition_Impl::Rule(this) {
                Ok(ok__) => {
                    rule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Weight<Identity: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, weight: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRuleStateTransition_Impl::Weight(this) {
                Ok(ok__) => {
                    weight.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyName<Identity: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRuleStateTransition_Impl::PropertyName(this) {
                Ok(ok__) => {
                    propertyname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyId<Identity: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRuleStateTransition_Impl::PropertyId(this) {
                Ok(ok__) => {
                    propertyid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyValue<Identity: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyvalue: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRuleStateTransition_Impl::PropertyValue(this) {
                Ok(ok__) => {
                    propertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextState<Identity: ISpeechGrammarRuleStateTransition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nextstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRuleStateTransition_Impl::NextState(this) {
                Ok(ok__) => {
                    nextstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Type: Type::<Identity, OFFSET>,
            Text: Text::<Identity, OFFSET>,
            Rule: Rule::<Identity, OFFSET>,
            Weight: Weight::<Identity, OFFSET>,
            PropertyName: PropertyName::<Identity, OFFSET>,
            PropertyId: PropertyId::<Identity, OFFSET>,
            PropertyValue: PropertyValue::<Identity, OFFSET>,
            NextState: NextState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechGrammarRuleStateTransition as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechGrammarRuleStateTransitions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: i32) -> windows_core::Result<ISpeechGrammarRuleStateTransition>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechGrammarRuleStateTransitions {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechGrammarRuleStateTransitions_Vtbl {
    pub const fn new<Identity: ISpeechGrammarRuleStateTransitions_Impl, const OFFSET: isize>() -> ISpeechGrammarRuleStateTransitions_Vtbl {
        unsafe extern "system" fn Count<Identity: ISpeechGrammarRuleStateTransitions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRuleStateTransitions_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ISpeechGrammarRuleStateTransitions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, transition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRuleStateTransitions_Impl::Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    transition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISpeechGrammarRuleStateTransitions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumvariant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRuleStateTransitions_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    enumvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechGrammarRuleStateTransitions as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechGrammarRules_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn FindRule(&self, rulenameorid: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISpeechGrammarRule>;
    fn Item(&self, index: i32) -> windows_core::Result<ISpeechGrammarRule>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Dynamic(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Add(&self, rulename: &windows_core::BSTR, attributes: SpeechRuleAttributes, ruleid: i32) -> windows_core::Result<ISpeechGrammarRule>;
    fn Commit(&self) -> windows_core::Result<()>;
    fn CommitAndSave(&self, errortext: *mut windows_core::BSTR, savestream: *mut super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechGrammarRules {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechGrammarRules_Vtbl {
    pub const fn new<Identity: ISpeechGrammarRules_Impl, const OFFSET: isize>() -> ISpeechGrammarRules_Vtbl {
        unsafe extern "system" fn Count<Identity: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRules_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindRule<Identity: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rulenameorid: core::mem::MaybeUninit<super::super::System::Variant::VARIANT>, rule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRules_Impl::FindRule(this, core::mem::transmute(&rulenameorid)) {
                Ok(ok__) => {
                    rule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, rule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRules_Impl::Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    rule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumvariant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRules_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    enumvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dynamic<Identity: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dynamic: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRules_Impl::Dynamic(this) {
                Ok(ok__) => {
                    dynamic.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rulename: core::mem::MaybeUninit<windows_core::BSTR>, attributes: SpeechRuleAttributes, ruleid: i32, rule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechGrammarRules_Impl::Add(this, core::mem::transmute(&rulename), core::mem::transmute_copy(&attributes), core::mem::transmute_copy(&ruleid)) {
                Ok(ok__) => {
                    rule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Identity: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechGrammarRules_Impl::Commit(this).into()
        }
        unsafe extern "system" fn CommitAndSave<Identity: ISpeechGrammarRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errortext: *mut core::mem::MaybeUninit<windows_core::BSTR>, savestream: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechGrammarRules_Impl::CommitAndSave(this, core::mem::transmute_copy(&errortext), core::mem::transmute_copy(&savestream)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            FindRule: FindRule::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Dynamic: Dynamic::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            CommitAndSave: CommitAndSave::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechGrammarRules as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechLexicon_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GenerationId(&self) -> windows_core::Result<i32>;
    fn GetWords(&self, flags: SpeechLexiconType, generationid: *mut i32, words: *mut Option<ISpeechLexiconWords>) -> windows_core::Result<()>;
    fn AddPronunciation(&self, bstrword: &windows_core::BSTR, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddPronunciationByPhoneIds(&self, bstrword: &windows_core::BSTR, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn RemovePronunciation(&self, bstrword: &windows_core::BSTR, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemovePronunciationByPhoneIds(&self, bstrword: &windows_core::BSTR, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn GetPronunciations(&self, bstrword: &windows_core::BSTR, langid: i32, typeflags: SpeechLexiconType) -> windows_core::Result<ISpeechLexiconPronunciations>;
    fn GetGenerationChange(&self, generationid: *mut i32, ppwords: *mut Option<ISpeechLexiconWords>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechLexicon {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechLexicon_Vtbl {
    pub const fn new<Identity: ISpeechLexicon_Impl, const OFFSET: isize>() -> ISpeechLexicon_Vtbl {
        unsafe extern "system" fn GenerationId<Identity: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, generationid: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechLexicon_Impl::GenerationId(this) {
                Ok(ok__) => {
                    generationid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWords<Identity: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: SpeechLexiconType, generationid: *mut i32, words: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechLexicon_Impl::GetWords(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&generationid), core::mem::transmute_copy(&words)).into()
        }
        unsafe extern "system" fn AddPronunciation<Identity: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrword: core::mem::MaybeUninit<windows_core::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechLexicon_Impl::AddPronunciation(this, core::mem::transmute(&bstrword), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&partofspeech), core::mem::transmute(&bstrpronunciation)).into()
        }
        unsafe extern "system" fn AddPronunciationByPhoneIds<Identity: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrword: core::mem::MaybeUninit<windows_core::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechLexicon_Impl::AddPronunciationByPhoneIds(this, core::mem::transmute(&bstrword), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&partofspeech), core::mem::transmute_copy(&phoneids)).into()
        }
        unsafe extern "system" fn RemovePronunciation<Identity: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrword: core::mem::MaybeUninit<windows_core::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, bstrpronunciation: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechLexicon_Impl::RemovePronunciation(this, core::mem::transmute(&bstrword), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&partofspeech), core::mem::transmute(&bstrpronunciation)).into()
        }
        unsafe extern "system" fn RemovePronunciationByPhoneIds<Identity: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrword: core::mem::MaybeUninit<windows_core::BSTR>, langid: i32, partofspeech: SpeechPartOfSpeech, phoneids: *const core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechLexicon_Impl::RemovePronunciationByPhoneIds(this, core::mem::transmute(&bstrword), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&partofspeech), core::mem::transmute_copy(&phoneids)).into()
        }
        unsafe extern "system" fn GetPronunciations<Identity: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrword: core::mem::MaybeUninit<windows_core::BSTR>, langid: i32, typeflags: SpeechLexiconType, pppronunciations: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechLexicon_Impl::GetPronunciations(this, core::mem::transmute(&bstrword), core::mem::transmute_copy(&langid), core::mem::transmute_copy(&typeflags)) {
                Ok(ok__) => {
                    pppronunciations.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGenerationChange<Identity: ISpeechLexicon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, generationid: *mut i32, ppwords: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechLexicon_Impl::GetGenerationChange(this, core::mem::transmute_copy(&generationid), core::mem::transmute_copy(&ppwords)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GenerationId: GenerationId::<Identity, OFFSET>,
            GetWords: GetWords::<Identity, OFFSET>,
            AddPronunciation: AddPronunciation::<Identity, OFFSET>,
            AddPronunciationByPhoneIds: AddPronunciationByPhoneIds::<Identity, OFFSET>,
            RemovePronunciation: RemovePronunciation::<Identity, OFFSET>,
            RemovePronunciationByPhoneIds: RemovePronunciationByPhoneIds::<Identity, OFFSET>,
            GetPronunciations: GetPronunciations::<Identity, OFFSET>,
            GetGenerationChange: GetGenerationChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechLexicon as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechLexiconPronunciation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Type(&self) -> windows_core::Result<SpeechLexiconType>;
    fn LangId(&self) -> windows_core::Result<i32>;
    fn PartOfSpeech(&self) -> windows_core::Result<SpeechPartOfSpeech>;
    fn PhoneIds(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Symbolic(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechLexiconPronunciation {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechLexiconPronunciation_Vtbl {
    pub const fn new<Identity: ISpeechLexiconPronunciation_Impl, const OFFSET: isize>() -> ISpeechLexiconPronunciation_Vtbl {
        unsafe extern "system" fn Type<Identity: ISpeechLexiconPronunciation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lexicontype: *mut SpeechLexiconType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechLexiconPronunciation_Impl::Type(this) {
                Ok(ok__) => {
                    lexicontype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LangId<Identity: ISpeechLexiconPronunciation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechLexiconPronunciation_Impl::LangId(this) {
                Ok(ok__) => {
                    langid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartOfSpeech<Identity: ISpeechLexiconPronunciation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, partofspeech: *mut SpeechPartOfSpeech) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechLexiconPronunciation_Impl::PartOfSpeech(this) {
                Ok(ok__) => {
                    partofspeech.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneIds<Identity: ISpeechLexiconPronunciation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phoneids: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechLexiconPronunciation_Impl::PhoneIds(this) {
                Ok(ok__) => {
                    phoneids.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Symbolic<Identity: ISpeechLexiconPronunciation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, symbolic: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechLexiconPronunciation_Impl::Symbolic(this) {
                Ok(ok__) => {
                    symbolic.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Type: Type::<Identity, OFFSET>,
            LangId: LangId::<Identity, OFFSET>,
            PartOfSpeech: PartOfSpeech::<Identity, OFFSET>,
            PhoneIds: PhoneIds::<Identity, OFFSET>,
            Symbolic: Symbolic::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechLexiconPronunciation as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechLexiconPronunciations_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: i32) -> windows_core::Result<ISpeechLexiconPronunciation>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechLexiconPronunciations {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechLexiconPronunciations_Vtbl {
    pub const fn new<Identity: ISpeechLexiconPronunciations_Impl, const OFFSET: isize>() -> ISpeechLexiconPronunciations_Vtbl {
        unsafe extern "system" fn Count<Identity: ISpeechLexiconPronunciations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechLexiconPronunciations_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ISpeechLexiconPronunciations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pronunciation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechLexiconPronunciations_Impl::Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pronunciation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISpeechLexiconPronunciations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumvariant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechLexiconPronunciations_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    enumvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechLexiconPronunciations as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechLexiconWord_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn LangId(&self) -> windows_core::Result<i32>;
    fn Type(&self) -> windows_core::Result<SpeechWordType>;
    fn Word(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Pronunciations(&self) -> windows_core::Result<ISpeechLexiconPronunciations>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechLexiconWord {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechLexiconWord_Vtbl {
    pub const fn new<Identity: ISpeechLexiconWord_Impl, const OFFSET: isize>() -> ISpeechLexiconWord_Vtbl {
        unsafe extern "system" fn LangId<Identity: ISpeechLexiconWord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, langid: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechLexiconWord_Impl::LangId(this) {
                Ok(ok__) => {
                    langid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ISpeechLexiconWord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wordtype: *mut SpeechWordType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechLexiconWord_Impl::Type(this) {
                Ok(ok__) => {
                    wordtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Word<Identity: ISpeechLexiconWord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, word: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechLexiconWord_Impl::Word(this) {
                Ok(ok__) => {
                    word.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pronunciations<Identity: ISpeechLexiconWord_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pronunciations: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechLexiconWord_Impl::Pronunciations(this) {
                Ok(ok__) => {
                    pronunciations.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            LangId: LangId::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            Word: Word::<Identity, OFFSET>,
            Pronunciations: Pronunciations::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechLexiconWord as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechLexiconWords_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: i32) -> windows_core::Result<ISpeechLexiconWord>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechLexiconWords {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechLexiconWords_Vtbl {
    pub const fn new<Identity: ISpeechLexiconWords_Impl, const OFFSET: isize>() -> ISpeechLexiconWords_Vtbl {
        unsafe extern "system" fn Count<Identity: ISpeechLexiconWords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechLexiconWords_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ISpeechLexiconWords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, word: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechLexiconWords_Impl::Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    word.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISpeechLexiconWords_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumvariant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechLexiconWords_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    enumvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechLexiconWords as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechMMSysAudio_Impl: Sized + ISpeechAudio_Impl {
    fn DeviceId(&self) -> windows_core::Result<i32>;
    fn SetDeviceId(&self, deviceid: i32) -> windows_core::Result<()>;
    fn LineId(&self) -> windows_core::Result<i32>;
    fn SetLineId(&self, lineid: i32) -> windows_core::Result<()>;
    fn MMHandle(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechMMSysAudio {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechMMSysAudio_Vtbl {
    pub const fn new<Identity: ISpeechMMSysAudio_Impl, const OFFSET: isize>() -> ISpeechMMSysAudio_Vtbl {
        unsafe extern "system" fn DeviceId<Identity: ISpeechMMSysAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceid: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechMMSysAudio_Impl::DeviceId(this) {
                Ok(ok__) => {
                    deviceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceId<Identity: ISpeechMMSysAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceid: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechMMSysAudio_Impl::SetDeviceId(this, core::mem::transmute_copy(&deviceid)).into()
        }
        unsafe extern "system" fn LineId<Identity: ISpeechMMSysAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lineid: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechMMSysAudio_Impl::LineId(this) {
                Ok(ok__) => {
                    lineid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLineId<Identity: ISpeechMMSysAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lineid: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechMMSysAudio_Impl::SetLineId(this, core::mem::transmute_copy(&lineid)).into()
        }
        unsafe extern "system" fn MMHandle<Identity: ISpeechMMSysAudio_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechMMSysAudio_Impl::MMHandle(this) {
                Ok(ok__) => {
                    handle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ISpeechAudio_Vtbl::new::<Identity, OFFSET>(),
            DeviceId: DeviceId::<Identity, OFFSET>,
            SetDeviceId: SetDeviceId::<Identity, OFFSET>,
            LineId: LineId::<Identity, OFFSET>,
            SetLineId: SetLineId::<Identity, OFFSET>,
            MMHandle: MMHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechMMSysAudio as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISpeechBaseStream as windows_core::Interface>::IID || iid == &<ISpeechAudio as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechMemoryStream_Impl: Sized + ISpeechBaseStream_Impl {
    fn SetData(&self, data: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn GetData(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechMemoryStream {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechMemoryStream_Vtbl {
    pub const fn new<Identity: ISpeechMemoryStream_Impl, const OFFSET: isize>() -> ISpeechMemoryStream_Vtbl {
        unsafe extern "system" fn SetData<Identity: ISpeechMemoryStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechMemoryStream_Impl::SetData(this, core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn GetData<Identity: ISpeechMemoryStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechMemoryStream_Impl::GetData(this) {
                Ok(ok__) => {
                    pdata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ISpeechBaseStream_Vtbl::new::<Identity, OFFSET>(), SetData: SetData::<Identity, OFFSET>, GetData: GetData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechMemoryStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISpeechBaseStream as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechObjectToken_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DataKey(&self) -> windows_core::Result<ISpeechDataKey>;
    fn Category(&self) -> windows_core::Result<ISpeechObjectTokenCategory>;
    fn GetDescription(&self, locale: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SetId(&self, id: &windows_core::BSTR, categoryid: &windows_core::BSTR, createifnotexist: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetAttribute(&self, attributename: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn CreateInstance(&self, punkouter: Option<&windows_core::IUnknown>, clscontext: SpeechTokenContext) -> windows_core::Result<windows_core::IUnknown>;
    fn Remove(&self, objectstorageclsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetStorageFileName(&self, objectstorageclsid: &windows_core::BSTR, keyname: &windows_core::BSTR, filename: &windows_core::BSTR, folder: SpeechTokenShellFolder) -> windows_core::Result<windows_core::BSTR>;
    fn RemoveStorageFileName(&self, objectstorageclsid: &windows_core::BSTR, keyname: &windows_core::BSTR, deletefile: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IsUISupported(&self, typeofui: &windows_core::BSTR, extradata: *const super::super::System::Variant::VARIANT, object: Option<&windows_core::IUnknown>) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn DisplayUI(&self, hwnd: i32, title: &windows_core::BSTR, typeofui: &windows_core::BSTR, extradata: *const super::super::System::Variant::VARIANT, object: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn MatchesAttributes(&self, attributes: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechObjectToken {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechObjectToken_Vtbl {
    pub const fn new<Identity: ISpeechObjectToken_Impl, const OFFSET: isize>() -> ISpeechObjectToken_Vtbl {
        unsafe extern "system" fn Id<Identity: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechObjectToken_Impl::Id(this) {
                Ok(ok__) => {
                    objectid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataKey<Identity: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datakey: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechObjectToken_Impl::DataKey(this) {
                Ok(ok__) => {
                    datakey.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Category<Identity: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechObjectToken_Impl::Category(this) {
                Ok(ok__) => {
                    category.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Identity: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, locale: i32, description: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechObjectToken_Impl::GetDescription(this, core::mem::transmute_copy(&locale)) {
                Ok(ok__) => {
                    description.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Identity: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: core::mem::MaybeUninit<windows_core::BSTR>, categoryid: core::mem::MaybeUninit<windows_core::BSTR>, createifnotexist: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechObjectToken_Impl::SetId(this, core::mem::transmute(&id), core::mem::transmute(&categoryid), core::mem::transmute_copy(&createifnotexist)).into()
        }
        unsafe extern "system" fn GetAttribute<Identity: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributename: core::mem::MaybeUninit<windows_core::BSTR>, attributevalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechObjectToken_Impl::GetAttribute(this, core::mem::transmute(&attributename)) {
                Ok(ok__) => {
                    attributevalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, clscontext: SpeechTokenContext, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechObjectToken_Impl::CreateInstance(this, windows_core::from_raw_borrowed(&punkouter), core::mem::transmute_copy(&clscontext)) {
                Ok(ok__) => {
                    object.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectstorageclsid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechObjectToken_Impl::Remove(this, core::mem::transmute(&objectstorageclsid)).into()
        }
        unsafe extern "system" fn GetStorageFileName<Identity: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectstorageclsid: core::mem::MaybeUninit<windows_core::BSTR>, keyname: core::mem::MaybeUninit<windows_core::BSTR>, filename: core::mem::MaybeUninit<windows_core::BSTR>, folder: SpeechTokenShellFolder, filepath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechObjectToken_Impl::GetStorageFileName(this, core::mem::transmute(&objectstorageclsid), core::mem::transmute(&keyname), core::mem::transmute(&filename), core::mem::transmute_copy(&folder)) {
                Ok(ok__) => {
                    filepath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStorageFileName<Identity: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectstorageclsid: core::mem::MaybeUninit<windows_core::BSTR>, keyname: core::mem::MaybeUninit<windows_core::BSTR>, deletefile: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechObjectToken_Impl::RemoveStorageFileName(this, core::mem::transmute(&objectstorageclsid), core::mem::transmute(&keyname), core::mem::transmute_copy(&deletefile)).into()
        }
        unsafe extern "system" fn IsUISupported<Identity: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, typeofui: core::mem::MaybeUninit<windows_core::BSTR>, extradata: *const core::mem::MaybeUninit<super::super::System::Variant::VARIANT>, object: *mut core::ffi::c_void, supported: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechObjectToken_Impl::IsUISupported(this, core::mem::transmute(&typeofui), core::mem::transmute_copy(&extradata), windows_core::from_raw_borrowed(&object)) {
                Ok(ok__) => {
                    supported.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Identity: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: i32, title: core::mem::MaybeUninit<windows_core::BSTR>, typeofui: core::mem::MaybeUninit<windows_core::BSTR>, extradata: *const core::mem::MaybeUninit<super::super::System::Variant::VARIANT>, object: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechObjectToken_Impl::DisplayUI(this, core::mem::transmute_copy(&hwnd), core::mem::transmute(&title), core::mem::transmute(&typeofui), core::mem::transmute_copy(&extradata), windows_core::from_raw_borrowed(&object)).into()
        }
        unsafe extern "system" fn MatchesAttributes<Identity: ISpeechObjectToken_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributes: core::mem::MaybeUninit<windows_core::BSTR>, matches: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechObjectToken_Impl::MatchesAttributes(this, core::mem::transmute(&attributes)) {
                Ok(ok__) => {
                    matches.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            DataKey: DataKey::<Identity, OFFSET>,
            Category: Category::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            SetId: SetId::<Identity, OFFSET>,
            GetAttribute: GetAttribute::<Identity, OFFSET>,
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            GetStorageFileName: GetStorageFileName::<Identity, OFFSET>,
            RemoveStorageFileName: RemoveStorageFileName::<Identity, OFFSET>,
            IsUISupported: IsUISupported::<Identity, OFFSET>,
            DisplayUI: DisplayUI::<Identity, OFFSET>,
            MatchesAttributes: MatchesAttributes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechObjectToken as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechObjectTokenCategory_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDefault(&self, tokenid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Default(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetId(&self, id: &windows_core::BSTR, createifnotexist: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetDataKey(&self, location: SpeechDataKeyLocation) -> windows_core::Result<ISpeechDataKey>;
    fn EnumerateTokens(&self, requiredattributes: &windows_core::BSTR, optionalattributes: &windows_core::BSTR) -> windows_core::Result<ISpeechObjectTokens>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechObjectTokenCategory {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechObjectTokenCategory_Vtbl {
    pub const fn new<Identity: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>() -> ISpeechObjectTokenCategory_Vtbl {
        unsafe extern "system" fn Id<Identity: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechObjectTokenCategory_Impl::Id(this) {
                Ok(ok__) => {
                    id.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefault<Identity: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tokenid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechObjectTokenCategory_Impl::SetDefault(this, core::mem::transmute(&tokenid)).into()
        }
        unsafe extern "system" fn Default<Identity: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tokenid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechObjectTokenCategory_Impl::Default(this) {
                Ok(ok__) => {
                    tokenid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Identity: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: core::mem::MaybeUninit<windows_core::BSTR>, createifnotexist: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechObjectTokenCategory_Impl::SetId(this, core::mem::transmute(&id), core::mem::transmute_copy(&createifnotexist)).into()
        }
        unsafe extern "system" fn GetDataKey<Identity: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, location: SpeechDataKeyLocation, datakey: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechObjectTokenCategory_Impl::GetDataKey(this, core::mem::transmute_copy(&location)) {
                Ok(ok__) => {
                    datakey.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateTokens<Identity: ISpeechObjectTokenCategory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requiredattributes: core::mem::MaybeUninit<windows_core::BSTR>, optionalattributes: core::mem::MaybeUninit<windows_core::BSTR>, tokens: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechObjectTokenCategory_Impl::EnumerateTokens(this, core::mem::transmute(&requiredattributes), core::mem::transmute(&optionalattributes)) {
                Ok(ok__) => {
                    tokens.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            SetDefault: SetDefault::<Identity, OFFSET>,
            Default: Default::<Identity, OFFSET>,
            SetId: SetId::<Identity, OFFSET>,
            GetDataKey: GetDataKey::<Identity, OFFSET>,
            EnumerateTokens: EnumerateTokens::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechObjectTokenCategory as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechObjectTokens_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: i32) -> windows_core::Result<ISpeechObjectToken>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechObjectTokens {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechObjectTokens_Vtbl {
    pub const fn new<Identity: ISpeechObjectTokens_Impl, const OFFSET: isize>() -> ISpeechObjectTokens_Vtbl {
        unsafe extern "system" fn Count<Identity: ISpeechObjectTokens_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechObjectTokens_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ISpeechObjectTokens_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, token: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechObjectTokens_Impl::Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    token.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISpeechObjectTokens_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumvariant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechObjectTokens_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppenumvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechObjectTokens as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhoneConverter_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn LanguageId(&self) -> windows_core::Result<i32>;
    fn SetLanguageId(&self, languageid: i32) -> windows_core::Result<()>;
    fn PhoneToId(&self, phonemes: &windows_core::BSTR) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn IdToPhone(&self, idarray: &super::super::System::Variant::VARIANT) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechPhoneConverter {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhoneConverter_Vtbl {
    pub const fn new<Identity: ISpeechPhoneConverter_Impl, const OFFSET: isize>() -> ISpeechPhoneConverter_Vtbl {
        unsafe extern "system" fn LanguageId<Identity: ISpeechPhoneConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, languageid: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhoneConverter_Impl::LanguageId(this) {
                Ok(ok__) => {
                    languageid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguageId<Identity: ISpeechPhoneConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, languageid: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechPhoneConverter_Impl::SetLanguageId(this, core::mem::transmute_copy(&languageid)).into()
        }
        unsafe extern "system" fn PhoneToId<Identity: ISpeechPhoneConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phonemes: core::mem::MaybeUninit<windows_core::BSTR>, idarray: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhoneConverter_Impl::PhoneToId(this, core::mem::transmute(&phonemes)) {
                Ok(ok__) => {
                    idarray.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IdToPhone<Identity: ISpeechPhoneConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idarray: core::mem::MaybeUninit<super::super::System::Variant::VARIANT>, phonemes: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhoneConverter_Impl::IdToPhone(this, core::mem::transmute(&idarray)) {
                Ok(ok__) => {
                    phonemes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            LanguageId: LanguageId::<Identity, OFFSET>,
            SetLanguageId: SetLanguageId::<Identity, OFFSET>,
            PhoneToId: PhoneToId::<Identity, OFFSET>,
            IdToPhone: IdToPhone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechPhoneConverter as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseAlternate_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RecoResult(&self) -> windows_core::Result<ISpeechRecoResult>;
    fn StartElementInResult(&self) -> windows_core::Result<i32>;
    fn NumberOfElementsInResult(&self) -> windows_core::Result<i32>;
    fn PhraseInfo(&self) -> windows_core::Result<ISpeechPhraseInfo>;
    fn Commit(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechPhraseAlternate {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseAlternate_Vtbl {
    pub const fn new<Identity: ISpeechPhraseAlternate_Impl, const OFFSET: isize>() -> ISpeechPhraseAlternate_Vtbl {
        unsafe extern "system" fn RecoResult<Identity: ISpeechPhraseAlternate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recoresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseAlternate_Impl::RecoResult(this) {
                Ok(ok__) => {
                    recoresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartElementInResult<Identity: ISpeechPhraseAlternate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startelement: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseAlternate_Impl::StartElementInResult(this) {
                Ok(ok__) => {
                    startelement.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfElementsInResult<Identity: ISpeechPhraseAlternate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numberofelements: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseAlternate_Impl::NumberOfElementsInResult(this) {
                Ok(ok__) => {
                    numberofelements.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhraseInfo<Identity: ISpeechPhraseAlternate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phraseinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseAlternate_Impl::PhraseInfo(this) {
                Ok(ok__) => {
                    phraseinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Identity: ISpeechPhraseAlternate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechPhraseAlternate_Impl::Commit(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            RecoResult: RecoResult::<Identity, OFFSET>,
            StartElementInResult: StartElementInResult::<Identity, OFFSET>,
            NumberOfElementsInResult: NumberOfElementsInResult::<Identity, OFFSET>,
            PhraseInfo: PhraseInfo::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseAlternate as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseAlternates_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: i32) -> windows_core::Result<ISpeechPhraseAlternate>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechPhraseAlternates {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseAlternates_Vtbl {
    pub const fn new<Identity: ISpeechPhraseAlternates_Impl, const OFFSET: isize>() -> ISpeechPhraseAlternates_Vtbl {
        unsafe extern "system" fn Count<Identity: ISpeechPhraseAlternates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseAlternates_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ISpeechPhraseAlternates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, phrasealternate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseAlternates_Impl::Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    phrasealternate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISpeechPhraseAlternates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumvariant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseAlternates_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    enumvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseAlternates as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseElement_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AudioTimeOffset(&self) -> windows_core::Result<i32>;
    fn AudioSizeTime(&self) -> windows_core::Result<i32>;
    fn AudioStreamOffset(&self) -> windows_core::Result<i32>;
    fn AudioSizeBytes(&self) -> windows_core::Result<i32>;
    fn RetainedStreamOffset(&self) -> windows_core::Result<i32>;
    fn RetainedSizeBytes(&self) -> windows_core::Result<i32>;
    fn DisplayText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn LexicalForm(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Pronunciation(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn DisplayAttributes(&self) -> windows_core::Result<SpeechDisplayAttributes>;
    fn RequiredConfidence(&self) -> windows_core::Result<SpeechEngineConfidence>;
    fn ActualConfidence(&self) -> windows_core::Result<SpeechEngineConfidence>;
    fn EngineConfidence(&self) -> windows_core::Result<f32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechPhraseElement {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseElement_Vtbl {
    pub const fn new<Identity: ISpeechPhraseElement_Impl, const OFFSET: isize>() -> ISpeechPhraseElement_Vtbl {
        unsafe extern "system" fn AudioTimeOffset<Identity: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audiotimeoffset: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseElement_Impl::AudioTimeOffset(this) {
                Ok(ok__) => {
                    audiotimeoffset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioSizeTime<Identity: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audiosizetime: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseElement_Impl::AudioSizeTime(this) {
                Ok(ok__) => {
                    audiosizetime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioStreamOffset<Identity: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audiostreamoffset: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseElement_Impl::AudioStreamOffset(this) {
                Ok(ok__) => {
                    audiostreamoffset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioSizeBytes<Identity: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audiosizebytes: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseElement_Impl::AudioSizeBytes(this) {
                Ok(ok__) => {
                    audiosizebytes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainedStreamOffset<Identity: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retainedstreamoffset: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseElement_Impl::RetainedStreamOffset(this) {
                Ok(ok__) => {
                    retainedstreamoffset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainedSizeBytes<Identity: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retainedsizebytes: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseElement_Impl::RetainedSizeBytes(this) {
                Ok(ok__) => {
                    retainedsizebytes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayText<Identity: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displaytext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseElement_Impl::DisplayText(this) {
                Ok(ok__) => {
                    displaytext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LexicalForm<Identity: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lexicalform: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseElement_Impl::LexicalForm(this) {
                Ok(ok__) => {
                    lexicalform.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pronunciation<Identity: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pronunciation: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseElement_Impl::Pronunciation(this) {
                Ok(ok__) => {
                    pronunciation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayAttributes<Identity: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayattributes: *mut SpeechDisplayAttributes) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseElement_Impl::DisplayAttributes(this) {
                Ok(ok__) => {
                    displayattributes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequiredConfidence<Identity: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requiredconfidence: *mut SpeechEngineConfidence) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseElement_Impl::RequiredConfidence(this) {
                Ok(ok__) => {
                    requiredconfidence.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActualConfidence<Identity: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, actualconfidence: *mut SpeechEngineConfidence) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseElement_Impl::ActualConfidence(this) {
                Ok(ok__) => {
                    actualconfidence.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EngineConfidence<Identity: ISpeechPhraseElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, engineconfidence: *mut f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseElement_Impl::EngineConfidence(this) {
                Ok(ok__) => {
                    engineconfidence.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AudioTimeOffset: AudioTimeOffset::<Identity, OFFSET>,
            AudioSizeTime: AudioSizeTime::<Identity, OFFSET>,
            AudioStreamOffset: AudioStreamOffset::<Identity, OFFSET>,
            AudioSizeBytes: AudioSizeBytes::<Identity, OFFSET>,
            RetainedStreamOffset: RetainedStreamOffset::<Identity, OFFSET>,
            RetainedSizeBytes: RetainedSizeBytes::<Identity, OFFSET>,
            DisplayText: DisplayText::<Identity, OFFSET>,
            LexicalForm: LexicalForm::<Identity, OFFSET>,
            Pronunciation: Pronunciation::<Identity, OFFSET>,
            DisplayAttributes: DisplayAttributes::<Identity, OFFSET>,
            RequiredConfidence: RequiredConfidence::<Identity, OFFSET>,
            ActualConfidence: ActualConfidence::<Identity, OFFSET>,
            EngineConfidence: EngineConfidence::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseElement as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseElements_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: i32) -> windows_core::Result<ISpeechPhraseElement>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechPhraseElements {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseElements_Vtbl {
    pub const fn new<Identity: ISpeechPhraseElements_Impl, const OFFSET: isize>() -> ISpeechPhraseElements_Vtbl {
        unsafe extern "system" fn Count<Identity: ISpeechPhraseElements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseElements_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ISpeechPhraseElements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, element: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseElements_Impl::Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    element.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISpeechPhraseElements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumvariant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseElements_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    enumvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseElements as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn LanguageId(&self) -> windows_core::Result<i32>;
    fn GrammarId(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn StartTime(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AudioStreamPosition(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn AudioSizeBytes(&self) -> windows_core::Result<i32>;
    fn RetainedSizeBytes(&self) -> windows_core::Result<i32>;
    fn AudioSizeTime(&self) -> windows_core::Result<i32>;
    fn Rule(&self) -> windows_core::Result<ISpeechPhraseRule>;
    fn Properties(&self) -> windows_core::Result<ISpeechPhraseProperties>;
    fn Elements(&self) -> windows_core::Result<ISpeechPhraseElements>;
    fn Replacements(&self) -> windows_core::Result<ISpeechPhraseReplacements>;
    fn EngineId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn EnginePrivateData(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SaveToMemory(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetText(&self, startelement: i32, elements: i32, usereplacements: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<windows_core::BSTR>;
    fn GetDisplayAttributes(&self, startelement: i32, elements: i32, usereplacements: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<SpeechDisplayAttributes>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechPhraseInfo {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseInfo_Vtbl {
    pub const fn new<Identity: ISpeechPhraseInfo_Impl, const OFFSET: isize>() -> ISpeechPhraseInfo_Vtbl {
        unsafe extern "system" fn LanguageId<Identity: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, languageid: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseInfo_Impl::LanguageId(this) {
                Ok(ok__) => {
                    languageid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GrammarId<Identity: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grammarid: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseInfo_Impl::GrammarId(this) {
                Ok(ok__) => {
                    grammarid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTime<Identity: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, starttime: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseInfo_Impl::StartTime(this) {
                Ok(ok__) => {
                    starttime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioStreamPosition<Identity: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audiostreamposition: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseInfo_Impl::AudioStreamPosition(this) {
                Ok(ok__) => {
                    audiostreamposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioSizeBytes<Identity: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paudiosizebytes: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseInfo_Impl::AudioSizeBytes(this) {
                Ok(ok__) => {
                    paudiosizebytes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetainedSizeBytes<Identity: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retainedsizebytes: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseInfo_Impl::RetainedSizeBytes(this) {
                Ok(ok__) => {
                    retainedsizebytes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioSizeTime<Identity: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audiosizetime: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseInfo_Impl::AudioSizeTime(this) {
                Ok(ok__) => {
                    audiosizetime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rule<Identity: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseInfo_Impl::Rule(this) {
                Ok(ok__) => {
                    rule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, properties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseInfo_Impl::Properties(this) {
                Ok(ok__) => {
                    properties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Elements<Identity: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, elements: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseInfo_Impl::Elements(this) {
                Ok(ok__) => {
                    elements.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Replacements<Identity: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, replacements: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseInfo_Impl::Replacements(this) {
                Ok(ok__) => {
                    replacements.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EngineId<Identity: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, engineidguid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseInfo_Impl::EngineId(this) {
                Ok(ok__) => {
                    engineidguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnginePrivateData<Identity: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, privatedata: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseInfo_Impl::EnginePrivateData(this) {
                Ok(ok__) => {
                    privatedata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveToMemory<Identity: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phraseblock: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseInfo_Impl::SaveToMemory(this) {
                Ok(ok__) => {
                    phraseblock.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Identity: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startelement: i32, elements: i32, usereplacements: super::super::Foundation::VARIANT_BOOL, text: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseInfo_Impl::GetText(this, core::mem::transmute_copy(&startelement), core::mem::transmute_copy(&elements), core::mem::transmute_copy(&usereplacements)) {
                Ok(ok__) => {
                    text.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayAttributes<Identity: ISpeechPhraseInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startelement: i32, elements: i32, usereplacements: super::super::Foundation::VARIANT_BOOL, displayattributes: *mut SpeechDisplayAttributes) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseInfo_Impl::GetDisplayAttributes(this, core::mem::transmute_copy(&startelement), core::mem::transmute_copy(&elements), core::mem::transmute_copy(&usereplacements)) {
                Ok(ok__) => {
                    displayattributes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            LanguageId: LanguageId::<Identity, OFFSET>,
            GrammarId: GrammarId::<Identity, OFFSET>,
            StartTime: StartTime::<Identity, OFFSET>,
            AudioStreamPosition: AudioStreamPosition::<Identity, OFFSET>,
            AudioSizeBytes: AudioSizeBytes::<Identity, OFFSET>,
            RetainedSizeBytes: RetainedSizeBytes::<Identity, OFFSET>,
            AudioSizeTime: AudioSizeTime::<Identity, OFFSET>,
            Rule: Rule::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            Elements: Elements::<Identity, OFFSET>,
            Replacements: Replacements::<Identity, OFFSET>,
            EngineId: EngineId::<Identity, OFFSET>,
            EnginePrivateData: EnginePrivateData::<Identity, OFFSET>,
            SaveToMemory: SaveToMemory::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            GetDisplayAttributes: GetDisplayAttributes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseInfo as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseInfoBuilder_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RestorePhraseFromMemory(&self, phraseinmemory: *const super::super::System::Variant::VARIANT) -> windows_core::Result<ISpeechPhraseInfo>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechPhraseInfoBuilder {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseInfoBuilder_Vtbl {
    pub const fn new<Identity: ISpeechPhraseInfoBuilder_Impl, const OFFSET: isize>() -> ISpeechPhraseInfoBuilder_Vtbl {
        unsafe extern "system" fn RestorePhraseFromMemory<Identity: ISpeechPhraseInfoBuilder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phraseinmemory: *const core::mem::MaybeUninit<super::super::System::Variant::VARIANT>, phraseinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseInfoBuilder_Impl::RestorePhraseFromMemory(this, core::mem::transmute_copy(&phraseinmemory)) {
                Ok(ok__) => {
                    phraseinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            RestorePhraseFromMemory: RestorePhraseFromMemory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseInfoBuilder as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseProperties_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: i32) -> windows_core::Result<ISpeechPhraseProperty>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechPhraseProperties {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseProperties_Vtbl {
    pub const fn new<Identity: ISpeechPhraseProperties_Impl, const OFFSET: isize>() -> ISpeechPhraseProperties_Vtbl {
        unsafe extern "system" fn Count<Identity: ISpeechPhraseProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseProperties_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ISpeechPhraseProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, property: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseProperties_Impl::Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    property.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISpeechPhraseProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumvariant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseProperties_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    enumvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseProperties as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseProperty_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Id(&self) -> windows_core::Result<i32>;
    fn Value(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn FirstElement(&self) -> windows_core::Result<i32>;
    fn NumberOfElements(&self) -> windows_core::Result<i32>;
    fn EngineConfidence(&self) -> windows_core::Result<f32>;
    fn Confidence(&self) -> windows_core::Result<SpeechEngineConfidence>;
    fn Parent(&self) -> windows_core::Result<ISpeechPhraseProperty>;
    fn Children(&self) -> windows_core::Result<ISpeechPhraseProperties>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechPhraseProperty {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseProperty_Vtbl {
    pub const fn new<Identity: ISpeechPhraseProperty_Impl, const OFFSET: isize>() -> ISpeechPhraseProperty_Vtbl {
        unsafe extern "system" fn Name<Identity: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseProperty_Impl::Name(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseProperty_Impl::Id(this) {
                Ok(ok__) => {
                    id.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Identity: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseProperty_Impl::Value(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstElement<Identity: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, firstelement: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseProperty_Impl::FirstElement(this) {
                Ok(ok__) => {
                    firstelement.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfElements<Identity: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numberofelements: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseProperty_Impl::NumberOfElements(this) {
                Ok(ok__) => {
                    numberofelements.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EngineConfidence<Identity: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, confidence: *mut f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseProperty_Impl::EngineConfidence(this) {
                Ok(ok__) => {
                    confidence.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Confidence<Identity: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, confidence: *mut SpeechEngineConfidence) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseProperty_Impl::Confidence(this) {
                Ok(ok__) => {
                    confidence.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parentproperty: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseProperty_Impl::Parent(this) {
                Ok(ok__) => {
                    parentproperty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Identity: ISpeechPhraseProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, children: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseProperty_Impl::Children(this) {
                Ok(ok__) => {
                    children.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            FirstElement: FirstElement::<Identity, OFFSET>,
            NumberOfElements: NumberOfElements::<Identity, OFFSET>,
            EngineConfidence: EngineConfidence::<Identity, OFFSET>,
            Confidence: Confidence::<Identity, OFFSET>,
            Parent: Parent::<Identity, OFFSET>,
            Children: Children::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseProperty as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseReplacement_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn DisplayAttributes(&self) -> windows_core::Result<SpeechDisplayAttributes>;
    fn Text(&self) -> windows_core::Result<windows_core::BSTR>;
    fn FirstElement(&self) -> windows_core::Result<i32>;
    fn NumberOfElements(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechPhraseReplacement {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseReplacement_Vtbl {
    pub const fn new<Identity: ISpeechPhraseReplacement_Impl, const OFFSET: isize>() -> ISpeechPhraseReplacement_Vtbl {
        unsafe extern "system" fn DisplayAttributes<Identity: ISpeechPhraseReplacement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayattributes: *mut SpeechDisplayAttributes) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseReplacement_Impl::DisplayAttributes(this) {
                Ok(ok__) => {
                    displayattributes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Identity: ISpeechPhraseReplacement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseReplacement_Impl::Text(this) {
                Ok(ok__) => {
                    text.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstElement<Identity: ISpeechPhraseReplacement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, firstelement: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseReplacement_Impl::FirstElement(this) {
                Ok(ok__) => {
                    firstelement.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfElements<Identity: ISpeechPhraseReplacement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numberofelements: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseReplacement_Impl::NumberOfElements(this) {
                Ok(ok__) => {
                    numberofelements.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            DisplayAttributes: DisplayAttributes::<Identity, OFFSET>,
            Text: Text::<Identity, OFFSET>,
            FirstElement: FirstElement::<Identity, OFFSET>,
            NumberOfElements: NumberOfElements::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseReplacement as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseReplacements_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: i32) -> windows_core::Result<ISpeechPhraseReplacement>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechPhraseReplacements {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseReplacements_Vtbl {
    pub const fn new<Identity: ISpeechPhraseReplacements_Impl, const OFFSET: isize>() -> ISpeechPhraseReplacements_Vtbl {
        unsafe extern "system" fn Count<Identity: ISpeechPhraseReplacements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseReplacements_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ISpeechPhraseReplacements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, reps: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseReplacements_Impl::Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    reps.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISpeechPhraseReplacements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumvariant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseReplacements_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    enumvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseReplacements as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseRule_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Id(&self) -> windows_core::Result<i32>;
    fn FirstElement(&self) -> windows_core::Result<i32>;
    fn NumberOfElements(&self) -> windows_core::Result<i32>;
    fn Parent(&self) -> windows_core::Result<ISpeechPhraseRule>;
    fn Children(&self) -> windows_core::Result<ISpeechPhraseRules>;
    fn Confidence(&self) -> windows_core::Result<SpeechEngineConfidence>;
    fn EngineConfidence(&self) -> windows_core::Result<f32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechPhraseRule {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseRule_Vtbl {
    pub const fn new<Identity: ISpeechPhraseRule_Impl, const OFFSET: isize>() -> ISpeechPhraseRule_Vtbl {
        unsafe extern "system" fn Name<Identity: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseRule_Impl::Name(this) {
                Ok(ok__) => {
                    name.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Identity: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseRule_Impl::Id(this) {
                Ok(ok__) => {
                    id.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstElement<Identity: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, firstelement: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseRule_Impl::FirstElement(this) {
                Ok(ok__) => {
                    firstelement.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfElements<Identity: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numberofelements: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseRule_Impl::NumberOfElements(this) {
                Ok(ok__) => {
                    numberofelements.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<Identity: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseRule_Impl::Parent(this) {
                Ok(ok__) => {
                    parent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Children<Identity: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, children: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseRule_Impl::Children(this) {
                Ok(ok__) => {
                    children.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Confidence<Identity: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, actualconfidence: *mut SpeechEngineConfidence) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseRule_Impl::Confidence(this) {
                Ok(ok__) => {
                    actualconfidence.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EngineConfidence<Identity: ISpeechPhraseRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, engineconfidence: *mut f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseRule_Impl::EngineConfidence(this) {
                Ok(ok__) => {
                    engineconfidence.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            FirstElement: FirstElement::<Identity, OFFSET>,
            NumberOfElements: NumberOfElements::<Identity, OFFSET>,
            Parent: Parent::<Identity, OFFSET>,
            Children: Children::<Identity, OFFSET>,
            Confidence: Confidence::<Identity, OFFSET>,
            EngineConfidence: EngineConfidence::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseRule as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechPhraseRules_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Item(&self, index: i32) -> windows_core::Result<ISpeechPhraseRule>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechPhraseRules {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechPhraseRules_Vtbl {
    pub const fn new<Identity: ISpeechPhraseRules_Impl, const OFFSET: isize>() -> ISpeechPhraseRules_Vtbl {
        unsafe extern "system" fn Count<Identity: ISpeechPhraseRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseRules_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ISpeechPhraseRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, rule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseRules_Impl::Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    rule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISpeechPhraseRules_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumvariant: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechPhraseRules_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    enumvariant.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechPhraseRules as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecoContext_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Recognizer(&self) -> windows_core::Result<ISpeechRecognizer>;
    fn AudioInputInterferenceStatus(&self) -> windows_core::Result<SpeechInterference>;
    fn RequestedUIType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn putref_Voice(&self, voice: Option<&ISpeechVoice>) -> windows_core::Result<()>;
    fn Voice(&self) -> windows_core::Result<ISpeechVoice>;
    fn SetAllowVoiceFormatMatchingOnNextSet(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowVoiceFormatMatchingOnNextSet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetVoicePurgeEvent(&self, eventinterest: SpeechRecoEvents) -> windows_core::Result<()>;
    fn VoicePurgeEvent(&self) -> windows_core::Result<SpeechRecoEvents>;
    fn SetEventInterests(&self, eventinterest: SpeechRecoEvents) -> windows_core::Result<()>;
    fn EventInterests(&self) -> windows_core::Result<SpeechRecoEvents>;
    fn SetCmdMaxAlternates(&self, maxalternates: i32) -> windows_core::Result<()>;
    fn CmdMaxAlternates(&self) -> windows_core::Result<i32>;
    fn SetState(&self, state: SpeechRecoContextState) -> windows_core::Result<()>;
    fn State(&self) -> windows_core::Result<SpeechRecoContextState>;
    fn SetRetainedAudio(&self, option: SpeechRetainedAudioOptions) -> windows_core::Result<()>;
    fn RetainedAudio(&self) -> windows_core::Result<SpeechRetainedAudioOptions>;
    fn putref_RetainedAudioFormat(&self, format: Option<&ISpeechAudioFormat>) -> windows_core::Result<()>;
    fn RetainedAudioFormat(&self) -> windows_core::Result<ISpeechAudioFormat>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn CreateGrammar(&self, grammarid: &super::super::System::Variant::VARIANT) -> windows_core::Result<ISpeechRecoGrammar>;
    fn CreateResultFromMemory(&self, resultblock: *const super::super::System::Variant::VARIANT) -> windows_core::Result<ISpeechRecoResult>;
    fn Bookmark(&self, options: SpeechBookmarkOptions, streampos: &super::super::System::Variant::VARIANT, bookmarkid: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn SetAdaptationData(&self, adaptationstring: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechRecoContext {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechRecoContext_Vtbl {
    pub const fn new<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>() -> ISpeechRecoContext_Vtbl {
        unsafe extern "system" fn Recognizer<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recognizer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoContext_Impl::Recognizer(this) {
                Ok(ok__) => {
                    recognizer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioInputInterferenceStatus<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, interference: *mut SpeechInterference) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoContext_Impl::AudioInputInterferenceStatus(this) {
                Ok(ok__) => {
                    interference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedUIType<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uitype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoContext_Impl::RequestedUIType(this) {
                Ok(ok__) => {
                    uitype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Voice<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, voice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoContext_Impl::putref_Voice(this, windows_core::from_raw_borrowed(&voice)).into()
        }
        unsafe extern "system" fn Voice<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, voice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoContext_Impl::Voice(this) {
                Ok(ok__) => {
                    voice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowVoiceFormatMatchingOnNextSet<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoContext_Impl::SetAllowVoiceFormatMatchingOnNextSet(this, core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowVoiceFormatMatchingOnNextSet<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pallow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoContext_Impl::AllowVoiceFormatMatchingOnNextSet(this) {
                Ok(ok__) => {
                    pallow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVoicePurgeEvent<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventinterest: SpeechRecoEvents) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoContext_Impl::SetVoicePurgeEvent(this, core::mem::transmute_copy(&eventinterest)).into()
        }
        unsafe extern "system" fn VoicePurgeEvent<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventinterest: *mut SpeechRecoEvents) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoContext_Impl::VoicePurgeEvent(this) {
                Ok(ok__) => {
                    eventinterest.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventInterests<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventinterest: SpeechRecoEvents) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoContext_Impl::SetEventInterests(this, core::mem::transmute_copy(&eventinterest)).into()
        }
        unsafe extern "system" fn EventInterests<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventinterest: *mut SpeechRecoEvents) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoContext_Impl::EventInterests(this) {
                Ok(ok__) => {
                    eventinterest.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCmdMaxAlternates<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxalternates: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoContext_Impl::SetCmdMaxAlternates(this, core::mem::transmute_copy(&maxalternates)).into()
        }
        unsafe extern "system" fn CmdMaxAlternates<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxalternates: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoContext_Impl::CmdMaxAlternates(this) {
                Ok(ok__) => {
                    maxalternates.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: SpeechRecoContextState) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoContext_Impl::SetState(this, core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn State<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut SpeechRecoContextState) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoContext_Impl::State(this) {
                Ok(ok__) => {
                    state.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRetainedAudio<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: SpeechRetainedAudioOptions) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoContext_Impl::SetRetainedAudio(this, core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn RetainedAudio<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: *mut SpeechRetainedAudioOptions) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoContext_Impl::RetainedAudio(this) {
                Ok(ok__) => {
                    option.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_RetainedAudioFormat<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoContext_Impl::putref_RetainedAudioFormat(this, windows_core::from_raw_borrowed(&format)).into()
        }
        unsafe extern "system" fn RetainedAudioFormat<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoContext_Impl::RetainedAudioFormat(this) {
                Ok(ok__) => {
                    format.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoContext_Impl::Pause(this).into()
        }
        unsafe extern "system" fn Resume<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoContext_Impl::Resume(this).into()
        }
        unsafe extern "system" fn CreateGrammar<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grammarid: core::mem::MaybeUninit<super::super::System::Variant::VARIANT>, grammar: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoContext_Impl::CreateGrammar(this, core::mem::transmute(&grammarid)) {
                Ok(ok__) => {
                    grammar.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResultFromMemory<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resultblock: *const core::mem::MaybeUninit<super::super::System::Variant::VARIANT>, result: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoContext_Impl::CreateResultFromMemory(this, core::mem::transmute_copy(&resultblock)) {
                Ok(ok__) => {
                    result.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bookmark<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: SpeechBookmarkOptions, streampos: core::mem::MaybeUninit<super::super::System::Variant::VARIANT>, bookmarkid: core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoContext_Impl::Bookmark(this, core::mem::transmute_copy(&options), core::mem::transmute(&streampos), core::mem::transmute(&bookmarkid)).into()
        }
        unsafe extern "system" fn SetAdaptationData<Identity: ISpeechRecoContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adaptationstring: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoContext_Impl::SetAdaptationData(this, core::mem::transmute(&adaptationstring)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Recognizer: Recognizer::<Identity, OFFSET>,
            AudioInputInterferenceStatus: AudioInputInterferenceStatus::<Identity, OFFSET>,
            RequestedUIType: RequestedUIType::<Identity, OFFSET>,
            putref_Voice: putref_Voice::<Identity, OFFSET>,
            Voice: Voice::<Identity, OFFSET>,
            SetAllowVoiceFormatMatchingOnNextSet: SetAllowVoiceFormatMatchingOnNextSet::<Identity, OFFSET>,
            AllowVoiceFormatMatchingOnNextSet: AllowVoiceFormatMatchingOnNextSet::<Identity, OFFSET>,
            SetVoicePurgeEvent: SetVoicePurgeEvent::<Identity, OFFSET>,
            VoicePurgeEvent: VoicePurgeEvent::<Identity, OFFSET>,
            SetEventInterests: SetEventInterests::<Identity, OFFSET>,
            EventInterests: EventInterests::<Identity, OFFSET>,
            SetCmdMaxAlternates: SetCmdMaxAlternates::<Identity, OFFSET>,
            CmdMaxAlternates: CmdMaxAlternates::<Identity, OFFSET>,
            SetState: SetState::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            SetRetainedAudio: SetRetainedAudio::<Identity, OFFSET>,
            RetainedAudio: RetainedAudio::<Identity, OFFSET>,
            putref_RetainedAudioFormat: putref_RetainedAudioFormat::<Identity, OFFSET>,
            RetainedAudioFormat: RetainedAudioFormat::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            CreateGrammar: CreateGrammar::<Identity, OFFSET>,
            CreateResultFromMemory: CreateResultFromMemory::<Identity, OFFSET>,
            Bookmark: Bookmark::<Identity, OFFSET>,
            SetAdaptationData: SetAdaptationData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechRecoContext as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecoGrammar_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Id(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn RecoContext(&self) -> windows_core::Result<ISpeechRecoContext>;
    fn SetState(&self, state: SpeechGrammarState) -> windows_core::Result<()>;
    fn State(&self) -> windows_core::Result<SpeechGrammarState>;
    fn Rules(&self) -> windows_core::Result<ISpeechGrammarRules>;
    fn Reset(&self, newlanguage: i32) -> windows_core::Result<()>;
    fn CmdLoadFromFile(&self, filename: &windows_core::BSTR, loadoption: SpeechLoadOption) -> windows_core::Result<()>;
    fn CmdLoadFromObject(&self, classid: &windows_core::BSTR, grammarname: &windows_core::BSTR, loadoption: SpeechLoadOption) -> windows_core::Result<()>;
    fn CmdLoadFromResource(&self, hmodule: i32, resourcename: &super::super::System::Variant::VARIANT, resourcetype: &super::super::System::Variant::VARIANT, languageid: i32, loadoption: SpeechLoadOption) -> windows_core::Result<()>;
    fn CmdLoadFromMemory(&self, grammardata: &super::super::System::Variant::VARIANT, loadoption: SpeechLoadOption) -> windows_core::Result<()>;
    fn CmdLoadFromProprietaryGrammar(&self, proprietaryguid: &windows_core::BSTR, proprietarystring: &windows_core::BSTR, proprietarydata: &super::super::System::Variant::VARIANT, loadoption: SpeechLoadOption) -> windows_core::Result<()>;
    fn CmdSetRuleState(&self, name: &windows_core::BSTR, state: SpeechRuleState) -> windows_core::Result<()>;
    fn CmdSetRuleIdState(&self, ruleid: i32, state: SpeechRuleState) -> windows_core::Result<()>;
    fn DictationLoad(&self, topicname: &windows_core::BSTR, loadoption: SpeechLoadOption) -> windows_core::Result<()>;
    fn DictationUnload(&self) -> windows_core::Result<()>;
    fn DictationSetState(&self, state: SpeechRuleState) -> windows_core::Result<()>;
    fn SetWordSequenceData(&self, text: &windows_core::BSTR, textlength: i32, info: Option<&ISpeechTextSelectionInformation>) -> windows_core::Result<()>;
    fn SetTextSelection(&self, info: Option<&ISpeechTextSelectionInformation>) -> windows_core::Result<()>;
    fn IsPronounceable(&self, word: &windows_core::BSTR) -> windows_core::Result<SpeechWordPronounceable>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechRecoGrammar {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechRecoGrammar_Vtbl {
    pub const fn new<Identity: ISpeechRecoGrammar_Impl, const OFFSET: isize>() -> ISpeechRecoGrammar_Vtbl {
        unsafe extern "system" fn Id<Identity: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoGrammar_Impl::Id(this) {
                Ok(ok__) => {
                    id.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecoContext<Identity: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recocontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoGrammar_Impl::RecoContext(this) {
                Ok(ok__) => {
                    recocontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: SpeechGrammarState) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoGrammar_Impl::SetState(this, core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn State<Identity: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut SpeechGrammarState) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoGrammar_Impl::State(this) {
                Ok(ok__) => {
                    state.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Rules<Identity: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rules: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoGrammar_Impl::Rules(this) {
                Ok(ok__) => {
                    rules.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newlanguage: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoGrammar_Impl::Reset(this, core::mem::transmute_copy(&newlanguage)).into()
        }
        unsafe extern "system" fn CmdLoadFromFile<Identity: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filename: core::mem::MaybeUninit<windows_core::BSTR>, loadoption: SpeechLoadOption) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoGrammar_Impl::CmdLoadFromFile(this, core::mem::transmute(&filename), core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn CmdLoadFromObject<Identity: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classid: core::mem::MaybeUninit<windows_core::BSTR>, grammarname: core::mem::MaybeUninit<windows_core::BSTR>, loadoption: SpeechLoadOption) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoGrammar_Impl::CmdLoadFromObject(this, core::mem::transmute(&classid), core::mem::transmute(&grammarname), core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn CmdLoadFromResource<Identity: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hmodule: i32, resourcename: core::mem::MaybeUninit<super::super::System::Variant::VARIANT>, resourcetype: core::mem::MaybeUninit<super::super::System::Variant::VARIANT>, languageid: i32, loadoption: SpeechLoadOption) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoGrammar_Impl::CmdLoadFromResource(this, core::mem::transmute_copy(&hmodule), core::mem::transmute(&resourcename), core::mem::transmute(&resourcetype), core::mem::transmute_copy(&languageid), core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn CmdLoadFromMemory<Identity: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grammardata: core::mem::MaybeUninit<super::super::System::Variant::VARIANT>, loadoption: SpeechLoadOption) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoGrammar_Impl::CmdLoadFromMemory(this, core::mem::transmute(&grammardata), core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn CmdLoadFromProprietaryGrammar<Identity: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, proprietaryguid: core::mem::MaybeUninit<windows_core::BSTR>, proprietarystring: core::mem::MaybeUninit<windows_core::BSTR>, proprietarydata: core::mem::MaybeUninit<super::super::System::Variant::VARIANT>, loadoption: SpeechLoadOption) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoGrammar_Impl::CmdLoadFromProprietaryGrammar(this, core::mem::transmute(&proprietaryguid), core::mem::transmute(&proprietarystring), core::mem::transmute(&proprietarydata), core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn CmdSetRuleState<Identity: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, state: SpeechRuleState) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoGrammar_Impl::CmdSetRuleState(this, core::mem::transmute(&name), core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn CmdSetRuleIdState<Identity: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ruleid: i32, state: SpeechRuleState) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoGrammar_Impl::CmdSetRuleIdState(this, core::mem::transmute_copy(&ruleid), core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn DictationLoad<Identity: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, topicname: core::mem::MaybeUninit<windows_core::BSTR>, loadoption: SpeechLoadOption) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoGrammar_Impl::DictationLoad(this, core::mem::transmute(&topicname), core::mem::transmute_copy(&loadoption)).into()
        }
        unsafe extern "system" fn DictationUnload<Identity: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoGrammar_Impl::DictationUnload(this).into()
        }
        unsafe extern "system" fn DictationSetState<Identity: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: SpeechRuleState) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoGrammar_Impl::DictationSetState(this, core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn SetWordSequenceData<Identity: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: core::mem::MaybeUninit<windows_core::BSTR>, textlength: i32, info: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoGrammar_Impl::SetWordSequenceData(this, core::mem::transmute(&text), core::mem::transmute_copy(&textlength), windows_core::from_raw_borrowed(&info)).into()
        }
        unsafe extern "system" fn SetTextSelection<Identity: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, info: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoGrammar_Impl::SetTextSelection(this, windows_core::from_raw_borrowed(&info)).into()
        }
        unsafe extern "system" fn IsPronounceable<Identity: ISpeechRecoGrammar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, word: core::mem::MaybeUninit<windows_core::BSTR>, wordpronounceable: *mut SpeechWordPronounceable) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoGrammar_Impl::IsPronounceable(this, core::mem::transmute(&word)) {
                Ok(ok__) => {
                    wordpronounceable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            RecoContext: RecoContext::<Identity, OFFSET>,
            SetState: SetState::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            Rules: Rules::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            CmdLoadFromFile: CmdLoadFromFile::<Identity, OFFSET>,
            CmdLoadFromObject: CmdLoadFromObject::<Identity, OFFSET>,
            CmdLoadFromResource: CmdLoadFromResource::<Identity, OFFSET>,
            CmdLoadFromMemory: CmdLoadFromMemory::<Identity, OFFSET>,
            CmdLoadFromProprietaryGrammar: CmdLoadFromProprietaryGrammar::<Identity, OFFSET>,
            CmdSetRuleState: CmdSetRuleState::<Identity, OFFSET>,
            CmdSetRuleIdState: CmdSetRuleIdState::<Identity, OFFSET>,
            DictationLoad: DictationLoad::<Identity, OFFSET>,
            DictationUnload: DictationUnload::<Identity, OFFSET>,
            DictationSetState: DictationSetState::<Identity, OFFSET>,
            SetWordSequenceData: SetWordSequenceData::<Identity, OFFSET>,
            SetTextSelection: SetTextSelection::<Identity, OFFSET>,
            IsPronounceable: IsPronounceable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechRecoGrammar as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecoResult_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RecoContext(&self) -> windows_core::Result<ISpeechRecoContext>;
    fn Times(&self) -> windows_core::Result<ISpeechRecoResultTimes>;
    fn putref_AudioFormat(&self, format: Option<&ISpeechAudioFormat>) -> windows_core::Result<()>;
    fn AudioFormat(&self) -> windows_core::Result<ISpeechAudioFormat>;
    fn PhraseInfo(&self) -> windows_core::Result<ISpeechPhraseInfo>;
    fn Alternates(&self, requestcount: i32, startelement: i32, elements: i32) -> windows_core::Result<ISpeechPhraseAlternates>;
    fn Audio(&self, startelement: i32, elements: i32) -> windows_core::Result<ISpeechMemoryStream>;
    fn SpeakAudio(&self, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags) -> windows_core::Result<i32>;
    fn SaveToMemory(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn DiscardResultInfo(&self, valuetypes: SpeechDiscardType) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechRecoResult {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechRecoResult_Vtbl {
    pub const fn new<Identity: ISpeechRecoResult_Impl, const OFFSET: isize>() -> ISpeechRecoResult_Vtbl {
        unsafe extern "system" fn RecoContext<Identity: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recocontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoResult_Impl::RecoContext(this) {
                Ok(ok__) => {
                    recocontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Times<Identity: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, times: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoResult_Impl::Times(this) {
                Ok(ok__) => {
                    times.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioFormat<Identity: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoResult_Impl::putref_AudioFormat(this, windows_core::from_raw_borrowed(&format)).into()
        }
        unsafe extern "system" fn AudioFormat<Identity: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoResult_Impl::AudioFormat(this) {
                Ok(ok__) => {
                    format.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhraseInfo<Identity: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phraseinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoResult_Impl::PhraseInfo(this) {
                Ok(ok__) => {
                    phraseinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Alternates<Identity: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestcount: i32, startelement: i32, elements: i32, alternates: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoResult_Impl::Alternates(this, core::mem::transmute_copy(&requestcount), core::mem::transmute_copy(&startelement), core::mem::transmute_copy(&elements)) {
                Ok(ok__) => {
                    alternates.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Audio<Identity: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startelement: i32, elements: i32, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoResult_Impl::Audio(this, core::mem::transmute_copy(&startelement), core::mem::transmute_copy(&elements)) {
                Ok(ok__) => {
                    stream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakAudio<Identity: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoResult_Impl::SpeakAudio(this, core::mem::transmute_copy(&startelement), core::mem::transmute_copy(&elements), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    streamnumber.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveToMemory<Identity: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resultblock: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoResult_Impl::SaveToMemory(this) {
                Ok(ok__) => {
                    resultblock.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscardResultInfo<Identity: ISpeechRecoResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, valuetypes: SpeechDiscardType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoResult_Impl::DiscardResultInfo(this, core::mem::transmute_copy(&valuetypes)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            RecoContext: RecoContext::<Identity, OFFSET>,
            Times: Times::<Identity, OFFSET>,
            putref_AudioFormat: putref_AudioFormat::<Identity, OFFSET>,
            AudioFormat: AudioFormat::<Identity, OFFSET>,
            PhraseInfo: PhraseInfo::<Identity, OFFSET>,
            Alternates: Alternates::<Identity, OFFSET>,
            Audio: Audio::<Identity, OFFSET>,
            SpeakAudio: SpeakAudio::<Identity, OFFSET>,
            SaveToMemory: SaveToMemory::<Identity, OFFSET>,
            DiscardResultInfo: DiscardResultInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechRecoResult as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecoResult2_Impl: Sized + ISpeechRecoResult_Impl {
    fn SetTextFeedback(&self, feedback: &windows_core::BSTR, wassuccessful: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechRecoResult2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechRecoResult2_Vtbl {
    pub const fn new<Identity: ISpeechRecoResult2_Impl, const OFFSET: isize>() -> ISpeechRecoResult2_Vtbl {
        unsafe extern "system" fn SetTextFeedback<Identity: ISpeechRecoResult2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, feedback: core::mem::MaybeUninit<windows_core::BSTR>, wassuccessful: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoResult2_Impl::SetTextFeedback(this, core::mem::transmute(&feedback), core::mem::transmute_copy(&wassuccessful)).into()
        }
        Self { base__: ISpeechRecoResult_Vtbl::new::<Identity, OFFSET>(), SetTextFeedback: SetTextFeedback::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechRecoResult2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISpeechRecoResult as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecoResultDispatch_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn RecoContext(&self) -> windows_core::Result<ISpeechRecoContext>;
    fn Times(&self) -> windows_core::Result<ISpeechRecoResultTimes>;
    fn putref_AudioFormat(&self, format: Option<&ISpeechAudioFormat>) -> windows_core::Result<()>;
    fn AudioFormat(&self) -> windows_core::Result<ISpeechAudioFormat>;
    fn PhraseInfo(&self) -> windows_core::Result<ISpeechPhraseInfo>;
    fn Alternates(&self, requestcount: i32, startelement: i32, elements: i32) -> windows_core::Result<ISpeechPhraseAlternates>;
    fn Audio(&self, startelement: i32, elements: i32) -> windows_core::Result<ISpeechMemoryStream>;
    fn SpeakAudio(&self, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags) -> windows_core::Result<i32>;
    fn SaveToMemory(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn DiscardResultInfo(&self, valuetypes: SpeechDiscardType) -> windows_core::Result<()>;
    fn GetXMLResult(&self, options: SPXMLRESULTOPTIONS) -> windows_core::Result<windows_core::BSTR>;
    fn GetXMLErrorInfo(&self, linenumber: *mut i32, scriptline: *mut windows_core::BSTR, source: *mut windows_core::BSTR, description: *mut windows_core::BSTR, resultcode: *mut windows_core::HRESULT, iserror: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetTextFeedback(&self, feedback: &windows_core::BSTR, wassuccessful: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechRecoResultDispatch {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechRecoResultDispatch_Vtbl {
    pub const fn new<Identity: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>() -> ISpeechRecoResultDispatch_Vtbl {
        unsafe extern "system" fn RecoContext<Identity: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recocontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoResultDispatch_Impl::RecoContext(this) {
                Ok(ok__) => {
                    recocontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Times<Identity: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, times: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoResultDispatch_Impl::Times(this) {
                Ok(ok__) => {
                    times.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioFormat<Identity: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoResultDispatch_Impl::putref_AudioFormat(this, windows_core::from_raw_borrowed(&format)).into()
        }
        unsafe extern "system" fn AudioFormat<Identity: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoResultDispatch_Impl::AudioFormat(this) {
                Ok(ok__) => {
                    format.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhraseInfo<Identity: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phraseinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoResultDispatch_Impl::PhraseInfo(this) {
                Ok(ok__) => {
                    phraseinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Alternates<Identity: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestcount: i32, startelement: i32, elements: i32, alternates: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoResultDispatch_Impl::Alternates(this, core::mem::transmute_copy(&requestcount), core::mem::transmute_copy(&startelement), core::mem::transmute_copy(&elements)) {
                Ok(ok__) => {
                    alternates.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Audio<Identity: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startelement: i32, elements: i32, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoResultDispatch_Impl::Audio(this, core::mem::transmute_copy(&startelement), core::mem::transmute_copy(&elements)) {
                Ok(ok__) => {
                    stream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakAudio<Identity: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startelement: i32, elements: i32, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoResultDispatch_Impl::SpeakAudio(this, core::mem::transmute_copy(&startelement), core::mem::transmute_copy(&elements), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    streamnumber.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveToMemory<Identity: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resultblock: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoResultDispatch_Impl::SaveToMemory(this) {
                Ok(ok__) => {
                    resultblock.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscardResultInfo<Identity: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, valuetypes: SpeechDiscardType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoResultDispatch_Impl::DiscardResultInfo(this, core::mem::transmute_copy(&valuetypes)).into()
        }
        unsafe extern "system" fn GetXMLResult<Identity: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: SPXMLRESULTOPTIONS, presult: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoResultDispatch_Impl::GetXMLResult(this, core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    presult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXMLErrorInfo<Identity: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linenumber: *mut i32, scriptline: *mut core::mem::MaybeUninit<windows_core::BSTR>, source: *mut core::mem::MaybeUninit<windows_core::BSTR>, description: *mut core::mem::MaybeUninit<windows_core::BSTR>, resultcode: *mut windows_core::HRESULT, iserror: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoResultDispatch_Impl::GetXMLErrorInfo(this, core::mem::transmute_copy(&linenumber), core::mem::transmute_copy(&scriptline), core::mem::transmute_copy(&source), core::mem::transmute_copy(&description), core::mem::transmute_copy(&resultcode), core::mem::transmute_copy(&iserror)).into()
        }
        unsafe extern "system" fn SetTextFeedback<Identity: ISpeechRecoResultDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, feedback: core::mem::MaybeUninit<windows_core::BSTR>, wassuccessful: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecoResultDispatch_Impl::SetTextFeedback(this, core::mem::transmute(&feedback), core::mem::transmute_copy(&wassuccessful)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            RecoContext: RecoContext::<Identity, OFFSET>,
            Times: Times::<Identity, OFFSET>,
            putref_AudioFormat: putref_AudioFormat::<Identity, OFFSET>,
            AudioFormat: AudioFormat::<Identity, OFFSET>,
            PhraseInfo: PhraseInfo::<Identity, OFFSET>,
            Alternates: Alternates::<Identity, OFFSET>,
            Audio: Audio::<Identity, OFFSET>,
            SpeakAudio: SpeakAudio::<Identity, OFFSET>,
            SaveToMemory: SaveToMemory::<Identity, OFFSET>,
            DiscardResultInfo: DiscardResultInfo::<Identity, OFFSET>,
            GetXMLResult: GetXMLResult::<Identity, OFFSET>,
            GetXMLErrorInfo: GetXMLErrorInfo::<Identity, OFFSET>,
            SetTextFeedback: SetTextFeedback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechRecoResultDispatch as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecoResultTimes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StreamTime(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Length(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn TickCount(&self) -> windows_core::Result<i32>;
    fn OffsetFromStart(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechRecoResultTimes {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechRecoResultTimes_Vtbl {
    pub const fn new<Identity: ISpeechRecoResultTimes_Impl, const OFFSET: isize>() -> ISpeechRecoResultTimes_Vtbl {
        unsafe extern "system" fn StreamTime<Identity: ISpeechRecoResultTimes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, time: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoResultTimes_Impl::StreamTime(this) {
                Ok(ok__) => {
                    time.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Identity: ISpeechRecoResultTimes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoResultTimes_Impl::Length(this) {
                Ok(ok__) => {
                    length.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TickCount<Identity: ISpeechRecoResultTimes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tickcount: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoResultTimes_Impl::TickCount(this) {
                Ok(ok__) => {
                    tickcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OffsetFromStart<Identity: ISpeechRecoResultTimes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offsetfromstart: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecoResultTimes_Impl::OffsetFromStart(this) {
                Ok(ok__) => {
                    offsetfromstart.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StreamTime: StreamTime::<Identity, OFFSET>,
            Length: Length::<Identity, OFFSET>,
            TickCount: TickCount::<Identity, OFFSET>,
            OffsetFromStart: OffsetFromStart::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechRecoResultTimes as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecognizer_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn putref_Recognizer(&self, recognizer: Option<&ISpeechObjectToken>) -> windows_core::Result<()>;
    fn Recognizer(&self) -> windows_core::Result<ISpeechObjectToken>;
    fn SetAllowAudioInputFormatChangesOnNextSet(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowAudioInputFormatChangesOnNextSet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn putref_AudioInput(&self, audioinput: Option<&ISpeechObjectToken>) -> windows_core::Result<()>;
    fn AudioInput(&self) -> windows_core::Result<ISpeechObjectToken>;
    fn putref_AudioInputStream(&self, audioinputstream: Option<&ISpeechBaseStream>) -> windows_core::Result<()>;
    fn AudioInputStream(&self) -> windows_core::Result<ISpeechBaseStream>;
    fn IsShared(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetState(&self, state: SpeechRecognizerState) -> windows_core::Result<()>;
    fn State(&self) -> windows_core::Result<SpeechRecognizerState>;
    fn Status(&self) -> windows_core::Result<ISpeechRecognizerStatus>;
    fn putref_Profile(&self, profile: Option<&ISpeechObjectToken>) -> windows_core::Result<()>;
    fn Profile(&self) -> windows_core::Result<ISpeechObjectToken>;
    fn EmulateRecognition(&self, textelements: &super::super::System::Variant::VARIANT, elementdisplayattributes: *const super::super::System::Variant::VARIANT, languageid: i32) -> windows_core::Result<()>;
    fn CreateRecoContext(&self) -> windows_core::Result<ISpeechRecoContext>;
    fn GetFormat(&self, r#type: SpeechFormatType) -> windows_core::Result<ISpeechAudioFormat>;
    fn SetPropertyNumber(&self, name: &windows_core::BSTR, value: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetPropertyNumber(&self, name: &windows_core::BSTR, value: *mut i32, supported: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetPropertyString(&self, name: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetPropertyString(&self, name: &windows_core::BSTR, value: *mut windows_core::BSTR, supported: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn IsUISupported(&self, typeofui: &windows_core::BSTR, extradata: *const super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn DisplayUI(&self, hwndparent: i32, title: &windows_core::BSTR, typeofui: &windows_core::BSTR, extradata: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn GetRecognizers(&self, requiredattributes: &windows_core::BSTR, optionalattributes: &windows_core::BSTR) -> windows_core::Result<ISpeechObjectTokens>;
    fn GetAudioInputs(&self, requiredattributes: &windows_core::BSTR, optionalattributes: &windows_core::BSTR) -> windows_core::Result<ISpeechObjectTokens>;
    fn GetProfiles(&self, requiredattributes: &windows_core::BSTR, optionalattributes: &windows_core::BSTR) -> windows_core::Result<ISpeechObjectTokens>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechRecognizer {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechRecognizer_Vtbl {
    pub const fn new<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>() -> ISpeechRecognizer_Vtbl {
        unsafe extern "system" fn putref_Recognizer<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recognizer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecognizer_Impl::putref_Recognizer(this, windows_core::from_raw_borrowed(&recognizer)).into()
        }
        unsafe extern "system" fn Recognizer<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recognizer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecognizer_Impl::Recognizer(this) {
                Ok(ok__) => {
                    recognizer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowAudioInputFormatChangesOnNextSet<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecognizer_Impl::SetAllowAudioInputFormatChangesOnNextSet(this, core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowAudioInputFormatChangesOnNextSet<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecognizer_Impl::AllowAudioInputFormatChangesOnNextSet(this) {
                Ok(ok__) => {
                    allow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioInput<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audioinput: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecognizer_Impl::putref_AudioInput(this, windows_core::from_raw_borrowed(&audioinput)).into()
        }
        unsafe extern "system" fn AudioInput<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audioinput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecognizer_Impl::AudioInput(this) {
                Ok(ok__) => {
                    audioinput.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioInputStream<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audioinputstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecognizer_Impl::putref_AudioInputStream(this, windows_core::from_raw_borrowed(&audioinputstream)).into()
        }
        unsafe extern "system" fn AudioInputStream<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audioinputstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecognizer_Impl::AudioInputStream(this) {
                Ok(ok__) => {
                    audioinputstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsShared<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, shared: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecognizer_Impl::IsShared(this) {
                Ok(ok__) => {
                    shared.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: SpeechRecognizerState) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecognizer_Impl::SetState(this, core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn State<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut SpeechRecognizerState) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecognizer_Impl::State(this) {
                Ok(ok__) => {
                    state.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecognizer_Impl::Status(this) {
                Ok(ok__) => {
                    status.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Profile<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecognizer_Impl::putref_Profile(this, windows_core::from_raw_borrowed(&profile)).into()
        }
        unsafe extern "system" fn Profile<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecognizer_Impl::Profile(this) {
                Ok(ok__) => {
                    profile.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EmulateRecognition<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textelements: core::mem::MaybeUninit<super::super::System::Variant::VARIANT>, elementdisplayattributes: *const core::mem::MaybeUninit<super::super::System::Variant::VARIANT>, languageid: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecognizer_Impl::EmulateRecognition(this, core::mem::transmute(&textelements), core::mem::transmute_copy(&elementdisplayattributes), core::mem::transmute_copy(&languageid)).into()
        }
        unsafe extern "system" fn CreateRecoContext<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecognizer_Impl::CreateRecoContext(this) {
                Ok(ok__) => {
                    newcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: SpeechFormatType, format: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecognizer_Impl::GetFormat(this, core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    format.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyNumber<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, value: i32, supported: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecognizer_Impl::SetPropertyNumber(this, core::mem::transmute(&name), core::mem::transmute_copy(&value)) {
                Ok(ok__) => {
                    supported.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyNumber<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, value: *mut i32, supported: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecognizer_Impl::GetPropertyNumber(this, core::mem::transmute(&name), core::mem::transmute_copy(&value), core::mem::transmute_copy(&supported)).into()
        }
        unsafe extern "system" fn SetPropertyString<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, value: core::mem::MaybeUninit<windows_core::BSTR>, supported: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecognizer_Impl::SetPropertyString(this, core::mem::transmute(&name), core::mem::transmute(&value)) {
                Ok(ok__) => {
                    supported.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyString<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, value: *mut core::mem::MaybeUninit<windows_core::BSTR>, supported: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecognizer_Impl::GetPropertyString(this, core::mem::transmute(&name), core::mem::transmute_copy(&value), core::mem::transmute_copy(&supported)).into()
        }
        unsafe extern "system" fn IsUISupported<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, typeofui: core::mem::MaybeUninit<windows_core::BSTR>, extradata: *const core::mem::MaybeUninit<super::super::System::Variant::VARIANT>, supported: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecognizer_Impl::IsUISupported(this, core::mem::transmute(&typeofui), core::mem::transmute_copy(&extradata)) {
                Ok(ok__) => {
                    supported.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: i32, title: core::mem::MaybeUninit<windows_core::BSTR>, typeofui: core::mem::MaybeUninit<windows_core::BSTR>, extradata: *const core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechRecognizer_Impl::DisplayUI(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute(&title), core::mem::transmute(&typeofui), core::mem::transmute_copy(&extradata)).into()
        }
        unsafe extern "system" fn GetRecognizers<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requiredattributes: core::mem::MaybeUninit<windows_core::BSTR>, optionalattributes: core::mem::MaybeUninit<windows_core::BSTR>, objecttokens: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecognizer_Impl::GetRecognizers(this, core::mem::transmute(&requiredattributes), core::mem::transmute(&optionalattributes)) {
                Ok(ok__) => {
                    objecttokens.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioInputs<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requiredattributes: core::mem::MaybeUninit<windows_core::BSTR>, optionalattributes: core::mem::MaybeUninit<windows_core::BSTR>, objecttokens: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecognizer_Impl::GetAudioInputs(this, core::mem::transmute(&requiredattributes), core::mem::transmute(&optionalattributes)) {
                Ok(ok__) => {
                    objecttokens.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfiles<Identity: ISpeechRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requiredattributes: core::mem::MaybeUninit<windows_core::BSTR>, optionalattributes: core::mem::MaybeUninit<windows_core::BSTR>, objecttokens: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecognizer_Impl::GetProfiles(this, core::mem::transmute(&requiredattributes), core::mem::transmute(&optionalattributes)) {
                Ok(ok__) => {
                    objecttokens.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            putref_Recognizer: putref_Recognizer::<Identity, OFFSET>,
            Recognizer: Recognizer::<Identity, OFFSET>,
            SetAllowAudioInputFormatChangesOnNextSet: SetAllowAudioInputFormatChangesOnNextSet::<Identity, OFFSET>,
            AllowAudioInputFormatChangesOnNextSet: AllowAudioInputFormatChangesOnNextSet::<Identity, OFFSET>,
            putref_AudioInput: putref_AudioInput::<Identity, OFFSET>,
            AudioInput: AudioInput::<Identity, OFFSET>,
            putref_AudioInputStream: putref_AudioInputStream::<Identity, OFFSET>,
            AudioInputStream: AudioInputStream::<Identity, OFFSET>,
            IsShared: IsShared::<Identity, OFFSET>,
            SetState: SetState::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            putref_Profile: putref_Profile::<Identity, OFFSET>,
            Profile: Profile::<Identity, OFFSET>,
            EmulateRecognition: EmulateRecognition::<Identity, OFFSET>,
            CreateRecoContext: CreateRecoContext::<Identity, OFFSET>,
            GetFormat: GetFormat::<Identity, OFFSET>,
            SetPropertyNumber: SetPropertyNumber::<Identity, OFFSET>,
            GetPropertyNumber: GetPropertyNumber::<Identity, OFFSET>,
            SetPropertyString: SetPropertyString::<Identity, OFFSET>,
            GetPropertyString: GetPropertyString::<Identity, OFFSET>,
            IsUISupported: IsUISupported::<Identity, OFFSET>,
            DisplayUI: DisplayUI::<Identity, OFFSET>,
            GetRecognizers: GetRecognizers::<Identity, OFFSET>,
            GetAudioInputs: GetAudioInputs::<Identity, OFFSET>,
            GetProfiles: GetProfiles::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechRecognizer as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechRecognizerStatus_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AudioStatus(&self) -> windows_core::Result<ISpeechAudioStatus>;
    fn CurrentStreamPosition(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn CurrentStreamNumber(&self) -> windows_core::Result<i32>;
    fn NumberOfActiveRules(&self) -> windows_core::Result<i32>;
    fn ClsidEngine(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SupportedLanguages(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechRecognizerStatus {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechRecognizerStatus_Vtbl {
    pub const fn new<Identity: ISpeechRecognizerStatus_Impl, const OFFSET: isize>() -> ISpeechRecognizerStatus_Vtbl {
        unsafe extern "system" fn AudioStatus<Identity: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audiostatus: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecognizerStatus_Impl::AudioStatus(this) {
                Ok(ok__) => {
                    audiostatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentStreamPosition<Identity: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcurrentstreampos: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecognizerStatus_Impl::CurrentStreamPosition(this) {
                Ok(ok__) => {
                    pcurrentstreampos.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentStreamNumber<Identity: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamnumber: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecognizerStatus_Impl::CurrentStreamNumber(this) {
                Ok(ok__) => {
                    streamnumber.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfActiveRules<Identity: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numberofactiverules: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecognizerStatus_Impl::NumberOfActiveRules(this) {
                Ok(ok__) => {
                    numberofactiverules.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClsidEngine<Identity: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsidengine: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecognizerStatus_Impl::ClsidEngine(this) {
                Ok(ok__) => {
                    clsidengine.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedLanguages<Identity: ISpeechRecognizerStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedlanguages: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechRecognizerStatus_Impl::SupportedLanguages(this) {
                Ok(ok__) => {
                    supportedlanguages.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AudioStatus: AudioStatus::<Identity, OFFSET>,
            CurrentStreamPosition: CurrentStreamPosition::<Identity, OFFSET>,
            CurrentStreamNumber: CurrentStreamNumber::<Identity, OFFSET>,
            NumberOfActiveRules: NumberOfActiveRules::<Identity, OFFSET>,
            ClsidEngine: ClsidEngine::<Identity, OFFSET>,
            SupportedLanguages: SupportedLanguages::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechRecognizerStatus as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechResourceLoader_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn LoadResource(&self, bstrresourceuri: &windows_core::BSTR, falwaysreload: super::super::Foundation::VARIANT_BOOL, pstream: *mut Option<windows_core::IUnknown>, pbstrmimetype: *mut windows_core::BSTR, pfmodified: *mut super::super::Foundation::VARIANT_BOOL, pbstrredirecturl: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetLocalCopy(&self, bstrresourceuri: &windows_core::BSTR, pbstrlocalpath: *mut windows_core::BSTR, pbstrmimetype: *mut windows_core::BSTR, pbstrredirecturl: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn ReleaseLocalCopy(&self, pbstrlocalpath: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechResourceLoader {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechResourceLoader_Vtbl {
    pub const fn new<Identity: ISpeechResourceLoader_Impl, const OFFSET: isize>() -> ISpeechResourceLoader_Vtbl {
        unsafe extern "system" fn LoadResource<Identity: ISpeechResourceLoader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrresourceuri: core::mem::MaybeUninit<windows_core::BSTR>, falwaysreload: super::super::Foundation::VARIANT_BOOL, pstream: *mut *mut core::ffi::c_void, pbstrmimetype: *mut core::mem::MaybeUninit<windows_core::BSTR>, pfmodified: *mut super::super::Foundation::VARIANT_BOOL, pbstrredirecturl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechResourceLoader_Impl::LoadResource(this, core::mem::transmute(&bstrresourceuri), core::mem::transmute_copy(&falwaysreload), core::mem::transmute_copy(&pstream), core::mem::transmute_copy(&pbstrmimetype), core::mem::transmute_copy(&pfmodified), core::mem::transmute_copy(&pbstrredirecturl)).into()
        }
        unsafe extern "system" fn GetLocalCopy<Identity: ISpeechResourceLoader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrresourceuri: core::mem::MaybeUninit<windows_core::BSTR>, pbstrlocalpath: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrmimetype: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrredirecturl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechResourceLoader_Impl::GetLocalCopy(this, core::mem::transmute(&bstrresourceuri), core::mem::transmute_copy(&pbstrlocalpath), core::mem::transmute_copy(&pbstrmimetype), core::mem::transmute_copy(&pbstrredirecturl)).into()
        }
        unsafe extern "system" fn ReleaseLocalCopy<Identity: ISpeechResourceLoader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlocalpath: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechResourceLoader_Impl::ReleaseLocalCopy(this, core::mem::transmute(&pbstrlocalpath)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            LoadResource: LoadResource::<Identity, OFFSET>,
            GetLocalCopy: GetLocalCopy::<Identity, OFFSET>,
            ReleaseLocalCopy: ReleaseLocalCopy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechResourceLoader as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechTextSelectionInformation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetActiveOffset(&self, activeoffset: i32) -> windows_core::Result<()>;
    fn ActiveOffset(&self) -> windows_core::Result<i32>;
    fn SetActiveLength(&self, activelength: i32) -> windows_core::Result<()>;
    fn ActiveLength(&self) -> windows_core::Result<i32>;
    fn SetSelectionOffset(&self, selectionoffset: i32) -> windows_core::Result<()>;
    fn SelectionOffset(&self) -> windows_core::Result<i32>;
    fn SetSelectionLength(&self, selectionlength: i32) -> windows_core::Result<()>;
    fn SelectionLength(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechTextSelectionInformation {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechTextSelectionInformation_Vtbl {
    pub const fn new<Identity: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>() -> ISpeechTextSelectionInformation_Vtbl {
        unsafe extern "system" fn SetActiveOffset<Identity: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activeoffset: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechTextSelectionInformation_Impl::SetActiveOffset(this, core::mem::transmute_copy(&activeoffset)).into()
        }
        unsafe extern "system" fn ActiveOffset<Identity: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activeoffset: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechTextSelectionInformation_Impl::ActiveOffset(this) {
                Ok(ok__) => {
                    activeoffset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveLength<Identity: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activelength: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechTextSelectionInformation_Impl::SetActiveLength(this, core::mem::transmute_copy(&activelength)).into()
        }
        unsafe extern "system" fn ActiveLength<Identity: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activelength: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechTextSelectionInformation_Impl::ActiveLength(this) {
                Ok(ok__) => {
                    activelength.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionOffset<Identity: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, selectionoffset: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechTextSelectionInformation_Impl::SetSelectionOffset(this, core::mem::transmute_copy(&selectionoffset)).into()
        }
        unsafe extern "system" fn SelectionOffset<Identity: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, selectionoffset: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechTextSelectionInformation_Impl::SelectionOffset(this) {
                Ok(ok__) => {
                    selectionoffset.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionLength<Identity: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, selectionlength: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechTextSelectionInformation_Impl::SetSelectionLength(this, core::mem::transmute_copy(&selectionlength)).into()
        }
        unsafe extern "system" fn SelectionLength<Identity: ISpeechTextSelectionInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, selectionlength: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechTextSelectionInformation_Impl::SelectionLength(this) {
                Ok(ok__) => {
                    selectionlength.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetActiveOffset: SetActiveOffset::<Identity, OFFSET>,
            ActiveOffset: ActiveOffset::<Identity, OFFSET>,
            SetActiveLength: SetActiveLength::<Identity, OFFSET>,
            ActiveLength: ActiveLength::<Identity, OFFSET>,
            SetSelectionOffset: SetSelectionOffset::<Identity, OFFSET>,
            SelectionOffset: SelectionOffset::<Identity, OFFSET>,
            SetSelectionLength: SetSelectionLength::<Identity, OFFSET>,
            SelectionLength: SelectionLength::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechTextSelectionInformation as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechVoice_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Status(&self) -> windows_core::Result<ISpeechVoiceStatus>;
    fn Voice(&self) -> windows_core::Result<ISpeechObjectToken>;
    fn putref_Voice(&self, voice: Option<&ISpeechObjectToken>) -> windows_core::Result<()>;
    fn AudioOutput(&self) -> windows_core::Result<ISpeechObjectToken>;
    fn putref_AudioOutput(&self, audiooutput: Option<&ISpeechObjectToken>) -> windows_core::Result<()>;
    fn AudioOutputStream(&self) -> windows_core::Result<ISpeechBaseStream>;
    fn putref_AudioOutputStream(&self, audiooutputstream: Option<&ISpeechBaseStream>) -> windows_core::Result<()>;
    fn Rate(&self) -> windows_core::Result<i32>;
    fn SetRate(&self, rate: i32) -> windows_core::Result<()>;
    fn Volume(&self) -> windows_core::Result<i32>;
    fn SetVolume(&self, volume: i32) -> windows_core::Result<()>;
    fn SetAllowAudioOutputFormatChangesOnNextSet(&self, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AllowAudioOutputFormatChangesOnNextSet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EventInterests(&self) -> windows_core::Result<SpeechVoiceEvents>;
    fn SetEventInterests(&self, eventinterestflags: SpeechVoiceEvents) -> windows_core::Result<()>;
    fn SetPriority(&self, priority: SpeechVoicePriority) -> windows_core::Result<()>;
    fn Priority(&self) -> windows_core::Result<SpeechVoicePriority>;
    fn SetAlertBoundary(&self, boundary: SpeechVoiceEvents) -> windows_core::Result<()>;
    fn AlertBoundary(&self) -> windows_core::Result<SpeechVoiceEvents>;
    fn SetSynchronousSpeakTimeout(&self, mstimeout: i32) -> windows_core::Result<()>;
    fn SynchronousSpeakTimeout(&self) -> windows_core::Result<i32>;
    fn Speak(&self, text: &windows_core::BSTR, flags: SpeechVoiceSpeakFlags) -> windows_core::Result<i32>;
    fn SpeakStream(&self, stream: Option<&ISpeechBaseStream>, flags: SpeechVoiceSpeakFlags) -> windows_core::Result<i32>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Resume(&self) -> windows_core::Result<()>;
    fn Skip(&self, r#type: &windows_core::BSTR, numitems: i32) -> windows_core::Result<i32>;
    fn GetVoices(&self, requiredattributes: &windows_core::BSTR, optionalattributes: &windows_core::BSTR) -> windows_core::Result<ISpeechObjectTokens>;
    fn GetAudioOutputs(&self, requiredattributes: &windows_core::BSTR, optionalattributes: &windows_core::BSTR) -> windows_core::Result<ISpeechObjectTokens>;
    fn WaitUntilDone(&self, mstimeout: i32) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SpeakCompleteEvent(&self) -> windows_core::Result<i32>;
    fn IsUISupported(&self, typeofui: &windows_core::BSTR, extradata: *const super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn DisplayUI(&self, hwndparent: i32, title: &windows_core::BSTR, typeofui: &windows_core::BSTR, extradata: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechVoice {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechVoice_Vtbl {
    pub const fn new<Identity: ISpeechVoice_Impl, const OFFSET: isize>() -> ISpeechVoice_Vtbl {
        unsafe extern "system" fn Status<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoice_Impl::Status(this) {
                Ok(ok__) => {
                    status.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Voice<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, voice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoice_Impl::Voice(this) {
                Ok(ok__) => {
                    voice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_Voice<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, voice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechVoice_Impl::putref_Voice(this, windows_core::from_raw_borrowed(&voice)).into()
        }
        unsafe extern "system" fn AudioOutput<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audiooutput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoice_Impl::AudioOutput(this) {
                Ok(ok__) => {
                    audiooutput.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioOutput<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audiooutput: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechVoice_Impl::putref_AudioOutput(this, windows_core::from_raw_borrowed(&audiooutput)).into()
        }
        unsafe extern "system" fn AudioOutputStream<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audiooutputstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoice_Impl::AudioOutputStream(this) {
                Ok(ok__) => {
                    audiooutputstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_AudioOutputStream<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audiooutputstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechVoice_Impl::putref_AudioOutputStream(this, windows_core::from_raw_borrowed(&audiooutputstream)).into()
        }
        unsafe extern "system" fn Rate<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rate: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoice_Impl::Rate(this) {
                Ok(ok__) => {
                    rate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRate<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rate: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechVoice_Impl::SetRate(this, core::mem::transmute_copy(&rate)).into()
        }
        unsafe extern "system" fn Volume<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, volume: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoice_Impl::Volume(this) {
                Ok(ok__) => {
                    volume.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, volume: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechVoice_Impl::SetVolume(this, core::mem::transmute_copy(&volume)).into()
        }
        unsafe extern "system" fn SetAllowAudioOutputFormatChangesOnNextSet<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechVoice_Impl::SetAllowAudioOutputFormatChangesOnNextSet(this, core::mem::transmute_copy(&allow)).into()
        }
        unsafe extern "system" fn AllowAudioOutputFormatChangesOnNextSet<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allow: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoice_Impl::AllowAudioOutputFormatChangesOnNextSet(this) {
                Ok(ok__) => {
                    allow.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventInterests<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventinterestflags: *mut SpeechVoiceEvents) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoice_Impl::EventInterests(this) {
                Ok(ok__) => {
                    eventinterestflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventInterests<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventinterestflags: SpeechVoiceEvents) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechVoice_Impl::SetEventInterests(this, core::mem::transmute_copy(&eventinterestflags)).into()
        }
        unsafe extern "system" fn SetPriority<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, priority: SpeechVoicePriority) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechVoice_Impl::SetPriority(this, core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn Priority<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, priority: *mut SpeechVoicePriority) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoice_Impl::Priority(this) {
                Ok(ok__) => {
                    priority.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlertBoundary<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, boundary: SpeechVoiceEvents) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechVoice_Impl::SetAlertBoundary(this, core::mem::transmute_copy(&boundary)).into()
        }
        unsafe extern "system" fn AlertBoundary<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, boundary: *mut SpeechVoiceEvents) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoice_Impl::AlertBoundary(this) {
                Ok(ok__) => {
                    boundary.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSynchronousSpeakTimeout<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mstimeout: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechVoice_Impl::SetSynchronousSpeakTimeout(this, core::mem::transmute_copy(&mstimeout)).into()
        }
        unsafe extern "system" fn SynchronousSpeakTimeout<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mstimeout: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoice_Impl::SynchronousSpeakTimeout(this) {
                Ok(ok__) => {
                    mstimeout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Speak<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: core::mem::MaybeUninit<windows_core::BSTR>, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoice_Impl::Speak(this, core::mem::transmute(&text), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    streamnumber.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakStream<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, flags: SpeechVoiceSpeakFlags, streamnumber: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoice_Impl::SpeakStream(this, windows_core::from_raw_borrowed(&stream), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    streamnumber.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pause<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechVoice_Impl::Pause(this).into()
        }
        unsafe extern "system" fn Resume<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechVoice_Impl::Resume(this).into()
        }
        unsafe extern "system" fn Skip<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: core::mem::MaybeUninit<windows_core::BSTR>, numitems: i32, numskipped: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoice_Impl::Skip(this, core::mem::transmute(&r#type), core::mem::transmute_copy(&numitems)) {
                Ok(ok__) => {
                    numskipped.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVoices<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requiredattributes: core::mem::MaybeUninit<windows_core::BSTR>, optionalattributes: core::mem::MaybeUninit<windows_core::BSTR>, objecttokens: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoice_Impl::GetVoices(this, core::mem::transmute(&requiredattributes), core::mem::transmute(&optionalattributes)) {
                Ok(ok__) => {
                    objecttokens.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioOutputs<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requiredattributes: core::mem::MaybeUninit<windows_core::BSTR>, optionalattributes: core::mem::MaybeUninit<windows_core::BSTR>, objecttokens: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoice_Impl::GetAudioOutputs(this, core::mem::transmute(&requiredattributes), core::mem::transmute(&optionalattributes)) {
                Ok(ok__) => {
                    objecttokens.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitUntilDone<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mstimeout: i32, done: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoice_Impl::WaitUntilDone(this, core::mem::transmute_copy(&mstimeout)) {
                Ok(ok__) => {
                    done.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeakCompleteEvent<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoice_Impl::SpeakCompleteEvent(this) {
                Ok(ok__) => {
                    handle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUISupported<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, typeofui: core::mem::MaybeUninit<windows_core::BSTR>, extradata: *const core::mem::MaybeUninit<super::super::System::Variant::VARIANT>, supported: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoice_Impl::IsUISupported(this, core::mem::transmute(&typeofui), core::mem::transmute_copy(&extradata)) {
                Ok(ok__) => {
                    supported.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUI<Identity: ISpeechVoice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: i32, title: core::mem::MaybeUninit<windows_core::BSTR>, typeofui: core::mem::MaybeUninit<windows_core::BSTR>, extradata: *const core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechVoice_Impl::DisplayUI(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute(&title), core::mem::transmute(&typeofui), core::mem::transmute_copy(&extradata)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Status: Status::<Identity, OFFSET>,
            Voice: Voice::<Identity, OFFSET>,
            putref_Voice: putref_Voice::<Identity, OFFSET>,
            AudioOutput: AudioOutput::<Identity, OFFSET>,
            putref_AudioOutput: putref_AudioOutput::<Identity, OFFSET>,
            AudioOutputStream: AudioOutputStream::<Identity, OFFSET>,
            putref_AudioOutputStream: putref_AudioOutputStream::<Identity, OFFSET>,
            Rate: Rate::<Identity, OFFSET>,
            SetRate: SetRate::<Identity, OFFSET>,
            Volume: Volume::<Identity, OFFSET>,
            SetVolume: SetVolume::<Identity, OFFSET>,
            SetAllowAudioOutputFormatChangesOnNextSet: SetAllowAudioOutputFormatChangesOnNextSet::<Identity, OFFSET>,
            AllowAudioOutputFormatChangesOnNextSet: AllowAudioOutputFormatChangesOnNextSet::<Identity, OFFSET>,
            EventInterests: EventInterests::<Identity, OFFSET>,
            SetEventInterests: SetEventInterests::<Identity, OFFSET>,
            SetPriority: SetPriority::<Identity, OFFSET>,
            Priority: Priority::<Identity, OFFSET>,
            SetAlertBoundary: SetAlertBoundary::<Identity, OFFSET>,
            AlertBoundary: AlertBoundary::<Identity, OFFSET>,
            SetSynchronousSpeakTimeout: SetSynchronousSpeakTimeout::<Identity, OFFSET>,
            SynchronousSpeakTimeout: SynchronousSpeakTimeout::<Identity, OFFSET>,
            Speak: Speak::<Identity, OFFSET>,
            SpeakStream: SpeakStream::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Resume: Resume::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            GetVoices: GetVoices::<Identity, OFFSET>,
            GetAudioOutputs: GetAudioOutputs::<Identity, OFFSET>,
            WaitUntilDone: WaitUntilDone::<Identity, OFFSET>,
            SpeakCompleteEvent: SpeakCompleteEvent::<Identity, OFFSET>,
            IsUISupported: IsUISupported::<Identity, OFFSET>,
            DisplayUI: DisplayUI::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechVoice as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechVoiceStatus_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CurrentStreamNumber(&self) -> windows_core::Result<i32>;
    fn LastStreamNumberQueued(&self) -> windows_core::Result<i32>;
    fn LastHResult(&self) -> windows_core::Result<i32>;
    fn RunningState(&self) -> windows_core::Result<SpeechRunState>;
    fn InputWordPosition(&self) -> windows_core::Result<i32>;
    fn InputWordLength(&self) -> windows_core::Result<i32>;
    fn InputSentencePosition(&self) -> windows_core::Result<i32>;
    fn InputSentenceLength(&self) -> windows_core::Result<i32>;
    fn LastBookmark(&self) -> windows_core::Result<windows_core::BSTR>;
    fn LastBookmarkId(&self) -> windows_core::Result<i32>;
    fn PhonemeId(&self) -> windows_core::Result<i16>;
    fn VisemeId(&self) -> windows_core::Result<i16>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechVoiceStatus {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechVoiceStatus_Vtbl {
    pub const fn new<Identity: ISpeechVoiceStatus_Impl, const OFFSET: isize>() -> ISpeechVoiceStatus_Vtbl {
        unsafe extern "system" fn CurrentStreamNumber<Identity: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamnumber: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoiceStatus_Impl::CurrentStreamNumber(this) {
                Ok(ok__) => {
                    streamnumber.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastStreamNumberQueued<Identity: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamnumber: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoiceStatus_Impl::LastStreamNumberQueued(this) {
                Ok(ok__) => {
                    streamnumber.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastHResult<Identity: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoiceStatus_Impl::LastHResult(this) {
                Ok(ok__) => {
                    hresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunningState<Identity: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: *mut SpeechRunState) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoiceStatus_Impl::RunningState(this) {
                Ok(ok__) => {
                    state.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputWordPosition<Identity: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, position: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoiceStatus_Impl::InputWordPosition(this) {
                Ok(ok__) => {
                    position.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputWordLength<Identity: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoiceStatus_Impl::InputWordLength(this) {
                Ok(ok__) => {
                    length.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputSentencePosition<Identity: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, position: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoiceStatus_Impl::InputSentencePosition(this) {
                Ok(ok__) => {
                    position.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputSentenceLength<Identity: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoiceStatus_Impl::InputSentenceLength(this) {
                Ok(ok__) => {
                    length.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBookmark<Identity: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bookmark: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoiceStatus_Impl::LastBookmark(this) {
                Ok(ok__) => {
                    bookmark.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBookmarkId<Identity: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bookmarkid: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoiceStatus_Impl::LastBookmarkId(this) {
                Ok(ok__) => {
                    bookmarkid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhonemeId<Identity: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phoneid: *mut i16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoiceStatus_Impl::PhonemeId(this) {
                Ok(ok__) => {
                    phoneid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VisemeId<Identity: ISpeechVoiceStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, visemeid: *mut i16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechVoiceStatus_Impl::VisemeId(this) {
                Ok(ok__) => {
                    visemeid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CurrentStreamNumber: CurrentStreamNumber::<Identity, OFFSET>,
            LastStreamNumberQueued: LastStreamNumberQueued::<Identity, OFFSET>,
            LastHResult: LastHResult::<Identity, OFFSET>,
            RunningState: RunningState::<Identity, OFFSET>,
            InputWordPosition: InputWordPosition::<Identity, OFFSET>,
            InputWordLength: InputWordLength::<Identity, OFFSET>,
            InputSentencePosition: InputSentencePosition::<Identity, OFFSET>,
            InputSentenceLength: InputSentenceLength::<Identity, OFFSET>,
            LastBookmark: LastBookmark::<Identity, OFFSET>,
            LastBookmarkId: LastBookmarkId::<Identity, OFFSET>,
            PhonemeId: PhonemeId::<Identity, OFFSET>,
            VisemeId: VisemeId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechVoiceStatus as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechWaveFormatEx_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn FormatTag(&self) -> windows_core::Result<i16>;
    fn SetFormatTag(&self, formattag: i16) -> windows_core::Result<()>;
    fn Channels(&self) -> windows_core::Result<i16>;
    fn SetChannels(&self, channels: i16) -> windows_core::Result<()>;
    fn SamplesPerSec(&self) -> windows_core::Result<i32>;
    fn SetSamplesPerSec(&self, samplespersec: i32) -> windows_core::Result<()>;
    fn AvgBytesPerSec(&self) -> windows_core::Result<i32>;
    fn SetAvgBytesPerSec(&self, avgbytespersec: i32) -> windows_core::Result<()>;
    fn BlockAlign(&self) -> windows_core::Result<i16>;
    fn SetBlockAlign(&self, blockalign: i16) -> windows_core::Result<()>;
    fn BitsPerSample(&self) -> windows_core::Result<i16>;
    fn SetBitsPerSample(&self, bitspersample: i16) -> windows_core::Result<()>;
    fn ExtraData(&self) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetExtraData(&self, extradata: &super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechWaveFormatEx {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechWaveFormatEx_Vtbl {
    pub const fn new<Identity: ISpeechWaveFormatEx_Impl, const OFFSET: isize>() -> ISpeechWaveFormatEx_Vtbl {
        unsafe extern "system" fn FormatTag<Identity: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, formattag: *mut i16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechWaveFormatEx_Impl::FormatTag(this) {
                Ok(ok__) => {
                    formattag.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormatTag<Identity: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, formattag: i16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechWaveFormatEx_Impl::SetFormatTag(this, core::mem::transmute_copy(&formattag)).into()
        }
        unsafe extern "system" fn Channels<Identity: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, channels: *mut i16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechWaveFormatEx_Impl::Channels(this) {
                Ok(ok__) => {
                    channels.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannels<Identity: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, channels: i16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechWaveFormatEx_Impl::SetChannels(this, core::mem::transmute_copy(&channels)).into()
        }
        unsafe extern "system" fn SamplesPerSec<Identity: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, samplespersec: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechWaveFormatEx_Impl::SamplesPerSec(this) {
                Ok(ok__) => {
                    samplespersec.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSamplesPerSec<Identity: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, samplespersec: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechWaveFormatEx_Impl::SetSamplesPerSec(this, core::mem::transmute_copy(&samplespersec)).into()
        }
        unsafe extern "system" fn AvgBytesPerSec<Identity: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, avgbytespersec: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechWaveFormatEx_Impl::AvgBytesPerSec(this) {
                Ok(ok__) => {
                    avgbytespersec.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAvgBytesPerSec<Identity: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, avgbytespersec: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechWaveFormatEx_Impl::SetAvgBytesPerSec(this, core::mem::transmute_copy(&avgbytespersec)).into()
        }
        unsafe extern "system" fn BlockAlign<Identity: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blockalign: *mut i16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechWaveFormatEx_Impl::BlockAlign(this) {
                Ok(ok__) => {
                    blockalign.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBlockAlign<Identity: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blockalign: i16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechWaveFormatEx_Impl::SetBlockAlign(this, core::mem::transmute_copy(&blockalign)).into()
        }
        unsafe extern "system" fn BitsPerSample<Identity: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitspersample: *mut i16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechWaveFormatEx_Impl::BitsPerSample(this) {
                Ok(ok__) => {
                    bitspersample.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitsPerSample<Identity: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitspersample: i16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechWaveFormatEx_Impl::SetBitsPerSample(this, core::mem::transmute_copy(&bitspersample)).into()
        }
        unsafe extern "system" fn ExtraData<Identity: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, extradata: *mut core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechWaveFormatEx_Impl::ExtraData(this) {
                Ok(ok__) => {
                    extradata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtraData<Identity: ISpeechWaveFormatEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, extradata: core::mem::MaybeUninit<super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechWaveFormatEx_Impl::SetExtraData(this, core::mem::transmute(&extradata)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            FormatTag: FormatTag::<Identity, OFFSET>,
            SetFormatTag: SetFormatTag::<Identity, OFFSET>,
            Channels: Channels::<Identity, OFFSET>,
            SetChannels: SetChannels::<Identity, OFFSET>,
            SamplesPerSec: SamplesPerSec::<Identity, OFFSET>,
            SetSamplesPerSec: SetSamplesPerSec::<Identity, OFFSET>,
            AvgBytesPerSec: AvgBytesPerSec::<Identity, OFFSET>,
            SetAvgBytesPerSec: SetAvgBytesPerSec::<Identity, OFFSET>,
            BlockAlign: BlockAlign::<Identity, OFFSET>,
            SetBlockAlign: SetBlockAlign::<Identity, OFFSET>,
            BitsPerSample: BitsPerSample::<Identity, OFFSET>,
            SetBitsPerSample: SetBitsPerSample::<Identity, OFFSET>,
            ExtraData: ExtraData::<Identity, OFFSET>,
            SetExtraData: SetExtraData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechWaveFormatEx as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISpeechXMLRecoResult_Impl: Sized + ISpeechRecoResult_Impl {
    fn GetXMLResult(&self, options: SPXMLRESULTOPTIONS) -> windows_core::Result<windows_core::BSTR>;
    fn GetXMLErrorInfo(&self, linenumber: *mut i32, scriptline: *mut windows_core::BSTR, source: *mut windows_core::BSTR, description: *mut windows_core::BSTR, resultcode: *mut i32, iserror: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISpeechXMLRecoResult {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISpeechXMLRecoResult_Vtbl {
    pub const fn new<Identity: ISpeechXMLRecoResult_Impl, const OFFSET: isize>() -> ISpeechXMLRecoResult_Vtbl {
        unsafe extern "system" fn GetXMLResult<Identity: ISpeechXMLRecoResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: SPXMLRESULTOPTIONS, presult: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISpeechXMLRecoResult_Impl::GetXMLResult(this, core::mem::transmute_copy(&options)) {
                Ok(ok__) => {
                    presult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXMLErrorInfo<Identity: ISpeechXMLRecoResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, linenumber: *mut i32, scriptline: *mut core::mem::MaybeUninit<windows_core::BSTR>, source: *mut core::mem::MaybeUninit<windows_core::BSTR>, description: *mut core::mem::MaybeUninit<windows_core::BSTR>, resultcode: *mut i32, iserror: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISpeechXMLRecoResult_Impl::GetXMLErrorInfo(this, core::mem::transmute_copy(&linenumber), core::mem::transmute_copy(&scriptline), core::mem::transmute_copy(&source), core::mem::transmute_copy(&description), core::mem::transmute_copy(&resultcode), core::mem::transmute_copy(&iserror)).into()
        }
        Self {
            base__: ISpeechRecoResult_Vtbl::new::<Identity, OFFSET>(),
            GetXMLResult: GetXMLResult::<Identity, OFFSET>,
            GetXMLErrorInfo: GetXMLErrorInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpeechXMLRecoResult as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ISpeechRecoResult as windows_core::Interface>::IID
    }
}
pub trait _ISpPrivateEngineCall_Impl: Sized + windows_core::IUnknownImpl {
    fn CallEngine(&self, pcallframe: *mut core::ffi::c_void, ulcallframesize: u32) -> windows_core::Result<()>;
    fn CallEngineEx(&self, pinframe: *const core::ffi::c_void, ulinframesize: u32, ppcomemoutframe: *mut *mut core::ffi::c_void, puloutframesize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for _ISpPrivateEngineCall {}
impl _ISpPrivateEngineCall_Vtbl {
    pub const fn new<Identity: _ISpPrivateEngineCall_Impl, const OFFSET: isize>() -> _ISpPrivateEngineCall_Vtbl {
        unsafe extern "system" fn CallEngine<Identity: _ISpPrivateEngineCall_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallframe: *mut core::ffi::c_void, ulcallframesize: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISpPrivateEngineCall_Impl::CallEngine(this, core::mem::transmute_copy(&pcallframe), core::mem::transmute_copy(&ulcallframesize)).into()
        }
        unsafe extern "system" fn CallEngineEx<Identity: _ISpPrivateEngineCall_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinframe: *const core::ffi::c_void, ulinframesize: u32, ppcomemoutframe: *mut *mut core::ffi::c_void, puloutframesize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            _ISpPrivateEngineCall_Impl::CallEngineEx(this, core::mem::transmute_copy(&pinframe), core::mem::transmute_copy(&ulinframesize), core::mem::transmute_copy(&ppcomemoutframe), core::mem::transmute_copy(&puloutframesize)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CallEngine: CallEngine::<Identity, OFFSET>,
            CallEngineEx: CallEngineEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_ISpPrivateEngineCall as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _ISpeechRecoContextEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for _ISpeechRecoContextEvents {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl _ISpeechRecoContextEvents_Vtbl {
    pub const fn new<Identity: _ISpeechRecoContextEvents_Impl, const OFFSET: isize>() -> _ISpeechRecoContextEvents_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_ISpeechRecoContextEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _ISpeechVoiceEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for _ISpeechVoiceEvents {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl _ISpeechVoiceEvents_Vtbl {
    pub const fn new<Identity: _ISpeechVoiceEvents_Impl, const OFFSET: isize>() -> _ISpeechVoiceEvents_Vtbl {
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<_ISpeechVoiceEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
