pub trait IAudioFrameNativeImpl: Sized {
    fn GetData();
}
impl ::windows::core::RuntimeName for IAudioFrameNative {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Media.IAudioFrameNative";
}
impl IAudioFrameNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioFrameNativeImpl, const OFFSET: isize>() -> IAudioFrameNativeVtbl {
        unsafe extern "system" fn GetData<Impl: IAudioFrameNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: &::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetData(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioFrameNative>, ::windows::core::GetTrustLevel, GetData::<Impl, OFFSET>)
    }
}
pub trait IAudioFrameNativeFactoryImpl: Sized {
    fn CreateFromMFSample();
}
impl ::windows::core::RuntimeName for IAudioFrameNativeFactory {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Media.IAudioFrameNativeFactory";
}
impl IAudioFrameNativeFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioFrameNativeFactoryImpl, const OFFSET: isize>() -> IAudioFrameNativeFactoryVtbl {
        unsafe extern "system" fn CreateFromMFSample<Impl: IAudioFrameNativeFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, forcereadonly: super::super::super::Foundation::BOOL, riid: &::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioFrameNativeFactory>, ::windows::core::GetTrustLevel, CreateFromMFSample::<Impl, OFFSET>)
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoFrameNativeImpl, const OFFSET: isize>() -> IVideoFrameNativeVtbl {
        unsafe extern "system" fn GetData<Impl: IVideoFrameNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: &::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetData(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevice<Impl: IVideoFrameNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: &::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevice(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVideoFrameNative>, ::windows::core::GetTrustLevel, GetData::<Impl, OFFSET>, GetDevice::<Impl, OFFSET>)
    }
}
pub trait IVideoFrameNativeFactoryImpl: Sized {
    fn CreateFromMFSample();
}
impl ::windows::core::RuntimeName for IVideoFrameNativeFactory {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Media.IVideoFrameNativeFactory";
}
impl IVideoFrameNativeFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoFrameNativeFactoryImpl, const OFFSET: isize>() -> IVideoFrameNativeFactoryVtbl {
        unsafe extern "system" fn CreateFromMFSample<Impl: IVideoFrameNativeFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, subtype: &::windows::core::GUID, width: u32, height: u32, forcereadonly: super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::Media::MediaFoundation::MFVideoArea, device: ::windows::core::RawPtr, riid: &::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVideoFrameNativeFactory>, ::windows::core::GetTrustLevel, CreateFromMFSample::<Impl, OFFSET>)
    }
}
