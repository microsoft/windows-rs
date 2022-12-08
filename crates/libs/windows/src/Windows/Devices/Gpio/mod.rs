#[cfg(feature = "Devices_Gpio_Provider")]
pub mod Provider;
#[doc(hidden)]
#[repr(transparent)]
pub struct IGpioChangeCounter(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGpioChangeCounter {
    type Vtable = IGpioChangeCounter_Vtbl;
}
unsafe impl ::windows::core::Interface for IGpioChangeCounter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb5ec0de_6801_43ff_803d_4576628a8b26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioChangeCounter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetPolarity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GpioChangePolarity) -> ::windows::core::HRESULT,
    pub Polarity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GpioChangePolarity) -> ::windows::core::HRESULT,
    pub IsStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GpioChangeCount) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Read: usize,
    #[cfg(feature = "Foundation")]
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GpioChangeCount) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Reset: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGpioChangeCounterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGpioChangeCounterFactory {
    type Vtable = IGpioChangeCounterFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IGpioChangeCounterFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x147d94b6_0a9e_410c_b4fa_f89f4052084d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioChangeCounterFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGpioChangeReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGpioChangeReader {
    type Vtable = IGpioChangeReader_Vtbl;
}
unsafe impl ::windows::core::Interface for IGpioChangeReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0abc885f_e031_48e8_8590_70de78363c6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioChangeReader_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Capacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub IsEmpty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsOverflowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetPolarity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GpioChangePolarity) -> ::windows::core::HRESULT,
    pub Polarity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GpioChangePolarity) -> ::windows::core::HRESULT,
    pub IsStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetNextItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GpioChangeRecord) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetNextItem: usize,
    #[cfg(feature = "Foundation")]
    pub PeekNextItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GpioChangeRecord) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PeekNextItem: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllItems: usize,
    #[cfg(feature = "Foundation")]
    pub WaitForItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WaitForItemsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGpioChangeReaderFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGpioChangeReaderFactory {
    type Vtable = IGpioChangeReaderFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IGpioChangeReaderFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9598ef3_390e_441a_9d1c_e8de0b2df0df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioChangeReaderFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithCapacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: *mut ::core::ffi::c_void, mincapacity: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGpioController(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGpioController {
    type Vtable = IGpioController_Vtbl;
}
unsafe impl ::windows::core::Interface for IGpioController {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x284012e3_7461_469c_a8bc_61d69d08a53c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioController_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PinCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub OpenPin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinnumber: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OpenPinWithSharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinnumber: i32, sharingmode: GpioSharingMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TryOpenPin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinnumber: i32, sharingmode: GpioSharingMode, pin: *mut *mut ::core::ffi::c_void, openstatus: *mut GpioOpenStatus, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGpioControllerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGpioControllerStatics {
    type Vtable = IGpioControllerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IGpioControllerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ed6f42e_7af7_4116_9533_c43d99a1fb64);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioControllerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGpioControllerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGpioControllerStatics2 {
    type Vtable = IGpioControllerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IGpioControllerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x912b7d20_6ca4_4106_a373_fffd346b0e5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioControllerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Gpio_Provider", feature = "Foundation_Collections"))]
    pub GetControllersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Gpio_Provider", feature = "Foundation_Collections")))]
    GetControllersAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGpioPin(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGpioPin {
    type Vtable = IGpioPin_Vtbl;
}
unsafe impl ::windows::core::Interface for IGpioPin {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11d9b087_afae_4790_9ee9_e0eac942d201);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioPin_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValueChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveValueChanged: usize,
    #[cfg(feature = "Foundation")]
    pub DebounceTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DebounceTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetDebounceTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDebounceTimeout: usize,
    pub PinNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GpioSharingMode) -> ::windows::core::HRESULT,
    pub IsDriveModeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, drivemode: GpioPinDriveMode, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetDriveMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GpioPinDriveMode) -> ::windows::core::HRESULT,
    pub SetDriveMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GpioPinDriveMode) -> ::windows::core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: GpioPinValue) -> ::windows::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GpioPinValue) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGpioPinValueChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGpioPinValueChangedEventArgs {
    type Vtable = IGpioPinValueChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IGpioPinValueChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3137aae1_703d_4059_bd24_b5b25dffb84e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioPinValueChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Edge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GpioPinEdge) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
pub struct GpioChangeCounter(::windows::core::IUnknown);
impl GpioChangeCounter {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn SetPolarity(&self, value: GpioChangePolarity) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPolarity)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Polarity(&self) -> ::windows::core::Result<GpioChangePolarity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Polarity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsStarted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStarted)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Start)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Stop)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Read(&self) -> ::windows::core::Result<GpioChangeCount> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Read)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Reset(&self) -> ::windows::core::Result<GpioChangeCount> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Reset)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Create(pin: &GpioPin) -> ::windows::core::Result<GpioChangeCounter> {
        Self::IGpioChangeCounterFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(pin), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGpioChangeCounterFactory<R, F: FnOnce(&IGpioChangeCounterFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GpioChangeCounter, IGpioChangeCounterFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for GpioChangeCounter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GpioChangeCounter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GpioChangeCounter {}
impl ::core::fmt::Debug for GpioChangeCounter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioChangeCounter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GpioChangeCounter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Gpio.GpioChangeCounter;{cb5ec0de-6801-43ff-803d-4576628a8b26})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GpioChangeCounter {
    type Vtable = IGpioChangeCounter_Vtbl;
}
unsafe impl ::windows::core::Interface for GpioChangeCounter {
    const IID: ::windows::core::GUID = <IGpioChangeCounter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GpioChangeCounter {
    const NAME: &'static str = "Windows.Devices.Gpio.GpioChangeCounter";
}
::windows::core::interface_hierarchy!(GpioChangeCounter, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<GpioChangeCounter> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: GpioChangeCounter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&GpioChangeCounter> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &GpioChangeCounter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&GpioChangeCounter> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &GpioChangeCounter) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for GpioChangeCounter {}
unsafe impl ::core::marker::Sync for GpioChangeCounter {}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
pub struct GpioChangeReader(::windows::core::IUnknown);
impl GpioChangeReader {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Capacity(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Capacity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Length(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Length)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsEmpty(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsEmpty)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsOverflowed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsOverflowed)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPolarity(&self, value: GpioChangePolarity) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPolarity)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Polarity(&self) -> ::windows::core::Result<GpioChangePolarity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Polarity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsStarted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStarted)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Start)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Stop)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetNextItem(&self) -> ::windows::core::Result<GpioChangeRecord> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNextItem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PeekNextItem(&self) -> ::windows::core::Result<GpioChangeRecord> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PeekNextItem)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<GpioChangeRecord>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAllItems)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WaitForItemsAsync(&self, count: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WaitForItemsAsync)(::windows::core::Vtable::as_raw(this), count, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Create(pin: &GpioPin) -> ::windows::core::Result<GpioChangeReader> {
        Self::IGpioChangeReaderFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(pin), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateWithCapacity(pin: &GpioPin, mincapacity: i32) -> ::windows::core::Result<GpioChangeReader> {
        Self::IGpioChangeReaderFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithCapacity)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(pin), mincapacity, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGpioChangeReaderFactory<R, F: FnOnce(&IGpioChangeReaderFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GpioChangeReader, IGpioChangeReaderFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for GpioChangeReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GpioChangeReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GpioChangeReader {}
impl ::core::fmt::Debug for GpioChangeReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioChangeReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GpioChangeReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Gpio.GpioChangeReader;{0abc885f-e031-48e8-8590-70de78363c6d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GpioChangeReader {
    type Vtable = IGpioChangeReader_Vtbl;
}
unsafe impl ::windows::core::Interface for GpioChangeReader {
    const IID: ::windows::core::GUID = <IGpioChangeReader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GpioChangeReader {
    const NAME: &'static str = "Windows.Devices.Gpio.GpioChangeReader";
}
::windows::core::interface_hierarchy!(GpioChangeReader, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<GpioChangeReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: GpioChangeReader) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&GpioChangeReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &GpioChangeReader) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&GpioChangeReader> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &GpioChangeReader) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for GpioChangeReader {}
unsafe impl ::core::marker::Sync for GpioChangeReader {}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
pub struct GpioController(::windows::core::IUnknown);
impl GpioController {
    pub fn PinCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PinCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OpenPin(&self, pinnumber: i32) -> ::windows::core::Result<GpioPin> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OpenPin)(::windows::core::Vtable::as_raw(this), pinnumber, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OpenPinWithSharingMode(&self, pinnumber: i32, sharingmode: GpioSharingMode) -> ::windows::core::Result<GpioPin> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OpenPinWithSharingMode)(::windows::core::Vtable::as_raw(this), pinnumber, sharingmode, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TryOpenPin(&self, pinnumber: i32, sharingmode: GpioSharingMode, pin: &mut ::core::option::Option<GpioPin>, openstatus: &mut GpioOpenStatus) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryOpenPin)(::windows::core::Vtable::as_raw(this), pinnumber, sharingmode, pin as *mut _ as _, openstatus, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<GpioController> {
        Self::IGpioControllerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefault)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Gpio_Provider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Gpio_Provider", feature = "Foundation_Collections"))]
    pub fn GetControllersAsync<P0, E0>(provider: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<GpioController>>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<Provider::IGpioProvider>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IGpioControllerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetControllersAsync)(::windows::core::Vtable::as_raw(this), provider.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GpioController>> {
        Self::IGpioControllerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDefaultAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGpioControllerStatics<R, F: FnOnce(&IGpioControllerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GpioController, IGpioControllerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGpioControllerStatics2<R, F: FnOnce(&IGpioControllerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GpioController, IGpioControllerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for GpioController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GpioController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GpioController {}
impl ::core::fmt::Debug for GpioController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GpioController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Gpio.GpioController;{284012e3-7461-469c-a8bc-61d69d08a53c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GpioController {
    type Vtable = IGpioController_Vtbl;
}
unsafe impl ::windows::core::Interface for GpioController {
    const IID: ::windows::core::GUID = <IGpioController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GpioController {
    const NAME: &'static str = "Windows.Devices.Gpio.GpioController";
}
::windows::core::interface_hierarchy!(GpioController, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GpioController {}
unsafe impl ::core::marker::Sync for GpioController {}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
pub struct GpioPin(::windows::core::IUnknown);
impl GpioPin {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ValueChanged(&self, handler: &super::super::Foundation::TypedEventHandler<GpioPin, GpioPinValueChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ValueChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveValueChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveValueChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DebounceTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DebounceTimeout)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDebounceTimeout(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDebounceTimeout)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PinNumber(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PinNumber)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SharingMode(&self) -> ::windows::core::Result<GpioSharingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SharingMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsDriveModeSupported(&self, drivemode: GpioPinDriveMode) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsDriveModeSupported)(::windows::core::Vtable::as_raw(this), drivemode, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDriveMode(&self) -> ::windows::core::Result<GpioPinDriveMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDriveMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDriveMode(&self, value: GpioPinDriveMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDriveMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Write(&self, value: GpioPinValue) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Write)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Read(&self) -> ::windows::core::Result<GpioPinValue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Read)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for GpioPin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GpioPin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GpioPin {}
impl ::core::fmt::Debug for GpioPin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioPin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GpioPin {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Gpio.GpioPin;{11d9b087-afae-4790-9ee9-e0eac942d201})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GpioPin {
    type Vtable = IGpioPin_Vtbl;
}
unsafe impl ::windows::core::Interface for GpioPin {
    const IID: ::windows::core::GUID = <IGpioPin as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GpioPin {
    const NAME: &'static str = "Windows.Devices.Gpio.GpioPin";
}
::windows::core::interface_hierarchy!(GpioPin, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<GpioPin> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: GpioPin) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&GpioPin> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &GpioPin) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&GpioPin> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &GpioPin) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for GpioPin {}
unsafe impl ::core::marker::Sync for GpioPin {}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
pub struct GpioPinValueChangedEventArgs(::windows::core::IUnknown);
impl GpioPinValueChangedEventArgs {
    pub fn Edge(&self) -> ::windows::core::Result<GpioPinEdge> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Edge)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for GpioPinValueChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GpioPinValueChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GpioPinValueChangedEventArgs {}
impl ::core::fmt::Debug for GpioPinValueChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioPinValueChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GpioPinValueChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Gpio.GpioPinValueChangedEventArgs;{3137aae1-703d-4059-bd24-b5b25dffb84e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for GpioPinValueChangedEventArgs {
    type Vtable = IGpioPinValueChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for GpioPinValueChangedEventArgs {
    const IID: ::windows::core::GUID = <IGpioPinValueChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GpioPinValueChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Gpio.GpioPinValueChangedEventArgs";
}
::windows::core::interface_hierarchy!(GpioPinValueChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GpioPinValueChangedEventArgs {}
unsafe impl ::core::marker::Sync for GpioPinValueChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GpioChangePolarity(pub i32);
impl GpioChangePolarity {
    pub const Falling: Self = Self(0i32);
    pub const Rising: Self = Self(1i32);
    pub const Both: Self = Self(2i32);
}
impl ::core::marker::Copy for GpioChangePolarity {}
impl ::core::clone::Clone for GpioChangePolarity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GpioChangePolarity {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GpioChangePolarity {
    type Abi = Self;
}
impl ::core::fmt::Debug for GpioChangePolarity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioChangePolarity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GpioChangePolarity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioChangePolarity;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GpioOpenStatus(pub i32);
impl GpioOpenStatus {
    pub const PinOpened: Self = Self(0i32);
    pub const PinUnavailable: Self = Self(1i32);
    pub const SharingViolation: Self = Self(2i32);
    pub const MuxingConflict: Self = Self(3i32);
    pub const UnknownError: Self = Self(4i32);
}
impl ::core::marker::Copy for GpioOpenStatus {}
impl ::core::clone::Clone for GpioOpenStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GpioOpenStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GpioOpenStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for GpioOpenStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioOpenStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GpioOpenStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioOpenStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GpioPinDriveMode(pub i32);
impl GpioPinDriveMode {
    pub const Input: Self = Self(0i32);
    pub const Output: Self = Self(1i32);
    pub const InputPullUp: Self = Self(2i32);
    pub const InputPullDown: Self = Self(3i32);
    pub const OutputOpenDrain: Self = Self(4i32);
    pub const OutputOpenDrainPullUp: Self = Self(5i32);
    pub const OutputOpenSource: Self = Self(6i32);
    pub const OutputOpenSourcePullDown: Self = Self(7i32);
}
impl ::core::marker::Copy for GpioPinDriveMode {}
impl ::core::clone::Clone for GpioPinDriveMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GpioPinDriveMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GpioPinDriveMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for GpioPinDriveMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioPinDriveMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GpioPinDriveMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioPinDriveMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GpioPinEdge(pub i32);
impl GpioPinEdge {
    pub const FallingEdge: Self = Self(0i32);
    pub const RisingEdge: Self = Self(1i32);
}
impl ::core::marker::Copy for GpioPinEdge {}
impl ::core::clone::Clone for GpioPinEdge {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GpioPinEdge {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GpioPinEdge {
    type Abi = Self;
}
impl ::core::fmt::Debug for GpioPinEdge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioPinEdge").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GpioPinEdge {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioPinEdge;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GpioPinValue(pub i32);
impl GpioPinValue {
    pub const Low: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for GpioPinValue {}
impl ::core::clone::Clone for GpioPinValue {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GpioPinValue {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GpioPinValue {
    type Abi = Self;
}
impl ::core::fmt::Debug for GpioPinValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioPinValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GpioPinValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioPinValue;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_Gpio\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GpioSharingMode(pub i32);
impl GpioSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const SharedReadOnly: Self = Self(1i32);
}
impl ::core::marker::Copy for GpioSharingMode {}
impl ::core::clone::Clone for GpioSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GpioSharingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GpioSharingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for GpioSharingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioSharingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GpioSharingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.GpioSharingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Devices_Gpio\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct GpioChangeCount {
    pub Count: u64,
    pub RelativeTime: super::super::Foundation::TimeSpan,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for GpioChangeCount {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for GpioChangeCount {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for GpioChangeCount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GpioChangeCount").field("Count", &self.Count).field("RelativeTime", &self.RelativeTime).finish()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Abi for GpioChangeCount {
    type Abi = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for GpioChangeCount {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Devices.Gpio.GpioChangeCount;u8;struct(Windows.Foundation.TimeSpan;i8))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for GpioChangeCount {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.RelativeTime == other.RelativeTime
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for GpioChangeCount {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for GpioChangeCount {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Devices_Gpio\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct GpioChangeRecord {
    pub RelativeTime: super::super::Foundation::TimeSpan,
    pub Edge: GpioPinEdge,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for GpioChangeRecord {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for GpioChangeRecord {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for GpioChangeRecord {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GpioChangeRecord").field("RelativeTime", &self.RelativeTime).field("Edge", &self.Edge).finish()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Abi for GpioChangeRecord {
    type Abi = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for GpioChangeRecord {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Devices.Gpio.GpioChangeRecord;struct(Windows.Foundation.TimeSpan;i8);enum(Windows.Devices.Gpio.GpioPinEdge;i4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for GpioChangeRecord {
    fn eq(&self, other: &Self) -> bool {
        self.RelativeTime == other.RelativeTime && self.Edge == other.Edge
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for GpioChangeRecord {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for GpioChangeRecord {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
