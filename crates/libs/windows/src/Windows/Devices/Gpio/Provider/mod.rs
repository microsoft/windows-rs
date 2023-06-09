#[doc = "*Required features: `\"Devices_Gpio_Provider\"`*"]
#[repr(transparent)]
pub struct IGpioControllerProvider(::windows_core::IUnknown);
impl IGpioControllerProvider {
    pub fn PinCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PinCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OpenPinProvider(&self, pin: i32, sharingmode: ProviderGpioSharingMode) -> ::windows_core::Result<IGpioPinProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenPinProvider)(::windows_core::Interface::as_raw(this), pin, sharingmode, &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IGpioControllerProvider, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IGpioControllerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGpioControllerProvider {}
impl ::core::fmt::Debug for IGpioControllerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGpioControllerProvider").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IGpioControllerProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{ad11cec7-19ea-4b21-874f-b91aed4a25db}");
}
unsafe impl ::windows_core::Interface for IGpioControllerProvider {
    type Vtable = IGpioControllerProvider_Vtbl;
}
impl ::core::clone::Clone for IGpioControllerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGpioControllerProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xad11cec7_19ea_4b21_874f_b91aed4a25db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioControllerProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PinCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub OpenPinProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pin: i32, sharingmode: ProviderGpioSharingMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Gpio_Provider\"`*"]
#[repr(transparent)]
pub struct IGpioPinProvider(::windows_core::IUnknown);
impl IGpioPinProvider {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ValueChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<IGpioPinProvider, GpioPinProviderValueChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ValueChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveValueChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveValueChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DebounceTimeout(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DebounceTimeout)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDebounceTimeout(&self, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDebounceTimeout)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PinNumber(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PinNumber)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SharingMode(&self) -> ::windows_core::Result<ProviderGpioSharingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SharingMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsDriveModeSupported(&self, drivemode: ProviderGpioPinDriveMode) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDriveModeSupported)(::windows_core::Interface::as_raw(this), drivemode, &mut result__).from_abi(result__)
        }
    }
    pub fn GetDriveMode(&self) -> ::windows_core::Result<ProviderGpioPinDriveMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDriveMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDriveMode(&self, value: ProviderGpioPinDriveMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDriveMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Write(&self, value: ProviderGpioPinValue) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Write)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Read(&self) -> ::windows_core::Result<ProviderGpioPinValue> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Read)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IGpioPinProvider, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IGpioPinProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGpioPinProvider {}
impl ::core::fmt::Debug for IGpioPinProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGpioPinProvider").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IGpioPinProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{42344cb7-6abc-40ff-9ce7-73b85301b900}");
}
unsafe impl ::windows_core::Interface for IGpioPinProvider {
    type Vtable = IGpioPinProvider_Vtbl;
}
impl ::core::clone::Clone for IGpioPinProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGpioPinProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x42344cb7_6abc_40ff_9ce7_73b85301b900);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioPinProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValueChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveValueChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveValueChanged: usize,
    #[cfg(feature = "Foundation")]
    pub DebounceTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DebounceTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub SetDebounceTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDebounceTimeout: usize,
    pub PinNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SharingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProviderGpioSharingMode) -> ::windows_core::HRESULT,
    pub IsDriveModeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, drivemode: ProviderGpioPinDriveMode, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetDriveMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProviderGpioPinDriveMode) -> ::windows_core::HRESULT,
    pub SetDriveMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ProviderGpioPinDriveMode) -> ::windows_core::HRESULT,
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ProviderGpioPinValue) -> ::windows_core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProviderGpioPinValue) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGpioPinProviderValueChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGpioPinProviderValueChangedEventArgs {
    type Vtable = IGpioPinProviderValueChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IGpioPinProviderValueChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGpioPinProviderValueChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x32a6d6f2_3d5b_44cd_8fbe_13a69f2edb24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioPinProviderValueChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Edge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProviderGpioPinEdge) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGpioPinProviderValueChangedEventArgsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGpioPinProviderValueChangedEventArgsFactory {
    type Vtable = IGpioPinProviderValueChangedEventArgsFactory_Vtbl;
}
impl ::core::clone::Clone for IGpioPinProviderValueChangedEventArgsFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGpioPinProviderValueChangedEventArgsFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ecb0b59_568c_4392_b24a_8a59a902b1f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioPinProviderValueChangedEventArgsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, edge: ProviderGpioPinEdge, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Gpio_Provider\"`*"]
#[repr(transparent)]
pub struct IGpioProvider(::windows_core::IUnknown);
impl IGpioProvider {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetControllers(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<IGpioControllerProvider>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetControllers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IGpioProvider, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IGpioProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGpioProvider {}
impl ::core::fmt::Debug for IGpioProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGpioProvider").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IGpioProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{44e82707-08ca-434a-afe0-d61580446f7e}");
}
unsafe impl ::windows_core::Interface for IGpioProvider {
    type Vtable = IGpioProvider_Vtbl;
}
impl ::core::clone::Clone for IGpioProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGpioProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x44e82707_08ca_434a_afe0_d61580446f7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGpioProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetControllers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetControllers: usize,
}
#[doc = "*Required features: `\"Devices_Gpio_Provider\"`*"]
#[repr(transparent)]
pub struct GpioPinProviderValueChangedEventArgs(::windows_core::IUnknown);
impl GpioPinProviderValueChangedEventArgs {
    pub fn Edge(&self) -> ::windows_core::Result<ProviderGpioPinEdge> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Edge)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(edge: ProviderGpioPinEdge) -> ::windows_core::Result<GpioPinProviderValueChangedEventArgs> {
        Self::IGpioPinProviderValueChangedEventArgsFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), edge, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGpioPinProviderValueChangedEventArgsFactory<R, F: FnOnce(&IGpioPinProviderValueChangedEventArgsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GpioPinProviderValueChangedEventArgs, IGpioPinProviderValueChangedEventArgsFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for GpioPinProviderValueChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GpioPinProviderValueChangedEventArgs {}
impl ::core::fmt::Debug for GpioPinProviderValueChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GpioPinProviderValueChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GpioPinProviderValueChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Gpio.Provider.GpioPinProviderValueChangedEventArgs;{32a6d6f2-3d5b-44cd-8fbe-13a69f2edb24})");
}
impl ::core::clone::Clone for GpioPinProviderValueChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GpioPinProviderValueChangedEventArgs {
    type Vtable = IGpioPinProviderValueChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GpioPinProviderValueChangedEventArgs {
    const IID: ::windows_core::GUID = <IGpioPinProviderValueChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GpioPinProviderValueChangedEventArgs {
    const NAME: &'static str = "Windows.Devices.Gpio.Provider.GpioPinProviderValueChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(GpioPinProviderValueChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GpioPinProviderValueChangedEventArgs {}
unsafe impl ::core::marker::Sync for GpioPinProviderValueChangedEventArgs {}
#[doc = "*Required features: `\"Devices_Gpio_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ProviderGpioPinDriveMode(pub i32);
impl ProviderGpioPinDriveMode {
    pub const Input: Self = Self(0i32);
    pub const Output: Self = Self(1i32);
    pub const InputPullUp: Self = Self(2i32);
    pub const InputPullDown: Self = Self(3i32);
    pub const OutputOpenDrain: Self = Self(4i32);
    pub const OutputOpenDrainPullUp: Self = Self(5i32);
    pub const OutputOpenSource: Self = Self(6i32);
    pub const OutputOpenSourcePullDown: Self = Self(7i32);
}
impl ::core::marker::Copy for ProviderGpioPinDriveMode {}
impl ::core::clone::Clone for ProviderGpioPinDriveMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProviderGpioPinDriveMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ProviderGpioPinDriveMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ProviderGpioPinDriveMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderGpioPinDriveMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ProviderGpioPinDriveMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.Provider.ProviderGpioPinDriveMode;i4)");
}
#[doc = "*Required features: `\"Devices_Gpio_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ProviderGpioPinEdge(pub i32);
impl ProviderGpioPinEdge {
    pub const FallingEdge: Self = Self(0i32);
    pub const RisingEdge: Self = Self(1i32);
}
impl ::core::marker::Copy for ProviderGpioPinEdge {}
impl ::core::clone::Clone for ProviderGpioPinEdge {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProviderGpioPinEdge {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ProviderGpioPinEdge {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ProviderGpioPinEdge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderGpioPinEdge").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ProviderGpioPinEdge {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.Provider.ProviderGpioPinEdge;i4)");
}
#[doc = "*Required features: `\"Devices_Gpio_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ProviderGpioPinValue(pub i32);
impl ProviderGpioPinValue {
    pub const Low: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for ProviderGpioPinValue {}
impl ::core::clone::Clone for ProviderGpioPinValue {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProviderGpioPinValue {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ProviderGpioPinValue {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ProviderGpioPinValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderGpioPinValue").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ProviderGpioPinValue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.Provider.ProviderGpioPinValue;i4)");
}
#[doc = "*Required features: `\"Devices_Gpio_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ProviderGpioSharingMode(pub i32);
impl ProviderGpioSharingMode {
    pub const Exclusive: Self = Self(0i32);
    pub const SharedReadOnly: Self = Self(1i32);
}
impl ::core::marker::Copy for ProviderGpioSharingMode {}
impl ::core::clone::Clone for ProviderGpioSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProviderGpioSharingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ProviderGpioSharingMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ProviderGpioSharingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProviderGpioSharingMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ProviderGpioSharingMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Gpio.Provider.ProviderGpioSharingMode;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
