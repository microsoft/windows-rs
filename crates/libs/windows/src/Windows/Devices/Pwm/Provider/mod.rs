#[doc = "*Required features: `\"Devices_Pwm_Provider\"`*"]
#[repr(transparent)]
pub struct IPwmControllerProvider(::windows::core::IUnknown);
impl IPwmControllerProvider {
    pub fn PinCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).PinCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActualFrequency(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).ActualFrequency)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDesiredFrequency(&self, frequency: f64) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).SetDesiredFrequency)(::windows::core::Interface::as_raw(this), frequency, &mut result__).from_abi(result__)
        }
    }
    pub fn MaxFrequency(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).MaxFrequency)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinFrequency(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).MinFrequency)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AcquirePin(&self, pin: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AcquirePin)(::windows::core::Interface::as_raw(this), pin).ok() }
    }
    pub fn ReleasePin(&self, pin: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReleasePin)(::windows::core::Interface::as_raw(this), pin).ok() }
    }
    pub fn EnablePin(&self, pin: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).EnablePin)(::windows::core::Interface::as_raw(this), pin).ok() }
    }
    pub fn DisablePin(&self, pin: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DisablePin)(::windows::core::Interface::as_raw(this), pin).ok() }
    }
    pub fn SetPulseParameters(&self, pin: i32, dutycycle: f64, invertpolarity: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPulseParameters)(::windows::core::Interface::as_raw(this), pin, dutycycle, invertpolarity).ok() }
    }
}
::windows::imp::interface_hierarchy!(IPwmControllerProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::windows::core::RuntimeType for IPwmControllerProvider {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{1300593b-e2e3-40a4-b7d9-48dff0377a52}");
}
unsafe impl ::windows::core::Interface for IPwmControllerProvider {
    type Vtable = IPwmControllerProvider_Vtbl;
}
impl ::core::clone::Clone for IPwmControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPwmControllerProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1300593b_e2e3_40a4_b7d9_48dff0377a52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPwmControllerProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PinCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub ActualFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDesiredFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frequency: f64, result__: *mut f64) -> ::windows::core::HRESULT,
    pub MaxFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub MinFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub AcquirePin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows::core::HRESULT,
    pub ReleasePin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows::core::HRESULT,
    pub EnablePin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows::core::HRESULT,
    pub DisablePin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: i32) -> ::windows::core::HRESULT,
    pub SetPulseParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: i32, dutycycle: f64, invertpolarity: bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Pwm_Provider\"`*"]
#[repr(transparent)]
pub struct IPwmProvider(::windows::core::IUnknown);
impl IPwmProvider {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetControllers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IPwmControllerProvider>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<IPwmControllerProvider>>();
            (::windows::core::Interface::vtable(this).GetControllers)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IPwmProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::windows::core::RuntimeType for IPwmProvider {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{a3301228-52f1-47b0-9349-66ba43d25902}");
}
unsafe impl ::windows::core::Interface for IPwmProvider {
    type Vtable = IPwmProvider_Vtbl;
}
impl ::core::clone::Clone for IPwmProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPwmProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3301228_52f1_47b0_9349_66ba43d25902);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPwmProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetControllers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetControllers: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
