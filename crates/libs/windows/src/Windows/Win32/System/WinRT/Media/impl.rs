pub trait IAudioFrameNative_Impl: Sized {
    fn GetData(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAudioFrameNative {
    const NAME: &'static str = "";
}
impl IAudioFrameNative_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioFrameNative_Impl, const OFFSET: isize>() -> IAudioFrameNative_Vtbl {
        unsafe extern "system" fn GetData<Identity: ::windows::core::IUnknownImpl, Impl: IAudioFrameNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetData(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioFrameNative, OFFSET>(), GetData: GetData::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioFrameNative as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
pub trait IAudioFrameNativeFactory_Impl: Sized {
    fn CreateFromMFSample(&self, data: &::core::option::Option<super::super::super::Media::MediaFoundation::IMFSample>, forcereadonly: super::super::super::Foundation::BOOL, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
impl ::windows::core::RuntimeName for IAudioFrameNativeFactory {
    const NAME: &'static str = "";
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
impl IAudioFrameNativeFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioFrameNativeFactory_Impl, const OFFSET: isize>() -> IAudioFrameNativeFactory_Vtbl {
        unsafe extern "system" fn CreateFromMFSample<Identity: ::windows::core::IUnknownImpl, Impl: IAudioFrameNativeFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, forcereadonly: super::super::super::Foundation::BOOL, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateFromMFSample(::core::mem::transmute(&data), ::core::mem::transmute_copy(&forcereadonly), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAudioFrameNativeFactory, OFFSET>(),
            CreateFromMFSample: CreateFromMFSample::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioFrameNativeFactory as ::windows::core::Interface>::IID
    }
}
pub trait IVideoFrameNative_Impl: Sized {
    fn GetData(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetDevice(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IVideoFrameNative {
    const NAME: &'static str = "";
}
impl IVideoFrameNative_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoFrameNative_Impl, const OFFSET: isize>() -> IVideoFrameNative_Vtbl {
        unsafe extern "system" fn GetData<Identity: ::windows::core::IUnknownImpl, Impl: IVideoFrameNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetData(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl, Impl: IVideoFrameNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDevice(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoFrameNative, OFFSET>(),
            GetData: GetData::<Identity, Impl, OFFSET>,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoFrameNative as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
pub trait IVideoFrameNativeFactory_Impl: Sized {
    fn CreateFromMFSample(&self, data: &::core::option::Option<super::super::super::Media::MediaFoundation::IMFSample>, subtype: *const ::windows::core::GUID, width: u32, height: u32, forcereadonly: super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::Media::MediaFoundation::MFVideoArea, device: &::core::option::Option<super::super::super::Media::MediaFoundation::IMFDXGIDeviceManager>, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
impl ::windows::core::RuntimeName for IVideoFrameNativeFactory {
    const NAME: &'static str = "";
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
impl IVideoFrameNativeFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVideoFrameNativeFactory_Impl, const OFFSET: isize>() -> IVideoFrameNativeFactory_Vtbl {
        unsafe extern "system" fn CreateFromMFSample<Identity: ::windows::core::IUnknownImpl, Impl: IVideoFrameNativeFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, subtype: *const ::windows::core::GUID, width: u32, height: u32, forcereadonly: super::super::super::Foundation::BOOL, mindisplayaperture: *const super::super::super::Media::MediaFoundation::MFVideoArea, device: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateFromMFSample(::core::mem::transmute(&data), ::core::mem::transmute_copy(&subtype), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&forcereadonly), ::core::mem::transmute_copy(&mindisplayaperture), ::core::mem::transmute(&device), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVideoFrameNativeFactory, OFFSET>(),
            CreateFromMFSample: CreateFromMFSample::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVideoFrameNativeFactory as ::windows::core::Interface>::IID
    }
}
