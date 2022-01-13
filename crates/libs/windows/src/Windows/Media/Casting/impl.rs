#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICastingConnectionImpl: Sized + IClosableImpl {
    fn State(&mut self) -> ::windows::core::Result<CastingConnectionState>;
    fn Device(&mut self) -> ::windows::core::Result<CastingDevice>;
    fn Source(&mut self) -> ::windows::core::Result<CastingSource>;
    fn SetSource(&mut self, value: &::core::option::Option<CastingSource>) -> ::windows::core::Result<()>;
    fn StateChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CastingConnection, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ErrorOccurred(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CastingConnection, CastingConnectionErrorOccurredEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveErrorOccurred(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RequestStartCastingAsync(&mut self, value: &::core::option::Option<CastingSource>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CastingConnectionErrorStatus>>;
    fn DisconnectAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CastingConnectionErrorStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICastingConnection {
    const NAME: &'static str = "Windows.Media.Casting.ICastingConnection";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICastingConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICastingConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICastingConnectionVtbl {
        unsafe extern "system" fn State<Impl: ICastingConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CastingConnectionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Device<Impl: ICastingConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Device() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Source<Impl: ICastingConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSource<Impl: ICastingConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSource(&*(&value as *const <CastingSource as ::windows::core::Abi>::Abi as *const <CastingSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StateChanged<Impl: ICastingConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CastingConnection, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CastingConnection, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStateChanged<Impl: ICastingConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ErrorOccurred<Impl: ICastingConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorOccurred(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CastingConnection, CastingConnectionErrorOccurredEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CastingConnection, CastingConnectionErrorOccurredEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveErrorOccurred<Impl: ICastingConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveErrorOccurred(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequestStartCastingAsync<Impl: ICastingConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestStartCastingAsync(&*(&value as *const <CastingSource as ::windows::core::Abi>::Abi as *const <CastingSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectAsync<Impl: ICastingConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisconnectAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICastingConnection, BASE_OFFSET>(),
            State: State::<Impl, IMPL_OFFSET>,
            Device: Device::<Impl, IMPL_OFFSET>,
            Source: Source::<Impl, IMPL_OFFSET>,
            SetSource: SetSource::<Impl, IMPL_OFFSET>,
            StateChanged: StateChanged::<Impl, IMPL_OFFSET>,
            RemoveStateChanged: RemoveStateChanged::<Impl, IMPL_OFFSET>,
            ErrorOccurred: ErrorOccurred::<Impl, IMPL_OFFSET>,
            RemoveErrorOccurred: RemoveErrorOccurred::<Impl, IMPL_OFFSET>,
            RequestStartCastingAsync: RequestStartCastingAsync::<Impl, IMPL_OFFSET>,
            DisconnectAsync: DisconnectAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICastingConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICastingConnectionErrorOccurredEventArgsImpl: Sized {
    fn ErrorStatus(&mut self) -> ::windows::core::Result<CastingConnectionErrorStatus>;
    fn Message(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICastingConnectionErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Media.Casting.ICastingConnectionErrorOccurredEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICastingConnectionErrorOccurredEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICastingConnectionErrorOccurredEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICastingConnectionErrorOccurredEventArgsVtbl {
        unsafe extern "system" fn ErrorStatus<Impl: ICastingConnectionErrorOccurredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CastingConnectionErrorStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Impl: ICastingConnectionErrorOccurredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICastingConnectionErrorOccurredEventArgs, BASE_OFFSET>(),
            ErrorStatus: ErrorStatus::<Impl, IMPL_OFFSET>,
            Message: Message::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICastingConnectionErrorOccurredEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ICastingDeviceImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FriendlyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Icon(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType>;
    fn GetSupportedCastingPlaybackTypesAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CastingPlaybackTypes>>;
    fn CreateCastingConnection(&mut self) -> ::windows::core::Result<CastingConnection>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICastingDevice {
    const NAME: &'static str = "Windows.Media.Casting.ICastingDevice";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ICastingDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICastingDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICastingDeviceVtbl {
        unsafe extern "system" fn Id<Impl: ICastingDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FriendlyName<Impl: ICastingDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Icon<Impl: ICastingDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Icon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedCastingPlaybackTypesAsync<Impl: ICastingDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedCastingPlaybackTypesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCastingConnection<Impl: ICastingDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCastingConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICastingDevice, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            FriendlyName: FriendlyName::<Impl, IMPL_OFFSET>,
            Icon: Icon::<Impl, IMPL_OFFSET>,
            GetSupportedCastingPlaybackTypesAsync: GetSupportedCastingPlaybackTypesAsync::<Impl, IMPL_OFFSET>,
            CreateCastingConnection: CreateCastingConnection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICastingDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
pub trait ICastingDevicePickerImpl: Sized {
    fn Filter(&mut self) -> ::windows::core::Result<CastingDevicePickerFilter>;
    fn Appearance(&mut self) -> ::windows::core::Result<super::super::Devices::Enumeration::DevicePickerAppearance>;
    fn CastingDeviceSelected(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CastingDevicePicker, CastingDeviceSelectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCastingDeviceSelected(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CastingDevicePickerDismissed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CastingDevicePicker, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCastingDevicePickerDismissed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Show(&mut self, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn ShowWithPlacement(&mut self, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<()>;
    fn Hide(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICastingDevicePicker {
    const NAME: &'static str = "Windows.Media.Casting.ICastingDevicePicker";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
impl ICastingDevicePickerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICastingDevicePickerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICastingDevicePickerVtbl {
        unsafe extern "system" fn Filter<Impl: ICastingDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Filter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Appearance<Impl: ICastingDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Appearance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CastingDeviceSelected<Impl: ICastingDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CastingDeviceSelected(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CastingDevicePicker, CastingDeviceSelectedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CastingDevicePicker, CastingDeviceSelectedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCastingDeviceSelected<Impl: ICastingDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCastingDeviceSelected(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CastingDevicePickerDismissed<Impl: ICastingDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CastingDevicePickerDismissed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CastingDevicePicker, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CastingDevicePicker, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCastingDevicePickerDismissed<Impl: ICastingDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCastingDevicePickerDismissed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Show<Impl: ICastingDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show(&*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShowWithPlacement<Impl: ICastingDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowWithPlacement(&*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), preferredplacement).into()
        }
        unsafe extern "system" fn Hide<Impl: ICastingDevicePickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Hide().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICastingDevicePicker, BASE_OFFSET>(),
            Filter: Filter::<Impl, IMPL_OFFSET>,
            Appearance: Appearance::<Impl, IMPL_OFFSET>,
            CastingDeviceSelected: CastingDeviceSelected::<Impl, IMPL_OFFSET>,
            RemoveCastingDeviceSelected: RemoveCastingDeviceSelected::<Impl, IMPL_OFFSET>,
            CastingDevicePickerDismissed: CastingDevicePickerDismissed::<Impl, IMPL_OFFSET>,
            RemoveCastingDevicePickerDismissed: RemoveCastingDevicePickerDismissed::<Impl, IMPL_OFFSET>,
            Show: Show::<Impl, IMPL_OFFSET>,
            ShowWithPlacement: ShowWithPlacement::<Impl, IMPL_OFFSET>,
            Hide: Hide::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICastingDevicePicker as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICastingDevicePickerFilterImpl: Sized {
    fn SupportsAudio(&mut self) -> ::windows::core::Result<bool>;
    fn SetSupportsAudio(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn SupportsVideo(&mut self) -> ::windows::core::Result<bool>;
    fn SetSupportsVideo(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn SupportsPictures(&mut self) -> ::windows::core::Result<bool>;
    fn SetSupportsPictures(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn SupportedCastingSources(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<CastingSource>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICastingDevicePickerFilter {
    const NAME: &'static str = "Windows.Media.Casting.ICastingDevicePickerFilter";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICastingDevicePickerFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICastingDevicePickerFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICastingDevicePickerFilterVtbl {
        unsafe extern "system" fn SupportsAudio<Impl: ICastingDevicePickerFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsAudio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportsAudio<Impl: ICastingDevicePickerFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSupportsAudio(value).into()
        }
        unsafe extern "system" fn SupportsVideo<Impl: ICastingDevicePickerFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsVideo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportsVideo<Impl: ICastingDevicePickerFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSupportsVideo(value).into()
        }
        unsafe extern "system" fn SupportsPictures<Impl: ICastingDevicePickerFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsPictures() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportsPictures<Impl: ICastingDevicePickerFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSupportsPictures(value).into()
        }
        unsafe extern "system" fn SupportedCastingSources<Impl: ICastingDevicePickerFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedCastingSources() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICastingDevicePickerFilter, BASE_OFFSET>(),
            SupportsAudio: SupportsAudio::<Impl, IMPL_OFFSET>,
            SetSupportsAudio: SetSupportsAudio::<Impl, IMPL_OFFSET>,
            SupportsVideo: SupportsVideo::<Impl, IMPL_OFFSET>,
            SetSupportsVideo: SetSupportsVideo::<Impl, IMPL_OFFSET>,
            SupportsPictures: SupportsPictures::<Impl, IMPL_OFFSET>,
            SetSupportsPictures: SetSupportsPictures::<Impl, IMPL_OFFSET>,
            SupportedCastingSources: SupportedCastingSources::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICastingDevicePickerFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICastingDeviceSelectedEventArgsImpl: Sized {
    fn SelectedCastingDevice(&mut self) -> ::windows::core::Result<CastingDevice>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICastingDeviceSelectedEventArgs {
    const NAME: &'static str = "Windows.Media.Casting.ICastingDeviceSelectedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICastingDeviceSelectedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICastingDeviceSelectedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICastingDeviceSelectedEventArgsVtbl {
        unsafe extern "system" fn SelectedCastingDevice<Impl: ICastingDeviceSelectedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedCastingDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICastingDeviceSelectedEventArgs, BASE_OFFSET>(),
            SelectedCastingDevice: SelectedCastingDevice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICastingDeviceSelectedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICastingDeviceStaticsImpl: Sized {
    fn GetDeviceSelector(&mut self, r#type: CastingPlaybackTypes) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromCastingSourceAsync(&mut self, castingsource: &::core::option::Option<CastingSource>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn FromIdAsync(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CastingDevice>>;
    fn DeviceInfoSupportsCastingAsync(&mut self, device: &::core::option::Option<super::super::Devices::Enumeration::DeviceInformation>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICastingDeviceStatics {
    const NAME: &'static str = "Windows.Media.Casting.ICastingDeviceStatics";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "implement_exclusive"))]
impl ICastingDeviceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICastingDeviceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICastingDeviceStaticsVtbl {
        unsafe extern "system" fn GetDeviceSelector<Impl: ICastingDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: CastingPlaybackTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelectorFromCastingSourceAsync<Impl: ICastingDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, castingsource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelectorFromCastingSourceAsync(&*(&castingsource as *const <CastingSource as ::windows::core::Abi>::Abi as *const <CastingSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: ICastingDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceInfoSupportsCastingAsync<Impl: ICastingDeviceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceInfoSupportsCastingAsync(&*(&device as *const <super::super::Devices::Enumeration::DeviceInformation as ::windows::core::Abi>::Abi as *const <super::super::Devices::Enumeration::DeviceInformation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICastingDeviceStatics, BASE_OFFSET>(),
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            GetDeviceSelectorFromCastingSourceAsync: GetDeviceSelectorFromCastingSourceAsync::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            DeviceInfoSupportsCastingAsync: DeviceInfoSupportsCastingAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICastingDeviceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICastingSourceImpl: Sized {
    fn PreferredSourceUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetPreferredSourceUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICastingSource {
    const NAME: &'static str = "Windows.Media.Casting.ICastingSource";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICastingSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICastingSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICastingSourceVtbl {
        unsafe extern "system" fn PreferredSourceUri<Impl: ICastingSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredSourceUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredSourceUri<Impl: ICastingSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredSourceUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICastingSource, BASE_OFFSET>(),
            PreferredSourceUri: PreferredSourceUri::<Impl, IMPL_OFFSET>,
            SetPreferredSourceUri: SetPreferredSourceUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICastingSource as ::windows::core::Interface>::IID
    }
}
