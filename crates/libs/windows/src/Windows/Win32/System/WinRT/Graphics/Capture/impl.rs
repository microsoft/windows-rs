pub trait IGraphicsCaptureItemInteropImpl: Sized {
    fn CreateForWindow();
    fn CreateForMonitor();
}
impl ::windows::core::RuntimeName for IGraphicsCaptureItemInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Graphics.Capture.IGraphicsCaptureItemInterop";
}
impl IGraphicsCaptureItemInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGraphicsCaptureItemInteropImpl, const OFFSET: isize>() -> IGraphicsCaptureItemInteropVtbl {
        unsafe extern "system" fn CreateForWindow<Impl: IGraphicsCaptureItemInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::super::super::Foundation::HWND, riid: &::windows::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForWindow(&*(&window as *const <super::super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&result)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForMonitor<Impl: IGraphicsCaptureItemInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, monitor: super::super::super::super::Graphics::Gdi::HMONITOR, riid: &::windows::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForMonitor(&*(&monitor as *const <super::super::super::super::Graphics::Gdi::HMONITOR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Graphics::Gdi::HMONITOR as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&result)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGraphicsCaptureItemInterop>, ::windows::core::GetTrustLevel, CreateForWindow::<Impl, OFFSET>, CreateForMonitor::<Impl, OFFSET>)
    }
}
