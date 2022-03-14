#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct INamedPolicyData(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INamedPolicyData {
    type Vtable = INamedPolicyData_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38dcb198_95ac_4077_a643_8078cae26400);
}
#[repr(C)]
#[doc(hidden)]
pub struct INamedPolicyData_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Area: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NamedPolicyKind) -> ::windows::core::HRESULT,
    pub IsManaged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsUserPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    pub GetBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetBinary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetBinary: usize,
    pub GetInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub GetInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changedhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f793be7_76c4_4058_8cad_67662cd05f0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct INamedPolicyStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetPolicyFromPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, area: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetPolicyFromPathForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, area: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetPolicyFromPathForUser: usize,
}
#[doc = "*Required features: `\"Management_Policies\"`*"]
pub struct NamedPolicy {}
impl NamedPolicy {
    #[doc = "*Required features: `\"Management_Policies\"`*"]
    pub fn GetPolicyFromPath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(area: Param0, name: Param1) -> ::windows::core::Result<NamedPolicyData> {
        Self::INamedPolicyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPolicyFromPath)(::core::mem::transmute_copy(this), area.into_param().abi(), name.into_param().abi(), &mut result__).from_abi::<NamedPolicyData>(result__)
        })
    }
    #[doc = "*Required features: `\"Management_Policies\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetPolicyFromPathForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, area: Param1, name: Param2) -> ::windows::core::Result<NamedPolicyData> {
        Self::INamedPolicyStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPolicyFromPathForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), area.into_param().abi(), name.into_param().abi(), &mut result__).from_abi::<NamedPolicyData>(result__)
        })
    }
    #[doc(hidden)]
    pub fn INamedPolicyStatics<R, F: FnOnce(&INamedPolicyStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<NamedPolicy, INamedPolicyStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for NamedPolicy {
    const NAME: &'static str = "Windows.Management.Policies.NamedPolicy";
}
#[doc = "*Required features: `\"Management_Policies\"`*"]
#[repr(transparent)]
pub struct NamedPolicyData(::windows::core::IUnknown);
impl NamedPolicyData {
    #[doc = "*Required features: `\"Management_Policies\"`*"]
    pub fn Area(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Area)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Management_Policies\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Management_Policies\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<NamedPolicyKind> {
        let this = self;
        unsafe {
            let mut result__: NamedPolicyKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<NamedPolicyKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Management_Policies\"`*"]
    pub fn IsManaged(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsManaged)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Management_Policies\"`*"]
    pub fn IsUserPolicy(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsUserPolicy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Management_Policies\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).User)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"Management_Policies\"`*"]
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBoolean)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Management_Policies\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetBinary(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetBinary)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Management_Policies\"`*"]
    pub fn GetInt32(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetInt32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Management_Policies\"`*"]
    pub fn GetInt64(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetInt64)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `\"Management_Policies\"`*"]
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetString)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Management_Policies\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Changed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<NamedPolicyData, ::windows::core::IInspectable>>>(&self, changedhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Changed)(::core::mem::transmute_copy(this), changedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Management_Policies\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveChanged)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for NamedPolicyData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for NamedPolicyData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Management.Policies.NamedPolicyData;{38dcb198-95ac-4077-a643-8078cae26400})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for NamedPolicyData {
    type Vtable = INamedPolicyData_Vtbl;
    const IID: ::windows::core::GUID = <INamedPolicyData as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for NamedPolicyData {
    const NAME: &'static str = "Windows.Management.Policies.NamedPolicyData";
}
impl ::core::convert::From<NamedPolicyData> for ::windows::core::IUnknown {
    fn from(value: NamedPolicyData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NamedPolicyData> for ::windows::core::IUnknown {
    fn from(value: &NamedPolicyData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for NamedPolicyData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a NamedPolicyData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<NamedPolicyData> for ::windows::core::IInspectable {
    fn from(value: NamedPolicyData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NamedPolicyData> for ::windows::core::IInspectable {
    fn from(value: &NamedPolicyData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for NamedPolicyData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a NamedPolicyData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for NamedPolicyData {}
unsafe impl ::core::marker::Sync for NamedPolicyData {}
#[doc = "*Required features: `\"Management_Policies\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for NamedPolicyKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for NamedPolicyKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NamedPolicyKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NamedPolicyKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Management.Policies.NamedPolicyKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
