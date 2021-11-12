#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppCapability(pub ::windows::core::IInspectable);
impl AppCapability {
    pub fn CapabilityName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<AppCapabilityAccessStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<AppCapabilityAccessStatus>>(result__)
        }
    }
    pub fn CheckAccess(&self) -> ::windows::core::Result<AppCapabilityAccessStatus> {
        let this = self;
        unsafe {
            let mut result__: AppCapabilityAccessStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCapabilityAccessStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn AccessChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AppCapability, AppCapabilityAccessChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAccessChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RequestAccessForCapabilitiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(capabilitynames: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, AppCapabilityAccessStatus>>> {
        Self::IAppCapabilityStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), capabilitynames.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, AppCapabilityAccessStatus>>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System"))]
    pub fn RequestAccessForCapabilitiesForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(user: Param0, capabilitynames: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, AppCapabilityAccessStatus>>> {
        Self::IAppCapabilityStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), user.into_param().abi(), capabilitynames.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, AppCapabilityAccessStatus>>>(result__)
        })
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(capabilityname: Param0) -> ::windows::core::Result<AppCapability> {
        Self::IAppCapabilityStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), capabilityname.into_param().abi(), &mut result__).from_abi::<AppCapability>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn CreateWithProcessIdForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, capabilityname: Param1, pid: u32) -> ::windows::core::Result<AppCapability> {
        Self::IAppCapabilityStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), user.into_param().abi(), capabilityname.into_param().abi(), pid, &mut result__).from_abi::<AppCapability>(result__)
        })
    }
    pub fn IAppCapabilityStatics<R, F: FnOnce(&IAppCapabilityStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppCapability, IAppCapabilityStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for AppCapability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authorization.AppCapabilityAccess.AppCapability;{4c49d915-8a2a-4295-9437-2df7c396aff4})");
}
unsafe impl ::windows::core::Interface for AppCapability {
    type Vtable = IAppCapability_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c49d915_8a2a_4295_9437_2df7c396aff4);
}
impl ::windows::core::RuntimeName for AppCapability {
    const NAME: &'static str = "Windows.Security.Authorization.AppCapabilityAccess.AppCapability";
}
impl ::core::convert::From<AppCapability> for ::windows::core::IUnknown {
    fn from(value: AppCapability) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppCapability> for ::windows::core::IUnknown {
    fn from(value: &AppCapability) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCapability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCapability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppCapability> for ::windows::core::IInspectable {
    fn from(value: AppCapability) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppCapability> for ::windows::core::IInspectable {
    fn from(value: &AppCapability) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCapability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCapability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppCapability {}
unsafe impl ::core::marker::Sync for AppCapability {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppCapabilityAccessChangedEventArgs(pub ::windows::core::IInspectable);
impl AppCapabilityAccessChangedEventArgs {}
unsafe impl ::windows::core::RuntimeType for AppCapabilityAccessChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessChangedEventArgs;{0a578d15-bdd7-457e-8cca-6f53bd2e5944})");
}
unsafe impl ::windows::core::Interface for AppCapabilityAccessChangedEventArgs {
    type Vtable = IAppCapabilityAccessChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a578d15_bdd7_457e_8cca_6f53bd2e5944);
}
impl ::windows::core::RuntimeName for AppCapabilityAccessChangedEventArgs {
    const NAME: &'static str = "Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessChangedEventArgs";
}
impl ::core::convert::From<AppCapabilityAccessChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppCapabilityAccessChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppCapabilityAccessChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppCapabilityAccessChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCapabilityAccessChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppCapabilityAccessChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppCapabilityAccessChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppCapabilityAccessChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppCapabilityAccessChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppCapabilityAccessChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCapabilityAccessChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppCapabilityAccessChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for AppCapabilityAccessChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppCapabilityAccessChangedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppCapabilityAccessStatus(pub i32);
impl AppCapabilityAccessStatus {
    pub const DeniedBySystem: AppCapabilityAccessStatus = AppCapabilityAccessStatus(0i32);
    pub const NotDeclaredByApp: AppCapabilityAccessStatus = AppCapabilityAccessStatus(1i32);
    pub const DeniedByUser: AppCapabilityAccessStatus = AppCapabilityAccessStatus(2i32);
    pub const UserPromptRequired: AppCapabilityAccessStatus = AppCapabilityAccessStatus(3i32);
    pub const Allowed: AppCapabilityAccessStatus = AppCapabilityAccessStatus(4i32);
}
impl ::core::convert::From<i32> for AppCapabilityAccessStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AppCapabilityAccessStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AppCapabilityAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessStatus;i4)");
}
impl ::windows::core::DefaultType for AppCapabilityAccessStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCapability(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppCapability {
    type Vtable = IAppCapability_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c49d915_8a2a_4295_9437_2df7c396aff4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCapability_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AppCapabilityAccessStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCapabilityAccessChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppCapabilityAccessChangedEventArgs {
    type Vtable = IAppCapabilityAccessChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a578d15_bdd7_457e_8cca_6f53bd2e5944);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCapabilityAccessChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppCapabilityStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppCapabilityStatics {
    type Vtable = IAppCapabilityStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c353e2a_46ee_44e5_af3d_6ad3fc49bd22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCapabilityStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, capabilitynames: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, capabilitynames: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, capabilityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, capabilityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, pid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
