#[doc(hidden)]
#[repr(transparent)]
pub struct IGazeDevicePreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGazeDevicePreview {
    type Vtable = IGazeDevicePreview_Vtbl;
}
impl ::core::clone::Clone for IGazeDevicePreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGazeDevicePreview {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe79e7ee9_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDevicePreview_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub CanTrackEyes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanTrackHead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ConfigurationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GazeDeviceConfigurationStatePreview) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestCalibrationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestCalibrationAsync: usize,
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))]
    pub GetNumericControlDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections")))]
    GetNumericControlDescriptions: usize,
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))]
    pub GetBooleanControlDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections")))]
    GetBooleanControlDescriptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGazeDeviceWatcherAddedPreviewEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGazeDeviceWatcherAddedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherAddedPreviewEventArgs_Vtbl;
}
impl ::core::clone::Clone for IGazeDeviceWatcherAddedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGazeDeviceWatcherAddedPreviewEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe79e7eed_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherAddedPreviewEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGazeDeviceWatcherPreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGazeDeviceWatcherPreview {
    type Vtable = IGazeDeviceWatcherPreview_Vtbl;
}
impl ::core::clone::Clone for IGazeDeviceWatcherPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGazeDeviceWatcherPreview {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe79e7ee7_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherPreview_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub Removed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Removed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGazeDeviceWatcherRemovedPreviewEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGazeDeviceWatcherRemovedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherRemovedPreviewEventArgs_Vtbl;
}
impl ::core::clone::Clone for IGazeDeviceWatcherRemovedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGazeDeviceWatcherRemovedPreviewEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2631f08_0e3f_431f_a606_50b35af94a1c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherRemovedPreviewEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGazeDeviceWatcherUpdatedPreviewEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGazeDeviceWatcherUpdatedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherUpdatedPreviewEventArgs_Vtbl;
}
impl ::core::clone::Clone for IGazeDeviceWatcherUpdatedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGazeDeviceWatcherUpdatedPreviewEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fe830ef_7f08_4737_88e1_4a83ae4e4885);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherUpdatedPreviewEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGazeEnteredPreviewEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGazeEnteredPreviewEventArgs {
    type Vtable = IGazeEnteredPreviewEventArgs_Vtbl;
}
impl ::core::clone::Clone for IGazeEnteredPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGazeEnteredPreviewEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2567bf43_1225_489f_9dd1_daa7c50fbf4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeEnteredPreviewEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CurrentPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGazeExitedPreviewEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGazeExitedPreviewEventArgs {
    type Vtable = IGazeExitedPreviewEventArgs_Vtbl;
}
impl ::core::clone::Clone for IGazeExitedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGazeExitedPreviewEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d0af07e_7d83_40ef_9f0a_fbc1bbdcc5ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeExitedPreviewEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CurrentPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGazeInputSourcePreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGazeInputSourcePreview {
    type Vtable = IGazeInputSourcePreview_Vtbl;
}
impl ::core::clone::Clone for IGazeInputSourcePreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGazeInputSourcePreview {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe79e7ee8_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeInputSourcePreview_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GazeMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GazeMoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGazeMoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGazeMoved: usize,
    #[cfg(feature = "Foundation")]
    pub GazeEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GazeEntered: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGazeEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGazeEntered: usize,
    #[cfg(feature = "Foundation")]
    pub GazeExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GazeExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGazeExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGazeExited: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGazeInputSourcePreviewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGazeInputSourcePreviewStatics {
    type Vtable = IGazeInputSourcePreviewStatics_Vtbl;
}
impl ::core::clone::Clone for IGazeInputSourcePreviewStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGazeInputSourcePreviewStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe79e7ee6_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeInputSourcePreviewStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGazeMovedPreviewEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGazeMovedPreviewEventArgs {
    type Vtable = IGazeMovedPreviewEventArgs_Vtbl;
}
impl ::core::clone::Clone for IGazeMovedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGazeMovedPreviewEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe79e7eeb_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeMovedPreviewEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CurrentPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetIntermediatePoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetIntermediatePoints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGazePointPreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGazePointPreview {
    type Vtable = IGazePointPreview_Vtbl;
}
impl ::core::clone::Clone for IGazePointPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGazePointPreview {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe79e7eea_b389_11e7_b201_c8d3ffb75721);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazePointPreview_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SourceDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EyeGazePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EyeGazePosition: usize,
    #[cfg(feature = "Foundation")]
    pub HeadGazePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HeadGazePosition: usize,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_HumanInterfaceDevice")]
    pub HidInputReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_HumanInterfaceDevice"))]
    HidInputReport: usize,
}
#[doc = "*Required features: `\"Devices_Input_Preview\"`*"]
#[repr(transparent)]
pub struct GazeDevicePreview(::windows::core::IUnknown);
impl GazeDevicePreview {
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanTrackEyes(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanTrackEyes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanTrackHead(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanTrackHead)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConfigurationState(&self) -> ::windows::core::Result<GazeDeviceConfigurationStatePreview> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GazeDeviceConfigurationStatePreview>();
            (::windows::core::Interface::vtable(this).ConfigurationState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestCalibrationAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).RequestCalibrationAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))]
    pub fn GetNumericControlDescriptions(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::HumanInterfaceDevice::HidNumericControlDescription>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<super::super::HumanInterfaceDevice::HidNumericControlDescription>>();
            (::windows::core::Interface::vtable(this).GetNumericControlDescriptions)(::windows::core::Interface::as_raw(this), usagepage, usageid, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))]
    pub fn GetBooleanControlDescriptions(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::HumanInterfaceDevice::HidBooleanControlDescription>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<super::super::HumanInterfaceDevice::HidBooleanControlDescription>>();
            (::windows::core::Interface::vtable(this).GetBooleanControlDescriptions)(::windows::core::Interface::as_raw(this), usagepage, usageid, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GazeDevicePreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GazeDevicePreview {}
impl ::core::fmt::Debug for GazeDevicePreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazeDevicePreview").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GazeDevicePreview {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeDevicePreview;{e79e7ee9-b389-11e7-b201-c8d3ffb75721})");
}
impl ::core::clone::Clone for GazeDevicePreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GazeDevicePreview {
    type Vtable = IGazeDevicePreview_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GazeDevicePreview {
    const IID: ::windows::core::GUID = <IGazeDevicePreview as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GazeDevicePreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDevicePreview";
}
::windows::imp::interface_hierarchy!(GazeDevicePreview, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GazeDevicePreview {}
unsafe impl ::core::marker::Sync for GazeDevicePreview {}
#[doc = "*Required features: `\"Devices_Input_Preview\"`*"]
#[repr(transparent)]
pub struct GazeDeviceWatcherAddedPreviewEventArgs(::windows::core::IUnknown);
impl GazeDeviceWatcherAddedPreviewEventArgs {
    pub fn Device(&self) -> ::windows::core::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GazeDevicePreview>();
            (::windows::core::Interface::vtable(this).Device)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GazeDeviceWatcherAddedPreviewEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GazeDeviceWatcherAddedPreviewEventArgs {}
impl ::core::fmt::Debug for GazeDeviceWatcherAddedPreviewEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazeDeviceWatcherAddedPreviewEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GazeDeviceWatcherAddedPreviewEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeDeviceWatcherAddedPreviewEventArgs;{e79e7eed-b389-11e7-b201-c8d3ffb75721})");
}
impl ::core::clone::Clone for GazeDeviceWatcherAddedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GazeDeviceWatcherAddedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherAddedPreviewEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GazeDeviceWatcherAddedPreviewEventArgs {
    const IID: ::windows::core::GUID = <IGazeDeviceWatcherAddedPreviewEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GazeDeviceWatcherAddedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherAddedPreviewEventArgs";
}
::windows::imp::interface_hierarchy!(GazeDeviceWatcherAddedPreviewEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GazeDeviceWatcherAddedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeDeviceWatcherAddedPreviewEventArgs {}
#[doc = "*Required features: `\"Devices_Input_Preview\"`*"]
#[repr(transparent)]
pub struct GazeDeviceWatcherPreview(::windows::core::IUnknown);
impl GazeDeviceWatcherPreview {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Added(&self, handler: &super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, GazeDeviceWatcherAddedPreviewEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Added)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAdded)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Removed(&self, handler: &super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, GazeDeviceWatcherRemovedPreviewEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Removed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveRemoved)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Updated(&self, handler: &super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, GazeDeviceWatcherUpdatedPreviewEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Updated)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUpdated)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted(&self, handler: &super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).EnumerationCompleted)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for GazeDeviceWatcherPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GazeDeviceWatcherPreview {}
impl ::core::fmt::Debug for GazeDeviceWatcherPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazeDeviceWatcherPreview").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GazeDeviceWatcherPreview {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeDeviceWatcherPreview;{e79e7ee7-b389-11e7-b201-c8d3ffb75721})");
}
impl ::core::clone::Clone for GazeDeviceWatcherPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GazeDeviceWatcherPreview {
    type Vtable = IGazeDeviceWatcherPreview_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GazeDeviceWatcherPreview {
    const IID: ::windows::core::GUID = <IGazeDeviceWatcherPreview as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GazeDeviceWatcherPreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherPreview";
}
::windows::imp::interface_hierarchy!(GazeDeviceWatcherPreview, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GazeDeviceWatcherPreview {}
unsafe impl ::core::marker::Sync for GazeDeviceWatcherPreview {}
#[doc = "*Required features: `\"Devices_Input_Preview\"`*"]
#[repr(transparent)]
pub struct GazeDeviceWatcherRemovedPreviewEventArgs(::windows::core::IUnknown);
impl GazeDeviceWatcherRemovedPreviewEventArgs {
    pub fn Device(&self) -> ::windows::core::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GazeDevicePreview>();
            (::windows::core::Interface::vtable(this).Device)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GazeDeviceWatcherRemovedPreviewEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GazeDeviceWatcherRemovedPreviewEventArgs {}
impl ::core::fmt::Debug for GazeDeviceWatcherRemovedPreviewEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazeDeviceWatcherRemovedPreviewEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GazeDeviceWatcherRemovedPreviewEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeDeviceWatcherRemovedPreviewEventArgs;{f2631f08-0e3f-431f-a606-50b35af94a1c})");
}
impl ::core::clone::Clone for GazeDeviceWatcherRemovedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GazeDeviceWatcherRemovedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherRemovedPreviewEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GazeDeviceWatcherRemovedPreviewEventArgs {
    const IID: ::windows::core::GUID = <IGazeDeviceWatcherRemovedPreviewEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GazeDeviceWatcherRemovedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherRemovedPreviewEventArgs";
}
::windows::imp::interface_hierarchy!(GazeDeviceWatcherRemovedPreviewEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GazeDeviceWatcherRemovedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeDeviceWatcherRemovedPreviewEventArgs {}
#[doc = "*Required features: `\"Devices_Input_Preview\"`*"]
#[repr(transparent)]
pub struct GazeDeviceWatcherUpdatedPreviewEventArgs(::windows::core::IUnknown);
impl GazeDeviceWatcherUpdatedPreviewEventArgs {
    pub fn Device(&self) -> ::windows::core::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GazeDevicePreview>();
            (::windows::core::Interface::vtable(this).Device)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GazeDeviceWatcherUpdatedPreviewEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GazeDeviceWatcherUpdatedPreviewEventArgs {}
impl ::core::fmt::Debug for GazeDeviceWatcherUpdatedPreviewEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazeDeviceWatcherUpdatedPreviewEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GazeDeviceWatcherUpdatedPreviewEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeDeviceWatcherUpdatedPreviewEventArgs;{7fe830ef-7f08-4737-88e1-4a83ae4e4885})");
}
impl ::core::clone::Clone for GazeDeviceWatcherUpdatedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GazeDeviceWatcherUpdatedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherUpdatedPreviewEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GazeDeviceWatcherUpdatedPreviewEventArgs {
    const IID: ::windows::core::GUID = <IGazeDeviceWatcherUpdatedPreviewEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GazeDeviceWatcherUpdatedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherUpdatedPreviewEventArgs";
}
::windows::imp::interface_hierarchy!(GazeDeviceWatcherUpdatedPreviewEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GazeDeviceWatcherUpdatedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeDeviceWatcherUpdatedPreviewEventArgs {}
#[doc = "*Required features: `\"Devices_Input_Preview\"`*"]
#[repr(transparent)]
pub struct GazeEnteredPreviewEventArgs(::windows::core::IUnknown);
impl GazeEnteredPreviewEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Handled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CurrentPoint(&self) -> ::windows::core::Result<GazePointPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GazePointPreview>();
            (::windows::core::Interface::vtable(this).CurrentPoint)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GazeEnteredPreviewEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GazeEnteredPreviewEventArgs {}
impl ::core::fmt::Debug for GazeEnteredPreviewEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazeEnteredPreviewEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GazeEnteredPreviewEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeEnteredPreviewEventArgs;{2567bf43-1225-489f-9dd1-daa7c50fbf4b})");
}
impl ::core::clone::Clone for GazeEnteredPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GazeEnteredPreviewEventArgs {
    type Vtable = IGazeEnteredPreviewEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GazeEnteredPreviewEventArgs {
    const IID: ::windows::core::GUID = <IGazeEnteredPreviewEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GazeEnteredPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeEnteredPreviewEventArgs";
}
::windows::imp::interface_hierarchy!(GazeEnteredPreviewEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GazeEnteredPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeEnteredPreviewEventArgs {}
#[doc = "*Required features: `\"Devices_Input_Preview\"`*"]
#[repr(transparent)]
pub struct GazeExitedPreviewEventArgs(::windows::core::IUnknown);
impl GazeExitedPreviewEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Handled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CurrentPoint(&self) -> ::windows::core::Result<GazePointPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GazePointPreview>();
            (::windows::core::Interface::vtable(this).CurrentPoint)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GazeExitedPreviewEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GazeExitedPreviewEventArgs {}
impl ::core::fmt::Debug for GazeExitedPreviewEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazeExitedPreviewEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GazeExitedPreviewEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeExitedPreviewEventArgs;{5d0af07e-7d83-40ef-9f0a-fbc1bbdcc5ac})");
}
impl ::core::clone::Clone for GazeExitedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GazeExitedPreviewEventArgs {
    type Vtable = IGazeExitedPreviewEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GazeExitedPreviewEventArgs {
    const IID: ::windows::core::GUID = <IGazeExitedPreviewEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GazeExitedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeExitedPreviewEventArgs";
}
::windows::imp::interface_hierarchy!(GazeExitedPreviewEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GazeExitedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeExitedPreviewEventArgs {}
#[doc = "*Required features: `\"Devices_Input_Preview\"`*"]
#[repr(transparent)]
pub struct GazeInputSourcePreview(::windows::core::IUnknown);
impl GazeInputSourcePreview {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GazeMoved(&self, handler: &super::super::super::Foundation::TypedEventHandler<GazeInputSourcePreview, GazeMovedPreviewEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).GazeMoved)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGazeMoved(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveGazeMoved)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GazeEntered(&self, handler: &super::super::super::Foundation::TypedEventHandler<GazeInputSourcePreview, GazeEnteredPreviewEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).GazeEntered)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGazeEntered(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveGazeEntered)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GazeExited(&self, handler: &super::super::super::Foundation::TypedEventHandler<GazeInputSourcePreview, GazeExitedPreviewEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).GazeExited)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGazeExited(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveGazeExited)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<GazeInputSourcePreview> {
        Self::IGazeInputSourcePreviewStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<GazeInputSourcePreview>();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWatcher() -> ::windows::core::Result<GazeDeviceWatcherPreview> {
        Self::IGazeInputSourcePreviewStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<GazeDeviceWatcherPreview>();
            (::windows::core::Interface::vtable(this).CreateWatcher)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGazeInputSourcePreviewStatics<R, F: FnOnce(&IGazeInputSourcePreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GazeInputSourcePreview, IGazeInputSourcePreviewStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for GazeInputSourcePreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GazeInputSourcePreview {}
impl ::core::fmt::Debug for GazeInputSourcePreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazeInputSourcePreview").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GazeInputSourcePreview {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeInputSourcePreview;{e79e7ee8-b389-11e7-b201-c8d3ffb75721})");
}
impl ::core::clone::Clone for GazeInputSourcePreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GazeInputSourcePreview {
    type Vtable = IGazeInputSourcePreview_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GazeInputSourcePreview {
    const IID: ::windows::core::GUID = <IGazeInputSourcePreview as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GazeInputSourcePreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeInputSourcePreview";
}
::windows::imp::interface_hierarchy!(GazeInputSourcePreview, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GazeInputSourcePreview {}
unsafe impl ::core::marker::Sync for GazeInputSourcePreview {}
#[doc = "*Required features: `\"Devices_Input_Preview\"`*"]
#[repr(transparent)]
pub struct GazeMovedPreviewEventArgs(::windows::core::IUnknown);
impl GazeMovedPreviewEventArgs {
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Handled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CurrentPoint(&self) -> ::windows::core::Result<GazePointPreview> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GazePointPreview>();
            (::windows::core::Interface::vtable(this).CurrentPoint)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIntermediatePoints(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<GazePointPreview>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVector<GazePointPreview>>();
            (::windows::core::Interface::vtable(this).GetIntermediatePoints)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GazeMovedPreviewEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GazeMovedPreviewEventArgs {}
impl ::core::fmt::Debug for GazeMovedPreviewEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazeMovedPreviewEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GazeMovedPreviewEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeMovedPreviewEventArgs;{e79e7eeb-b389-11e7-b201-c8d3ffb75721})");
}
impl ::core::clone::Clone for GazeMovedPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GazeMovedPreviewEventArgs {
    type Vtable = IGazeMovedPreviewEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GazeMovedPreviewEventArgs {
    const IID: ::windows::core::GUID = <IGazeMovedPreviewEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GazeMovedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeMovedPreviewEventArgs";
}
::windows::imp::interface_hierarchy!(GazeMovedPreviewEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GazeMovedPreviewEventArgs {}
unsafe impl ::core::marker::Sync for GazeMovedPreviewEventArgs {}
#[doc = "*Required features: `\"Devices_Input_Preview\"`*"]
#[repr(transparent)]
pub struct GazePointPreview(::windows::core::IUnknown);
impl GazePointPreview {
    pub fn SourceDevice(&self) -> ::windows::core::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GazeDevicePreview>();
            (::windows::core::Interface::vtable(this).SourceDevice)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EyeGazePosition(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IReference<super::super::super::Foundation::Point>>();
            (::windows::core::Interface::vtable(this).EyeGazePosition)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HeadGazePosition(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IReference<super::super::super::Foundation::Point>>();
            (::windows::core::Interface::vtable(this).HeadGazePosition)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Timestamp(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u64>();
            (::windows::core::Interface::vtable(this).Timestamp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
    #[cfg(feature = "Devices_HumanInterfaceDevice")]
    pub fn HidInputReport(&self) -> ::windows::core::Result<super::super::HumanInterfaceDevice::HidInputReport> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::HumanInterfaceDevice::HidInputReport>();
            (::windows::core::Interface::vtable(this).HidInputReport)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GazePointPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GazePointPreview {}
impl ::core::fmt::Debug for GazePointPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazePointPreview").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GazePointPreview {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazePointPreview;{e79e7eea-b389-11e7-b201-c8d3ffb75721})");
}
impl ::core::clone::Clone for GazePointPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GazePointPreview {
    type Vtable = IGazePointPreview_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GazePointPreview {
    const IID: ::windows::core::GUID = <IGazePointPreview as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GazePointPreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazePointPreview";
}
::windows::imp::interface_hierarchy!(GazePointPreview, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GazePointPreview {}
unsafe impl ::core::marker::Sync for GazePointPreview {}
#[doc = "*Required features: `\"Devices_Input_Preview\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GazeDeviceConfigurationStatePreview(pub i32);
impl GazeDeviceConfigurationStatePreview {
    pub const Unknown: Self = Self(0i32);
    pub const Ready: Self = Self(1i32);
    pub const Configuring: Self = Self(2i32);
    pub const ScreenSetupNeeded: Self = Self(3i32);
    pub const UserCalibrationNeeded: Self = Self(4i32);
}
impl ::core::marker::Copy for GazeDeviceConfigurationStatePreview {}
impl ::core::clone::Clone for GazeDeviceConfigurationStatePreview {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GazeDeviceConfigurationStatePreview {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for GazeDeviceConfigurationStatePreview {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for GazeDeviceConfigurationStatePreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GazeDeviceConfigurationStatePreview").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GazeDeviceConfigurationStatePreview {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Input.Preview.GazeDeviceConfigurationStatePreview;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
