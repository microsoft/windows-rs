#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HapticDeviceType(pub i32);
impl HapticDeviceType {
    pub const None: Self = Self(0i32);
    pub const Generic: Self = Self(1i32);
    pub const Pen: Self = Self(2i32);
    pub const Touchpad: Self = Self(3i32);
    pub const Mouse: Self = Self(4i32);
}
impl windows_core::TypeKind for HapticDeviceType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for HapticDeviceType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Haptics.HapticDeviceType;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HapticsControllerOverrideToken {
    pub Value: i64,
}
impl windows_core::TypeKind for HapticsControllerOverrideToken {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for HapticsControllerOverrideToken {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Devices.Haptics.HapticsControllerOverrideToken;i8)");
}
windows_core::imp::define_interface!(IInputHapticsManager, IInputHapticsManager_Vtbl, 0x040e91df_bb3a_507c_9e25_a2d2c685b2e5);
impl windows_core::RuntimeType for IInputHapticsManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputHapticsManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ThreadId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CurrentHapticsControllerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HapticDeviceType) -> windows_core::HRESULT,
    pub CurrentHapticsController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TrySendHapticWaveform: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, *mut bool) -> windows_core::HRESULT,
    pub TrySendHapticWaveformWithIntensity: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, f64, *mut bool) -> windows_core::HRESULT,
    pub TrySendHapticWaveformForDuration: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, f64, super::super::Foundation::TimeSpan, *mut bool) -> windows_core::HRESULT,
    pub TrySendHapticWaveformForPlayCount: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u16, f64, i32, super::super::Foundation::TimeSpan, *mut bool) -> windows_core::HRESULT,
    pub TryStopFeedback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetOverrideHapticsController: unsafe extern "system" fn(*mut core::ffi::c_void, HapticDeviceType, *mut core::ffi::c_void, *mut HapticsControllerOverrideToken) -> windows_core::HRESULT,
    pub ClearOverrideHapticsController: unsafe extern "system" fn(*mut core::ffi::c_void, HapticsControllerOverrideToken) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputHapticsManagerStatics, IInputHapticsManagerStatics_Vtbl, 0x7bb40f77_e187_5322_844e_aa58223c281a);
impl windows_core::RuntimeType for IInputHapticsManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputHapticsManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsHapticDevicePresent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetForCurrentThread: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryGetForThread: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKnownSimpleHapticsControllerWaveformsStatics, IKnownSimpleHapticsControllerWaveformsStatics_Vtbl, 0x3d577ef7_4cee_11e6_b535_001bdc06ab3b);
impl windows_core::RuntimeType for IKnownSimpleHapticsControllerWaveformsStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownSimpleHapticsControllerWaveformsStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Click: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub BuzzContinuous: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub RumbleContinuous: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Press: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Release: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKnownSimpleHapticsControllerWaveformsStatics2, IKnownSimpleHapticsControllerWaveformsStatics2_Vtbl, 0xa7d24c27_b79d_510a_bf79_ff6d49173e1d);
impl windows_core::RuntimeType for IKnownSimpleHapticsControllerWaveformsStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownSimpleHapticsControllerWaveformsStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BrushContinuous: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub ChiselMarkerContinuous: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub EraserContinuous: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GalaxyPenContinuous: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Hover: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub InkContinuous: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub MarkerContinuous: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub PencilContinuous: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Success: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISimpleHapticsController, ISimpleHapticsController_Vtbl, 0x3d577ef9_4cee_11e6_b535_001bdc06ab3b);
impl windows_core::RuntimeType for ISimpleHapticsController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleHapticsController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SupportedFeedback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsIntensitySupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPlayCountSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPlayDurationSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsReplayPauseIntervalSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub StopFeedback: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendHapticFeedback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendHapticFeedbackWithIntensity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub SendHapticFeedbackForDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f64, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub SendHapticFeedbackForPlayCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f64, i32, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISimpleHapticsControllerFeedback, ISimpleHapticsControllerFeedback_Vtbl, 0x3d577ef8_4cee_11e6_b535_001bdc06ab3b);
impl windows_core::RuntimeType for ISimpleHapticsControllerFeedback {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleHapticsControllerFeedback_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Waveform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVibrationDevice, IVibrationDevice_Vtbl, 0x40f21a3e_8844_47ff_b312_06185a3844da);
impl windows_core::RuntimeType for IVibrationDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVibrationDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SimpleHapticsController: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVibrationDeviceStatics, IVibrationDeviceStatics_Vtbl, 0x53e2eded_2290_4ac9_8eb3_1a84122eb71c);
impl windows_core::RuntimeType for IVibrationDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVibrationDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestAccessAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceSelector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FromIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindAllAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputHapticsManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputHapticsManager, windows_core::IUnknown, windows_core::IInspectable);
impl InputHapticsManager {
    pub fn ThreadId(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ThreadId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CurrentHapticsControllerDeviceType(&self) -> windows_core::Result<HapticDeviceType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentHapticsControllerDeviceType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CurrentHapticsController(&self) -> windows_core::Result<SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentHapticsController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TrySendHapticWaveform(&self, waveform: u16, waveformfallback: u16) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySendHapticWaveform)(windows_core::Interface::as_raw(this), waveform, waveformfallback, &mut result__).map(|| result__)
        }
    }
    pub fn TrySendHapticWaveformWithIntensity(&self, waveform: u16, waveformfallback: u16, intensity: f64) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySendHapticWaveformWithIntensity)(windows_core::Interface::as_raw(this), waveform, waveformfallback, intensity, &mut result__).map(|| result__)
        }
    }
    pub fn TrySendHapticWaveformForDuration(&self, waveform: u16, waveformfallback: u16, intensity: f64, playduration: super::super::Foundation::TimeSpan) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySendHapticWaveformForDuration)(windows_core::Interface::as_raw(this), waveform, waveformfallback, intensity, playduration, &mut result__).map(|| result__)
        }
    }
    pub fn TrySendHapticWaveformForPlayCount(&self, waveform: u16, waveformfallback: u16, intensity: f64, playcount: i32, replaypauseinterval: super::super::Foundation::TimeSpan) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySendHapticWaveformForPlayCount)(windows_core::Interface::as_raw(this), waveform, waveformfallback, intensity, playcount, replaypauseinterval, &mut result__).map(|| result__)
        }
    }
    pub fn TryStopFeedback(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryStopFeedback)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOverrideHapticsController<P1>(&self, devicetype: HapticDeviceType, controller: P1) -> windows_core::Result<HapticsControllerOverrideToken>
    where
        P1: windows_core::Param<SimpleHapticsController>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetOverrideHapticsController)(windows_core::Interface::as_raw(this), devicetype, controller.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn ClearOverrideHapticsController(&self, token: HapticsControllerOverrideToken) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ClearOverrideHapticsController)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn IsSupported() -> windows_core::Result<bool> {
        Self::IInputHapticsManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn IsHapticDevicePresent() -> windows_core::Result<bool> {
        Self::IInputHapticsManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHapticDevicePresent)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GetForCurrentThread() -> windows_core::Result<InputHapticsManager> {
        Self::IInputHapticsManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentThread)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn TryGetForThread(threadid: u32) -> windows_core::Result<InputHapticsManager> {
        Self::IInputHapticsManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetForThread)(windows_core::Interface::as_raw(this), threadid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IInputHapticsManagerStatics<R, F: FnOnce(&IInputHapticsManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputHapticsManager, IInputHapticsManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputHapticsManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputHapticsManager>();
}
unsafe impl windows_core::Interface for InputHapticsManager {
    type Vtable = <IInputHapticsManager as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputHapticsManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputHapticsManager {
    const NAME: &'static str = "Windows.Devices.Haptics.InputHapticsManager";
}
unsafe impl Send for InputHapticsManager {}
unsafe impl Sync for InputHapticsManager {}
pub struct KnownSimpleHapticsControllerWaveforms;
impl KnownSimpleHapticsControllerWaveforms {
    pub fn Click() -> windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Click)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn BuzzContinuous() -> windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BuzzContinuous)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn RumbleContinuous() -> windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RumbleContinuous)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Press() -> windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Press)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Release() -> windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Release)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn BrushContinuous() -> windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BrushContinuous)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn ChiselMarkerContinuous() -> windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChiselMarkerContinuous)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn EraserContinuous() -> windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EraserContinuous)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Error() -> windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Error)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn GalaxyPenContinuous() -> windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GalaxyPenContinuous)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Hover() -> windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Hover)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn InkContinuous() -> windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InkContinuous)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn MarkerContinuous() -> windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MarkerContinuous)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn PencilContinuous() -> windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PencilContinuous)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Success() -> windows_core::Result<u16> {
        Self::IKnownSimpleHapticsControllerWaveformsStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Success)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    fn IKnownSimpleHapticsControllerWaveformsStatics<R, F: FnOnce(&IKnownSimpleHapticsControllerWaveformsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<KnownSimpleHapticsControllerWaveforms, IKnownSimpleHapticsControllerWaveformsStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IKnownSimpleHapticsControllerWaveformsStatics2<R, F: FnOnce(&IKnownSimpleHapticsControllerWaveformsStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<KnownSimpleHapticsControllerWaveforms, IKnownSimpleHapticsControllerWaveformsStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for KnownSimpleHapticsControllerWaveforms {
    const NAME: &'static str = "Windows.Devices.Haptics.KnownSimpleHapticsControllerWaveforms";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleHapticsController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SimpleHapticsController, windows_core::IUnknown, windows_core::IInspectable);
impl SimpleHapticsController {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SupportedFeedback(&self) -> windows_core::Result<windows_collections::IVectorView<SimpleHapticsControllerFeedback>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedFeedback)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsIntensitySupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsIntensitySupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPlayCountSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPlayCountSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPlayDurationSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPlayDurationSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsReplayPauseIntervalSupported(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsReplayPauseIntervalSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StopFeedback(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).StopFeedback)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SendHapticFeedback<P0>(&self, feedback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SimpleHapticsControllerFeedback>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SendHapticFeedback)(windows_core::Interface::as_raw(this), feedback.param().abi()).ok() }
    }
    pub fn SendHapticFeedbackWithIntensity<P0>(&self, feedback: P0, intensity: f64) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SimpleHapticsControllerFeedback>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SendHapticFeedbackWithIntensity)(windows_core::Interface::as_raw(this), feedback.param().abi(), intensity).ok() }
    }
    pub fn SendHapticFeedbackForDuration<P0>(&self, feedback: P0, intensity: f64, playduration: super::super::Foundation::TimeSpan) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SimpleHapticsControllerFeedback>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SendHapticFeedbackForDuration)(windows_core::Interface::as_raw(this), feedback.param().abi(), intensity, playduration).ok() }
    }
    pub fn SendHapticFeedbackForPlayCount<P0>(&self, feedback: P0, intensity: f64, playcount: i32, replaypauseinterval: super::super::Foundation::TimeSpan) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SimpleHapticsControllerFeedback>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SendHapticFeedbackForPlayCount)(windows_core::Interface::as_raw(this), feedback.param().abi(), intensity, playcount, replaypauseinterval).ok() }
    }
}
impl windows_core::RuntimeType for SimpleHapticsController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISimpleHapticsController>();
}
unsafe impl windows_core::Interface for SimpleHapticsController {
    type Vtable = <ISimpleHapticsController as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISimpleHapticsController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SimpleHapticsController {
    const NAME: &'static str = "Windows.Devices.Haptics.SimpleHapticsController";
}
unsafe impl Send for SimpleHapticsController {}
unsafe impl Sync for SimpleHapticsController {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleHapticsControllerFeedback(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SimpleHapticsControllerFeedback, windows_core::IUnknown, windows_core::IInspectable);
impl SimpleHapticsControllerFeedback {
    pub fn Waveform(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Waveform)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Duration(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Duration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for SimpleHapticsControllerFeedback {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISimpleHapticsControllerFeedback>();
}
unsafe impl windows_core::Interface for SimpleHapticsControllerFeedback {
    type Vtable = <ISimpleHapticsControllerFeedback as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISimpleHapticsControllerFeedback as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SimpleHapticsControllerFeedback {
    const NAME: &'static str = "Windows.Devices.Haptics.SimpleHapticsControllerFeedback";
}
unsafe impl Send for SimpleHapticsControllerFeedback {}
unsafe impl Sync for SimpleHapticsControllerFeedback {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VibrationAccessStatus(pub i32);
impl VibrationAccessStatus {
    pub const Allowed: Self = Self(0i32);
    pub const DeniedByUser: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const DeniedByEnergySaver: Self = Self(3i32);
}
impl windows_core::TypeKind for VibrationAccessStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for VibrationAccessStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Haptics.VibrationAccessStatus;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VibrationDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(VibrationDevice, windows_core::IUnknown, windows_core::IInspectable);
impl VibrationDevice {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SimpleHapticsController(&self) -> windows_core::Result<SimpleHapticsController> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SimpleHapticsController)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestAccessAsync() -> windows_core::Result<windows_future::IAsyncOperation<VibrationAccessStatus>> {
        Self::IVibrationDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestAccessAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDeviceSelector() -> windows_core::Result<windows_core::HSTRING> {
        Self::IVibrationDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeviceSelector)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn FromIdAsync(deviceid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<VibrationDevice>> {
        Self::IVibrationDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromIdAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(deviceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetDefaultAsync() -> windows_core::Result<windows_future::IAsyncOperation<VibrationDevice>> {
        Self::IVibrationDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FindAllAsync() -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<VibrationDevice>>> {
        Self::IVibrationDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindAllAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IVibrationDeviceStatics<R, F: FnOnce(&IVibrationDeviceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<VibrationDevice, IVibrationDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for VibrationDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IVibrationDevice>();
}
unsafe impl windows_core::Interface for VibrationDevice {
    type Vtable = <IVibrationDevice as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IVibrationDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for VibrationDevice {
    const NAME: &'static str = "Windows.Devices.Haptics.VibrationDevice";
}
unsafe impl Send for VibrationDevice {}
unsafe impl Sync for VibrationDevice {}
