#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataAccountPartnerAccountInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccountPartnerAccountInfo {
    type Vtable = IUserDataAccountPartnerAccountInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5f200037_f6ef_4ec3_8630_012c59c1149f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountPartnerAccountInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UserDataAccountProviderPartnerAccountKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataAccountProviderAddAccountOperation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccountProviderAddAccountOperation {
    type Vtable = IUserDataAccountProviderAddAccountOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb9c72530_3f84_4b5d_8eaa_45e97aa842ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderAddAccountOperation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::UserDataAccountContentKinds) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, userdataaccountid: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_UserDataAccounts_Provider`*"]
pub struct IUserDataAccountProviderOperation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccountProviderOperation {
    type Vtable = IUserDataAccountProviderOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa20aad63_888c_4a62_a3dd_34d07a802b2b);
}
impl IUserDataAccountProviderOperation {
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_Provider`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<UserDataAccountProviderOperationKind> {
        let this = self;
        unsafe {
            let mut result__: UserDataAccountProviderOperationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAccountProviderOperationKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IUserDataAccountProviderOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{a20aad63-888c-4a62-a3dd-34d07a802b2b}");
}
impl ::core::convert::From<IUserDataAccountProviderOperation> for ::windows::runtime::IUnknown {
    fn from(value: IUserDataAccountProviderOperation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IUserDataAccountProviderOperation> for ::windows::runtime::IUnknown {
    fn from(value: &IUserDataAccountProviderOperation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUserDataAccountProviderOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUserDataAccountProviderOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IUserDataAccountProviderOperation> for ::windows::runtime::IInspectable {
    fn from(value: IUserDataAccountProviderOperation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUserDataAccountProviderOperation> for ::windows::runtime::IInspectable {
    fn from(value: &IUserDataAccountProviderOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IUserDataAccountProviderOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IUserDataAccountProviderOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderOperation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UserDataAccountProviderOperationKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataAccountProviderResolveErrorsOperation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccountProviderResolveErrorsOperation {
    type Vtable = IUserDataAccountProviderResolveErrorsOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6235dc15_bfcb_41e1_9957_9759a28846cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderResolveErrorsOperation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataAccountProviderSettingsOperation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccountProviderSettingsOperation {
    type Vtable = IUserDataAccountProviderSettingsOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x92034db7_8648_4f30_acfa_3002658ca80d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderSettingsOperation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `ApplicationModel_UserDataAccounts_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserDataAccountPartnerAccountInfo(pub ::windows::runtime::IInspectable);
impl UserDataAccountPartnerAccountInfo {
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_Provider`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_Provider`*"]
    pub fn Priority(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_Provider`*"]
    pub fn AccountKind(&self) -> ::windows::runtime::Result<UserDataAccountProviderPartnerAccountKind> {
        let this = self;
        unsafe {
            let mut result__: UserDataAccountProviderPartnerAccountKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAccountProviderPartnerAccountKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataAccountPartnerAccountInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountPartnerAccountInfo;{5f200037-f6ef-4ec3-8630-012c59c1149f})");
}
unsafe impl ::windows::runtime::Interface for UserDataAccountPartnerAccountInfo {
    type Vtable = IUserDataAccountPartnerAccountInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5f200037_f6ef_4ec3_8630_012c59c1149f);
}
impl ::windows::runtime::RuntimeName for UserDataAccountPartnerAccountInfo {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountPartnerAccountInfo";
}
impl ::core::convert::From<UserDataAccountPartnerAccountInfo> for ::windows::runtime::IUnknown {
    fn from(value: UserDataAccountPartnerAccountInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserDataAccountPartnerAccountInfo> for ::windows::runtime::IUnknown {
    fn from(value: &UserDataAccountPartnerAccountInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserDataAccountPartnerAccountInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a UserDataAccountPartnerAccountInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserDataAccountPartnerAccountInfo> for ::windows::runtime::IInspectable {
    fn from(value: UserDataAccountPartnerAccountInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserDataAccountPartnerAccountInfo> for ::windows::runtime::IInspectable {
    fn from(value: &UserDataAccountPartnerAccountInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UserDataAccountPartnerAccountInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UserDataAccountPartnerAccountInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserDataAccountPartnerAccountInfo {}
unsafe impl ::core::marker::Sync for UserDataAccountPartnerAccountInfo {}
#[doc = "*Required features: `ApplicationModel_UserDataAccounts_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserDataAccountProviderAddAccountOperation(pub ::windows::runtime::IInspectable);
impl UserDataAccountProviderAddAccountOperation {
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_Provider`*"]
    pub fn ContentKinds(&self) -> ::windows::runtime::Result<super::UserDataAccountContentKinds> {
        let this = self;
        unsafe {
            let mut result__: super::UserDataAccountContentKinds = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UserDataAccountContentKinds>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_Provider`, `Foundation_Collections`*"]
    pub fn PartnerAccountInfos(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<UserDataAccountPartnerAccountInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<UserDataAccountPartnerAccountInfo>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_Provider`*"]
    pub fn ReportCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, userdataaccountid: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), userdataaccountid.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_Provider`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<UserDataAccountProviderOperationKind> {
        let this = &::windows::runtime::Interface::cast::<IUserDataAccountProviderOperation>(self)?;
        unsafe {
            let mut result__: UserDataAccountProviderOperationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAccountProviderOperationKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataAccountProviderAddAccountOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderAddAccountOperation;{b9c72530-3f84-4b5d-8eaa-45e97aa842ed})");
}
unsafe impl ::windows::runtime::Interface for UserDataAccountProviderAddAccountOperation {
    type Vtable = IUserDataAccountProviderAddAccountOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb9c72530_3f84_4b5d_8eaa_45e97aa842ed);
}
impl ::windows::runtime::RuntimeName for UserDataAccountProviderAddAccountOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderAddAccountOperation";
}
impl ::core::convert::From<UserDataAccountProviderAddAccountOperation> for ::windows::runtime::IUnknown {
    fn from(value: UserDataAccountProviderAddAccountOperation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserDataAccountProviderAddAccountOperation> for ::windows::runtime::IUnknown {
    fn from(value: &UserDataAccountProviderAddAccountOperation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserDataAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a UserDataAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserDataAccountProviderAddAccountOperation> for ::windows::runtime::IInspectable {
    fn from(value: UserDataAccountProviderAddAccountOperation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserDataAccountProviderAddAccountOperation> for ::windows::runtime::IInspectable {
    fn from(value: &UserDataAccountProviderAddAccountOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UserDataAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UserDataAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<UserDataAccountProviderAddAccountOperation> for IUserDataAccountProviderOperation {
    type Error = ::windows::runtime::Error;
    fn try_from(value: UserDataAccountProviderAddAccountOperation) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserDataAccountProviderAddAccountOperation> for IUserDataAccountProviderOperation {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &UserDataAccountProviderAddAccountOperation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IUserDataAccountProviderOperation> for UserDataAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IUserDataAccountProviderOperation> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IUserDataAccountProviderOperation> for &UserDataAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IUserDataAccountProviderOperation> {
        ::core::convert::TryInto::<IUserDataAccountProviderOperation>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for UserDataAccountProviderAddAccountOperation {}
unsafe impl ::core::marker::Sync for UserDataAccountProviderAddAccountOperation {}
#[doc = "*Required features: `ApplicationModel_UserDataAccounts_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataAccountProviderOperationKind(pub i32);
impl UserDataAccountProviderOperationKind {
    pub const AddAccount: UserDataAccountProviderOperationKind = UserDataAccountProviderOperationKind(0i32);
    pub const Settings: UserDataAccountProviderOperationKind = UserDataAccountProviderOperationKind(1i32);
    pub const ResolveErrors: UserDataAccountProviderOperationKind = UserDataAccountProviderOperationKind(2i32);
}
impl ::core::convert::From<i32> for UserDataAccountProviderOperationKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataAccountProviderOperationKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataAccountProviderOperationKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderOperationKind;i4)");
}
impl ::windows::runtime::DefaultType for UserDataAccountProviderOperationKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_UserDataAccounts_Provider`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataAccountProviderPartnerAccountKind(pub i32);
impl UserDataAccountProviderPartnerAccountKind {
    pub const Exchange: UserDataAccountProviderPartnerAccountKind = UserDataAccountProviderPartnerAccountKind(0i32);
    pub const PopOrImap: UserDataAccountProviderPartnerAccountKind = UserDataAccountProviderPartnerAccountKind(1i32);
}
impl ::core::convert::From<i32> for UserDataAccountProviderPartnerAccountKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataAccountProviderPartnerAccountKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataAccountProviderPartnerAccountKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderPartnerAccountKind;i4)");
}
impl ::windows::runtime::DefaultType for UserDataAccountProviderPartnerAccountKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_UserDataAccounts_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserDataAccountProviderResolveErrorsOperation(pub ::windows::runtime::IInspectable);
impl UserDataAccountProviderResolveErrorsOperation {
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_Provider`*"]
    pub fn UserDataAccountId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_Provider`*"]
    pub fn ReportCompleted(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_Provider`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<UserDataAccountProviderOperationKind> {
        let this = &::windows::runtime::Interface::cast::<IUserDataAccountProviderOperation>(self)?;
        unsafe {
            let mut result__: UserDataAccountProviderOperationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAccountProviderOperationKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataAccountProviderResolveErrorsOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderResolveErrorsOperation;{6235dc15-bfcb-41e1-9957-9759a28846cc})");
}
unsafe impl ::windows::runtime::Interface for UserDataAccountProviderResolveErrorsOperation {
    type Vtable = IUserDataAccountProviderResolveErrorsOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6235dc15_bfcb_41e1_9957_9759a28846cc);
}
impl ::windows::runtime::RuntimeName for UserDataAccountProviderResolveErrorsOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderResolveErrorsOperation";
}
impl ::core::convert::From<UserDataAccountProviderResolveErrorsOperation> for ::windows::runtime::IUnknown {
    fn from(value: UserDataAccountProviderResolveErrorsOperation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserDataAccountProviderResolveErrorsOperation> for ::windows::runtime::IUnknown {
    fn from(value: &UserDataAccountProviderResolveErrorsOperation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserDataAccountProviderResolveErrorsOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a UserDataAccountProviderResolveErrorsOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserDataAccountProviderResolveErrorsOperation> for ::windows::runtime::IInspectable {
    fn from(value: UserDataAccountProviderResolveErrorsOperation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserDataAccountProviderResolveErrorsOperation> for ::windows::runtime::IInspectable {
    fn from(value: &UserDataAccountProviderResolveErrorsOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UserDataAccountProviderResolveErrorsOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UserDataAccountProviderResolveErrorsOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<UserDataAccountProviderResolveErrorsOperation> for IUserDataAccountProviderOperation {
    type Error = ::windows::runtime::Error;
    fn try_from(value: UserDataAccountProviderResolveErrorsOperation) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserDataAccountProviderResolveErrorsOperation> for IUserDataAccountProviderOperation {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &UserDataAccountProviderResolveErrorsOperation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IUserDataAccountProviderOperation> for UserDataAccountProviderResolveErrorsOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IUserDataAccountProviderOperation> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IUserDataAccountProviderOperation> for &UserDataAccountProviderResolveErrorsOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IUserDataAccountProviderOperation> {
        ::core::convert::TryInto::<IUserDataAccountProviderOperation>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for UserDataAccountProviderResolveErrorsOperation {}
unsafe impl ::core::marker::Sync for UserDataAccountProviderResolveErrorsOperation {}
#[doc = "*Required features: `ApplicationModel_UserDataAccounts_Provider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserDataAccountProviderSettingsOperation(pub ::windows::runtime::IInspectable);
impl UserDataAccountProviderSettingsOperation {
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_Provider`*"]
    pub fn UserDataAccountId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_Provider`*"]
    pub fn ReportCompleted(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_Provider`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<UserDataAccountProviderOperationKind> {
        let this = &::windows::runtime::Interface::cast::<IUserDataAccountProviderOperation>(self)?;
        unsafe {
            let mut result__: UserDataAccountProviderOperationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAccountProviderOperationKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataAccountProviderSettingsOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderSettingsOperation;{92034db7-8648-4f30-acfa-3002658ca80d})");
}
unsafe impl ::windows::runtime::Interface for UserDataAccountProviderSettingsOperation {
    type Vtable = IUserDataAccountProviderSettingsOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x92034db7_8648_4f30_acfa_3002658ca80d);
}
impl ::windows::runtime::RuntimeName for UserDataAccountProviderSettingsOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderSettingsOperation";
}
impl ::core::convert::From<UserDataAccountProviderSettingsOperation> for ::windows::runtime::IUnknown {
    fn from(value: UserDataAccountProviderSettingsOperation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserDataAccountProviderSettingsOperation> for ::windows::runtime::IUnknown {
    fn from(value: &UserDataAccountProviderSettingsOperation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserDataAccountProviderSettingsOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a UserDataAccountProviderSettingsOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserDataAccountProviderSettingsOperation> for ::windows::runtime::IInspectable {
    fn from(value: UserDataAccountProviderSettingsOperation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserDataAccountProviderSettingsOperation> for ::windows::runtime::IInspectable {
    fn from(value: &UserDataAccountProviderSettingsOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UserDataAccountProviderSettingsOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UserDataAccountProviderSettingsOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<UserDataAccountProviderSettingsOperation> for IUserDataAccountProviderOperation {
    type Error = ::windows::runtime::Error;
    fn try_from(value: UserDataAccountProviderSettingsOperation) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserDataAccountProviderSettingsOperation> for IUserDataAccountProviderOperation {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &UserDataAccountProviderSettingsOperation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IUserDataAccountProviderOperation> for UserDataAccountProviderSettingsOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IUserDataAccountProviderOperation> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IUserDataAccountProviderOperation> for &UserDataAccountProviderSettingsOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IUserDataAccountProviderOperation> {
        ::core::convert::TryInto::<IUserDataAccountProviderOperation>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for UserDataAccountProviderSettingsOperation {}
unsafe impl ::core::marker::Sync for UserDataAccountProviderSettingsOperation {}
