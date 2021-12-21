#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownSimpleHapticsControllerWaveformsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKnownSimpleHapticsControllerWaveformsStatics {
    type Vtable = IKnownSimpleHapticsControllerWaveformsStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577ef7_4cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownSimpleHapticsControllerWaveformsStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IKnownSimpleHapticsControllerWaveformsStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKnownSimpleHapticsControllerWaveformsStatics2 {
    type Vtable = IKnownSimpleHapticsControllerWaveformsStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7d24c27_b79d_510a_bf79_ff6d49173e1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownSimpleHapticsControllerWaveformsStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISimpleHapticsController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISimpleHapticsController {
    type Vtable = ISimpleHapticsControllerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577ef9_4cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleHapticsControllerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedback: ::windows::core::RawPtr, intensity: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedback: ::windows::core::RawPtr, intensity: f64, playduration: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, feedback: ::windows::core::RawPtr, intensity: f64, playcount: i32, replaypauseinterval: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISimpleHapticsControllerFeedback(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISimpleHapticsControllerFeedback {
    type Vtable = ISimpleHapticsControllerFeedbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577ef8_4cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleHapticsControllerFeedbackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVibrationDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVibrationDevice {
    type Vtable = IVibrationDeviceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40f21a3e_8844_47ff_b312_06185a3844da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVibrationDeviceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IVibrationDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVibrationDeviceStatics {
    type Vtable = IVibrationDeviceStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53e2eded_2290_4ac9_8eb3_1a84122eb71c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVibrationDeviceStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc = "*Required features: 'Devices_Haptics'*"]
pub struct KnownSimpleHapticsControllerWaveforms {}
impl KnownSimpleHapticsControllerWaveforms {
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn Click() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn BuzzContinuous() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn RumbleContinuous() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn Press() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn Release() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn BrushContinuous() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn ChiselMarkerContinuous() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn EraserContinuous() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn Error() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn GalaxyPenContinuous() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn Hover() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn InkContinuous() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn MarkerContinuous() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn PencilContinuous() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn Success() -> ::windows::core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownSimpleHapticsControllerWaveformsStatics<R, F: FnOnce(&IKnownSimpleHapticsControllerWaveformsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownSimpleHapticsControllerWaveforms, IKnownSimpleHapticsControllerWaveformsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IKnownSimpleHapticsControllerWaveformsStatics2<R, F: FnOnce(&IKnownSimpleHapticsControllerWaveformsStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KnownSimpleHapticsControllerWaveforms, IKnownSimpleHapticsControllerWaveformsStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for KnownSimpleHapticsControllerWaveforms {
    const NAME: &'static str = "Windows.Devices.Haptics.KnownSimpleHapticsControllerWaveforms";
}
#[doc = "*Required features: 'Devices_Haptics'*"]
#[repr(transparent)]
pub struct SimpleHapticsController(::windows::core::IUnknown);
impl SimpleHapticsController {
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Haptics', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedFeedback(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SimpleHapticsControllerFeedback>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SimpleHapticsControllerFeedback>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn IsIntensitySupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn IsPlayCountSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn IsPlayDurationSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn IsReplayPauseIntervalSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn StopFeedback(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn SendHapticFeedback<'a, Param0: ::windows::core::IntoParam<'a, SimpleHapticsControllerFeedback>>(&self, feedback: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), feedback.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn SendHapticFeedbackWithIntensity<'a, Param0: ::windows::core::IntoParam<'a, SimpleHapticsControllerFeedback>>(&self, feedback: Param0, intensity: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), feedback.into_param().abi(), intensity).ok() }
    }
    #[doc = "*Required features: 'Devices_Haptics', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SendHapticFeedbackForDuration<'a, Param0: ::windows::core::IntoParam<'a, SimpleHapticsControllerFeedback>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, feedback: Param0, intensity: f64, playduration: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), feedback.into_param().abi(), intensity, playduration.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Haptics', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SendHapticFeedbackForPlayCount<'a, Param0: ::windows::core::IntoParam<'a, SimpleHapticsControllerFeedback>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, feedback: Param0, intensity: f64, playcount: i32, replaypauseinterval: Param3) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), feedback.into_param().abi(), intensity, playcount, replaypauseinterval.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for SimpleHapticsController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SimpleHapticsController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SimpleHapticsController {}
impl ::core::fmt::Debug for SimpleHapticsController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SimpleHapticsController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SimpleHapticsController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Haptics.SimpleHapticsController;{3d577ef9-4cee-11e6-b535-001bdc06ab3b})");
}
unsafe impl ::windows::core::Interface for SimpleHapticsController {
    type Vtable = ISimpleHapticsControllerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577ef9_4cee_11e6_b535_001bdc06ab3b);
}
impl ::windows::core::RuntimeName for SimpleHapticsController {
    const NAME: &'static str = "Windows.Devices.Haptics.SimpleHapticsController";
}
impl ::core::convert::From<SimpleHapticsController> for ::windows::core::IUnknown {
    fn from(value: SimpleHapticsController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SimpleHapticsController> for ::windows::core::IUnknown {
    fn from(value: &SimpleHapticsController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SimpleHapticsController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SimpleHapticsController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SimpleHapticsController> for ::windows::core::IInspectable {
    fn from(value: SimpleHapticsController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SimpleHapticsController> for ::windows::core::IInspectable {
    fn from(value: &SimpleHapticsController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SimpleHapticsController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SimpleHapticsController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SimpleHapticsController {}
unsafe impl ::core::marker::Sync for SimpleHapticsController {}
#[doc = "*Required features: 'Devices_Haptics'*"]
#[repr(transparent)]
pub struct SimpleHapticsControllerFeedback(::windows::core::IUnknown);
impl SimpleHapticsControllerFeedback {
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn Waveform(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Haptics', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for SimpleHapticsControllerFeedback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SimpleHapticsControllerFeedback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SimpleHapticsControllerFeedback {}
impl ::core::fmt::Debug for SimpleHapticsControllerFeedback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SimpleHapticsControllerFeedback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SimpleHapticsControllerFeedback {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Haptics.SimpleHapticsControllerFeedback;{3d577ef8-4cee-11e6-b535-001bdc06ab3b})");
}
unsafe impl ::windows::core::Interface for SimpleHapticsControllerFeedback {
    type Vtable = ISimpleHapticsControllerFeedbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577ef8_4cee_11e6_b535_001bdc06ab3b);
}
impl ::windows::core::RuntimeName for SimpleHapticsControllerFeedback {
    const NAME: &'static str = "Windows.Devices.Haptics.SimpleHapticsControllerFeedback";
}
impl ::core::convert::From<SimpleHapticsControllerFeedback> for ::windows::core::IUnknown {
    fn from(value: SimpleHapticsControllerFeedback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SimpleHapticsControllerFeedback> for ::windows::core::IUnknown {
    fn from(value: &SimpleHapticsControllerFeedback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SimpleHapticsControllerFeedback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SimpleHapticsControllerFeedback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SimpleHapticsControllerFeedback> for ::windows::core::IInspectable {
    fn from(value: SimpleHapticsControllerFeedback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SimpleHapticsControllerFeedback> for ::windows::core::IInspectable {
    fn from(value: &SimpleHapticsControllerFeedback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SimpleHapticsControllerFeedback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SimpleHapticsControllerFeedback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SimpleHapticsControllerFeedback {}
unsafe impl ::core::marker::Sync for SimpleHapticsControllerFeedback {}
#[doc = "*Required features: 'Devices_Haptics'*"]
#[repr(transparent)]
pub struct VibrationAccessStatus(pub i32);
impl VibrationAccessStatus {
    pub const Allowed: Self = Self(0i32);
    pub const DeniedByUser: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const DeniedByEnergySaver: Self = Self(3i32);
}
impl ::core::marker::Copy for VibrationAccessStatus {}
impl ::core::clone::Clone for VibrationAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for VibrationAccessStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VibrationAccessStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VibrationAccessStatus {}
impl ::core::fmt::Debug for VibrationAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VibrationAccessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VibrationAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Haptics.VibrationAccessStatus;i4)");
}
impl ::windows::core::DefaultType for VibrationAccessStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Devices_Haptics'*"]
#[repr(transparent)]
pub struct VibrationDevice(::windows::core::IUnknown);
impl VibrationDevice {
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn SimpleHapticsController(&self) -> ::windows::core::Result<SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SimpleHapticsController>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Haptics', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VibrationAccessStatus>> {
        Self::IVibrationDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VibrationAccessStatus>>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Haptics'*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IVibrationDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Haptics', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VibrationDevice>> {
        Self::IVibrationDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VibrationDevice>>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Haptics', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VibrationDevice>> {
        Self::IVibrationDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VibrationDevice>>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Haptics', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindAllAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<VibrationDevice>>> {
        Self::IVibrationDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<VibrationDevice>>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVibrationDeviceStatics<R, F: FnOnce(&IVibrationDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VibrationDevice, IVibrationDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for VibrationDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VibrationDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VibrationDevice {}
impl ::core::fmt::Debug for VibrationDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VibrationDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VibrationDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Haptics.VibrationDevice;{40f21a3e-8844-47ff-b312-06185a3844da})");
}
unsafe impl ::windows::core::Interface for VibrationDevice {
    type Vtable = IVibrationDeviceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40f21a3e_8844_47ff_b312_06185a3844da);
}
impl ::windows::core::RuntimeName for VibrationDevice {
    const NAME: &'static str = "Windows.Devices.Haptics.VibrationDevice";
}
impl ::core::convert::From<VibrationDevice> for ::windows::core::IUnknown {
    fn from(value: VibrationDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VibrationDevice> for ::windows::core::IUnknown {
    fn from(value: &VibrationDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VibrationDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &VibrationDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<VibrationDevice> for ::windows::core::IInspectable {
    fn from(value: VibrationDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VibrationDevice> for ::windows::core::IInspectable {
    fn from(value: &VibrationDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VibrationDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &VibrationDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for VibrationDevice {}
unsafe impl ::core::marker::Sync for VibrationDevice {}
