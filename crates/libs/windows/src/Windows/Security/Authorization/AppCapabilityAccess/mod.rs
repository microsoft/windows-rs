#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Security_Authorization_AppCapabilityAccess'*"]
#[repr(transparent)]
pub struct AppCapability(::windows::core::IUnknown);
impl AppCapability {
    #[doc = "*Required features: 'Security_Authorization_AppCapabilityAccess'*"]
    pub fn CapabilityName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authorization_AppCapabilityAccess', 'System'*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authorization_AppCapabilityAccess', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<AppCapabilityAccessStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<AppCapabilityAccessStatus>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authorization_AppCapabilityAccess'*"]
    pub fn CheckAccess(&self) -> ::windows::core::Result<AppCapabilityAccessStatus> {
        let this = self;
        unsafe {
            let mut result__: AppCapabilityAccessStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppCapabilityAccessStatus>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authorization_AppCapabilityAccess', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn AccessChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AppCapability, AppCapabilityAccessChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authorization_AppCapabilityAccess', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAccessChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Security_Authorization_AppCapabilityAccess', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RequestAccessForCapabilitiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(capabilitynames: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, AppCapabilityAccessStatus>>> {
        Self::IAppCapabilityStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), capabilitynames.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, AppCapabilityAccessStatus>>>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Authorization_AppCapabilityAccess', 'Foundation', 'Foundation_Collections', 'System'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System"))]
    pub fn RequestAccessForCapabilitiesForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(user: Param0, capabilitynames: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, AppCapabilityAccessStatus>>> {
        Self::IAppCapabilityStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), user.into_param().abi(), capabilitynames.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, AppCapabilityAccessStatus>>>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Authorization_AppCapabilityAccess'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(capabilityname: Param0) -> ::windows::core::Result<AppCapability> {
        Self::IAppCapabilityStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), capabilityname.into_param().abi(), &mut result__).from_abi::<AppCapability>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Authorization_AppCapabilityAccess', 'System'*"]
    #[cfg(feature = "System")]
    pub fn CreateWithProcessIdForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, capabilityname: Param1, pid: u32) -> ::windows::core::Result<AppCapability> {
        Self::IAppCapabilityStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), user.into_param().abi(), capabilityname.into_param().abi(), pid, &mut result__).from_abi::<AppCapability>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppCapabilityStatics<R, F: FnOnce(&IAppCapabilityStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppCapability, IAppCapabilityStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppCapability {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppCapability {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCapability {}
impl ::core::fmt::Debug for AppCapability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCapability").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppCapability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authorization.AppCapabilityAccess.AppCapability;{4c49d915-8a2a-4295-9437-2df7c396aff4})");
}
unsafe impl ::windows::core::Interface for AppCapability {
    type Vtable = IAppCapabilityVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c49d915_8a2a_4295_9437_2df7c396aff4);
}
impl ::windows::core::RuntimeName for AppCapability {
    const NAME: &'static str = "Windows.Security.Authorization.AppCapabilityAccess.AppCapability";
}
impl ::core::convert::From<AppCapability> for ::windows::core::IUnknown {
    fn from(value: AppCapability) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCapability> for ::windows::core::IUnknown {
    fn from(value: &AppCapability) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCapability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppCapability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCapability> for ::windows::core::IInspectable {
    fn from(value: AppCapability) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCapability> for ::windows::core::IInspectable {
    fn from(value: &AppCapability) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCapability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppCapability {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppCapability {}
unsafe impl ::core::marker::Sync for AppCapability {}
#[doc = "*Required features: 'Security_Authorization_AppCapabilityAccess'*"]
#[repr(transparent)]
pub struct AppCapabilityAccessChangedEventArgs(::windows::core::IUnknown);
impl AppCapabilityAccessChangedEventArgs {}
impl ::core::clone::Clone for AppCapabilityAccessChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppCapabilityAccessChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCapabilityAccessChangedEventArgs {}
impl ::core::fmt::Debug for AppCapabilityAccessChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCapabilityAccessChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppCapabilityAccessChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessChangedEventArgs;{0a578d15-bdd7-457e-8cca-6f53bd2e5944})");
}
unsafe impl ::windows::core::Interface for AppCapabilityAccessChangedEventArgs {
    type Vtable = IAppCapabilityAccessChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a578d15_bdd7_457e_8cca_6f53bd2e5944);
}
impl ::windows::core::RuntimeName for AppCapabilityAccessChangedEventArgs {
    const NAME: &'static str = "Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessChangedEventArgs";
}
impl ::core::convert::From<AppCapabilityAccessChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppCapabilityAccessChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCapabilityAccessChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppCapabilityAccessChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppCapabilityAccessChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppCapabilityAccessChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppCapabilityAccessChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppCapabilityAccessChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppCapabilityAccessChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppCapabilityAccessChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppCapabilityAccessChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppCapabilityAccessChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppCapabilityAccessChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppCapabilityAccessChangedEventArgs {}
#[doc = "*Required features: 'Security_Authorization_AppCapabilityAccess'*"]
#[repr(transparent)]
pub struct AppCapabilityAccessStatus(pub i32);
impl AppCapabilityAccessStatus {
    pub const DeniedBySystem: Self = Self(0i32);
    pub const NotDeclaredByApp: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const UserPromptRequired: Self = Self(3i32);
    pub const Allowed: Self = Self(4i32);
}
impl ::core::marker::Copy for AppCapabilityAccessStatus {}
impl ::core::clone::Clone for AppCapabilityAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AppCapabilityAccessStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AppCapabilityAccessStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppCapabilityAccessStatus {}
impl ::core::fmt::Debug for AppCapabilityAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCapabilityAccessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppCapabilityAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessStatus;i4)");
}
impl ::windows::core::DefaultType for AppCapabilityAccessStatus {
    type DefaultType = Self;
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCapability(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppCapability {
    type Vtable = IAppCapabilityVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c49d915_8a2a_4295_9437_2df7c396aff4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCapabilityVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCapabilityAccessStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCapabilityAccessChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppCapabilityAccessChangedEventArgs {
    type Vtable = IAppCapabilityAccessChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a578d15_bdd7_457e_8cca_6f53bd2e5944);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCapabilityAccessChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCapabilityStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppCapabilityStatics {
    type Vtable = IAppCapabilityStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c353e2a_46ee_44e5_af3d_6ad3fc49bd22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCapabilityStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilitynames: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, capabilitynames: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, capabilityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, pid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
