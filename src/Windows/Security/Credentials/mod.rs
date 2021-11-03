#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Security_Credentials_UI")]
pub mod UI;
#[repr(transparent)]
#[doc(hidden)]
pub struct ICredentialFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICredentialFactory {
    type Vtable = ICredentialFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1424954273, 48934, 18357, [151, 221, 222, 119, 155, 124, 173, 88]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resource: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, username: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, password: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyCredential(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKeyCredential {
    type Vtable = IKeyCredential_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2508582797, 17787, 18503, [177, 26, 250, 150, 11, 189, 177, 56]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredential_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(all(feature = "Security_Cryptography_Core", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, blobtype: super::Cryptography::Core::CryptographicPublicKeyBlobType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Security_Cryptography_Core", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyCredentialAttestationResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKeyCredentialAttestationResult {
    type Vtable = IKeyCredentialAttestationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2024453025, 41921, 16643, [182, 204, 71, 44, 68, 23, 28, 187]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredentialAttestationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut KeyCredentialAttestationStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyCredentialManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKeyCredentialManagerStatics {
    type Vtable = IKeyCredentialManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1789675147, 3825, 19680, [130, 144, 65, 6, 218, 106, 99, 181]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredentialManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, option: KeyCredentialCreationOption, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyCredentialOperationResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKeyCredentialOperationResult {
    type Vtable = IKeyCredentialOperationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4114056897, 21089, 19677, [151, 109, 204, 144, 154, 199, 22, 32]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredentialOperationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut KeyCredentialStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKeyCredentialRetrievalResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKeyCredentialRetrievalResult {
    type Vtable = IKeyCredentialRetrievalResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1489860355, 36231, 16969, [155, 88, 246, 89, 140, 201, 100, 78]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyCredentialRetrievalResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut KeyCredentialStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPasswordCredential(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPasswordCredential {
    type Vtable = IPasswordCredential_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1790019977, 50976, 16807, [166, 193, 254, 173, 179, 99, 41, 160]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPasswordCredential_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resource: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, username: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, password: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPasswordVault(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPasswordVault {
    type Vtable = IPasswordVault_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1643981835, 51412, 18625, [165, 79, 188, 90, 100, 32, 90, 242]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPasswordVault_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, credential: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, credential: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resource: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, username: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resource: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, username: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Security_Credentials`*"]
pub struct IWebAccount(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAccount {
    type Vtable = IWebAccount_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1766276786, 32817, 18878, [128, 187, 150, 203, 70, 217, 154, 186]);
}
impl IWebAccount {
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn WebAccountProvider(&self) -> ::windows::runtime::Result<WebAccountProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountProvider>(result__)
        }
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn UserName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn State(&self) -> ::windows::runtime::Result<WebAccountState> {
        let this = self;
        unsafe {
            let mut result__: WebAccountState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountState>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IWebAccount {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{69473eb2-8031-49be-80bb-96cb46d99aba}");
}
impl ::std::convert::From<IWebAccount> for ::windows::runtime::IUnknown {
    fn from(value: IWebAccount) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWebAccount> for ::windows::runtime::IUnknown {
    fn from(value: &IWebAccount) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWebAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWebAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWebAccount> for ::windows::runtime::IInspectable {
    fn from(value: IWebAccount) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWebAccount> for ::windows::runtime::IInspectable {
    fn from(value: &IWebAccount) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IWebAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IWebAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccount_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut WebAccountState) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebAccount2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAccount2 {
    type Vtable = IWebAccount2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2069288696, 39179, 20149, [148, 167, 86, 33, 243, 168, 184, 36]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccount2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, desizedsize: WebAccountPictureSize, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clientid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebAccountFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAccountFactory {
    type Vtable = IWebAccountFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2895838009, 7657, 20114, [183, 143, 5, 129, 168, 127, 110, 92]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, webaccountprovider: ::windows::runtime::RawPtr, username: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, state: WebAccountState, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebAccountProvider(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAccountProvider {
    type Vtable = IWebAccountProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(702335171, 31417, 19068, [163, 54, 185, 66, 249, 219, 247, 199]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebAccountProvider2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAccountProvider2 {
    type Vtable = IWebAccountProvider2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1241639685, 20034, 16852, [181, 24, 224, 8, 165, 22, 54, 20]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProvider2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebAccountProvider3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAccountProvider3 {
    type Vtable = IWebAccountProvider3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3659288971, 38669, 19785, [130, 92, 242, 112, 111, 140, 167, 254]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProvider3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebAccountProvider4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAccountProvider4 {
    type Vtable = IWebAccountProvider4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1905252571, 59286, 16912, [183, 78, 132, 210, 152, 148, 176, 128]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProvider4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebAccountProviderFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAccountProviderFactory {
    type Vtable = IWebAccountProviderFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(494304753, 57825, 19354, [167, 116, 92, 124, 126, 59, 243, 113]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, displayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, iconuri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: `Security_Credentials`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct KeyCredential(::windows::runtime::IInspectable);
impl KeyCredential {
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Security_Credentials`, `Storage_Streams`*"]
    pub fn RetrievePublicKeyWithDefaultBlobType(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(all(feature = "Security_Cryptography_Core", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Security_Credentials`, `Security_Cryptography_Core`, `Storage_Streams`*"]
    pub fn RetrievePublicKeyWithBlobType(&self, blobtype: super::Cryptography::Core::CryptographicPublicKeyBlobType) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), blobtype, &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Security_Credentials`, `Foundation`, `Storage_Streams`*"]
    pub fn RequestSignAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, data: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<KeyCredentialOperationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<KeyCredentialOperationResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Credentials`, `Foundation`*"]
    pub fn GetAttestationAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<KeyCredentialAttestationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<KeyCredentialAttestationResult>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for KeyCredential {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.KeyCredential;{9585ef8d-457b-4847-b11a-fa960bbdb138})");
}
unsafe impl ::windows::runtime::Interface for KeyCredential {
    type Vtable = IKeyCredential_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2508582797, 17787, 18503, [177, 26, 250, 150, 11, 189, 177, 56]);
}
impl ::windows::runtime::RuntimeName for KeyCredential {
    const NAME: &'static str = "Windows.Security.Credentials.KeyCredential";
}
impl ::std::convert::From<KeyCredential> for ::windows::runtime::IUnknown {
    fn from(value: KeyCredential) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&KeyCredential> for ::windows::runtime::IUnknown {
    fn from(value: &KeyCredential) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for KeyCredential {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &KeyCredential {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<KeyCredential> for ::windows::runtime::IInspectable {
    fn from(value: KeyCredential) -> Self {
        value.0
    }
}
impl ::std::convert::From<&KeyCredential> for ::windows::runtime::IInspectable {
    fn from(value: &KeyCredential) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for KeyCredential {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a KeyCredential {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for KeyCredential {}
unsafe impl ::std::marker::Sync for KeyCredential {}
#[doc = "*Required features: `Security_Credentials`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct KeyCredentialAttestationResult(::windows::runtime::IInspectable);
impl KeyCredentialAttestationResult {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Security_Credentials`, `Storage_Streams`*"]
    pub fn CertificateChainBuffer(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Security_Credentials`, `Storage_Streams`*"]
    pub fn AttestationBuffer(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<KeyCredentialAttestationStatus> {
        let this = self;
        unsafe {
            let mut result__: KeyCredentialAttestationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<KeyCredentialAttestationStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for KeyCredentialAttestationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.KeyCredentialAttestationResult;{78aab3a1-a3c1-4103-b6cc-472c44171cbb})");
}
unsafe impl ::windows::runtime::Interface for KeyCredentialAttestationResult {
    type Vtable = IKeyCredentialAttestationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2024453025, 41921, 16643, [182, 204, 71, 44, 68, 23, 28, 187]);
}
impl ::windows::runtime::RuntimeName for KeyCredentialAttestationResult {
    const NAME: &'static str = "Windows.Security.Credentials.KeyCredentialAttestationResult";
}
impl ::std::convert::From<KeyCredentialAttestationResult> for ::windows::runtime::IUnknown {
    fn from(value: KeyCredentialAttestationResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&KeyCredentialAttestationResult> for ::windows::runtime::IUnknown {
    fn from(value: &KeyCredentialAttestationResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for KeyCredentialAttestationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &KeyCredentialAttestationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<KeyCredentialAttestationResult> for ::windows::runtime::IInspectable {
    fn from(value: KeyCredentialAttestationResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&KeyCredentialAttestationResult> for ::windows::runtime::IInspectable {
    fn from(value: &KeyCredentialAttestationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for KeyCredentialAttestationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a KeyCredentialAttestationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for KeyCredentialAttestationResult {}
unsafe impl ::std::marker::Sync for KeyCredentialAttestationResult {}
#[doc = "*Required features: `Security_Credentials`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct KeyCredentialAttestationStatus(pub i32);
impl KeyCredentialAttestationStatus {
    pub const Success: KeyCredentialAttestationStatus = KeyCredentialAttestationStatus(0i32);
    pub const UnknownError: KeyCredentialAttestationStatus = KeyCredentialAttestationStatus(1i32);
    pub const NotSupported: KeyCredentialAttestationStatus = KeyCredentialAttestationStatus(2i32);
    pub const TemporaryFailure: KeyCredentialAttestationStatus = KeyCredentialAttestationStatus(3i32);
}
impl ::std::convert::From<i32> for KeyCredentialAttestationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KeyCredentialAttestationStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for KeyCredentialAttestationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.KeyCredentialAttestationStatus;i4)");
}
impl ::windows::runtime::DefaultType for KeyCredentialAttestationStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Credentials`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct KeyCredentialCreationOption(pub i32);
impl KeyCredentialCreationOption {
    pub const ReplaceExisting: KeyCredentialCreationOption = KeyCredentialCreationOption(0i32);
    pub const FailIfExists: KeyCredentialCreationOption = KeyCredentialCreationOption(1i32);
}
impl ::std::convert::From<i32> for KeyCredentialCreationOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KeyCredentialCreationOption {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for KeyCredentialCreationOption {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.KeyCredentialCreationOption;i4)");
}
impl ::windows::runtime::DefaultType for KeyCredentialCreationOption {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Credentials`*"]
pub struct KeyCredentialManager {}
impl KeyCredentialManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Credentials`, `Foundation`*"]
    pub fn IsSupportedAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Credentials`, `Foundation`*"]
    pub fn RenewAttestationAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Credentials`, `Foundation`*"]
    pub fn RequestCreateAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0, option: KeyCredentialCreationOption) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<KeyCredentialRetrievalResult>> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), name.into_param().abi(), option, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<KeyCredentialRetrievalResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Credentials`, `Foundation`*"]
    pub fn OpenAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<KeyCredentialRetrievalResult>> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<KeyCredentialRetrievalResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Credentials`, `Foundation`*"]
    pub fn DeleteAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::IKeyCredentialManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn IKeyCredentialManagerStatics<R, F: FnOnce(&IKeyCredentialManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<KeyCredentialManager, IKeyCredentialManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for KeyCredentialManager {
    const NAME: &'static str = "Windows.Security.Credentials.KeyCredentialManager";
}
#[doc = "*Required features: `Security_Credentials`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct KeyCredentialOperationResult(::windows::runtime::IInspectable);
impl KeyCredentialOperationResult {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Security_Credentials`, `Storage_Streams`*"]
    pub fn Result(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<KeyCredentialStatus> {
        let this = self;
        unsafe {
            let mut result__: KeyCredentialStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<KeyCredentialStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for KeyCredentialOperationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.KeyCredentialOperationResult;{f53786c1-5261-4cdd-976d-cc909ac71620})");
}
unsafe impl ::windows::runtime::Interface for KeyCredentialOperationResult {
    type Vtable = IKeyCredentialOperationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4114056897, 21089, 19677, [151, 109, 204, 144, 154, 199, 22, 32]);
}
impl ::windows::runtime::RuntimeName for KeyCredentialOperationResult {
    const NAME: &'static str = "Windows.Security.Credentials.KeyCredentialOperationResult";
}
impl ::std::convert::From<KeyCredentialOperationResult> for ::windows::runtime::IUnknown {
    fn from(value: KeyCredentialOperationResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&KeyCredentialOperationResult> for ::windows::runtime::IUnknown {
    fn from(value: &KeyCredentialOperationResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for KeyCredentialOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &KeyCredentialOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<KeyCredentialOperationResult> for ::windows::runtime::IInspectable {
    fn from(value: KeyCredentialOperationResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&KeyCredentialOperationResult> for ::windows::runtime::IInspectable {
    fn from(value: &KeyCredentialOperationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for KeyCredentialOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a KeyCredentialOperationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for KeyCredentialOperationResult {}
unsafe impl ::std::marker::Sync for KeyCredentialOperationResult {}
#[doc = "*Required features: `Security_Credentials`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct KeyCredentialRetrievalResult(::windows::runtime::IInspectable);
impl KeyCredentialRetrievalResult {
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn Credential(&self) -> ::windows::runtime::Result<KeyCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<KeyCredential>(result__)
        }
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<KeyCredentialStatus> {
        let this = self;
        unsafe {
            let mut result__: KeyCredentialStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<KeyCredentialStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for KeyCredentialRetrievalResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.KeyCredentialRetrievalResult;{58cd7703-8d87-4249-9b58-f6598cc9644e})");
}
unsafe impl ::windows::runtime::Interface for KeyCredentialRetrievalResult {
    type Vtable = IKeyCredentialRetrievalResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1489860355, 36231, 16969, [155, 88, 246, 89, 140, 201, 100, 78]);
}
impl ::windows::runtime::RuntimeName for KeyCredentialRetrievalResult {
    const NAME: &'static str = "Windows.Security.Credentials.KeyCredentialRetrievalResult";
}
impl ::std::convert::From<KeyCredentialRetrievalResult> for ::windows::runtime::IUnknown {
    fn from(value: KeyCredentialRetrievalResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&KeyCredentialRetrievalResult> for ::windows::runtime::IUnknown {
    fn from(value: &KeyCredentialRetrievalResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for KeyCredentialRetrievalResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &KeyCredentialRetrievalResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<KeyCredentialRetrievalResult> for ::windows::runtime::IInspectable {
    fn from(value: KeyCredentialRetrievalResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&KeyCredentialRetrievalResult> for ::windows::runtime::IInspectable {
    fn from(value: &KeyCredentialRetrievalResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for KeyCredentialRetrievalResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a KeyCredentialRetrievalResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for KeyCredentialRetrievalResult {}
unsafe impl ::std::marker::Sync for KeyCredentialRetrievalResult {}
#[doc = "*Required features: `Security_Credentials`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct KeyCredentialStatus(pub i32);
impl KeyCredentialStatus {
    pub const Success: KeyCredentialStatus = KeyCredentialStatus(0i32);
    pub const UnknownError: KeyCredentialStatus = KeyCredentialStatus(1i32);
    pub const NotFound: KeyCredentialStatus = KeyCredentialStatus(2i32);
    pub const UserCanceled: KeyCredentialStatus = KeyCredentialStatus(3i32);
    pub const UserPrefersPassword: KeyCredentialStatus = KeyCredentialStatus(4i32);
    pub const CredentialAlreadyExists: KeyCredentialStatus = KeyCredentialStatus(5i32);
    pub const SecurityDeviceLocked: KeyCredentialStatus = KeyCredentialStatus(6i32);
}
impl ::std::convert::From<i32> for KeyCredentialStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KeyCredentialStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for KeyCredentialStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.KeyCredentialStatus;i4)");
}
impl ::windows::runtime::DefaultType for KeyCredentialStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Credentials`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PasswordCredential(::windows::runtime::IInspectable);
impl PasswordCredential {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PasswordCredential, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn Resource(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn SetResource<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, resource: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), resource.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn UserName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn SetUserName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, username: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), username.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn Password(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn SetPassword<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, password: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), password.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn RetrievePassword(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Credentials`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn CreatePasswordCredential<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(resource: Param0, username: Param1, password: Param2) -> ::windows::runtime::Result<PasswordCredential> {
        Self::ICredentialFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), resource.into_param().abi(), username.into_param().abi(), password.into_param().abi(), &mut result__).from_abi::<PasswordCredential>(result__)
        })
    }
    pub fn ICredentialFactory<R, F: FnOnce(&ICredentialFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PasswordCredential, ICredentialFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PasswordCredential {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.PasswordCredential;{6ab18989-c720-41a7-a6c1-feadb36329a0})");
}
unsafe impl ::windows::runtime::Interface for PasswordCredential {
    type Vtable = IPasswordCredential_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1790019977, 50976, 16807, [166, 193, 254, 173, 179, 99, 41, 160]);
}
impl ::windows::runtime::RuntimeName for PasswordCredential {
    const NAME: &'static str = "Windows.Security.Credentials.PasswordCredential";
}
impl ::std::convert::From<PasswordCredential> for ::windows::runtime::IUnknown {
    fn from(value: PasswordCredential) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PasswordCredential> for ::windows::runtime::IUnknown {
    fn from(value: &PasswordCredential) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PasswordCredential {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PasswordCredential {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PasswordCredential> for ::windows::runtime::IInspectable {
    fn from(value: PasswordCredential) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PasswordCredential> for ::windows::runtime::IInspectable {
    fn from(value: &PasswordCredential) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PasswordCredential {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PasswordCredential {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PasswordCredential {}
unsafe impl ::std::marker::Sync for PasswordCredential {}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `Security_Credentials`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PasswordCredentialPropertyStore(::windows::runtime::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl PasswordCredentialPropertyStore {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PasswordCredentialPropertyStore, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Credentials`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Credentials`, `Foundation_Collections`*"]
    pub fn Lookup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Credentials`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Credentials`, `Foundation_Collections`*"]
    pub fn HasKey<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Credentials`, `Foundation_Collections`*"]
    pub fn GetView(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Credentials`, `Foundation_Collections`*"]
    pub fn Insert<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, key: Param0, value: Param1) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Credentials`, `Foundation_Collections`*"]
    pub fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Credentials`, `Foundation_Collections`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Security_Credentials`, `Foundation`, `Foundation_Collections`*"]
    pub fn MapChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::MapChangedEventHandler<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>(&self, vhnd: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), vhnd.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Security_Credentials`, `Foundation`, `Foundation_Collections`*"]
    pub fn RemoveMapChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::Collections::IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for PasswordCredentialPropertyStore {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.PasswordCredentialPropertyStore;{8a43ed9f-f4e6-4421-acf9-1dab2986820c})");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for PasswordCredentialPropertyStore {
    type Vtable = super::super::Foundation::Collections::IPropertySet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2319707551, 62694, 17441, [172, 249, 29, 171, 41, 134, 130, 12]);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::runtime::RuntimeName for PasswordCredentialPropertyStore {
    const NAME: &'static str = "Windows.Security.Credentials.PasswordCredentialPropertyStore";
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PasswordCredentialPropertyStore> for ::windows::runtime::IUnknown {
    fn from(value: PasswordCredentialPropertyStore) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PasswordCredentialPropertyStore> for ::windows::runtime::IUnknown {
    fn from(value: &PasswordCredentialPropertyStore) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PasswordCredentialPropertyStore> for ::windows::runtime::IInspectable {
    fn from(value: PasswordCredentialPropertyStore) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PasswordCredentialPropertyStore> for ::windows::runtime::IInspectable {
    fn from(value: &PasswordCredentialPropertyStore) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<PasswordCredentialPropertyStore> for super::super::Foundation::Collections::IPropertySet {
    fn from(value: PasswordCredentialPropertyStore) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&PasswordCredentialPropertyStore> for super::super::Foundation::Collections::IPropertySet {
    fn from(value: &PasswordCredentialPropertyStore) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IPropertySet> for PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IPropertySet> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::Foundation::Collections::IPropertySet>::into(self))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IPropertySet> for &PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IPropertySet> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::Foundation::Collections::IPropertySet>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<PasswordCredentialPropertyStore> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PasswordCredentialPropertyStore) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&PasswordCredentialPropertyStore> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PasswordCredentialPropertyStore) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> for PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> for &PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>> {
        ::std::convert::TryInto::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<PasswordCredentialPropertyStore> for super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PasswordCredentialPropertyStore) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&PasswordCredentialPropertyStore> for super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PasswordCredentialPropertyStore) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for &PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::std::convert::TryInto::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<PasswordCredentialPropertyStore> for super::super::Foundation::Collections::IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PasswordCredentialPropertyStore) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&PasswordCredentialPropertyStore> for super::super::Foundation::Collections::IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PasswordCredentialPropertyStore) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> for &PasswordCredentialPropertyStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::Collections::IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        ::std::convert::TryInto::<super::super::Foundation::Collections::IObservableMap<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::std::marker::Send for PasswordCredentialPropertyStore {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::std::marker::Sync for PasswordCredentialPropertyStore {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for PasswordCredentialPropertyStore {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &PasswordCredentialPropertyStore {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[doc = "*Required features: `Security_Credentials`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PasswordVault(::windows::runtime::IInspectable);
impl PasswordVault {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PasswordVault, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, PasswordCredential>>(&self, credential: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), credential.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, PasswordCredential>>(&self, credential: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), credential.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn Retrieve<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, resource: Param0, username: Param1) -> ::windows::runtime::Result<PasswordCredential> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), resource.into_param().abi(), username.into_param().abi(), &mut result__).from_abi::<PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Credentials`, `Foundation_Collections`*"]
    pub fn FindAllByResource<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, resource: Param0) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<PasswordCredential>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), resource.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PasswordCredential>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Credentials`, `Foundation_Collections`*"]
    pub fn FindAllByUserName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, username: Param0) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<PasswordCredential>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), username.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PasswordCredential>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Credentials`, `Foundation_Collections`*"]
    pub fn RetrieveAll(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<PasswordCredential>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PasswordCredential>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PasswordVault {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.PasswordVault;{61fd2c0b-c8d4-48c1-a54f-bc5a64205af2})");
}
unsafe impl ::windows::runtime::Interface for PasswordVault {
    type Vtable = IPasswordVault_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1643981835, 51412, 18625, [165, 79, 188, 90, 100, 32, 90, 242]);
}
impl ::windows::runtime::RuntimeName for PasswordVault {
    const NAME: &'static str = "Windows.Security.Credentials.PasswordVault";
}
impl ::std::convert::From<PasswordVault> for ::windows::runtime::IUnknown {
    fn from(value: PasswordVault) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PasswordVault> for ::windows::runtime::IUnknown {
    fn from(value: &PasswordVault) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PasswordVault {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PasswordVault {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PasswordVault> for ::windows::runtime::IInspectable {
    fn from(value: PasswordVault) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PasswordVault> for ::windows::runtime::IInspectable {
    fn from(value: &PasswordVault) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PasswordVault {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PasswordVault {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PasswordVault {}
unsafe impl ::std::marker::Sync for PasswordVault {}
#[doc = "*Required features: `Security_Credentials`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WebAccount(::windows::runtime::IInspectable);
impl WebAccount {
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn WebAccountProvider(&self) -> ::windows::runtime::Result<WebAccountProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountProvider>(result__)
        }
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn UserName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn State(&self) -> ::windows::runtime::Result<WebAccountState> {
        let this = self;
        unsafe {
            let mut result__: WebAccountState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WebAccountState>(result__)
        }
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Credentials`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        let this = &::windows::runtime::Interface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Security_Credentials`, `Foundation`, `Storage_Streams`*"]
    pub fn GetPictureAsync(&self, desizedsize: WebAccountPictureSize) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>> {
        let this = &::windows::runtime::Interface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), desizedsize, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Credentials`, `Foundation`*"]
    pub fn SignOutAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Credentials`, `Foundation`*"]
    pub fn SignOutWithClientIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, clientid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IWebAccount2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), clientid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn CreateWebAccount<'a, Param0: ::windows::runtime::IntoParam<'a, WebAccountProvider>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(webaccountprovider: Param0, username: Param1, state: WebAccountState) -> ::windows::runtime::Result<WebAccount> {
        Self::IWebAccountFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), webaccountprovider.into_param().abi(), username.into_param().abi(), state, &mut result__).from_abi::<WebAccount>(result__)
        })
    }
    pub fn IWebAccountFactory<R, F: FnOnce(&IWebAccountFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebAccount, IWebAccountFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebAccount {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.WebAccount;{69473eb2-8031-49be-80bb-96cb46d99aba})");
}
unsafe impl ::windows::runtime::Interface for WebAccount {
    type Vtable = IWebAccount_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1766276786, 32817, 18878, [128, 187, 150, 203, 70, 217, 154, 186]);
}
impl ::windows::runtime::RuntimeName for WebAccount {
    const NAME: &'static str = "Windows.Security.Credentials.WebAccount";
}
impl ::std::convert::From<WebAccount> for ::windows::runtime::IUnknown {
    fn from(value: WebAccount) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WebAccount> for ::windows::runtime::IUnknown {
    fn from(value: &WebAccount) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WebAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &WebAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<WebAccount> for ::windows::runtime::IInspectable {
    fn from(value: WebAccount) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WebAccount> for ::windows::runtime::IInspectable {
    fn from(value: &WebAccount) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WebAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WebAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<WebAccount> for IWebAccount {
    fn from(value: WebAccount) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WebAccount> for IWebAccount {
    fn from(value: &WebAccount) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWebAccount> for WebAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebAccount> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWebAccount>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWebAccount> for &WebAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebAccount> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWebAccount>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for WebAccount {}
unsafe impl ::std::marker::Sync for WebAccount {}
#[doc = "*Required features: `Security_Credentials`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WebAccountPictureSize(pub i32);
impl WebAccountPictureSize {
    pub const Size64x64: WebAccountPictureSize = WebAccountPictureSize(64i32);
    pub const Size208x208: WebAccountPictureSize = WebAccountPictureSize(208i32);
    pub const Size424x424: WebAccountPictureSize = WebAccountPictureSize(424i32);
    pub const Size1080x1080: WebAccountPictureSize = WebAccountPictureSize(1080i32);
}
impl ::std::convert::From<i32> for WebAccountPictureSize {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WebAccountPictureSize {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WebAccountPictureSize {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.WebAccountPictureSize;i4)");
}
impl ::windows::runtime::DefaultType for WebAccountPictureSize {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Credentials`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WebAccountProvider(::windows::runtime::IInspectable);
impl WebAccountProvider {
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Credentials`, `Foundation`*"]
    pub fn IconUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn DisplayPurpose(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IWebAccountProvider2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn Authority(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IWebAccountProvider2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Security_Credentials`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IWebAccountProvider3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Credentials`, `Foundation`*"]
    pub fn CreateWebAccountProvider<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(id: Param0, displayname: Param1, iconuri: Param2) -> ::windows::runtime::Result<WebAccountProvider> {
        Self::IWebAccountProviderFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), id.into_param().abi(), displayname.into_param().abi(), iconuri.into_param().abi(), &mut result__).from_abi::<WebAccountProvider>(result__)
        })
    }
    #[doc = "*Required features: `Security_Credentials`*"]
    pub fn IsSystemProvider(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IWebAccountProvider4>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IWebAccountProviderFactory<R, F: FnOnce(&IWebAccountProviderFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebAccountProvider, IWebAccountProviderFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebAccountProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.WebAccountProvider;{29dcc8c3-7ab9-4a7c-a336-b942f9dbf7c7})");
}
unsafe impl ::windows::runtime::Interface for WebAccountProvider {
    type Vtable = IWebAccountProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(702335171, 31417, 19068, [163, 54, 185, 66, 249, 219, 247, 199]);
}
impl ::windows::runtime::RuntimeName for WebAccountProvider {
    const NAME: &'static str = "Windows.Security.Credentials.WebAccountProvider";
}
impl ::std::convert::From<WebAccountProvider> for ::windows::runtime::IUnknown {
    fn from(value: WebAccountProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WebAccountProvider> for ::windows::runtime::IUnknown {
    fn from(value: &WebAccountProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WebAccountProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &WebAccountProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<WebAccountProvider> for ::windows::runtime::IInspectable {
    fn from(value: WebAccountProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WebAccountProvider> for ::windows::runtime::IInspectable {
    fn from(value: &WebAccountProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WebAccountProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WebAccountProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for WebAccountProvider {}
unsafe impl ::std::marker::Sync for WebAccountProvider {}
#[doc = "*Required features: `Security_Credentials`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WebAccountState(pub i32);
impl WebAccountState {
    pub const None: WebAccountState = WebAccountState(0i32);
    pub const Connected: WebAccountState = WebAccountState(1i32);
    pub const Error: WebAccountState = WebAccountState(2i32);
}
impl ::std::convert::From<i32> for WebAccountState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WebAccountState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WebAccountState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.WebAccountState;i4)");
}
impl ::windows::runtime::DefaultType for WebAccountState {
    type DefaultType = Self;
}
