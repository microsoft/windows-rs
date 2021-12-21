#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Security_Credentials_UI")]
pub mod UI;
#[doc(hidden)]
#[repr(transparent)]
pub struct ICredentialFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICredentialFactory {
    type Vtable = ICredentialFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54ef13a1_bf26_47b5_97dd_de779b7cad58);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyCredential(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyCredential {
    type Vtable = IKeyCredentialVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9585ef8d_457b_4847_b11a_fa960bbdb138);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredentialVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(all(feature = "Security_Cryptography_Core", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blobtype: super::Cryptography::Core::CryptographicPublicKeyBlobType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Security_Cryptography_Core", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyCredentialAttestationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyCredentialAttestationResult {
    type Vtable = IKeyCredentialAttestationResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78aab3a1_a3c1_4103_b6cc_472c44171cbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredentialAttestationResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut KeyCredentialAttestationStatus) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyCredentialManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyCredentialManagerStatics {
    type Vtable = IKeyCredentialManagerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6aac468b_0ef1_4ce0_8290_4106da6a63b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredentialManagerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, option: KeyCredentialCreationOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyCredentialOperationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyCredentialOperationResult {
    type Vtable = IKeyCredentialOperationResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf53786c1_5261_4cdd_976d_cc909ac71620);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredentialOperationResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut KeyCredentialStatus) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IKeyCredentialRetrievalResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IKeyCredentialRetrievalResult {
    type Vtable = IKeyCredentialRetrievalResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58cd7703_8d87_4249_9b58_f6598cc9644e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredentialRetrievalResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut KeyCredentialStatus) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPasswordCredential(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPasswordCredential {
    type Vtable = IPasswordCredentialVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ab18989_c720_41a7_a6c1_feadb36329a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPasswordCredentialVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, password: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPasswordVault(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPasswordVault {
    type Vtable = IPasswordVaultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61fd2c0b_c8d4_48c1_a54f_bc5a64205af2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPasswordVaultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credential: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, credential: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc = "*Required features: 'Security_Credentials'*"]
#[repr(transparent)]
pub struct IWebAccount(::windows::core::IUnknown);
impl IWebAccount {
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn WebAccountProvider(&self) -> ::windows::core::Result<WebAccountProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountProvider>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn State(&self) -> ::windows::core::Result<WebAccountState> {
        let this = self;
        unsafe {
            let mut result__: WebAccountState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountState>(result__)
        }
    }
}
impl ::core::convert::From<IWebAccount> for ::windows::core::IInspectable {
    fn from(value: IWebAccount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccount> for ::windows::core::IInspectable {
    fn from(value: &IWebAccount) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWebAccount {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IWebAccount {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebAccount> for ::windows::core::IUnknown {
    fn from(value: IWebAccount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebAccount> for ::windows::core::IUnknown {
    fn from(value: &IWebAccount) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebAccount {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IWebAccount {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
}
unsafe impl ::windows::core::Interface for IWebAccount {
    type Vtable = IWebAccountVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69473eb2_8031_49be_80bb_96cb46d99aba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebAccountState) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccount2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccount2 {
    type Vtable = IWebAccount2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b56d6f8_990b_4eb5_94a7_5621f3a8b824);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccount2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desizedsize: WebAccountPictureSize, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountFactory {
    type Vtable = IWebAccountFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac9afb39_1de9_4e92_b78f_0581a87f6e5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccountprovider: ::windows::core::RawPtr, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, state: WebAccountState, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountProvider {
    type Vtable = IWebAccountProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29dcc8c3_7ab9_4a7c_a336_b942f9dbf7c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProvider2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountProvider2 {
    type Vtable = IWebAccountProvider2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a01eb05_4e42_41d4_b518_e008a5163614);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProvider2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProvider3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountProvider3 {
    type Vtable = IWebAccountProvider3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda1c518b_970d_4d49_825c_f2706f8ca7fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProvider3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProvider4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountProvider4 {
    type Vtable = IWebAccountProvider4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x718fd8db_e796_4210_b74e_84d29894b080);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProvider4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAccountProviderFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAccountProviderFactory {
    type Vtable = IWebAccountProviderFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d767df1_e1e1_4b9a_a774_5c7c7e3bf371);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, iconuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: 'Security_Credentials'*"]
#[repr(transparent)]
pub struct KeyCredential(::windows::core::IUnknown);
impl KeyCredential {
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RetrievePublicKeyWithDefaultBlobType(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Security_Cryptography_Core', 'Storage_Streams'*"]
    #[cfg(all(feature = "Security_Cryptography_Core", feature = "Storage_Streams"))]
    pub fn RetrievePublicKeyWithBlobType(&self, blobtype: super::Cryptography::Core::CryptographicPublicKeyBlobType) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), blobtype, &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn RequestSignAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, data: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<KeyCredentialOperationResult>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAttestationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialAttestationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<KeyCredentialAttestationResult>>(result__)
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
}
unsafe impl ::windows::core::Interface for KeyCredential {
    type Vtable = IKeyCredentialVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9585ef8d_457b_4847_b11a_fa960bbdb138);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for KeyCredential {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &KeyCredential {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for KeyCredential {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &KeyCredential {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for KeyCredential {}
unsafe impl ::core::marker::Sync for KeyCredential {}
#[doc = "*Required features: 'Security_Credentials'*"]
#[repr(transparent)]
pub struct KeyCredentialAttestationResult(::windows::core::IUnknown);
impl KeyCredentialAttestationResult {
    #[doc = "*Required features: 'Security_Credentials', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CertificateChainBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AttestationBuffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn Status(&self) -> ::windows::core::Result<KeyCredentialAttestationStatus> {
        let this = self;
        unsafe {
            let mut result__: KeyCredentialAttestationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeyCredentialAttestationStatus>(result__)
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
}
unsafe impl ::windows::core::Interface for KeyCredentialAttestationResult {
    type Vtable = IKeyCredentialAttestationResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78aab3a1_a3c1_4103_b6cc_472c44171cbb);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for KeyCredentialAttestationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &KeyCredentialAttestationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for KeyCredentialAttestationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &KeyCredentialAttestationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for KeyCredentialAttestationResult {}
unsafe impl ::core::marker::Sync for KeyCredentialAttestationResult {}
#[doc = "*Required features: 'Security_Credentials'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for KeyCredentialAttestationStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KeyCredentialAttestationStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyCredentialAttestationStatus {}
impl ::core::fmt::Debug for KeyCredentialAttestationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredentialAttestationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeyCredentialAttestationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.KeyCredentialAttestationStatus;i4)");
}
impl ::windows::core::DefaultType for KeyCredentialAttestationStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Security_Credentials'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for KeyCredentialCreationOption {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KeyCredentialCreationOption {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyCredentialCreationOption {}
impl ::core::fmt::Debug for KeyCredentialCreationOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredentialCreationOption").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeyCredentialCreationOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.KeyCredentialCreationOption;i4)");
}
impl ::windows::core::DefaultType for KeyCredentialCreationOption {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Security_Credentials'*"]
pub struct KeyCredentialManager {}
impl KeyCredentialManager {
    #[doc = "*Required features: 'Security_Credentials', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn IsSupportedAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RenewAttestationAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestCreateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0, option: KeyCredentialCreationOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialRetrievalResult>> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), name.into_param().abi(), option, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<KeyCredentialRetrievalResult>>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<KeyCredentialRetrievalResult>> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<KeyCredentialRetrievalResult>>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IKeyCredentialManagerStatics<R, F: FnOnce(&IKeyCredentialManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<KeyCredentialManager, IKeyCredentialManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for KeyCredentialManager {
    const NAME: &'static str = "Windows.Security.Credentials.KeyCredentialManager";
}
#[doc = "*Required features: 'Security_Credentials'*"]
#[repr(transparent)]
pub struct KeyCredentialOperationResult(::windows::core::IUnknown);
impl KeyCredentialOperationResult {
    #[doc = "*Required features: 'Security_Credentials', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Result(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn Status(&self) -> ::windows::core::Result<KeyCredentialStatus> {
        let this = self;
        unsafe {
            let mut result__: KeyCredentialStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeyCredentialStatus>(result__)
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
}
unsafe impl ::windows::core::Interface for KeyCredentialOperationResult {
    type Vtable = IKeyCredentialOperationResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf53786c1_5261_4cdd_976d_cc909ac71620);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for KeyCredentialOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &KeyCredentialOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for KeyCredentialOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &KeyCredentialOperationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for KeyCredentialOperationResult {}
unsafe impl ::core::marker::Sync for KeyCredentialOperationResult {}
#[doc = "*Required features: 'Security_Credentials'*"]
#[repr(transparent)]
pub struct KeyCredentialRetrievalResult(::windows::core::IUnknown);
impl KeyCredentialRetrievalResult {
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn Credential(&self) -> ::windows::core::Result<KeyCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeyCredential>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn Status(&self) -> ::windows::core::Result<KeyCredentialStatus> {
        let this = self;
        unsafe {
            let mut result__: KeyCredentialStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<KeyCredentialStatus>(result__)
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
}
unsafe impl ::windows::core::Interface for KeyCredentialRetrievalResult {
    type Vtable = IKeyCredentialRetrievalResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58cd7703_8d87_4249_9b58_f6598cc9644e);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for KeyCredentialRetrievalResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &KeyCredentialRetrievalResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for KeyCredentialRetrievalResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &KeyCredentialRetrievalResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for KeyCredentialRetrievalResult {}
unsafe impl ::core::marker::Sync for KeyCredentialRetrievalResult {}
#[doc = "*Required features: 'Security_Credentials'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for KeyCredentialStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for KeyCredentialStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for KeyCredentialStatus {}
impl ::core::fmt::Debug for KeyCredentialStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyCredentialStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for KeyCredentialStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.KeyCredentialStatus;i4)");
}
impl ::windows::core::DefaultType for KeyCredentialStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Security_Credentials'*"]
#[repr(transparent)]
pub struct PasswordCredential(::windows::core::IUnknown);
impl PasswordCredential {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PasswordCredential, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn CreatePasswordCredential<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(resource: Param0, username: Param1, password: Param2) -> ::windows::core::Result<PasswordCredential> {
        Self::ICredentialFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), resource.into_param().abi(), username.into_param().abi(), password.into_param().abi(), &mut result__).from_abi::<PasswordCredential>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn Resource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn SetResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, resource: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), resource.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn SetUserName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, username: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), username.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn Password(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn SetPassword<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, password: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), password.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn RetrievePassword(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc(hidden)]
    pub fn ICredentialFactory<R, F: FnOnce(&ICredentialFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PasswordCredential, ICredentialFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
}
unsafe impl ::windows::core::Interface for PasswordCredential {
    type Vtable = IPasswordCredentialVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ab18989_c720_41a7_a6c1_feadb36329a0);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PasswordCredential {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PasswordCredential {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PasswordCredential {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PasswordCredential {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PasswordCredential {}
unsafe impl ::core::marker::Sync for PasswordCredential {}
#[doc = "*Required features: 'Security_Credentials', 'Foundation_Collections'*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct PasswordCredentialPropertyStore(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl PasswordCredentialPropertyStore {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PasswordCredentialPropertyStore, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, key: Param0, value: Param1) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn MapChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::MapChangedEventHandler<::windows::core::HSTRING, ::windows::core::IInspectable>>>(&self, vhnd: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), vhnd.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RemoveMapChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
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
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for PasswordCredentialPropertyStore {
    type Vtable = super::super::Foundation::Collections::IPropertySetVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a43ed9f_f4e6_4421_acf9_1dab2986820c);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> for PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> for &PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>> for PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>> for &PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>> for PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>> for &PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IObservableMap<::windows::core::HSTRING, ::windows::core::IInspectable>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet> for PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IPropertySet> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IPropertySet> for &PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IPropertySet> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IPropertySet>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for PasswordCredentialPropertyStore {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for PasswordCredentialPropertyStore {}
#[doc = "*Required features: 'Security_Credentials'*"]
#[repr(transparent)]
pub struct PasswordVault(::windows::core::IUnknown);
impl PasswordVault {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PasswordVault, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn Add<'a, Param0: ::windows::core::IntoParam<'a, PasswordCredential>>(&self, credential: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), credential.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, PasswordCredential>>(&self, credential: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), credential.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn Retrieve<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, resource: Param0, username: Param1) -> ::windows::core::Result<PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), resource.into_param().abi(), username.into_param().abi(), &mut result__).from_abi::<PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllByResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, resource: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PasswordCredential>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), resource.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PasswordCredential>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllByUserName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, username: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PasswordCredential>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), username.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PasswordCredential>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RetrieveAll(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PasswordCredential>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PasswordCredential>>(result__)
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
}
unsafe impl ::windows::core::Interface for PasswordVault {
    type Vtable = IPasswordVaultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61fd2c0b_c8d4_48c1_a54f_bc5a64205af2);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PasswordVault {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PasswordVault {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PasswordVault {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PasswordVault {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PasswordVault {}
unsafe impl ::core::marker::Sync for PasswordVault {}
#[doc = "*Required features: 'Security_Credentials'*"]
#[repr(transparent)]
pub struct WebAccount(::windows::core::IUnknown);
impl WebAccount {
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn WebAccountProvider(&self) -> ::windows::core::Result<WebAccountProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountProvider>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn State(&self) -> ::windows::core::Result<WebAccountState> {
        let this = self;
        unsafe {
            let mut result__: WebAccountState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountState>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetPictureAsync(&self, desizedsize: WebAccountPictureSize) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>> {
        let this = &::windows::core::Interface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), desizedsize, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SignOutAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SignOutWithClientIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, clientid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), clientid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn CreateWebAccount<'a, Param0: ::windows::core::IntoParam<'a, WebAccountProvider>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(webaccountprovider: Param0, username: Param1, state: WebAccountState) -> ::windows::core::Result<WebAccount> {
        Self::IWebAccountFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), webaccountprovider.into_param().abi(), username.into_param().abi(), state, &mut result__).from_abi::<WebAccount>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebAccountFactory<R, F: FnOnce(&IWebAccountFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebAccount, IWebAccountFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
}
unsafe impl ::windows::core::Interface for WebAccount {
    type Vtable = IWebAccountVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69473eb2_8031_49be_80bb_96cb46d99aba);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebAccount {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WebAccount {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebAccount {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WebAccount {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IWebAccount> for WebAccount {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccount> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccount> for &WebAccount {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccount> {
        ::core::convert::TryInto::<IWebAccount>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebAccount {}
unsafe impl ::core::marker::Sync for WebAccount {}
#[doc = "*Required features: 'Security_Credentials'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for WebAccountPictureSize {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WebAccountPictureSize {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountPictureSize {}
impl ::core::fmt::Debug for WebAccountPictureSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountPictureSize").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebAccountPictureSize {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.WebAccountPictureSize;i4)");
}
impl ::windows::core::DefaultType for WebAccountPictureSize {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Security_Credentials'*"]
#[repr(transparent)]
pub struct WebAccountProvider(::windows::core::IUnknown);
impl WebAccountProvider {
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation', 'deprecated'*"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn IconUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn DisplayPurpose(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IWebAccountProvider2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn Authority(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IWebAccountProvider2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials', 'System'*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IWebAccountProvider3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials'*"]
    pub fn IsSystemProvider(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IWebAccountProvider4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Credentials', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWebAccountProvider<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(id: Param0, displayname: Param1, iconuri: Param2) -> ::windows::core::Result<WebAccountProvider> {
        Self::IWebAccountProviderFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), id.into_param().abi(), displayname.into_param().abi(), iconuri.into_param().abi(), &mut result__).from_abi::<WebAccountProvider>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebAccountProviderFactory<R, F: FnOnce(&IWebAccountProviderFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebAccountProvider, IWebAccountProviderFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
}
unsafe impl ::windows::core::Interface for WebAccountProvider {
    type Vtable = IWebAccountProviderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29dcc8c3_7ab9_4a7c_a336_b942f9dbf7c7);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebAccountProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WebAccountProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebAccountProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WebAccountProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WebAccountProvider {}
unsafe impl ::core::marker::Sync for WebAccountProvider {}
#[doc = "*Required features: 'Security_Credentials'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for WebAccountState {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WebAccountState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountState {}
impl ::core::fmt::Debug for WebAccountState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebAccountState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.WebAccountState;i4)");
}
impl ::windows::core::DefaultType for WebAccountState {
    type DefaultType = Self;
}
