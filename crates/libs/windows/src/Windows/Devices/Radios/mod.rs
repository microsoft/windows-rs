#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadio(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadio {
    type Vtable = IRadioVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x252118df_b33e_416a_875f_1cf38ae2d83e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadioVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: RadioState, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RadioState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RadioKind) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRadioStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRadioStatics {
    type Vtable = IRadioStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5fb6a12e_67cb_46ae_aae9_65919f86eff4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadioStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: 'Devices_Radios'*"]
#[repr(transparent)]
pub struct Radio(::windows::core::IUnknown);
impl Radio {
    #[doc = "*Required features: 'Devices_Radios', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStateAsync(&self, value: RadioState) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RadioAccessStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<RadioAccessStatus>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Radios', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<Radio, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Radios', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Radios'*"]
    pub fn State(&self) -> ::windows::core::Result<RadioState> {
        let this = self;
        unsafe {
            let mut result__: RadioState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RadioState>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Radios'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Radios'*"]
    pub fn Kind(&self) -> ::windows::core::Result<RadioKind> {
        let this = self;
        unsafe {
            let mut result__: RadioKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RadioKind>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Radios', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetRadiosAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Radio>>> {
        Self::IRadioStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Radio>>>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Radios'*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IRadioStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Radios', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Radio>> {
        Self::IRadioStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Radio>>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Radios', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RadioAccessStatus>> {
        Self::IRadioStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<RadioAccessStatus>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRadioStatics<R, F: FnOnce(&IRadioStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Radio, IRadioStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Radio {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Radio {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Radio {}
impl ::core::fmt::Debug for Radio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Radio").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Radio {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Radios.Radio;{252118df-b33e-416a-875f-1cf38ae2d83e})");
}
unsafe impl ::windows::core::Interface for Radio {
    type Vtable = IRadioVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x252118df_b33e_416a_875f_1cf38ae2d83e);
}
impl ::windows::core::RuntimeName for Radio {
    const NAME: &'static str = "Windows.Devices.Radios.Radio";
}
impl ::core::convert::From<Radio> for ::windows::core::IUnknown {
    fn from(value: Radio) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Radio> for ::windows::core::IUnknown {
    fn from(value: &Radio) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Radio {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &Radio {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Radio> for ::windows::core::IInspectable {
    fn from(value: Radio) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Radio> for ::windows::core::IInspectable {
    fn from(value: &Radio) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Radio {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &Radio {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for Radio {}
unsafe impl ::core::marker::Sync for Radio {}
#[doc = "*Required features: 'Devices_Radios'*"]
#[repr(transparent)]
pub struct RadioAccessStatus(pub i32);
impl RadioAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for RadioAccessStatus {}
impl ::core::clone::Clone for RadioAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RadioAccessStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RadioAccessStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadioAccessStatus {}
impl ::core::fmt::Debug for RadioAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadioAccessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RadioAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Radios.RadioAccessStatus;i4)");
}
impl ::windows::core::DefaultType for RadioAccessStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Devices_Radios'*"]
#[repr(transparent)]
pub struct RadioKind(pub i32);
impl RadioKind {
    pub const Other: Self = Self(0i32);
    pub const WiFi: Self = Self(1i32);
    pub const MobileBroadband: Self = Self(2i32);
    pub const Bluetooth: Self = Self(3i32);
    pub const FM: Self = Self(4i32);
}
impl ::core::marker::Copy for RadioKind {}
impl ::core::clone::Clone for RadioKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RadioKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RadioKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadioKind {}
impl ::core::fmt::Debug for RadioKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadioKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RadioKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Radios.RadioKind;i4)");
}
impl ::windows::core::DefaultType for RadioKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Devices_Radios'*"]
#[repr(transparent)]
pub struct RadioState(pub i32);
impl RadioState {
    pub const Unknown: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Off: Self = Self(2i32);
    pub const Disabled: Self = Self(3i32);
}
impl ::core::marker::Copy for RadioState {}
impl ::core::clone::Clone for RadioState {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RadioState {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RadioState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RadioState {}
impl ::core::fmt::Debug for RadioState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RadioState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RadioState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Radios.RadioState;i4)");
}
impl ::windows::core::DefaultType for RadioState {
    type DefaultType = Self;
}
