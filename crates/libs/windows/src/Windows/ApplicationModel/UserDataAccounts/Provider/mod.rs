#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountPartnerAccountInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataAccountPartnerAccountInfo {
    type Vtable = IUserDataAccountPartnerAccountInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f200037_f6ef_4ec3_8630_012c59c1149f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountPartnerAccountInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub AccountKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataAccountProviderPartnerAccountKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountProviderAddAccountOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataAccountProviderAddAccountOperation {
    type Vtable = IUserDataAccountProviderAddAccountOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9c72530_3f84_4b5d_8eaa_45e97aa842ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderAddAccountOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ContentKinds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::UserDataAccountContentKinds) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PartnerAccountInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PartnerAccountInfos: usize,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdataaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
#[repr(transparent)]
pub struct IUserDataAccountProviderOperation(::windows::core::IUnknown);
impl IUserDataAccountProviderOperation {
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<UserDataAccountProviderOperationKind> {
        let this = self;
        unsafe {
            let mut result__: UserDataAccountProviderOperationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAccountProviderOperationKind>(result__)
        }
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUserDataAccountProviderOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IUserDataAccountProviderOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
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
impl ::core::fmt::Debug for IUserDataAccountProviderOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserDataAccountProviderOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IUserDataAccountProviderOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a20aad63-888c-4a62-a3dd-34d07a802b2b}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IUserDataAccountProviderOperation {
    type Vtable = IUserDataAccountProviderOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa20aad63_888c_4a62_a3dd_34d07a802b2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataAccountProviderOperationKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountProviderResolveErrorsOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataAccountProviderResolveErrorsOperation {
    type Vtable = IUserDataAccountProviderResolveErrorsOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6235dc15_bfcb_41e1_9957_9759a28846cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderResolveErrorsOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountProviderSettingsOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataAccountProviderSettingsOperation {
    type Vtable = IUserDataAccountProviderSettingsOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92034db7_8648_4f30_acfa_3002658ca80d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderSettingsOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
#[repr(transparent)]
pub struct UserDataAccountPartnerAccountInfo(::windows::core::IUnknown);
impl UserDataAccountPartnerAccountInfo {
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
    pub fn Priority(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Priority)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
    pub fn AccountKind(&self) -> ::windows::core::Result<UserDataAccountProviderPartnerAccountKind> {
        let this = self;
        unsafe {
            let mut result__: UserDataAccountProviderPartnerAccountKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccountKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAccountProviderPartnerAccountKind>(result__)
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
impl ::core::fmt::Debug for UserDataAccountPartnerAccountInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountPartnerAccountInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataAccountPartnerAccountInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountPartnerAccountInfo;{5f200037-f6ef-4ec3-8630-012c59c1149f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserDataAccountPartnerAccountInfo {
    type Vtable = IUserDataAccountPartnerAccountInfo_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataAccountPartnerAccountInfo as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataAccountPartnerAccountInfo {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataAccountPartnerAccountInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataAccountPartnerAccountInfo {}
unsafe impl ::core::marker::Sync for UserDataAccountPartnerAccountInfo {}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
#[repr(transparent)]
pub struct UserDataAccountProviderAddAccountOperation(::windows::core::IUnknown);
impl UserDataAccountProviderAddAccountOperation {
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
    pub fn ContentKinds(&self) -> ::windows::core::Result<super::UserDataAccountContentKinds> {
        let this = self;
        unsafe {
            let mut result__: super::UserDataAccountContentKinds = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentKinds)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UserDataAccountContentKinds>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PartnerAccountInfos(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<UserDataAccountPartnerAccountInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PartnerAccountInfos)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<UserDataAccountPartnerAccountInfo>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
    pub fn ReportCompleted<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, userdataaccountid: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCompleted)(::core::mem::transmute_copy(this), userdataaccountid.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<UserDataAccountProviderOperationKind> {
        let this = &::windows::core::Interface::cast::<IUserDataAccountProviderOperation>(self)?;
        unsafe {
            let mut result__: UserDataAccountProviderOperationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAccountProviderOperationKind>(result__)
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
impl ::core::fmt::Debug for UserDataAccountProviderAddAccountOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountProviderAddAccountOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataAccountProviderAddAccountOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderAddAccountOperation;{b9c72530-3f84-4b5d-8eaa-45e97aa842ed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserDataAccountProviderAddAccountOperation {
    type Vtable = IUserDataAccountProviderAddAccountOperation_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataAccountProviderAddAccountOperation as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataAccountProviderAddAccountOperation {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataAccountProviderAddAccountOperation {
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
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for UserDataAccountProviderOperationKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserDataAccountProviderOperationKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataAccountProviderOperationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountProviderOperationKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataAccountProviderOperationKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderOperationKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
impl ::core::default::Default for UserDataAccountProviderPartnerAccountKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserDataAccountProviderPartnerAccountKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataAccountProviderPartnerAccountKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountProviderPartnerAccountKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataAccountProviderPartnerAccountKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderPartnerAccountKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
#[repr(transparent)]
pub struct UserDataAccountProviderResolveErrorsOperation(::windows::core::IUnknown);
impl UserDataAccountProviderResolveErrorsOperation {
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<UserDataAccountProviderOperationKind> {
        let this = &::windows::core::Interface::cast::<IUserDataAccountProviderOperation>(self)?;
        unsafe {
            let mut result__: UserDataAccountProviderOperationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAccountProviderOperationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
    pub fn UserDataAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UserDataAccountId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
    pub fn ReportCompleted(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCompleted)(::core::mem::transmute_copy(this)).ok() }
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
impl ::core::fmt::Debug for UserDataAccountProviderResolveErrorsOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountProviderResolveErrorsOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataAccountProviderResolveErrorsOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderResolveErrorsOperation;{6235dc15-bfcb-41e1-9957-9759a28846cc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserDataAccountProviderResolveErrorsOperation {
    type Vtable = IUserDataAccountProviderResolveErrorsOperation_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataAccountProviderResolveErrorsOperation as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataAccountProviderResolveErrorsOperation {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataAccountProviderResolveErrorsOperation {
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
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
#[repr(transparent)]
pub struct UserDataAccountProviderSettingsOperation(::windows::core::IUnknown);
impl UserDataAccountProviderSettingsOperation {
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<UserDataAccountProviderOperationKind> {
        let this = &::windows::core::Interface::cast::<IUserDataAccountProviderOperation>(self)?;
        unsafe {
            let mut result__: UserDataAccountProviderOperationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAccountProviderOperationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
    pub fn UserDataAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UserDataAccountId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
    pub fn ReportCompleted(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCompleted)(::core::mem::transmute_copy(this)).ok() }
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
impl ::core::fmt::Debug for UserDataAccountProviderSettingsOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountProviderSettingsOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataAccountProviderSettingsOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderSettingsOperation;{92034db7-8648-4f30-acfa-3002658ca80d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserDataAccountProviderSettingsOperation {
    type Vtable = IUserDataAccountProviderSettingsOperation_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataAccountProviderSettingsOperation as ::windows::core::Interface>::IID;
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataAccountProviderSettingsOperation {
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataAccountProviderSettingsOperation {
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
#[cfg(feature = "implement")]
::core::include!("impl.rs");
