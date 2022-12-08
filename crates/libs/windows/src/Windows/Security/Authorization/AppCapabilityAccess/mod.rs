#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCapability(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppCapability {
    type Vtable = IAppCapability_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppCapability {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c49d915_8a2a_4295_9437_2df7c396aff4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCapability_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CapabilityName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
    pub CheckAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppCapabilityAccessStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AccessChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AccessChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAccessChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAccessChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCapability2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppCapability2 {
    type Vtable = IAppCapability2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppCapability2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11c7ccb6_c74f_50a3_b960_88008767d939);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCapability2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDisplayMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCapabilityAccessChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppCapabilityAccessChangedEventArgs {
    type Vtable = IAppCapabilityAccessChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppCapabilityAccessChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a578d15_bdd7_457e_8cca_6f53bd2e5944);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCapabilityAccessChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppCapabilityStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IAppCapabilityStatics {
    type Vtable = IAppCapabilityStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppCapabilityStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c353e2a_46ee_44e5_af3d_6ad3fc49bd22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppCapabilityStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestAccessForCapabilitiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilitynames: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestAccessForCapabilitiesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub RequestAccessForCapabilitiesForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, capabilitynames: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System")))]
    RequestAccessForCapabilitiesForUserAsync: usize,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilityname: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub CreateWithProcessIdForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, capabilityname: *mut ::core::ffi::c_void, pid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    CreateWithProcessIdForUser: usize,
}
#[doc = "*Required features: `\"Security_Authorization_AppCapabilityAccess\"`*"]
#[repr(transparent)]
pub struct AppCapability(::windows::core::IUnknown);
impl AppCapability {
    pub fn CapabilityName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CapabilityName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<AppCapabilityAccessStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestAccessAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CheckAccess(&self) -> ::windows::core::Result<AppCapabilityAccessStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CheckAccess)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AccessChanged(&self, handler: &super::super::super::Foundation::TypedEventHandler<AppCapability, AppCapabilityAccessChangedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AccessChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAccessChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAccessChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn DisplayMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppCapability2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayMessage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDisplayMessage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppCapability2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetDisplayMessage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestAccessForCapabilitiesAsync<P0, E0>(capabilitynames: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, AppCapabilityAccessStatus>>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IAppCapabilityStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestAccessForCapabilitiesAsync)(::windows::core::Vtable::as_raw(this), capabilitynames.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub fn RequestAccessForCapabilitiesForUserAsync<P0, E0>(user: &super::super::super::System::User, capabilitynames: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, AppCapabilityAccessStatus>>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IAppCapabilityStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestAccessForCapabilitiesForUserAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), capabilitynames.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Create(capabilityname: &::windows::core::HSTRING) -> ::windows::core::Result<AppCapability> {
        Self::IAppCapabilityStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(capabilityname), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn CreateWithProcessIdForUser(user: &super::super::super::System::User, capabilityname: &::windows::core::HSTRING, pid: u32) -> ::windows::core::Result<AppCapability> {
        Self::IAppCapabilityStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithProcessIdForUser)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(capabilityname), pid, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppCapabilityStatics<R, F: FnOnce(&IAppCapabilityStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AppCapability, IAppCapabilityStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppCapability {
    type Vtable = IAppCapability_Vtbl;
}
unsafe impl ::windows::core::Interface for AppCapability {
    const IID: ::windows::core::GUID = <IAppCapability as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppCapability {
    const NAME: &'static str = "Windows.Security.Authorization.AppCapabilityAccess.AppCapability";
}
::windows::core::interface_hierarchy!(AppCapability, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppCapability {}
unsafe impl ::core::marker::Sync for AppCapability {}
#[doc = "*Required features: `\"Security_Authorization_AppCapabilityAccess\"`*"]
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
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for AppCapabilityAccessChangedEventArgs {
    type Vtable = IAppCapabilityAccessChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for AppCapabilityAccessChangedEventArgs {
    const IID: ::windows::core::GUID = <IAppCapabilityAccessChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppCapabilityAccessChangedEventArgs {
    const NAME: &'static str = "Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessChangedEventArgs";
}
::windows::core::interface_hierarchy!(AppCapabilityAccessChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppCapabilityAccessChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppCapabilityAccessChangedEventArgs {}
#[doc = "*Required features: `\"Security_Authorization_AppCapabilityAccess\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for AppCapabilityAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppCapabilityAccessStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppCapabilityAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppCapabilityAccessStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppCapabilityAccessStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authorization.AppCapabilityAccess.AppCapabilityAccessStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
