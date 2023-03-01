#[doc(hidden)]
#[repr(transparent)]
pub struct INamedPolicyData(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INamedPolicyData {
    type Vtable = INamedPolicyData_Vtbl;
}
impl ::core::clone::Clone for INamedPolicyData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INamedPolicyData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38dcb198_95ac_4077_a643_8078cae26400);
}
#[repr(C)]
#[doc(hidden)]
pub struct INamedPolicyData_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Area: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NamedPolicyKind) -> ::windows::core::HRESULT,
    pub IsManaged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsUserPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    pub GetBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetBinary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetBinary: usize,
    pub GetInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub GetInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changedhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INamedPolicyStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INamedPolicyStatics {
    type Vtable = INamedPolicyStatics_Vtbl;
}
impl ::core::clone::Clone for INamedPolicyStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INamedPolicyStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f793be7_76c4_4058_8cad_67662cd05f0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct INamedPolicyStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetPolicyFromPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, area: ::std::mem::MaybeUninit<::windows::core::HSTRING>, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetPolicyFromPathForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, area: ::std::mem::MaybeUninit<::windows::core::HSTRING>, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetPolicyFromPathForUser: usize,
}
#[doc = "*Required features: `\"Management_Policies\"`*"]
pub struct NamedPolicy;
impl NamedPolicy {
    pub fn GetPolicyFromPath(area: &::windows::core::HSTRING, name: &::windows::core::HSTRING) -> ::windows::core::Result<NamedPolicyData> {
        Self::INamedPolicyStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<NamedPolicyData>();
            (::windows::core::Interface::vtable(this).GetPolicyFromPath)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(area), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetPolicyFromPathForUser(user: &super::super::System::User, area: &::windows::core::HSTRING, name: &::windows::core::HSTRING) -> ::windows::core::Result<NamedPolicyData> {
        Self::INamedPolicyStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<NamedPolicyData>();
            (::windows::core::Interface::vtable(this).GetPolicyFromPathForUser)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(user), ::core::mem::transmute_copy(area), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn INamedPolicyStatics<R, F: FnOnce(&INamedPolicyStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<NamedPolicy, INamedPolicyStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for NamedPolicy {
    const NAME: &'static str = "Windows.Management.Policies.NamedPolicy";
}
#[doc = "*Required features: `\"Management_Policies\"`*"]
#[repr(transparent)]
pub struct NamedPolicyData(::windows::core::IUnknown);
impl NamedPolicyData {
    pub fn Area(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Area)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<NamedPolicyKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<NamedPolicyKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsManaged(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsManaged)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsUserPolicy(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsUserPolicy)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).GetBoolean)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetBinary(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IBuffer>();
            (::windows::core::Interface::vtable(this).GetBinary)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetInt32(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).GetInt32)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetInt64(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i64>();
            (::windows::core::Interface::vtable(this).GetInt64)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Changed(&self, changedhandler: &super::super::Foundation::TypedEventHandler<NamedPolicyData, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Changed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(changedhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveChanged)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
}
impl ::core::cmp::PartialEq for NamedPolicyData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NamedPolicyData {}
impl ::core::fmt::Debug for NamedPolicyData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NamedPolicyData").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for NamedPolicyData {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Management.Policies.NamedPolicyData;{38dcb198-95ac-4077-a643-8078cae26400})");
}
impl ::core::clone::Clone for NamedPolicyData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for NamedPolicyData {
    type Vtable = INamedPolicyData_Vtbl;
}
unsafe impl ::windows::core::ComInterface for NamedPolicyData {
    const IID: ::windows::core::GUID = <INamedPolicyData as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for NamedPolicyData {
    const NAME: &'static str = "Windows.Management.Policies.NamedPolicyData";
}
::windows::imp::interface_hierarchy!(NamedPolicyData, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for NamedPolicyData {}
unsafe impl ::core::marker::Sync for NamedPolicyData {}
#[doc = "*Required features: `\"Management_Policies\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NamedPolicyKind(pub i32);
impl NamedPolicyKind {
    pub const Invalid: Self = Self(0i32);
    pub const Binary: Self = Self(1i32);
    pub const Boolean: Self = Self(2i32);
    pub const Int32: Self = Self(3i32);
    pub const Int64: Self = Self(4i32);
    pub const String: Self = Self(5i32);
}
impl ::core::marker::Copy for NamedPolicyKind {}
impl ::core::clone::Clone for NamedPolicyKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NamedPolicyKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for NamedPolicyKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for NamedPolicyKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NamedPolicyKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for NamedPolicyKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Policies.NamedPolicyKind;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
