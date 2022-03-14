#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Devices_Pwm_Provider\"`*"]
#[repr(transparent)]
pub struct IPwmControllerProvider(::windows::core::IUnknown);
impl IPwmControllerProvider {
    #[doc = "*Required features: `\"Devices_Pwm_Provider\"`*"]
    pub fn PinCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PinCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Pwm_Provider\"`*"]
    pub fn ActualFrequency(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActualFrequency)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Pwm_Provider\"`*"]
    pub fn SetDesiredFrequency(&self, frequency: f64) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetDesiredFrequency)(::core::mem::transmute_copy(this), frequency, &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Pwm_Provider\"`*"]
    pub fn MaxFrequency(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxFrequency)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Pwm_Provider\"`*"]
    pub fn MinFrequency(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MinFrequency)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Pwm_Provider\"`*"]
    pub fn AcquirePin(&self, pin: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AcquirePin)(::core::mem::transmute_copy(this), pin).ok() }
    }
    #[doc = "*Required features: `\"Devices_Pwm_Provider\"`*"]
    pub fn ReleasePin(&self, pin: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReleasePin)(::core::mem::transmute_copy(this), pin).ok() }
    }
    #[doc = "*Required features: `\"Devices_Pwm_Provider\"`*"]
    pub fn EnablePin(&self, pin: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).EnablePin)(::core::mem::transmute_copy(this), pin).ok() }
    }
    #[doc = "*Required features: `\"Devices_Pwm_Provider\"`*"]
    pub fn DisablePin(&self, pin: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DisablePin)(::core::mem::transmute_copy(this), pin).ok() }
    }
    #[doc = "*Required features: `\"Devices_Pwm_Provider\"`*"]
    pub fn SetPulseParameters(&self, pin: i32, dutycycle: f64, invertpolarity: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPulseParameters)(::core::mem::transmute_copy(this), pin, dutycycle, invertpolarity).ok() }
    }
}
impl ::core::convert::From<IPwmControllerProvider> for ::windows::core::IUnknown {
    fn from(value: IPwmControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPwmControllerProvider> for ::windows::core::IUnknown {
    fn from(value: &IPwmControllerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPwmControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPwmControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPwmControllerProvider> for ::windows::core::IInspectable {
    fn from(value: IPwmControllerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPwmControllerProvider> for ::windows::core::IInspectable {
    fn from(value: &IPwmControllerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPwmControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPwmControllerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPwmControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IPwmControllerProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1300593b-e2e3-40a4-b7d9-48dff0377a52}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IPwmControllerProvider {
    type Vtable = IPwmControllerProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1300593b_e2e3_40a4_b7d9_48dff0377a52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPwmControllerProvider_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
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
    #[doc = "*Required features: `\"Devices_Pwm_Provider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetControllers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IPwmControllerProvider>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetControllers)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<IPwmControllerProvider>>(result__)
        }
    }
}
impl ::core::convert::From<IPwmProvider> for ::windows::core::IUnknown {
    fn from(value: IPwmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPwmProvider> for ::windows::core::IUnknown {
    fn from(value: &IPwmProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPwmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPwmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPwmProvider> for ::windows::core::IInspectable {
    fn from(value: IPwmProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPwmProvider> for ::windows::core::IInspectable {
    fn from(value: &IPwmProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPwmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPwmProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPwmProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IPwmProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a3301228-52f1-47b0-9349-66ba43d25902}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IPwmProvider {
    type Vtable = IPwmProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3301228_52f1_47b0_9349_66ba43d25902);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPwmProvider_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetControllers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetControllers: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
