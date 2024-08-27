#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IFilter_Impl: Sized + windows_core::IUnknownImpl {
    fn Init(&self, grfflags: u32, cattributes: u32, aattributes: *const FULLPROPSPEC, pflags: *mut u32) -> i32;
    fn GetChunk(&self, pstat: *mut STAT_CHUNK) -> i32;
    fn GetText(&self, pcwcbuffer: *mut u32, awcbuffer: windows_core::PWSTR) -> i32;
    fn GetValue(&self, pppropvalue: *mut *mut windows_core::PROPVARIANT) -> i32;
    fn BindRegion(&self, origpos: &FILTERREGION, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> i32;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for IFilter {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IFilter_Vtbl {
    pub const fn new<Identity: IFilter_Impl, const OFFSET: isize>() -> IFilter_Vtbl {
        unsafe extern "system" fn Init<Identity: IFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfflags: u32, cattributes: u32, aattributes: *const FULLPROPSPEC, pflags: *mut u32) -> i32 {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFilter_Impl::Init(this, core::mem::transmute_copy(&grfflags), core::mem::transmute_copy(&cattributes), core::mem::transmute_copy(&aattributes), core::mem::transmute_copy(&pflags))
        }
        unsafe extern "system" fn GetChunk<Identity: IFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstat: *mut STAT_CHUNK) -> i32 {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFilter_Impl::GetChunk(this, core::mem::transmute_copy(&pstat))
        }
        unsafe extern "system" fn GetText<Identity: IFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcwcbuffer: *mut u32, awcbuffer: windows_core::PWSTR) -> i32 {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFilter_Impl::GetText(this, core::mem::transmute_copy(&pcwcbuffer), core::mem::transmute_copy(&awcbuffer))
        }
        unsafe extern "system" fn GetValue<Identity: IFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropvalue: *mut *mut windows_core::PROPVARIANT) -> i32 {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFilter_Impl::GetValue(this, core::mem::transmute_copy(&pppropvalue))
        }
        unsafe extern "system" fn BindRegion<Identity: IFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, origpos: FILTERREGION, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> i32 {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFilter_Impl::BindRegion(this, core::mem::transmute(&origpos), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            GetChunk: GetChunk::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            BindRegion: BindRegion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFilter as windows_core::Interface>::IID
    }
}
pub trait IPhraseSink_Impl: Sized + windows_core::IUnknownImpl {
    fn PutSmallPhrase(&self, pwcnoun: &windows_core::PCWSTR, cwcnoun: u32, pwcmodifier: &windows_core::PCWSTR, cwcmodifier: u32, ulattachmenttype: u32) -> windows_core::Result<()>;
    fn PutPhrase(&self, pwcphrase: &windows_core::PCWSTR, cwcphrase: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPhraseSink {}
impl IPhraseSink_Vtbl {
    pub const fn new<Identity: IPhraseSink_Impl, const OFFSET: isize>() -> IPhraseSink_Vtbl {
        unsafe extern "system" fn PutSmallPhrase<Identity: IPhraseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcnoun: windows_core::PCWSTR, cwcnoun: u32, pwcmodifier: windows_core::PCWSTR, cwcmodifier: u32, ulattachmenttype: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPhraseSink_Impl::PutSmallPhrase(this, core::mem::transmute(&pwcnoun), core::mem::transmute_copy(&cwcnoun), core::mem::transmute(&pwcmodifier), core::mem::transmute_copy(&cwcmodifier), core::mem::transmute_copy(&ulattachmenttype)).into()
        }
        unsafe extern "system" fn PutPhrase<Identity: IPhraseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcphrase: windows_core::PCWSTR, cwcphrase: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPhraseSink_Impl::PutPhrase(this, core::mem::transmute(&pwcphrase), core::mem::transmute_copy(&cwcphrase)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PutSmallPhrase: PutSmallPhrase::<Identity, OFFSET>,
            PutPhrase: PutPhrase::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPhraseSink as windows_core::Interface>::IID
    }
}
