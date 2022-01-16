pub trait IDefaultAudioDeviceChangedEventArgs_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Role(&mut self) -> ::windows::core::Result<AudioDeviceRole>;
}
impl ::windows::core::RuntimeName for IDefaultAudioDeviceChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Devices.IDefaultAudioDeviceChangedEventArgs";
}
impl IDefaultAudioDeviceChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDefaultAudioDeviceChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDefaultAudioDeviceChangedEventArgs_Vtbl {
        unsafe extern "system" fn Id<Impl: IDefaultAudioDeviceChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Role<Impl: IDefaultAudioDeviceChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AudioDeviceRole) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Role() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDefaultAudioDeviceChangedEventArgs, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Role: Role::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDefaultAudioDeviceChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
pub trait IMediaDeviceController_Impl: Sized {
    fn GetAvailableMediaStreamProperties(&mut self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::MediaProperties::IMediaEncodingProperties>>;
    fn GetMediaStreamProperties(&mut self, mediastreamtype: super::Capture::MediaStreamType) -> ::windows::core::Result<super::MediaProperties::IMediaEncodingProperties>;
    fn SetMediaStreamPropertiesAsync(&mut self, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: &::core::option::Option<super::MediaProperties::IMediaEncodingProperties>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
impl ::windows::core::RuntimeName for IMediaDeviceController {
    const NAME: &'static str = "Windows.Media.Devices.IMediaDeviceController";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Capture", feature = "Media_MediaProperties"))]
impl IMediaDeviceController_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaDeviceController_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaDeviceController_Vtbl {
        unsafe extern "system" fn GetAvailableMediaStreamProperties<Impl: IMediaDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediastreamtype: super::Capture::MediaStreamType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAvailableMediaStreamProperties(mediastreamtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMediaStreamProperties<Impl: IMediaDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediastreamtype: super::Capture::MediaStreamType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMediaStreamProperties(mediastreamtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMediaStreamPropertiesAsync<Impl: IMediaDeviceController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mediastreamtype: super::Capture::MediaStreamType, mediaencodingproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMediaStreamPropertiesAsync(mediastreamtype, &*(&mediaencodingproperties as *const <super::MediaProperties::IMediaEncodingProperties as ::windows::core::Abi>::Abi as *const <super::MediaProperties::IMediaEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaDeviceController, BASE_OFFSET>(),
            GetAvailableMediaStreamProperties: GetAvailableMediaStreamProperties::<Impl, IMPL_OFFSET>,
            GetMediaStreamProperties: GetMediaStreamProperties::<Impl, IMPL_OFFSET>,
            SetMediaStreamPropertiesAsync: SetMediaStreamPropertiesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaDeviceController as ::windows::core::Interface>::IID
    }
}
