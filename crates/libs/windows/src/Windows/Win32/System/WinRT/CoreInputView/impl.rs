pub trait ICoreFrameworkInputViewInteropImpl: Sized {
    fn GetForWindow();
}
impl ::windows::core::RuntimeName for ICoreFrameworkInputViewInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.CoreInputView.ICoreFrameworkInputViewInterop";
}
impl ICoreFrameworkInputViewInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreFrameworkInputViewInteropImpl, const OFFSET: isize>() -> ICoreFrameworkInputViewInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: ICoreFrameworkInputViewInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, coreframeworkinputview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForWindow(&*(&appwindow as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&coreframeworkinputview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreFrameworkInputViewInterop>, ::windows::core::GetTrustLevel, GetForWindow::<Impl, OFFSET>)
    }
}
