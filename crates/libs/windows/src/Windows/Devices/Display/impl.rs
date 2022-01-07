#[cfg(feature = "implement_exclusive")]
pub trait IDisplayMonitorImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConnectionKind(&self) -> ::windows::core::Result<DisplayMonitorConnectionKind>;
    fn PhysicalConnector(&self) -> ::windows::core::Result<DisplayMonitorPhysicalConnectorKind>;
    fn DisplayAdapterDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayAdapterId(&self) -> ::windows::core::Result<super::super::Graphics::DisplayAdapterId>;
    fn DisplayAdapterTargetId(&self) -> ::windows::core::Result<u32>;
    fn UsageKind(&self) -> ::windows::core::Result<DisplayMonitorUsageKind>;
    fn NativeResolutionInRawPixels(&self) -> ::windows::core::Result<super::super::Graphics::SizeInt32>;
    fn PhysicalSizeInInches(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Size>>;
    fn RawDpiX(&self) -> ::windows::core::Result<f32>;
    fn RawDpiY(&self) -> ::windows::core::Result<f32>;
    fn RedPrimary(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn GreenPrimary(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn BluePrimary(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn WhitePoint(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn MaxLuminanceInNits(&self) -> ::windows::core::Result<f32>;
    fn MinLuminanceInNits(&self) -> ::windows::core::Result<f32>;
    fn MaxAverageFullFrameLuminanceInNits(&self) -> ::windows::core::Result<f32>;
    fn GetDescriptor(&self, descriptorkind: DisplayMonitorDescriptorKind) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayMonitor {
    const NAME: &'static str = "Windows.Devices.Display.IDisplayMonitor";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayMonitorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayMonitorImpl, const OFFSET: isize>() -> IDisplayMonitorVtbl {
        unsafe extern "system" fn DeviceId<Impl: IDisplayMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IDisplayMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionKind<Impl: IDisplayMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayMonitorConnectionKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhysicalConnector<Impl: IDisplayMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayMonitorPhysicalConnectorKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhysicalConnector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayAdapterDeviceId<Impl: IDisplayMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayAdapterDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayAdapterId<Impl: IDisplayMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::DisplayAdapterId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayAdapterId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayAdapterTargetId<Impl: IDisplayMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayAdapterTargetId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsageKind<Impl: IDisplayMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DisplayMonitorUsageKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsageKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NativeResolutionInRawPixels<Impl: IDisplayMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Graphics::SizeInt32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NativeResolutionInRawPixels() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhysicalSizeInInches<Impl: IDisplayMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhysicalSizeInInches() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawDpiX<Impl: IDisplayMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawDpiX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawDpiY<Impl: IDisplayMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawDpiY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedPrimary<Impl: IDisplayMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedPrimary() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GreenPrimary<Impl: IDisplayMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GreenPrimary() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BluePrimary<Impl: IDisplayMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BluePrimary() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WhitePoint<Impl: IDisplayMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WhitePoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxLuminanceInNits<Impl: IDisplayMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxLuminanceInNits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinLuminanceInNits<Impl: IDisplayMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinLuminanceInNits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxAverageFullFrameLuminanceInNits<Impl: IDisplayMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxAverageFullFrameLuminanceInNits() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescriptor<Impl: IDisplayMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, descriptorkind: DisplayMonitorDescriptorKind, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescriptor(descriptorkind) {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDisplayMonitor>,
            ::windows::core::GetTrustLevel,
            DeviceId::<Impl, OFFSET>,
            DisplayName::<Impl, OFFSET>,
            ConnectionKind::<Impl, OFFSET>,
            PhysicalConnector::<Impl, OFFSET>,
            DisplayAdapterDeviceId::<Impl, OFFSET>,
            DisplayAdapterId::<Impl, OFFSET>,
            DisplayAdapterTargetId::<Impl, OFFSET>,
            UsageKind::<Impl, OFFSET>,
            NativeResolutionInRawPixels::<Impl, OFFSET>,
            PhysicalSizeInInches::<Impl, OFFSET>,
            RawDpiX::<Impl, OFFSET>,
            RawDpiY::<Impl, OFFSET>,
            RedPrimary::<Impl, OFFSET>,
            GreenPrimary::<Impl, OFFSET>,
            BluePrimary::<Impl, OFFSET>,
            WhitePoint::<Impl, OFFSET>,
            MaxLuminanceInNits::<Impl, OFFSET>,
            MinLuminanceInNits::<Impl, OFFSET>,
            MaxAverageFullFrameLuminanceInNits::<Impl, OFFSET>,
            GetDescriptor::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayMonitor2Impl: Sized {
    fn IsDolbyVisionSupportedInHdrMode(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayMonitor2 {
    const NAME: &'static str = "Windows.Devices.Display.IDisplayMonitor2";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayMonitor2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayMonitor2Impl, const OFFSET: isize>() -> IDisplayMonitor2Vtbl {
        unsafe extern "system" fn IsDolbyVisionSupportedInHdrMode<Impl: IDisplayMonitor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDolbyVisionSupportedInHdrMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayMonitor2>, ::windows::core::GetTrustLevel, IsDolbyVisionSupportedInHdrMode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayMonitorStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DisplayMonitor>>;
    fn FromInterfaceIdAsync(&self, deviceinterfaceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DisplayMonitor>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayMonitorStatics {
    const NAME: &'static str = "Windows.Devices.Display.IDisplayMonitorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayMonitorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayMonitorStaticsImpl, const OFFSET: isize>() -> IDisplayMonitorStaticsVtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: IDisplayMonitorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IDisplayMonitorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromInterfaceIdAsync<Impl: IDisplayMonitorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceinterfaceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromInterfaceIdAsync(&*(&deviceinterfaceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayMonitorStatics>, ::windows::core::GetTrustLevel, GetDeviceSelector::<Impl, OFFSET>, FromIdAsync::<Impl, OFFSET>, FromInterfaceIdAsync::<Impl, OFFSET>)
    }
}
