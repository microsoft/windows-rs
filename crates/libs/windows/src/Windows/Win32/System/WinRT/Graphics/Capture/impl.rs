pub trait IGraphicsCaptureItemInteropImpl: Sized {
    fn CreateForWindow();
    fn CreateForMonitor();
}
impl ::windows::core::RuntimeName for IGraphicsCaptureItemInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Graphics.Capture.IGraphicsCaptureItemInterop";
}
impl IGraphicsCaptureItemInteropVtbl {
    pub const fn new<Impl: IGraphicsCaptureItemInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGraphicsCaptureItemInteropVtbl {
        unsafe extern "system" fn CreateForWindow<Impl: IGraphicsCaptureItemInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: super::super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForWindow(&*(&window as *const <super::super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&result)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForMonitor<Impl: IGraphicsCaptureItemInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, monitor: super::super::super::super::Graphics::Gdi::HMONITOR, riid: *const ::windows::core::GUID, result: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForMonitor(&*(&monitor as *const <super::super::super::super::Graphics::Gdi::HMONITOR as ::windows::core::Abi>::Abi as *const <super::super::super::super::Graphics::Gdi::HMONITOR as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&result)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGraphicsCaptureItemInterop>, base.5, CreateForWindow::<Impl, OFFSET>, CreateForMonitor::<Impl, OFFSET>)
    }
}
