pub trait IAudioFrameNativeImpl: Sized {
    fn GetData();
}
impl ::windows::core::RuntimeName for IAudioFrameNative {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Media.IAudioFrameNative";
}
impl IAudioFrameNativeVtbl {
    pub const fn new<Impl: IAudioFrameNativeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioFrameNativeVtbl {
        unsafe extern "system" fn GetData<Impl: IAudioFrameNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetData(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioFrameNative>, base.5, GetData::<Impl, OFFSET>)
    }
}
pub trait IAudioFrameNativeFactoryImpl: Sized {
    fn CreateFromMFSample();
}
impl ::windows::core::RuntimeName for IAudioFrameNativeFactory {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Media.IAudioFrameNativeFactory";
}
impl IAudioFrameNativeFactoryVtbl {
    pub const fn new<Impl: IAudioFrameNativeFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAudioFrameNativeFactoryVtbl {
        unsafe extern "system" fn CreateFromMFSample<Impl: IAudioFrameNativeFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, forcereadonly: super::super::super::Foundation::BOOL, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromMFSample(
                &*(&data as *const <super::super::super::Media::MediaFoundation::IMFSample as ::windows::core::Abi>::Abi as *const <super::super::super::Media::MediaFoundation::IMFSample as ::windows::core::DefaultType>::DefaultType),
                &*(&forcereadonly as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppv),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAudioFrameNativeFactory>, base.5, CreateFromMFSample::<Impl, OFFSET>)
    }
}
pub trait IVideoFrameNativeImpl: Sized {
    fn GetData();
    fn GetDevice();
}
impl ::windows::core::RuntimeName for IVideoFrameNative {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Media.IVideoFrameNative";
}
impl IVideoFrameNativeVtbl {
    pub const fn new<Impl: IVideoFrameNativeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoFrameNativeVtbl {
        unsafe extern "system" fn GetData<Impl: IVideoFrameNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetData(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevice<Impl: IVideoFrameNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDevice(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoFrameNative>, base.5, GetData::<Impl, OFFSET>, GetDevice::<Impl, OFFSET>)
    }
}
pub trait IVideoFrameNativeFactoryImpl: Sized {
    fn CreateFromMFSample();
}
impl ::windows::core::RuntimeName for IVideoFrameNativeFactory {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Media.IVideoFrameNativeFactory";
}
impl IVideoFrameNativeFactoryVtbl {
    pub const fn new<Impl: IVideoFrameNativeFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVideoFrameNativeFactoryVtbl {
        unsafe extern "system" fn CreateFromMFSample<Impl: IVideoFrameNativeFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, subtype: *const ::windows::core::GUID, width: u32, height: u32, forcereadonly: super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::Media::MediaFoundation::MFVideoArea, device: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFromMFSample(
                &*(&data as *const <super::super::super::Media::MediaFoundation::IMFSample as ::windows::core::Abi>::Abi as *const <super::super::super::Media::MediaFoundation::IMFSample as ::windows::core::DefaultType>::DefaultType),
                &*(&subtype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                width,
                height,
                &*(&forcereadonly as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&mindisplayaperture as *const <super::super::super::Media::MediaFoundation::MFVideoArea as ::windows::core::Abi>::Abi as *const <super::super::super::Media::MediaFoundation::MFVideoArea as ::windows::core::DefaultType>::DefaultType),
                &*(&device as *const <super::super::super::Media::MediaFoundation::IMFDXGIDeviceManager as ::windows::core::Abi>::Abi as *const <super::super::super::Media::MediaFoundation::IMFDXGIDeviceManager as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppv),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVideoFrameNativeFactory>, base.5, CreateFromMFSample::<Impl, OFFSET>)
    }
}
