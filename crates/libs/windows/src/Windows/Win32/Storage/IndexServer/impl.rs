#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IFilter_Impl: Sized {
    fn Init(&mut self, grfflags: u32, cattributes: u32, aattributes: *const FULLPROPSPEC, pflags: *mut u32) -> i32;
    fn GetChunk(&mut self, pstat: *mut STAT_CHUNK) -> i32;
    fn GetText(&mut self, pcwcbuffer: *mut u32, awcbuffer: super::super::Foundation::PWSTR) -> i32;
    fn GetValue(&mut self, pppropvalue: *mut *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> i32;
    fn BindRegion(&mut self, origpos: &FILTERREGION, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> i32;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFilter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFilter_Vtbl {
        unsafe extern "system" fn Init<Impl: IFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfflags: u32, cattributes: u32, aattributes: *const FULLPROPSPEC, pflags: *mut u32) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Init(::core::mem::transmute_copy(&grfflags), ::core::mem::transmute_copy(&cattributes), ::core::mem::transmute_copy(&aattributes), ::core::mem::transmute_copy(&pflags))
        }
        unsafe extern "system" fn GetChunk<Impl: IFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstat: *mut STAT_CHUNK) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChunk(::core::mem::transmute_copy(&pstat))
        }
        unsafe extern "system" fn GetText<Impl: IFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcwcbuffer: *mut u32, awcbuffer: super::super::Foundation::PWSTR) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetText(::core::mem::transmute_copy(&pcwcbuffer), ::core::mem::transmute_copy(&awcbuffer))
        }
        unsafe extern "system" fn GetValue<Impl: IFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropvalue: *mut *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetValue(::core::mem::transmute_copy(&pppropvalue))
        }
        unsafe extern "system" fn BindRegion<Impl: IFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, origpos: FILTERREGION, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindRegion(::core::mem::transmute_copy(&origpos), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Init: Init::<Impl, IMPL_OFFSET>,
            GetChunk: GetChunk::<Impl, IMPL_OFFSET>,
            GetText: GetText::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            BindRegion: BindRegion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPhraseSink_Impl: Sized {
    fn PutSmallPhrase(&mut self, pwcnoun: super::super::Foundation::PWSTR, cwcnoun: u32, pwcmodifier: super::super::Foundation::PWSTR, cwcmodifier: u32, ulattachmenttype: u32) -> ::windows::core::Result<()>;
    fn PutPhrase(&mut self, pwcphrase: super::super::Foundation::PWSTR, cwcphrase: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPhraseSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhraseSink_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhraseSink_Vtbl {
        unsafe extern "system" fn PutSmallPhrase<Impl: IPhraseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcnoun: super::super::Foundation::PWSTR, cwcnoun: u32, pwcmodifier: super::super::Foundation::PWSTR, cwcmodifier: u32, ulattachmenttype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutSmallPhrase(::core::mem::transmute_copy(&pwcnoun), ::core::mem::transmute_copy(&cwcnoun), ::core::mem::transmute_copy(&pwcmodifier), ::core::mem::transmute_copy(&cwcmodifier), ::core::mem::transmute_copy(&ulattachmenttype)).into()
        }
        unsafe extern "system" fn PutPhrase<Impl: IPhraseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcphrase: super::super::Foundation::PWSTR, cwcphrase: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutPhrase(::core::mem::transmute_copy(&pwcphrase), ::core::mem::transmute_copy(&cwcphrase)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            PutSmallPhrase: PutSmallPhrase::<Impl, IMPL_OFFSET>,
            PutPhrase: PutPhrase::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhraseSink as ::windows::core::Interface>::IID
    }
}
