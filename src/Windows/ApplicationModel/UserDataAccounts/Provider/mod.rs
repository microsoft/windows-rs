#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IUserDataAccountPartnerAccountInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccountPartnerAccountInfo {
    type Vtable = IUserDataAccountPartnerAccountInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1595932727, 63215, 20163, [134, 48, 1, 44, 89, 193, 20, 159]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UserDataAccountProviderPartnerAccountKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IUserDataAccountProviderAddAccountOperation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccountProviderAddAccountOperation {
    type Vtable = IUserDataAccountProviderAddAccountOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3116836144, 16260, 19293, [142, 170, 69, 233, 122, 168, 66, 237]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, userdataaccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IUserDataAccountProviderOperation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccountProviderOperation {
    type Vtable = IUserDataAccountProviderOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2718608739, 34956, 19042, [163, 221, 52, 208, 122, 128, 43, 43]);
}
impl IUserDataAccountProviderOperation {
    pub fn Kind(&self) -> ::windows::runtime::Result<UserDataAccountProviderOperationKind> {
        let this = self;
        unsafe {
            let mut result__: UserDataAccountProviderOperationKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAccountProviderOperationKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IUserDataAccountProviderOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{a20aad63-888c-4a62-a3dd-34d07a802b2b}");
}
impl ::std::convert::From<IUserDataAccountProviderOperation> for ::windows::runtime::IUnknown {
    fn from(value: IUserDataAccountProviderOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IUserDataAccountProviderOperation> for ::windows::runtime::IUnknown {
    fn from(value: &IUserDataAccountProviderOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUserDataAccountProviderOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IUserDataAccountProviderOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IUserDataAccountProviderOperation> for ::windows::runtime::IInspectable {
    fn from(value: IUserDataAccountProviderOperation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IUserDataAccountProviderOperation> for ::windows::runtime::IInspectable {
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IUserDataAccountProviderResolveErrorsOperation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccountProviderResolveErrorsOperation {
    type Vtable = IUserDataAccountProviderResolveErrorsOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1647696917, 49099, 16865, [153, 87, 151, 89, 162, 136, 70, 204]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IUserDataAccountProviderSettingsOperation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccountProviderSettingsOperation {
    type Vtable = IUserDataAccountProviderSettingsOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2449690039, 34376, 20272, [172, 250, 48, 2, 101, 140, 168, 13]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UserDataAccountPartnerAccountInfo(::windows::runtime::IInspectable);
impl UserDataAccountPartnerAccountInfo {
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Priority(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn AccountKind(&self) -> ::windows::runtime::Result<UserDataAccountProviderPartnerAccountKind> {
        let this = self;
        unsafe {
            let mut result__: UserDataAccountProviderPartnerAccountKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAccountProviderPartnerAccountKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataAccountPartnerAccountInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountPartnerAccountInfo;{5f200037-f6ef-4ec3-8630-012c59c1149f})");
}
unsafe impl ::windows::runtime::Interface for UserDataAccountPartnerAccountInfo {
    type Vtable = IUserDataAccountPartnerAccountInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1595932727, 63215, 20163, [134, 48, 1, 44, 89, 193, 20, 159]);
}
impl ::windows::runtime::RuntimeName for UserDataAccountPartnerAccountInfo {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountPartnerAccountInfo";
}
impl ::std::convert::From<UserDataAccountPartnerAccountInfo> for ::windows::runtime::IUnknown {
    fn from(value: UserDataAccountPartnerAccountInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UserDataAccountPartnerAccountInfo> for ::windows::runtime::IUnknown {
    fn from(value: &UserDataAccountPartnerAccountInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserDataAccountPartnerAccountInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UserDataAccountPartnerAccountInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UserDataAccountPartnerAccountInfo> for ::windows::runtime::IInspectable {
    fn from(value: UserDataAccountPartnerAccountInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UserDataAccountPartnerAccountInfo> for ::windows::runtime::IInspectable {
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
unsafe impl ::std::marker::Send for UserDataAccountPartnerAccountInfo {}
unsafe impl ::std::marker::Sync for UserDataAccountPartnerAccountInfo {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UserDataAccountProviderAddAccountOperation(::windows::runtime::IInspectable);
impl UserDataAccountProviderAddAccountOperation {
    pub fn ContentKinds(&self) -> ::windows::runtime::Result<super::UserDataAccountContentKinds> {
        let this = self;
        unsafe {
            let mut result__: super::UserDataAccountContentKinds = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::UserDataAccountContentKinds>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn PartnerAccountInfos(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<UserDataAccountPartnerAccountInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<UserDataAccountPartnerAccountInfo>>(result__)
        }
    }
    pub fn ReportCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, userdataaccountid: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), userdataaccountid.into_param().abi()).ok() }
    }
    pub fn Kind(&self) -> ::windows::runtime::Result<UserDataAccountProviderOperationKind> {
        let this = &::windows::runtime::Interface::cast::<IUserDataAccountProviderOperation>(self)?;
        unsafe {
            let mut result__: UserDataAccountProviderOperationKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAccountProviderOperationKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataAccountProviderAddAccountOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderAddAccountOperation;{b9c72530-3f84-4b5d-8eaa-45e97aa842ed})");
}
unsafe impl ::windows::runtime::Interface for UserDataAccountProviderAddAccountOperation {
    type Vtable = IUserDataAccountProviderAddAccountOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3116836144, 16260, 19293, [142, 170, 69, 233, 122, 168, 66, 237]);
}
impl ::windows::runtime::RuntimeName for UserDataAccountProviderAddAccountOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderAddAccountOperation";
}
impl ::std::convert::From<UserDataAccountProviderAddAccountOperation> for ::windows::runtime::IUnknown {
    fn from(value: UserDataAccountProviderAddAccountOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UserDataAccountProviderAddAccountOperation> for ::windows::runtime::IUnknown {
    fn from(value: &UserDataAccountProviderAddAccountOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserDataAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UserDataAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UserDataAccountProviderAddAccountOperation> for ::windows::runtime::IInspectable {
    fn from(value: UserDataAccountProviderAddAccountOperation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UserDataAccountProviderAddAccountOperation> for ::windows::runtime::IInspectable {
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
impl ::std::convert::TryFrom<UserDataAccountProviderAddAccountOperation> for IUserDataAccountProviderOperation {
    type Error = ::windows::runtime::Error;
    fn try_from(value: UserDataAccountProviderAddAccountOperation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&UserDataAccountProviderAddAccountOperation> for IUserDataAccountProviderOperation {
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
        ::std::convert::TryInto::<IUserDataAccountProviderOperation>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for UserDataAccountProviderAddAccountOperation {}
unsafe impl ::std::marker::Sync for UserDataAccountProviderAddAccountOperation {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataAccountProviderOperationKind(pub i32);
impl UserDataAccountProviderOperationKind {
    pub const AddAccount: UserDataAccountProviderOperationKind = UserDataAccountProviderOperationKind(0i32);
    pub const Settings: UserDataAccountProviderOperationKind = UserDataAccountProviderOperationKind(1i32);
    pub const ResolveErrors: UserDataAccountProviderOperationKind = UserDataAccountProviderOperationKind(2i32);
}
impl ::std::convert::From<i32> for UserDataAccountProviderOperationKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataAccountProviderOperationKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataAccountProviderOperationKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderOperationKind;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataAccountProviderPartnerAccountKind(pub i32);
impl UserDataAccountProviderPartnerAccountKind {
    pub const Exchange: UserDataAccountProviderPartnerAccountKind = UserDataAccountProviderPartnerAccountKind(0i32);
    pub const PopOrImap: UserDataAccountProviderPartnerAccountKind = UserDataAccountProviderPartnerAccountKind(1i32);
}
impl ::std::convert::From<i32> for UserDataAccountProviderPartnerAccountKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataAccountProviderPartnerAccountKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataAccountProviderPartnerAccountKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderPartnerAccountKind;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UserDataAccountProviderResolveErrorsOperation(::windows::runtime::IInspectable);
impl UserDataAccountProviderResolveErrorsOperation {
    pub fn UserDataAccountId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn ReportCompleted(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows::runtime::Result<UserDataAccountProviderOperationKind> {
        let this = &::windows::runtime::Interface::cast::<IUserDataAccountProviderOperation>(self)?;
        unsafe {
            let mut result__: UserDataAccountProviderOperationKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAccountProviderOperationKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataAccountProviderResolveErrorsOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderResolveErrorsOperation;{6235dc15-bfcb-41e1-9957-9759a28846cc})");
}
unsafe impl ::windows::runtime::Interface for UserDataAccountProviderResolveErrorsOperation {
    type Vtable = IUserDataAccountProviderResolveErrorsOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1647696917, 49099, 16865, [153, 87, 151, 89, 162, 136, 70, 204]);
}
impl ::windows::runtime::RuntimeName for UserDataAccountProviderResolveErrorsOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderResolveErrorsOperation";
}
impl ::std::convert::From<UserDataAccountProviderResolveErrorsOperation> for ::windows::runtime::IUnknown {
    fn from(value: UserDataAccountProviderResolveErrorsOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UserDataAccountProviderResolveErrorsOperation> for ::windows::runtime::IUnknown {
    fn from(value: &UserDataAccountProviderResolveErrorsOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserDataAccountProviderResolveErrorsOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UserDataAccountProviderResolveErrorsOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UserDataAccountProviderResolveErrorsOperation> for ::windows::runtime::IInspectable {
    fn from(value: UserDataAccountProviderResolveErrorsOperation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UserDataAccountProviderResolveErrorsOperation> for ::windows::runtime::IInspectable {
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
impl ::std::convert::TryFrom<UserDataAccountProviderResolveErrorsOperation> for IUserDataAccountProviderOperation {
    type Error = ::windows::runtime::Error;
    fn try_from(value: UserDataAccountProviderResolveErrorsOperation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&UserDataAccountProviderResolveErrorsOperation> for IUserDataAccountProviderOperation {
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
        ::std::convert::TryInto::<IUserDataAccountProviderOperation>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for UserDataAccountProviderResolveErrorsOperation {}
unsafe impl ::std::marker::Sync for UserDataAccountProviderResolveErrorsOperation {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UserDataAccountProviderSettingsOperation(::windows::runtime::IInspectable);
impl UserDataAccountProviderSettingsOperation {
    pub fn UserDataAccountId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn ReportCompleted(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows::runtime::Result<UserDataAccountProviderOperationKind> {
        let this = &::windows::runtime::Interface::cast::<IUserDataAccountProviderOperation>(self)?;
        unsafe {
            let mut result__: UserDataAccountProviderOperationKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAccountProviderOperationKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataAccountProviderSettingsOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderSettingsOperation;{92034db7-8648-4f30-acfa-3002658ca80d})");
}
unsafe impl ::windows::runtime::Interface for UserDataAccountProviderSettingsOperation {
    type Vtable = IUserDataAccountProviderSettingsOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2449690039, 34376, 20272, [172, 250, 48, 2, 101, 140, 168, 13]);
}
impl ::windows::runtime::RuntimeName for UserDataAccountProviderSettingsOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderSettingsOperation";
}
impl ::std::convert::From<UserDataAccountProviderSettingsOperation> for ::windows::runtime::IUnknown {
    fn from(value: UserDataAccountProviderSettingsOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UserDataAccountProviderSettingsOperation> for ::windows::runtime::IUnknown {
    fn from(value: &UserDataAccountProviderSettingsOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserDataAccountProviderSettingsOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UserDataAccountProviderSettingsOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UserDataAccountProviderSettingsOperation> for ::windows::runtime::IInspectable {
    fn from(value: UserDataAccountProviderSettingsOperation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UserDataAccountProviderSettingsOperation> for ::windows::runtime::IInspectable {
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
impl ::std::convert::TryFrom<UserDataAccountProviderSettingsOperation> for IUserDataAccountProviderOperation {
    type Error = ::windows::runtime::Error;
    fn try_from(value: UserDataAccountProviderSettingsOperation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&UserDataAccountProviderSettingsOperation> for IUserDataAccountProviderOperation {
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
        ::std::convert::TryInto::<IUserDataAccountProviderOperation>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for UserDataAccountProviderSettingsOperation {}
unsafe impl ::std::marker::Sync for UserDataAccountProviderSettingsOperation {}
