#[cfg(feature = "Security_Credentials_UI")]
pub mod UI;
#[doc(hidden)]
#[repr(transparent)]
pub struct ICredentialFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICredentialFactory {
    type Vtable = ICredentialFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54ef13a1_bf26_47b5_97dd_de779b7cad58);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreatePasswordCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyCredential(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyCredential {
    type Vtable = IKeyCredential_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9585ef8d_457b_4847_b11a_fa960bbdb138);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredential_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub RetrievePublicKeyWithDefaultBlobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RetrievePublicKeyWithDefaultBlobType: usize,
    #[cfg(all(feature = "Security_Cryptography_Core", feature = "Storage_Streams"))]
    pub RetrievePublicKeyWithBlobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blobtype: super::Cryptography::Core::CryptographicPublicKeyBlobType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Security_Cryptography_Core", feature = "Storage_Streams")))]
    RetrievePublicKeyWithBlobType: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub RequestSignAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    RequestSignAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAttestationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAttestationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyCredentialAttestationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyCredentialAttestationResult {
    type Vtable = IKeyCredentialAttestationResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78aab3a1_a3c1_4103_b6cc_472c44171cbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredentialAttestationResult_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CertificateChainBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CertificateChainBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AttestationBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AttestationBuffer: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut KeyCredentialAttestationStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyCredentialManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyCredentialManagerStatics {
    type Vtable = IKeyCredentialManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6aac468b_0ef1_4ce0_8290_4106da6a63b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredentialManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub IsSupportedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsSupportedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RenewAttestationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RenewAttestationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestCreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: KeyCredentialCreationOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestCreateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyCredentialOperationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyCredentialOperationResult {
    type Vtable = IKeyCredentialOperationResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf53786c1_5261_4cdd_976d_cc909ac71620);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredentialOperationResult_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Result: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut KeyCredentialStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyCredentialRetrievalResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyCredentialRetrievalResult {
    type Vtable = IKeyCredentialRetrievalResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58cd7703_8d87_4249_9b58_f6598cc9644e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredentialRetrievalResult_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Credential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut KeyCredentialStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPasswordCredential(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPasswordCredential {
    type Vtable = IPasswordCredential_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ab18989_c720_41a7_a6c1_feadb36329a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPasswordCredential_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Resource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Password: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RetrievePassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPasswordVault(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPasswordVault {
    type Vtable = IPasswordVault_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61fd2c0b_c8d4_48c1_a54f_bc5a64205af2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPasswordVault_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credential: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credential: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Retrieve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllByResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllByResource: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllByUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllByUserName: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RetrieveAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RetrieveAll: usize,
}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
pub struct IWebAccount(::windows::core::IUnknown);
impl IWebAccount {
    pub fn WebAccountProvider(&self) -> ::windows::core::Result<WebAccountProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).WebAccountProvider)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebAccountProvider>(result__)
        }
    }
    pub fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UserName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn State(&self) -> ::windows::core::Result<WebAccountState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebAccountState>(result__)
        }
    }
}
impl ::core::convert::From<IWebAccount> for ::windows::core::IUnknown {
    fn from(value: IWebAccount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWebAccount> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWebAccount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccount> for ::windows::core::IUnknown {
    fn from(value: &IWebAccount) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IWebAccount> for ::windows::core::IInspectable {
    fn from(value: IWebAccount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWebAccount> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IWebAccount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccount> for ::windows::core::IInspectable {
    fn from(value: &IWebAccount) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWebAccount {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebAccount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAccount {}
impl ::core::fmt::Debug for IWebAccount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebAccount").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IWebAccount {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{69473eb2-8031-49be-80bb-96cb46d99aba}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IWebAccount {
    type Vtable = IWebAccount_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69473eb2_8031_49be_80bb_96cb46d99aba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccount_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub WebAccountProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebAccountState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccount2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccount2 {
    type Vtable = IWebAccount2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b56d6f8_990b_4eb5_94a7_5621f3a8b824);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccount2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetPictureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desizedsize: WebAccountPictureSize, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetPictureAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SignOutAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignOutAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SignOutWithClientIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignOutWithClientIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountFactory {
    type Vtable = IWebAccountFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac9afb39_1de9_4e92_b78f_0581a87f6e5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateWebAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountprovider: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, state: WebAccountState, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountProvider {
    type Vtable = IWebAccountProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29dcc8c3_7ab9_4a7c_a336_b942f9dbf7c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub IconUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    IconUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProvider2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountProvider2 {
    type Vtable = IWebAccountProvider2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a01eb05_4e42_41d4_b518_e008a5163614);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProvider2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DisplayPurpose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Authority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProvider3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountProvider3 {
    type Vtable = IWebAccountProvider3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda1c518b_970d_4d49_825c_f2706f8ca7fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProvider3_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProvider4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountProvider4 {
    type Vtable = IWebAccountProvider4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x718fd8db_e796_4210_b74e_84d29894b080);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProvider4_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsSystemProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProviderFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountProviderFactory {
    type Vtable = IWebAccountProviderFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d767df1_e1e1_4b9a_a774_5c7c7e3bf371);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CreateWebAccountProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, iconuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWebAccountProvider: usize,
}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
pub struct KeyCredential(::windows::core::IUnknown);
impl KeyCredential {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RetrievePublicKeyWithDefaultBlobType(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RetrievePublicKeyWithDefaultBlobType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Cryptography_Core\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Security_Cryptography_Core", feature = "Storage_Streams"))]
    pub fn RetrievePublicKeyWithBlobType(&self, blobtype: super::Cryptography::Core::CryptographicPublicKeyBlobType) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RetrievePublicKeyWithBlobType)(::windows::core::Interface::as_raw(this), blobtype, result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn RequestSignAsync<'a, P0, E0>(&self, data: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialOperationResult>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RequestSignAsync)(::windows::core::Interface::as_raw(this), data.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<KeyCredentialOperationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAttestationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialAttestationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetAttestationAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<KeyCredentialAttestationResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for KeyCredential {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyCredential {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyCredential {}
impl ::core::fmt::Debug for KeyCredential {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredential").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeyCredential {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.KeyCredential;{9585ef8d-457b-4847-b11a-fa960bbdb138})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for KeyCredential {
    type Vtable = IKeyCredential_Vtbl;
    const IID: ::windows::core::GUID = <IKeyCredential as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for KeyCredential {
    const NAME: &'static str = "Windows.Security.Credentials.KeyCredential";
}
impl ::core::convert::From<KeyCredential> for ::windows::core::IUnknown {
    fn from(value: KeyCredential) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyCredential> for ::windows::core::IUnknown {
    fn from(value: &KeyCredential) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&KeyCredential> for &::windows::core::IUnknown {
    fn from(value: &KeyCredential) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<KeyCredential> for ::windows::core::IInspectable {
    fn from(value: KeyCredential) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyCredential> for ::windows::core::IInspectable {
    fn from(value: &KeyCredential) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&KeyCredential> for &::windows::core::IInspectable {
    fn from(value: &KeyCredential) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for KeyCredential {}
unsafe impl ::core::marker::Sync for KeyCredential {}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
pub struct KeyCredentialAttestationResult(::windows::core::IUnknown);
impl KeyCredentialAttestationResult {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CertificateChainBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CertificateChainBuffer)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AttestationBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AttestationBuffer)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<KeyCredentialAttestationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<KeyCredentialAttestationStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for KeyCredentialAttestationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyCredentialAttestationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyCredentialAttestationResult {}
impl ::core::fmt::Debug for KeyCredentialAttestationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredentialAttestationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeyCredentialAttestationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.KeyCredentialAttestationResult;{78aab3a1-a3c1-4103-b6cc-472c44171cbb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for KeyCredentialAttestationResult {
    type Vtable = IKeyCredentialAttestationResult_Vtbl;
    const IID: ::windows::core::GUID = <IKeyCredentialAttestationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for KeyCredentialAttestationResult {
    const NAME: &'static str = "Windows.Security.Credentials.KeyCredentialAttestationResult";
}
impl ::core::convert::From<KeyCredentialAttestationResult> for ::windows::core::IUnknown {
    fn from(value: KeyCredentialAttestationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyCredentialAttestationResult> for ::windows::core::IUnknown {
    fn from(value: &KeyCredentialAttestationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&KeyCredentialAttestationResult> for &::windows::core::IUnknown {
    fn from(value: &KeyCredentialAttestationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<KeyCredentialAttestationResult> for ::windows::core::IInspectable {
    fn from(value: KeyCredentialAttestationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyCredentialAttestationResult> for ::windows::core::IInspectable {
    fn from(value: &KeyCredentialAttestationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&KeyCredentialAttestationResult> for &::windows::core::IInspectable {
    fn from(value: &KeyCredentialAttestationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for KeyCredentialAttestationResult {}
unsafe impl ::core::marker::Sync for KeyCredentialAttestationResult {}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KeyCredentialAttestationStatus(pub i32);
impl KeyCredentialAttestationStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const TemporaryFailure: Self = Self(3i32);
}
impl ::core::marker::Copy for KeyCredentialAttestationStatus {}
impl ::core::clone::Clone for KeyCredentialAttestationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KeyCredentialAttestationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KeyCredentialAttestationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for KeyCredentialAttestationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredentialAttestationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeyCredentialAttestationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.KeyCredentialAttestationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KeyCredentialCreationOption(pub i32);
impl KeyCredentialCreationOption {
    pub const ReplaceExisting: Self = Self(0i32);
    pub const FailIfExists: Self = Self(1i32);
}
impl ::core::marker::Copy for KeyCredentialCreationOption {}
impl ::core::clone::Clone for KeyCredentialCreationOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KeyCredentialCreationOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KeyCredentialCreationOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for KeyCredentialCreationOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredentialCreationOption").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeyCredentialCreationOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.KeyCredentialCreationOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
pub struct KeyCredentialManager;
impl KeyCredentialManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsSupportedAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsSupportedAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenewAttestationAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RenewAttestationAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestCreateAsync(name: &::windows::core::HSTRING, option: KeyCredentialCreationOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialRetrievalResult>> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RequestCreateAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), option, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<KeyCredentialRetrievalResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenAsync(name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialRetrievalResult>> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OpenAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<KeyCredentialRetrievalResult>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKeyCredentialManagerStatics<R, F: FnOnce(&IKeyCredentialManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<KeyCredentialManager, IKeyCredentialManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for KeyCredentialManager {
    const NAME: &'static str = "Windows.Security.Credentials.KeyCredentialManager";
}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
pub struct KeyCredentialOperationResult(::windows::core::IUnknown);
impl KeyCredentialOperationResult {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Result(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Result)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<KeyCredentialStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<KeyCredentialStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for KeyCredentialOperationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyCredentialOperationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyCredentialOperationResult {}
impl ::core::fmt::Debug for KeyCredentialOperationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredentialOperationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeyCredentialOperationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.KeyCredentialOperationResult;{f53786c1-5261-4cdd-976d-cc909ac71620})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for KeyCredentialOperationResult {
    type Vtable = IKeyCredentialOperationResult_Vtbl;
    const IID: ::windows::core::GUID = <IKeyCredentialOperationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for KeyCredentialOperationResult {
    const NAME: &'static str = "Windows.Security.Credentials.KeyCredentialOperationResult";
}
impl ::core::convert::From<KeyCredentialOperationResult> for ::windows::core::IUnknown {
    fn from(value: KeyCredentialOperationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyCredentialOperationResult> for ::windows::core::IUnknown {
    fn from(value: &KeyCredentialOperationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&KeyCredentialOperationResult> for &::windows::core::IUnknown {
    fn from(value: &KeyCredentialOperationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<KeyCredentialOperationResult> for ::windows::core::IInspectable {
    fn from(value: KeyCredentialOperationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyCredentialOperationResult> for ::windows::core::IInspectable {
    fn from(value: &KeyCredentialOperationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&KeyCredentialOperationResult> for &::windows::core::IInspectable {
    fn from(value: &KeyCredentialOperationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for KeyCredentialOperationResult {}
unsafe impl ::core::marker::Sync for KeyCredentialOperationResult {}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
pub struct KeyCredentialRetrievalResult(::windows::core::IUnknown);
impl KeyCredentialRetrievalResult {
    pub fn Credential(&self) -> ::windows::core::Result<KeyCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Credential)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<KeyCredential>(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<KeyCredentialStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<KeyCredentialStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for KeyCredentialRetrievalResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for KeyCredentialRetrievalResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyCredentialRetrievalResult {}
impl ::core::fmt::Debug for KeyCredentialRetrievalResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredentialRetrievalResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeyCredentialRetrievalResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.KeyCredentialRetrievalResult;{58cd7703-8d87-4249-9b58-f6598cc9644e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for KeyCredentialRetrievalResult {
    type Vtable = IKeyCredentialRetrievalResult_Vtbl;
    const IID: ::windows::core::GUID = <IKeyCredentialRetrievalResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for KeyCredentialRetrievalResult {
    const NAME: &'static str = "Windows.Security.Credentials.KeyCredentialRetrievalResult";
}
impl ::core::convert::From<KeyCredentialRetrievalResult> for ::windows::core::IUnknown {
    fn from(value: KeyCredentialRetrievalResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyCredentialRetrievalResult> for ::windows::core::IUnknown {
    fn from(value: &KeyCredentialRetrievalResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&KeyCredentialRetrievalResult> for &::windows::core::IUnknown {
    fn from(value: &KeyCredentialRetrievalResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<KeyCredentialRetrievalResult> for ::windows::core::IInspectable {
    fn from(value: KeyCredentialRetrievalResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&KeyCredentialRetrievalResult> for ::windows::core::IInspectable {
    fn from(value: &KeyCredentialRetrievalResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&KeyCredentialRetrievalResult> for &::windows::core::IInspectable {
    fn from(value: &KeyCredentialRetrievalResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for KeyCredentialRetrievalResult {}
unsafe impl ::core::marker::Sync for KeyCredentialRetrievalResult {}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KeyCredentialStatus(pub i32);
impl KeyCredentialStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const NotFound: Self = Self(2i32);
    pub const UserCanceled: Self = Self(3i32);
    pub const UserPrefersPassword: Self = Self(4i32);
    pub const CredentialAlreadyExists: Self = Self(5i32);
    pub const SecurityDeviceLocked: Self = Self(6i32);
}
impl ::core::marker::Copy for KeyCredentialStatus {}
impl ::core::clone::Clone for KeyCredentialStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KeyCredentialStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KeyCredentialStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for KeyCredentialStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredentialStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeyCredentialStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.KeyCredentialStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
pub struct PasswordCredential(::windows::core::IUnknown);
impl PasswordCredential {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PasswordCredential, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CreatePasswordCredential(resource: &::windows::core::HSTRING, username: &::windows::core::HSTRING, password: &::windows::core::HSTRING) -> ::windows::core::Result<PasswordCredential> {
        Self::ICredentialFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreatePasswordCredential)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(resource), ::core::mem::transmute_copy(username), ::core::mem::transmute_copy(password), result__.as_mut_ptr()).from_abi::<PasswordCredential>(result__)
        })
    }
    pub fn Resource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Resource)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetResource(&self, resource: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetResource)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(resource)).ok() }
    }
    pub fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UserName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetUserName(&self, username: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUserName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(username)).ok() }
    }
    pub fn Password(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Password)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetPassword(&self, password: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPassword)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(password)).ok() }
    }
    pub fn RetrievePassword(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RetrievePassword)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc(hidden)]
    pub fn ICredentialFactory<R, F: FnOnce(&ICredentialFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PasswordCredential, ICredentialFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PasswordCredential {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PasswordCredential {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PasswordCredential {}
impl ::core::fmt::Debug for PasswordCredential {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PasswordCredential").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PasswordCredential {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.PasswordCredential;{6ab18989-c720-41a7-a6c1-feadb36329a0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PasswordCredential {
    type Vtable = IPasswordCredential_Vtbl;
    const IID: ::windows::core::GUID = <IPasswordCredential as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PasswordCredential {
    const NAME: &'static str = "Windows.Security.Credentials.PasswordCredential";
}
impl ::core::convert::From<PasswordCredential> for ::windows::core::IUnknown {
    fn from(value: PasswordCredential) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PasswordCredential> for ::windows::core::IUnknown {
    fn from(value: &PasswordCredential) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PasswordCredential> for &::windows::core::IUnknown {
    fn from(value: &PasswordCredential) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PasswordCredential> for ::windows::core::IInspectable {
    fn from(value: PasswordCredential) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PasswordCredential> for ::windows::core::IInspectable {
    fn from(value: &PasswordCredential) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PasswordCredential> for &::windows::core::IInspectable {
    fn from(value: &PasswordCredential) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PasswordCredential {}
unsafe impl ::core::marker::Sync for PasswordCredential {}
#[doc = "*Required features: `\"Security_Credentials\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct PasswordCredentialPropertyStore(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl PasswordCredentialPropertyStore {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PasswordCredentialPropertyStore, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).First)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Lookup)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(key), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasKey)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(key), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetView)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<'a, P0>(&self, key: &::windows::core::HSTRING, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Insert)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(key), value.into().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Remove)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(key)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MapChanged<'a, P0>(&self, vhnd: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::Collections::MapChangedEventHandler<::windows::core::HSTRING, ::windows::core::IInspectable>>>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MapChanged)(::windows::core::Interface::as_raw(this), vhnd.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveMapChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMapChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for PasswordCredentialPropertyStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for PasswordCredentialPropertyStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for PasswordCredentialPropertyStore {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for PasswordCredentialPropertyStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PasswordCredentialPropertyStore").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for PasswordCredentialPropertyStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.PasswordCredentialPropertyStore;{8a43ed9f-f4e6-4421-acf9-1dab2986820c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for PasswordCredentialPropertyStore {
    type Vtable = super::super::Foundation::Collections::IPropertySet_Vtbl;
    const IID: ::windows::core::GUID = <super::super::Foundation::Collections::IPropertySet as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for PasswordCredentialPropertyStore {
    const NAME: &'static str = "Windows.Security.Credentials.PasswordCredentialPropertyStore";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for PasswordCredentialPropertyStore {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &PasswordCredentialPropertyStore {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PasswordCredentialPropertyStore> for ::windows::core::IUnknown {
    fn from(value: PasswordCredentialPropertyStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PasswordCredentialPropertyStore> for ::windows::core::IUnknown {
    fn from(value: &PasswordCredentialPropertyStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PasswordCredentialPropertyStore> for &::windows::core::IUnknown {
    fn from(value: &PasswordCredentialPropertyStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<PasswordCredentialPropertyStore> for ::windows::core::IInspectable {
    fn from(value: PasswordCredentialPropertyStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PasswordCredentialPropertyStore> for ::windows::core::IInspectable {
    fn from(value: &PasswordCredentialPropertyStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&PasswordCredentialPropertyStore> for &::windows::core::IInspectable {
    fn from(value: &PasswordCredentialPropertyStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PasswordCredentialPropertyStore> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> {
    type Error = ::windows::core::Error;
    fn try_from(value: PasswordCredentialPropertyStore) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PasswordCredentialPropertyStore> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PasswordCredentialPropertyStore) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::core::convert::TryFrom<&PasswordCredentialPropertyStore> for ::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PasswordCredentialPropertyStore) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PasswordCredentialPropertyStore> for super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: PasswordCredentialPropertyStore) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PasswordCredentialPropertyStore> for super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PasswordCredentialPropertyStore) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::core::convert::TryFrom<&PasswordCredentialPropertyStore> for ::windows::core::InParam<'a, super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PasswordCredentialPropertyStore) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PasswordCredentialPropertyStore> for super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: PasswordCredentialPropertyStore) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PasswordCredentialPropertyStore> for super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PasswordCredentialPropertyStore) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::core::convert::TryFrom<&PasswordCredentialPropertyStore> for ::windows::core::InParam<'a, super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PasswordCredentialPropertyStore) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<PasswordCredentialPropertyStore> for super::super::Foundation::Collections::IPropertySet {
    type Error = ::windows::core::Error;
    fn try_from(value: PasswordCredentialPropertyStore) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&PasswordCredentialPropertyStore> for super::super::Foundation::Collections::IPropertySet {
    type Error = ::windows::core::Error;
    fn try_from(value: &PasswordCredentialPropertyStore) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::core::convert::TryFrom<&PasswordCredentialPropertyStore> for ::windows::core::InParam<'a, super::super::Foundation::Collections::IPropertySet> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PasswordCredentialPropertyStore) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for PasswordCredentialPropertyStore {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for PasswordCredentialPropertyStore {}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
pub struct PasswordVault(::windows::core::IUnknown);
impl PasswordVault {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PasswordVault, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Add<'a, P0>(&self, credential: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PasswordCredential>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Add)(::windows::core::Interface::as_raw(this), credential.into().abi()).ok() }
    }
    pub fn Remove<'a, P0>(&self, credential: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, PasswordCredential>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Remove)(::windows::core::Interface::as_raw(this), credential.into().abi()).ok() }
    }
    pub fn Retrieve(&self, resource: &::windows::core::HSTRING, username: &::windows::core::HSTRING) -> ::windows::core::Result<PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Retrieve)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(resource), ::core::mem::transmute_copy(username), result__.as_mut_ptr()).from_abi::<PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllByResource(&self, resource: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PasswordCredential>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FindAllByResource)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(resource), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PasswordCredential>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllByUserName(&self, username: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PasswordCredential>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FindAllByUserName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(username), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PasswordCredential>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RetrieveAll(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PasswordCredential>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RetrieveAll)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PasswordCredential>>(result__)
        }
    }
}
impl ::core::clone::Clone for PasswordVault {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PasswordVault {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PasswordVault {}
impl ::core::fmt::Debug for PasswordVault {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PasswordVault").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PasswordVault {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.PasswordVault;{61fd2c0b-c8d4-48c1-a54f-bc5a64205af2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PasswordVault {
    type Vtable = IPasswordVault_Vtbl;
    const IID: ::windows::core::GUID = <IPasswordVault as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PasswordVault {
    const NAME: &'static str = "Windows.Security.Credentials.PasswordVault";
}
impl ::core::convert::From<PasswordVault> for ::windows::core::IUnknown {
    fn from(value: PasswordVault) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PasswordVault> for ::windows::core::IUnknown {
    fn from(value: &PasswordVault) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PasswordVault> for &::windows::core::IUnknown {
    fn from(value: &PasswordVault) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PasswordVault> for ::windows::core::IInspectable {
    fn from(value: PasswordVault) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PasswordVault> for ::windows::core::IInspectable {
    fn from(value: &PasswordVault) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PasswordVault> for &::windows::core::IInspectable {
    fn from(value: &PasswordVault) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PasswordVault {}
unsafe impl ::core::marker::Sync for PasswordVault {}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
pub struct WebAccount(::windows::core::IUnknown);
impl WebAccount {
    pub fn WebAccountProvider(&self) -> ::windows::core::Result<WebAccountProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).WebAccountProvider)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebAccountProvider>(result__)
        }
    }
    pub fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UserName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn State(&self) -> ::windows::core::Result<WebAccountState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebAccountState>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetPictureAsync(&self, desizedsize: WebAccountPictureSize) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>> {
        let this = &::windows::core::Interface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetPictureAsync)(::windows::core::Interface::as_raw(this), desizedsize, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SignOutAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SignOutAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SignOutWithClientIdAsync(&self, clientid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SignOutWithClientIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(clientid), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn CreateWebAccount<'a, P0>(webaccountprovider: P0, username: &::windows::core::HSTRING, state: WebAccountState) -> ::windows::core::Result<WebAccount>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, WebAccountProvider>>,
    {
        Self::IWebAccountFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWebAccount)(::windows::core::Interface::as_raw(this), webaccountprovider.into().abi(), ::core::mem::transmute_copy(username), state, result__.as_mut_ptr()).from_abi::<WebAccount>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebAccountFactory<R, F: FnOnce(&IWebAccountFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WebAccount, IWebAccountFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for WebAccount {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebAccount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccount {}
impl ::core::fmt::Debug for WebAccount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccount").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebAccount {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.WebAccount;{69473eb2-8031-49be-80bb-96cb46d99aba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebAccount {
    type Vtable = IWebAccount_Vtbl;
    const IID: ::windows::core::GUID = <IWebAccount as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebAccount {
    const NAME: &'static str = "Windows.Security.Credentials.WebAccount";
}
impl ::core::convert::From<WebAccount> for ::windows::core::IUnknown {
    fn from(value: WebAccount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccount> for ::windows::core::IUnknown {
    fn from(value: &WebAccount) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WebAccount> for &::windows::core::IUnknown {
    fn from(value: &WebAccount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<WebAccount> for ::windows::core::IInspectable {
    fn from(value: WebAccount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccount> for ::windows::core::IInspectable {
    fn from(value: &WebAccount) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WebAccount> for &::windows::core::IInspectable {
    fn from(value: &WebAccount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<WebAccount> for IWebAccount {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccount) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccount> for IWebAccount {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccount) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&WebAccount> for ::windows::core::InParam<'a, IWebAccount> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccount) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for WebAccount {}
unsafe impl ::core::marker::Sync for WebAccount {}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WebAccountPictureSize(pub i32);
impl WebAccountPictureSize {
    pub const Size64x64: Self = Self(64i32);
    pub const Size208x208: Self = Self(208i32);
    pub const Size424x424: Self = Self(424i32);
    pub const Size1080x1080: Self = Self(1080i32);
}
impl ::core::marker::Copy for WebAccountPictureSize {}
impl ::core::clone::Clone for WebAccountPictureSize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WebAccountPictureSize {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WebAccountPictureSize {
    type Abi = Self;
}
impl ::core::fmt::Debug for WebAccountPictureSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountPictureSize").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebAccountPictureSize {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.WebAccountPictureSize;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
pub struct WebAccountProvider(::windows::core::IUnknown);
impl WebAccountProvider {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn IconUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IconUri)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    pub fn DisplayPurpose(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IWebAccountProvider2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DisplayPurpose)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Authority(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IWebAccountProvider2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Authority)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IWebAccountProvider3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn IsSystemProvider(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IWebAccountProvider4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsSystemProvider)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWebAccountProvider<'a, P0>(id: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING, iconuri: P0) -> ::windows::core::Result<WebAccountProvider>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::Uri>>,
    {
        Self::IWebAccountProviderFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWebAccountProvider)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), ::core::mem::transmute_copy(displayname), iconuri.into().abi(), result__.as_mut_ptr()).from_abi::<WebAccountProvider>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebAccountProviderFactory<R, F: FnOnce(&IWebAccountProviderFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WebAccountProvider, IWebAccountProviderFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for WebAccountProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebAccountProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountProvider {}
impl ::core::fmt::Debug for WebAccountProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebAccountProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.WebAccountProvider;{29dcc8c3-7ab9-4a7c-a336-b942f9dbf7c7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebAccountProvider {
    type Vtable = IWebAccountProvider_Vtbl;
    const IID: ::windows::core::GUID = <IWebAccountProvider as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebAccountProvider {
    const NAME: &'static str = "Windows.Security.Credentials.WebAccountProvider";
}
impl ::core::convert::From<WebAccountProvider> for ::windows::core::IUnknown {
    fn from(value: WebAccountProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProvider> for ::windows::core::IUnknown {
    fn from(value: &WebAccountProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WebAccountProvider> for &::windows::core::IUnknown {
    fn from(value: &WebAccountProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<WebAccountProvider> for ::windows::core::IInspectable {
    fn from(value: WebAccountProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProvider> for ::windows::core::IInspectable {
    fn from(value: &WebAccountProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WebAccountProvider> for &::windows::core::IInspectable {
    fn from(value: &WebAccountProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for WebAccountProvider {}
unsafe impl ::core::marker::Sync for WebAccountProvider {}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WebAccountState(pub i32);
impl WebAccountState {
    pub const None: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
    pub const Error: Self = Self(2i32);
}
impl ::core::marker::Copy for WebAccountState {}
impl ::core::clone::Clone for WebAccountState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WebAccountState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WebAccountState {
    type Abi = Self;
}
impl ::core::fmt::Debug for WebAccountState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebAccountState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.WebAccountState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
