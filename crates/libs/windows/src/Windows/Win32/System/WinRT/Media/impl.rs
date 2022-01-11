pub trait IAudioFrameNativeImpl: Sized {
    fn GetData();
}
impl ::windows::core::RuntimeName for IAudioFrameNative {
    const NAME: &'static str = "";
}
impl IAudioFrameNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioFrameNativeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioFrameNativeVtbl {
        unsafe extern "system" fn GetData<Impl: IAudioFrameNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioFrameNative>, ::windows::core::GetTrustLevel, GetData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioFrameNative as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
pub trait IAudioFrameNativeFactoryImpl: Sized {
    fn CreateFromMFSample();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
impl ::windows::core::RuntimeName for IAudioFrameNativeFactory {
    const NAME: &'static str = "";
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
impl IAudioFrameNativeFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioFrameNativeFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioFrameNativeFactoryVtbl {
        unsafe extern "system" fn CreateFromMFSample<Impl: IAudioFrameNativeFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, forcereadonly: super::super::super::Foundation::BOOL, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioFrameNativeFactory>, ::windows::core::GetTrustLevel, CreateFromMFSample::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioFrameNativeFactory as ::windows::core::Interface>::IID
    }
}
pub trait IVideoFrameNativeImpl: Sized {
    fn GetData();
    fn GetDevice();
}
impl ::windows::core::RuntimeName for IVideoFrameNative {
    const NAME: &'static str = "";
}
impl IVideoFrameNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoFrameNativeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoFrameNativeVtbl {
        unsafe extern "system" fn GetData<Impl: IVideoFrameNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDevice<Impl: IVideoFrameNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVideoFrameNative>, ::windows::core::GetTrustLevel, GetData::<Impl, IMPL_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoFrameNative as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
pub trait IVideoFrameNativeFactoryImpl: Sized {
    fn CreateFromMFSample();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
impl ::windows::core::RuntimeName for IVideoFrameNativeFactory {
    const NAME: &'static str = "";
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
impl IVideoFrameNativeFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoFrameNativeFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVideoFrameNativeFactoryVtbl {
        unsafe extern "system" fn CreateFromMFSample<Impl: IVideoFrameNativeFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, subtype: *const ::windows::core::GUID, width: u32, height: u32, forcereadonly: super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::Media::MediaFoundation::MFVideoArea, device: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVideoFrameNativeFactory>, ::windows::core::GetTrustLevel, CreateFromMFSample::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoFrameNativeFactory as ::windows::core::Interface>::IID
    }
}
