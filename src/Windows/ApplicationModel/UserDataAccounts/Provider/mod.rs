#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountPartnerAccountInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataAccountPartnerAccountInfo {
    type Vtable = IUserDataAccountPartnerAccountInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f200037_f6ef_4ec3_8630_012c59c1149f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountPartnerAccountInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataAccountProviderPartnerAccountKind) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountProviderAddAccountOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataAccountProviderAddAccountOperation {
    type Vtable = IUserDataAccountProviderAddAccountOperationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9c72530_3f84_4b5d_8eaa_45e97aa842ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderAddAccountOperationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::UserDataAccountContentKinds) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdataaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IUserDataAccountProviderOperation(::windows::core::IUnknown);
impl IUserDataAccountProviderOperation {
    pub fn Kind(&self) -> ::windows::core::Result<UserDataAccountProviderOperationKind> {
        let this = self;
        unsafe {
            let mut result__: UserDataAccountProviderOperationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAccountProviderOperationKind>(result__)
        }
    }
}
impl ::core::convert::From<IUserDataAccountProviderOperation> for ::windows::core::IInspectable {
    fn from(value: IUserDataAccountProviderOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserDataAccountProviderOperation> for ::windows::core::IInspectable {
    fn from(value: &IUserDataAccountProviderOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IUserDataAccountProviderOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IUserDataAccountProviderOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IUserDataAccountProviderOperation> for ::windows::core::IUnknown {
    fn from(value: IUserDataAccountProviderOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUserDataAccountProviderOperation> for ::windows::core::IUnknown {
    fn from(value: &IUserDataAccountProviderOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUserDataAccountProviderOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IUserDataAccountProviderOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUserDataAccountProviderOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUserDataAccountProviderOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserDataAccountProviderOperation {}
unsafe impl ::windows::core::RuntimeType for IUserDataAccountProviderOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a20aad63-888c-4a62-a3dd-34d07a802b2b}");
}
unsafe impl ::windows::core::Interface for IUserDataAccountProviderOperation {
    type Vtable = IUserDataAccountProviderOperationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa20aad63_888c_4a62_a3dd_34d07a802b2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderOperationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataAccountProviderOperationKind) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountProviderResolveErrorsOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataAccountProviderResolveErrorsOperation {
    type Vtable = IUserDataAccountProviderResolveErrorsOperationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6235dc15_bfcb_41e1_9957_9759a28846cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderResolveErrorsOperationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountProviderSettingsOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataAccountProviderSettingsOperation {
    type Vtable = IUserDataAccountProviderSettingsOperationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92034db7_8648_4f30_acfa_3002658ca80d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderSettingsOperationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct UserDataAccountPartnerAccountInfo(::windows::core::IUnknown);
impl UserDataAccountPartnerAccountInfo {
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Priority(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn AccountKind(&self) -> ::windows::core::Result<UserDataAccountProviderPartnerAccountKind> {
        let this = self;
        unsafe {
            let mut result__: UserDataAccountProviderPartnerAccountKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAccountProviderPartnerAccountKind>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataAccountPartnerAccountInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataAccountPartnerAccountInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccountPartnerAccountInfo {}
unsafe impl ::windows::core::RuntimeType for UserDataAccountPartnerAccountInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountPartnerAccountInfo;{5f200037-f6ef-4ec3-8630-012c59c1149f})");
}
unsafe impl ::windows::core::Interface for UserDataAccountPartnerAccountInfo {
    type Vtable = IUserDataAccountPartnerAccountInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f200037_f6ef_4ec3_8630_012c59c1149f);
}
impl ::windows::core::RuntimeName for UserDataAccountPartnerAccountInfo {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountPartnerAccountInfo";
}
impl ::core::convert::From<UserDataAccountPartnerAccountInfo> for ::windows::core::IUnknown {
    fn from(value: UserDataAccountPartnerAccountInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountPartnerAccountInfo> for ::windows::core::IUnknown {
    fn from(value: &UserDataAccountPartnerAccountInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataAccountPartnerAccountInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UserDataAccountPartnerAccountInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataAccountPartnerAccountInfo> for ::windows::core::IInspectable {
    fn from(value: UserDataAccountPartnerAccountInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountPartnerAccountInfo> for ::windows::core::IInspectable {
    fn from(value: &UserDataAccountPartnerAccountInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataAccountPartnerAccountInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UserDataAccountPartnerAccountInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataAccountPartnerAccountInfo {}
unsafe impl ::core::marker::Sync for UserDataAccountPartnerAccountInfo {}
#[repr(transparent)]
pub struct UserDataAccountProviderAddAccountOperation(::windows::core::IUnknown);
impl UserDataAccountProviderAddAccountOperation {
    pub fn ContentKinds(&self) -> ::windows::core::Result<super::UserDataAccountContentKinds> {
        let this = self;
        unsafe {
            let mut result__: super::UserDataAccountContentKinds = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UserDataAccountContentKinds>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PartnerAccountInfos(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<UserDataAccountPartnerAccountInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<UserDataAccountPartnerAccountInfo>>(result__)
        }
    }
    pub fn ReportCompleted<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, userdataaccountid: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), userdataaccountid.into_param().abi()).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<UserDataAccountProviderOperationKind> {
        let this = &::windows::core::Interface::cast::<IUserDataAccountProviderOperation>(self)?;
        unsafe {
            let mut result__: UserDataAccountProviderOperationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAccountProviderOperationKind>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataAccountProviderAddAccountOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataAccountProviderAddAccountOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccountProviderAddAccountOperation {}
unsafe impl ::windows::core::RuntimeType for UserDataAccountProviderAddAccountOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderAddAccountOperation;{b9c72530-3f84-4b5d-8eaa-45e97aa842ed})");
}
unsafe impl ::windows::core::Interface for UserDataAccountProviderAddAccountOperation {
    type Vtable = IUserDataAccountProviderAddAccountOperationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9c72530_3f84_4b5d_8eaa_45e97aa842ed);
}
impl ::windows::core::RuntimeName for UserDataAccountProviderAddAccountOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderAddAccountOperation";
}
impl ::core::convert::From<UserDataAccountProviderAddAccountOperation> for ::windows::core::IUnknown {
    fn from(value: UserDataAccountProviderAddAccountOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountProviderAddAccountOperation> for ::windows::core::IUnknown {
    fn from(value: &UserDataAccountProviderAddAccountOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UserDataAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataAccountProviderAddAccountOperation> for ::windows::core::IInspectable {
    fn from(value: UserDataAccountProviderAddAccountOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountProviderAddAccountOperation> for ::windows::core::IInspectable {
    fn from(value: &UserDataAccountProviderAddAccountOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UserDataAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<UserDataAccountProviderAddAccountOperation> for IUserDataAccountProviderOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: UserDataAccountProviderAddAccountOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserDataAccountProviderAddAccountOperation> for IUserDataAccountProviderOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &UserDataAccountProviderAddAccountOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IUserDataAccountProviderOperation> for UserDataAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IUserDataAccountProviderOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IUserDataAccountProviderOperation> for &UserDataAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IUserDataAccountProviderOperation> {
        ::core::convert::TryInto::<IUserDataAccountProviderOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for UserDataAccountProviderAddAccountOperation {}
unsafe impl ::core::marker::Sync for UserDataAccountProviderAddAccountOperation {}
#[repr(transparent)]
pub struct UserDataAccountProviderOperationKind(pub i32);
impl UserDataAccountProviderOperationKind {
    pub const AddAccount: Self = Self(0i32);
    pub const Settings: Self = Self(1i32);
    pub const ResolveErrors: Self = Self(2i32);
}
impl ::core::marker::Copy for UserDataAccountProviderOperationKind {}
impl ::core::clone::Clone for UserDataAccountProviderOperationKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UserDataAccountProviderOperationKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UserDataAccountProviderOperationKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccountProviderOperationKind {}
unsafe impl ::windows::core::RuntimeType for UserDataAccountProviderOperationKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderOperationKind;i4)");
}
impl ::windows::core::DefaultType for UserDataAccountProviderOperationKind {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct UserDataAccountProviderPartnerAccountKind(pub i32);
impl UserDataAccountProviderPartnerAccountKind {
    pub const Exchange: Self = Self(0i32);
    pub const PopOrImap: Self = Self(1i32);
}
impl ::core::marker::Copy for UserDataAccountProviderPartnerAccountKind {}
impl ::core::clone::Clone for UserDataAccountProviderPartnerAccountKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UserDataAccountProviderPartnerAccountKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UserDataAccountProviderPartnerAccountKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccountProviderPartnerAccountKind {}
unsafe impl ::windows::core::RuntimeType for UserDataAccountProviderPartnerAccountKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderPartnerAccountKind;i4)");
}
impl ::windows::core::DefaultType for UserDataAccountProviderPartnerAccountKind {
    type DefaultType = Self;
}
#[repr(transparent)]
pub struct UserDataAccountProviderResolveErrorsOperation(::windows::core::IUnknown);
impl UserDataAccountProviderResolveErrorsOperation {
    pub fn Kind(&self) -> ::windows::core::Result<UserDataAccountProviderOperationKind> {
        let this = &::windows::core::Interface::cast::<IUserDataAccountProviderOperation>(self)?;
        unsafe {
            let mut result__: UserDataAccountProviderOperationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAccountProviderOperationKind>(result__)
        }
    }
    pub fn UserDataAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ReportCompleted(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for UserDataAccountProviderResolveErrorsOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataAccountProviderResolveErrorsOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccountProviderResolveErrorsOperation {}
unsafe impl ::windows::core::RuntimeType for UserDataAccountProviderResolveErrorsOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderResolveErrorsOperation;{6235dc15-bfcb-41e1-9957-9759a28846cc})");
}
unsafe impl ::windows::core::Interface for UserDataAccountProviderResolveErrorsOperation {
    type Vtable = IUserDataAccountProviderResolveErrorsOperationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6235dc15_bfcb_41e1_9957_9759a28846cc);
}
impl ::windows::core::RuntimeName for UserDataAccountProviderResolveErrorsOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderResolveErrorsOperation";
}
impl ::core::convert::From<UserDataAccountProviderResolveErrorsOperation> for ::windows::core::IUnknown {
    fn from(value: UserDataAccountProviderResolveErrorsOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountProviderResolveErrorsOperation> for ::windows::core::IUnknown {
    fn from(value: &UserDataAccountProviderResolveErrorsOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataAccountProviderResolveErrorsOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UserDataAccountProviderResolveErrorsOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataAccountProviderResolveErrorsOperation> for ::windows::core::IInspectable {
    fn from(value: UserDataAccountProviderResolveErrorsOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountProviderResolveErrorsOperation> for ::windows::core::IInspectable {
    fn from(value: &UserDataAccountProviderResolveErrorsOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataAccountProviderResolveErrorsOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UserDataAccountProviderResolveErrorsOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<UserDataAccountProviderResolveErrorsOperation> for IUserDataAccountProviderOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: UserDataAccountProviderResolveErrorsOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserDataAccountProviderResolveErrorsOperation> for IUserDataAccountProviderOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &UserDataAccountProviderResolveErrorsOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IUserDataAccountProviderOperation> for UserDataAccountProviderResolveErrorsOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IUserDataAccountProviderOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IUserDataAccountProviderOperation> for &UserDataAccountProviderResolveErrorsOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IUserDataAccountProviderOperation> {
        ::core::convert::TryInto::<IUserDataAccountProviderOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for UserDataAccountProviderResolveErrorsOperation {}
unsafe impl ::core::marker::Sync for UserDataAccountProviderResolveErrorsOperation {}
#[repr(transparent)]
pub struct UserDataAccountProviderSettingsOperation(::windows::core::IUnknown);
impl UserDataAccountProviderSettingsOperation {
    pub fn Kind(&self) -> ::windows::core::Result<UserDataAccountProviderOperationKind> {
        let this = &::windows::core::Interface::cast::<IUserDataAccountProviderOperation>(self)?;
        unsafe {
            let mut result__: UserDataAccountProviderOperationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAccountProviderOperationKind>(result__)
        }
    }
    pub fn UserDataAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ReportCompleted(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for UserDataAccountProviderSettingsOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataAccountProviderSettingsOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccountProviderSettingsOperation {}
unsafe impl ::windows::core::RuntimeType for UserDataAccountProviderSettingsOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderSettingsOperation;{92034db7-8648-4f30-acfa-3002658ca80d})");
}
unsafe impl ::windows::core::Interface for UserDataAccountProviderSettingsOperation {
    type Vtable = IUserDataAccountProviderSettingsOperationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92034db7_8648_4f30_acfa_3002658ca80d);
}
impl ::windows::core::RuntimeName for UserDataAccountProviderSettingsOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderSettingsOperation";
}
impl ::core::convert::From<UserDataAccountProviderSettingsOperation> for ::windows::core::IUnknown {
    fn from(value: UserDataAccountProviderSettingsOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountProviderSettingsOperation> for ::windows::core::IUnknown {
    fn from(value: &UserDataAccountProviderSettingsOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataAccountProviderSettingsOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UserDataAccountProviderSettingsOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataAccountProviderSettingsOperation> for ::windows::core::IInspectable {
    fn from(value: UserDataAccountProviderSettingsOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountProviderSettingsOperation> for ::windows::core::IInspectable {
    fn from(value: &UserDataAccountProviderSettingsOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataAccountProviderSettingsOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UserDataAccountProviderSettingsOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<UserDataAccountProviderSettingsOperation> for IUserDataAccountProviderOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: UserDataAccountProviderSettingsOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserDataAccountProviderSettingsOperation> for IUserDataAccountProviderOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &UserDataAccountProviderSettingsOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IUserDataAccountProviderOperation> for UserDataAccountProviderSettingsOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IUserDataAccountProviderOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IUserDataAccountProviderOperation> for &UserDataAccountProviderSettingsOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IUserDataAccountProviderOperation> {
        ::core::convert::TryInto::<IUserDataAccountProviderOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for UserDataAccountProviderSettingsOperation {}
unsafe impl ::core::marker::Sync for UserDataAccountProviderSettingsOperation {}
