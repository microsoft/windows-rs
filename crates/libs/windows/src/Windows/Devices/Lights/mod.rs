#[cfg(feature = "Devices_Lights_Effects")]
pub mod Effects;
#[doc(hidden)]
#[repr(transparent)]
pub struct ILamp(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILamp {
    type Vtable = ILamp_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x047d5b9a_ea45_4b2b_b1a2_14dff00bde7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILamp_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub BrightnessLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetBrightnessLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub IsColorSettable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")]
    pub Color: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Color: usize,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    #[cfg(feature = "Foundation")]
    pub AvailabilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AvailabilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAvailabilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAvailabilityChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArray(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILampArray {
    type Vtable = ILampArray_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ace9787_c8a0_4e95_a1e0_d58676538649);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArray_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HardwareVendorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub HardwareProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub HardwareVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub LampArrayKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LampArrayKind) -> ::windows::core::HRESULT,
    pub LampCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MinUpdateInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinUpdateInterval: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub BoundingBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    BoundingBox: usize,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub BrightnessLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetBrightnessLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SupportsVirtualKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetLampInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lampindex: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetIndicesForKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: super::super::System::VirtualKey, result_size__: *mut u32, result__: *mut *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetIndicesForKey: usize,
    pub GetIndicesForPurposes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, purposes: LampPurposes, result_size__: *mut u32, result__: *mut *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredcolor: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    #[cfg(feature = "UI")]
    pub SetColorForIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lampindex: i32, desiredcolor: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColorForIndex: usize,
    #[cfg(feature = "UI")]
    pub SetSingleColorForIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredcolor: super::super::UI::Color, lampIndexes_array_size: u32, lampindexes: *const i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetSingleColorForIndices: usize,
    #[cfg(feature = "UI")]
    pub SetColorsForIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredColors_array_size: u32, desiredcolors: *const super::super::UI::Color, lampIndexes_array_size: u32, lampindexes: *const i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColorsForIndices: usize,
    #[cfg(all(feature = "System", feature = "UI"))]
    pub SetColorsForKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredcolor: super::super::UI::Color, key: super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "System", feature = "UI")))]
    SetColorsForKey: usize,
    #[cfg(all(feature = "System", feature = "UI"))]
    pub SetColorsForKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredColors_array_size: u32, desiredcolors: *const super::super::UI::Color, keys_array_size: u32, keys: *const super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "System", feature = "UI")))]
    SetColorsForKeys: usize,
    #[cfg(feature = "UI")]
    pub SetColorsForPurposes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredcolor: super::super::UI::Color, purposes: LampPurposes) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColorsForPurposes: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SendMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: i32, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SendMessageAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub RequestMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    RequestMessageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILampArrayStatics {
    type Vtable = ILampArrayStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bb8c98d_5fc1_452d_bb1f_4ad410d398ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampAvailabilityChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILampAvailabilityChangedEventArgs {
    type Vtable = ILampAvailabilityChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f6e3ded_07a2_499d_9260_67e304532ba4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampAvailabilityChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILampInfo {
    type Vtable = ILampInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30bb521c_0acf_49da_8c10_150b9cf62713);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampInfo_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Index: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Purposes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LampPurposes) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Position: usize,
    pub RedLevelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub GreenLevelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub BlueLevelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub GainLevelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub FixedColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))]
    FixedColor: usize,
    #[cfg(feature = "UI")]
    pub GetNearestSupportedColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredcolor: super::super::UI::Color, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    GetNearestSupportedColor: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateLatency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateLatency: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILampStatics {
    type Vtable = ILampStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa822416c_8885_401e_b821_8e8b38a8e8ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
}
#[doc = "*Required features: `\"Devices_Lights\"`*"]
#[repr(transparent)]
pub struct Lamp(::windows::core::IUnknown);
impl Lamp {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn BrightnessLevel(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BrightnessLevel)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetBrightnessLevel(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBrightnessLevel)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsColorSettable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsColorSettable)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn Color(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Color)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetColor(&self, value: super::super::UI::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetColor)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AvailabilityChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<Lamp, LampAvailabilityChangedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AvailabilityChanged)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAvailabilityChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAvailabilityChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILampStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Lamp>> {
        Self::ILampStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<Lamp>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Lamp>> {
        Self::ILampStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDefaultAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<Lamp>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILampStatics<R, F: FnOnce(&ILampStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Lamp, ILampStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Lamp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Lamp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Lamp {}
impl ::core::fmt::Debug for Lamp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Lamp").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Lamp {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Lamp;{047d5b9a-ea45-4b2b-b1a2-14dff00bde7b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Lamp {
    type Vtable = ILamp_Vtbl;
    const IID: ::windows::core::GUID = <ILamp as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Lamp {
    const NAME: &'static str = "Windows.Devices.Lights.Lamp";
}
impl ::core::convert::From<Lamp> for ::windows::core::IUnknown {
    fn from(value: Lamp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Lamp> for ::windows::core::IUnknown {
    fn from(value: &Lamp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&Lamp> for &::windows::core::IUnknown {
    fn from(value: &Lamp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<Lamp> for ::windows::core::IInspectable {
    fn from(value: Lamp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Lamp> for ::windows::core::IInspectable {
    fn from(value: &Lamp) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&Lamp> for &::windows::core::IInspectable {
    fn from(value: &Lamp) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<Lamp> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: Lamp) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&Lamp> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &Lamp) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&Lamp> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &Lamp) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for Lamp {}
unsafe impl ::core::marker::Sync for Lamp {}
#[doc = "*Required features: `\"Devices_Lights\"`*"]
#[repr(transparent)]
pub struct LampArray(::windows::core::IUnknown);
impl LampArray {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn HardwareVendorId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HardwareVendorId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn HardwareProductId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HardwareProductId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn HardwareVersion(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HardwareVersion)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn LampArrayKind(&self) -> ::windows::core::Result<LampArrayKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LampArrayKind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LampArrayKind>(result__)
        }
    }
    pub fn LampCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LampCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MinUpdateInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MinUpdateInterval)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn BoundingBox(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BoundingBox)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn BrightnessLevel(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BrightnessLevel)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    pub fn SetBrightnessLevel(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBrightnessLevel)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsConnected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsConnected)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SupportsVirtualKeys(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SupportsVirtualKeys)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn GetLampInfo(&self, lampindex: i32) -> ::windows::core::Result<LampInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetLampInfo)(::windows::core::Interface::as_raw(this), lampindex, result__.as_mut_ptr()).from_abi::<LampInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetIndicesForKey(&self, key: super::super::System::VirtualKey) -> ::windows::core::Result<::windows::core::Array<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetIndicesForKey)(::windows::core::Interface::as_raw(this), key, ::windows::core::Array::<i32>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn GetIndicesForPurposes(&self, purposes: LampPurposes) -> ::windows::core::Result<::windows::core::Array<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetIndicesForPurposes)(::windows::core::Interface::as_raw(this), purposes, ::windows::core::Array::<i32>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetColor(&self, desiredcolor: super::super::UI::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetColor)(::windows::core::Interface::as_raw(this), desiredcolor).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetColorForIndex(&self, lampindex: i32, desiredcolor: super::super::UI::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetColorForIndex)(::windows::core::Interface::as_raw(this), lampindex, desiredcolor).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetSingleColorForIndices(&self, desiredcolor: super::super::UI::Color, lampindexes: &[i32]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSingleColorForIndices)(::windows::core::Interface::as_raw(this), desiredcolor, lampindexes.len() as u32, lampindexes.as_ptr()).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetColorsForIndices(&self, desiredcolors: &[super::super::UI::Color], lampindexes: &[i32]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetColorsForIndices)(::windows::core::Interface::as_raw(this), desiredcolors.len() as u32, desiredcolors.as_ptr(), lampindexes.len() as u32, lampindexes.as_ptr()).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"UI\"`*"]
    #[cfg(all(feature = "System", feature = "UI"))]
    pub fn SetColorsForKey(&self, desiredcolor: super::super::UI::Color, key: super::super::System::VirtualKey) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetColorsForKey)(::windows::core::Interface::as_raw(this), desiredcolor, key).ok() }
    }
    #[doc = "*Required features: `\"System\"`, `\"UI\"`*"]
    #[cfg(all(feature = "System", feature = "UI"))]
    pub fn SetColorsForKeys(&self, desiredcolors: &[super::super::UI::Color], keys: &[super::super::System::VirtualKey]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetColorsForKeys)(::windows::core::Interface::as_raw(this), desiredcolors.len() as u32, desiredcolors.as_ptr(), keys.len() as u32, keys.as_ptr()).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetColorsForPurposes(&self, desiredcolor: super::super::UI::Color, purposes: LampPurposes) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetColorsForPurposes)(::windows::core::Interface::as_raw(this), desiredcolor, purposes).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SendMessageAsync<'a, P0, E0>(&self, messageid: i32, message: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SendMessageAsync)(::windows::core::Interface::as_raw(this), messageid, message.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn RequestMessageAsync(&self, messageid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RequestMessageAsync)(::windows::core::Interface::as_raw(this), messageid, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILampArrayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LampArray>> {
        Self::ILampArrayStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<LampArray>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILampArrayStatics<R, F: FnOnce(&ILampArrayStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<LampArray, ILampArrayStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for LampArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LampArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArray {}
impl ::core::fmt::Debug for LampArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArray").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LampArray {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.LampArray;{7ace9787-c8a0-4e95-a1e0-d58676538649})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LampArray {
    type Vtable = ILampArray_Vtbl;
    const IID: ::windows::core::GUID = <ILampArray as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LampArray {
    const NAME: &'static str = "Windows.Devices.Lights.LampArray";
}
impl ::core::convert::From<LampArray> for ::windows::core::IUnknown {
    fn from(value: LampArray) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArray> for ::windows::core::IUnknown {
    fn from(value: &LampArray) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&LampArray> for &::windows::core::IUnknown {
    fn from(value: &LampArray) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<LampArray> for ::windows::core::IInspectable {
    fn from(value: LampArray) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampArray> for ::windows::core::IInspectable {
    fn from(value: &LampArray) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&LampArray> for &::windows::core::IInspectable {
    fn from(value: &LampArray) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for LampArray {}
unsafe impl ::core::marker::Sync for LampArray {}
#[doc = "*Required features: `\"Devices_Lights\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LampArrayKind(pub i32);
impl LampArrayKind {
    pub const Undefined: Self = Self(0i32);
    pub const Keyboard: Self = Self(1i32);
    pub const Mouse: Self = Self(2i32);
    pub const GameController: Self = Self(3i32);
    pub const Peripheral: Self = Self(4i32);
    pub const Scene: Self = Self(5i32);
    pub const Notification: Self = Self(6i32);
    pub const Chassis: Self = Self(7i32);
    pub const Wearable: Self = Self(8i32);
    pub const Furniture: Self = Self(9i32);
    pub const Art: Self = Self(10i32);
}
impl ::core::marker::Copy for LampArrayKind {}
impl ::core::clone::Clone for LampArrayKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LampArrayKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LampArrayKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for LampArrayKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LampArrayKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Lights.LampArrayKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Lights\"`*"]
#[repr(transparent)]
pub struct LampAvailabilityChangedEventArgs(::windows::core::IUnknown);
impl LampAvailabilityChangedEventArgs {
    pub fn IsAvailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsAvailable)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for LampAvailabilityChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LampAvailabilityChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampAvailabilityChangedEventArgs {}
impl ::core::fmt::Debug for LampAvailabilityChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampAvailabilityChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LampAvailabilityChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.LampAvailabilityChangedEventArgs;{4f6e3ded-07a2-499d-9260-67e304532ba4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LampAvailabilityChangedEventArgs {
    type Vtable = ILampAvailabilityChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ILampAvailabilityChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LampAvailabilityChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Lights.LampAvailabilityChangedEventArgs";
}
impl ::core::convert::From<LampAvailabilityChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: LampAvailabilityChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampAvailabilityChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LampAvailabilityChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&LampAvailabilityChangedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &LampAvailabilityChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<LampAvailabilityChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: LampAvailabilityChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampAvailabilityChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LampAvailabilityChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&LampAvailabilityChangedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &LampAvailabilityChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for LampAvailabilityChangedEventArgs {}
unsafe impl ::core::marker::Sync for LampAvailabilityChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Lights\"`*"]
#[repr(transparent)]
pub struct LampInfo(::windows::core::IUnknown);
impl LampInfo {
    pub fn Index(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Index)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn Purposes(&self) -> ::windows::core::Result<LampPurposes> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Purposes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<LampPurposes>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    pub fn RedLevelCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RedLevelCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn GreenLevelCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GreenLevelCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn BlueLevelCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BlueLevelCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    pub fn GainLevelCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GainLevelCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub fn FixedColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FixedColor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::UI::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn GetNearestSupportedColor(&self, desiredcolor: super::super::UI::Color) -> ::windows::core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetNearestSupportedColor)(::windows::core::Interface::as_raw(this), desiredcolor, result__.as_mut_ptr()).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateLatency(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UpdateLatency)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for LampInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LampInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampInfo {}
impl ::core::fmt::Debug for LampInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LampInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.LampInfo;{30bb521c-0acf-49da-8c10-150b9cf62713})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LampInfo {
    type Vtable = ILampInfo_Vtbl;
    const IID: ::windows::core::GUID = <ILampInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LampInfo {
    const NAME: &'static str = "Windows.Devices.Lights.LampInfo";
}
impl ::core::convert::From<LampInfo> for ::windows::core::IUnknown {
    fn from(value: LampInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampInfo> for ::windows::core::IUnknown {
    fn from(value: &LampInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&LampInfo> for &::windows::core::IUnknown {
    fn from(value: &LampInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<LampInfo> for ::windows::core::IInspectable {
    fn from(value: LampInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LampInfo> for ::windows::core::IInspectable {
    fn from(value: &LampInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&LampInfo> for &::windows::core::IInspectable {
    fn from(value: &LampInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for LampInfo {}
unsafe impl ::core::marker::Sync for LampInfo {}
#[doc = "*Required features: `\"Devices_Lights\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LampPurposes(pub u32);
impl LampPurposes {
    pub const Undefined: Self = Self(0u32);
    pub const Control: Self = Self(1u32);
    pub const Accent: Self = Self(2u32);
    pub const Branding: Self = Self(4u32);
    pub const Status: Self = Self(8u32);
    pub const Illumination: Self = Self(16u32);
    pub const Presentation: Self = Self(32u32);
}
impl ::core::marker::Copy for LampPurposes {}
impl ::core::clone::Clone for LampPurposes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LampPurposes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LampPurposes {
    type Abi = Self;
}
impl ::core::fmt::Debug for LampPurposes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampPurposes").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LampPurposes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LampPurposes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LampPurposes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LampPurposes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LampPurposes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for LampPurposes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Lights.LampPurposes;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
