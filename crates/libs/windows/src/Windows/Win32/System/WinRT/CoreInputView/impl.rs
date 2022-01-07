pub trait ICoreFrameworkInputViewInteropImpl: Sized {
    fn GetForWindow();
}
impl ::windows::core::RuntimeName for ICoreFrameworkInputViewInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.CoreInputView.ICoreFrameworkInputViewInterop";
}
impl ICoreFrameworkInputViewInteropVtbl {
    pub const fn new<Impl: ICoreFrameworkInputViewInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICoreFrameworkInputViewInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: ICoreFrameworkInputViewInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, coreframeworkinputview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForWindow(&*(&appwindow as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&coreframeworkinputview)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICoreFrameworkInputViewInterop>, base.5, GetForWindow::<Impl, OFFSET>)
    }
}
