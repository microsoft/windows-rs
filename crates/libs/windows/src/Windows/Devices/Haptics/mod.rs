::windows_core::imp::com_interface!(IKnownSimpleHapticsControllerWaveformsStatics, IKnownSimpleHapticsControllerWaveformsStatics_Vtbl, 0x3d577ef7_4cee_11e6_b535_001bdc06ab3b);
#[repr(C)]
pub struct IKnownSimpleHapticsControllerWaveformsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Click: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u16) -> ::windows_core::HRESULT,
    pub BuzzContinuous: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u16) -> ::windows_core::HRESULT,
    pub RumbleContinuous: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u16) -> ::windows_core::HRESULT,
    pub Press: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u16) -> ::windows_core::HRESULT,
    pub Release: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u16) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IKnownSimpleHapticsControllerWaveformsStatics2, IKnownSimpleHapticsControllerWaveformsStatics2_Vtbl, 0xa7d24c27_b79d_510a_bf79_ff6d49173e1d);
#[repr(C)]
pub struct IKnownSimpleHapticsControllerWaveformsStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub BrushContinuous: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u16) -> ::windows_core::HRESULT,
    pub ChiselMarkerContinuous: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u16) -> ::windows_core::HRESULT,
    pub EraserContinuous: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u16) -> ::windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u16) -> ::windows_core::HRESULT,
    pub GalaxyPenContinuous: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u16) -> ::windows_core::HRESULT,
    pub Hover: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u16) -> ::windows_core::HRESULT,
    pub InkContinuous: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u16) -> ::windows_core::HRESULT,
    pub MarkerContinuous: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u16) -> ::windows_core::HRESULT,
    pub PencilContinuous: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u16) -> ::windows_core::HRESULT,
    pub Success: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u16) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISimpleHapticsController, ISimpleHapticsController_Vtbl, 0x3d577ef9_4cee_11e6_b535_001bdc06ab3b);
#[repr(C)]
pub struct ISimpleHapticsController_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedFeedback: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedFeedback: usize,
    pub IsIntensitySupported: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub IsPlayCountSupported: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub IsPlayDurationSupported: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub IsReplayPauseIntervalSupported: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut bool) -> ::windows_core::HRESULT,
    pub StopFeedback: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SendHapticFeedback: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SendHapticFeedbackWithIntensity: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, f64) -> ::windows_core::HRESULT,
    pub SendHapticFeedbackForDuration: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, f64, super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    pub SendHapticFeedbackForPlayCount: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, f64, i32, super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISimpleHapticsControllerFeedback, ISimpleHapticsControllerFeedback_Vtbl, 0x3d577ef8_4cee_11e6_b535_001bdc06ab3b);
#[repr(C)]
pub struct ISimpleHapticsControllerFeedback_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Waveform: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut u16) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IVibrationDevice, IVibrationDevice_Vtbl, 0x40f21a3e_8844_47ff_b312_06185a3844da);
#[repr(C)]
pub struct IVibrationDevice_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SimpleHapticsController: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IVibrationDeviceStatics, IVibrationDeviceStatics_Vtbl, 0x53e2eded_2290_4ac9_8eb3_1a84122eb71c);
#[repr(C)]
pub struct IVibrationDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RequestAccessAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDefaultAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
}
pub struct KnownSimpleHapticsControllerWaveforms;
impl KnownSimpleHapticsControllerWaveforms {
    pub fn Click() -> ::windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Click)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn BuzzContinuous() -> ::windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BuzzContinuous)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RumbleContinuous() -> ::windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RumbleContinuous)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Press() -> ::windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Press)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Release() -> ::windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Release)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn BrushContinuous() -> ::windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BrushContinuous)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ChiselMarkerContinuous() -> ::windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChiselMarkerContinuous)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn EraserContinuous() -> ::windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EraserContinuous)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Error() -> ::windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Error)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GalaxyPenContinuous() -> ::windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GalaxyPenContinuous)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Hover() -> ::windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Hover)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn InkContinuous() -> ::windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InkContinuous)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MarkerContinuous() -> ::windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MarkerContinuous)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PencilContinuous() -> ::windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PencilContinuous)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Success() -> ::windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Success)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    #[doc(hidden)]
    pub fn IKnownSimpleHapticsControllerWaveformsStatics<R, F: FnOnce(&IKnownSimpleHapticsControllerWaveformsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<KnownSimpleHapticsControllerWaveforms, IKnownSimpleHapticsControllerWaveformsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IKnownSimpleHapticsControllerWaveformsStatics2<R, F: FnOnce(&IKnownSimpleHapticsControllerWaveformsStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<KnownSimpleHapticsControllerWaveforms, IKnownSimpleHapticsControllerWaveformsStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for KnownSimpleHapticsControllerWaveforms {
    const NAME: &'static str = "Windows.Devices.Haptics.KnownSimpleHapticsControllerWaveforms";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SimpleHapticsController(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(SimpleHapticsController, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl SimpleHapticsController {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedFeedback(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<SimpleHapticsControllerFeedback>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedFeedback)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsIntensitySupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsIntensitySupported)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPlayCountSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPlayCountSupported)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPlayDurationSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsPlayDurationSupported)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsReplayPauseIntervalSupported(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsReplayPauseIntervalSupported)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StopFeedback(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).StopFeedback)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SendHapticFeedback<P0>(&self, feedback: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<SimpleHapticsControllerFeedback>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SendHapticFeedback)(::windows_core::Interface::as_raw(this), feedback.into_param().abi()).ok() }
    }
    pub fn SendHapticFeedbackWithIntensity<P0>(&self, feedback: P0, intensity: f64) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<SimpleHapticsControllerFeedback>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SendHapticFeedbackWithIntensity)(::windows_core::Interface::as_raw(this), feedback.into_param().abi(), intensity).ok() }
    }
    pub fn SendHapticFeedbackForDuration<P0>(&self, feedback: P0, intensity: f64, playduration: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<SimpleHapticsControllerFeedback>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SendHapticFeedbackForDuration)(::windows_core::Interface::as_raw(this), feedback.into_param().abi(), intensity, playduration).ok() }
    }
    pub fn SendHapticFeedbackForPlayCount<P0>(&self, feedback: P0, intensity: f64, playcount: i32, replaypauseinterval: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<SimpleHapticsControllerFeedback>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SendHapticFeedbackForPlayCount)(::windows_core::Interface::as_raw(this), feedback.into_param().abi(), intensity, playcount, replaypauseinterval).ok() }
    }
}
impl ::windows_core::RuntimeType for SimpleHapticsController {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for SimpleHapticsController {
    type Vtable = ISimpleHapticsController_Vtbl;
    const IID: ::windows_core::GUID = <ISimpleHapticsController as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SimpleHapticsController {
    const NAME: &'static str = "Windows.Devices.Haptics.SimpleHapticsController";
}
unsafe impl ::core::marker::Send for SimpleHapticsController {}
unsafe impl ::core::marker::Sync for SimpleHapticsController {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SimpleHapticsControllerFeedback(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(SimpleHapticsControllerFeedback, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl SimpleHapticsControllerFeedback {
    pub fn Waveform(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Waveform)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl ::windows_core::RuntimeType for SimpleHapticsControllerFeedback {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for SimpleHapticsControllerFeedback {
    type Vtable = ISimpleHapticsControllerFeedback_Vtbl;
    const IID: ::windows_core::GUID = <ISimpleHapticsControllerFeedback as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for SimpleHapticsControllerFeedback {
    const NAME: &'static str = "Windows.Devices.Haptics.SimpleHapticsControllerFeedback";
}
unsafe impl ::core::marker::Send for SimpleHapticsControllerFeedback {}
unsafe impl ::core::marker::Sync for SimpleHapticsControllerFeedback {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct VibrationDevice(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(VibrationDevice, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl VibrationDevice {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SimpleHapticsController(&self) -> ::windows_core::Result<SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SimpleHapticsController)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestAccessAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<VibrationAccessStatus>> {
        Self::IVibrationDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IVibrationDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeviceSelector)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<VibrationDevice>> {
        Self::IVibrationDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<VibrationDevice>> {
        Self::IVibrationDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDefaultAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<VibrationDevice>>> {
        Self::IVibrationDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[doc(hidden)]
    pub fn IVibrationDeviceStatics<R, F: FnOnce(&IVibrationDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VibrationDevice, IVibrationDeviceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for VibrationDevice {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for VibrationDevice {
    type Vtable = IVibrationDevice_Vtbl;
    const IID: ::windows_core::GUID = <IVibrationDevice as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for VibrationDevice {
    const NAME: &'static str = "Windows.Devices.Haptics.VibrationDevice";
}
unsafe impl ::core::marker::Send for VibrationDevice {}
unsafe impl ::core::marker::Sync for VibrationDevice {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct VibrationAccessStatus(pub i32);
impl VibrationAccessStatus {
    pub const Allowed: Self = Self(0i32);
    pub const DeniedByUser: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const DeniedByEnergySaver: Self = Self(3i32);
}
impl ::windows_core::TypeKind for VibrationAccessStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VibrationAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VibrationAccessStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VibrationAccessStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Haptics.VibrationAccessStatus;i4)");
}
