#[cfg(feature = "Security_Authentication_Web_Core")]
pub mod Core;
#[cfg(feature = "Security_Authentication_Web_Provider")]
pub mod Provider;
windows_core::imp::define_interface!(IWebAuthenticationBrokerStatics, IWebAuthenticationBrokerStatics_Vtbl, 0x2f149f1a_e673_40b5_bc22_201a6864a37b);
impl windows_core::RuntimeType for IWebAuthenticationBrokerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAuthenticationBrokerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AuthenticateWithCallbackUriAsync: unsafe extern "system" fn(*mut core::ffi::c_void, WebAuthenticationOptions, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AuthenticateWithoutCallbackUriAsync: unsafe extern "system" fn(*mut core::ffi::c_void, WebAuthenticationOptions, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentApplicationCallbackUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebAuthenticationBrokerStatics2, IWebAuthenticationBrokerStatics2_Vtbl, 0x73cdfb9e_14e7_41da_a971_aaf4410b621e);
impl windows_core::RuntimeType for IWebAuthenticationBrokerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAuthenticationBrokerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AuthenticateAndContinue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AuthenticateWithCallbackUriAndContinue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, WebAuthenticationOptions) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue: usize,
    pub AuthenticateSilentlyAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AuthenticateSilentlyWithOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WebAuthenticationOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWebAuthenticationResult, IWebAuthenticationResult_Vtbl, 0x64002b4b_ede9_470a_a5cd_0323faf6e262);
impl windows_core::RuntimeType for IWebAuthenticationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWebAuthenticationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResponseData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ResponseStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WebAuthenticationStatus) -> windows_core::HRESULT,
    pub ResponseErrorDetail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub struct WebAuthenticationBroker;
impl WebAuthenticationBroker {}
impl windows_core::RuntimeName for WebAuthenticationBroker {
    const NAME: &'static str = "Windows.Security.Authentication.Web.WebAuthenticationBroker";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct WebAuthenticationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WebAuthenticationResult, windows_core::IUnknown, windows_core::IInspectable);
impl WebAuthenticationResult {
    pub fn ResponseData(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseData)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ResponseStatus(&self) -> windows_core::Result<WebAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseStatus)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ResponseErrorDetail(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResponseErrorDetail)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WebAuthenticationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWebAuthenticationResult>();
}
unsafe impl windows_core::Interface for WebAuthenticationResult {
    type Vtable = <IWebAuthenticationResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWebAuthenticationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WebAuthenticationResult {
    const NAME: &'static str = "Windows.Security.Authentication.Web.WebAuthenticationResult";
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TokenBindingKeyType(pub i32);
impl TokenBindingKeyType {
    pub const Rsa2048: Self = Self(0i32);
    pub const EcdsaP256: Self = Self(1i32);
    pub const AnyExisting: Self = Self(2i32);
}
impl windows_core::TypeKind for TokenBindingKeyType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TokenBindingKeyType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.TokenBindingKeyType;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WebAuthenticationOptions(pub u32);
impl WebAuthenticationOptions {
    pub const None: Self = Self(0u32);
    pub const SilentMode: Self = Self(1u32);
    pub const UseTitle: Self = Self(2u32);
    pub const UseHttpPost: Self = Self(4u32);
    pub const UseCorporateNetwork: Self = Self(8u32);
}
impl windows_core::TypeKind for WebAuthenticationOptions {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for WebAuthenticationOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.WebAuthenticationOptions;u4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WebAuthenticationStatus(pub i32);
impl WebAuthenticationStatus {
    pub const Success: Self = Self(0i32);
    pub const UserCancel: Self = Self(1i32);
    pub const ErrorHttp: Self = Self(2i32);
}
impl windows_core::TypeKind for WebAuthenticationStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for WebAuthenticationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.WebAuthenticationStatus;i4)");
}
