#[cfg(all(feature = "Foundation", feature = "UI", feature = "implement_exclusive"))]
pub trait ILampImpl: Sized + IClosableImpl {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn BrightnessLevel(&mut self) -> ::windows::core::Result<f32>;
    fn SetBrightnessLevel(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn IsColorSettable(&mut self) -> ::windows::core::Result<bool>;
    fn Color(&mut self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetColor(&mut self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn AvailabilityChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Lamp, LampAvailabilityChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAvailabilityChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILamp {
    const NAME: &'static str = "Windows.Devices.Lights.ILamp";
}
#[cfg(all(feature = "Foundation", feature = "UI", feature = "implement_exclusive"))]
impl ILampVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILampVtbl {
        unsafe extern "system" fn DeviceId<Impl: ILampImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEnabled<Impl: ILampImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Impl: ILampImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn BrightnessLevel<Impl: ILampImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrightnessLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBrightnessLevel<Impl: ILampImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBrightnessLevel(value).into()
        }
        unsafe extern "system" fn IsColorSettable<Impl: ILampImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsColorSettable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Color<Impl: ILampImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: ILampImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AvailabilityChanged<Impl: ILampImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailabilityChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<Lamp, LampAvailabilityChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<Lamp, LampAvailabilityChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAvailabilityChanged<Impl: ILampImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAvailabilityChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILamp, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            SetIsEnabled: SetIsEnabled::<Impl, IMPL_OFFSET>,
            BrightnessLevel: BrightnessLevel::<Impl, IMPL_OFFSET>,
            SetBrightnessLevel: SetBrightnessLevel::<Impl, IMPL_OFFSET>,
            IsColorSettable: IsColorSettable::<Impl, IMPL_OFFSET>,
            Color: Color::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
            AvailabilityChanged: AvailabilityChanged::<Impl, IMPL_OFFSET>,
            RemoveAvailabilityChanged: RemoveAvailabilityChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILamp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Storage_Streams", feature = "System", feature = "UI", feature = "implement_exclusive"))]
pub trait ILampArrayImpl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HardwareVendorId(&mut self) -> ::windows::core::Result<u16>;
    fn HardwareProductId(&mut self) -> ::windows::core::Result<u16>;
    fn HardwareVersion(&mut self) -> ::windows::core::Result<u16>;
    fn LampArrayKind(&mut self) -> ::windows::core::Result<LampArrayKind>;
    fn LampCount(&mut self) -> ::windows::core::Result<i32>;
    fn MinUpdateInterval(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn BoundingBox(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn BrightnessLevel(&mut self) -> ::windows::core::Result<f64>;
    fn SetBrightnessLevel(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn IsConnected(&mut self) -> ::windows::core::Result<bool>;
    fn SupportsVirtualKeys(&mut self) -> ::windows::core::Result<bool>;
    fn GetLampInfo(&mut self, lampindex: i32) -> ::windows::core::Result<LampInfo>;
    fn GetIndicesForKey(&mut self, key: super::super::System::VirtualKey) -> ::windows::core::Result<::windows::core::Array<i32>>;
    fn GetIndicesForPurposes(&mut self, purposes: LampPurposes) -> ::windows::core::Result<::windows::core::Array<i32>>;
    fn SetColor(&mut self, desiredcolor: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn SetColorForIndex(&mut self, lampindex: i32, desiredcolor: &super::super::UI::Color) -> ::windows::core::Result<()>;
    fn SetSingleColorForIndices(&mut self, desiredcolor: &super::super::UI::Color, lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn SetColorsForIndices(&mut self, desiredcolors: &[<super::super::UI::Color as ::windows::core::DefaultType>::DefaultType], lampindexes: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn SetColorsForKey(&mut self, desiredcolor: &super::super::UI::Color, key: super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn SetColorsForKeys(&mut self, desiredcolors: &[<super::super::UI::Color as ::windows::core::DefaultType>::DefaultType], keys: &[<super::super::System::VirtualKey as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn SetColorsForPurposes(&mut self, desiredcolor: &super::super::UI::Color, purposes: LampPurposes) -> ::windows::core::Result<()>;
    fn SendMessageAsync(&mut self, messageid: i32, message: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RequestMessageAsync(&mut self, messageid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Storage_Streams", feature = "System", feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILampArray {
    const NAME: &'static str = "Windows.Devices.Lights.ILampArray";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Storage_Streams", feature = "System", feature = "UI", feature = "implement_exclusive"))]
impl ILampArrayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILampArrayVtbl {
        unsafe extern "system" fn DeviceId<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HardwareVendorId<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HardwareVendorId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareProductId<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HardwareProductId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareVersion<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HardwareVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LampArrayKind<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LampArrayKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LampArrayKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LampCount<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LampCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinUpdateInterval<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinUpdateInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BoundingBox<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BoundingBox() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabled<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn BrightnessLevel<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BrightnessLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBrightnessLevel<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBrightnessLevel(value).into()
        }
        unsafe extern "system" fn IsConnected<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportsVirtualKeys<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsVirtualKeys() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLampInfo<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lampindex: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLampInfo(lampindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndicesForKey<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: super::super::System::VirtualKey, result_size__: *mut u32, result__: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIndicesForKey(key) {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndicesForPurposes<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, purposes: LampPurposes, result_size__: *mut u32, result__: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIndicesForPurposes(purposes) {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredcolor: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&desiredcolor as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetColorForIndex<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lampindex: i32, desiredcolor: super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorForIndex(lampindex, &*(&desiredcolor as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetSingleColorForIndices<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredcolor: super::super::UI::Color, lampIndexes_array_size: u32, lampindexes: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSingleColorForIndices(&*(&desiredcolor as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&lampindexes), lampIndexes_array_size as _)).into()
        }
        unsafe extern "system" fn SetColorsForIndices<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredColors_array_size: u32, desiredcolors: *const super::super::UI::Color, lampIndexes_array_size: u32, lampindexes: *const i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorsForIndices(::core::slice::from_raw_parts(::core::mem::transmute_copy(&desiredcolors), desiredColors_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&lampindexes), lampIndexes_array_size as _)).into()
        }
        unsafe extern "system" fn SetColorsForKey<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredcolor: super::super::UI::Color, key: super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorsForKey(&*(&desiredcolor as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType), key).into()
        }
        unsafe extern "system" fn SetColorsForKeys<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredColors_array_size: u32, desiredcolors: *const super::super::UI::Color, keys_array_size: u32, keys: *const super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorsForKeys(::core::slice::from_raw_parts(::core::mem::transmute_copy(&desiredcolors), desiredColors_array_size as _), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&keys), keys_array_size as _)).into()
        }
        unsafe extern "system" fn SetColorsForPurposes<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredcolor: super::super::UI::Color, purposes: LampPurposes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorsForPurposes(&*(&desiredcolor as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType), purposes).into()
        }
        unsafe extern "system" fn SendMessageAsync<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageid: i32, message: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendMessageAsync(messageid, &*(&message as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestMessageAsync<Impl: ILampArrayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messageid: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestMessageAsync(messageid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILampArray, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            HardwareVendorId: HardwareVendorId::<Impl, IMPL_OFFSET>,
            HardwareProductId: HardwareProductId::<Impl, IMPL_OFFSET>,
            HardwareVersion: HardwareVersion::<Impl, IMPL_OFFSET>,
            LampArrayKind: LampArrayKind::<Impl, IMPL_OFFSET>,
            LampCount: LampCount::<Impl, IMPL_OFFSET>,
            MinUpdateInterval: MinUpdateInterval::<Impl, IMPL_OFFSET>,
            BoundingBox: BoundingBox::<Impl, IMPL_OFFSET>,
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            SetIsEnabled: SetIsEnabled::<Impl, IMPL_OFFSET>,
            BrightnessLevel: BrightnessLevel::<Impl, IMPL_OFFSET>,
            SetBrightnessLevel: SetBrightnessLevel::<Impl, IMPL_OFFSET>,
            IsConnected: IsConnected::<Impl, IMPL_OFFSET>,
            SupportsVirtualKeys: SupportsVirtualKeys::<Impl, IMPL_OFFSET>,
            GetLampInfo: GetLampInfo::<Impl, IMPL_OFFSET>,
            GetIndicesForKey: GetIndicesForKey::<Impl, IMPL_OFFSET>,
            GetIndicesForPurposes: GetIndicesForPurposes::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
            SetColorForIndex: SetColorForIndex::<Impl, IMPL_OFFSET>,
            SetSingleColorForIndices: SetSingleColorForIndices::<Impl, IMPL_OFFSET>,
            SetColorsForIndices: SetColorsForIndices::<Impl, IMPL_OFFSET>,
            SetColorsForKey: SetColorsForKey::<Impl, IMPL_OFFSET>,
            SetColorsForKeys: SetColorsForKeys::<Impl, IMPL_OFFSET>,
            SetColorsForPurposes: SetColorsForPurposes::<Impl, IMPL_OFFSET>,
            SendMessageAsync: SendMessageAsync::<Impl, IMPL_OFFSET>,
            RequestMessageAsync: RequestMessageAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILampArray as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILampArrayStaticsImpl: Sized {
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LampArray>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILampArrayStatics {
    const NAME: &'static str = "Windows.Devices.Lights.ILampArrayStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILampArrayStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampArrayStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILampArrayStaticsVtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: ILampArrayStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromIdAsync<Impl: ILampArrayStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILampArrayStatics, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILampArrayStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILampAvailabilityChangedEventArgsImpl: Sized {
    fn IsAvailable(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILampAvailabilityChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Lights.ILampAvailabilityChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ILampAvailabilityChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampAvailabilityChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILampAvailabilityChangedEventArgsVtbl {
        unsafe extern "system" fn IsAvailable<Impl: ILampAvailabilityChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILampAvailabilityChangedEventArgs, BASE_OFFSET>(),
            IsAvailable: IsAvailable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILampAvailabilityChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "UI", feature = "implement_exclusive"))]
pub trait ILampInfoImpl: Sized {
    fn Index(&mut self) -> ::windows::core::Result<i32>;
    fn Purposes(&mut self) -> ::windows::core::Result<LampPurposes>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn RedLevelCount(&mut self) -> ::windows::core::Result<i32>;
    fn GreenLevelCount(&mut self) -> ::windows::core::Result<i32>;
    fn BlueLevelCount(&mut self) -> ::windows::core::Result<i32>;
    fn GainLevelCount(&mut self) -> ::windows::core::Result<i32>;
    fn FixedColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::UI::Color>>;
    fn GetNearestSupportedColor(&mut self, desiredcolor: &super::super::UI::Color) -> ::windows::core::Result<super::super::UI::Color>;
    fn UpdateLatency(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "UI", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILampInfo {
    const NAME: &'static str = "Windows.Devices.Lights.ILampInfo";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "UI", feature = "implement_exclusive"))]
impl ILampInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILampInfoVtbl {
        unsafe extern "system" fn Index<Impl: ILampInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Index() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Purposes<Impl: ILampInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LampPurposes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Purposes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: ILampInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedLevelCount<Impl: ILampInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RedLevelCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GreenLevelCount<Impl: ILampInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GreenLevelCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlueLevelCount<Impl: ILampInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlueLevelCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GainLevelCount<Impl: ILampInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GainLevelCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FixedColor<Impl: ILampInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FixedColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNearestSupportedColor<Impl: ILampInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredcolor: super::super::UI::Color, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNearestSupportedColor(&*(&desiredcolor as *const <super::super::UI::Color as ::windows::core::Abi>::Abi as *const <super::super::UI::Color as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateLatency<Impl: ILampInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILampInfo, BASE_OFFSET>(),
            Index: Index::<Impl, IMPL_OFFSET>,
            Purposes: Purposes::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            RedLevelCount: RedLevelCount::<Impl, IMPL_OFFSET>,
            GreenLevelCount: GreenLevelCount::<Impl, IMPL_OFFSET>,
            BlueLevelCount: BlueLevelCount::<Impl, IMPL_OFFSET>,
            GainLevelCount: GainLevelCount::<Impl, IMPL_OFFSET>,
            FixedColor: FixedColor::<Impl, IMPL_OFFSET>,
            GetNearestSupportedColor: GetNearestSupportedColor::<Impl, IMPL_OFFSET>,
            UpdateLatency: UpdateLatency::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILampInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILampStaticsImpl: Sized {
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Lamp>>;
    fn GetDefaultAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Lamp>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILampStatics {
    const NAME: &'static str = "Windows.Devices.Lights.ILampStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILampStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILampStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILampStaticsVtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: ILampStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromIdAsync<Impl: ILampStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDefaultAsync<Impl: ILampStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILampStatics, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            GetDefaultAsync: GetDefaultAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILampStatics as ::windows::core::Interface>::IID
    }
}
