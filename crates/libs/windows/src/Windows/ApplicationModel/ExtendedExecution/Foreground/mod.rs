::windows_core::imp::com_interface!(IExtendedExecutionForegroundRevokedEventArgs, IExtendedExecutionForegroundRevokedEventArgs_Vtbl, 0xb07cd940_9557_aea4_2c99_bdd56d9be461);
#[repr(C)]
pub struct IExtendedExecutionForegroundRevokedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ExtendedExecutionForegroundRevokedReason) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IExtendedExecutionForegroundSession, IExtendedExecutionForegroundSession_Vtbl, 0xfbf440e1_9d10_4201_b01e_c83275296f2e);
#[repr(C)]
pub struct IExtendedExecutionForegroundSession_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Description: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Revoked: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveRevoked: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RequestExtensionAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Reason: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ExtendedExecutionForegroundReason) -> ::windows_core::HRESULT,
    pub SetReason: unsafe extern "system" fn(*mut ::core::ffi::c_void, ExtendedExecutionForegroundReason) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ExtendedExecutionForegroundRevokedEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(ExtendedExecutionForegroundRevokedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ExtendedExecutionForegroundRevokedEventArgs {
    pub fn Reason(&self) -> ::windows_core::Result<ExtendedExecutionForegroundRevokedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl ::windows_core::RuntimeType for ExtendedExecutionForegroundRevokedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ExtendedExecutionForegroundRevokedEventArgs {
    type Vtable = IExtendedExecutionForegroundRevokedEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IExtendedExecutionForegroundRevokedEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ExtendedExecutionForegroundRevokedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundRevokedEventArgs";
}
unsafe impl ::core::marker::Send for ExtendedExecutionForegroundRevokedEventArgs {}
unsafe impl ::core::marker::Sync for ExtendedExecutionForegroundRevokedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ExtendedExecutionForegroundSession(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(ExtendedExecutionForegroundSession, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(ExtendedExecutionForegroundSession, super::super::super::Foundation::IClosable);
impl ExtendedExecutionForegroundSession {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ExtendedExecutionForegroundSession, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Description(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Description)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Revoked<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<::windows_core::IInspectable, ExtendedExecutionForegroundRevokedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Revoked)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveRevoked(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveRevoked)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn RequestExtensionAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<ExtendedExecutionForegroundResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestExtensionAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Reason(&self) -> ::windows_core::Result<ExtendedExecutionForegroundReason> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetReason(&self, value: ExtendedExecutionForegroundReason) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReason)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::windows_core::RuntimeType for ExtendedExecutionForegroundSession {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ExtendedExecutionForegroundSession {
    type Vtable = IExtendedExecutionForegroundSession_Vtbl;
    const IID: ::windows_core::GUID = <IExtendedExecutionForegroundSession as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for ExtendedExecutionForegroundSession {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundSession";
}
unsafe impl ::core::marker::Send for ExtendedExecutionForegroundSession {}
unsafe impl ::core::marker::Sync for ExtendedExecutionForegroundSession {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct ExtendedExecutionForegroundReason(pub i32);
impl ExtendedExecutionForegroundReason {
    pub const Unspecified: Self = Self(0i32);
    pub const SavingData: Self = Self(1i32);
    pub const BackgroundAudio: Self = Self(2i32);
    pub const Unconstrained: Self = Self(3i32);
}
impl ::windows_core::TypeKind for ExtendedExecutionForegroundReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ExtendedExecutionForegroundReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedExecutionForegroundReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ExtendedExecutionForegroundReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundReason;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct ExtendedExecutionForegroundResult(pub i32);
impl ExtendedExecutionForegroundResult {
    pub const Allowed: Self = Self(0i32);
    pub const Denied: Self = Self(1i32);
}
impl ::windows_core::TypeKind for ExtendedExecutionForegroundResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ExtendedExecutionForegroundResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedExecutionForegroundResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ExtendedExecutionForegroundResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundResult;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct ExtendedExecutionForegroundRevokedReason(pub i32);
impl ExtendedExecutionForegroundRevokedReason {
    pub const Resumed: Self = Self(0i32);
    pub const SystemPolicy: Self = Self(1i32);
}
impl ::windows_core::TypeKind for ExtendedExecutionForegroundRevokedReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ExtendedExecutionForegroundRevokedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedExecutionForegroundRevokedReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ExtendedExecutionForegroundRevokedReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundRevokedReason;i4)");
}
