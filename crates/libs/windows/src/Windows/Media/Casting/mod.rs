#[doc(hidden)]
#[repr(transparent)]
pub struct ICastingConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICastingConnection {
    type Vtable = ICastingConnection_Vtbl;
}
impl ::core::clone::Clone for ICastingConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICastingConnection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd951653_c2f1_4498_8b78_5fb4cd3640dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingConnection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CastingConnectionState) -> ::windows::core::HRESULT,
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ErrorOccurred: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveErrorOccurred: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStartCastingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStartCastingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisconnectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisconnectAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICastingConnectionErrorOccurredEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICastingConnectionErrorOccurredEventArgs {
    type Vtable = ICastingConnectionErrorOccurredEventArgs_Vtbl;
}
impl ::core::clone::Clone for ICastingConnectionErrorOccurredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICastingConnectionErrorOccurredEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7fb3c69_8719_4f00_81fb_961863c79a32);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingConnectionErrorOccurredEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ErrorStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CastingConnectionErrorStatus) -> ::windows::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICastingDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICastingDevice {
    type Vtable = ICastingDevice_Vtbl;
}
impl ::core::clone::Clone for ICastingDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICastingDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde721c83_4a43_4ad1_a6d2_2492a796c3f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingDevice_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Icon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Icon: usize,
    #[cfg(feature = "Foundation")]
    pub GetSupportedCastingPlaybackTypesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSupportedCastingPlaybackTypesAsync: usize,
    pub CreateCastingConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICastingDevicePicker(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICastingDevicePicker {
    type Vtable = ICastingDevicePicker_Vtbl;
}
impl ::core::clone::Clone for ICastingDevicePicker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICastingDevicePicker {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdcd39924_0591_49be_aacb_4b82ee756a95);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingDevicePicker_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Filter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub Appearance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    Appearance: usize,
    #[cfg(feature = "Foundation")]
    pub CastingDeviceSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CastingDeviceSelected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCastingDeviceSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCastingDeviceSelected: usize,
    #[cfg(feature = "Foundation")]
    pub CastingDevicePickerDismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CastingDevicePickerDismissed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCastingDevicePickerDismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCastingDevicePickerDismissed: usize,
    #[cfg(feature = "Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Show: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowWithPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowWithPlacement: usize,
    pub Hide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICastingDevicePickerFilter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICastingDevicePickerFilter {
    type Vtable = ICastingDevicePickerFilter_Vtbl;
}
impl ::core::clone::Clone for ICastingDevicePickerFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICastingDevicePickerFilter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbe8c619c_b563_4354_ae33_9fdaad8c6291);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingDevicePickerFilter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SupportsAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetSupportsAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SupportsVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetSupportsVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SupportsPictures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetSupportsPictures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCastingSources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCastingSources: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICastingDeviceSelectedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICastingDeviceSelectedEventArgs {
    type Vtable = ICastingDeviceSelectedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ICastingDeviceSelectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICastingDeviceSelectedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc439e86_dd57_4d0d_9400_af45e4fb3663);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingDeviceSelectedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SelectedCastingDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICastingDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICastingDeviceStatics {
    type Vtable = ICastingDeviceStatics_Vtbl;
}
impl ::core::clone::Clone for ICastingDeviceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICastingDeviceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7d958d7_4d13_4237_a365_4c4f6a4cfd2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingDeviceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: CastingPlaybackTypes, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeviceSelectorFromCastingSourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, castingsource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeviceSelectorFromCastingSourceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub DeviceInfoSupportsCastingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    DeviceInfoSupportsCastingAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICastingSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICastingSource {
    type Vtable = ICastingSource_Vtbl;
}
impl ::core::clone::Clone for ICastingSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICastingSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf429ea72_3467_47e6_a027_522923e9d727);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingSource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PreferredSourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreferredSourceUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetPreferredSourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPreferredSourceUri: usize,
}
#[doc = "*Required features: `\"Media_Casting\"`*"]
#[repr(transparent)]
pub struct CastingConnection(::windows::core::IUnknown);
impl CastingConnection {
    pub fn State(&self) -> ::windows::core::Result<CastingConnectionState> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CastingConnectionState>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Device(&self) -> ::windows::core::Result<CastingDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CastingDevice>();
            (::windows::core::Interface::vtable(this).Device)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Source(&self) -> ::windows::core::Result<CastingSource> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CastingSource>();
            (::windows::core::Interface::vtable(this).Source)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSource(&self, value: &CastingSource) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSource)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged(&self, handler: &super::super::Foundation::TypedEventHandler<CastingConnection, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).StateChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStateChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorOccurred(&self, handler: &super::super::Foundation::TypedEventHandler<CastingConnection, CastingConnectionErrorOccurredEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ErrorOccurred)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveErrorOccurred(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveErrorOccurred)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStartCastingAsync(&self, value: &CastingSource) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CastingConnectionErrorStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<CastingConnectionErrorStatus>>();
            (::windows::core::Interface::vtable(this).RequestStartCastingAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisconnectAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CastingConnectionErrorStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<CastingConnectionErrorStatus>>();
            (::windows::core::Interface::vtable(this).DisconnectAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for CastingConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CastingConnection {}
impl ::core::fmt::Debug for CastingConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CastingConnection").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CastingConnection {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingConnection;{cd951653-c2f1-4498-8b78-5fb4cd3640dd})");
}
impl ::core::clone::Clone for CastingConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CastingConnection {
    type Vtable = ICastingConnection_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CastingConnection {
    const IID: ::windows::core::GUID = <ICastingConnection as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CastingConnection {
    const NAME: &'static str = "Windows.Media.Casting.CastingConnection";
}
::windows::imp::interface_hierarchy!(CastingConnection, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::Foundation::IClosable> for CastingConnection {}
unsafe impl ::core::marker::Send for CastingConnection {}
unsafe impl ::core::marker::Sync for CastingConnection {}
#[doc = "*Required features: `\"Media_Casting\"`*"]
#[repr(transparent)]
pub struct CastingConnectionErrorOccurredEventArgs(::windows::core::IUnknown);
impl CastingConnectionErrorOccurredEventArgs {
    pub fn ErrorStatus(&self) -> ::windows::core::Result<CastingConnectionErrorStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CastingConnectionErrorStatus>();
            (::windows::core::Interface::vtable(this).ErrorStatus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Message)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CastingConnectionErrorOccurredEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CastingConnectionErrorOccurredEventArgs {}
impl ::core::fmt::Debug for CastingConnectionErrorOccurredEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CastingConnectionErrorOccurredEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CastingConnectionErrorOccurredEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingConnectionErrorOccurredEventArgs;{a7fb3c69-8719-4f00-81fb-961863c79a32})");
}
impl ::core::clone::Clone for CastingConnectionErrorOccurredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CastingConnectionErrorOccurredEventArgs {
    type Vtable = ICastingConnectionErrorOccurredEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CastingConnectionErrorOccurredEventArgs {
    const IID: ::windows::core::GUID = <ICastingConnectionErrorOccurredEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CastingConnectionErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Media.Casting.CastingConnectionErrorOccurredEventArgs";
}
::windows::imp::interface_hierarchy!(CastingConnectionErrorOccurredEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CastingConnectionErrorOccurredEventArgs {}
unsafe impl ::core::marker::Sync for CastingConnectionErrorOccurredEventArgs {}
#[doc = "*Required features: `\"Media_Casting\"`*"]
#[repr(transparent)]
pub struct CastingDevice(::windows::core::IUnknown);
impl CastingDevice {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).FriendlyName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Icon(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IRandomAccessStreamWithContentType>();
            (::windows::core::Interface::vtable(this).Icon)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSupportedCastingPlaybackTypesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CastingPlaybackTypes>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<CastingPlaybackTypes>>();
            (::windows::core::Interface::vtable(this).GetSupportedCastingPlaybackTypesAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateCastingConnection(&self) -> ::windows::core::Result<CastingConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CastingConnection>();
            (::windows::core::Interface::vtable(this).CreateCastingConnection)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeviceSelector(r#type: CastingPlaybackTypes) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICastingDeviceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), r#type, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeviceSelectorFromCastingSourceAsync(castingsource: &CastingSource) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::ICastingDeviceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorFromCastingSourceAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(castingsource), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(value: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CastingDevice>> {
        Self::ICastingDeviceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<CastingDevice>>();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub fn DeviceInfoSupportsCastingAsync(device: &super::super::Devices::Enumeration::DeviceInformation) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ICastingDeviceStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).DeviceInfoSupportsCastingAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(device), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICastingDeviceStatics<R, F: FnOnce(&ICastingDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<CastingDevice, ICastingDeviceStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for CastingDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CastingDevice {}
impl ::core::fmt::Debug for CastingDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CastingDevice").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CastingDevice {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingDevice;{de721c83-4a43-4ad1-a6d2-2492a796c3f2})");
}
impl ::core::clone::Clone for CastingDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CastingDevice {
    type Vtable = ICastingDevice_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CastingDevice {
    const IID: ::windows::core::GUID = <ICastingDevice as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CastingDevice {
    const NAME: &'static str = "Windows.Media.Casting.CastingDevice";
}
::windows::imp::interface_hierarchy!(CastingDevice, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CastingDevice {}
unsafe impl ::core::marker::Sync for CastingDevice {}
#[doc = "*Required features: `\"Media_Casting\"`*"]
#[repr(transparent)]
pub struct CastingDevicePicker(::windows::core::IUnknown);
impl CastingDevicePicker {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<CastingDevicePicker, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Filter(&self) -> ::windows::core::Result<CastingDevicePickerFilter> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CastingDevicePickerFilter>();
            (::windows::core::Interface::vtable(this).Filter)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn Appearance(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DevicePickerAppearance> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Devices::Enumeration::DevicePickerAppearance>();
            (::windows::core::Interface::vtable(this).Appearance)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CastingDeviceSelected(&self, handler: &super::super::Foundation::TypedEventHandler<CastingDevicePicker, CastingDeviceSelectedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).CastingDeviceSelected)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCastingDeviceSelected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCastingDeviceSelected)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CastingDevicePickerDismissed(&self, handler: &super::super::Foundation::TypedEventHandler<CastingDevicePicker, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).CastingDevicePickerDismissed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCastingDevicePickerDismissed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCastingDevicePickerDismissed)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Show(&self, selection: super::super::Foundation::Rect) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Show)(::windows::core::Interface::as_raw(this), selection).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowWithPlacement(&self, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ShowWithPlacement)(::windows::core::Interface::as_raw(this), selection, preferredplacement).ok() }
    }
    pub fn Hide(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Hide)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for CastingDevicePicker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CastingDevicePicker {}
impl ::core::fmt::Debug for CastingDevicePicker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CastingDevicePicker").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CastingDevicePicker {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingDevicePicker;{dcd39924-0591-49be-aacb-4b82ee756a95})");
}
impl ::core::clone::Clone for CastingDevicePicker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CastingDevicePicker {
    type Vtable = ICastingDevicePicker_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CastingDevicePicker {
    const IID: ::windows::core::GUID = <ICastingDevicePicker as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CastingDevicePicker {
    const NAME: &'static str = "Windows.Media.Casting.CastingDevicePicker";
}
::windows::imp::interface_hierarchy!(CastingDevicePicker, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CastingDevicePicker {}
unsafe impl ::core::marker::Sync for CastingDevicePicker {}
#[doc = "*Required features: `\"Media_Casting\"`*"]
#[repr(transparent)]
pub struct CastingDevicePickerFilter(::windows::core::IUnknown);
impl CastingDevicePickerFilter {
    pub fn SupportsAudio(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).SupportsAudio)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSupportsAudio(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSupportsAudio)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SupportsVideo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).SupportsVideo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSupportsVideo(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSupportsVideo)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SupportsPictures(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).SupportsPictures)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSupportsPictures(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSupportsPictures)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCastingSources(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<CastingSource>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<CastingSource>>();
            (::windows::core::Interface::vtable(this).SupportedCastingSources)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CastingDevicePickerFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CastingDevicePickerFilter {}
impl ::core::fmt::Debug for CastingDevicePickerFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CastingDevicePickerFilter").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CastingDevicePickerFilter {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingDevicePickerFilter;{be8c619c-b563-4354-ae33-9fdaad8c6291})");
}
impl ::core::clone::Clone for CastingDevicePickerFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CastingDevicePickerFilter {
    type Vtable = ICastingDevicePickerFilter_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CastingDevicePickerFilter {
    const IID: ::windows::core::GUID = <ICastingDevicePickerFilter as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CastingDevicePickerFilter {
    const NAME: &'static str = "Windows.Media.Casting.CastingDevicePickerFilter";
}
::windows::imp::interface_hierarchy!(CastingDevicePickerFilter, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CastingDevicePickerFilter {}
unsafe impl ::core::marker::Sync for CastingDevicePickerFilter {}
#[doc = "*Required features: `\"Media_Casting\"`*"]
#[repr(transparent)]
pub struct CastingDeviceSelectedEventArgs(::windows::core::IUnknown);
impl CastingDeviceSelectedEventArgs {
    pub fn SelectedCastingDevice(&self) -> ::windows::core::Result<CastingDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CastingDevice>();
            (::windows::core::Interface::vtable(this).SelectedCastingDevice)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CastingDeviceSelectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CastingDeviceSelectedEventArgs {}
impl ::core::fmt::Debug for CastingDeviceSelectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CastingDeviceSelectedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CastingDeviceSelectedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingDeviceSelectedEventArgs;{dc439e86-dd57-4d0d-9400-af45e4fb3663})");
}
impl ::core::clone::Clone for CastingDeviceSelectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CastingDeviceSelectedEventArgs {
    type Vtable = ICastingDeviceSelectedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CastingDeviceSelectedEventArgs {
    const IID: ::windows::core::GUID = <ICastingDeviceSelectedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CastingDeviceSelectedEventArgs {
    const NAME: &'static str = "Windows.Media.Casting.CastingDeviceSelectedEventArgs";
}
::windows::imp::interface_hierarchy!(CastingDeviceSelectedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CastingDeviceSelectedEventArgs {}
unsafe impl ::core::marker::Sync for CastingDeviceSelectedEventArgs {}
#[doc = "*Required features: `\"Media_Casting\"`*"]
#[repr(transparent)]
pub struct CastingSource(::windows::core::IUnknown);
impl CastingSource {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PreferredSourceUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).PreferredSourceUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPreferredSourceUri(&self, value: &super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPreferredSourceUri)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::cmp::PartialEq for CastingSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CastingSource {}
impl ::core::fmt::Debug for CastingSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CastingSource").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CastingSource {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingSource;{f429ea72-3467-47e6-a027-522923e9d727})");
}
impl ::core::clone::Clone for CastingSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CastingSource {
    type Vtable = ICastingSource_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CastingSource {
    const IID: ::windows::core::GUID = <ICastingSource as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CastingSource {
    const NAME: &'static str = "Windows.Media.Casting.CastingSource";
}
::windows::imp::interface_hierarchy!(CastingSource, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CastingSource {}
unsafe impl ::core::marker::Sync for CastingSource {}
#[doc = "*Required features: `\"Media_Casting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CastingConnectionErrorStatus(pub i32);
impl CastingConnectionErrorStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const DeviceDidNotRespond: Self = Self(1i32);
    pub const DeviceError: Self = Self(2i32);
    pub const DeviceLocked: Self = Self(3i32);
    pub const ProtectedPlaybackFailed: Self = Self(4i32);
    pub const InvalidCastingSource: Self = Self(5i32);
    pub const Unknown: Self = Self(6i32);
}
impl ::core::marker::Copy for CastingConnectionErrorStatus {}
impl ::core::clone::Clone for CastingConnectionErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CastingConnectionErrorStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CastingConnectionErrorStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CastingConnectionErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CastingConnectionErrorStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CastingConnectionErrorStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Casting.CastingConnectionErrorStatus;i4)");
}
#[doc = "*Required features: `\"Media_Casting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CastingConnectionState(pub i32);
impl CastingConnectionState {
    pub const Disconnected: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
    pub const Rendering: Self = Self(2i32);
    pub const Disconnecting: Self = Self(3i32);
    pub const Connecting: Self = Self(4i32);
}
impl ::core::marker::Copy for CastingConnectionState {}
impl ::core::clone::Clone for CastingConnectionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CastingConnectionState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CastingConnectionState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CastingConnectionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CastingConnectionState").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CastingConnectionState {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Casting.CastingConnectionState;i4)");
}
#[doc = "*Required features: `\"Media_Casting\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CastingPlaybackTypes(pub u32);
impl CastingPlaybackTypes {
    pub const None: Self = Self(0u32);
    pub const Audio: Self = Self(1u32);
    pub const Video: Self = Self(2u32);
    pub const Picture: Self = Self(4u32);
}
impl ::core::marker::Copy for CastingPlaybackTypes {}
impl ::core::clone::Clone for CastingPlaybackTypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CastingPlaybackTypes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CastingPlaybackTypes {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CastingPlaybackTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CastingPlaybackTypes").field(&self.0).finish()
    }
}
impl CastingPlaybackTypes {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CastingPlaybackTypes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CastingPlaybackTypes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CastingPlaybackTypes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CastingPlaybackTypes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CastingPlaybackTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for CastingPlaybackTypes {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Casting.CastingPlaybackTypes;u4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
