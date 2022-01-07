pub trait IDummyHICONIncluderImpl: Sized {
    fn Dummy();
}
impl ::windows::core::RuntimeName for IDummyHICONIncluder {
    const NAME: &'static str = "Windows.Win32.System.Com.UI.IDummyHICONIncluder";
}
impl IDummyHICONIncluderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDummyHICONIncluderImpl, const OFFSET: isize>() -> IDummyHICONIncluderVtbl {
        unsafe extern "system" fn Dummy<Impl: IDummyHICONIncluderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, h1: super::super::super::UI::WindowsAndMessaging::HICON, h2: super::super::super::Graphics::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Dummy(&*(&h1 as *const <super::super::super::UI::WindowsAndMessaging::HICON as ::windows::core::Abi>::Abi as *const <super::super::super::UI::WindowsAndMessaging::HICON as ::windows::core::DefaultType>::DefaultType), &*(&h2 as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Gdi::HDC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDummyHICONIncluder>, ::windows::core::GetTrustLevel, Dummy::<Impl, OFFSET>)
    }
}
pub trait IThumbnailExtractorImpl: Sized {
    fn ExtractThumbnail();
    fn OnFileUpdated();
}
impl ::windows::core::RuntimeName for IThumbnailExtractor {
    const NAME: &'static str = "Windows.Win32.System.Com.UI.IThumbnailExtractor";
}
impl IThumbnailExtractorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IThumbnailExtractorImpl, const OFFSET: isize>() -> IThumbnailExtractorVtbl {
        unsafe extern "system" fn ExtractThumbnail<Impl: IThumbnailExtractorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstg: ::windows::core::RawPtr, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::super::super::Graphics::Gdi::HBITMAP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtractThumbnail(&*(&pstg as *const <super::StructuredStorage::IStorage as ::windows::core::Abi>::Abi as *const <super::StructuredStorage::IStorage as ::windows::core::DefaultType>::DefaultType), ullength, ulheight, ::core::mem::transmute_copy(&puloutputlength), ::core::mem::transmute_copy(&puloutputheight), ::core::mem::transmute_copy(&phoutputbitmap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnFileUpdated<Impl: IThumbnailExtractorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnFileUpdated(&*(&pstg as *const <super::StructuredStorage::IStorage as ::windows::core::Abi>::Abi as *const <super::StructuredStorage::IStorage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IThumbnailExtractor>, ::windows::core::GetTrustLevel, ExtractThumbnail::<Impl, OFFSET>, OnFileUpdated::<Impl, OFFSET>)
    }
}
