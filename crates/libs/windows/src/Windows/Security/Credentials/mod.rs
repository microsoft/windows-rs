#[cfg(feature = "Security_Credentials_UI")]
pub mod UI;
#[doc(hidden)]
#[repr(transparent)]
pub struct ICredentialFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICredentialFactory {
    type Vtable = ICredentialFactory_Vtbl;
}
impl ::core::clone::Clone for ICredentialFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICredentialFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x54ef13a1_bf26_47b5_97dd_de779b7cad58);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreatePasswordCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::std::mem::MaybeUninit<::windows_core::HSTRING>, username: ::std::mem::MaybeUninit<::windows_core::HSTRING>, password: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyCredential(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyCredential {
    type Vtable = IKeyCredential_Vtbl;
}
impl ::core::clone::Clone for IKeyCredential {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IKeyCredential {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9585ef8d_457b_4847_b11a_fa960bbdb138);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredential_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub RetrievePublicKeyWithDefaultBlobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RetrievePublicKeyWithDefaultBlobType: usize,
    #[cfg(all(feature = "Security_Cryptography_Core", feature = "Storage_Streams"))]
    pub RetrievePublicKeyWithBlobType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blobtype: super::Cryptography::Core::CryptographicPublicKeyBlobType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Security_Cryptography_Core", feature = "Storage_Streams")))]
    RetrievePublicKeyWithBlobType: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub RequestSignAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    RequestSignAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAttestationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAttestationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyCredentialAttestationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyCredentialAttestationResult {
    type Vtable = IKeyCredentialAttestationResult_Vtbl;
}
impl ::core::clone::Clone for IKeyCredentialAttestationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IKeyCredentialAttestationResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x78aab3a1_a3c1_4103_b6cc_472c44171cbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredentialAttestationResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CertificateChainBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CertificateChainBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AttestationBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AttestationBuffer: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut KeyCredentialAttestationStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyCredentialManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyCredentialManagerStatics {
    type Vtable = IKeyCredentialManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IKeyCredentialManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IKeyCredentialManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6aac468b_0ef1_4ce0_8290_4106da6a63b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredentialManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub IsSupportedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsSupportedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RenewAttestationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RenewAttestationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestCreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, option: KeyCredentialCreationOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestCreateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub OpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyCredentialOperationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyCredentialOperationResult {
    type Vtable = IKeyCredentialOperationResult_Vtbl;
}
impl ::core::clone::Clone for IKeyCredentialOperationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IKeyCredentialOperationResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf53786c1_5261_4cdd_976d_cc909ac71620);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredentialOperationResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Result: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut KeyCredentialStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyCredentialRetrievalResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IKeyCredentialRetrievalResult {
    type Vtable = IKeyCredentialRetrievalResult_Vtbl;
}
impl ::core::clone::Clone for IKeyCredentialRetrievalResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IKeyCredentialRetrievalResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x58cd7703_8d87_4249_9b58_f6598cc9644e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredentialRetrievalResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Credential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut KeyCredentialStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPasswordCredential(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPasswordCredential {
    type Vtable = IPasswordCredential_Vtbl;
}
impl ::core::clone::Clone for IPasswordCredential {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPasswordCredential {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6ab18989_c720_41a7_a6c1_feadb36329a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPasswordCredential_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Resource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Password: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, password: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RetrievePassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPasswordVault(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPasswordVault {
    type Vtable = IPasswordVault_Vtbl;
}
impl ::core::clone::Clone for IPasswordVault {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPasswordVault {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x61fd2c0b_c8d4_48c1_a54f_bc5a64205af2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPasswordVault_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credential: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credential: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Retrieve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::std::mem::MaybeUninit<::windows_core::HSTRING>, username: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllByResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllByResource: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllByUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllByUserName: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RetrieveAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RetrieveAll: usize,
}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
pub struct IWebAccount(::windows_core::IUnknown);
impl IWebAccount {
    pub fn WebAccountProvider(&self) -> ::windows_core::Result<WebAccountProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WebAccountProvider)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UserName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<WebAccountState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IWebAccount, ::windows_core::IUnknown, ::windows_core::IInspectable);
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
impl ::windows_core::RuntimeType for IWebAccount {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{69473eb2-8031-49be-80bb-96cb46d99aba}");
}
unsafe impl ::windows_core::Interface for IWebAccount {
    type Vtable = IWebAccount_Vtbl;
}
impl ::core::clone::Clone for IWebAccount {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccount {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69473eb2_8031_49be_80bb_96cb46d99aba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccount_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub WebAccountProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebAccountState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccount2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccount2 {
    type Vtable = IWebAccount2_Vtbl;
}
impl ::core::clone::Clone for IWebAccount2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccount2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7b56d6f8_990b_4eb5_94a7_5621f3a8b824);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccount2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetPictureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desizedsize: WebAccountPictureSize, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetPictureAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SignOutAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignOutAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SignOutWithClientIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignOutWithClientIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountFactory {
    type Vtable = IWebAccountFactory_Vtbl;
}
impl ::core::clone::Clone for IWebAccountFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xac9afb39_1de9_4e92_b78f_0581a87f6e5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateWebAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountprovider: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows_core::HSTRING>, state: WebAccountState, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProvider(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountProvider {
    type Vtable = IWebAccountProvider_Vtbl;
}
impl ::core::clone::Clone for IWebAccountProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x29dcc8c3_7ab9_4a7c_a336_b942f9dbf7c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub IconUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    IconUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProvider2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountProvider2 {
    type Vtable = IWebAccountProvider2_Vtbl;
}
impl ::core::clone::Clone for IWebAccountProvider2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountProvider2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a01eb05_4e42_41d4_b518_e008a5163614);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProvider2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DisplayPurpose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Authority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProvider3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountProvider3 {
    type Vtable = IWebAccountProvider3_Vtbl;
}
impl ::core::clone::Clone for IWebAccountProvider3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountProvider3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda1c518b_970d_4d49_825c_f2706f8ca7fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProvider3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProvider4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountProvider4 {
    type Vtable = IWebAccountProvider4_Vtbl;
}
impl ::core::clone::Clone for IWebAccountProvider4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountProvider4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x718fd8db_e796_4210_b74e_84d29894b080);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProvider4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSystemProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProviderFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebAccountProviderFactory {
    type Vtable = IWebAccountProviderFactory_Vtbl;
}
impl ::core::clone::Clone for IWebAccountProviderFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IWebAccountProviderFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d767df1_e1e1_4b9a_a774_5c7c7e3bf371);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateWebAccountProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows_core::HSTRING>, displayname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, iconuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWebAccountProvider: usize,
}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
pub struct KeyCredential(::windows_core::IUnknown);
impl KeyCredential {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RetrievePublicKeyWithDefaultBlobType(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RetrievePublicKeyWithDefaultBlobType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Cryptography_Core\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Security_Cryptography_Core", feature = "Storage_Streams"))]
    pub fn RetrievePublicKeyWithBlobType(&self, blobtype: super::Cryptography::Core::CryptographicPublicKeyBlobType) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RetrievePublicKeyWithBlobType)(::windows_core::Interface::as_raw(this), blobtype, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn RequestSignAsync<P0>(&self, data: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialOperationResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestSignAsync)(::windows_core::Interface::as_raw(this), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAttestationAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialAttestationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAttestationAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for KeyCredential {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.KeyCredential;{9585ef8d-457b-4847-b11a-fa960bbdb138})");
}
impl ::core::clone::Clone for KeyCredential {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for KeyCredential {
    type Vtable = IKeyCredential_Vtbl;
}
unsafe impl ::windows_core::ComInterface for KeyCredential {
    const IID: ::windows_core::GUID = <IKeyCredential as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for KeyCredential {
    const NAME: &'static str = "Windows.Security.Credentials.KeyCredential";
}
::windows_core::imp::interface_hierarchy!(KeyCredential, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for KeyCredential {}
unsafe impl ::core::marker::Sync for KeyCredential {}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
pub struct KeyCredentialAttestationResult(::windows_core::IUnknown);
impl KeyCredentialAttestationResult {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CertificateChainBuffer(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CertificateChainBuffer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AttestationBuffer(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AttestationBuffer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<KeyCredentialAttestationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for KeyCredentialAttestationResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.KeyCredentialAttestationResult;{78aab3a1-a3c1-4103-b6cc-472c44171cbb})");
}
impl ::core::clone::Clone for KeyCredentialAttestationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for KeyCredentialAttestationResult {
    type Vtable = IKeyCredentialAttestationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for KeyCredentialAttestationResult {
    const IID: ::windows_core::GUID = <IKeyCredentialAttestationResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for KeyCredentialAttestationResult {
    const NAME: &'static str = "Windows.Security.Credentials.KeyCredentialAttestationResult";
}
::windows_core::imp::interface_hierarchy!(KeyCredentialAttestationResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for KeyCredentialAttestationResult {}
unsafe impl ::core::marker::Sync for KeyCredentialAttestationResult {}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
pub struct KeyCredentialManager;
impl KeyCredentialManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsSupportedAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSupportedAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenewAttestationAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RenewAttestationAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestCreateAsync(name: &::windows_core::HSTRING, option: KeyCredentialCreationOption) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialRetrievalResult>> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestCreateAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), option, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenAsync(name: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialRetrievalResult>> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(name: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeleteAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKeyCredentialManagerStatics<R, F: FnOnce(&IKeyCredentialManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<KeyCredentialManager, IKeyCredentialManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for KeyCredentialManager {
    const NAME: &'static str = "Windows.Security.Credentials.KeyCredentialManager";
}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
pub struct KeyCredentialOperationResult(::windows_core::IUnknown);
impl KeyCredentialOperationResult {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Result(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Result)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<KeyCredentialStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for KeyCredentialOperationResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.KeyCredentialOperationResult;{f53786c1-5261-4cdd-976d-cc909ac71620})");
}
impl ::core::clone::Clone for KeyCredentialOperationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for KeyCredentialOperationResult {
    type Vtable = IKeyCredentialOperationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for KeyCredentialOperationResult {
    const IID: ::windows_core::GUID = <IKeyCredentialOperationResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for KeyCredentialOperationResult {
    const NAME: &'static str = "Windows.Security.Credentials.KeyCredentialOperationResult";
}
::windows_core::imp::interface_hierarchy!(KeyCredentialOperationResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for KeyCredentialOperationResult {}
unsafe impl ::core::marker::Sync for KeyCredentialOperationResult {}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
pub struct KeyCredentialRetrievalResult(::windows_core::IUnknown);
impl KeyCredentialRetrievalResult {
    pub fn Credential(&self) -> ::windows_core::Result<KeyCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Credential)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows_core::Result<KeyCredentialStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for KeyCredentialRetrievalResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.KeyCredentialRetrievalResult;{58cd7703-8d87-4249-9b58-f6598cc9644e})");
}
impl ::core::clone::Clone for KeyCredentialRetrievalResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for KeyCredentialRetrievalResult {
    type Vtable = IKeyCredentialRetrievalResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for KeyCredentialRetrievalResult {
    const IID: ::windows_core::GUID = <IKeyCredentialRetrievalResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for KeyCredentialRetrievalResult {
    const NAME: &'static str = "Windows.Security.Credentials.KeyCredentialRetrievalResult";
}
::windows_core::imp::interface_hierarchy!(KeyCredentialRetrievalResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for KeyCredentialRetrievalResult {}
unsafe impl ::core::marker::Sync for KeyCredentialRetrievalResult {}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
pub struct PasswordCredential(::windows_core::IUnknown);
impl PasswordCredential {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PasswordCredential, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CreatePasswordCredential(resource: &::windows_core::HSTRING, username: &::windows_core::HSTRING, password: &::windows_core::HSTRING) -> ::windows_core::Result<PasswordCredential> {
        Self::ICredentialFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreatePasswordCredential)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(resource), ::core::mem::transmute_copy(username), ::core::mem::transmute_copy(password), &mut result__).from_abi(result__)
        })
    }
    pub fn Resource(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Resource)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetResource(&self, resource: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetResource)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(resource)).ok() }
    }
    pub fn UserName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetUserName(&self, username: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUserName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(username)).ok() }
    }
    pub fn Password(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Password)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPassword(&self, password: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPassword)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(password)).ok() }
    }
    pub fn RetrievePassword(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RetrievePassword)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ICredentialFactory<R, F: FnOnce(&ICredentialFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PasswordCredential, ICredentialFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for PasswordCredential {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.PasswordCredential;{6ab18989-c720-41a7-a6c1-feadb36329a0})");
}
impl ::core::clone::Clone for PasswordCredential {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PasswordCredential {
    type Vtable = IPasswordCredential_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PasswordCredential {
    const IID: ::windows_core::GUID = <IPasswordCredential as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PasswordCredential {
    const NAME: &'static str = "Windows.Security.Credentials.PasswordCredential";
}
::windows_core::imp::interface_hierarchy!(PasswordCredential, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PasswordCredential {}
unsafe impl ::core::marker::Sync for PasswordCredential {}
#[doc = "*Required features: `\"Security_Credentials\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct PasswordCredentialPropertyStore(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl PasswordCredentialPropertyStore {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PasswordCredentialPropertyStore, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<P0>(&self, key: &::windows_core::HSTRING, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MapChanged<P0>(&self, vhnd: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Collections::MapChangedEventHandler<::windows_core::HSTRING, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MapChanged)(::windows_core::Interface::as_raw(this), vhnd.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveMapChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::IInspectable>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMapChanged)(::windows_core::Interface::as_raw(this), token).ok() }
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
impl ::windows_core::RuntimeType for PasswordCredentialPropertyStore {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.PasswordCredentialPropertyStore;{8a43ed9f-f4e6-4421-acf9-1dab2986820c})");
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for PasswordCredentialPropertyStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for PasswordCredentialPropertyStore {
    type Vtable = super::super::Foundation::Collections::IPropertySet_Vtbl;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for PasswordCredentialPropertyStore {
    const IID: ::windows_core::GUID = <super::super::Foundation::Collections::IPropertySet as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for PasswordCredentialPropertyStore {
    const NAME: &'static str = "Windows.Security.Credentials.PasswordCredentialPropertyStore";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for PasswordCredentialPropertyStore {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &PasswordCredentialPropertyStore {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(PasswordCredentialPropertyStore, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::IInspectable>>> for PasswordCredentialPropertyStore {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::IInspectable>> for PasswordCredentialPropertyStore {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::IInspectable>> for PasswordCredentialPropertyStore {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IPropertySet> for PasswordCredentialPropertyStore {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for PasswordCredentialPropertyStore {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for PasswordCredentialPropertyStore {}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
pub struct PasswordVault(::windows_core::IUnknown);
impl PasswordVault {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PasswordVault, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Add<P0>(&self, credential: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PasswordCredential>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Add)(::windows_core::Interface::as_raw(this), credential.into_param().abi()).ok() }
    }
    pub fn Remove<P0>(&self, credential: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<PasswordCredential>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), credential.into_param().abi()).ok() }
    }
    pub fn Retrieve(&self, resource: &::windows_core::HSTRING, username: &::windows_core::HSTRING) -> ::windows_core::Result<PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Retrieve)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(resource), ::core::mem::transmute_copy(username), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllByResource(&self, resource: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<PasswordCredential>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllByResource)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(resource), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllByUserName(&self, username: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<PasswordCredential>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllByUserName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(username), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RetrieveAll(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<PasswordCredential>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RetrieveAll)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for PasswordVault {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.PasswordVault;{61fd2c0b-c8d4-48c1-a54f-bc5a64205af2})");
}
impl ::core::clone::Clone for PasswordVault {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PasswordVault {
    type Vtable = IPasswordVault_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PasswordVault {
    const IID: ::windows_core::GUID = <IPasswordVault as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PasswordVault {
    const NAME: &'static str = "Windows.Security.Credentials.PasswordVault";
}
::windows_core::imp::interface_hierarchy!(PasswordVault, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PasswordVault {}
unsafe impl ::core::marker::Sync for PasswordVault {}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
pub struct WebAccount(::windows_core::IUnknown);
impl WebAccount {
    pub fn WebAccountProvider(&self) -> ::windows_core::Result<WebAccountProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WebAccountProvider)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UserName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn State(&self) -> ::windows_core::Result<WebAccountState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = &::windows_core::ComInterface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetPictureAsync(&self, desizedsize: WebAccountPictureSize) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>> {
        let this = &::windows_core::ComInterface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPictureAsync)(::windows_core::Interface::as_raw(this), desizedsize, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SignOutAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SignOutAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SignOutWithClientIdAsync(&self, clientid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SignOutWithClientIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(clientid), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateWebAccount<P0>(webaccountprovider: P0, username: &::windows_core::HSTRING, state: WebAccountState) -> ::windows_core::Result<WebAccount>
    where
        P0: ::windows_core::IntoParam<WebAccountProvider>,
    {
        Self::IWebAccountFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWebAccount)(::windows_core::Interface::as_raw(this), webaccountprovider.into_param().abi(), ::core::mem::transmute_copy(username), state, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebAccountFactory<R, F: FnOnce(&IWebAccountFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WebAccount, IWebAccountFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for WebAccount {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.WebAccount;{69473eb2-8031-49be-80bb-96cb46d99aba})");
}
impl ::core::clone::Clone for WebAccount {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WebAccount {
    type Vtable = IWebAccount_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebAccount {
    const IID: ::windows_core::GUID = <IWebAccount as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebAccount {
    const NAME: &'static str = "Windows.Security.Credentials.WebAccount";
}
::windows_core::imp::interface_hierarchy!(WebAccount, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IWebAccount> for WebAccount {}
unsafe impl ::core::marker::Send for WebAccount {}
unsafe impl ::core::marker::Sync for WebAccount {}
#[doc = "*Required features: `\"Security_Credentials\"`*"]
#[repr(transparent)]
pub struct WebAccountProvider(::windows_core::IUnknown);
impl WebAccountProvider {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn IconUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IconUri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayPurpose(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProvider2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayPurpose)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Authority(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProvider2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Authority)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProvider3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsSystemProvider(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IWebAccountProvider4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSystemProvider)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWebAccountProvider<P0>(id: &::windows_core::HSTRING, displayname: &::windows_core::HSTRING, iconuri: P0) -> ::windows_core::Result<WebAccountProvider>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::IWebAccountProviderFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWebAccountProvider)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(id), ::core::mem::transmute_copy(displayname), iconuri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebAccountProviderFactory<R, F: FnOnce(&IWebAccountProviderFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WebAccountProvider, IWebAccountProviderFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for WebAccountProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.WebAccountProvider;{29dcc8c3-7ab9-4a7c-a336-b942f9dbf7c7})");
}
impl ::core::clone::Clone for WebAccountProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for WebAccountProvider {
    type Vtable = IWebAccountProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebAccountProvider {
    const IID: ::windows_core::GUID = <IWebAccountProvider as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebAccountProvider {
    const NAME: &'static str = "Windows.Security.Credentials.WebAccountProvider";
}
::windows_core::imp::interface_hierarchy!(WebAccountProvider, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WebAccountProvider {}
unsafe impl ::core::marker::Sync for WebAccountProvider {}
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
impl ::windows_core::TypeKind for KeyCredentialAttestationStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for KeyCredentialAttestationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredentialAttestationStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for KeyCredentialAttestationStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.KeyCredentialAttestationStatus;i4)");
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
impl ::windows_core::TypeKind for KeyCredentialCreationOption {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for KeyCredentialCreationOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredentialCreationOption").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for KeyCredentialCreationOption {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.KeyCredentialCreationOption;i4)");
}
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
impl ::windows_core::TypeKind for KeyCredentialStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for KeyCredentialStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredentialStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for KeyCredentialStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.KeyCredentialStatus;i4)");
}
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
impl ::windows_core::TypeKind for WebAccountPictureSize {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WebAccountPictureSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountPictureSize").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WebAccountPictureSize {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.WebAccountPictureSize;i4)");
}
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
impl ::windows_core::TypeKind for WebAccountState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WebAccountState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WebAccountState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.WebAccountState;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
