#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Security_Credentials_UI'*"]
#[repr(transparent)]
pub struct AuthenticationProtocol(pub i32);
impl AuthenticationProtocol {
    pub const Basic: Self = Self(0i32);
    pub const Digest: Self = Self(1i32);
    pub const Ntlm: Self = Self(2i32);
    pub const Kerberos: Self = Self(3i32);
    pub const Negotiate: Self = Self(4i32);
    pub const CredSsp: Self = Self(5i32);
    pub const Custom: Self = Self(6i32);
}
impl ::core::marker::Copy for AuthenticationProtocol {}
impl ::core::clone::Clone for AuthenticationProtocol {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AuthenticationProtocol {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AuthenticationProtocol {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AuthenticationProtocol {}
unsafe impl ::windows::core::RuntimeType for AuthenticationProtocol {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.UI.AuthenticationProtocol;i4)");
}
impl ::windows::core::DefaultType for AuthenticationProtocol {
    type DefaultType = Self;
}
pub struct CredentialPicker {}
impl CredentialPicker {
    #[cfg(feature = "Foundation")]
    pub fn PickWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, CredentialPickerOptions>>(options: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CredentialPickerResults>> {
        Self::ICredentialPickerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), options.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<CredentialPickerResults>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn PickWithMessageAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(targetname: Param0, message: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CredentialPickerResults>> {
        Self::ICredentialPickerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), targetname.into_param().abi(), message.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<CredentialPickerResults>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn PickWithCaptionAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(targetname: Param0, message: Param1, caption: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CredentialPickerResults>> {
        Self::ICredentialPickerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), targetname.into_param().abi(), message.into_param().abi(), caption.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<CredentialPickerResults>>(result__)
        })
    }
    pub fn ICredentialPickerStatics<R, F: FnOnce(&ICredentialPickerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CredentialPicker, ICredentialPickerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CredentialPicker {
    const NAME: &'static str = "Windows.Security.Credentials.UI.CredentialPicker";
}
#[doc = "*Required features: 'Security_Credentials_UI'*"]
#[repr(transparent)]
pub struct CredentialPickerOptions(::windows::core::IUnknown);
impl CredentialPickerOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CredentialPickerOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SetCaption<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Caption(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetErrorCode(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ErrorCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetTargetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetAuthenticationProtocol(&self, value: AuthenticationProtocol) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AuthenticationProtocol(&self) -> ::windows::core::Result<AuthenticationProtocol> {
        let this = self;
        unsafe {
            let mut result__: AuthenticationProtocol = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AuthenticationProtocol>(result__)
        }
    }
    pub fn SetCustomAuthenticationProtocol<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn CustomAuthenticationProtocol(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetPreviousCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn PreviousCredential(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    pub fn SetAlwaysDisplayDialog(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AlwaysDisplayDialog(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetCallerSavesCredential(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CallerSavesCredential(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetCredentialSaveOption(&self, value: CredentialSaveOption) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn CredentialSaveOption(&self) -> ::windows::core::Result<CredentialSaveOption> {
        let this = self;
        unsafe {
            let mut result__: CredentialSaveOption = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CredentialSaveOption>(result__)
        }
    }
}
impl ::core::clone::Clone for CredentialPickerOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CredentialPickerOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CredentialPickerOptions {}
unsafe impl ::windows::core::RuntimeType for CredentialPickerOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.UI.CredentialPickerOptions;{965a0b4c-95fa-467f-992b-0b22e5859bf6})");
}
unsafe impl ::windows::core::Interface for CredentialPickerOptions {
    type Vtable = ICredentialPickerOptionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x965a0b4c_95fa_467f_992b_0b22e5859bf6);
}
impl ::windows::core::RuntimeName for CredentialPickerOptions {
    const NAME: &'static str = "Windows.Security.Credentials.UI.CredentialPickerOptions";
}
impl ::core::convert::From<CredentialPickerOptions> for ::windows::core::IUnknown {
    fn from(value: CredentialPickerOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CredentialPickerOptions> for ::windows::core::IUnknown {
    fn from(value: &CredentialPickerOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CredentialPickerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CredentialPickerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CredentialPickerOptions> for ::windows::core::IInspectable {
    fn from(value: CredentialPickerOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CredentialPickerOptions> for ::windows::core::IInspectable {
    fn from(value: &CredentialPickerOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CredentialPickerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CredentialPickerOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Security_Credentials_UI'*"]
#[repr(transparent)]
pub struct CredentialPickerResults(::windows::core::IUnknown);
impl CredentialPickerResults {
    pub fn ErrorCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn CredentialSaveOption(&self) -> ::windows::core::Result<CredentialSaveOption> {
        let this = self;
        unsafe {
            let mut result__: CredentialSaveOption = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CredentialSaveOption>(result__)
        }
    }
    pub fn CredentialSaved(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Credential(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    pub fn CredentialDomainName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CredentialUserName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn CredentialPassword(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for CredentialPickerResults {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CredentialPickerResults {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CredentialPickerResults {}
unsafe impl ::windows::core::RuntimeType for CredentialPickerResults {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.UI.CredentialPickerResults;{1948f99a-cc30-410c-9c38-cc0884c5b3d7})");
}
unsafe impl ::windows::core::Interface for CredentialPickerResults {
    type Vtable = ICredentialPickerResultsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1948f99a_cc30_410c_9c38_cc0884c5b3d7);
}
impl ::windows::core::RuntimeName for CredentialPickerResults {
    const NAME: &'static str = "Windows.Security.Credentials.UI.CredentialPickerResults";
}
impl ::core::convert::From<CredentialPickerResults> for ::windows::core::IUnknown {
    fn from(value: CredentialPickerResults) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CredentialPickerResults> for ::windows::core::IUnknown {
    fn from(value: &CredentialPickerResults) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CredentialPickerResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CredentialPickerResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CredentialPickerResults> for ::windows::core::IInspectable {
    fn from(value: CredentialPickerResults) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CredentialPickerResults> for ::windows::core::IInspectable {
    fn from(value: &CredentialPickerResults) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CredentialPickerResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CredentialPickerResults {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Security_Credentials_UI'*"]
#[repr(transparent)]
pub struct CredentialSaveOption(pub i32);
impl CredentialSaveOption {
    pub const Unselected: Self = Self(0i32);
    pub const Selected: Self = Self(1i32);
    pub const Hidden: Self = Self(2i32);
}
impl ::core::marker::Copy for CredentialSaveOption {}
impl ::core::clone::Clone for CredentialSaveOption {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CredentialSaveOption {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CredentialSaveOption {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CredentialSaveOption {}
unsafe impl ::windows::core::RuntimeType for CredentialSaveOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.UI.CredentialSaveOption;i4)");
}
impl ::windows::core::DefaultType for CredentialSaveOption {
    type DefaultType = Self;
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICredentialPickerOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICredentialPickerOptions {
    type Vtable = ICredentialPickerOptionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x965a0b4c_95fa_467f_992b_0b22e5859bf6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialPickerOptionsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AuthenticationProtocol) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AuthenticationProtocol) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CredentialSaveOption) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CredentialSaveOption) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICredentialPickerResults(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICredentialPickerResults {
    type Vtable = ICredentialPickerResultsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1948f99a_cc30_410c_9c38_cc0884c5b3d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialPickerResultsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CredentialSaveOption) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICredentialPickerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICredentialPickerStatics {
    type Vtable = ICredentialPickerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa3a5c73_c9ea_4782_99fb_e6d7e938e12d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialPickerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, caption: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserConsentVerifierStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserConsentVerifierStatics {
    type Vtable = IUserConsentVerifierStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf4f3f91_564c_4ddc_b8b5_973447627c65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserConsentVerifierStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: 'Security_Credentials_UI'*"]
#[repr(transparent)]
pub struct UserConsentVerificationResult(pub i32);
impl UserConsentVerificationResult {
    pub const Verified: Self = Self(0i32);
    pub const DeviceNotPresent: Self = Self(1i32);
    pub const NotConfiguredForUser: Self = Self(2i32);
    pub const DisabledByPolicy: Self = Self(3i32);
    pub const DeviceBusy: Self = Self(4i32);
    pub const RetriesExhausted: Self = Self(5i32);
    pub const Canceled: Self = Self(6i32);
}
impl ::core::marker::Copy for UserConsentVerificationResult {}
impl ::core::clone::Clone for UserConsentVerificationResult {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UserConsentVerificationResult {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UserConsentVerificationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserConsentVerificationResult {}
unsafe impl ::windows::core::RuntimeType for UserConsentVerificationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.UI.UserConsentVerificationResult;i4)");
}
impl ::windows::core::DefaultType for UserConsentVerificationResult {
    type DefaultType = Self;
}
pub struct UserConsentVerifier {}
impl UserConsentVerifier {
    #[cfg(feature = "Foundation")]
    pub fn CheckAvailabilityAsync() -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<UserConsentVerifierAvailability>> {
        Self::IUserConsentVerifierStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<UserConsentVerifierAvailability>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestVerificationAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(message: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<UserConsentVerificationResult>> {
        Self::IUserConsentVerifierStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), message.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<UserConsentVerificationResult>>(result__)
        })
    }
    pub fn IUserConsentVerifierStatics<R, F: FnOnce(&IUserConsentVerifierStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserConsentVerifier, IUserConsentVerifierStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for UserConsentVerifier {
    const NAME: &'static str = "Windows.Security.Credentials.UI.UserConsentVerifier";
}
#[doc = "*Required features: 'Security_Credentials_UI'*"]
#[repr(transparent)]
pub struct UserConsentVerifierAvailability(pub i32);
impl UserConsentVerifierAvailability {
    pub const Available: Self = Self(0i32);
    pub const DeviceNotPresent: Self = Self(1i32);
    pub const NotConfiguredForUser: Self = Self(2i32);
    pub const DisabledByPolicy: Self = Self(3i32);
    pub const DeviceBusy: Self = Self(4i32);
}
impl ::core::marker::Copy for UserConsentVerifierAvailability {}
impl ::core::clone::Clone for UserConsentVerifierAvailability {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UserConsentVerifierAvailability {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UserConsentVerifierAvailability {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserConsentVerifierAvailability {}
unsafe impl ::windows::core::RuntimeType for UserConsentVerifierAvailability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.UI.UserConsentVerifierAvailability;i4)");
}
impl ::windows::core::DefaultType for UserConsentVerifierAvailability {
    type DefaultType = Self;
}
