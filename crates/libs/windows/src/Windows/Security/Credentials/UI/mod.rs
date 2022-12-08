#[doc(hidden)]
#[repr(transparent)]
pub struct ICredentialPickerOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICredentialPickerOptions {
    type Vtable = ICredentialPickerOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for ICredentialPickerOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x965a0b4c_95fa_467f_992b_0b22e5859bf6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialPickerOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetCaption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Caption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetTargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TargetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAuthenticationProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AuthenticationProtocol) -> ::windows::core::HRESULT,
    pub AuthenticationProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AuthenticationProtocol) -> ::windows::core::HRESULT,
    pub SetCustomAuthenticationProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CustomAuthenticationProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetPreviousCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetPreviousCredential: usize,
    #[cfg(feature = "Storage_Streams")]
    pub PreviousCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PreviousCredential: usize,
    pub SetAlwaysDisplayDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AlwaysDisplayDialog: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCallerSavesCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CallerSavesCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCredentialSaveOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CredentialSaveOption) -> ::windows::core::HRESULT,
    pub CredentialSaveOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CredentialSaveOption) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICredentialPickerResults(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICredentialPickerResults {
    type Vtable = ICredentialPickerResults_Vtbl;
}
unsafe impl ::windows::core::Interface for ICredentialPickerResults {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1948f99a_cc30_410c_9c38_cc0884c5b3d7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialPickerResults_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub CredentialSaveOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CredentialSaveOption) -> ::windows::core::HRESULT,
    pub CredentialSaved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Credential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Credential: usize,
    pub CredentialDomainName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CredentialUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CredentialPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICredentialPickerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICredentialPickerStatics {
    type Vtable = ICredentialPickerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICredentialPickerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa3a5c73_c9ea_4782_99fb_e6d7e938e12d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialPickerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PickWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PickWithMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickWithMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PickWithCaptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetname: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, caption: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PickWithCaptionAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserConsentVerifierStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserConsentVerifierStatics {
    type Vtable = IUserConsentVerifierStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserConsentVerifierStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf4f3f91_564c_4ddc_b8b5_973447627c65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserConsentVerifierStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CheckAvailabilityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckAvailabilityAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestVerificationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestVerificationAsync: usize,
}
#[doc = "*Required features: `\"Security_Credentials_UI\"`*"]
pub struct CredentialPicker;
impl CredentialPicker {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PickWithOptionsAsync(options: &CredentialPickerOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CredentialPickerResults>> {
        Self::ICredentialPickerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PickWithOptionsAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(options), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PickWithMessageAsync(targetname: &::windows::core::HSTRING, message: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CredentialPickerResults>> {
        Self::ICredentialPickerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PickWithMessageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(targetname), ::core::mem::transmute_copy(message), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PickWithCaptionAsync(targetname: &::windows::core::HSTRING, message: &::windows::core::HSTRING, caption: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CredentialPickerResults>> {
        Self::ICredentialPickerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PickWithCaptionAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(targetname), ::core::mem::transmute_copy(message), ::core::mem::transmute_copy(caption), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICredentialPickerStatics<R, F: FnOnce(&ICredentialPickerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CredentialPicker, ICredentialPickerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for CredentialPicker {
    const NAME: &'static str = "Windows.Security.Credentials.UI.CredentialPicker";
}
#[doc = "*Required features: `\"Security_Credentials_UI\"`*"]
#[repr(transparent)]
pub struct CredentialPickerOptions(::windows::core::IUnknown);
impl CredentialPickerOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CredentialPickerOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetCaption(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCaption)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Caption(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Caption)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetMessage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMessage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Message)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetErrorCode(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetErrorCode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ErrorCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTargetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTargetName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TargetName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TargetName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAuthenticationProtocol(&self, value: AuthenticationProtocol) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAuthenticationProtocol)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AuthenticationProtocol(&self) -> ::windows::core::Result<AuthenticationProtocol> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AuthenticationProtocol)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCustomAuthenticationProtocol(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCustomAuthenticationProtocol)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn CustomAuthenticationProtocol(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CustomAuthenticationProtocol)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetPreviousCredential<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPreviousCredential)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn PreviousCredential(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviousCredential)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetAlwaysDisplayDialog(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAlwaysDisplayDialog)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn AlwaysDisplayDialog(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AlwaysDisplayDialog)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCallerSavesCredential(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCallerSavesCredential)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn CallerSavesCredential(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CallerSavesCredential)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCredentialSaveOption(&self, value: CredentialSaveOption) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCredentialSaveOption)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn CredentialSaveOption(&self) -> ::windows::core::Result<CredentialSaveOption> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CredentialSaveOption)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
impl ::core::fmt::Debug for CredentialPickerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CredentialPickerOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CredentialPickerOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.UI.CredentialPickerOptions;{965a0b4c-95fa-467f-992b-0b22e5859bf6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CredentialPickerOptions {
    type Vtable = ICredentialPickerOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for CredentialPickerOptions {
    const IID: ::windows::core::GUID = <ICredentialPickerOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CredentialPickerOptions {
    const NAME: &'static str = "Windows.Security.Credentials.UI.CredentialPickerOptions";
}
::windows::core::interface_hierarchy!(CredentialPickerOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Security_Credentials_UI\"`*"]
#[repr(transparent)]
pub struct CredentialPickerResults(::windows::core::IUnknown);
impl CredentialPickerResults {
    pub fn ErrorCode(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CredentialSaveOption(&self) -> ::windows::core::Result<CredentialSaveOption> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CredentialSaveOption)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CredentialSaved(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CredentialSaved)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Credential(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Credential)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CredentialDomainName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CredentialDomainName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CredentialUserName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CredentialUserName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CredentialPassword(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CredentialPassword)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
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
impl ::core::fmt::Debug for CredentialPickerResults {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CredentialPickerResults").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CredentialPickerResults {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Credentials.UI.CredentialPickerResults;{1948f99a-cc30-410c-9c38-cc0884c5b3d7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CredentialPickerResults {
    type Vtable = ICredentialPickerResults_Vtbl;
}
unsafe impl ::windows::core::Interface for CredentialPickerResults {
    const IID: ::windows::core::GUID = <ICredentialPickerResults as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CredentialPickerResults {
    const NAME: &'static str = "Windows.Security.Credentials.UI.CredentialPickerResults";
}
::windows::core::interface_hierarchy!(CredentialPickerResults, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Security_Credentials_UI\"`*"]
pub struct UserConsentVerifier;
impl UserConsentVerifier {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CheckAvailabilityAsync() -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<UserConsentVerifierAvailability>> {
        Self::IUserConsentVerifierStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CheckAvailabilityAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestVerificationAsync(message: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<UserConsentVerificationResult>> {
        Self::IUserConsentVerifierStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestVerificationAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(message), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserConsentVerifierStatics<R, F: FnOnce(&IUserConsentVerifierStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<UserConsentVerifier, IUserConsentVerifierStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for UserConsentVerifier {
    const NAME: &'static str = "Windows.Security.Credentials.UI.UserConsentVerifier";
}
#[doc = "*Required features: `\"Security_Credentials_UI\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for AuthenticationProtocol {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AuthenticationProtocol {
    type Abi = Self;
}
impl ::core::fmt::Debug for AuthenticationProtocol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AuthenticationProtocol").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AuthenticationProtocol {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.UI.AuthenticationProtocol;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Credentials_UI\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for CredentialSaveOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CredentialSaveOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for CredentialSaveOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CredentialSaveOption").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CredentialSaveOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.UI.CredentialSaveOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Credentials_UI\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for UserConsentVerificationResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserConsentVerificationResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserConsentVerificationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserConsentVerificationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserConsentVerificationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.UI.UserConsentVerificationResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Security_Credentials_UI\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for UserConsentVerifierAvailability {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserConsentVerifierAvailability {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserConsentVerifierAvailability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserConsentVerifierAvailability").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserConsentVerifierAvailability {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Credentials.UI.UserConsentVerifierAvailability;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
