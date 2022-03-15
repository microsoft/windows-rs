#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Devices_Pwm_Provider")]
pub mod Provider;
#[doc(hidden)]
#[repr(transparent)]
pub struct IPwmController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPwmController {
    type Vtable = IPwmController_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc45f5c85_d2e8_42cf_9bd6_cf5ed029e6a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPwmController_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PinCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub ActualFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDesiredFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredfrequency: f64, result__: *mut f64) -> ::windows::core::HRESULT,
    pub MinFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub MaxFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub OpenPin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinnumber: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPwmControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPwmControllerStatics {
    type Vtable = IPwmControllerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4263bda1_8946_4404_bd48_81dd124af4d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPwmControllerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Devices_Pwm_Provider", feature = "Foundation_Collections"))]
    pub GetControllersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Pwm_Provider", feature = "Foundation_Collections")))]
    GetControllersAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPwmControllerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPwmControllerStatics2 {
    type Vtable = IPwmControllerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44fc5b1f_f119_4bdd_97ad_f76ef986736d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPwmControllerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPwmControllerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPwmControllerStatics3 {
    type Vtable = IPwmControllerStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2581871_0229_4344_ae3f_9b7cd0e66b94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPwmControllerStatics3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorFromFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, friendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPwmPin(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPwmPin {
    type Vtable = IPwmPin_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22972dc8_c6cf_4821_b7f9_c6454fb6af79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPwmPin_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Controller: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetActiveDutyCyclePercentage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetActiveDutyCyclePercentage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dutycyclepercentage: f64) -> ::windows::core::HRESULT,
    pub Polarity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PwmPulsePolarity) -> ::windows::core::HRESULT,
    pub SetPolarity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PwmPulsePolarity) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Pwm\"`*"]
#[repr(transparent)]
pub struct PwmController(::windows::core::IUnknown);
impl PwmController {
    #[doc = "*Required features: `\"Devices_Pwm\"`*"]
    pub fn PinCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PinCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Pwm\"`*"]
    pub fn ActualFrequency(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActualFrequency)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Pwm\"`*"]
    pub fn SetDesiredFrequency(&self, desiredfrequency: f64) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetDesiredFrequency)(::core::mem::transmute_copy(this), desiredfrequency, &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Pwm\"`*"]
    pub fn MinFrequency(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MinFrequency)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Pwm\"`*"]
    pub fn MaxFrequency(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxFrequency)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Pwm\"`*"]
    pub fn OpenPin(&self, pinnumber: i32) -> ::windows::core::Result<PwmPin> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenPin)(::core::mem::transmute_copy(this), pinnumber, &mut result__).from_abi::<PwmPin>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Pwm\"`, `\"Devices_Pwm_Provider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Pwm_Provider", feature = "Foundation_Collections"))]
    pub fn GetControllersAsync<'a, Param0: ::windows::core::IntoParam<'a, Provider::IPwmProvider>>(provider: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PwmController>>> {
        Self::IPwmControllerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetControllersAsync)(::core::mem::transmute_copy(this), provider.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PwmController>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Pwm\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PwmController>> {
        Self::IPwmControllerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefaultAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PwmController>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Pwm\"`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPwmControllerStatics3(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Pwm\"`*"]
    pub fn GetDeviceSelectorFromFriendlyName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(friendlyname: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPwmControllerStatics3(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorFromFriendlyName)(::core::mem::transmute_copy(this), friendlyname.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Pwm\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PwmController>> {
        Self::IPwmControllerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PwmController>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPwmControllerStatics<R, F: FnOnce(&IPwmControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PwmController, IPwmControllerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IPwmControllerStatics2<R, F: FnOnce(&IPwmControllerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PwmController, IPwmControllerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IPwmControllerStatics3<R, F: FnOnce(&IPwmControllerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PwmController, IPwmControllerStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PwmController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PwmController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PwmController {}
impl ::core::fmt::Debug for PwmController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PwmController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PwmController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Pwm.PwmController;{c45f5c85-d2e8-42cf-9bd6-cf5ed029e6a7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PwmController {
    type Vtable = IPwmController_Vtbl;
    const IID: ::windows::core::GUID = <IPwmController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PwmController {
    const NAME: &'static str = "Windows.Devices.Pwm.PwmController";
}
impl ::core::convert::From<PwmController> for ::windows::core::IUnknown {
    fn from(value: PwmController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PwmController> for ::windows::core::IUnknown {
    fn from(value: &PwmController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PwmController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PwmController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PwmController> for ::windows::core::IInspectable {
    fn from(value: PwmController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PwmController> for ::windows::core::IInspectable {
    fn from(value: &PwmController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PwmController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PwmController {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PwmController {}
unsafe impl ::core::marker::Sync for PwmController {}
#[doc = "*Required features: `\"Devices_Pwm\"`*"]
#[repr(transparent)]
pub struct PwmPin(::windows::core::IUnknown);
impl PwmPin {
    #[doc = "*Required features: `\"Devices_Pwm\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Pwm\"`*"]
    pub fn Controller(&self) -> ::windows::core::Result<PwmController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Controller)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PwmController>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Pwm\"`*"]
    pub fn GetActiveDutyCyclePercentage(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetActiveDutyCyclePercentage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Pwm\"`*"]
    pub fn SetActiveDutyCyclePercentage(&self, dutycyclepercentage: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetActiveDutyCyclePercentage)(::core::mem::transmute_copy(this), dutycyclepercentage).ok() }
    }
    #[doc = "*Required features: `\"Devices_Pwm\"`*"]
    pub fn Polarity(&self) -> ::windows::core::Result<PwmPulsePolarity> {
        let this = self;
        unsafe {
            let mut result__: PwmPulsePolarity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Polarity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PwmPulsePolarity>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Pwm\"`*"]
    pub fn SetPolarity(&self, value: PwmPulsePolarity) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPolarity)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_Pwm\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Pwm\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Pwm\"`*"]
    pub fn IsStarted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStarted)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for PwmPin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PwmPin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PwmPin {}
impl ::core::fmt::Debug for PwmPin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PwmPin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PwmPin {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Pwm.PwmPin;{22972dc8-c6cf-4821-b7f9-c6454fb6af79})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PwmPin {
    type Vtable = IPwmPin_Vtbl;
    const IID: ::windows::core::GUID = <IPwmPin as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PwmPin {
    const NAME: &'static str = "Windows.Devices.Pwm.PwmPin";
}
impl ::core::convert::From<PwmPin> for ::windows::core::IUnknown {
    fn from(value: PwmPin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PwmPin> for ::windows::core::IUnknown {
    fn from(value: &PwmPin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PwmPin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PwmPin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PwmPin> for ::windows::core::IInspectable {
    fn from(value: PwmPin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PwmPin> for ::windows::core::IInspectable {
    fn from(value: &PwmPin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PwmPin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PwmPin {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<PwmPin> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: PwmPin) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&PwmPin> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &PwmPin) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for PwmPin {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &PwmPin {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PwmPin {}
unsafe impl ::core::marker::Sync for PwmPin {}
#[doc = "*Required features: `\"Devices_Pwm\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PwmPulsePolarity(pub i32);
impl PwmPulsePolarity {
    pub const ActiveHigh: Self = Self(0i32);
    pub const ActiveLow: Self = Self(1i32);
}
impl ::core::marker::Copy for PwmPulsePolarity {}
impl ::core::clone::Clone for PwmPulsePolarity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PwmPulsePolarity {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PwmPulsePolarity {
    type Abi = Self;
}
impl ::core::fmt::Debug for PwmPulsePolarity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PwmPulsePolarity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PwmPulsePolarity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Pwm.PwmPulsePolarity;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
