#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Management_Core")]
pub mod Core;
#[cfg(feature = "Management_Deployment")]
pub mod Deployment;
#[cfg(feature = "Management_Policies")]
pub mod Policies;
#[cfg(feature = "Management_Update")]
pub mod Update;
#[cfg(feature = "Management_Workplace")]
pub mod Workplace;
#[doc(hidden)]
#[repr(transparent)]
pub struct IMdmAlert(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMdmAlert {
    type Vtable = IMdmAlert_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0fbc327_28c1_4b52_a548_c5807caf70b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMdmAlert_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MdmAlertDataType) -> ::windows::core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MdmAlertDataType) -> ::windows::core::HRESULT,
    pub Mark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MdmAlertMark) -> ::windows::core::HRESULT,
    pub SetMark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MdmAlertMark) -> ::windows::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Target: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMdmSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMdmSession {
    type Vtable = IMdmSession_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe89314c_8f64_4797_a9d7_9d88f86ae166);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMdmSession_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Alerts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Alerts: usize,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MdmSessionState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AttachAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AttachAsync: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StartWithAlertsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alerts: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartWithAlertsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMdmSessionManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMdmSessionManagerStatics {
    type Vtable = IMdmSessionManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf4ad959_f745_4b79_9b5c_de0bf8efe44b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMdmSessionManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SessionIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SessionIds: usize,
    pub TryCreateSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DeleteSessionById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetSessionById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Management\"`*"]
#[repr(transparent)]
pub struct MdmAlert(::windows::core::IUnknown);
impl MdmAlert {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MdmAlert, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Management\"`*"]
    pub fn Data(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Data)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Management\"`*"]
    pub fn SetData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetData)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Management\"`*"]
    pub fn Format(&self) -> ::windows::core::Result<MdmAlertDataType> {
        let this = self;
        unsafe {
            let mut result__: MdmAlertDataType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Format)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MdmAlertDataType>(result__)
        }
    }
    #[doc = "*Required features: `\"Management\"`*"]
    pub fn SetFormat(&self, value: MdmAlertDataType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFormat)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Management\"`*"]
    pub fn Mark(&self) -> ::windows::core::Result<MdmAlertMark> {
        let this = self;
        unsafe {
            let mut result__: MdmAlertMark = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Mark)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MdmAlertMark>(result__)
        }
    }
    #[doc = "*Required features: `\"Management\"`*"]
    pub fn SetMark(&self, value: MdmAlertMark) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMark)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Management\"`*"]
    pub fn Source(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Source)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Management\"`*"]
    pub fn SetSource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSource)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Management\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Management\"`*"]
    pub fn Target(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Target)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Management\"`*"]
    pub fn SetTarget<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTarget)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Management\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Management\"`*"]
    pub fn SetType<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetType)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for MdmAlert {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MdmAlert {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MdmAlert {}
impl ::core::fmt::Debug for MdmAlert {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MdmAlert").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MdmAlert {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.MdmAlert;{b0fbc327-28c1-4b52-a548-c5807caf70b6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MdmAlert {
    type Vtable = IMdmAlert_Vtbl;
    const IID: ::windows::core::GUID = <IMdmAlert as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MdmAlert {
    const NAME: &'static str = "Windows.Management.MdmAlert";
}
impl ::core::convert::From<MdmAlert> for ::windows::core::IUnknown {
    fn from(value: MdmAlert) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MdmAlert> for ::windows::core::IUnknown {
    fn from(value: &MdmAlert) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MdmAlert {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MdmAlert {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MdmAlert> for ::windows::core::IInspectable {
    fn from(value: MdmAlert) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MdmAlert> for ::windows::core::IInspectable {
    fn from(value: &MdmAlert) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MdmAlert {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MdmAlert {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Management\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MdmAlertDataType(pub i32);
impl MdmAlertDataType {
    pub const String: Self = Self(0i32);
    pub const Base64: Self = Self(1i32);
    pub const Boolean: Self = Self(2i32);
    pub const Integer: Self = Self(3i32);
}
impl ::core::marker::Copy for MdmAlertDataType {}
impl ::core::clone::Clone for MdmAlertDataType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MdmAlertDataType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MdmAlertDataType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MdmAlertDataType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MdmAlertDataType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MdmAlertDataType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Management.MdmAlertDataType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Management\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MdmAlertMark(pub i32);
impl MdmAlertMark {
    pub const None: Self = Self(0i32);
    pub const Fatal: Self = Self(1i32);
    pub const Critical: Self = Self(2i32);
    pub const Warning: Self = Self(3i32);
    pub const Informational: Self = Self(4i32);
}
impl ::core::marker::Copy for MdmAlertMark {}
impl ::core::clone::Clone for MdmAlertMark {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MdmAlertMark {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MdmAlertMark {
    type Abi = Self;
}
impl ::core::fmt::Debug for MdmAlertMark {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MdmAlertMark").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MdmAlertMark {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Management.MdmAlertMark;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Management\"`*"]
#[repr(transparent)]
pub struct MdmSession(::windows::core::IUnknown);
impl MdmSession {
    #[doc = "*Required features: `\"Management\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Alerts(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<MdmAlert>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Alerts)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<MdmAlert>>(result__)
        }
    }
    #[doc = "*Required features: `\"Management\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `\"Management\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Management\"`*"]
    pub fn State(&self) -> ::windows::core::Result<MdmSessionState> {
        let this = self;
        unsafe {
            let mut result__: MdmSessionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MdmSessionState>(result__)
        }
    }
    #[doc = "*Required features: `\"Management\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AttachAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AttachAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Management\"`*"]
    pub fn Delete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Delete)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Management\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Management\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartWithAlertsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Foundation::Collections::IIterable<MdmAlert>>>(&self, alerts: Param0) -> ::windows::core::Result<super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartWithAlertsAsync)(::core::mem::transmute_copy(this), alerts.into_param().abi(), &mut result__).from_abi::<super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for MdmSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MdmSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MdmSession {}
impl ::core::fmt::Debug for MdmSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MdmSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MdmSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.MdmSession;{fe89314c-8f64-4797-a9d7-9d88f86ae166})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MdmSession {
    type Vtable = IMdmSession_Vtbl;
    const IID: ::windows::core::GUID = <IMdmSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MdmSession {
    const NAME: &'static str = "Windows.Management.MdmSession";
}
impl ::core::convert::From<MdmSession> for ::windows::core::IUnknown {
    fn from(value: MdmSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MdmSession> for ::windows::core::IUnknown {
    fn from(value: &MdmSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MdmSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MdmSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MdmSession> for ::windows::core::IInspectable {
    fn from(value: MdmSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MdmSession> for ::windows::core::IInspectable {
    fn from(value: &MdmSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MdmSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MdmSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Management\"`*"]
pub struct MdmSessionManager {}
impl MdmSessionManager {
    #[doc = "*Required features: `\"Management\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SessionIds() -> ::windows::core::Result<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        Self::IMdmSessionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SessionIds)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `\"Management\"`*"]
    pub fn TryCreateSession() -> ::windows::core::Result<MdmSession> {
        Self::IMdmSessionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryCreateSession)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MdmSession>(result__)
        })
    }
    #[doc = "*Required features: `\"Management\"`*"]
    pub fn DeleteSessionById<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(sessionid: Param0) -> ::windows::core::Result<()> {
        Self::IMdmSessionManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).DeleteSessionById)(::core::mem::transmute_copy(this), sessionid.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Management\"`*"]
    pub fn GetSessionById<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(sessionid: Param0) -> ::windows::core::Result<MdmSession> {
        Self::IMdmSessionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSessionById)(::core::mem::transmute_copy(this), sessionid.into_param().abi(), &mut result__).from_abi::<MdmSession>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMdmSessionManagerStatics<R, F: FnOnce(&IMdmSessionManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MdmSessionManager, IMdmSessionManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for MdmSessionManager {
    const NAME: &'static str = "Windows.Management.MdmSessionManager";
}
#[doc = "*Required features: `\"Management\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MdmSessionState(pub i32);
impl MdmSessionState {
    pub const NotStarted: Self = Self(0i32);
    pub const Starting: Self = Self(1i32);
    pub const Connecting: Self = Self(2i32);
    pub const Communicating: Self = Self(3i32);
    pub const AlertStatusAvailable: Self = Self(4i32);
    pub const Retrying: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
}
impl ::core::marker::Copy for MdmSessionState {}
impl ::core::clone::Clone for MdmSessionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MdmSessionState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MdmSessionState {
    type Abi = Self;
}
impl ::core::fmt::Debug for MdmSessionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MdmSessionState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MdmSessionState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Management.MdmSessionState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
