#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountPartnerAccountInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataAccountPartnerAccountInfo {
    type Vtable = IUserDataAccountPartnerAccountInfo_Vtbl;
}
impl ::core::clone::Clone for IUserDataAccountPartnerAccountInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUserDataAccountPartnerAccountInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f200037_f6ef_4ec3_8630_012c59c1149f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountPartnerAccountInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub AccountKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataAccountProviderPartnerAccountKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountProviderAddAccountOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataAccountProviderAddAccountOperation {
    type Vtable = IUserDataAccountProviderAddAccountOperation_Vtbl;
}
impl ::core::clone::Clone for IUserDataAccountProviderAddAccountOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUserDataAccountProviderAddAccountOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb9c72530_3f84_4b5d_8eaa_45e97aa842ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderAddAccountOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ContentKinds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::UserDataAccountContentKinds) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PartnerAccountInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PartnerAccountInfos: usize,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdataaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
#[repr(transparent)]
pub struct IUserDataAccountProviderOperation(::windows_core::IUnknown);
impl IUserDataAccountProviderOperation {
    pub fn Kind(&self) -> ::windows_core::Result<UserDataAccountProviderOperationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IUserDataAccountProviderOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for IUserDataAccountProviderOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{a20aad63-888c-4a62-a3dd-34d07a802b2b}");
}
unsafe impl ::windows_core::Interface for IUserDataAccountProviderOperation {
    type Vtable = IUserDataAccountProviderOperation_Vtbl;
}
impl ::core::clone::Clone for IUserDataAccountProviderOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUserDataAccountProviderOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa20aad63_888c_4a62_a3dd_34d07a802b2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataAccountProviderOperationKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountProviderResolveErrorsOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataAccountProviderResolveErrorsOperation {
    type Vtable = IUserDataAccountProviderResolveErrorsOperation_Vtbl;
}
impl ::core::clone::Clone for IUserDataAccountProviderResolveErrorsOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUserDataAccountProviderResolveErrorsOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6235dc15_bfcb_41e1_9957_9759a28846cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderResolveErrorsOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountProviderSettingsOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserDataAccountProviderSettingsOperation {
    type Vtable = IUserDataAccountProviderSettingsOperation_Vtbl;
}
impl ::core::clone::Clone for IUserDataAccountProviderSettingsOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IUserDataAccountProviderSettingsOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92034db7_8648_4f30_acfa_3002658ca80d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderSettingsOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
#[repr(transparent)]
pub struct UserDataAccountPartnerAccountInfo(::windows_core::IUnknown);
impl UserDataAccountPartnerAccountInfo {
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Priority(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Priority)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AccountKind(&self) -> ::windows_core::Result<UserDataAccountProviderPartnerAccountKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccountKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for UserDataAccountPartnerAccountInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountPartnerAccountInfo;{5f200037-f6ef-4ec3-8630-012c59c1149f})");
}
impl ::core::clone::Clone for UserDataAccountPartnerAccountInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for UserDataAccountPartnerAccountInfo {
    type Vtable = IUserDataAccountPartnerAccountInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UserDataAccountPartnerAccountInfo {
    const IID: ::windows_core::GUID = <IUserDataAccountPartnerAccountInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UserDataAccountPartnerAccountInfo {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountPartnerAccountInfo";
}
::windows_core::imp::interface_hierarchy!(UserDataAccountPartnerAccountInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for UserDataAccountPartnerAccountInfo {}
unsafe impl ::core::marker::Sync for UserDataAccountPartnerAccountInfo {}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
#[repr(transparent)]
pub struct UserDataAccountProviderAddAccountOperation(::windows_core::IUnknown);
impl UserDataAccountProviderAddAccountOperation {
    pub fn ContentKinds(&self) -> ::windows_core::Result<super::UserDataAccountContentKinds> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentKinds)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PartnerAccountInfos(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<UserDataAccountPartnerAccountInfo>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PartnerAccountInfos)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportCompleted(&self, userdataaccountid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCompleted)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(userdataaccountid)).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<UserDataAccountProviderOperationKind> {
        let this = &::windows_core::ComInterface::cast::<IUserDataAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for UserDataAccountProviderAddAccountOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderAddAccountOperation;{b9c72530-3f84-4b5d-8eaa-45e97aa842ed})");
}
impl ::core::clone::Clone for UserDataAccountProviderAddAccountOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for UserDataAccountProviderAddAccountOperation {
    type Vtable = IUserDataAccountProviderAddAccountOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UserDataAccountProviderAddAccountOperation {
    const IID: ::windows_core::GUID = <IUserDataAccountProviderAddAccountOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UserDataAccountProviderAddAccountOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderAddAccountOperation";
}
::windows_core::imp::interface_hierarchy!(UserDataAccountProviderAddAccountOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IUserDataAccountProviderOperation> for UserDataAccountProviderAddAccountOperation {}
unsafe impl ::core::marker::Send for UserDataAccountProviderAddAccountOperation {}
unsafe impl ::core::marker::Sync for UserDataAccountProviderAddAccountOperation {}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
#[repr(transparent)]
pub struct UserDataAccountProviderResolveErrorsOperation(::windows_core::IUnknown);
impl UserDataAccountProviderResolveErrorsOperation {
    pub fn Kind(&self) -> ::windows_core::Result<UserDataAccountProviderOperationKind> {
        let this = &::windows_core::ComInterface::cast::<IUserDataAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UserDataAccountId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserDataAccountId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportCompleted(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCompleted)(::windows_core::Interface::as_raw(this)).ok() }
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
impl ::windows_core::RuntimeType for UserDataAccountProviderResolveErrorsOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderResolveErrorsOperation;{6235dc15-bfcb-41e1-9957-9759a28846cc})");
}
impl ::core::clone::Clone for UserDataAccountProviderResolveErrorsOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for UserDataAccountProviderResolveErrorsOperation {
    type Vtable = IUserDataAccountProviderResolveErrorsOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UserDataAccountProviderResolveErrorsOperation {
    const IID: ::windows_core::GUID = <IUserDataAccountProviderResolveErrorsOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UserDataAccountProviderResolveErrorsOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderResolveErrorsOperation";
}
::windows_core::imp::interface_hierarchy!(UserDataAccountProviderResolveErrorsOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IUserDataAccountProviderOperation> for UserDataAccountProviderResolveErrorsOperation {}
unsafe impl ::core::marker::Send for UserDataAccountProviderResolveErrorsOperation {}
unsafe impl ::core::marker::Sync for UserDataAccountProviderResolveErrorsOperation {}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
#[repr(transparent)]
pub struct UserDataAccountProviderSettingsOperation(::windows_core::IUnknown);
impl UserDataAccountProviderSettingsOperation {
    pub fn Kind(&self) -> ::windows_core::Result<UserDataAccountProviderOperationKind> {
        let this = &::windows_core::ComInterface::cast::<IUserDataAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UserDataAccountId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserDataAccountId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportCompleted(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCompleted)(::windows_core::Interface::as_raw(this)).ok() }
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
impl ::windows_core::RuntimeType for UserDataAccountProviderSettingsOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderSettingsOperation;{92034db7-8648-4f30-acfa-3002658ca80d})");
}
impl ::core::clone::Clone for UserDataAccountProviderSettingsOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for UserDataAccountProviderSettingsOperation {
    type Vtable = IUserDataAccountProviderSettingsOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UserDataAccountProviderSettingsOperation {
    const IID: ::windows_core::GUID = <IUserDataAccountProviderSettingsOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UserDataAccountProviderSettingsOperation {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderSettingsOperation";
}
::windows_core::imp::interface_hierarchy!(UserDataAccountProviderSettingsOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IUserDataAccountProviderOperation> for UserDataAccountProviderSettingsOperation {}
unsafe impl ::core::marker::Send for UserDataAccountProviderSettingsOperation {}
unsafe impl ::core::marker::Sync for UserDataAccountProviderSettingsOperation {}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows_core::TypeKind for UserDataAccountProviderOperationKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UserDataAccountProviderOperationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountProviderOperationKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UserDataAccountProviderOperationKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderOperationKind;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows_core::TypeKind for UserDataAccountProviderPartnerAccountKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UserDataAccountProviderPartnerAccountKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountProviderPartnerAccountKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for UserDataAccountProviderPartnerAccountKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderPartnerAccountKind;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
