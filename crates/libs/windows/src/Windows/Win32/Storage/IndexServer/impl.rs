pub trait IFilterImpl: Sized {
    fn Init();
    fn GetChunk();
    fn GetText();
    fn GetValue();
    fn BindRegion();
}
impl ::windows::core::RuntimeName for IFilter {
    const NAME: &'static str = "Windows.Win32.Storage.IndexServer.IFilter";
}
impl IFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFilterImpl, const OFFSET: isize>() -> IFilterVtbl {
        unsafe extern "system" fn Init<Impl: IFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfflags: u32, cattributes: u32, aattributes: *const FULLPROPSPEC, pflags: *mut u32) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Init(grfflags, cattributes, &*(&aattributes as *const <FULLPROPSPEC as ::windows::core::Abi>::Abi as *const <FULLPROPSPEC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChunk<Impl: IFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstat: *mut STAT_CHUNK) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChunk(::core::mem::transmute_copy(&pstat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Impl: IFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcwcbuffer: *mut u32, awcbuffer: super::super::Foundation::PWSTR) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetText(pcwcbuffer, ::core::mem::transmute_copy(&awcbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: IFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropvalue: *mut *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(::core::mem::transmute_copy(&pppropvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindRegion<Impl: IFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, origpos: FILTERREGION, riid: &::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindRegion(&*(&origpos as *const <FILTERREGION as ::windows::core::Abi>::Abi as *const <FILTERREGION as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFilter>, ::windows::core::GetTrustLevel, Init::<Impl, OFFSET>, GetChunk::<Impl, OFFSET>, GetText::<Impl, OFFSET>, GetValue::<Impl, OFFSET>, BindRegion::<Impl, OFFSET>)
    }
}
pub trait IPhraseSinkImpl: Sized {
    fn PutSmallPhrase();
    fn PutPhrase();
}
impl ::windows::core::RuntimeName for IPhraseSink {
    const NAME: &'static str = "Windows.Win32.Storage.IndexServer.IPhraseSink";
}
impl IPhraseSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhraseSinkImpl, const OFFSET: isize>() -> IPhraseSinkVtbl {
        unsafe extern "system" fn PutSmallPhrase<Impl: IPhraseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcnoun: super::super::Foundation::PWSTR, cwcnoun: u32, pwcmodifier: super::super::Foundation::PWSTR, cwcmodifier: u32, ulattachmenttype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PutSmallPhrase(&*(&pwcnoun as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cwcnoun, &*(&pwcmodifier as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cwcmodifier, ulattachmenttype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutPhrase<Impl: IPhraseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcphrase: super::super::Foundation::PWSTR, cwcphrase: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PutPhrase(&*(&pwcphrase as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cwcphrase) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPhraseSink>, ::windows::core::GetTrustLevel, PutSmallPhrase::<Impl, OFFSET>, PutPhrase::<Impl, OFFSET>)
    }
}
