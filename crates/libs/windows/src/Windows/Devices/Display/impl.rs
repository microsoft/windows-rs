#[cfg(all(feature = "Foundation", feature = "Graphics", feature = "implement_exclusive"))]
pub trait IDisplayMonitorImpl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConnectionKind(&mut self) -> ::windows::core::Result<DisplayMonitorConnectionKind>;
    fn PhysicalConnector(&mut self) -> ::windows::core::Result<DisplayMonitorPhysicalConnectorKind>;
    fn DisplayAdapterDeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayAdapterId(&mut self) -> ::windows::core::Result<super::super::Graphics::DisplayAdapterId>;
    fn DisplayAdapterTargetId(&mut self) -> ::windows::core::Result<u32>;
    fn UsageKind(&mut self) -> ::windows::core::Result<DisplayMonitorUsageKind>;
    fn NativeResolutionInRawPixels(&mut self) -> ::windows::core::Result<super::super::Graphics::SizeInt32>;
    fn PhysicalSizeInInches(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Size>>;
    fn RawDpiX(&mut self) -> ::windows::core::Result<f32>;
    fn RawDpiY(&mut self) -> ::windows::core::Result<f32>;
    fn RedPrimary(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn GreenPrimary(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn BluePrimary(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn WhitePoint(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn MaxLuminanceInNits(&mut self) -> ::windows::core::Result<f32>;
    fn MinLuminanceInNits(&mut self) -> ::windows::core::Result<f32>;
    fn MaxAverageFullFrameLuminanceInNits(&mut self) -> ::windows::core::Result<f32>;
    fn GetDescriptor(&mut self, descriptorkind: DisplayMonitorDescriptorKind) -> ::windows::core::Result<::windows::core::Array<u8>>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayMonitor {
    const NAME: &'static str = "Windows.Devices.Display.IDisplayMonitor";
}
#[cfg(all(feature = "Foundation", feature = "Graphics", feature = "implement_exclusive"))]
impl IDisplayMonitorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayMonitorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayMonitorVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayMonitor, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            ConnectionKind: ConnectionKind::<Impl, IMPL_OFFSET>,
            PhysicalConnector: PhysicalConnector::<Impl, IMPL_OFFSET>,
            DisplayAdapterDeviceId: DisplayAdapterDeviceId::<Impl, IMPL_OFFSET>,
            DisplayAdapterId: DisplayAdapterId::<Impl, IMPL_OFFSET>,
            DisplayAdapterTargetId: DisplayAdapterTargetId::<Impl, IMPL_OFFSET>,
            UsageKind: UsageKind::<Impl, IMPL_OFFSET>,
            NativeResolutionInRawPixels: NativeResolutionInRawPixels::<Impl, IMPL_OFFSET>,
            PhysicalSizeInInches: PhysicalSizeInInches::<Impl, IMPL_OFFSET>,
            RawDpiX: RawDpiX::<Impl, IMPL_OFFSET>,
            RawDpiY: RawDpiY::<Impl, IMPL_OFFSET>,
            RedPrimary: RedPrimary::<Impl, IMPL_OFFSET>,
            GreenPrimary: GreenPrimary::<Impl, IMPL_OFFSET>,
            BluePrimary: BluePrimary::<Impl, IMPL_OFFSET>,
            WhitePoint: WhitePoint::<Impl, IMPL_OFFSET>,
            MaxLuminanceInNits: MaxLuminanceInNits::<Impl, IMPL_OFFSET>,
            MinLuminanceInNits: MinLuminanceInNits::<Impl, IMPL_OFFSET>,
            MaxAverageFullFrameLuminanceInNits: MaxAverageFullFrameLuminanceInNits::<Impl, IMPL_OFFSET>,
            GetDescriptor: GetDescriptor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayMonitor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDisplayMonitor2Impl: Sized {
    fn IsDolbyVisionSupportedInHdrMode(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayMonitor2 {
    const NAME: &'static str = "Windows.Devices.Display.IDisplayMonitor2";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayMonitor2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayMonitor2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayMonitor2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayMonitor2, BASE_OFFSET>(),
            IsDolbyVisionSupportedInHdrMode: IsDolbyVisionSupportedInHdrMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayMonitor2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDisplayMonitorStaticsImpl: Sized {
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DisplayMonitor>>;
    fn FromInterfaceIdAsync(&mut self, deviceinterfaceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DisplayMonitor>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayMonitorStatics {
    const NAME: &'static str = "Windows.Devices.Display.IDisplayMonitorStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDisplayMonitorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayMonitorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayMonitorStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayMonitorStatics, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            FromInterfaceIdAsync: FromInterfaceIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayMonitorStatics as ::windows::core::Interface>::IID
    }
}
