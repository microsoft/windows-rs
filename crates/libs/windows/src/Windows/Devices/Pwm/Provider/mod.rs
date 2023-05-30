#[doc = "*Required features: `\"Devices_Pwm_Provider\"`*"]
#[repr(transparent)]
pub struct IPwmControllerProvider(::windows_core::IUnknown);
impl IPwmControllerProvider {
    pub fn PinCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PinCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActualFrequency(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActualFrequency)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDesiredFrequency(&self, frequency: f64) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetDesiredFrequency)(::windows_core::Interface::as_raw(this), frequency, &mut result__).from_abi(result__)
        }
    }
    pub fn MaxFrequency(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxFrequency)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinFrequency(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinFrequency)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AcquirePin(&self, pin: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AcquirePin)(::windows_core::Interface::as_raw(this), pin).ok() }
    }
    pub fn ReleasePin(&self, pin: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReleasePin)(::windows_core::Interface::as_raw(this), pin).ok() }
    }
    pub fn EnablePin(&self, pin: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).EnablePin)(::windows_core::Interface::as_raw(this), pin).ok() }
    }
    pub fn DisablePin(&self, pin: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).DisablePin)(::windows_core::Interface::as_raw(this), pin).ok() }
    }
    pub fn SetPulseParameters(&self, pin: i32, dutycycle: f64, invertpolarity: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPulseParameters)(::windows_core::Interface::as_raw(this), pin, dutycycle, invertpolarity).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IPwmControllerProvider, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IPwmControllerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPwmControllerProvider {}
impl ::core::fmt::Debug for IPwmControllerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPwmControllerProvider").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IPwmControllerProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{1300593b-e2e3-40a4-b7d9-48dff0377a52}");
}
unsafe impl ::windows_core::Interface for IPwmControllerProvider {
    type Vtable = IPwmControllerProvider_Vtbl;
}
impl ::core::clone::Clone for IPwmControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPwmControllerProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1300593b_e2e3_40a4_b7d9_48dff0377a52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPwmControllerProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PinCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ActualFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetDesiredFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frequency: f64, result__: *mut f64) -> ::windows_core::HRESULT,
    pub MaxFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub MinFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub AcquirePin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows_core::HRESULT,
    pub ReleasePin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows_core::HRESULT,
    pub EnablePin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows_core::HRESULT,
    pub DisablePin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows_core::HRESULT,
    pub SetPulseParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: i32, dutycycle: f64, invertpolarity: bool) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Pwm_Provider\"`*"]
#[repr(transparent)]
pub struct IPwmProvider(::windows_core::IUnknown);
impl IPwmProvider {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetControllers(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<IPwmControllerProvider>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetControllers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IPwmProvider, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IPwmProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPwmProvider {}
impl ::core::fmt::Debug for IPwmProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPwmProvider").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IPwmProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{a3301228-52f1-47b0-9349-66ba43d25902}");
}
unsafe impl ::windows_core::Interface for IPwmProvider {
    type Vtable = IPwmProvider_Vtbl;
}
impl ::core::clone::Clone for IPwmProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPwmProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa3301228_52f1_47b0_9349_66ba43d25902);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPwmProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetControllers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetControllers: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
