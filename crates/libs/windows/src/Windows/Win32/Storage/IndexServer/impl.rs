#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IFilterImpl: Sized {
    fn Init();
    fn GetChunk();
    fn GetText();
    fn GetValue();
    fn BindRegion();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFilterVtbl {
        unsafe extern "system" fn Init<Impl: IFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfflags: u32, cattributes: u32, aattributes: *const FULLPROPSPEC, pflags: *mut u32) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChunk<Impl: IFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstat: *mut STAT_CHUNK) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetText<Impl: IFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcwcbuffer: *mut u32, awcbuffer: super::super::Foundation::PWSTR) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValue<Impl: IFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropvalue: *mut *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BindRegion<Impl: IFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, origpos: FILTERREGION, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Init::<Impl, IMPL_OFFSET>, GetChunk::<Impl, IMPL_OFFSET>, GetText::<Impl, IMPL_OFFSET>, GetValue::<Impl, IMPL_OFFSET>, BindRegion::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPhraseSinkImpl: Sized {
    fn PutSmallPhrase();
    fn PutPhrase();
}
#[cfg(feature = "Win32_Foundation")]
impl IPhraseSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhraseSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhraseSinkVtbl {
        unsafe extern "system" fn PutSmallPhrase<Impl: IPhraseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcnoun: super::super::Foundation::PWSTR, cwcnoun: u32, pwcmodifier: super::super::Foundation::PWSTR, cwcmodifier: u32, ulattachmenttype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PutPhrase<Impl: IPhraseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcphrase: super::super::Foundation::PWSTR, cwcphrase: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, PutSmallPhrase::<Impl, IMPL_OFFSET>, PutPhrase::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhraseSink as ::windows::core::Interface>::IID
    }
}
