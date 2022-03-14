#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountClientView(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountClientView {
    type Vtable = IWebAccountClientView_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7bd66ba_0bc7_4c66_bfd4_65d3082cbca8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountClientView_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ApplicationCallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplicationCallbackUri: usize,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebAccountClientViewType) -> ::windows::core::HRESULT,
    pub AccountPairwiseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountClientViewFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountClientViewFactory {
    type Vtable = IWebAccountClientViewFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x616d16a4_de22_4855_a326_06cebf2a3f23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountClientViewFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewtype: WebAccountClientViewType, applicationcallbackuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
    #[cfg(feature = "Foundation")]
    pub CreateWithPairwiseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewtype: WebAccountClientViewType, applicationcallbackuri: ::windows::core::RawPtr, accountpairwiseid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWithPairwiseId: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountManagerStatics {
    type Vtable = IWebAccountManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2e8e1a6_d49a_4032_84bf_1a2847747bf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub UpdateWebAccountPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, additionalproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    UpdateWebAccountPropertiesAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub AddWebAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    AddWebAccountAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub DeleteWebAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    DeleteWebAccountAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub FindAllProviderWebAccountsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    FindAllProviderWebAccountsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Http"))]
    pub PushCookiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, cookies: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Web_Http")))]
    PushCookiesAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub SetViewAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, view: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    SetViewAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub ClearViewAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, applicationcallbackuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    ClearViewAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub GetViewsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    GetViewsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "Storage_Streams"))]
    pub SetWebAccountPictureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, webaccountpicture: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials", feature = "Storage_Streams")))]
    SetWebAccountPictureAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub ClearWebAccountPictureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    ClearWebAccountPictureAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountManagerStatics2 {
    type Vtable = IWebAccountManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68a7a829_2d5f_4653_8bb0_bd2fa6bd2d87);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountManagerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub PullCookiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uristring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, callerpfn: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PullCookiesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountManagerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountManagerStatics3 {
    type Vtable = IWebAccountManagerStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd4523a6_8a4f_4aa2_b15e_03f550af1359);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountManagerStatics3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub FindAllProviderWebAccountsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System")))]
    FindAllProviderWebAccountsForUserAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub AddWebAccountForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System")))]
    AddWebAccountForUserAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub AddWebAccountWithScopeForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, scope: WebAccountScope, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System")))]
    AddWebAccountWithScopeForUserAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub AddWebAccountWithScopeAndMapForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, scope: WebAccountScope, peruserwebaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System")))]
    AddWebAccountWithScopeAndMapForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountManagerStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountManagerStatics4 {
    type Vtable = IWebAccountManagerStatics4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59ebc2d2_f7db_412f_bc3f_f2fea04430b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountManagerStatics4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub InvalidateAppCacheForAllAccountsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InvalidateAppCacheForAllAccountsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub InvalidateAppCacheForAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    InvalidateAppCacheForAccountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountMapManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountMapManagerStatics {
    type Vtable = IWebAccountMapManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8fa446f_3a1b_48a4_8e90_1e59ca6f54db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountMapManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub AddWebAccountWithScopeAndMapAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, scope: WebAccountScope, peruserwebaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    AddWebAccountWithScopeAndMapAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub SetPerAppToPerUserAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, perappaccount: ::windows::core::RawPtr, peruserwebaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    SetPerAppToPerUserAccountAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub GetPerUserFromPerAppAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, perappaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    GetPerUserFromPerAppAccountAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub ClearPerUserFromPerAppAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, perappaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    ClearPerUserFromPerAppAccountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProviderAddAccountOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountProviderAddAccountOperation {
    type Vtable = IWebAccountProviderAddAccountOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73ebdccf_4378_4c79_9335_a5d7ab81594e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderAddAccountOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct IWebAccountProviderBaseReportOperation(::windows::core::IUnknown);
impl IWebAccountProviderBaseReportOperation {
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn ReportCompleted(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCompleted)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<'a, Param0: ::windows::core::IntoParam<'a, super::Core::WebProviderError>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportError)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IWebAccountProviderBaseReportOperation> for ::windows::core::IUnknown {
    fn from(value: IWebAccountProviderBaseReportOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccountProviderBaseReportOperation> for ::windows::core::IUnknown {
    fn from(value: &IWebAccountProviderBaseReportOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebAccountProviderBaseReportOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebAccountProviderBaseReportOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebAccountProviderBaseReportOperation> for ::windows::core::IInspectable {
    fn from(value: IWebAccountProviderBaseReportOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccountProviderBaseReportOperation> for ::windows::core::IInspectable {
    fn from(value: &IWebAccountProviderBaseReportOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWebAccountProviderBaseReportOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IWebAccountProviderBaseReportOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebAccountProviderBaseReportOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IWebAccountProviderBaseReportOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{bba4acbb-993b-4d57-bbe4-1421e3668b4c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IWebAccountProviderBaseReportOperation {
    type Vtable = IWebAccountProviderBaseReportOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbba4acbb_993b_4d57_bbe4_1421e3668b4c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderBaseReportOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub ReportError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Core"))]
    ReportError: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProviderDeleteAccountOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountProviderDeleteAccountOperation {
    type Vtable = IWebAccountProviderDeleteAccountOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0abb48b8_9e01_49c9_a355_7d48caf7d6ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderDeleteAccountOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccount: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProviderManageAccountOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountProviderManageAccountOperation {
    type Vtable = IWebAccountProviderManageAccountOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed20dc5c_d21b_463e_a9b7_c1fd0edae978);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderManageAccountOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccount: usize,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct IWebAccountProviderOperation(::windows::core::IUnknown);
impl IWebAccountProviderOperation {
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<WebAccountProviderOperationKind> {
        let this = self;
        unsafe {
            let mut result__: WebAccountProviderOperationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountProviderOperationKind>(result__)
        }
    }
}
impl ::core::convert::From<IWebAccountProviderOperation> for ::windows::core::IUnknown {
    fn from(value: IWebAccountProviderOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccountProviderOperation> for ::windows::core::IUnknown {
    fn from(value: &IWebAccountProviderOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebAccountProviderOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebAccountProviderOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebAccountProviderOperation> for ::windows::core::IInspectable {
    fn from(value: IWebAccountProviderOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccountProviderOperation> for ::windows::core::IInspectable {
    fn from(value: &IWebAccountProviderOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWebAccountProviderOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IWebAccountProviderOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebAccountProviderOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IWebAccountProviderOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6d5d2426-10b1-419a-a44e-f9c5161574e6}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IWebAccountProviderOperation {
    type Vtable = IWebAccountProviderOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d5d2426_10b1_419a_a44e_f9c5161574e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebAccountProviderOperationKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProviderRetrieveCookiesOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountProviderRetrieveCookiesOperation {
    type Vtable = IWebAccountProviderRetrieveCookiesOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a040441_0fa3_4ab1_a01c_20b110358594);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderRetrieveCookiesOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Context: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Http"))]
    pub Cookies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Web_Http")))]
    Cookies: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub ApplicationCallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplicationCallbackUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProviderSignOutAccountOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountProviderSignOutAccountOperation {
    type Vtable = IWebAccountProviderSignOutAccountOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb890e21d_0c55_47bc_8c72_04a6fc7cac07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderSignOutAccountOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Security_Credentials")]
    pub WebAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    WebAccount: usize,
    #[cfg(feature = "Foundation")]
    pub ApplicationCallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplicationCallbackUri: usize,
    pub ClientId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct IWebAccountProviderSilentReportOperation(::windows::core::IUnknown);
impl IWebAccountProviderSilentReportOperation {
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn ReportUserInteractionRequired(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportUserInteractionRequired)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportUserInteractionRequiredWithError<'a, Param0: ::windows::core::IntoParam<'a, super::Core::WebProviderError>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportUserInteractionRequiredWithError)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn ReportCompleted(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReportCompleted)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<'a, Param0: ::windows::core::IntoParam<'a, super::Core::WebProviderError>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReportError)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IWebAccountProviderSilentReportOperation> for ::windows::core::IUnknown {
    fn from(value: IWebAccountProviderSilentReportOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccountProviderSilentReportOperation> for ::windows::core::IUnknown {
    fn from(value: &IWebAccountProviderSilentReportOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebAccountProviderSilentReportOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebAccountProviderSilentReportOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebAccountProviderSilentReportOperation> for ::windows::core::IInspectable {
    fn from(value: IWebAccountProviderSilentReportOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccountProviderSilentReportOperation> for ::windows::core::IInspectable {
    fn from(value: &IWebAccountProviderSilentReportOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWebAccountProviderSilentReportOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IWebAccountProviderSilentReportOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IWebAccountProviderSilentReportOperation> for IWebAccountProviderBaseReportOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: IWebAccountProviderSilentReportOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWebAccountProviderSilentReportOperation> for IWebAccountProviderBaseReportOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &IWebAccountProviderSilentReportOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderBaseReportOperation> for IWebAccountProviderSilentReportOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderBaseReportOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderBaseReportOperation> for &IWebAccountProviderSilentReportOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderBaseReportOperation> {
        ::core::convert::TryInto::<IWebAccountProviderBaseReportOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IWebAccountProviderSilentReportOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IWebAccountProviderSilentReportOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e0b545f8-3b0f-44da-924c-7b18baaa62a9}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IWebAccountProviderSilentReportOperation {
    type Vtable = IWebAccountProviderSilentReportOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0b545f8_3b0f_44da_924c_7b18baaa62a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderSilentReportOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ReportUserInteractionRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub ReportUserInteractionRequiredWithError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Core"))]
    ReportUserInteractionRequiredWithError: usize,
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct IWebAccountProviderTokenObjects(::windows::core::IUnknown);
impl IWebAccountProviderTokenObjects {
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn Operation(&self) -> ::windows::core::Result<IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Operation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IWebAccountProviderOperation>(result__)
        }
    }
}
impl ::core::convert::From<IWebAccountProviderTokenObjects> for ::windows::core::IUnknown {
    fn from(value: IWebAccountProviderTokenObjects) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccountProviderTokenObjects> for ::windows::core::IUnknown {
    fn from(value: &IWebAccountProviderTokenObjects) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebAccountProviderTokenObjects {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebAccountProviderTokenObjects {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebAccountProviderTokenObjects> for ::windows::core::IInspectable {
    fn from(value: IWebAccountProviderTokenObjects) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccountProviderTokenObjects> for ::windows::core::IInspectable {
    fn from(value: &IWebAccountProviderTokenObjects) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWebAccountProviderTokenObjects {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IWebAccountProviderTokenObjects {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWebAccountProviderTokenObjects {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IWebAccountProviderTokenObjects {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{408f284b-1328-42db-89a4-0bce7a717d8e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IWebAccountProviderTokenObjects {
    type Vtable = IWebAccountProviderTokenObjects_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x408f284b_1328_42db_89a4_0bce7a717d8e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderTokenObjects_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct IWebAccountProviderTokenObjects2(::windows::core::IUnknown);
impl IWebAccountProviderTokenObjects2 {
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).User)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn Operation(&self) -> ::windows::core::Result<IWebAccountProviderOperation> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderTokenObjects>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Operation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IWebAccountProviderOperation>(result__)
        }
    }
}
impl ::core::convert::From<IWebAccountProviderTokenObjects2> for ::windows::core::IUnknown {
    fn from(value: IWebAccountProviderTokenObjects2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccountProviderTokenObjects2> for ::windows::core::IUnknown {
    fn from(value: &IWebAccountProviderTokenObjects2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebAccountProviderTokenObjects2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebAccountProviderTokenObjects2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebAccountProviderTokenObjects2> for ::windows::core::IInspectable {
    fn from(value: IWebAccountProviderTokenObjects2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccountProviderTokenObjects2> for ::windows::core::IInspectable {
    fn from(value: &IWebAccountProviderTokenObjects2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWebAccountProviderTokenObjects2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IWebAccountProviderTokenObjects2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IWebAccountProviderTokenObjects2> for IWebAccountProviderTokenObjects {
    type Error = ::windows::core::Error;
    fn try_from(value: IWebAccountProviderTokenObjects2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWebAccountProviderTokenObjects2> for IWebAccountProviderTokenObjects {
    type Error = ::windows::core::Error;
    fn try_from(value: &IWebAccountProviderTokenObjects2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderTokenObjects> for IWebAccountProviderTokenObjects2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderTokenObjects> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderTokenObjects> for &IWebAccountProviderTokenObjects2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderTokenObjects> {
        ::core::convert::TryInto::<IWebAccountProviderTokenObjects>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IWebAccountProviderTokenObjects2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IWebAccountProviderTokenObjects2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1020b893-5ca5-4fff-95fb-b820273fc395}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IWebAccountProviderTokenObjects2 {
    type Vtable = IWebAccountProviderTokenObjects2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1020b893_5ca5_4fff_95fb_b820273fc395);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderTokenObjects2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct IWebAccountProviderTokenOperation(::windows::core::IUnknown);
impl IWebAccountProviderTokenOperation {
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn ProviderRequest(&self) -> ::windows::core::Result<WebProviderTokenRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderRequest)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebProviderTokenRequest>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProviderResponses(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<WebProviderTokenResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderResponses)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVector<WebProviderTokenResponse>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCacheExpirationTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCacheExpirationTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CacheExpirationTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CacheExpirationTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<WebAccountProviderOperationKind> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__: WebAccountProviderOperationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountProviderOperationKind>(result__)
        }
    }
}
impl ::core::convert::From<IWebAccountProviderTokenOperation> for ::windows::core::IUnknown {
    fn from(value: IWebAccountProviderTokenOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccountProviderTokenOperation> for ::windows::core::IUnknown {
    fn from(value: &IWebAccountProviderTokenOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebAccountProviderTokenOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebAccountProviderTokenOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebAccountProviderTokenOperation> for ::windows::core::IInspectable {
    fn from(value: IWebAccountProviderTokenOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccountProviderTokenOperation> for ::windows::core::IInspectable {
    fn from(value: &IWebAccountProviderTokenOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWebAccountProviderTokenOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IWebAccountProviderTokenOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IWebAccountProviderTokenOperation> for IWebAccountProviderOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: IWebAccountProviderTokenOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWebAccountProviderTokenOperation> for IWebAccountProviderOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &IWebAccountProviderTokenOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderOperation> for IWebAccountProviderTokenOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderOperation> for &IWebAccountProviderTokenOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderOperation> {
        ::core::convert::TryInto::<IWebAccountProviderOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IWebAccountProviderTokenOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IWebAccountProviderTokenOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{95c613be-2034-4c38-9434-d26c14b2b4b2}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IWebAccountProviderTokenOperation {
    type Vtable = IWebAccountProviderTokenOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95c613be_2034_4c38_9434_d26c14b2b4b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderTokenOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ProviderRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ProviderResponses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProviderResponses: usize,
    #[cfg(feature = "Foundation")]
    pub SetCacheExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCacheExpirationTime: usize,
    #[cfg(feature = "Foundation")]
    pub CacheExpirationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CacheExpirationTime: usize,
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct IWebAccountProviderUIReportOperation(::windows::core::IUnknown);
impl IWebAccountProviderUIReportOperation {
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn ReportUserCanceled(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportUserCanceled)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn ReportCompleted(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReportCompleted)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<'a, Param0: ::windows::core::IntoParam<'a, super::Core::WebProviderError>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReportError)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IWebAccountProviderUIReportOperation> for ::windows::core::IUnknown {
    fn from(value: IWebAccountProviderUIReportOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccountProviderUIReportOperation> for ::windows::core::IUnknown {
    fn from(value: &IWebAccountProviderUIReportOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebAccountProviderUIReportOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebAccountProviderUIReportOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebAccountProviderUIReportOperation> for ::windows::core::IInspectable {
    fn from(value: IWebAccountProviderUIReportOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccountProviderUIReportOperation> for ::windows::core::IInspectable {
    fn from(value: &IWebAccountProviderUIReportOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWebAccountProviderUIReportOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IWebAccountProviderUIReportOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IWebAccountProviderUIReportOperation> for IWebAccountProviderBaseReportOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: IWebAccountProviderUIReportOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWebAccountProviderUIReportOperation> for IWebAccountProviderBaseReportOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &IWebAccountProviderUIReportOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderBaseReportOperation> for IWebAccountProviderUIReportOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderBaseReportOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderBaseReportOperation> for &IWebAccountProviderUIReportOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderBaseReportOperation> {
        ::core::convert::TryInto::<IWebAccountProviderBaseReportOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IWebAccountProviderUIReportOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IWebAccountProviderUIReportOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{28ff92d3-8f80-42fb-944f-b2107bbd42e6}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IWebAccountProviderUIReportOperation {
    type Vtable = IWebAccountProviderUIReportOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28ff92d3_8f80_42fb_944f_b2107bbd42e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderUIReportOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ReportUserCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountScopeManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountScopeManagerStatics {
    type Vtable = IWebAccountScopeManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c6ce37c_12b2_423a_bf3d_85b8d7e53656);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountScopeManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub AddWebAccountWithScopeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, webaccountusername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, props: ::windows::core::RawPtr, scope: WebAccountScope, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    AddWebAccountWithScopeAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub SetScopeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, scope: WebAccountScope, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    SetScopeAsync: usize,
    #[cfg(feature = "Security_Credentials")]
    pub GetScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut WebAccountScope) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    GetScope: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebProviderTokenRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebProviderTokenRequest {
    type Vtable = IWebProviderTokenRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e18778b_8805_454b_9f11_468d2af1095a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebProviderTokenRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub ClientRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Core"))]
    ClientRequest: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub WebAccounts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))]
    WebAccounts: usize,
    pub WebAccountSelectionOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebAccountSelectionOptions) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ApplicationCallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ApplicationCallbackUri: usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Core"))]
    pub GetApplicationTokenBindingKeyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keytype: super::TokenBindingKeyType, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Cryptography_Core")))]
    GetApplicationTokenBindingKeyAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebProviderTokenRequest2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebProviderTokenRequest2 {
    type Vtable = IWebProviderTokenRequest2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5d72e4c_10b1_4aa6_88b1_0b6c9e0c1e46);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebProviderTokenRequest2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetApplicationTokenBindingKeyIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keytype: super::TokenBindingKeyType, target: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetApplicationTokenBindingKeyIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebProviderTokenRequest3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebProviderTokenRequest3 {
    type Vtable = IWebProviderTokenRequest3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b2716aa_4289_446e_9256_dafb6f66a51e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebProviderTokenRequest3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ApplicationPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ApplicationProcessName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CheckApplicationForCapabilityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckApplicationForCapabilityAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebProviderTokenResponse(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebProviderTokenResponse {
    type Vtable = IWebProviderTokenResponse_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef213793_ef55_4186_b7ce_8cb2e7f9849e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebProviderTokenResponse_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub ClientResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Core"))]
    ClientResponse: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebProviderTokenResponseFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebProviderTokenResponseFactory {
    type Vtable = IWebProviderTokenResponseFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa49d99a_25ba_4077_9cfa_9db4dea7b71a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebProviderTokenResponseFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webtokenresponse: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Core"))]
    Create: usize,
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebAccountClientView(::windows::core::IUnknown);
impl WebAccountClientView {
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ApplicationCallbackUri(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ApplicationCallbackUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<WebAccountClientViewType> {
        let this = self;
        unsafe {
            let mut result__: WebAccountClientViewType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountClientViewType>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn AccountPairwiseId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccountPairwiseId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Create<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Uri>>(viewtype: WebAccountClientViewType, applicationcallbackuri: Param1) -> ::windows::core::Result<WebAccountClientView> {
        Self::IWebAccountClientViewFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), viewtype, applicationcallbackuri.into_param().abi(), &mut result__).from_abi::<WebAccountClientView>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWithPairwiseId<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(viewtype: WebAccountClientViewType, applicationcallbackuri: Param1, accountpairwiseid: Param2) -> ::windows::core::Result<WebAccountClientView> {
        Self::IWebAccountClientViewFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithPairwiseId)(::core::mem::transmute_copy(this), viewtype, applicationcallbackuri.into_param().abi(), accountpairwiseid.into_param().abi(), &mut result__).from_abi::<WebAccountClientView>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebAccountClientViewFactory<R, F: FnOnce(&IWebAccountClientViewFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebAccountClientView, IWebAccountClientViewFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WebAccountClientView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for WebAccountClientView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Provider.WebAccountClientView;{e7bd66ba-0bc7-4c66-bfd4-65d3082cbca8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebAccountClientView {
    type Vtable = IWebAccountClientView_Vtbl;
    const IID: ::windows::core::GUID = <IWebAccountClientView as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebAccountClientView {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountClientView";
}
impl ::core::convert::From<WebAccountClientView> for ::windows::core::IUnknown {
    fn from(value: WebAccountClientView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountClientView> for ::windows::core::IUnknown {
    fn from(value: &WebAccountClientView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebAccountClientView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebAccountClientView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebAccountClientView> for ::windows::core::IInspectable {
    fn from(value: WebAccountClientView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountClientView> for ::windows::core::IInspectable {
    fn from(value: &WebAccountClientView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebAccountClientView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebAccountClientView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WebAccountClientView {}
unsafe impl ::core::marker::Sync for WebAccountClientView {}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for WebAccountClientViewType {
    type Abi = Self;
}
impl ::core::fmt::Debug for WebAccountClientViewType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountClientViewType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebAccountClientViewType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Provider.WebAccountClientViewType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
pub struct WebAccountManager {}
impl WebAccountManager {
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation_Collections\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn UpdateWebAccountPropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccount>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>>(webaccount: Param0, webaccountusername: Param1, additionalproperties: Param2) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateWebAccountPropertiesAsync)(::core::mem::transmute_copy(this), webaccount.into_param().abi(), webaccountusername.into_param().abi(), additionalproperties.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation_Collections\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn AddWebAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>>(webaccountid: Param0, webaccountusername: Param1, props: Param2) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>> {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddWebAccountAsync)(::core::mem::transmute_copy(this), webaccountid.into_param().abi(), webaccountusername.into_param().abi(), props.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn DeleteWebAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccount>>(webaccount: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteWebAccountAsync)(::core::mem::transmute_copy(this), webaccount.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation_Collections\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn FindAllProviderWebAccountsAsync() -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>> {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAllProviderWebAccountsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation_Collections\"`, `\"Web_Http\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Http"))]
    pub fn PushCookiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Web::Http::HttpCookie>>>(uri: Param0, cookies: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PushCookiesAsync)(::core::mem::transmute_copy(this), uri.into_param().abi(), cookies.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn SetViewAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccount>, Param1: ::windows::core::IntoParam<'a, WebAccountClientView>>(webaccount: Param0, view: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetViewAsync)(::core::mem::transmute_copy(this), webaccount.into_param().abi(), view.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn ClearViewAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccount>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Uri>>(webaccount: Param0, applicationcallbackuri: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClearViewAsync)(::core::mem::transmute_copy(this), webaccount.into_param().abi(), applicationcallbackuri.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation_Collections\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn GetViewsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccount>>(webaccount: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<WebAccountClientView>>> {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetViewsAsync)(::core::mem::transmute_copy(this), webaccount.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<WebAccountClientView>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`, `\"Security_Credentials\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "Storage_Streams"))]
    pub fn SetWebAccountPictureAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccount>, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Storage::Streams::IRandomAccessStream>>(webaccount: Param0, webaccountpicture: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetWebAccountPictureAsync)(::core::mem::transmute_copy(this), webaccount.into_param().abi(), webaccountpicture.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn ClearWebAccountPictureAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccount>>(webaccount: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::IWebAccountManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClearWebAccountPictureAsync)(::core::mem::transmute_copy(this), webaccount.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PullCookiesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(uristring: Param0, callerpfn: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::IWebAccountManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PullCookiesAsync)(::core::mem::transmute_copy(this), uristring.into_param().abi(), callerpfn.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation_Collections\"`, `\"Security_Credentials\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub fn FindAllProviderWebAccountsForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>>(user: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>> {
        Self::IWebAccountManagerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAllProviderWebAccountsForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation_Collections\"`, `\"Security_Credentials\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub fn AddWebAccountForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>>(user: Param0, webaccountid: Param1, webaccountusername: Param2, props: Param3) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>> {
        Self::IWebAccountManagerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddWebAccountForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), webaccountid.into_param().abi(), webaccountusername.into_param().abi(), props.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation_Collections\"`, `\"Security_Credentials\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub fn AddWebAccountWithScopeForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>>(user: Param0, webaccountid: Param1, webaccountusername: Param2, props: Param3, scope: WebAccountScope) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>> {
        Self::IWebAccountManagerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddWebAccountWithScopeForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), webaccountid.into_param().abi(), webaccountusername.into_param().abi(), props.into_param().abi(), scope, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation_Collections\"`, `\"Security_Credentials\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "System"))]
    pub fn AddWebAccountWithScopeAndMapForUserAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::System::User>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>, Param5: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(user: Param0, webaccountid: Param1, webaccountusername: Param2, props: Param3, scope: WebAccountScope, peruserwebaccountid: Param5) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>> {
        Self::IWebAccountManagerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddWebAccountWithScopeAndMapForUserAsync)(::core::mem::transmute_copy(this), user.into_param().abi(), webaccountid.into_param().abi(), webaccountusername.into_param().abi(), props.into_param().abi(), scope, peruserwebaccountid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InvalidateAppCacheForAllAccountsAsync() -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::IWebAccountManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InvalidateAppCacheForAllAccountsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn InvalidateAppCacheForAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccount>>(webaccount: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::IWebAccountManagerStatics4(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InvalidateAppCacheForAccountAsync)(::core::mem::transmute_copy(this), webaccount.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation_Collections\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn AddWebAccountWithScopeAndMapAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>, Param4: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(webaccountid: Param0, webaccountusername: Param1, props: Param2, scope: WebAccountScope, peruserwebaccountid: Param4) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>> {
        Self::IWebAccountMapManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddWebAccountWithScopeAndMapAsync)(::core::mem::transmute_copy(this), webaccountid.into_param().abi(), webaccountusername.into_param().abi(), props.into_param().abi(), scope, peruserwebaccountid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn SetPerAppToPerUserAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccount>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(perappaccount: Param0, peruserwebaccountid: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::IWebAccountMapManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetPerAppToPerUserAccountAsync)(::core::mem::transmute_copy(this), perappaccount.into_param().abi(), peruserwebaccountid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn GetPerUserFromPerAppAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccount>>(perappaccount: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>> {
        Self::IWebAccountMapManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPerUserFromPerAppAccountAsync)(::core::mem::transmute_copy(this), perappaccount.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn ClearPerUserFromPerAppAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccount>>(perappaccount: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::IWebAccountMapManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClearPerUserFromPerAppAccountAsync)(::core::mem::transmute_copy(this), perappaccount.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation_Collections\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn AddWebAccountWithScopeAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>>(webaccountid: Param0, webaccountusername: Param1, props: Param2, scope: WebAccountScope) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>> {
        Self::IWebAccountScopeManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddWebAccountWithScopeAsync)(::core::mem::transmute_copy(this), webaccountid.into_param().abi(), webaccountusername.into_param().abi(), props.into_param().abi(), scope, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn SetScopeAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccount>>(webaccount: Param0, scope: WebAccountScope) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction> {
        Self::IWebAccountScopeManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetScopeAsync)(::core::mem::transmute_copy(this), webaccount.into_param().abi(), scope, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn GetScope<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Credentials::WebAccount>>(webaccount: Param0) -> ::windows::core::Result<WebAccountScope> {
        Self::IWebAccountScopeManagerStatics(|this| unsafe {
            let mut result__: WebAccountScope = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetScope)(::core::mem::transmute_copy(this), webaccount.into_param().abi(), &mut result__).from_abi::<WebAccountScope>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebAccountManagerStatics<R, F: FnOnce(&IWebAccountManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebAccountManager, IWebAccountManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IWebAccountManagerStatics2<R, F: FnOnce(&IWebAccountManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebAccountManager, IWebAccountManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IWebAccountManagerStatics3<R, F: FnOnce(&IWebAccountManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebAccountManager, IWebAccountManagerStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IWebAccountManagerStatics4<R, F: FnOnce(&IWebAccountManagerStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebAccountManager, IWebAccountManagerStatics4> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IWebAccountMapManagerStatics<R, F: FnOnce(&IWebAccountMapManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebAccountManager, IWebAccountMapManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IWebAccountScopeManagerStatics<R, F: FnOnce(&IWebAccountScopeManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebAccountManager, IWebAccountScopeManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for WebAccountManager {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountManager";
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebAccountProviderAddAccountOperation(::windows::core::IUnknown);
impl WebAccountProviderAddAccountOperation {
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn ReportCompleted(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCompleted)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<WebAccountProviderOperationKind> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__: WebAccountProviderOperationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountProviderOperationKind>(result__)
        }
    }
}
impl ::core::clone::Clone for WebAccountProviderAddAccountOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for WebAccountProviderAddAccountOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Provider.WebAccountProviderAddAccountOperation;{73ebdccf-4378-4c79-9335-a5d7ab81594e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebAccountProviderAddAccountOperation {
    type Vtable = IWebAccountProviderAddAccountOperation_Vtbl;
    const IID: ::windows::core::GUID = <IWebAccountProviderAddAccountOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebAccountProviderAddAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderAddAccountOperation";
}
impl ::core::convert::From<WebAccountProviderAddAccountOperation> for ::windows::core::IUnknown {
    fn from(value: WebAccountProviderAddAccountOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderAddAccountOperation> for ::windows::core::IUnknown {
    fn from(value: &WebAccountProviderAddAccountOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebAccountProviderAddAccountOperation> for ::windows::core::IInspectable {
    fn from(value: WebAccountProviderAddAccountOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderAddAccountOperation> for ::windows::core::IInspectable {
    fn from(value: &WebAccountProviderAddAccountOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WebAccountProviderAddAccountOperation> for IWebAccountProviderOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderAddAccountOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderAddAccountOperation> for IWebAccountProviderOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderAddAccountOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderOperation> for WebAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderOperation> for &WebAccountProviderAddAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderOperation> {
        ::core::convert::TryInto::<IWebAccountProviderOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebAccountProviderAddAccountOperation {}
unsafe impl ::core::marker::Sync for WebAccountProviderAddAccountOperation {}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebAccountProviderDeleteAccountOperation(::windows::core::IUnknown);
impl WebAccountProviderDeleteAccountOperation {
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn ReportCompleted(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReportCompleted)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<'a, Param0: ::windows::core::IntoParam<'a, super::Core::WebProviderError>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReportError)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn WebAccount(&self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WebAccount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Credentials::WebAccount>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<WebAccountProviderOperationKind> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__: WebAccountProviderOperationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountProviderOperationKind>(result__)
        }
    }
}
impl ::core::clone::Clone for WebAccountProviderDeleteAccountOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for WebAccountProviderDeleteAccountOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Provider.WebAccountProviderDeleteAccountOperation;{0abb48b8-9e01-49c9-a355-7d48caf7d6ca})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebAccountProviderDeleteAccountOperation {
    type Vtable = IWebAccountProviderDeleteAccountOperation_Vtbl;
    const IID: ::windows::core::GUID = <IWebAccountProviderDeleteAccountOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebAccountProviderDeleteAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderDeleteAccountOperation";
}
impl ::core::convert::From<WebAccountProviderDeleteAccountOperation> for ::windows::core::IUnknown {
    fn from(value: WebAccountProviderDeleteAccountOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderDeleteAccountOperation> for ::windows::core::IUnknown {
    fn from(value: &WebAccountProviderDeleteAccountOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebAccountProviderDeleteAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebAccountProviderDeleteAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebAccountProviderDeleteAccountOperation> for ::windows::core::IInspectable {
    fn from(value: WebAccountProviderDeleteAccountOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderDeleteAccountOperation> for ::windows::core::IInspectable {
    fn from(value: &WebAccountProviderDeleteAccountOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebAccountProviderDeleteAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebAccountProviderDeleteAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WebAccountProviderDeleteAccountOperation> for IWebAccountProviderBaseReportOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderDeleteAccountOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderDeleteAccountOperation> for IWebAccountProviderBaseReportOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderDeleteAccountOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderBaseReportOperation> for WebAccountProviderDeleteAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderBaseReportOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderBaseReportOperation> for &WebAccountProviderDeleteAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderBaseReportOperation> {
        ::core::convert::TryInto::<IWebAccountProviderBaseReportOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<WebAccountProviderDeleteAccountOperation> for IWebAccountProviderOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderDeleteAccountOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderDeleteAccountOperation> for IWebAccountProviderOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderDeleteAccountOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderOperation> for WebAccountProviderDeleteAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderOperation> for &WebAccountProviderDeleteAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderOperation> {
        ::core::convert::TryInto::<IWebAccountProviderOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebAccountProviderDeleteAccountOperation {}
unsafe impl ::core::marker::Sync for WebAccountProviderDeleteAccountOperation {}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebAccountProviderGetTokenSilentOperation(::windows::core::IUnknown);
impl WebAccountProviderGetTokenSilentOperation {
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn ReportCompleted(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReportCompleted)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<'a, Param0: ::windows::core::IntoParam<'a, super::Core::WebProviderError>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReportError)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<WebAccountProviderOperationKind> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__: WebAccountProviderOperationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountProviderOperationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn ReportUserInteractionRequired(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderSilentReportOperation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReportUserInteractionRequired)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportUserInteractionRequiredWithError<'a, Param0: ::windows::core::IntoParam<'a, super::Core::WebProviderError>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderSilentReportOperation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReportUserInteractionRequiredWithError)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn ProviderRequest(&self) -> ::windows::core::Result<WebProviderTokenRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderRequest)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebProviderTokenRequest>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProviderResponses(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<WebProviderTokenResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderResponses)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVector<WebProviderTokenResponse>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCacheExpirationTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCacheExpirationTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CacheExpirationTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CacheExpirationTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::DateTime>(result__)
        }
    }
}
impl ::core::clone::Clone for WebAccountProviderGetTokenSilentOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for WebAccountProviderGetTokenSilentOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Provider.WebAccountProviderGetTokenSilentOperation;{95c613be-2034-4c38-9434-d26c14b2b4b2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebAccountProviderGetTokenSilentOperation {
    type Vtable = IWebAccountProviderTokenOperation_Vtbl;
    const IID: ::windows::core::GUID = <IWebAccountProviderTokenOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebAccountProviderGetTokenSilentOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderGetTokenSilentOperation";
}
impl ::core::convert::From<WebAccountProviderGetTokenSilentOperation> for ::windows::core::IUnknown {
    fn from(value: WebAccountProviderGetTokenSilentOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderGetTokenSilentOperation> for ::windows::core::IUnknown {
    fn from(value: &WebAccountProviderGetTokenSilentOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebAccountProviderGetTokenSilentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebAccountProviderGetTokenSilentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebAccountProviderGetTokenSilentOperation> for ::windows::core::IInspectable {
    fn from(value: WebAccountProviderGetTokenSilentOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderGetTokenSilentOperation> for ::windows::core::IInspectable {
    fn from(value: &WebAccountProviderGetTokenSilentOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebAccountProviderGetTokenSilentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebAccountProviderGetTokenSilentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WebAccountProviderGetTokenSilentOperation> for IWebAccountProviderBaseReportOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderGetTokenSilentOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderGetTokenSilentOperation> for IWebAccountProviderBaseReportOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderGetTokenSilentOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderBaseReportOperation> for WebAccountProviderGetTokenSilentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderBaseReportOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderBaseReportOperation> for &WebAccountProviderGetTokenSilentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderBaseReportOperation> {
        ::core::convert::TryInto::<IWebAccountProviderBaseReportOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<WebAccountProviderGetTokenSilentOperation> for IWebAccountProviderOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderGetTokenSilentOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderGetTokenSilentOperation> for IWebAccountProviderOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderGetTokenSilentOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderOperation> for WebAccountProviderGetTokenSilentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderOperation> for &WebAccountProviderGetTokenSilentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderOperation> {
        ::core::convert::TryInto::<IWebAccountProviderOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<WebAccountProviderGetTokenSilentOperation> for IWebAccountProviderSilentReportOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderGetTokenSilentOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderGetTokenSilentOperation> for IWebAccountProviderSilentReportOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderGetTokenSilentOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderSilentReportOperation> for WebAccountProviderGetTokenSilentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderSilentReportOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderSilentReportOperation> for &WebAccountProviderGetTokenSilentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderSilentReportOperation> {
        ::core::convert::TryInto::<IWebAccountProviderSilentReportOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<WebAccountProviderGetTokenSilentOperation> for IWebAccountProviderTokenOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderGetTokenSilentOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderGetTokenSilentOperation> for IWebAccountProviderTokenOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderGetTokenSilentOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderTokenOperation> for WebAccountProviderGetTokenSilentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderTokenOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderTokenOperation> for &WebAccountProviderGetTokenSilentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderTokenOperation> {
        ::core::convert::TryInto::<IWebAccountProviderTokenOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebAccountProviderGetTokenSilentOperation {}
unsafe impl ::core::marker::Sync for WebAccountProviderGetTokenSilentOperation {}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebAccountProviderManageAccountOperation(::windows::core::IUnknown);
impl WebAccountProviderManageAccountOperation {
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn WebAccount(&self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WebAccount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Credentials::WebAccount>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn ReportCompleted(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCompleted)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<WebAccountProviderOperationKind> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__: WebAccountProviderOperationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountProviderOperationKind>(result__)
        }
    }
}
impl ::core::clone::Clone for WebAccountProviderManageAccountOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for WebAccountProviderManageAccountOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Provider.WebAccountProviderManageAccountOperation;{ed20dc5c-d21b-463e-a9b7-c1fd0edae978})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebAccountProviderManageAccountOperation {
    type Vtable = IWebAccountProviderManageAccountOperation_Vtbl;
    const IID: ::windows::core::GUID = <IWebAccountProviderManageAccountOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebAccountProviderManageAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderManageAccountOperation";
}
impl ::core::convert::From<WebAccountProviderManageAccountOperation> for ::windows::core::IUnknown {
    fn from(value: WebAccountProviderManageAccountOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderManageAccountOperation> for ::windows::core::IUnknown {
    fn from(value: &WebAccountProviderManageAccountOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebAccountProviderManageAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebAccountProviderManageAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebAccountProviderManageAccountOperation> for ::windows::core::IInspectable {
    fn from(value: WebAccountProviderManageAccountOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderManageAccountOperation> for ::windows::core::IInspectable {
    fn from(value: &WebAccountProviderManageAccountOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebAccountProviderManageAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebAccountProviderManageAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WebAccountProviderManageAccountOperation> for IWebAccountProviderOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderManageAccountOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderManageAccountOperation> for IWebAccountProviderOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderManageAccountOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderOperation> for WebAccountProviderManageAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderOperation> for &WebAccountProviderManageAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderOperation> {
        ::core::convert::TryInto::<IWebAccountProviderOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebAccountProviderManageAccountOperation {}
unsafe impl ::core::marker::Sync for WebAccountProviderManageAccountOperation {}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for WebAccountProviderOperationKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for WebAccountProviderOperationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountProviderOperationKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebAccountProviderOperationKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Provider.WebAccountProviderOperationKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebAccountProviderRequestTokenOperation(::windows::core::IUnknown);
impl WebAccountProviderRequestTokenOperation {
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn ReportCompleted(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReportCompleted)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<'a, Param0: ::windows::core::IntoParam<'a, super::Core::WebProviderError>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReportError)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<WebAccountProviderOperationKind> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__: WebAccountProviderOperationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountProviderOperationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn ProviderRequest(&self) -> ::windows::core::Result<WebProviderTokenRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderRequest)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebProviderTokenRequest>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProviderResponses(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<WebProviderTokenResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderResponses)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVector<WebProviderTokenResponse>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCacheExpirationTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCacheExpirationTime)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CacheExpirationTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CacheExpirationTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn ReportUserCanceled(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderUIReportOperation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReportUserCanceled)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for WebAccountProviderRequestTokenOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for WebAccountProviderRequestTokenOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Provider.WebAccountProviderRequestTokenOperation;{95c613be-2034-4c38-9434-d26c14b2b4b2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebAccountProviderRequestTokenOperation {
    type Vtable = IWebAccountProviderTokenOperation_Vtbl;
    const IID: ::windows::core::GUID = <IWebAccountProviderTokenOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebAccountProviderRequestTokenOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderRequestTokenOperation";
}
impl ::core::convert::From<WebAccountProviderRequestTokenOperation> for ::windows::core::IUnknown {
    fn from(value: WebAccountProviderRequestTokenOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderRequestTokenOperation> for ::windows::core::IUnknown {
    fn from(value: &WebAccountProviderRequestTokenOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebAccountProviderRequestTokenOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebAccountProviderRequestTokenOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebAccountProviderRequestTokenOperation> for ::windows::core::IInspectable {
    fn from(value: WebAccountProviderRequestTokenOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderRequestTokenOperation> for ::windows::core::IInspectable {
    fn from(value: &WebAccountProviderRequestTokenOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebAccountProviderRequestTokenOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebAccountProviderRequestTokenOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WebAccountProviderRequestTokenOperation> for IWebAccountProviderBaseReportOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderRequestTokenOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderRequestTokenOperation> for IWebAccountProviderBaseReportOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderRequestTokenOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderBaseReportOperation> for WebAccountProviderRequestTokenOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderBaseReportOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderBaseReportOperation> for &WebAccountProviderRequestTokenOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderBaseReportOperation> {
        ::core::convert::TryInto::<IWebAccountProviderBaseReportOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<WebAccountProviderRequestTokenOperation> for IWebAccountProviderOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderRequestTokenOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderRequestTokenOperation> for IWebAccountProviderOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderRequestTokenOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderOperation> for WebAccountProviderRequestTokenOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderOperation> for &WebAccountProviderRequestTokenOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderOperation> {
        ::core::convert::TryInto::<IWebAccountProviderOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<WebAccountProviderRequestTokenOperation> for IWebAccountProviderTokenOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderRequestTokenOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderRequestTokenOperation> for IWebAccountProviderTokenOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderRequestTokenOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderTokenOperation> for WebAccountProviderRequestTokenOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderTokenOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderTokenOperation> for &WebAccountProviderRequestTokenOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderTokenOperation> {
        ::core::convert::TryInto::<IWebAccountProviderTokenOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<WebAccountProviderRequestTokenOperation> for IWebAccountProviderUIReportOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderRequestTokenOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderRequestTokenOperation> for IWebAccountProviderUIReportOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderRequestTokenOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderUIReportOperation> for WebAccountProviderRequestTokenOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderUIReportOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderUIReportOperation> for &WebAccountProviderRequestTokenOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderUIReportOperation> {
        ::core::convert::TryInto::<IWebAccountProviderUIReportOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebAccountProviderRequestTokenOperation {}
unsafe impl ::core::marker::Sync for WebAccountProviderRequestTokenOperation {}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebAccountProviderRetrieveCookiesOperation(::windows::core::IUnknown);
impl WebAccountProviderRetrieveCookiesOperation {
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn ReportCompleted(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReportCompleted)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<'a, Param0: ::windows::core::IntoParam<'a, super::Core::WebProviderError>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReportError)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<WebAccountProviderOperationKind> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__: WebAccountProviderOperationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountProviderOperationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Context(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Context)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation_Collections\"`, `\"Web_Http\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_Http"))]
    pub fn Cookies(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<super::super::super::super::Web::Http::HttpCookie>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cookies)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVector<super::super::super::super::Web::Http::HttpCookie>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Uri>>(&self, uri: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUri)(::core::mem::transmute_copy(this), uri.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ApplicationCallbackUri(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ApplicationCallbackUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Uri>(result__)
        }
    }
}
impl ::core::clone::Clone for WebAccountProviderRetrieveCookiesOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for WebAccountProviderRetrieveCookiesOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Provider.WebAccountProviderRetrieveCookiesOperation;{5a040441-0fa3-4ab1-a01c-20b110358594})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebAccountProviderRetrieveCookiesOperation {
    type Vtable = IWebAccountProviderRetrieveCookiesOperation_Vtbl;
    const IID: ::windows::core::GUID = <IWebAccountProviderRetrieveCookiesOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebAccountProviderRetrieveCookiesOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderRetrieveCookiesOperation";
}
impl ::core::convert::From<WebAccountProviderRetrieveCookiesOperation> for ::windows::core::IUnknown {
    fn from(value: WebAccountProviderRetrieveCookiesOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderRetrieveCookiesOperation> for ::windows::core::IUnknown {
    fn from(value: &WebAccountProviderRetrieveCookiesOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebAccountProviderRetrieveCookiesOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebAccountProviderRetrieveCookiesOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebAccountProviderRetrieveCookiesOperation> for ::windows::core::IInspectable {
    fn from(value: WebAccountProviderRetrieveCookiesOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderRetrieveCookiesOperation> for ::windows::core::IInspectable {
    fn from(value: &WebAccountProviderRetrieveCookiesOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebAccountProviderRetrieveCookiesOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebAccountProviderRetrieveCookiesOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WebAccountProviderRetrieveCookiesOperation> for IWebAccountProviderBaseReportOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderRetrieveCookiesOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderRetrieveCookiesOperation> for IWebAccountProviderBaseReportOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderRetrieveCookiesOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderBaseReportOperation> for WebAccountProviderRetrieveCookiesOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderBaseReportOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderBaseReportOperation> for &WebAccountProviderRetrieveCookiesOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderBaseReportOperation> {
        ::core::convert::TryInto::<IWebAccountProviderBaseReportOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<WebAccountProviderRetrieveCookiesOperation> for IWebAccountProviderOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderRetrieveCookiesOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderRetrieveCookiesOperation> for IWebAccountProviderOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderRetrieveCookiesOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderOperation> for WebAccountProviderRetrieveCookiesOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderOperation> for &WebAccountProviderRetrieveCookiesOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderOperation> {
        ::core::convert::TryInto::<IWebAccountProviderOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebAccountProviderRetrieveCookiesOperation {}
unsafe impl ::core::marker::Sync for WebAccountProviderRetrieveCookiesOperation {}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebAccountProviderSignOutAccountOperation(::windows::core::IUnknown);
impl WebAccountProviderSignOutAccountOperation {
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn ReportCompleted(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReportCompleted)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ReportError<'a, Param0: ::windows::core::IntoParam<'a, super::Core::WebProviderError>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderBaseReportOperation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReportError)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<WebAccountProviderOperationKind> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderOperation>(self)?;
        unsafe {
            let mut result__: WebAccountProviderOperationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountProviderOperationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn WebAccount(&self) -> ::windows::core::Result<super::super::super::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WebAccount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Credentials::WebAccount>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ApplicationCallbackUri(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ApplicationCallbackUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn ClientId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClientId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for WebAccountProviderSignOutAccountOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for WebAccountProviderSignOutAccountOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Provider.WebAccountProviderSignOutAccountOperation;{b890e21d-0c55-47bc-8c72-04a6fc7cac07})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebAccountProviderSignOutAccountOperation {
    type Vtable = IWebAccountProviderSignOutAccountOperation_Vtbl;
    const IID: ::windows::core::GUID = <IWebAccountProviderSignOutAccountOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebAccountProviderSignOutAccountOperation {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderSignOutAccountOperation";
}
impl ::core::convert::From<WebAccountProviderSignOutAccountOperation> for ::windows::core::IUnknown {
    fn from(value: WebAccountProviderSignOutAccountOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderSignOutAccountOperation> for ::windows::core::IUnknown {
    fn from(value: &WebAccountProviderSignOutAccountOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebAccountProviderSignOutAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebAccountProviderSignOutAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebAccountProviderSignOutAccountOperation> for ::windows::core::IInspectable {
    fn from(value: WebAccountProviderSignOutAccountOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderSignOutAccountOperation> for ::windows::core::IInspectable {
    fn from(value: &WebAccountProviderSignOutAccountOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebAccountProviderSignOutAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebAccountProviderSignOutAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WebAccountProviderSignOutAccountOperation> for IWebAccountProviderBaseReportOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderSignOutAccountOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderSignOutAccountOperation> for IWebAccountProviderBaseReportOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderSignOutAccountOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderBaseReportOperation> for WebAccountProviderSignOutAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderBaseReportOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderBaseReportOperation> for &WebAccountProviderSignOutAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderBaseReportOperation> {
        ::core::convert::TryInto::<IWebAccountProviderBaseReportOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<WebAccountProviderSignOutAccountOperation> for IWebAccountProviderOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderSignOutAccountOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderSignOutAccountOperation> for IWebAccountProviderOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderSignOutAccountOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderOperation> for WebAccountProviderSignOutAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderOperation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderOperation> for &WebAccountProviderSignOutAccountOperation {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderOperation> {
        ::core::convert::TryInto::<IWebAccountProviderOperation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebAccountProviderSignOutAccountOperation {}
unsafe impl ::core::marker::Sync for WebAccountProviderSignOutAccountOperation {}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebAccountProviderTriggerDetails(::windows::core::IUnknown);
impl WebAccountProviderTriggerDetails {
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn Operation(&self) -> ::windows::core::Result<IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Operation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IWebAccountProviderOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IWebAccountProviderTokenObjects2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).User)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::System::User>(result__)
        }
    }
}
impl ::core::clone::Clone for WebAccountProviderTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for WebAccountProviderTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Provider.WebAccountProviderTriggerDetails;{408f284b-1328-42db-89a4-0bce7a717d8e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebAccountProviderTriggerDetails {
    type Vtable = IWebAccountProviderTokenObjects_Vtbl;
    const IID: ::windows::core::GUID = <IWebAccountProviderTokenObjects as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebAccountProviderTriggerDetails {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebAccountProviderTriggerDetails";
}
impl ::core::convert::From<WebAccountProviderTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: WebAccountProviderTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &WebAccountProviderTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebAccountProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebAccountProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebAccountProviderTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: WebAccountProviderTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &WebAccountProviderTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebAccountProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebAccountProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WebAccountProviderTriggerDetails> for IWebAccountProviderTokenObjects {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderTriggerDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderTriggerDetails> for IWebAccountProviderTokenObjects {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderTriggerDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderTokenObjects> for WebAccountProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderTokenObjects> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderTokenObjects> for &WebAccountProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderTokenObjects> {
        ::core::convert::TryInto::<IWebAccountProviderTokenObjects>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<WebAccountProviderTriggerDetails> for IWebAccountProviderTokenObjects2 {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderTriggerDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderTriggerDetails> for IWebAccountProviderTokenObjects2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderTriggerDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderTokenObjects2> for WebAccountProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderTokenObjects2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderTokenObjects2> for &WebAccountProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderTokenObjects2> {
        ::core::convert::TryInto::<IWebAccountProviderTokenObjects2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebAccountProviderTriggerDetails {}
unsafe impl ::core::marker::Sync for WebAccountProviderTriggerDetails {}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for WebAccountScope {
    type Abi = Self;
}
impl ::core::fmt::Debug for WebAccountScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountScope").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebAccountScope {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Provider.WebAccountScope;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for WebAccountSelectionOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for WebAccountSelectionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountSelectionOptions").field(&self.0).finish()
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
unsafe impl ::windows::core::RuntimeType for WebAccountSelectionOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Provider.WebAccountSelectionOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebProviderTokenRequest(::windows::core::IUnknown);
impl WebProviderTokenRequest {
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ClientRequest(&self) -> ::windows::core::Result<super::Core::WebTokenRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClientRequest)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Core::WebTokenRequest>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation_Collections\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    pub fn WebAccounts(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WebAccounts)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn WebAccountSelectionOptions(&self) -> ::windows::core::Result<WebAccountSelectionOptions> {
        let this = self;
        unsafe {
            let mut result__: WebAccountSelectionOptions = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WebAccountSelectionOptions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountSelectionOptions>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ApplicationCallbackUri(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ApplicationCallbackUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`, `\"Security_Cryptography_Core\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Core"))]
    pub fn GetApplicationTokenBindingKeyAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Uri>>(&self, keytype: super::TokenBindingKeyType, target: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Cryptography::Core::CryptographicKey>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetApplicationTokenBindingKeyAsync)(::core::mem::transmute_copy(this), keytype, target.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Cryptography::Core::CryptographicKey>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetApplicationTokenBindingKeyIdAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Uri>>(&self, keytype: super::TokenBindingKeyType, target: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Storage::Streams::IBuffer>> {
        let this = &::windows::core::Interface::cast::<IWebProviderTokenRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetApplicationTokenBindingKeyIdAsync)(::core::mem::transmute_copy(this), keytype, target.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn ApplicationPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IWebProviderTokenRequest3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ApplicationPackageFamilyName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    pub fn ApplicationProcessName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IWebProviderTokenRequest3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ApplicationProcessName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CheckApplicationForCapabilityAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, capabilityname: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IWebProviderTokenRequest3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CheckApplicationForCapabilityAsync)(::core::mem::transmute_copy(this), capabilityname.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for WebProviderTokenRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for WebProviderTokenRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Provider.WebProviderTokenRequest;{1e18778b-8805-454b-9f11-468d2af1095a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebProviderTokenRequest {
    type Vtable = IWebProviderTokenRequest_Vtbl;
    const IID: ::windows::core::GUID = <IWebProviderTokenRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebProviderTokenRequest {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebProviderTokenRequest";
}
impl ::core::convert::From<WebProviderTokenRequest> for ::windows::core::IUnknown {
    fn from(value: WebProviderTokenRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebProviderTokenRequest> for ::windows::core::IUnknown {
    fn from(value: &WebProviderTokenRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebProviderTokenRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebProviderTokenRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebProviderTokenRequest> for ::windows::core::IInspectable {
    fn from(value: WebProviderTokenRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebProviderTokenRequest> for ::windows::core::IInspectable {
    fn from(value: &WebProviderTokenRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebProviderTokenRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebProviderTokenRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WebProviderTokenRequest {}
unsafe impl ::core::marker::Sync for WebProviderTokenRequest {}
#[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
#[repr(transparent)]
pub struct WebProviderTokenResponse(::windows::core::IUnknown);
impl WebProviderTokenResponse {
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn ClientResponse(&self) -> ::windows::core::Result<super::Core::WebTokenResponse> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClientResponse)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Core::WebTokenResponse>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`, `\"Security_Authentication_Web_Core\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Core")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::Core::WebTokenResponse>>(webtokenresponse: Param0) -> ::windows::core::Result<WebProviderTokenResponse> {
        Self::IWebProviderTokenResponseFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), webtokenresponse.into_param().abi(), &mut result__).from_abi::<WebProviderTokenResponse>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebProviderTokenResponseFactory<R, F: FnOnce(&IWebProviderTokenResponseFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebProviderTokenResponse, IWebProviderTokenResponseFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WebProviderTokenResponse {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for WebProviderTokenResponse {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Provider.WebProviderTokenResponse;{ef213793-ef55-4186-b7ce-8cb2e7f9849e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebProviderTokenResponse {
    type Vtable = IWebProviderTokenResponse_Vtbl;
    const IID: ::windows::core::GUID = <IWebProviderTokenResponse as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebProviderTokenResponse {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Provider.WebProviderTokenResponse";
}
impl ::core::convert::From<WebProviderTokenResponse> for ::windows::core::IUnknown {
    fn from(value: WebProviderTokenResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebProviderTokenResponse> for ::windows::core::IUnknown {
    fn from(value: &WebProviderTokenResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebProviderTokenResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebProviderTokenResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebProviderTokenResponse> for ::windows::core::IInspectable {
    fn from(value: WebProviderTokenResponse) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebProviderTokenResponse> for ::windows::core::IInspectable {
    fn from(value: &WebProviderTokenResponse) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebProviderTokenResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebProviderTokenResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WebProviderTokenResponse {}
unsafe impl ::core::marker::Sync for WebProviderTokenResponse {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
