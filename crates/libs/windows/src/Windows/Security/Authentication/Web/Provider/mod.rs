#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountClientView(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountClientView {
    type Vtable = IWebAccountClientView_Vtbl;
}
impl ::core::clone::Clone for IWebAccountClientView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountClientView {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe7bd66ba_0bc7_4c66_bfd4_65d3082cbca8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountClientView_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ApplicationCallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplicationCallbackUri: usize,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebAccountClientViewType) -> ::windows_core::HRESULT,
    pub AccountPairwiseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountClientViewFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountClientViewFactory {
    type Vtable = IWebAccountClientViewFactory_Vtbl;
}
impl ::core::clone::Clone for IWebAccountClientViewFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountClientViewFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x616d16a4_de22_4855_a326_06cebf2a3f23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountClientViewFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewtype: WebAccountClientViewType, applicationcallbackuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
    #[cfg(feature = "Foundation")]
    pub CreateWithPairwiseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewtype: WebAccountClientViewType, applicationcallbackuri: *mut ::core::ffi::c_void, accountpairwiseid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWithPairwiseId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountManagerStatics {
    type Vtable = IWebAccountManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IWebAccountManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2e8e1a6_d49a_4032_84bf_1a2847747bf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub UpdateWebAccountPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, webaccountusername: ::std::mem::MaybeUninit<::windows_core::HSTRING>, additionalproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    UpdateWebAccountPropertiesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub AddWebAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, webaccountusername: ::std::mem::MaybeUninit<::windows_core::HSTRING>, props: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    AddWebAccountAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub DeleteWebAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    DeleteWebAccountAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub FindAllProviderWebAccountsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    FindAllProviderWebAccountsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Http"))]
    pub PushCookiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, cookies: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Web_Http")))]
    PushCookiesAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub SetViewAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    SetViewAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub ClearViewAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, applicationcallbackuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    ClearViewAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub GetViewsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    GetViewsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "Storage_Streams"))]
    pub SetWebAccountPictureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, webaccountpicture: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials", feature = "Storage_Streams")))]
    SetWebAccountPictureAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub ClearWebAccountPictureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    ClearWebAccountPictureAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountManagerStatics2 {
    type Vtable = IWebAccountManagerStatics2_Vtbl;
}
impl ::core::clone::Clone for IWebAccountManagerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountManagerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68a7a829_2d5f_4653_8bb0_bd2fa6bd2d87);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PullCookiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uristring: ::std::mem::MaybeUninit<::windows_core::HSTRING>, callerpfn: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PullCookiesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountManagerStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountManagerStatics3 {
    type Vtable = IWebAccountManagerStatics3_Vtbl;
}
impl ::core::clone::Clone for IWebAccountManagerStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountManagerStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdd4523a6_8a4f_4aa2_b15e_03f550af1359);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountManagerStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub FindAllProviderWebAccountsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System")))]
    FindAllProviderWebAccountsForUserAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub AddWebAccountForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, webaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, webaccountusername: ::std::mem::MaybeUninit<::windows_core::HSTRING>, props: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System")))]
    AddWebAccountForUserAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub AddWebAccountWithScopeForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, webaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, webaccountusername: ::std::mem::MaybeUninit<::windows_core::HSTRING>, props: *mut ::core::ffi::c_void, scope: WebAccountScope, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System")))]
    AddWebAccountWithScopeForUserAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub AddWebAccountWithScopeAndMapForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, webaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, webaccountusername: ::std::mem::MaybeUninit<::windows_core::HSTRING>, props: *mut ::core::ffi::c_void, scope: WebAccountScope, peruserwebaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System")))]
    AddWebAccountWithScopeAndMapForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountManagerStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountManagerStatics4 {
    type Vtable = IWebAccountManagerStatics4_Vtbl;
}
impl ::core::clone::Clone for IWebAccountManagerStatics4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountManagerStatics4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59ebc2d2_f7db_412f_bc3f_f2fea04430b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountManagerStatics4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub InvalidateAppCacheForAllAccountsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InvalidateAppCacheForAllAccountsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub InvalidateAppCacheForAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    InvalidateAppCacheForAccountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountMapManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountMapManagerStatics {
    type Vtable = IWebAccountMapManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IWebAccountMapManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountMapManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8fa446f_3a1b_48a4_8e90_1e59ca6f54db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountMapManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub AddWebAccountWithScopeAndMapAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, webaccountusername: ::std::mem::MaybeUninit<::windows_core::HSTRING>, props: *mut ::core::ffi::c_void, scope: WebAccountScope, peruserwebaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    AddWebAccountWithScopeAndMapAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub SetPerAppToPerUserAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, perappaccount: *mut ::core::ffi::c_void, peruserwebaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    SetPerAppToPerUserAccountAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub GetPerUserFromPerAppAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, perappaccount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    GetPerUserFromPerAppAccountAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub ClearPerUserFromPerAppAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, perappaccount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    ClearPerUserFromPerAppAccountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProviderAddAccountOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountProviderAddAccountOperation {
    type Vtable = IWebAccountProviderAddAccountOperation_Vtbl;
}
impl ::core::clone::Clone for IWebAccountProviderAddAccountOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountProviderAddAccountOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73ebdccf_4378_4c79_9335_a5d7ab81594e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderAddAccountOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct IWebAccountProviderBaseReportOperation(::windows_core::IUnknown);
impl IWebAccountProviderBaseReportOperation {
    pub fn ReportCompleted(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCompleted)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Core::WebProviderError>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportError)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IWebAccountProviderBaseReportOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IWebAccountProviderBaseReportOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAccountProviderBaseReportOperation {}
impl ::core::fmt::Debug for IWebAccountProviderBaseReportOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebAccountProviderBaseReportOperation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IWebAccountProviderBaseReportOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{bba4acbb-993b-4d57-bbe4-1421e3668b4c}");
}
unsafe impl ::windows_core::Interface for IWebAccountProviderBaseReportOperation {
    type Vtable = IWebAccountProviderBaseReportOperation_Vtbl;
}
impl ::core::clone::Clone for IWebAccountProviderBaseReportOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountProviderBaseReportOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbba4acbb_993b_4d57_bbe4_1421e3668b4c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderBaseReportOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub ReportError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Core"))]
    ReportError: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProviderDeleteAccountOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountProviderDeleteAccountOperation {
    type Vtable = IWebAccountProviderDeleteAccountOperation_Vtbl;
}
impl ::core::clone::Clone for IWebAccountProviderDeleteAccountOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountProviderDeleteAccountOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0abb48b8_9e01_49c9_a355_7d48caf7d6ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderDeleteAccountOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccount: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProviderManageAccountOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountProviderManageAccountOperation {
    type Vtable = IWebAccountProviderManageAccountOperation_Vtbl;
}
impl ::core::clone::Clone for IWebAccountProviderManageAccountOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountProviderManageAccountOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed20dc5c_d21b_463e_a9b7_c1fd0edae978);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderManageAccountOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccount: usize,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct IWebAccountProviderOperation(::windows_core::IUnknown);
impl IWebAccountProviderOperation {
    pub fn Kind(&self) -> ::windows_core::Result<WebAccountProviderOperationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IWebAccountProviderOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IWebAccountProviderOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAccountProviderOperation {}
impl ::core::fmt::Debug for IWebAccountProviderOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebAccountProviderOperation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IWebAccountProviderOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{6d5d2426-10b1-419a-a44e-f9c5161574e6}");
}
unsafe impl ::windows_core::Interface for IWebAccountProviderOperation {
    type Vtable = IWebAccountProviderOperation_Vtbl;
}
impl ::core::clone::Clone for IWebAccountProviderOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountProviderOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6d5d2426_10b1_419a_a44e_f9c5161574e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebAccountProviderOperationKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProviderRetrieveCookiesOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountProviderRetrieveCookiesOperation {
    type Vtable = IWebAccountProviderRetrieveCookiesOperation_Vtbl;
}
impl ::core::clone::Clone for IWebAccountProviderRetrieveCookiesOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountProviderRetrieveCookiesOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5a040441_0fa3_4ab1_a01c_20b110358594);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderRetrieveCookiesOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Context: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Http"))]
    pub Cookies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Web_Http")))]
    Cookies: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub ApplicationCallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplicationCallbackUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProviderSignOutAccountOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountProviderSignOutAccountOperation {
    type Vtable = IWebAccountProviderSignOutAccountOperation_Vtbl;
}
impl ::core::clone::Clone for IWebAccountProviderSignOutAccountOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountProviderSignOutAccountOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb890e21d_0c55_47bc_8c72_04a6fc7cac07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderSignOutAccountOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccount: usize,
    #[cfg(feature = "Foundation")]
    pub ApplicationCallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplicationCallbackUri: usize,
    pub ClientId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct IWebAccountProviderSilentReportOperation(::windows_core::IUnknown);
impl IWebAccountProviderSilentReportOperation {
    pub fn ReportUserInteractionRequired(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportUserInteractionRequired)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportUserInteractionRequiredWithError<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Core::WebProviderError>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportUserInteractionRequiredWithError)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ReportCompleted(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReportCompleted)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Core::WebProviderError>,
    {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReportError)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IWebAccountProviderSilentReportOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IWebAccountProviderBaseReportOperation> for IWebAccountProviderSilentReportOperation {}
impl ::core::cmp::PartialEq for IWebAccountProviderSilentReportOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAccountProviderSilentReportOperation {}
impl ::core::fmt::Debug for IWebAccountProviderSilentReportOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebAccountProviderSilentReportOperation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IWebAccountProviderSilentReportOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{e0b545f8-3b0f-44da-924c-7b18baaa62a9}");
}
unsafe impl ::windows_core::Interface for IWebAccountProviderSilentReportOperation {
    type Vtable = IWebAccountProviderSilentReportOperation_Vtbl;
}
impl ::core::clone::Clone for IWebAccountProviderSilentReportOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountProviderSilentReportOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0b545f8_3b0f_44da_924c_7b18baaa62a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderSilentReportOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ReportUserInteractionRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub ReportUserInteractionRequiredWithError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Core"))]
    ReportUserInteractionRequiredWithError: usize,
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct IWebAccountProviderTokenObjects(::windows_core::IUnknown);
impl IWebAccountProviderTokenObjects {
    pub fn Operation(&self) -> ::windows_core::Result<IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IWebAccountProviderTokenObjects, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IWebAccountProviderTokenObjects {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAccountProviderTokenObjects {}
impl ::core::fmt::Debug for IWebAccountProviderTokenObjects {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebAccountProviderTokenObjects").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IWebAccountProviderTokenObjects {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{408f284b-1328-42db-89a4-0bce7a717d8e}");
}
unsafe impl ::windows_core::Interface for IWebAccountProviderTokenObjects {
    type Vtable = IWebAccountProviderTokenObjects_Vtbl;
}
impl ::core::clone::Clone for IWebAccountProviderTokenObjects {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountProviderTokenObjects {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x408f284b_1328_42db_89a4_0bce7a717d8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderTokenObjects_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct IWebAccountProviderTokenObjects2(::windows_core::IUnknown);
impl IWebAccountProviderTokenObjects2 {
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Operation(&self) -> ::windows_core::Result<IWebAccountProviderOperation> {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderTokenObjects>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IWebAccountProviderTokenObjects2, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IWebAccountProviderTokenObjects> for IWebAccountProviderTokenObjects2 {}
impl ::core::cmp::PartialEq for IWebAccountProviderTokenObjects2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAccountProviderTokenObjects2 {}
impl ::core::fmt::Debug for IWebAccountProviderTokenObjects2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebAccountProviderTokenObjects2").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IWebAccountProviderTokenObjects2 {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{1020b893-5ca5-4fff-95fb-b820273fc395}");
}
unsafe impl ::windows_core::Interface for IWebAccountProviderTokenObjects2 {
    type Vtable = IWebAccountProviderTokenObjects2_Vtbl;
}
impl ::core::clone::Clone for IWebAccountProviderTokenObjects2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountProviderTokenObjects2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1020b893_5ca5_4fff_95fb_b820273fc395);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderTokenObjects2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct IWebAccountProviderTokenOperation(::windows_core::IUnknown);
impl IWebAccountProviderTokenOperation {
    pub fn ProviderRequest(&self) -> ::windows_core::Result<WebProviderTokenRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProviderResponses(&self) -> ::windows_core::Result<super::super::super::super::Foundation::Collections::IVector<WebProviderTokenResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderResponses)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCacheExpirationTime(&self, value: super::super::super::super::Foundation::DateTime) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCacheExpirationTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CacheExpirationTime(&self) -> ::windows_core::Result<super::super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CacheExpirationTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<WebAccountProviderOperationKind> {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IWebAccountProviderTokenOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IWebAccountProviderOperation> for IWebAccountProviderTokenOperation {}
impl ::core::cmp::PartialEq for IWebAccountProviderTokenOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAccountProviderTokenOperation {}
impl ::core::fmt::Debug for IWebAccountProviderTokenOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebAccountProviderTokenOperation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IWebAccountProviderTokenOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{95c613be-2034-4c38-9434-d26c14b2b4b2}");
}
unsafe impl ::windows_core::Interface for IWebAccountProviderTokenOperation {
    type Vtable = IWebAccountProviderTokenOperation_Vtbl;
}
impl ::core::clone::Clone for IWebAccountProviderTokenOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountProviderTokenOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95c613be_2034_4c38_9434_d26c14b2b4b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderTokenOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProviderRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ProviderResponses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProviderResponses: usize,
    #[cfg(feature = "Foundation")]
    pub SetCacheExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCacheExpirationTime: usize,
    #[cfg(feature = "Foundation")]
    pub CacheExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CacheExpirationTime: usize,
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct IWebAccountProviderUIReportOperation(::windows_core::IUnknown);
impl IWebAccountProviderUIReportOperation {
    pub fn ReportUserCanceled(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportUserCanceled)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportCompleted(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReportCompleted)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Core::WebProviderError>,
    {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReportError)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IWebAccountProviderUIReportOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IWebAccountProviderBaseReportOperation> for IWebAccountProviderUIReportOperation {}
impl ::core::cmp::PartialEq for IWebAccountProviderUIReportOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAccountProviderUIReportOperation {}
impl ::core::fmt::Debug for IWebAccountProviderUIReportOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebAccountProviderUIReportOperation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IWebAccountProviderUIReportOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{28ff92d3-8f80-42fb-944f-b2107bbd42e6}");
}
unsafe impl ::windows_core::Interface for IWebAccountProviderUIReportOperation {
    type Vtable = IWebAccountProviderUIReportOperation_Vtbl;
}
impl ::core::clone::Clone for IWebAccountProviderUIReportOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountProviderUIReportOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x28ff92d3_8f80_42fb_944f_b2107bbd42e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderUIReportOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ReportUserCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountScopeManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountScopeManagerStatics {
    type Vtable = IWebAccountScopeManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IWebAccountScopeManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountScopeManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c6ce37c_12b2_423a_bf3d_85b8d7e53656);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountScopeManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub AddWebAccountWithScopeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, webaccountusername: ::std::mem::MaybeUninit<::windows_core::HSTRING>, props: *mut ::core::ffi::c_void, scope: WebAccountScope, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    AddWebAccountWithScopeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub SetScopeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, scope: WebAccountScope, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    SetScopeAsync: usize,
    #[cfg(feature = "Security_Credentials")]
    pub GetScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, result__: *mut WebAccountScope) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    GetScope: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebProviderTokenRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebProviderTokenRequest {
    type Vtable = IWebProviderTokenRequest_Vtbl;
}
impl ::core::clone::Clone for IWebProviderTokenRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebProviderTokenRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1e18778b_8805_454b_9f11_468d2af1095a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebProviderTokenRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub ClientRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Core"))]
    ClientRequest: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub WebAccounts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    WebAccounts: usize,
    pub WebAccountSelectionOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebAccountSelectionOptions) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ApplicationCallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplicationCallbackUri: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Core"))]
    pub GetApplicationTokenBindingKeyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keytype: super::TokenBindingKeyType, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Cryptography_Core")))]
    GetApplicationTokenBindingKeyAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebProviderTokenRequest2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebProviderTokenRequest2 {
    type Vtable = IWebProviderTokenRequest2_Vtbl;
}
impl ::core::clone::Clone for IWebProviderTokenRequest2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebProviderTokenRequest2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb5d72e4c_10b1_4aa6_88b1_0b6c9e0c1e46);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebProviderTokenRequest2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetApplicationTokenBindingKeyIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keytype: super::TokenBindingKeyType, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetApplicationTokenBindingKeyIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebProviderTokenRequest3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebProviderTokenRequest3 {
    type Vtable = IWebProviderTokenRequest3_Vtbl;
}
impl ::core::clone::Clone for IWebProviderTokenRequest3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebProviderTokenRequest3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b2716aa_4289_446e_9256_dafb6f66a51e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebProviderTokenRequest3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ApplicationPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ApplicationProcessName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CheckApplicationForCapabilityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilityname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckApplicationForCapabilityAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebProviderTokenResponse(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebProviderTokenResponse {
    type Vtable = IWebProviderTokenResponse_Vtbl;
}
impl ::core::clone::Clone for IWebProviderTokenResponse {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebProviderTokenResponse {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef213793_ef55_4186_b7ce_8cb2e7f9849e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebProviderTokenResponse_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub ClientResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Core"))]
    ClientResponse: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebProviderTokenResponseFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebProviderTokenResponseFactory {
    type Vtable = IWebProviderTokenResponseFactory_Vtbl;
}
impl ::core::clone::Clone for IWebProviderTokenResponseFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebProviderTokenResponseFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfa49d99a_25ba_4077_9cfa_9db4dea7b71a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebProviderTokenResponseFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webtokenresponse: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Core"))]
    Create: usize,
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebAccountClientView(::windows_core::IUnknown);
impl WebAccountClientView {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ApplicationCallbackUri(&self) -> ::windows_core::Result<super::super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ApplicationCallbackUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<WebAccountClientViewType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AccountPairwiseId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AccountPairwiseId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Create<P0>(viewtype: WebAccountClientViewType, applicationcallbackuri: P0) -> ::windows_core::Result<WebAccountClientView>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Foundation::Uri>,
    {
        Self::IWebAccountClientViewFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), viewtype, applicationcallbackuri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWithPairwiseId<P0>(viewtype: WebAccountClientViewType, applicationcallbackuri: P0, accountpairwiseid: &::windows_core::HSTRING) -> ::windows_core::Result<WebAccountClientView>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Foundation::Uri>,
    {
        Self::IWebAccountClientViewFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithPairwiseId)(::windows_core::Interface::as_raw(this), viewtype, applicationcallbackuri.into_param().abi(), ::core::mem::transmute_copy(accountpairwiseid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebAccountClientViewFactory<R, F: FnOnce(&IWebAccountClientViewFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WebAccountClientView, IWebAccountClientViewFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for WebAccountClientView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountClientView {}
impl ::core::fmt::Debug for WebAccountClientView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountClientView").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WebAccountClientView {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Provider.WebAccountClientView;{e7bd66ba-0bc7-4c66-bfd4-65d3082cbca8})");
}
impl ::core::clone::Clone for WebAccountClientView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WebAccountClientView {
    type Vtable = IWebAccountClientView_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebAccountClientView {
    const IID: ::windows_core::GUID = <IWebAccountClientView as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebAccountClientView {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountClientView";
}
::windows_core::imp::interface_hierarchy!(WebAccountClientView, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WebAccountClientView {}
unsafe impl ::core::marker::Sync for WebAccountClientView {}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
pub struct WebAccountManager;
impl WebAccountManager {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn UpdateWebAccountPropertiesAsync<P0, P1>(webaccount: P0, webaccountusername: &::windows_core::HSTRING, additionalproperties: P1) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::super::super::Credentials::WebAccount>,
        P1: ::windows_core::TryIntoParam<super::super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>,
    {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateWebAccountPropertiesAsync)(::windows_core::Interface::as_raw(this), webaccount.into_param().abi(), ::core::mem::transmute_copy(webaccountusername), additionalproperties.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn AddWebAccountAsync<P0>(webaccountid: &::windows_core::HSTRING, webaccountusername: &::windows_core::HSTRING, props: P0) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>,
    {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddWebAccountAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(webaccountid), ::core::mem::transmute_copy(webaccountusername), props.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn DeleteWebAccountAsync<P0>(webaccount: P0) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::super::super::Credentials::WebAccount>,
    {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeleteWebAccountAsync)(::windows_core::Interface::as_raw(this), webaccount.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn FindAllProviderWebAccountsAsync() -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>> {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllProviderWebAccountsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Web_Http\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Http"))]
    pub fn PushCookiesAsync<P0, P1>(uri: P0, cookies: P1) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Foundation::Uri>,
        P1: ::windows_core::TryIntoParam<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Web::Http::HttpCookie>>,
    {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PushCookiesAsync)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), cookies.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn SetViewAsync<P0, P1>(webaccount: P0, view: P1) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::super::super::Credentials::WebAccount>,
        P1: ::windows_core::IntoParam<WebAccountClientView>,
    {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetViewAsync)(::windows_core::Interface::as_raw(this), webaccount.into_param().abi(), view.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn ClearViewAsync<P0, P1>(webaccount: P0, applicationcallbackuri: P1) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::super::super::Credentials::WebAccount>,
        P1: ::windows_core::IntoParam<super::super::super::super::Foundation::Uri>,
    {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClearViewAsync)(::windows_core::Interface::as_raw(this), webaccount.into_param().abi(), applicationcallbackuri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn GetViewsAsync<P0>(webaccount: P0) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<WebAccountClientView>>>
    where
        P0: ::windows_core::IntoParam<super::super::super::Credentials::WebAccount>,
    {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetViewsAsync)(::windows_core::Interface::as_raw(this), webaccount.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "Storage_Streams"))]
    pub fn SetWebAccountPictureAsync<P0, P1>(webaccount: P0, webaccountpicture: P1) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::super::super::Credentials::WebAccount>,
        P1: ::windows_core::TryIntoParam<super::super::super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetWebAccountPictureAsync)(::windows_core::Interface::as_raw(this), webaccount.into_param().abi(), webaccountpicture.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn ClearWebAccountPictureAsync<P0>(webaccount: P0) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::super::super::Credentials::WebAccount>,
    {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClearWebAccountPictureAsync)(::windows_core::Interface::as_raw(this), webaccount.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PullCookiesAsync(uristring: &::windows_core::HSTRING, callerpfn: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::IWebAccountManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PullCookiesAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(uristring), ::core::mem::transmute_copy(callerpfn), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Credentials\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub fn FindAllProviderWebAccountsForUserAsync<P0>(user: P0) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::System::User>,
    {
        Self::IWebAccountManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllProviderWebAccountsForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Credentials\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub fn AddWebAccountForUserAsync<P0, P1>(user: P0, webaccountid: &::windows_core::HSTRING, webaccountusername: &::windows_core::HSTRING, props: P1) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::System::User>,
        P1: ::windows_core::TryIntoParam<super::super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>,
    {
        Self::IWebAccountManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddWebAccountForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(webaccountid), ::core::mem::transmute_copy(webaccountusername), props.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Credentials\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub fn AddWebAccountWithScopeForUserAsync<P0, P1>(user: P0, webaccountid: &::windows_core::HSTRING, webaccountusername: &::windows_core::HSTRING, props: P1, scope: WebAccountScope) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::System::User>,
        P1: ::windows_core::TryIntoParam<super::super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>,
    {
        Self::IWebAccountManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddWebAccountWithScopeForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(webaccountid), ::core::mem::transmute_copy(webaccountusername), props.try_into_param()?.abi(), scope, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Credentials\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub fn AddWebAccountWithScopeAndMapForUserAsync<P0, P1>(user: P0, webaccountid: &::windows_core::HSTRING, webaccountusername: &::windows_core::HSTRING, props: P1, scope: WebAccountScope, peruserwebaccountid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::System::User>,
        P1: ::windows_core::TryIntoParam<super::super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>,
    {
        Self::IWebAccountManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddWebAccountWithScopeAndMapForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(webaccountid), ::core::mem::transmute_copy(webaccountusername), props.try_into_param()?.abi(), scope, ::core::mem::transmute_copy(peruserwebaccountid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InvalidateAppCacheForAllAccountsAsync() -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::IWebAccountManagerStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InvalidateAppCacheForAllAccountsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn InvalidateAppCacheForAccountAsync<P0>(webaccount: P0) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::super::super::Credentials::WebAccount>,
    {
        Self::IWebAccountManagerStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InvalidateAppCacheForAccountAsync)(::windows_core::Interface::as_raw(this), webaccount.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn AddWebAccountWithScopeAndMapAsync<P0>(webaccountid: &::windows_core::HSTRING, webaccountusername: &::windows_core::HSTRING, props: P0, scope: WebAccountScope, peruserwebaccountid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>,
    {
        Self::IWebAccountMapManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddWebAccountWithScopeAndMapAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(webaccountid), ::core::mem::transmute_copy(webaccountusername), props.try_into_param()?.abi(), scope, ::core::mem::transmute_copy(peruserwebaccountid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn SetPerAppToPerUserAccountAsync<P0>(perappaccount: P0, peruserwebaccountid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::super::super::Credentials::WebAccount>,
    {
        Self::IWebAccountMapManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetPerAppToPerUserAccountAsync)(::windows_core::Interface::as_raw(this), perappaccount.into_param().abi(), ::core::mem::transmute_copy(peruserwebaccountid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn GetPerUserFromPerAppAccountAsync<P0>(perappaccount: P0) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>
    where
        P0: ::windows_core::IntoParam<super::super::super::Credentials::WebAccount>,
    {
        Self::IWebAccountMapManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPerUserFromPerAppAccountAsync)(::windows_core::Interface::as_raw(this), perappaccount.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn ClearPerUserFromPerAppAccountAsync<P0>(perappaccount: P0) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::super::super::Credentials::WebAccount>,
    {
        Self::IWebAccountMapManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClearPerUserFromPerAppAccountAsync)(::windows_core::Interface::as_raw(this), perappaccount.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn AddWebAccountWithScopeAsync<P0>(webaccountid: &::windows_core::HSTRING, webaccountusername: &::windows_core::HSTRING, props: P0, scope: WebAccountScope) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>,
    {
        Self::IWebAccountScopeManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddWebAccountWithScopeAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(webaccountid), ::core::mem::transmute_copy(webaccountusername), props.try_into_param()?.abi(), scope, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn SetScopeAsync<P0>(webaccount: P0, scope: WebAccountScope) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::super::super::Credentials::WebAccount>,
    {
        Self::IWebAccountScopeManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetScopeAsync)(::windows_core::Interface::as_raw(this), webaccount.into_param().abi(), scope, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn GetScope<P0>(webaccount: P0) -> ::windows_core::Result<WebAccountScope>
    where
        P0: ::windows_core::IntoParam<super::super::super::Credentials::WebAccount>,
    {
        Self::IWebAccountScopeManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetScope)(::windows_core::Interface::as_raw(this), webaccount.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebAccountManagerStatics<R, F: FnOnce(&IWebAccountManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WebAccountManager, IWebAccountManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWebAccountManagerStatics2<R, F: FnOnce(&IWebAccountManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WebAccountManager, IWebAccountManagerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWebAccountManagerStatics3<R, F: FnOnce(&IWebAccountManagerStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WebAccountManager, IWebAccountManagerStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWebAccountManagerStatics4<R, F: FnOnce(&IWebAccountManagerStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WebAccountManager, IWebAccountManagerStatics4> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWebAccountMapManagerStatics<R, F: FnOnce(&IWebAccountMapManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WebAccountManager, IWebAccountMapManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWebAccountScopeManagerStatics<R, F: FnOnce(&IWebAccountScopeManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WebAccountManager, IWebAccountScopeManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for WebAccountManager {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountManager";
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebAccountProviderAddAccountOperation(::windows_core::IUnknown);
impl WebAccountProviderAddAccountOperation {
    pub fn ReportCompleted(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCompleted)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<WebAccountProviderOperationKind> {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WebAccountProviderAddAccountOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountProviderAddAccountOperation {}
impl ::core::fmt::Debug for WebAccountProviderAddAccountOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountProviderAddAccountOperation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WebAccountProviderAddAccountOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Provider.WebAccountProviderAddAccountOperation;{73ebdccf-4378-4c79-9335-a5d7ab81594e})");
}
impl ::core::clone::Clone for WebAccountProviderAddAccountOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WebAccountProviderAddAccountOperation {
    type Vtable = IWebAccountProviderAddAccountOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebAccountProviderAddAccountOperation {
    const IID: ::windows_core::GUID = <IWebAccountProviderAddAccountOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebAccountProviderAddAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderAddAccountOperation";
}
::windows_core::imp::interface_hierarchy!(WebAccountProviderAddAccountOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IWebAccountProviderOperation> for WebAccountProviderAddAccountOperation {}
unsafe impl ::core::marker::Send for WebAccountProviderAddAccountOperation {}
unsafe impl ::core::marker::Sync for WebAccountProviderAddAccountOperation {}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebAccountProviderDeleteAccountOperation(::windows_core::IUnknown);
impl WebAccountProviderDeleteAccountOperation {
    pub fn ReportCompleted(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReportCompleted)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Core::WebProviderError>,
    {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReportError)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn WebAccount(&self) -> ::windows_core::Result<super::super::super::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WebAccount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<WebAccountProviderOperationKind> {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WebAccountProviderDeleteAccountOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountProviderDeleteAccountOperation {}
impl ::core::fmt::Debug for WebAccountProviderDeleteAccountOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountProviderDeleteAccountOperation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WebAccountProviderDeleteAccountOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Provider.WebAccountProviderDeleteAccountOperation;{0abb48b8-9e01-49c9-a355-7d48caf7d6ca})");
}
impl ::core::clone::Clone for WebAccountProviderDeleteAccountOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WebAccountProviderDeleteAccountOperation {
    type Vtable = IWebAccountProviderDeleteAccountOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebAccountProviderDeleteAccountOperation {
    const IID: ::windows_core::GUID = <IWebAccountProviderDeleteAccountOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebAccountProviderDeleteAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderDeleteAccountOperation";
}
::windows_core::imp::interface_hierarchy!(WebAccountProviderDeleteAccountOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IWebAccountProviderBaseReportOperation> for WebAccountProviderDeleteAccountOperation {}
impl ::windows_core::CanTryInto<IWebAccountProviderOperation> for WebAccountProviderDeleteAccountOperation {}
unsafe impl ::core::marker::Send for WebAccountProviderDeleteAccountOperation {}
unsafe impl ::core::marker::Sync for WebAccountProviderDeleteAccountOperation {}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebAccountProviderGetTokenSilentOperation(::windows_core::IUnknown);
impl WebAccountProviderGetTokenSilentOperation {
    pub fn ReportCompleted(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReportCompleted)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Core::WebProviderError>,
    {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReportError)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<WebAccountProviderOperationKind> {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportUserInteractionRequired(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderSilentReportOperation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReportUserInteractionRequired)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportUserInteractionRequiredWithError<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Core::WebProviderError>,
    {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderSilentReportOperation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReportUserInteractionRequiredWithError)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ProviderRequest(&self) -> ::windows_core::Result<WebProviderTokenRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProviderResponses(&self) -> ::windows_core::Result<super::super::super::super::Foundation::Collections::IVector<WebProviderTokenResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderResponses)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCacheExpirationTime(&self, value: super::super::super::super::Foundation::DateTime) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCacheExpirationTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CacheExpirationTime(&self) -> ::windows_core::Result<super::super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CacheExpirationTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WebAccountProviderGetTokenSilentOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountProviderGetTokenSilentOperation {}
impl ::core::fmt::Debug for WebAccountProviderGetTokenSilentOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountProviderGetTokenSilentOperation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WebAccountProviderGetTokenSilentOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Provider.WebAccountProviderGetTokenSilentOperation;{95c613be-2034-4c38-9434-d26c14b2b4b2})");
}
impl ::core::clone::Clone for WebAccountProviderGetTokenSilentOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WebAccountProviderGetTokenSilentOperation {
    type Vtable = IWebAccountProviderTokenOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebAccountProviderGetTokenSilentOperation {
    const IID: ::windows_core::GUID = <IWebAccountProviderTokenOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebAccountProviderGetTokenSilentOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderGetTokenSilentOperation";
}
::windows_core::imp::interface_hierarchy!(WebAccountProviderGetTokenSilentOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IWebAccountProviderBaseReportOperation> for WebAccountProviderGetTokenSilentOperation {}
impl ::windows_core::CanTryInto<IWebAccountProviderOperation> for WebAccountProviderGetTokenSilentOperation {}
impl ::windows_core::CanTryInto<IWebAccountProviderSilentReportOperation> for WebAccountProviderGetTokenSilentOperation {}
impl ::windows_core::CanTryInto<IWebAccountProviderTokenOperation> for WebAccountProviderGetTokenSilentOperation {}
unsafe impl ::core::marker::Send for WebAccountProviderGetTokenSilentOperation {}
unsafe impl ::core::marker::Sync for WebAccountProviderGetTokenSilentOperation {}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebAccountProviderManageAccountOperation(::windows_core::IUnknown);
impl WebAccountProviderManageAccountOperation {
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn WebAccount(&self) -> ::windows_core::Result<super::super::super::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WebAccount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportCompleted(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCompleted)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<WebAccountProviderOperationKind> {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WebAccountProviderManageAccountOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountProviderManageAccountOperation {}
impl ::core::fmt::Debug for WebAccountProviderManageAccountOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountProviderManageAccountOperation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WebAccountProviderManageAccountOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Provider.WebAccountProviderManageAccountOperation;{ed20dc5c-d21b-463e-a9b7-c1fd0edae978})");
}
impl ::core::clone::Clone for WebAccountProviderManageAccountOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WebAccountProviderManageAccountOperation {
    type Vtable = IWebAccountProviderManageAccountOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebAccountProviderManageAccountOperation {
    const IID: ::windows_core::GUID = <IWebAccountProviderManageAccountOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebAccountProviderManageAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderManageAccountOperation";
}
::windows_core::imp::interface_hierarchy!(WebAccountProviderManageAccountOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IWebAccountProviderOperation> for WebAccountProviderManageAccountOperation {}
unsafe impl ::core::marker::Send for WebAccountProviderManageAccountOperation {}
unsafe impl ::core::marker::Sync for WebAccountProviderManageAccountOperation {}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebAccountProviderRequestTokenOperation(::windows_core::IUnknown);
impl WebAccountProviderRequestTokenOperation {
    pub fn ReportCompleted(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReportCompleted)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Core::WebProviderError>,
    {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReportError)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<WebAccountProviderOperationKind> {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProviderRequest(&self) -> ::windows_core::Result<WebProviderTokenRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProviderResponses(&self) -> ::windows_core::Result<super::super::super::super::Foundation::Collections::IVector<WebProviderTokenResponse>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderResponses)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCacheExpirationTime(&self, value: super::super::super::super::Foundation::DateTime) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCacheExpirationTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CacheExpirationTime(&self) -> ::windows_core::Result<super::super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CacheExpirationTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportUserCanceled(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderUIReportOperation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReportUserCanceled)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for WebAccountProviderRequestTokenOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountProviderRequestTokenOperation {}
impl ::core::fmt::Debug for WebAccountProviderRequestTokenOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountProviderRequestTokenOperation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WebAccountProviderRequestTokenOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Provider.WebAccountProviderRequestTokenOperation;{95c613be-2034-4c38-9434-d26c14b2b4b2})");
}
impl ::core::clone::Clone for WebAccountProviderRequestTokenOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WebAccountProviderRequestTokenOperation {
    type Vtable = IWebAccountProviderTokenOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebAccountProviderRequestTokenOperation {
    const IID: ::windows_core::GUID = <IWebAccountProviderTokenOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebAccountProviderRequestTokenOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderRequestTokenOperation";
}
::windows_core::imp::interface_hierarchy!(WebAccountProviderRequestTokenOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IWebAccountProviderBaseReportOperation> for WebAccountProviderRequestTokenOperation {}
impl ::windows_core::CanTryInto<IWebAccountProviderOperation> for WebAccountProviderRequestTokenOperation {}
impl ::windows_core::CanTryInto<IWebAccountProviderTokenOperation> for WebAccountProviderRequestTokenOperation {}
impl ::windows_core::CanTryInto<IWebAccountProviderUIReportOperation> for WebAccountProviderRequestTokenOperation {}
unsafe impl ::core::marker::Send for WebAccountProviderRequestTokenOperation {}
unsafe impl ::core::marker::Sync for WebAccountProviderRequestTokenOperation {}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebAccountProviderRetrieveCookiesOperation(::windows_core::IUnknown);
impl WebAccountProviderRetrieveCookiesOperation {
    pub fn ReportCompleted(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReportCompleted)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Core::WebProviderError>,
    {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReportError)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<WebAccountProviderOperationKind> {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Context(&self) -> ::windows_core::Result<super::super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Context)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Web_Http\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Http"))]
    pub fn Cookies(&self) -> ::windows_core::Result<super::super::super::super::Foundation::Collections::IVector<super::super::super::super::Web::Http::HttpCookie>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Cookies)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri<P0>(&self, uri: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), uri.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ApplicationCallbackUri(&self) -> ::windows_core::Result<super::super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ApplicationCallbackUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WebAccountProviderRetrieveCookiesOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountProviderRetrieveCookiesOperation {}
impl ::core::fmt::Debug for WebAccountProviderRetrieveCookiesOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountProviderRetrieveCookiesOperation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WebAccountProviderRetrieveCookiesOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Provider.WebAccountProviderRetrieveCookiesOperation;{5a040441-0fa3-4ab1-a01c-20b110358594})");
}
impl ::core::clone::Clone for WebAccountProviderRetrieveCookiesOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WebAccountProviderRetrieveCookiesOperation {
    type Vtable = IWebAccountProviderRetrieveCookiesOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebAccountProviderRetrieveCookiesOperation {
    const IID: ::windows_core::GUID = <IWebAccountProviderRetrieveCookiesOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebAccountProviderRetrieveCookiesOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderRetrieveCookiesOperation";
}
::windows_core::imp::interface_hierarchy!(WebAccountProviderRetrieveCookiesOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IWebAccountProviderBaseReportOperation> for WebAccountProviderRetrieveCookiesOperation {}
impl ::windows_core::CanTryInto<IWebAccountProviderOperation> for WebAccountProviderRetrieveCookiesOperation {}
unsafe impl ::core::marker::Send for WebAccountProviderRetrieveCookiesOperation {}
unsafe impl ::core::marker::Sync for WebAccountProviderRetrieveCookiesOperation {}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebAccountProviderSignOutAccountOperation(::windows_core::IUnknown);
impl WebAccountProviderSignOutAccountOperation {
    pub fn ReportCompleted(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReportCompleted)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Core::WebProviderError>,
    {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).ReportError)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Kind(&self) -> ::windows_core::Result<WebAccountProviderOperationKind> {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn WebAccount(&self) -> ::windows_core::Result<super::super::super::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WebAccount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ApplicationCallbackUri(&self) -> ::windows_core::Result<super::super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ApplicationCallbackUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ClientId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClientId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WebAccountProviderSignOutAccountOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountProviderSignOutAccountOperation {}
impl ::core::fmt::Debug for WebAccountProviderSignOutAccountOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountProviderSignOutAccountOperation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WebAccountProviderSignOutAccountOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Provider.WebAccountProviderSignOutAccountOperation;{b890e21d-0c55-47bc-8c72-04a6fc7cac07})");
}
impl ::core::clone::Clone for WebAccountProviderSignOutAccountOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WebAccountProviderSignOutAccountOperation {
    type Vtable = IWebAccountProviderSignOutAccountOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebAccountProviderSignOutAccountOperation {
    const IID: ::windows_core::GUID = <IWebAccountProviderSignOutAccountOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebAccountProviderSignOutAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderSignOutAccountOperation";
}
::windows_core::imp::interface_hierarchy!(WebAccountProviderSignOutAccountOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IWebAccountProviderBaseReportOperation> for WebAccountProviderSignOutAccountOperation {}
impl ::windows_core::CanTryInto<IWebAccountProviderOperation> for WebAccountProviderSignOutAccountOperation {}
unsafe impl ::core::marker::Send for WebAccountProviderSignOutAccountOperation {}
unsafe impl ::core::marker::Sync for WebAccountProviderSignOutAccountOperation {}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebAccountProviderTriggerDetails(::windows_core::IUnknown);
impl WebAccountProviderTriggerDetails {
    pub fn Operation(&self) -> ::windows_core::Result<IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProviderTokenObjects2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WebAccountProviderTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountProviderTriggerDetails {}
impl ::core::fmt::Debug for WebAccountProviderTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountProviderTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WebAccountProviderTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Provider.WebAccountProviderTriggerDetails;{408f284b-1328-42db-89a4-0bce7a717d8e})");
}
impl ::core::clone::Clone for WebAccountProviderTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WebAccountProviderTriggerDetails {
    type Vtable = IWebAccountProviderTokenObjects_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebAccountProviderTriggerDetails {
    const IID: ::windows_core::GUID = <IWebAccountProviderTokenObjects as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebAccountProviderTriggerDetails {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(WebAccountProviderTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IWebAccountProviderTokenObjects> for WebAccountProviderTriggerDetails {}
impl ::windows_core::CanTryInto<IWebAccountProviderTokenObjects2> for WebAccountProviderTriggerDetails {}
unsafe impl ::core::marker::Send for WebAccountProviderTriggerDetails {}
unsafe impl ::core::marker::Sync for WebAccountProviderTriggerDetails {}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebProviderTokenRequest(::windows_core::IUnknown);
impl WebProviderTokenRequest {
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ClientRequest(&self) -> ::windows_core::Result<super::Core::WebTokenRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClientRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn WebAccounts(&self) -> ::windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WebAccounts)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn WebAccountSelectionOptions(&self) -> ::windows_core::Result<WebAccountSelectionOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WebAccountSelectionOptions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ApplicationCallbackUri(&self) -> ::windows_core::Result<super::super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ApplicationCallbackUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Cryptography_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Core"))]
    pub fn GetApplicationTokenBindingKeyAsync<P0>(&self, keytype: super::TokenBindingKeyType, target: P0) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Cryptography::Core::CryptographicKey>>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetApplicationTokenBindingKeyAsync)(::windows_core::Interface::as_raw(this), keytype, target.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetApplicationTokenBindingKeyIdAsync<P0>(&self, keytype: super::TokenBindingKeyType, target: P0) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Storage::Streams::IBuffer>>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Foundation::Uri>,
    {
        let this = &::windows_core::ComInterface::cast::<IWebProviderTokenRequest2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetApplicationTokenBindingKeyIdAsync)(::windows_core::Interface::as_raw(this), keytype, target.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn ApplicationPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IWebProviderTokenRequest3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ApplicationPackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ApplicationProcessName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IWebProviderTokenRequest3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ApplicationProcessName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CheckApplicationForCapabilityAsync(&self, capabilityname: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::ComInterface::cast::<IWebProviderTokenRequest3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CheckApplicationForCapabilityAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(capabilityname), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WebProviderTokenRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebProviderTokenRequest {}
impl ::core::fmt::Debug for WebProviderTokenRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebProviderTokenRequest").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WebProviderTokenRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Provider.WebProviderTokenRequest;{1e18778b-8805-454b-9f11-468d2af1095a})");
}
impl ::core::clone::Clone for WebProviderTokenRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WebProviderTokenRequest {
    type Vtable = IWebProviderTokenRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebProviderTokenRequest {
    const IID: ::windows_core::GUID = <IWebProviderTokenRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebProviderTokenRequest {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebProviderTokenRequest";
}
::windows_core::imp::interface_hierarchy!(WebProviderTokenRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WebProviderTokenRequest {}
unsafe impl ::core::marker::Sync for WebProviderTokenRequest {}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebProviderTokenResponse(::windows_core::IUnknown);
impl WebProviderTokenResponse {
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ClientResponse(&self) -> ::windows_core::Result<super::Core::WebTokenResponse> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ClientResponse)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn Create<P0>(webtokenresponse: P0) -> ::windows_core::Result<WebProviderTokenResponse>
    where
        P0: ::windows_core::IntoParam<super::Core::WebTokenResponse>,
    {
        Self::IWebProviderTokenResponseFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), webtokenresponse.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebProviderTokenResponseFactory<R, F: FnOnce(&IWebProviderTokenResponseFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WebProviderTokenResponse, IWebProviderTokenResponseFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for WebProviderTokenResponse {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebProviderTokenResponse {}
impl ::core::fmt::Debug for WebProviderTokenResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebProviderTokenResponse").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WebProviderTokenResponse {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Provider.WebProviderTokenResponse;{ef213793-ef55-4186-b7ce-8cb2e7f9849e})");
}
impl ::core::clone::Clone for WebProviderTokenResponse {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WebProviderTokenResponse {
    type Vtable = IWebProviderTokenResponse_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebProviderTokenResponse {
    const IID: ::windows_core::GUID = <IWebProviderTokenResponse as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebProviderTokenResponse {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebProviderTokenResponse";
}
::windows_core::imp::interface_hierarchy!(WebProviderTokenResponse, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WebProviderTokenResponse {}
unsafe impl ::core::marker::Sync for WebProviderTokenResponse {}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WebAccountClientViewType(pub i32);
impl WebAccountClientViewType {
    pub const IdOnly: Self = Self(0i32);
    pub const IdAndProperties: Self = Self(1i32);
}
impl ::core::marker::Copy for WebAccountClientViewType {}
impl ::core::clone::Clone for WebAccountClientViewType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WebAccountClientViewType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WebAccountClientViewType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WebAccountClientViewType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountClientViewType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WebAccountClientViewType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Provider.WebAccountClientViewType;i4)");
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WebAccountProviderOperationKind(pub i32);
impl WebAccountProviderOperationKind {
    pub const RequestToken: Self = Self(0i32);
    pub const GetTokenSilently: Self = Self(1i32);
    pub const AddAccount: Self = Self(2i32);
    pub const ManageAccount: Self = Self(3i32);
    pub const DeleteAccount: Self = Self(4i32);
    pub const RetrieveCookies: Self = Self(5i32);
    pub const SignOutAccount: Self = Self(6i32);
}
impl ::core::marker::Copy for WebAccountProviderOperationKind {}
impl ::core::clone::Clone for WebAccountProviderOperationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WebAccountProviderOperationKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WebAccountProviderOperationKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WebAccountProviderOperationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountProviderOperationKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WebAccountProviderOperationKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Provider.WebAccountProviderOperationKind;i4)");
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WebAccountScope(pub i32);
impl WebAccountScope {
    pub const PerUser: Self = Self(0i32);
    pub const PerApplication: Self = Self(1i32);
}
impl ::core::marker::Copy for WebAccountScope {}
impl ::core::clone::Clone for WebAccountScope {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WebAccountScope {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WebAccountScope {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WebAccountScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountScope").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WebAccountScope {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Provider.WebAccountScope;i4)");
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WebAccountSelectionOptions(pub u32);
impl WebAccountSelectionOptions {
    pub const Default: Self = Self(0u32);
    pub const New: Self = Self(1u32);
}
impl ::core::marker::Copy for WebAccountSelectionOptions {}
impl ::core::clone::Clone for WebAccountSelectionOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WebAccountSelectionOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WebAccountSelectionOptions {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WebAccountSelectionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountSelectionOptions").field(&self.0).finish()
    }
}
impl WebAccountSelectionOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for WebAccountSelectionOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WebAccountSelectionOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WebAccountSelectionOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WebAccountSelectionOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WebAccountSelectionOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for WebAccountSelectionOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Provider.WebAccountSelectionOptions;u4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
