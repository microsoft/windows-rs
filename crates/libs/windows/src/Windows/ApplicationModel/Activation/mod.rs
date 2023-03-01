#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IActivatedEventArgs(::windows::core::IUnknown);
impl IActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivatedEventArgs {}
impl ::core::fmt::Debug for IActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{cf651713-cd08-4fd8-b697-a281b6544e2e}");
}
unsafe impl ::windows::core::Interface for IActivatedEventArgs {
    type Vtable = IActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf651713_cd08_4fd8_b697_a281b6544e2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ActivationKind) -> ::windows::core::HRESULT,
    pub PreviousExecutionState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ApplicationExecutionState) -> ::windows::core::HRESULT,
    pub SplashScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IActivatedEventArgsWithUser(::windows::core::IUnknown);
impl IActivatedEventArgsWithUser {
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IActivatedEventArgsWithUser, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IActivatedEventArgsWithUser {}
impl ::core::cmp::PartialEq for IActivatedEventArgsWithUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivatedEventArgsWithUser {}
impl ::core::fmt::Debug for IActivatedEventArgsWithUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivatedEventArgsWithUser").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IActivatedEventArgsWithUser {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{1cf09b9e-9962-4936-80ff-afc8e8ae5c8c}");
}
unsafe impl ::windows::core::Interface for IActivatedEventArgsWithUser {
    type Vtable = IActivatedEventArgsWithUser_Vtbl;
}
impl ::core::clone::Clone for IActivatedEventArgsWithUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActivatedEventArgsWithUser {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cf09b9e_9962_4936_80ff_afc8e8ae5c8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedEventArgsWithUser_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IApplicationViewActivatedEventArgs(::windows::core::IUnknown);
impl IApplicationViewActivatedEventArgs {
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IApplicationViewActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IApplicationViewActivatedEventArgs {}
impl ::core::cmp::PartialEq for IApplicationViewActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IApplicationViewActivatedEventArgs {}
impl ::core::fmt::Debug for IApplicationViewActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IApplicationViewActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IApplicationViewActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{930cef4b-b829-40fc-88f4-8513e8a64738}");
}
unsafe impl ::windows::core::Interface for IApplicationViewActivatedEventArgs {
    type Vtable = IApplicationViewActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IApplicationViewActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IApplicationViewActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x930cef4b_b829_40fc_88f4_8513e8a64738);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CurrentlyShownApplicationViewId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IAppointmentsProviderActivatedEventArgs(::windows::core::IUnknown);
impl IAppointmentsProviderActivatedEventArgs {
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IAppointmentsProviderActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {}
impl ::core::cmp::PartialEq for IAppointmentsProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderActivatedEventArgs {}
impl ::core::fmt::Debug for IAppointmentsProviderActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppointmentsProviderActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IAppointmentsProviderActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{3364c405-933c-4e7d-a034-500fb8dcd9f3}");
}
unsafe impl ::windows::core::Interface for IAppointmentsProviderActivatedEventArgs {
    type Vtable = IAppointmentsProviderActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppointmentsProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppointmentsProviderActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3364c405_933c_4e7d_a034_500fb8dcd9f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Verb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IAppointmentsProviderAddAppointmentActivatedEventArgs(::windows::core::IUnknown);
impl IAppointmentsProviderAddAppointmentActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn AddAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Appointments::AppointmentsProvider::AddAppointmentOperation>();
            (::windows::core::Interface::vtable(this).AddAppointmentOperation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IAppointmentsProviderAddAppointmentActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IAppointmentsProviderAddAppointmentActivatedEventArgs {}
impl windows::core::CanTryInto<IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderAddAppointmentActivatedEventArgs {}
impl ::core::cmp::PartialEq for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderAddAppointmentActivatedEventArgs {}
impl ::core::fmt::Debug for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppointmentsProviderAddAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{a2861367-cee5-4e4d-9ed7-41c34ec18b02}");
}
unsafe impl ::windows::core::Interface for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderAddAppointmentActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2861367_cee5_4e4d_9ed7_41c34ec18b02);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderAddAppointmentActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub AddAppointmentOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    AddAppointmentOperation: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IAppointmentsProviderRemoveAppointmentActivatedEventArgs(::windows::core::IUnknown);
impl IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn RemoveAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation>();
            (::windows::core::Interface::vtable(this).RemoveAppointmentOperation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IAppointmentsProviderRemoveAppointmentActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {}
impl windows::core::CanTryInto<IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {}
impl ::core::cmp::PartialEq for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {}
impl ::core::fmt::Debug for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppointmentsProviderRemoveAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{751f3ab8-0b8e-451c-9f15-966e699bac25}");
}
unsafe impl ::windows::core::Interface for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x751f3ab8_0b8e_451c_9f15_966e699bac25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub RemoveAppointmentOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    RemoveAppointmentOperation: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IAppointmentsProviderReplaceAppointmentActivatedEventArgs(::windows::core::IUnknown);
impl IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn ReplaceAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation>();
            (::windows::core::Interface::vtable(this).ReplaceAppointmentOperation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IAppointmentsProviderReplaceAppointmentActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {}
impl windows::core::CanTryInto<IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {}
impl ::core::cmp::PartialEq for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {}
impl ::core::fmt::Debug for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppointmentsProviderReplaceAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{1551b7d4-a981-4067-8a62-0524e4ade121}");
}
unsafe impl ::windows::core::Interface for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1551b7d4_a981_4067_8a62_0524e4ade121);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub ReplaceAppointmentOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    ReplaceAppointmentOperation: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs(::windows::core::IUnknown);
impl IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InstanceStartDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>();
            (::windows::core::Interface::vtable(this).InstanceStartDate)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).LocalId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RoamingId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).RoamingId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
impl windows::core::CanTryInto<IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
impl ::core::cmp::PartialEq for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
impl ::core::fmt::Debug for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{3958f065-9841-4ca5-999b-885198b9ef2a}");
}
unsafe impl ::windows::core::Interface for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3958f065_9841_4ca5_999b_885198b9ef2a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub InstanceStartDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InstanceStartDate: usize,
    pub LocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RoamingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IAppointmentsProviderShowTimeFrameActivatedEventArgs(::windows::core::IUnknown);
impl IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeToShow(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).TimeToShow)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).Duration)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IAppointmentsProviderShowTimeFrameActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {}
impl windows::core::CanTryInto<IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {}
impl ::core::cmp::PartialEq for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppointmentsProviderShowTimeFrameActivatedEventArgs {}
impl ::core::fmt::Debug for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppointmentsProviderShowTimeFrameActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{9baeaba6-0e0b-49aa-babc-12b1dc774986}");
}
unsafe impl ::windows::core::Interface for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowTimeFrameActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9baeaba6_0e0b_49aa_babc_12b1dc774986);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderShowTimeFrameActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub TimeToShow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeToShow: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IBackgroundActivatedEventArgs(::windows::core::IUnknown);
impl IBackgroundActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn TaskInstance(&self) -> ::windows::core::Result<super::Background::IBackgroundTaskInstance> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Background::IBackgroundTaskInstance>();
            (::windows::core::Interface::vtable(this).TaskInstance)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IBackgroundActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IBackgroundActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundActivatedEventArgs {}
impl ::core::fmt::Debug for IBackgroundActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IBackgroundActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{ab14bee0-e760-440e-a91c-44796de3a92d}");
}
unsafe impl ::windows::core::Interface for IBackgroundActivatedEventArgs {
    type Vtable = IBackgroundActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IBackgroundActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab14bee0_e760_440e_a91c_44796de3a92d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Background")]
    pub TaskInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))]
    TaskInstance: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IBarcodeScannerPreviewActivatedEventArgs(::windows::core::IUnknown);
impl IBarcodeScannerPreviewActivatedEventArgs {
    pub fn ConnectionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ConnectionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IBarcodeScannerPreviewActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IBarcodeScannerPreviewActivatedEventArgs {}
impl ::core::cmp::PartialEq for IBarcodeScannerPreviewActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBarcodeScannerPreviewActivatedEventArgs {}
impl ::core::fmt::Debug for IBarcodeScannerPreviewActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBarcodeScannerPreviewActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IBarcodeScannerPreviewActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{6772797c-99bf-4349-af22-e4123560371c}");
}
unsafe impl ::windows::core::Interface for IBarcodeScannerPreviewActivatedEventArgs {
    type Vtable = IBarcodeScannerPreviewActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IBarcodeScannerPreviewActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBarcodeScannerPreviewActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6772797c_99bf_4349_af22_e4123560371c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerPreviewActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ConnectionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ICachedFileUpdaterActivatedEventArgs(::windows::core::IUnknown);
impl ICachedFileUpdaterActivatedEventArgs {
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    #[cfg(feature = "Storage_Provider")]
    pub fn CachedFileUpdaterUI(&self) -> ::windows::core::Result<super::super::Storage::Provider::CachedFileUpdaterUI> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Provider::CachedFileUpdaterUI>();
            (::windows::core::Interface::vtable(this).CachedFileUpdaterUI)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(ICachedFileUpdaterActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for ICachedFileUpdaterActivatedEventArgs {}
impl ::core::cmp::PartialEq for ICachedFileUpdaterActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICachedFileUpdaterActivatedEventArgs {}
impl ::core::fmt::Debug for ICachedFileUpdaterActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICachedFileUpdaterActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ICachedFileUpdaterActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{d06eb1c7-3805-4ecb-b757-6cf15e26fef3}");
}
unsafe impl ::windows::core::Interface for ICachedFileUpdaterActivatedEventArgs {
    type Vtable = ICachedFileUpdaterActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ICachedFileUpdaterActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICachedFileUpdaterActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd06eb1c7_3805_4ecb_b757_6cf15e26fef3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Provider")]
    pub CachedFileUpdaterUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Provider"))]
    CachedFileUpdaterUI: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ICameraSettingsActivatedEventArgs(::windows::core::IUnknown);
impl ICameraSettingsActivatedEventArgs {
    pub fn VideoDeviceController(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).VideoDeviceController)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VideoDeviceExtension(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).VideoDeviceExtension)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(ICameraSettingsActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for ICameraSettingsActivatedEventArgs {}
impl ::core::cmp::PartialEq for ICameraSettingsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICameraSettingsActivatedEventArgs {}
impl ::core::fmt::Debug for ICameraSettingsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICameraSettingsActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ICameraSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{fb67a508-2dad-490a-9170-dca036eb114b}");
}
unsafe impl ::windows::core::Interface for ICameraSettingsActivatedEventArgs {
    type Vtable = ICameraSettingsActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ICameraSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICameraSettingsActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb67a508_2dad_490a_9170_dca036eb114b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraSettingsActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub VideoDeviceController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub VideoDeviceExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ICommandLineActivatedEventArgs(::windows::core::IUnknown);
impl ICommandLineActivatedEventArgs {
    pub fn Operation(&self) -> ::windows::core::Result<CommandLineActivationOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CommandLineActivationOperation>();
            (::windows::core::Interface::vtable(this).Operation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(ICommandLineActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for ICommandLineActivatedEventArgs {}
impl ::core::cmp::PartialEq for ICommandLineActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommandLineActivatedEventArgs {}
impl ::core::fmt::Debug for ICommandLineActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommandLineActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ICommandLineActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{4506472c-006a-48eb-8afb-d07ab25e3366}");
}
unsafe impl ::windows::core::Interface for ICommandLineActivatedEventArgs {
    type Vtable = ICommandLineActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ICommandLineActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICommandLineActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4506472c_006a_48eb_8afb_d07ab25e3366);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandLineActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommandLineActivationOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICommandLineActivationOperation {
    type Vtable = ICommandLineActivationOperation_Vtbl;
}
impl ::core::clone::Clone for ICommandLineActivationOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICommandLineActivationOperation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x994b2841_c59e_4f69_bcfd_b61ed4e622eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandLineActivationOperation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CurrentDirectoryPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetExitCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub ExitCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IContactActivatedEventArgs(::windows::core::IUnknown);
impl IContactActivatedEventArgs {
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IContactActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IContactActivatedEventArgs {}
impl ::core::cmp::PartialEq for IContactActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactActivatedEventArgs {}
impl ::core::fmt::Debug for IContactActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IContactActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{d627a1c4-c025-4c41-9def-f1eafad075e7}");
}
unsafe impl ::windows::core::Interface for IContactActivatedEventArgs {
    type Vtable = IContactActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IContactActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IContactActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd627a1c4_c025_4c41_9def_f1eafad075e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Verb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IContactCallActivatedEventArgs(::windows::core::IUnknown);
impl IContactCallActivatedEventArgs {
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ServiceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ServiceUserId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Contacts::Contact>();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IContactCallActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IContactCallActivatedEventArgs {}
impl windows::core::CanTryInto<IContactActivatedEventArgs> for IContactCallActivatedEventArgs {}
impl ::core::cmp::PartialEq for IContactCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactCallActivatedEventArgs {}
impl ::core::fmt::Debug for IContactCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactCallActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IContactCallActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{c2df14c7-30eb-41c6-b3bc-5b1694f9dab3}");
}
unsafe impl ::windows::core::Interface for IContactCallActivatedEventArgs {
    type Vtable = IContactCallActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IContactCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IContactCallActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2df14c7_30eb_41c6_b3bc_5b1694f9dab3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactCallActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IContactMapActivatedEventArgs(::windows::core::IUnknown);
impl IContactMapActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Address(&self) -> ::windows::core::Result<super::Contacts::ContactAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Contacts::ContactAddress>();
            (::windows::core::Interface::vtable(this).Address)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Contacts::Contact>();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IContactMapActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IContactMapActivatedEventArgs {}
impl windows::core::CanTryInto<IContactActivatedEventArgs> for IContactMapActivatedEventArgs {}
impl ::core::cmp::PartialEq for IContactMapActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactMapActivatedEventArgs {}
impl ::core::fmt::Debug for IContactMapActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactMapActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IContactMapActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{b32bf870-eee7-4ad2-aaf1-a87effcf00a4}");
}
unsafe impl ::windows::core::Interface for IContactMapActivatedEventArgs {
    type Vtable = IContactMapActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IContactMapActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IContactMapActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb32bf870_eee7_4ad2_aaf1_a87effcf00a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactMapActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Address: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IContactMessageActivatedEventArgs(::windows::core::IUnknown);
impl IContactMessageActivatedEventArgs {
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ServiceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ServiceUserId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Contacts::Contact>();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IContactMessageActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IContactMessageActivatedEventArgs {}
impl windows::core::CanTryInto<IContactActivatedEventArgs> for IContactMessageActivatedEventArgs {}
impl ::core::cmp::PartialEq for IContactMessageActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactMessageActivatedEventArgs {}
impl ::core::fmt::Debug for IContactMessageActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactMessageActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IContactMessageActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{de598db2-0e03-43b0-bf56-bcc40b3162df}");
}
unsafe impl ::windows::core::Interface for IContactMessageActivatedEventArgs {
    type Vtable = IContactMessageActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IContactMessageActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IContactMessageActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde598db2_0e03_43b0_bf56_bcc40b3162df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactMessageActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IContactPanelActivatedEventArgs(::windows::core::IUnknown);
impl IContactPanelActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn ContactPanel(&self) -> ::windows::core::Result<super::Contacts::ContactPanel> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Contacts::ContactPanel>();
            (::windows::core::Interface::vtable(this).ContactPanel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Contacts::Contact>();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IContactPanelActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IContactPanelActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactPanelActivatedEventArgs {}
impl ::core::fmt::Debug for IContactPanelActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactPanelActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IContactPanelActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{52bb63e4-d3d4-4b63-8051-4af2082cab80}");
}
unsafe impl ::windows::core::Interface for IContactPanelActivatedEventArgs {
    type Vtable = IContactPanelActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IContactPanelActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IContactPanelActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52bb63e4_d3d4_4b63_8051_4af2082cab80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPanelActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub ContactPanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    ContactPanel: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IContactPickerActivatedEventArgs(::windows::core::IUnknown);
impl IContactPickerActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Contacts_Provider\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    pub fn ContactPickerUI(&self) -> ::windows::core::Result<super::Contacts::Provider::ContactPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Contacts::Provider::ContactPickerUI>();
            (::windows::core::Interface::vtable(this).ContactPickerUI)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IContactPickerActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IContactPickerActivatedEventArgs {}
impl ::core::cmp::PartialEq for IContactPickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactPickerActivatedEventArgs {}
impl ::core::fmt::Debug for IContactPickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactPickerActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IContactPickerActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{ce57aae7-6449-45a7-971f-d113be7a8936}");
}
unsafe impl ::windows::core::Interface for IContactPickerActivatedEventArgs {
    type Vtable = IContactPickerActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IContactPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IContactPickerActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce57aae7_6449_45a7_971f_d113be7a8936);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPickerActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    pub ContactPickerUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts_Provider"))]
    ContactPickerUI: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IContactPostActivatedEventArgs(::windows::core::IUnknown);
impl IContactPostActivatedEventArgs {
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ServiceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ServiceUserId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Contacts::Contact>();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IContactPostActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IContactPostActivatedEventArgs {}
impl windows::core::CanTryInto<IContactActivatedEventArgs> for IContactPostActivatedEventArgs {}
impl ::core::cmp::PartialEq for IContactPostActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactPostActivatedEventArgs {}
impl ::core::fmt::Debug for IContactPostActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactPostActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IContactPostActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{b35a3c67-f1e7-4655-ad6e-4857588f552f}");
}
unsafe impl ::windows::core::Interface for IContactPostActivatedEventArgs {
    type Vtable = IContactPostActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IContactPostActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IContactPostActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb35a3c67_f1e7_4655_ad6e_4857588f552f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPostActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IContactVideoCallActivatedEventArgs(::windows::core::IUnknown);
impl IContactVideoCallActivatedEventArgs {
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ServiceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ServiceUserId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Contacts::Contact>();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IContactVideoCallActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IContactVideoCallActivatedEventArgs {}
impl windows::core::CanTryInto<IContactActivatedEventArgs> for IContactVideoCallActivatedEventArgs {}
impl ::core::cmp::PartialEq for IContactVideoCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactVideoCallActivatedEventArgs {}
impl ::core::fmt::Debug for IContactVideoCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactVideoCallActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IContactVideoCallActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{61079db8-e3e7-4b4f-858d-5c63a96ef684}");
}
unsafe impl ::windows::core::Interface for IContactVideoCallActivatedEventArgs {
    type Vtable = IContactVideoCallActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IContactVideoCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IContactVideoCallActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61079db8_e3e7_4b4f_858d_5c63a96ef684);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactVideoCallActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IContactsProviderActivatedEventArgs(::windows::core::IUnknown);
impl IContactsProviderActivatedEventArgs {
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IContactsProviderActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IContactsProviderActivatedEventArgs {}
impl ::core::cmp::PartialEq for IContactsProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContactsProviderActivatedEventArgs {}
impl ::core::fmt::Debug for IContactsProviderActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContactsProviderActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IContactsProviderActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{4580dca8-5750-4916-aa52-c0829521eb94}");
}
unsafe impl ::windows::core::Interface for IContactsProviderActivatedEventArgs {
    type Vtable = IContactsProviderActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IContactsProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IContactsProviderActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4580dca8_5750_4916_aa52_c0829521eb94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactsProviderActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Verb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IContinuationActivatedEventArgs(::windows::core::IUnknown);
impl IContinuationActivatedEventArgs {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::ValueSet>();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IContinuationActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IContinuationActivatedEventArgs {}
impl ::core::cmp::PartialEq for IContinuationActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContinuationActivatedEventArgs {}
impl ::core::fmt::Debug for IContinuationActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContinuationActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IContinuationActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{e58106b5-155f-4a94-a742-c7e08f4e188c}");
}
unsafe impl ::windows::core::Interface for IContinuationActivatedEventArgs {
    type Vtable = IContinuationActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IContinuationActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IContinuationActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe58106b5_155f_4a94_a742_c7e08f4e188c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContinuationActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ContinuationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContinuationData: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IDeviceActivatedEventArgs(::windows::core::IUnknown);
impl IDeviceActivatedEventArgs {
    pub fn DeviceInformationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceInformationId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IDeviceActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IDeviceActivatedEventArgs {}
impl ::core::cmp::PartialEq for IDeviceActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDeviceActivatedEventArgs {}
impl ::core::fmt::Debug for IDeviceActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeviceActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IDeviceActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{cd50b9a9-ce10-44d2-8234-c355a073ef33}");
}
unsafe impl ::windows::core::Interface for IDeviceActivatedEventArgs {
    type Vtable = IDeviceActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IDeviceActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDeviceActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd50b9a9_ce10_44d2_8234_c355a073ef33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceInformationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Verb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IDevicePairingActivatedEventArgs(::windows::core::IUnknown);
impl IDevicePairingActivatedEventArgs {
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Devices::Enumeration::DeviceInformation>();
            (::windows::core::Interface::vtable(this).DeviceInformation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IDevicePairingActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IDevicePairingActivatedEventArgs {}
impl ::core::cmp::PartialEq for IDevicePairingActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDevicePairingActivatedEventArgs {}
impl ::core::fmt::Debug for IDevicePairingActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDevicePairingActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IDevicePairingActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{eba0d1e4-ecc6-4148-94ed-f4b37ec05b3e}");
}
unsafe impl ::windows::core::Interface for IDevicePairingActivatedEventArgs {
    type Vtable = IDevicePairingActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IDevicePairingActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDevicePairingActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba0d1e4_ecc6_4148_94ed_f4b37ec05b3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInformation: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IDialReceiverActivatedEventArgs(::windows::core::IUnknown);
impl IDialReceiverActivatedEventArgs {
    pub fn AppName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AppName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Arguments)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).TileId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IDialReceiverActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IDialReceiverActivatedEventArgs {}
impl windows::core::CanTryInto<ILaunchActivatedEventArgs> for IDialReceiverActivatedEventArgs {}
impl ::core::cmp::PartialEq for IDialReceiverActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDialReceiverActivatedEventArgs {}
impl ::core::fmt::Debug for IDialReceiverActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDialReceiverActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IDialReceiverActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{fb777ed7-85ee-456e-a44d-85d730e70aed}");
}
unsafe impl ::windows::core::Interface for IDialReceiverActivatedEventArgs {
    type Vtable = IDialReceiverActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IDialReceiverActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDialReceiverActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb777ed7_85ee_456e_a44d_85d730e70aed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialReceiverActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IFileActivatedEventArgs(::windows::core::IUnknown);
impl IFileActivatedEventArgs {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>();
            (::windows::core::Interface::vtable(this).Files)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IFileActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IFileActivatedEventArgs {}
impl ::core::cmp::PartialEq for IFileActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileActivatedEventArgs {}
impl ::core::fmt::Debug for IFileActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IFileActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{bb2afc33-93b1-42ed-8b26-236dd9c78496}");
}
unsafe impl ::windows::core::Interface for IFileActivatedEventArgs {
    type Vtable = IFileActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IFileActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFileActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb2afc33_93b1_42ed_8b26_236dd9c78496);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub Files: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    Files: usize,
    pub Verb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IFileActivatedEventArgsWithCallerPackageFamilyName(::windows::core::IUnknown);
impl IFileActivatedEventArgsWithCallerPackageFamilyName {
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).CallerPackageFamilyName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IFileActivatedEventArgsWithCallerPackageFamilyName, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IFileActivatedEventArgsWithCallerPackageFamilyName {}
impl ::core::cmp::PartialEq for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileActivatedEventArgsWithCallerPackageFamilyName {}
impl ::core::fmt::Debug for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileActivatedEventArgsWithCallerPackageFamilyName").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IFileActivatedEventArgsWithCallerPackageFamilyName {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{2d60f06b-d25f-4d25-8653-e1c5e1108309}");
}
unsafe impl ::windows::core::Interface for IFileActivatedEventArgsWithCallerPackageFamilyName {
    type Vtable = IFileActivatedEventArgsWithCallerPackageFamilyName_Vtbl;
}
impl ::core::clone::Clone for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFileActivatedEventArgsWithCallerPackageFamilyName {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d60f06b_d25f_4d25_8653_e1c5e1108309);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgsWithCallerPackageFamilyName_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IFileActivatedEventArgsWithNeighboringFiles(::windows::core::IUnknown);
impl IFileActivatedEventArgsWithNeighboringFiles {
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn NeighboringFilesQuery(&self) -> ::windows::core::Result<super::super::Storage::Search::StorageFileQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Search::StorageFileQueryResult>();
            (::windows::core::Interface::vtable(this).NeighboringFilesQuery)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = &::windows::core::ComInterface::cast::<IFileActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>();
            (::windows::core::Interface::vtable(this).Files)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IFileActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IFileActivatedEventArgsWithNeighboringFiles, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IFileActivatedEventArgsWithNeighboringFiles {}
impl windows::core::CanTryInto<IFileActivatedEventArgs> for IFileActivatedEventArgsWithNeighboringFiles {}
impl ::core::cmp::PartialEq for IFileActivatedEventArgsWithNeighboringFiles {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileActivatedEventArgsWithNeighboringFiles {}
impl ::core::fmt::Debug for IFileActivatedEventArgsWithNeighboringFiles {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileActivatedEventArgsWithNeighboringFiles").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IFileActivatedEventArgsWithNeighboringFiles {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{433ba1a4-e1e2-48fd-b7fc-b5d6eee65033}");
}
unsafe impl ::windows::core::Interface for IFileActivatedEventArgsWithNeighboringFiles {
    type Vtable = IFileActivatedEventArgsWithNeighboringFiles_Vtbl;
}
impl ::core::clone::Clone for IFileActivatedEventArgsWithNeighboringFiles {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFileActivatedEventArgsWithNeighboringFiles {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x433ba1a4_e1e2_48fd_b7fc_b5d6eee65033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgsWithNeighboringFiles_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Search")]
    pub NeighboringFilesQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    NeighboringFilesQuery: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IFileOpenPickerActivatedEventArgs(::windows::core::IUnknown);
impl IFileOpenPickerActivatedEventArgs {
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileOpenPickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Pickers::Provider::FileOpenPickerUI>();
            (::windows::core::Interface::vtable(this).FileOpenPickerUI)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IFileOpenPickerActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IFileOpenPickerActivatedEventArgs {}
impl ::core::cmp::PartialEq for IFileOpenPickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileOpenPickerActivatedEventArgs {}
impl ::core::fmt::Debug for IFileOpenPickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileOpenPickerActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IFileOpenPickerActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{72827082-5525-4bf2-bc09-1f5095d4964d}");
}
unsafe impl ::windows::core::Interface for IFileOpenPickerActivatedEventArgs {
    type Vtable = IFileOpenPickerActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IFileOpenPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFileOpenPickerActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72827082_5525_4bf2_bc09_1f5095d4964d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub FileOpenPickerUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Pickers_Provider"))]
    FileOpenPickerUI: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IFileOpenPickerActivatedEventArgs2(::windows::core::IUnknown);
impl IFileOpenPickerActivatedEventArgs2 {
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).CallerPackageFamilyName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IFileOpenPickerActivatedEventArgs2, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IFileOpenPickerActivatedEventArgs2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileOpenPickerActivatedEventArgs2 {}
impl ::core::fmt::Debug for IFileOpenPickerActivatedEventArgs2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileOpenPickerActivatedEventArgs2").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IFileOpenPickerActivatedEventArgs2 {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{5e731f66-8d1f-45fb-af1d-73205c8fc7a1}");
}
unsafe impl ::windows::core::Interface for IFileOpenPickerActivatedEventArgs2 {
    type Vtable = IFileOpenPickerActivatedEventArgs2_Vtbl;
}
impl ::core::clone::Clone for IFileOpenPickerActivatedEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFileOpenPickerActivatedEventArgs2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e731f66_8d1f_45fb_af1d_73205c8fc7a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerActivatedEventArgs2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IFileOpenPickerContinuationEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl IFileOpenPickerContinuationEventArgs {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>>();
            (::windows::core::Interface::vtable(this).Files)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::ComInterface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::ValueSet>();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(IFileOpenPickerContinuationEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl windows::core::CanTryInto<IActivatedEventArgs> for IFileOpenPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl windows::core::CanTryInto<IContinuationActivatedEventArgs> for IFileOpenPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for IFileOpenPickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for IFileOpenPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for IFileOpenPickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileOpenPickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for IFileOpenPickerContinuationEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{f0fa3f3a-d4e8-4ad3-9c34-2308f32fcec9}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IFileOpenPickerContinuationEventArgs {
    type Vtable = IFileOpenPickerContinuationEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IFileOpenPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for IFileOpenPickerContinuationEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0fa3f3a_d4e8_4ad3_9c34_2308f32fcec9);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerContinuationEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub Files: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated")))]
    Files: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IFileSavePickerActivatedEventArgs(::windows::core::IUnknown);
impl IFileSavePickerActivatedEventArgs {
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileSavePickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Pickers::Provider::FileSavePickerUI>();
            (::windows::core::Interface::vtable(this).FileSavePickerUI)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IFileSavePickerActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IFileSavePickerActivatedEventArgs {}
impl ::core::cmp::PartialEq for IFileSavePickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileSavePickerActivatedEventArgs {}
impl ::core::fmt::Debug for IFileSavePickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileSavePickerActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IFileSavePickerActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{81c19cf1-74e6-4387-82eb-bb8fd64b4346}");
}
unsafe impl ::windows::core::Interface for IFileSavePickerActivatedEventArgs {
    type Vtable = IFileSavePickerActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IFileSavePickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFileSavePickerActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81c19cf1_74e6_4387_82eb_bb8fd64b4346);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub FileSavePickerUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Pickers_Provider"))]
    FileSavePickerUI: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IFileSavePickerActivatedEventArgs2(::windows::core::IUnknown);
impl IFileSavePickerActivatedEventArgs2 {
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).CallerPackageFamilyName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).EnterpriseId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IFileSavePickerActivatedEventArgs2, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IFileSavePickerActivatedEventArgs2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileSavePickerActivatedEventArgs2 {}
impl ::core::fmt::Debug for IFileSavePickerActivatedEventArgs2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileSavePickerActivatedEventArgs2").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IFileSavePickerActivatedEventArgs2 {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{6b73fe13-2cf2-4d48-8cbc-af67d23f1ce7}");
}
unsafe impl ::windows::core::Interface for IFileSavePickerActivatedEventArgs2 {
    type Vtable = IFileSavePickerActivatedEventArgs2_Vtbl;
}
impl ::core::clone::Clone for IFileSavePickerActivatedEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFileSavePickerActivatedEventArgs2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b73fe13_2cf2_4d48_8cbc_af67d23f1ce7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerActivatedEventArgs2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IFileSavePickerContinuationEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl IFileSavePickerContinuationEventArgs {
    #[doc = "*Required features: `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::StorageFile>();
            (::windows::core::Interface::vtable(this).File)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::ComInterface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::ValueSet>();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(IFileSavePickerContinuationEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl windows::core::CanTryInto<IActivatedEventArgs> for IFileSavePickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl windows::core::CanTryInto<IContinuationActivatedEventArgs> for IFileSavePickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for IFileSavePickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for IFileSavePickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for IFileSavePickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileSavePickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for IFileSavePickerContinuationEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{2c846fe1-3bad-4f33-8c8b-e46fae824b4b}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IFileSavePickerContinuationEventArgs {
    type Vtable = IFileSavePickerContinuationEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IFileSavePickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for IFileSavePickerContinuationEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c846fe1_3bad_4f33_8c8b_e46fae824b4b);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerContinuationEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage", feature = "deprecated")))]
    File: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IFolderPickerContinuationEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl IFolderPickerContinuationEventArgs {
    #[doc = "*Required features: `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub fn Folder(&self) -> ::windows::core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::StorageFolder>();
            (::windows::core::Interface::vtable(this).Folder)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::ComInterface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::ValueSet>();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(IFolderPickerContinuationEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl windows::core::CanTryInto<IActivatedEventArgs> for IFolderPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl windows::core::CanTryInto<IContinuationActivatedEventArgs> for IFolderPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for IFolderPickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for IFolderPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for IFolderPickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFolderPickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for IFolderPickerContinuationEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{51882366-9f4b-498f-beb0-42684f6e1c29}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IFolderPickerContinuationEventArgs {
    type Vtable = IFolderPickerContinuationEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IFolderPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for IFolderPickerContinuationEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51882366_9f4b_498f_beb0_42684f6e1c29);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IFolderPickerContinuationEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub Folder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage", feature = "deprecated")))]
    Folder: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ILaunchActivatedEventArgs(::windows::core::IUnknown);
impl ILaunchActivatedEventArgs {
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Arguments)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).TileId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(ILaunchActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for ILaunchActivatedEventArgs {}
impl ::core::cmp::PartialEq for ILaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILaunchActivatedEventArgs {}
impl ::core::fmt::Debug for ILaunchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILaunchActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ILaunchActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{fbc93e26-a14a-4b4f-82b0-33bed920af52}");
}
unsafe impl ::windows::core::Interface for ILaunchActivatedEventArgs {
    type Vtable = ILaunchActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ILaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILaunchActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbc93e26_a14a_4b4f_82b0_33bed920af52);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ILaunchActivatedEventArgs2(::windows::core::IUnknown);
impl ILaunchActivatedEventArgs2 {
    pub fn TileActivatedInfo(&self) -> ::windows::core::Result<TileActivatedInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<TileActivatedInfo>();
            (::windows::core::Interface::vtable(this).TileActivatedInfo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Arguments)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).TileId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(ILaunchActivatedEventArgs2, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for ILaunchActivatedEventArgs2 {}
impl windows::core::CanTryInto<ILaunchActivatedEventArgs> for ILaunchActivatedEventArgs2 {}
impl ::core::cmp::PartialEq for ILaunchActivatedEventArgs2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILaunchActivatedEventArgs2 {}
impl ::core::fmt::Debug for ILaunchActivatedEventArgs2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILaunchActivatedEventArgs2").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ILaunchActivatedEventArgs2 {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{0fd37ebc-9dc9-46b5-9ace-bd95d4565345}");
}
unsafe impl ::windows::core::Interface for ILaunchActivatedEventArgs2 {
    type Vtable = ILaunchActivatedEventArgs2_Vtbl;
}
impl ::core::clone::Clone for ILaunchActivatedEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILaunchActivatedEventArgs2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fd37ebc_9dc9_46b5_9ace_bd95d4565345);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchActivatedEventArgs2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TileActivatedInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ILockScreenActivatedEventArgs(::windows::core::IUnknown);
impl ILockScreenActivatedEventArgs {
    pub fn Info(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Info)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(ILockScreenActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for ILockScreenActivatedEventArgs {}
impl ::core::cmp::PartialEq for ILockScreenActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILockScreenActivatedEventArgs {}
impl ::core::fmt::Debug for ILockScreenActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILockScreenActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ILockScreenActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{3ca77966-6108-4a41-8220-ee7d133c8532}");
}
unsafe impl ::windows::core::Interface for ILockScreenActivatedEventArgs {
    type Vtable = ILockScreenActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ILockScreenActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILockScreenActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ca77966_6108_4a41_8220_ee7d133c8532);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Info: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ILockScreenCallActivatedEventArgs(::windows::core::IUnknown);
impl ILockScreenCallActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    #[cfg(feature = "ApplicationModel_Calls")]
    pub fn CallUI(&self) -> ::windows::core::Result<super::Calls::LockScreenCallUI> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Calls::LockScreenCallUI>();
            (::windows::core::Interface::vtable(this).CallUI)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Arguments)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).TileId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(ILockScreenCallActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for ILockScreenCallActivatedEventArgs {}
impl windows::core::CanTryInto<ILaunchActivatedEventArgs> for ILockScreenCallActivatedEventArgs {}
impl ::core::cmp::PartialEq for ILockScreenCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILockScreenCallActivatedEventArgs {}
impl ::core::fmt::Debug for ILockScreenCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILockScreenCallActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ILockScreenCallActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{06f37fbe-b5f2-448b-b13e-e328ac1c516a}");
}
unsafe impl ::windows::core::Interface for ILockScreenCallActivatedEventArgs {
    type Vtable = ILockScreenCallActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ILockScreenCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILockScreenCallActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06f37fbe_b5f2_448b_b13e_e328ac1c516a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenCallActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Calls")]
    pub CallUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Calls"))]
    CallUI: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IPhoneCallActivatedEventArgs(::windows::core::IUnknown);
impl IPhoneCallActivatedEventArgs {
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).LineId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IPhoneCallActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IPhoneCallActivatedEventArgs {}
impl ::core::cmp::PartialEq for IPhoneCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhoneCallActivatedEventArgs {}
impl ::core::fmt::Debug for IPhoneCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhoneCallActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IPhoneCallActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{54615221-a3c1-4ced-b62f-8c60523619ad}");
}
unsafe impl ::windows::core::Interface for IPhoneCallActivatedEventArgs {
    type Vtable = IPhoneCallActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPhoneCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPhoneCallActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54615221_a3c1_4ced_b62f_8c60523619ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IPickerReturnedActivatedEventArgs(::windows::core::IUnknown);
impl IPickerReturnedActivatedEventArgs {
    pub fn PickerOperationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).PickerOperationId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IPickerReturnedActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IPickerReturnedActivatedEventArgs {}
impl ::core::cmp::PartialEq for IPickerReturnedActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPickerReturnedActivatedEventArgs {}
impl ::core::fmt::Debug for IPickerReturnedActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPickerReturnedActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IPickerReturnedActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{360defb9-a9d3-4984-a4ed-9ec734604921}");
}
unsafe impl ::windows::core::Interface for IPickerReturnedActivatedEventArgs {
    type Vtable = IPickerReturnedActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPickerReturnedActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPickerReturnedActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x360defb9_a9d3_4984_a4ed_9ec734604921);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerReturnedActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PickerOperationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IPrelaunchActivatedEventArgs(::windows::core::IUnknown);
impl IPrelaunchActivatedEventArgs {
    pub fn PrelaunchActivated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).PrelaunchActivated)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IPrelaunchActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IPrelaunchActivatedEventArgs {}
impl ::core::cmp::PartialEq for IPrelaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrelaunchActivatedEventArgs {}
impl ::core::fmt::Debug for IPrelaunchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrelaunchActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IPrelaunchActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{0c44717b-19f7-48d6-b046-cf22826eaa74}");
}
unsafe impl ::windows::core::Interface for IPrelaunchActivatedEventArgs {
    type Vtable = IPrelaunchActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPrelaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrelaunchActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c44717b_19f7_48d6_b046_cf22826eaa74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrelaunchActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PrelaunchActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IPrint3DWorkflowActivatedEventArgs(::windows::core::IUnknown);
impl IPrint3DWorkflowActivatedEventArgs {
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Workflow(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Devices::Printers::Extensions::Print3DWorkflow>();
            (::windows::core::Interface::vtable(this).Workflow)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IPrint3DWorkflowActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IPrint3DWorkflowActivatedEventArgs {}
impl ::core::cmp::PartialEq for IPrint3DWorkflowActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrint3DWorkflowActivatedEventArgs {}
impl ::core::fmt::Debug for IPrint3DWorkflowActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrint3DWorkflowActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IPrint3DWorkflowActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{3f57e78b-f2ac-4619-8302-ef855e1c9b90}");
}
unsafe impl ::windows::core::Interface for IPrint3DWorkflowActivatedEventArgs {
    type Vtable = IPrint3DWorkflowActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPrint3DWorkflowActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrint3DWorkflowActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f57e78b_f2ac_4619_8302_ef855e1c9b90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DWorkflowActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub Workflow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Printers_Extensions"))]
    Workflow: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IPrintTaskSettingsActivatedEventArgs(::windows::core::IUnknown);
impl IPrintTaskSettingsActivatedEventArgs {
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Configuration(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Devices::Printers::Extensions::PrintTaskConfiguration>();
            (::windows::core::Interface::vtable(this).Configuration)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IPrintTaskSettingsActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IPrintTaskSettingsActivatedEventArgs {}
impl ::core::cmp::PartialEq for IPrintTaskSettingsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintTaskSettingsActivatedEventArgs {}
impl ::core::fmt::Debug for IPrintTaskSettingsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintTaskSettingsActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IPrintTaskSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{ee30a0c9-ce56-4865-ba8e-8954ac271107}");
}
unsafe impl ::windows::core::Interface for IPrintTaskSettingsActivatedEventArgs {
    type Vtable = IPrintTaskSettingsActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPrintTaskSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintTaskSettingsActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee30a0c9_ce56_4865_ba8e_8954ac271107);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskSettingsActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Printers_Extensions"))]
    Configuration: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IProtocolActivatedEventArgs(::windows::core::IUnknown);
impl IProtocolActivatedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).Uri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IProtocolActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IProtocolActivatedEventArgs {}
impl ::core::cmp::PartialEq for IProtocolActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProtocolActivatedEventArgs {}
impl ::core::fmt::Debug for IProtocolActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProtocolActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IProtocolActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{6095f4dd-b7c0-46ab-81fe-d90f36d00d24}");
}
unsafe impl ::windows::core::Interface for IProtocolActivatedEventArgs {
    type Vtable = IProtocolActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IProtocolActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IProtocolActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6095f4dd_b7c0_46ab_81fe_d90f36d00d24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData(::windows::core::IUnknown);
impl IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).CallerPackageFamilyName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::ValueSet>();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {}
impl ::core::cmp::PartialEq for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {}
impl ::core::fmt::Debug for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{d84a0c12-5c8f-438c-83cb-c28fcc0b2fdb}");
}
unsafe impl ::windows::core::Interface for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Vtable = IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Vtbl;
}
impl ::core::clone::Clone for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd84a0c12_5c8f_438c_83cb_c28fcc0b2fdb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Data: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IProtocolForResultsActivatedEventArgs(::windows::core::IUnknown);
impl IProtocolForResultsActivatedEventArgs {
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ProtocolForResultsOperation(&self) -> ::windows::core::Result<super::super::System::ProtocolForResultsOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::ProtocolForResultsOperation>();
            (::windows::core::Interface::vtable(this).ProtocolForResultsOperation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IProtocolForResultsActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IProtocolForResultsActivatedEventArgs {}
impl ::core::cmp::PartialEq for IProtocolForResultsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProtocolForResultsActivatedEventArgs {}
impl ::core::fmt::Debug for IProtocolForResultsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProtocolForResultsActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IProtocolForResultsActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{e75132c2-7ae7-4517-80ac-dbe8d7cc5b9c}");
}
unsafe impl ::windows::core::Interface for IProtocolForResultsActivatedEventArgs {
    type Vtable = IProtocolForResultsActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IProtocolForResultsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IProtocolForResultsActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe75132c2_7ae7_4517_80ac_dbe8d7cc5b9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolForResultsActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub ProtocolForResultsOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    ProtocolForResultsOperation: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IRestrictedLaunchActivatedEventArgs(::windows::core::IUnknown);
impl IRestrictedLaunchActivatedEventArgs {
    pub fn SharedContext(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).SharedContext)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IRestrictedLaunchActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IRestrictedLaunchActivatedEventArgs {}
impl ::core::cmp::PartialEq for IRestrictedLaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRestrictedLaunchActivatedEventArgs {}
impl ::core::fmt::Debug for IRestrictedLaunchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRestrictedLaunchActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IRestrictedLaunchActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{e0b7ac81-bfc3-4344-a5da-19fd5a27baae}");
}
unsafe impl ::windows::core::Interface for IRestrictedLaunchActivatedEventArgs {
    type Vtable = IRestrictedLaunchActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IRestrictedLaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRestrictedLaunchActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0b7ac81_bfc3_4344_a5da_19fd5a27baae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRestrictedLaunchActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SharedContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ISearchActivatedEventArgs(::windows::core::IUnknown);
impl ISearchActivatedEventArgs {
    pub fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).QueryText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Language)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(ISearchActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for ISearchActivatedEventArgs {}
impl ::core::cmp::PartialEq for ISearchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchActivatedEventArgs {}
impl ::core::fmt::Debug for ISearchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ISearchActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{8cb36951-58c8-43e3-94bc-41d33f8b630e}");
}
unsafe impl ::windows::core::Interface for ISearchActivatedEventArgs {
    type Vtable = ISearchActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ISearchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISearchActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cb36951_58c8_43e3_94bc_41d33f8b630e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub QueryText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ISearchActivatedEventArgsWithLinguisticDetails(::windows::core::IUnknown);
impl ISearchActivatedEventArgsWithLinguisticDetails {
    #[doc = "*Required features: `\"ApplicationModel_Search\"`*"]
    #[cfg(feature = "ApplicationModel_Search")]
    pub fn LinguisticDetails(&self) -> ::windows::core::Result<super::Search::SearchPaneQueryLinguisticDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Search::SearchPaneQueryLinguisticDetails>();
            (::windows::core::Interface::vtable(this).LinguisticDetails)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(ISearchActivatedEventArgsWithLinguisticDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for ISearchActivatedEventArgsWithLinguisticDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchActivatedEventArgsWithLinguisticDetails {}
impl ::core::fmt::Debug for ISearchActivatedEventArgsWithLinguisticDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchActivatedEventArgsWithLinguisticDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ISearchActivatedEventArgsWithLinguisticDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{c09f33da-08ab-4931-9b7c-451025f21f81}");
}
unsafe impl ::windows::core::Interface for ISearchActivatedEventArgsWithLinguisticDetails {
    type Vtable = ISearchActivatedEventArgsWithLinguisticDetails_Vtbl;
}
impl ::core::clone::Clone for ISearchActivatedEventArgsWithLinguisticDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISearchActivatedEventArgsWithLinguisticDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc09f33da_08ab_4931_9b7c_451025f21f81);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchActivatedEventArgsWithLinguisticDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Search")]
    pub LinguisticDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Search"))]
    LinguisticDetails: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IShareTargetActivatedEventArgs(::windows::core::IUnknown);
impl IShareTargetActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_ShareTarget\"`*"]
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    pub fn ShareOperation(&self) -> ::windows::core::Result<super::DataTransfer::ShareTarget::ShareOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::DataTransfer::ShareTarget::ShareOperation>();
            (::windows::core::Interface::vtable(this).ShareOperation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IShareTargetActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IShareTargetActivatedEventArgs {}
impl ::core::cmp::PartialEq for IShareTargetActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IShareTargetActivatedEventArgs {}
impl ::core::fmt::Debug for IShareTargetActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IShareTargetActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IShareTargetActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{4bdaf9c8-cdb2-4acb-bfc3-6648563378ec}");
}
unsafe impl ::windows::core::Interface for IShareTargetActivatedEventArgs {
    type Vtable = IShareTargetActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IShareTargetActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IShareTargetActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bdaf9c8_cdb2_4acb_bfc3_6648563378ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareTargetActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    pub ShareOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer_ShareTarget"))]
    ShareOperation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISplashScreen(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISplashScreen {
    type Vtable = ISplashScreen_Vtbl;
}
impl ::core::clone::Clone for ISplashScreen {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISplashScreen {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca4d975c_d4d6_43f0_97c0_0833c6391c24);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplashScreen_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ImageLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImageLocation: usize,
    #[cfg(feature = "Foundation")]
    pub Dismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Dismissed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDismissed: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IStartupTaskActivatedEventArgs(::windows::core::IUnknown);
impl IStartupTaskActivatedEventArgs {
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).TaskId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IStartupTaskActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IStartupTaskActivatedEventArgs {}
impl ::core::cmp::PartialEq for IStartupTaskActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStartupTaskActivatedEventArgs {}
impl ::core::fmt::Debug for IStartupTaskActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStartupTaskActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IStartupTaskActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{03b11a58-5276-4d91-8621-54611864d5fa}");
}
unsafe impl ::windows::core::Interface for IStartupTaskActivatedEventArgs {
    type Vtable = IStartupTaskActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IStartupTaskActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStartupTaskActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03b11a58_5276_4d91_8621_54611864d5fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartupTaskActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TaskId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITileActivatedInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITileActivatedInfo {
    type Vtable = ITileActivatedInfo_Vtbl;
}
impl ::core::clone::Clone for ITileActivatedInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITileActivatedInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80e4a3b1_3980_4f17_b738_89194e0b8f65);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileActivatedInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Notifications"))]
    pub RecentlyShownNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Notifications")))]
    RecentlyShownNotifications: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IToastNotificationActivatedEventArgs(::windows::core::IUnknown);
impl IToastNotificationActivatedEventArgs {
    pub fn Argument(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Argument)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserInput(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::ValueSet>();
            (::windows::core::Interface::vtable(this).UserInput)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IToastNotificationActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IToastNotificationActivatedEventArgs {}
impl ::core::cmp::PartialEq for IToastNotificationActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IToastNotificationActivatedEventArgs {}
impl ::core::fmt::Debug for IToastNotificationActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IToastNotificationActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IToastNotificationActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{92a86f82-5290-431d-be85-c4aaeeb8685f}");
}
unsafe impl ::windows::core::Interface for IToastNotificationActivatedEventArgs {
    type Vtable = IToastNotificationActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IToastNotificationActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IToastNotificationActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92a86f82_5290_431d_be85_c4aaeeb8685f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Argument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub UserInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UserInput: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IUserDataAccountProviderActivatedEventArgs(::windows::core::IUnknown);
impl IUserDataAccountProviderActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    pub fn Operation(&self) -> ::windows::core::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation>();
            (::windows::core::Interface::vtable(this).Operation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IUserDataAccountProviderActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IUserDataAccountProviderActivatedEventArgs {}
impl ::core::cmp::PartialEq for IUserDataAccountProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUserDataAccountProviderActivatedEventArgs {}
impl ::core::fmt::Debug for IUserDataAccountProviderActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUserDataAccountProviderActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IUserDataAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{1bc9f723-8ef1-4a51-a63a-fe711eeab607}");
}
unsafe impl ::windows::core::Interface for IUserDataAccountProviderActivatedEventArgs {
    type Vtable = IUserDataAccountProviderActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IUserDataAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserDataAccountProviderActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bc9f723_8ef1_4a51_a63a_fe711eeab607);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_UserDataAccounts_Provider"))]
    Operation: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IViewSwitcherProvider(::windows::core::IUnknown);
impl IViewSwitcherProvider {
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::UI::ViewManagement::ActivationViewSwitcher>();
            (::windows::core::Interface::vtable(this).ViewSwitcher)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IViewSwitcherProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IViewSwitcherProvider {}
impl ::core::cmp::PartialEq for IViewSwitcherProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IViewSwitcherProvider {}
impl ::core::fmt::Debug for IViewSwitcherProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IViewSwitcherProvider").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IViewSwitcherProvider {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{33f288a6-5c2c-4d27-bac7-7536088f1219}");
}
unsafe impl ::windows::core::Interface for IViewSwitcherProvider {
    type Vtable = IViewSwitcherProvider_Vtbl;
}
impl ::core::clone::Clone for IViewSwitcherProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IViewSwitcherProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33f288a6_5c2c_4d27_bac7_7536088f1219);
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewSwitcherProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI_ViewManagement")]
    pub ViewSwitcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    ViewSwitcher: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IVoiceCommandActivatedEventArgs(::windows::core::IUnknown);
impl IVoiceCommandActivatedEventArgs {
    #[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
    #[cfg(feature = "Media_SpeechRecognition")]
    pub fn Result(&self) -> ::windows::core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Media::SpeechRecognition::SpeechRecognitionResult>();
            (::windows::core::Interface::vtable(this).Result)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IVoiceCommandActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IVoiceCommandActivatedEventArgs {}
impl ::core::cmp::PartialEq for IVoiceCommandActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVoiceCommandActivatedEventArgs {}
impl ::core::fmt::Debug for IVoiceCommandActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVoiceCommandActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IVoiceCommandActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{ab92dcfd-8d43-4de6-9775-20704b581b00}");
}
unsafe impl ::windows::core::Interface for IVoiceCommandActivatedEventArgs {
    type Vtable = IVoiceCommandActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IVoiceCommandActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IVoiceCommandActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab92dcfd_8d43_4de6_9775_20704b581b00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Media_SpeechRecognition")]
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_SpeechRecognition"))]
    Result: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IWalletActionActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl IWalletActionActivatedEventArgs {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ItemId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ItemId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Wallet\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "ApplicationModel_Wallet", feature = "deprecated"))]
    pub fn ActionKind(&self) -> ::windows::core::Result<super::Wallet::WalletActionKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Wallet::WalletActionKind>();
            (::windows::core::Interface::vtable(this).ActionKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ActionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ActionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(IWalletActionActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl windows::core::CanTryInto<IActivatedEventArgs> for IWalletActionActivatedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for IWalletActionActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for IWalletActionActivatedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for IWalletActionActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWalletActionActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for IWalletActionActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{fcfc027b-1a1a-4d22-923f-ae6f45fa52d9}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IWalletActionActivatedEventArgs {
    type Vtable = IWalletActionActivatedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IWalletActionActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for IWalletActionActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcfc027b_1a1a_4d22_923f_ae6f45fa52d9);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IWalletActionActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ItemId: usize,
    #[cfg(all(feature = "ApplicationModel_Wallet", feature = "deprecated"))]
    pub ActionKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Wallet::WalletActionKind) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Wallet", feature = "deprecated")))]
    ActionKind: usize,
    #[cfg(feature = "deprecated")]
    pub ActionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ActionId: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IWebAccountProviderActivatedEventArgs(::windows::core::IUnknown);
impl IWebAccountProviderActivatedEventArgs {
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    pub fn Operation(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation>();
            (::windows::core::Interface::vtable(this).Operation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IWebAccountProviderActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IWebAccountProviderActivatedEventArgs {}
impl ::core::cmp::PartialEq for IWebAccountProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAccountProviderActivatedEventArgs {}
impl ::core::fmt::Debug for IWebAccountProviderActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebAccountProviderActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IWebAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{72b71774-98ea-4ccf-9752-46d9051004f1}");
}
unsafe impl ::windows::core::Interface for IWebAccountProviderActivatedEventArgs {
    type Vtable = IWebAccountProviderActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IWebAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebAccountProviderActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72b71774_98ea_4ccf_9752_46d9051004f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Provider"))]
    Operation: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct IWebAuthenticationBrokerContinuationEventArgs(::windows::core::IUnknown);
impl IWebAuthenticationBrokerContinuationEventArgs {
    #[doc = "*Required features: `\"Security_Authentication_Web\"`*"]
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn WebAuthenticationResult(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Authentication::Web::WebAuthenticationResult>();
            (::windows::core::Interface::vtable(this).WebAuthenticationResult)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::ComInterface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::ValueSet>();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IWebAuthenticationBrokerContinuationEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IActivatedEventArgs> for IWebAuthenticationBrokerContinuationEventArgs {}
impl windows::core::CanTryInto<IContinuationActivatedEventArgs> for IWebAuthenticationBrokerContinuationEventArgs {}
impl ::core::cmp::PartialEq for IWebAuthenticationBrokerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebAuthenticationBrokerContinuationEventArgs {}
impl ::core::fmt::Debug for IWebAuthenticationBrokerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebAuthenticationBrokerContinuationEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IWebAuthenticationBrokerContinuationEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{75dda3d4-7714-453d-b7ff-b95e3a1709da}");
}
unsafe impl ::windows::core::Interface for IWebAuthenticationBrokerContinuationEventArgs {
    type Vtable = IWebAuthenticationBrokerContinuationEventArgs_Vtbl;
}
impl ::core::clone::Clone for IWebAuthenticationBrokerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebAuthenticationBrokerContinuationEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75dda3d4_7714_453d_b7ff_b95e3a1709da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationBrokerContinuationEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Authentication_Web")]
    pub WebAuthenticationResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))]
    WebAuthenticationResult: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct AppointmentsProviderAddAppointmentActivatedEventArgs(::windows::core::IUnknown);
impl AppointmentsProviderAddAppointmentActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn AddAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Appointments::AppointmentsProvider::AddAppointmentOperation>();
            (::windows::core::Interface::vtable(this).AddAppointmentOperation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentsProviderAddAppointmentActivatedEventArgs {}
impl ::core::fmt::Debug for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentsProviderAddAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppointmentsProviderAddAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderAddAppointmentActivatedEventArgs;{a2861367-cee5-4e4d-9ed7-41c34ec18b02})");
}
impl ::core::clone::Clone for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppointmentsProviderAddAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderAddAppointmentActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppointmentsProviderAddAppointmentActivatedEventArgs {
    const IID: ::windows::core::GUID = <IAppointmentsProviderAddAppointmentActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentsProviderAddAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderAddAppointmentActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(AppointmentsProviderAddAppointmentActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for AppointmentsProviderAddAppointmentActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for AppointmentsProviderAddAppointmentActivatedEventArgs {}
impl ::windows::core::CanTryInto<IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderAddAppointmentActivatedEventArgs {}
impl ::windows::core::CanTryInto<IAppointmentsProviderAddAppointmentActivatedEventArgs> for AppointmentsProviderAddAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Send for AppointmentsProviderAddAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderAddAppointmentActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct AppointmentsProviderRemoveAppointmentActivatedEventArgs(::windows::core::IUnknown);
impl AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn RemoveAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation>();
            (::windows::core::Interface::vtable(this).RemoveAppointmentOperation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
impl ::core::fmt::Debug for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentsProviderRemoveAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderRemoveAppointmentActivatedEventArgs;{751f3ab8-0b8e-451c-9f15-966e699bac25})");
}
impl ::core::clone::Clone for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const IID: ::windows::core::GUID = <IAppointmentsProviderRemoveAppointmentActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderRemoveAppointmentActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(AppointmentsProviderRemoveAppointmentActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
impl ::windows::core::CanTryInto<IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
impl ::windows::core::CanTryInto<IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Send for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct AppointmentsProviderReplaceAppointmentActivatedEventArgs(::windows::core::IUnknown);
impl AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn ReplaceAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation>();
            (::windows::core::Interface::vtable(this).ReplaceAppointmentOperation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
impl ::core::fmt::Debug for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentsProviderReplaceAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderReplaceAppointmentActivatedEventArgs;{1551b7d4-a981-4067-8a62-0524e4ade121})");
}
impl ::core::clone::Clone for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const IID: ::windows::core::GUID = <IAppointmentsProviderReplaceAppointmentActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderReplaceAppointmentActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(AppointmentsProviderReplaceAppointmentActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
impl ::windows::core::CanTryInto<IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
impl ::windows::core::CanTryInto<IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Send for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct AppointmentsProviderShowAppointmentDetailsActivatedEventArgs(::windows::core::IUnknown);
impl AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InstanceStartDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>();
            (::windows::core::Interface::vtable(this).InstanceStartDate)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).LocalId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RoamingId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).RoamingId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
impl ::core::fmt::Debug for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentsProviderShowAppointmentDetailsActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderShowAppointmentDetailsActivatedEventArgs;{3958f065-9841-4ca5-999b-885198b9ef2a})");
}
impl ::core::clone::Clone for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const IID: ::windows::core::GUID = <IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(AppointmentsProviderShowAppointmentDetailsActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
impl ::windows::core::CanTryInto<IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
impl ::windows::core::CanTryInto<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
unsafe impl ::core::marker::Send for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct AppointmentsProviderShowTimeFrameActivatedEventArgs(::windows::core::IUnknown);
impl AppointmentsProviderShowTimeFrameActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeToShow(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).TimeToShow)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).Duration)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
impl ::core::fmt::Debug for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentsProviderShowTimeFrameActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderShowTimeFrameActivatedEventArgs;{9baeaba6-0e0b-49aa-babc-12b1dc774986})");
}
impl ::core::clone::Clone for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowTimeFrameActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    const IID: ::windows::core::GUID = <IAppointmentsProviderShowTimeFrameActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderShowTimeFrameActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(AppointmentsProviderShowTimeFrameActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
impl ::windows::core::CanTryInto<IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
impl ::windows::core::CanTryInto<IAppointmentsProviderShowTimeFrameActivatedEventArgs> for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
unsafe impl ::core::marker::Send for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct BackgroundActivatedEventArgs(::windows::core::IUnknown);
impl BackgroundActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn TaskInstance(&self) -> ::windows::core::Result<super::Background::IBackgroundTaskInstance> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Background::IBackgroundTaskInstance>();
            (::windows::core::Interface::vtable(this).TaskInstance)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for BackgroundActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundActivatedEventArgs {}
impl ::core::fmt::Debug for BackgroundActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for BackgroundActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.BackgroundActivatedEventArgs;{ab14bee0-e760-440e-a91c-44796de3a92d})");
}
impl ::core::clone::Clone for BackgroundActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for BackgroundActivatedEventArgs {
    type Vtable = IBackgroundActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for BackgroundActivatedEventArgs {
    const IID: ::windows::core::GUID = <IBackgroundActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for BackgroundActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.BackgroundActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(BackgroundActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundActivatedEventArgs> for BackgroundActivatedEventArgs {}
unsafe impl ::core::marker::Send for BackgroundActivatedEventArgs {}
unsafe impl ::core::marker::Sync for BackgroundActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct BarcodeScannerPreviewActivatedEventArgs(::windows::core::IUnknown);
impl BarcodeScannerPreviewActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConnectionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ConnectionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerPreviewActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerPreviewActivatedEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerPreviewActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerPreviewActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for BarcodeScannerPreviewActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.BarcodeScannerPreviewActivatedEventArgs;{6772797c-99bf-4349-af22-e4123560371c})");
}
impl ::core::clone::Clone for BarcodeScannerPreviewActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for BarcodeScannerPreviewActivatedEventArgs {
    type Vtable = IBarcodeScannerPreviewActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for BarcodeScannerPreviewActivatedEventArgs {
    const IID: ::windows::core::GUID = <IBarcodeScannerPreviewActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for BarcodeScannerPreviewActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.BarcodeScannerPreviewActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(BarcodeScannerPreviewActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for BarcodeScannerPreviewActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for BarcodeScannerPreviewActivatedEventArgs {}
impl ::windows::core::CanTryInto<IBarcodeScannerPreviewActivatedEventArgs> for BarcodeScannerPreviewActivatedEventArgs {}
unsafe impl ::core::marker::Send for BarcodeScannerPreviewActivatedEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerPreviewActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct CachedFileUpdaterActivatedEventArgs(::windows::core::IUnknown);
impl CachedFileUpdaterActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    #[cfg(feature = "Storage_Provider")]
    pub fn CachedFileUpdaterUI(&self) -> ::windows::core::Result<super::super::Storage::Provider::CachedFileUpdaterUI> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Provider::CachedFileUpdaterUI>();
            (::windows::core::Interface::vtable(this).CachedFileUpdaterUI)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CachedFileUpdaterActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CachedFileUpdaterActivatedEventArgs {}
impl ::core::fmt::Debug for CachedFileUpdaterActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CachedFileUpdaterActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CachedFileUpdaterActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CachedFileUpdaterActivatedEventArgs;{d06eb1c7-3805-4ecb-b757-6cf15e26fef3})");
}
impl ::core::clone::Clone for CachedFileUpdaterActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CachedFileUpdaterActivatedEventArgs {
    type Vtable = ICachedFileUpdaterActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CachedFileUpdaterActivatedEventArgs {
    const IID: ::windows::core::GUID = <ICachedFileUpdaterActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CachedFileUpdaterActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CachedFileUpdaterActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(CachedFileUpdaterActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for CachedFileUpdaterActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for CachedFileUpdaterActivatedEventArgs {}
impl ::windows::core::CanTryInto<ICachedFileUpdaterActivatedEventArgs> for CachedFileUpdaterActivatedEventArgs {}
unsafe impl ::core::marker::Send for CachedFileUpdaterActivatedEventArgs {}
unsafe impl ::core::marker::Sync for CachedFileUpdaterActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct CameraSettingsActivatedEventArgs(::windows::core::IUnknown);
impl CameraSettingsActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VideoDeviceController(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).VideoDeviceController)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VideoDeviceExtension(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).VideoDeviceExtension)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CameraSettingsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CameraSettingsActivatedEventArgs {}
impl ::core::fmt::Debug for CameraSettingsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraSettingsActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CameraSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CameraSettingsActivatedEventArgs;{fb67a508-2dad-490a-9170-dca036eb114b})");
}
impl ::core::clone::Clone for CameraSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CameraSettingsActivatedEventArgs {
    type Vtable = ICameraSettingsActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CameraSettingsActivatedEventArgs {
    const IID: ::windows::core::GUID = <ICameraSettingsActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CameraSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CameraSettingsActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(CameraSettingsActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for CameraSettingsActivatedEventArgs {}
impl ::windows::core::CanTryInto<ICameraSettingsActivatedEventArgs> for CameraSettingsActivatedEventArgs {}
unsafe impl ::core::marker::Send for CameraSettingsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for CameraSettingsActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct CommandLineActivatedEventArgs(::windows::core::IUnknown);
impl CommandLineActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Operation(&self) -> ::windows::core::Result<CommandLineActivationOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CommandLineActivationOperation>();
            (::windows::core::Interface::vtable(this).Operation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CommandLineActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CommandLineActivatedEventArgs {}
impl ::core::fmt::Debug for CommandLineActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CommandLineActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CommandLineActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CommandLineActivatedEventArgs;{4506472c-006a-48eb-8afb-d07ab25e3366})");
}
impl ::core::clone::Clone for CommandLineActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CommandLineActivatedEventArgs {
    type Vtable = ICommandLineActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CommandLineActivatedEventArgs {
    const IID: ::windows::core::GUID = <ICommandLineActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CommandLineActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CommandLineActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(CommandLineActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for CommandLineActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for CommandLineActivatedEventArgs {}
impl ::windows::core::CanTryInto<ICommandLineActivatedEventArgs> for CommandLineActivatedEventArgs {}
unsafe impl ::core::marker::Send for CommandLineActivatedEventArgs {}
unsafe impl ::core::marker::Sync for CommandLineActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct CommandLineActivationOperation(::windows::core::IUnknown);
impl CommandLineActivationOperation {
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Arguments)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentDirectoryPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).CurrentDirectoryPath)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetExitCode(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetExitCode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExitCode(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).ExitCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Deferral>();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CommandLineActivationOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CommandLineActivationOperation {}
impl ::core::fmt::Debug for CommandLineActivationOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CommandLineActivationOperation").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CommandLineActivationOperation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CommandLineActivationOperation;{994b2841-c59e-4f69-bcfd-b61ed4e622eb})");
}
impl ::core::clone::Clone for CommandLineActivationOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CommandLineActivationOperation {
    type Vtable = ICommandLineActivationOperation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CommandLineActivationOperation {
    const IID: ::windows::core::GUID = <ICommandLineActivationOperation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CommandLineActivationOperation {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CommandLineActivationOperation";
}
::windows::imp::interface_hierarchy!(CommandLineActivationOperation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CommandLineActivationOperation {}
unsafe impl ::core::marker::Sync for CommandLineActivationOperation {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ContactCallActivatedEventArgs(::windows::core::IUnknown);
impl ContactCallActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ServiceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ServiceUserId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Contacts::Contact>();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ContactCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactCallActivatedEventArgs {}
impl ::core::fmt::Debug for ContactCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactCallActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ContactCallActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactCallActivatedEventArgs;{c2df14c7-30eb-41c6-b3bc-5b1694f9dab3})");
}
impl ::core::clone::Clone for ContactCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ContactCallActivatedEventArgs {
    type Vtable = IContactCallActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ContactCallActivatedEventArgs {
    const IID: ::windows::core::GUID = <IContactCallActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ContactCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactCallActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(ContactCallActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for ContactCallActivatedEventArgs {}
impl ::windows::core::CanTryInto<IContactActivatedEventArgs> for ContactCallActivatedEventArgs {}
impl ::windows::core::CanTryInto<IContactCallActivatedEventArgs> for ContactCallActivatedEventArgs {}
unsafe impl ::core::marker::Send for ContactCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactCallActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ContactMapActivatedEventArgs(::windows::core::IUnknown);
impl ContactMapActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Address(&self) -> ::windows::core::Result<super::Contacts::ContactAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Contacts::ContactAddress>();
            (::windows::core::Interface::vtable(this).Address)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Contacts::Contact>();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ContactMapActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactMapActivatedEventArgs {}
impl ::core::fmt::Debug for ContactMapActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactMapActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ContactMapActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactMapActivatedEventArgs;{b32bf870-eee7-4ad2-aaf1-a87effcf00a4})");
}
impl ::core::clone::Clone for ContactMapActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ContactMapActivatedEventArgs {
    type Vtable = IContactMapActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ContactMapActivatedEventArgs {
    const IID: ::windows::core::GUID = <IContactMapActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ContactMapActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactMapActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(ContactMapActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for ContactMapActivatedEventArgs {}
impl ::windows::core::CanTryInto<IContactActivatedEventArgs> for ContactMapActivatedEventArgs {}
impl ::windows::core::CanTryInto<IContactMapActivatedEventArgs> for ContactMapActivatedEventArgs {}
unsafe impl ::core::marker::Send for ContactMapActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactMapActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ContactMessageActivatedEventArgs(::windows::core::IUnknown);
impl ContactMessageActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ServiceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ServiceUserId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Contacts::Contact>();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ContactMessageActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactMessageActivatedEventArgs {}
impl ::core::fmt::Debug for ContactMessageActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactMessageActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ContactMessageActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactMessageActivatedEventArgs;{de598db2-0e03-43b0-bf56-bcc40b3162df})");
}
impl ::core::clone::Clone for ContactMessageActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ContactMessageActivatedEventArgs {
    type Vtable = IContactMessageActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ContactMessageActivatedEventArgs {
    const IID: ::windows::core::GUID = <IContactMessageActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ContactMessageActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactMessageActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(ContactMessageActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for ContactMessageActivatedEventArgs {}
impl ::windows::core::CanTryInto<IContactActivatedEventArgs> for ContactMessageActivatedEventArgs {}
impl ::windows::core::CanTryInto<IContactMessageActivatedEventArgs> for ContactMessageActivatedEventArgs {}
unsafe impl ::core::marker::Send for ContactMessageActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactMessageActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ContactPanelActivatedEventArgs(::windows::core::IUnknown);
impl ContactPanelActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn ContactPanel(&self) -> ::windows::core::Result<super::Contacts::ContactPanel> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Contacts::ContactPanel>();
            (::windows::core::Interface::vtable(this).ContactPanel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Contacts::Contact>();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ContactPanelActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactPanelActivatedEventArgs {}
impl ::core::fmt::Debug for ContactPanelActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactPanelActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ContactPanelActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactPanelActivatedEventArgs;{52bb63e4-d3d4-4b63-8051-4af2082cab80})");
}
impl ::core::clone::Clone for ContactPanelActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ContactPanelActivatedEventArgs {
    type Vtable = IContactPanelActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ContactPanelActivatedEventArgs {
    const IID: ::windows::core::GUID = <IContactPanelActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ContactPanelActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPanelActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(ContactPanelActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for ContactPanelActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for ContactPanelActivatedEventArgs {}
impl ::windows::core::CanTryInto<IContactPanelActivatedEventArgs> for ContactPanelActivatedEventArgs {}
unsafe impl ::core::marker::Send for ContactPanelActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactPanelActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ContactPickerActivatedEventArgs(::windows::core::IUnknown);
impl ContactPickerActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts_Provider\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    pub fn ContactPickerUI(&self) -> ::windows::core::Result<super::Contacts::Provider::ContactPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Contacts::Provider::ContactPickerUI>();
            (::windows::core::Interface::vtable(this).ContactPickerUI)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ContactPickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactPickerActivatedEventArgs {}
impl ::core::fmt::Debug for ContactPickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactPickerActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ContactPickerActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactPickerActivatedEventArgs;{ce57aae7-6449-45a7-971f-d113be7a8936})");
}
impl ::core::clone::Clone for ContactPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ContactPickerActivatedEventArgs {
    type Vtable = IContactPickerActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ContactPickerActivatedEventArgs {
    const IID: ::windows::core::GUID = <IContactPickerActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ContactPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPickerActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(ContactPickerActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for ContactPickerActivatedEventArgs {}
impl ::windows::core::CanTryInto<IContactPickerActivatedEventArgs> for ContactPickerActivatedEventArgs {}
unsafe impl ::core::marker::Send for ContactPickerActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactPickerActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ContactPostActivatedEventArgs(::windows::core::IUnknown);
impl ContactPostActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ServiceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ServiceUserId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Contacts::Contact>();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ContactPostActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactPostActivatedEventArgs {}
impl ::core::fmt::Debug for ContactPostActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactPostActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ContactPostActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactPostActivatedEventArgs;{b35a3c67-f1e7-4655-ad6e-4857588f552f})");
}
impl ::core::clone::Clone for ContactPostActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ContactPostActivatedEventArgs {
    type Vtable = IContactPostActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ContactPostActivatedEventArgs {
    const IID: ::windows::core::GUID = <IContactPostActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ContactPostActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPostActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(ContactPostActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for ContactPostActivatedEventArgs {}
impl ::windows::core::CanTryInto<IContactActivatedEventArgs> for ContactPostActivatedEventArgs {}
impl ::windows::core::CanTryInto<IContactPostActivatedEventArgs> for ContactPostActivatedEventArgs {}
unsafe impl ::core::marker::Send for ContactPostActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactPostActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ContactVideoCallActivatedEventArgs(::windows::core::IUnknown);
impl ContactVideoCallActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ServiceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ServiceUserId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Contacts\"`*"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Contacts::Contact>();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ContactVideoCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactVideoCallActivatedEventArgs {}
impl ::core::fmt::Debug for ContactVideoCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactVideoCallActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ContactVideoCallActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactVideoCallActivatedEventArgs;{61079db8-e3e7-4b4f-858d-5c63a96ef684})");
}
impl ::core::clone::Clone for ContactVideoCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ContactVideoCallActivatedEventArgs {
    type Vtable = IContactVideoCallActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ContactVideoCallActivatedEventArgs {
    const IID: ::windows::core::GUID = <IContactVideoCallActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ContactVideoCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactVideoCallActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(ContactVideoCallActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for ContactVideoCallActivatedEventArgs {}
impl ::windows::core::CanTryInto<IContactActivatedEventArgs> for ContactVideoCallActivatedEventArgs {}
impl ::windows::core::CanTryInto<IContactVideoCallActivatedEventArgs> for ContactVideoCallActivatedEventArgs {}
unsafe impl ::core::marker::Send for ContactVideoCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactVideoCallActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct DeviceActivatedEventArgs(::windows::core::IUnknown);
impl DeviceActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::ComInterface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceInformationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceInformationId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::ComInterface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::UI::ViewManagement::ActivationViewSwitcher>();
            (::windows::core::Interface::vtable(this).ViewSwitcher)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DeviceActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceActivatedEventArgs {}
impl ::core::fmt::Debug for DeviceActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DeviceActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.DeviceActivatedEventArgs;{cd50b9a9-ce10-44d2-8234-c355a073ef33})");
}
impl ::core::clone::Clone for DeviceActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DeviceActivatedEventArgs {
    type Vtable = IDeviceActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DeviceActivatedEventArgs {
    const IID: ::windows::core::GUID = <IDeviceActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DeviceActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DeviceActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(DeviceActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for DeviceActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for DeviceActivatedEventArgs {}
impl ::windows::core::CanTryInto<IApplicationViewActivatedEventArgs> for DeviceActivatedEventArgs {}
impl ::windows::core::CanTryInto<IDeviceActivatedEventArgs> for DeviceActivatedEventArgs {}
impl ::windows::core::CanTryInto<IViewSwitcherProvider> for DeviceActivatedEventArgs {}
unsafe impl ::core::marker::Send for DeviceActivatedEventArgs {}
unsafe impl ::core::marker::Sync for DeviceActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct DevicePairingActivatedEventArgs(::windows::core::IUnknown);
impl DevicePairingActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Enumeration\"`*"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Devices::Enumeration::DeviceInformation>();
            (::windows::core::Interface::vtable(this).DeviceInformation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DevicePairingActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePairingActivatedEventArgs {}
impl ::core::fmt::Debug for DevicePairingActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePairingActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DevicePairingActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.DevicePairingActivatedEventArgs;{eba0d1e4-ecc6-4148-94ed-f4b37ec05b3e})");
}
impl ::core::clone::Clone for DevicePairingActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DevicePairingActivatedEventArgs {
    type Vtable = IDevicePairingActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DevicePairingActivatedEventArgs {
    const IID: ::windows::core::GUID = <IDevicePairingActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DevicePairingActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DevicePairingActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(DevicePairingActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for DevicePairingActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for DevicePairingActivatedEventArgs {}
impl ::windows::core::CanTryInto<IDevicePairingActivatedEventArgs> for DevicePairingActivatedEventArgs {}
unsafe impl ::core::marker::Send for DevicePairingActivatedEventArgs {}
unsafe impl ::core::marker::Sync for DevicePairingActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct DialReceiverActivatedEventArgs(::windows::core::IUnknown);
impl DialReceiverActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::ComInterface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AppName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Arguments)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).TileId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::ComInterface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::UI::ViewManagement::ActivationViewSwitcher>();
            (::windows::core::Interface::vtable(this).ViewSwitcher)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DialReceiverActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DialReceiverActivatedEventArgs {}
impl ::core::fmt::Debug for DialReceiverActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DialReceiverActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DialReceiverActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.DialReceiverActivatedEventArgs;{fb777ed7-85ee-456e-a44d-85d730e70aed})");
}
impl ::core::clone::Clone for DialReceiverActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DialReceiverActivatedEventArgs {
    type Vtable = IDialReceiverActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DialReceiverActivatedEventArgs {
    const IID: ::windows::core::GUID = <IDialReceiverActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DialReceiverActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DialReceiverActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(DialReceiverActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for DialReceiverActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for DialReceiverActivatedEventArgs {}
impl ::windows::core::CanTryInto<IApplicationViewActivatedEventArgs> for DialReceiverActivatedEventArgs {}
impl ::windows::core::CanTryInto<IDialReceiverActivatedEventArgs> for DialReceiverActivatedEventArgs {}
impl ::windows::core::CanTryInto<ILaunchActivatedEventArgs> for DialReceiverActivatedEventArgs {}
impl ::windows::core::CanTryInto<IViewSwitcherProvider> for DialReceiverActivatedEventArgs {}
unsafe impl ::core::marker::Send for DialReceiverActivatedEventArgs {}
unsafe impl ::core::marker::Sync for DialReceiverActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct FileActivatedEventArgs(::windows::core::IUnknown);
impl FileActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::ComInterface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>();
            (::windows::core::Interface::vtable(this).Files)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IFileActivatedEventArgsWithCallerPackageFamilyName>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).CallerPackageFamilyName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn NeighboringFilesQuery(&self) -> ::windows::core::Result<super::super::Storage::Search::StorageFileQueryResult> {
        let this = &::windows::core::ComInterface::cast::<IFileActivatedEventArgsWithNeighboringFiles>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Search::StorageFileQueryResult>();
            (::windows::core::Interface::vtable(this).NeighboringFilesQuery)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::ComInterface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::UI::ViewManagement::ActivationViewSwitcher>();
            (::windows::core::Interface::vtable(this).ViewSwitcher)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for FileActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileActivatedEventArgs {}
impl ::core::fmt::Debug for FileActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for FileActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileActivatedEventArgs;{bb2afc33-93b1-42ed-8b26-236dd9c78496})");
}
impl ::core::clone::Clone for FileActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for FileActivatedEventArgs {
    type Vtable = IFileActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for FileActivatedEventArgs {
    const IID: ::windows::core::GUID = <IFileActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for FileActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(FileActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for FileActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for FileActivatedEventArgs {}
impl ::windows::core::CanTryInto<IApplicationViewActivatedEventArgs> for FileActivatedEventArgs {}
impl ::windows::core::CanTryInto<IFileActivatedEventArgs> for FileActivatedEventArgs {}
impl ::windows::core::CanTryInto<IFileActivatedEventArgsWithCallerPackageFamilyName> for FileActivatedEventArgs {}
impl ::windows::core::CanTryInto<IFileActivatedEventArgsWithNeighboringFiles> for FileActivatedEventArgs {}
impl ::windows::core::CanTryInto<IViewSwitcherProvider> for FileActivatedEventArgs {}
unsafe impl ::core::marker::Send for FileActivatedEventArgs {}
unsafe impl ::core::marker::Sync for FileActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct FileOpenPickerActivatedEventArgs(::windows::core::IUnknown);
impl FileOpenPickerActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileOpenPickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Pickers::Provider::FileOpenPickerUI>();
            (::windows::core::Interface::vtable(this).FileOpenPickerUI)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IFileOpenPickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).CallerPackageFamilyName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for FileOpenPickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileOpenPickerActivatedEventArgs {}
impl ::core::fmt::Debug for FileOpenPickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileOpenPickerActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for FileOpenPickerActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileOpenPickerActivatedEventArgs;{72827082-5525-4bf2-bc09-1f5095d4964d})");
}
impl ::core::clone::Clone for FileOpenPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for FileOpenPickerActivatedEventArgs {
    type Vtable = IFileOpenPickerActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for FileOpenPickerActivatedEventArgs {
    const IID: ::windows::core::GUID = <IFileOpenPickerActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for FileOpenPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileOpenPickerActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(FileOpenPickerActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for FileOpenPickerActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for FileOpenPickerActivatedEventArgs {}
impl ::windows::core::CanTryInto<IFileOpenPickerActivatedEventArgs> for FileOpenPickerActivatedEventArgs {}
impl ::windows::core::CanTryInto<IFileOpenPickerActivatedEventArgs2> for FileOpenPickerActivatedEventArgs {}
unsafe impl ::core::marker::Send for FileOpenPickerActivatedEventArgs {}
unsafe impl ::core::marker::Sync for FileOpenPickerActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct FileOpenPickerContinuationEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl FileOpenPickerContinuationEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::ComInterface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::ValueSet>();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>>();
            (::windows::core::Interface::vtable(this).Files)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for FileOpenPickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for FileOpenPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for FileOpenPickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileOpenPickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for FileOpenPickerContinuationEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileOpenPickerContinuationEventArgs;{f0fa3f3a-d4e8-4ad3-9c34-2308f32fcec9})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for FileOpenPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for FileOpenPickerContinuationEventArgs {
    type Vtable = IFileOpenPickerContinuationEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for FileOpenPickerContinuationEventArgs {
    const IID: ::windows::core::GUID = <IFileOpenPickerContinuationEventArgs as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for FileOpenPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileOpenPickerContinuationEventArgs";
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(FileOpenPickerContinuationEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows::core::CanTryInto<IActivatedEventArgs> for FileOpenPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for FileOpenPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows::core::CanTryInto<IContinuationActivatedEventArgs> for FileOpenPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows::core::CanTryInto<IFileOpenPickerContinuationEventArgs> for FileOpenPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for FileOpenPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for FileOpenPickerContinuationEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct FileSavePickerActivatedEventArgs(::windows::core::IUnknown);
impl FileSavePickerActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Pickers_Provider\"`*"]
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileSavePickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Pickers::Provider::FileSavePickerUI>();
            (::windows::core::Interface::vtable(this).FileSavePickerUI)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).CallerPackageFamilyName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).EnterpriseId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for FileSavePickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileSavePickerActivatedEventArgs {}
impl ::core::fmt::Debug for FileSavePickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileSavePickerActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for FileSavePickerActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileSavePickerActivatedEventArgs;{81c19cf1-74e6-4387-82eb-bb8fd64b4346})");
}
impl ::core::clone::Clone for FileSavePickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for FileSavePickerActivatedEventArgs {
    type Vtable = IFileSavePickerActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for FileSavePickerActivatedEventArgs {
    const IID: ::windows::core::GUID = <IFileSavePickerActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for FileSavePickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileSavePickerActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(FileSavePickerActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for FileSavePickerActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for FileSavePickerActivatedEventArgs {}
impl ::windows::core::CanTryInto<IFileSavePickerActivatedEventArgs> for FileSavePickerActivatedEventArgs {}
impl ::windows::core::CanTryInto<IFileSavePickerActivatedEventArgs2> for FileSavePickerActivatedEventArgs {}
unsafe impl ::core::marker::Send for FileSavePickerActivatedEventArgs {}
unsafe impl ::core::marker::Sync for FileSavePickerActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct FileSavePickerContinuationEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl FileSavePickerContinuationEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::ComInterface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::ValueSet>();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::StorageFile>();
            (::windows::core::Interface::vtable(this).File)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for FileSavePickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for FileSavePickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for FileSavePickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileSavePickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for FileSavePickerContinuationEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileSavePickerContinuationEventArgs;{2c846fe1-3bad-4f33-8c8b-e46fae824b4b})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for FileSavePickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for FileSavePickerContinuationEventArgs {
    type Vtable = IFileSavePickerContinuationEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for FileSavePickerContinuationEventArgs {
    const IID: ::windows::core::GUID = <IFileSavePickerContinuationEventArgs as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for FileSavePickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileSavePickerContinuationEventArgs";
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(FileSavePickerContinuationEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows::core::CanTryInto<IActivatedEventArgs> for FileSavePickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for FileSavePickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows::core::CanTryInto<IContinuationActivatedEventArgs> for FileSavePickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows::core::CanTryInto<IFileSavePickerContinuationEventArgs> for FileSavePickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for FileSavePickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for FileSavePickerContinuationEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct FolderPickerContinuationEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl FolderPickerContinuationEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::ComInterface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::ValueSet>();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub fn Folder(&self) -> ::windows::core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::StorageFolder>();
            (::windows::core::Interface::vtable(this).Folder)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for FolderPickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for FolderPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for FolderPickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FolderPickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for FolderPickerContinuationEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FolderPickerContinuationEventArgs;{51882366-9f4b-498f-beb0-42684f6e1c29})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for FolderPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for FolderPickerContinuationEventArgs {
    type Vtable = IFolderPickerContinuationEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for FolderPickerContinuationEventArgs {
    const IID: ::windows::core::GUID = <IFolderPickerContinuationEventArgs as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for FolderPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FolderPickerContinuationEventArgs";
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(FolderPickerContinuationEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows::core::CanTryInto<IActivatedEventArgs> for FolderPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for FolderPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows::core::CanTryInto<IContinuationActivatedEventArgs> for FolderPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows::core::CanTryInto<IFolderPickerContinuationEventArgs> for FolderPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for FolderPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for FolderPickerContinuationEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct LaunchActivatedEventArgs(::windows::core::IUnknown);
impl LaunchActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::ComInterface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Arguments)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).TileId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TileActivatedInfo(&self) -> ::windows::core::Result<TileActivatedInfo> {
        let this = &::windows::core::ComInterface::cast::<ILaunchActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<TileActivatedInfo>();
            (::windows::core::Interface::vtable(this).TileActivatedInfo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PrelaunchActivated(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IPrelaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).PrelaunchActivated)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::ComInterface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::UI::ViewManagement::ActivationViewSwitcher>();
            (::windows::core::Interface::vtable(this).ViewSwitcher)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for LaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LaunchActivatedEventArgs {}
impl ::core::fmt::Debug for LaunchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LaunchActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LaunchActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LaunchActivatedEventArgs;{fbc93e26-a14a-4b4f-82b0-33bed920af52})");
}
impl ::core::clone::Clone for LaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LaunchActivatedEventArgs {
    type Vtable = ILaunchActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LaunchActivatedEventArgs {
    const IID: ::windows::core::GUID = <ILaunchActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LaunchActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(LaunchActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for LaunchActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for LaunchActivatedEventArgs {}
impl ::windows::core::CanTryInto<IApplicationViewActivatedEventArgs> for LaunchActivatedEventArgs {}
impl ::windows::core::CanTryInto<ILaunchActivatedEventArgs> for LaunchActivatedEventArgs {}
impl ::windows::core::CanTryInto<ILaunchActivatedEventArgs2> for LaunchActivatedEventArgs {}
impl ::windows::core::CanTryInto<IPrelaunchActivatedEventArgs> for LaunchActivatedEventArgs {}
impl ::windows::core::CanTryInto<IViewSwitcherProvider> for LaunchActivatedEventArgs {}
unsafe impl ::core::marker::Send for LaunchActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LaunchActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct LockScreenActivatedEventArgs(::windows::core::IUnknown);
impl LockScreenActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Info(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Info)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for LockScreenActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LockScreenActivatedEventArgs {}
impl ::core::fmt::Debug for LockScreenActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LockScreenActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LockScreenActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LockScreenActivatedEventArgs;{3ca77966-6108-4a41-8220-ee7d133c8532})");
}
impl ::core::clone::Clone for LockScreenActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LockScreenActivatedEventArgs {
    type Vtable = ILockScreenActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LockScreenActivatedEventArgs {
    const IID: ::windows::core::GUID = <ILockScreenActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LockScreenActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(LockScreenActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for LockScreenActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for LockScreenActivatedEventArgs {}
impl ::windows::core::CanTryInto<ILockScreenActivatedEventArgs> for LockScreenActivatedEventArgs {}
unsafe impl ::core::marker::Send for LockScreenActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct LockScreenCallActivatedEventArgs(::windows::core::IUnknown);
impl LockScreenCallActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::ComInterface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Arguments)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).TileId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls\"`*"]
    #[cfg(feature = "ApplicationModel_Calls")]
    pub fn CallUI(&self) -> ::windows::core::Result<super::Calls::LockScreenCallUI> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Calls::LockScreenCallUI>();
            (::windows::core::Interface::vtable(this).CallUI)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::ComInterface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::UI::ViewManagement::ActivationViewSwitcher>();
            (::windows::core::Interface::vtable(this).ViewSwitcher)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for LockScreenCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LockScreenCallActivatedEventArgs {}
impl ::core::fmt::Debug for LockScreenCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LockScreenCallActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LockScreenCallActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LockScreenCallActivatedEventArgs;{06f37fbe-b5f2-448b-b13e-e328ac1c516a})");
}
impl ::core::clone::Clone for LockScreenCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LockScreenCallActivatedEventArgs {
    type Vtable = ILockScreenCallActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LockScreenCallActivatedEventArgs {
    const IID: ::windows::core::GUID = <ILockScreenCallActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LockScreenCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenCallActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(LockScreenCallActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for LockScreenCallActivatedEventArgs {}
impl ::windows::core::CanTryInto<IApplicationViewActivatedEventArgs> for LockScreenCallActivatedEventArgs {}
impl ::windows::core::CanTryInto<ILaunchActivatedEventArgs> for LockScreenCallActivatedEventArgs {}
impl ::windows::core::CanTryInto<ILockScreenCallActivatedEventArgs> for LockScreenCallActivatedEventArgs {}
impl ::windows::core::CanTryInto<IViewSwitcherProvider> for LockScreenCallActivatedEventArgs {}
unsafe impl ::core::marker::Send for LockScreenCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenCallActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct LockScreenComponentActivatedEventArgs(::windows::core::IUnknown);
impl LockScreenComponentActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for LockScreenComponentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LockScreenComponentActivatedEventArgs {}
impl ::core::fmt::Debug for LockScreenComponentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LockScreenComponentActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LockScreenComponentActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LockScreenComponentActivatedEventArgs;{cf651713-cd08-4fd8-b697-a281b6544e2e})");
}
impl ::core::clone::Clone for LockScreenComponentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LockScreenComponentActivatedEventArgs {
    type Vtable = IActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LockScreenComponentActivatedEventArgs {
    const IID: ::windows::core::GUID = <IActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LockScreenComponentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenComponentActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(LockScreenComponentActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for LockScreenComponentActivatedEventArgs {}
unsafe impl ::core::marker::Send for LockScreenComponentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenComponentActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct PhoneCallActivatedEventArgs(::windows::core::IUnknown);
impl PhoneCallActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).LineId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PhoneCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneCallActivatedEventArgs {}
impl ::core::fmt::Debug for PhoneCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneCallActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PhoneCallActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.PhoneCallActivatedEventArgs;{54615221-a3c1-4ced-b62f-8c60523619ad})");
}
impl ::core::clone::Clone for PhoneCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PhoneCallActivatedEventArgs {
    type Vtable = IPhoneCallActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PhoneCallActivatedEventArgs {
    const IID: ::windows::core::GUID = <IPhoneCallActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PhoneCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PhoneCallActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(PhoneCallActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for PhoneCallActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for PhoneCallActivatedEventArgs {}
impl ::windows::core::CanTryInto<IPhoneCallActivatedEventArgs> for PhoneCallActivatedEventArgs {}
unsafe impl ::core::marker::Send for PhoneCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PhoneCallActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct PickerReturnedActivatedEventArgs(::windows::core::IUnknown);
impl PickerReturnedActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PickerOperationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).PickerOperationId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PickerReturnedActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PickerReturnedActivatedEventArgs {}
impl ::core::fmt::Debug for PickerReturnedActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PickerReturnedActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PickerReturnedActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.PickerReturnedActivatedEventArgs;{360defb9-a9d3-4984-a4ed-9ec734604921})");
}
impl ::core::clone::Clone for PickerReturnedActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PickerReturnedActivatedEventArgs {
    type Vtable = IPickerReturnedActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PickerReturnedActivatedEventArgs {
    const IID: ::windows::core::GUID = <IPickerReturnedActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PickerReturnedActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PickerReturnedActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(PickerReturnedActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for PickerReturnedActivatedEventArgs {}
impl ::windows::core::CanTryInto<IPickerReturnedActivatedEventArgs> for PickerReturnedActivatedEventArgs {}
unsafe impl ::core::marker::Send for PickerReturnedActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PickerReturnedActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct Print3DWorkflowActivatedEventArgs(::windows::core::IUnknown);
impl Print3DWorkflowActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Workflow(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Devices::Printers::Extensions::Print3DWorkflow>();
            (::windows::core::Interface::vtable(this).Workflow)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for Print3DWorkflowActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DWorkflowActivatedEventArgs {}
impl ::core::fmt::Debug for Print3DWorkflowActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DWorkflowActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Print3DWorkflowActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.Print3DWorkflowActivatedEventArgs;{3f57e78b-f2ac-4619-8302-ef855e1c9b90})");
}
impl ::core::clone::Clone for Print3DWorkflowActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Print3DWorkflowActivatedEventArgs {
    type Vtable = IPrint3DWorkflowActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Print3DWorkflowActivatedEventArgs {
    const IID: ::windows::core::GUID = <IPrint3DWorkflowActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Print3DWorkflowActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.Print3DWorkflowActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(Print3DWorkflowActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for Print3DWorkflowActivatedEventArgs {}
impl ::windows::core::CanTryInto<IPrint3DWorkflowActivatedEventArgs> for Print3DWorkflowActivatedEventArgs {}
unsafe impl ::core::marker::Send for Print3DWorkflowActivatedEventArgs {}
unsafe impl ::core::marker::Sync for Print3DWorkflowActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct PrintTaskSettingsActivatedEventArgs(::windows::core::IUnknown);
impl PrintTaskSettingsActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers_Extensions\"`*"]
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Configuration(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Devices::Printers::Extensions::PrintTaskConfiguration>();
            (::windows::core::Interface::vtable(this).Configuration)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintTaskSettingsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskSettingsActivatedEventArgs {}
impl ::core::fmt::Debug for PrintTaskSettingsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskSettingsActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintTaskSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.PrintTaskSettingsActivatedEventArgs;{ee30a0c9-ce56-4865-ba8e-8954ac271107})");
}
impl ::core::clone::Clone for PrintTaskSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintTaskSettingsActivatedEventArgs {
    type Vtable = IPrintTaskSettingsActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintTaskSettingsActivatedEventArgs {
    const IID: ::windows::core::GUID = <IPrintTaskSettingsActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintTaskSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PrintTaskSettingsActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(PrintTaskSettingsActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for PrintTaskSettingsActivatedEventArgs {}
impl ::windows::core::CanTryInto<IPrintTaskSettingsActivatedEventArgs> for PrintTaskSettingsActivatedEventArgs {}
unsafe impl ::core::marker::Send for PrintTaskSettingsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PrintTaskSettingsActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ProtocolActivatedEventArgs(::windows::core::IUnknown);
impl ProtocolActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::ComInterface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).Uri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).CallerPackageFamilyName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::ComInterface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::ValueSet>();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::ComInterface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::UI::ViewManagement::ActivationViewSwitcher>();
            (::windows::core::Interface::vtable(this).ViewSwitcher)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ProtocolActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtocolActivatedEventArgs {}
impl ::core::fmt::Debug for ProtocolActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtocolActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ProtocolActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ProtocolActivatedEventArgs;{6095f4dd-b7c0-46ab-81fe-d90f36d00d24})");
}
impl ::core::clone::Clone for ProtocolActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ProtocolActivatedEventArgs {
    type Vtable = IProtocolActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ProtocolActivatedEventArgs {
    const IID: ::windows::core::GUID = <IProtocolActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ProtocolActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ProtocolActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(ProtocolActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for ProtocolActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for ProtocolActivatedEventArgs {}
impl ::windows::core::CanTryInto<IApplicationViewActivatedEventArgs> for ProtocolActivatedEventArgs {}
impl ::windows::core::CanTryInto<IProtocolActivatedEventArgs> for ProtocolActivatedEventArgs {}
impl ::windows::core::CanTryInto<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ProtocolActivatedEventArgs {}
impl ::windows::core::CanTryInto<IViewSwitcherProvider> for ProtocolActivatedEventArgs {}
unsafe impl ::core::marker::Send for ProtocolActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ProtocolActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ProtocolForResultsActivatedEventArgs(::windows::core::IUnknown);
impl ProtocolForResultsActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::ComInterface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::ComInterface::cast::<IProtocolActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).Uri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).CallerPackageFamilyName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::ComInterface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::ValueSet>();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn ProtocolForResultsOperation(&self) -> ::windows::core::Result<super::super::System::ProtocolForResultsOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::ProtocolForResultsOperation>();
            (::windows::core::Interface::vtable(this).ProtocolForResultsOperation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::ComInterface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::UI::ViewManagement::ActivationViewSwitcher>();
            (::windows::core::Interface::vtable(this).ViewSwitcher)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ProtocolForResultsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProtocolForResultsActivatedEventArgs {}
impl ::core::fmt::Debug for ProtocolForResultsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtocolForResultsActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ProtocolForResultsActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ProtocolForResultsActivatedEventArgs;{e75132c2-7ae7-4517-80ac-dbe8d7cc5b9c})");
}
impl ::core::clone::Clone for ProtocolForResultsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ProtocolForResultsActivatedEventArgs {
    type Vtable = IProtocolForResultsActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ProtocolForResultsActivatedEventArgs {
    const IID: ::windows::core::GUID = <IProtocolForResultsActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ProtocolForResultsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ProtocolForResultsActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(ProtocolForResultsActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for ProtocolForResultsActivatedEventArgs {}
impl ::windows::core::CanTryInto<IApplicationViewActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {}
impl ::windows::core::CanTryInto<IProtocolActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {}
impl ::windows::core::CanTryInto<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ProtocolForResultsActivatedEventArgs {}
impl ::windows::core::CanTryInto<IProtocolForResultsActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {}
impl ::windows::core::CanTryInto<IViewSwitcherProvider> for ProtocolForResultsActivatedEventArgs {}
unsafe impl ::core::marker::Send for ProtocolForResultsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ProtocolForResultsActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct RestrictedLaunchActivatedEventArgs(::windows::core::IUnknown);
impl RestrictedLaunchActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SharedContext(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).SharedContext)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for RestrictedLaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RestrictedLaunchActivatedEventArgs {}
impl ::core::fmt::Debug for RestrictedLaunchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RestrictedLaunchActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for RestrictedLaunchActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.RestrictedLaunchActivatedEventArgs;{e0b7ac81-bfc3-4344-a5da-19fd5a27baae})");
}
impl ::core::clone::Clone for RestrictedLaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for RestrictedLaunchActivatedEventArgs {
    type Vtable = IRestrictedLaunchActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for RestrictedLaunchActivatedEventArgs {
    const IID: ::windows::core::GUID = <IRestrictedLaunchActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for RestrictedLaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.RestrictedLaunchActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(RestrictedLaunchActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for RestrictedLaunchActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for RestrictedLaunchActivatedEventArgs {}
impl ::windows::core::CanTryInto<IRestrictedLaunchActivatedEventArgs> for RestrictedLaunchActivatedEventArgs {}
unsafe impl ::core::marker::Send for RestrictedLaunchActivatedEventArgs {}
unsafe impl ::core::marker::Sync for RestrictedLaunchActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct SearchActivatedEventArgs(::windows::core::IUnknown);
impl SearchActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::ComInterface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).QueryText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Language)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Search\"`*"]
    #[cfg(feature = "ApplicationModel_Search")]
    pub fn LinguisticDetails(&self) -> ::windows::core::Result<super::Search::SearchPaneQueryLinguisticDetails> {
        let this = &::windows::core::ComInterface::cast::<ISearchActivatedEventArgsWithLinguisticDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Search::SearchPaneQueryLinguisticDetails>();
            (::windows::core::Interface::vtable(this).LinguisticDetails)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::ComInterface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::UI::ViewManagement::ActivationViewSwitcher>();
            (::windows::core::Interface::vtable(this).ViewSwitcher)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SearchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SearchActivatedEventArgs {}
impl ::core::fmt::Debug for SearchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SearchActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SearchActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.SearchActivatedEventArgs;{8cb36951-58c8-43e3-94bc-41d33f8b630e})");
}
impl ::core::clone::Clone for SearchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SearchActivatedEventArgs {
    type Vtable = ISearchActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SearchActivatedEventArgs {
    const IID: ::windows::core::GUID = <ISearchActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SearchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.SearchActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(SearchActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for SearchActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for SearchActivatedEventArgs {}
impl ::windows::core::CanTryInto<IApplicationViewActivatedEventArgs> for SearchActivatedEventArgs {}
impl ::windows::core::CanTryInto<ISearchActivatedEventArgs> for SearchActivatedEventArgs {}
impl ::windows::core::CanTryInto<ISearchActivatedEventArgsWithLinguisticDetails> for SearchActivatedEventArgs {}
impl ::windows::core::CanTryInto<IViewSwitcherProvider> for SearchActivatedEventArgs {}
unsafe impl ::core::marker::Send for SearchActivatedEventArgs {}
unsafe impl ::core::marker::Sync for SearchActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ShareTargetActivatedEventArgs(::windows::core::IUnknown);
impl ShareTargetActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer_ShareTarget\"`*"]
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    pub fn ShareOperation(&self) -> ::windows::core::Result<super::DataTransfer::ShareTarget::ShareOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::DataTransfer::ShareTarget::ShareOperation>();
            (::windows::core::Interface::vtable(this).ShareOperation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ShareTargetActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ShareTargetActivatedEventArgs {}
impl ::core::fmt::Debug for ShareTargetActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ShareTargetActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ShareTargetActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ShareTargetActivatedEventArgs;{4bdaf9c8-cdb2-4acb-bfc3-6648563378ec})");
}
impl ::core::clone::Clone for ShareTargetActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ShareTargetActivatedEventArgs {
    type Vtable = IShareTargetActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ShareTargetActivatedEventArgs {
    const IID: ::windows::core::GUID = <IShareTargetActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ShareTargetActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ShareTargetActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(ShareTargetActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for ShareTargetActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for ShareTargetActivatedEventArgs {}
impl ::windows::core::CanTryInto<IShareTargetActivatedEventArgs> for ShareTargetActivatedEventArgs {}
unsafe impl ::core::marker::Send for ShareTargetActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ShareTargetActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct SplashScreen(::windows::core::IUnknown);
impl SplashScreen {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ImageLocation(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Rect>();
            (::windows::core::Interface::vtable(this).ImageLocation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Dismissed(&self, handler: &super::super::Foundation::TypedEventHandler<SplashScreen, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Dismissed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDismissed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDismissed)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
}
impl ::core::cmp::PartialEq for SplashScreen {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SplashScreen {}
impl ::core::fmt::Debug for SplashScreen {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SplashScreen").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SplashScreen {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.SplashScreen;{ca4d975c-d4d6-43f0-97c0-0833c6391c24})");
}
impl ::core::clone::Clone for SplashScreen {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SplashScreen {
    type Vtable = ISplashScreen_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SplashScreen {
    const IID: ::windows::core::GUID = <ISplashScreen as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SplashScreen {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.SplashScreen";
}
::windows::imp::interface_hierarchy!(SplashScreen, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct StartupTaskActivatedEventArgs(::windows::core::IUnknown);
impl StartupTaskActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).TaskId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for StartupTaskActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StartupTaskActivatedEventArgs {}
impl ::core::fmt::Debug for StartupTaskActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StartupTaskActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StartupTaskActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.StartupTaskActivatedEventArgs;{03b11a58-5276-4d91-8621-54611864d5fa})");
}
impl ::core::clone::Clone for StartupTaskActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for StartupTaskActivatedEventArgs {
    type Vtable = IStartupTaskActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for StartupTaskActivatedEventArgs {
    const IID: ::windows::core::GUID = <IStartupTaskActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for StartupTaskActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.StartupTaskActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(StartupTaskActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for StartupTaskActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for StartupTaskActivatedEventArgs {}
impl ::windows::core::CanTryInto<IStartupTaskActivatedEventArgs> for StartupTaskActivatedEventArgs {}
unsafe impl ::core::marker::Send for StartupTaskActivatedEventArgs {}
unsafe impl ::core::marker::Sync for StartupTaskActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct TileActivatedInfo(::windows::core::IUnknown);
impl TileActivatedInfo {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"UI_Notifications\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Notifications"))]
    pub fn RecentlyShownNotifications(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::UI::Notifications::ShownTileNotification>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::UI::Notifications::ShownTileNotification>>();
            (::windows::core::Interface::vtable(this).RecentlyShownNotifications)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for TileActivatedInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TileActivatedInfo {}
impl ::core::fmt::Debug for TileActivatedInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TileActivatedInfo").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for TileActivatedInfo {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.TileActivatedInfo;{80e4a3b1-3980-4f17-b738-89194e0b8f65})");
}
impl ::core::clone::Clone for TileActivatedInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TileActivatedInfo {
    type Vtable = ITileActivatedInfo_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TileActivatedInfo {
    const IID: ::windows::core::GUID = <ITileActivatedInfo as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TileActivatedInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.TileActivatedInfo";
}
::windows::imp::interface_hierarchy!(TileActivatedInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for TileActivatedInfo {}
unsafe impl ::core::marker::Sync for TileActivatedInfo {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct ToastNotificationActivatedEventArgs(::windows::core::IUnknown);
impl ToastNotificationActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::ComInterface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Argument(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Argument)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserInput(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::ValueSet>();
            (::windows::core::Interface::vtable(this).UserInput)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ToastNotificationActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotificationActivatedEventArgs {}
impl ::core::fmt::Debug for ToastNotificationActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ToastNotificationActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ToastNotificationActivatedEventArgs;{92a86f82-5290-431d-be85-c4aaeeb8685f})");
}
impl ::core::clone::Clone for ToastNotificationActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ToastNotificationActivatedEventArgs {
    type Vtable = IToastNotificationActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ToastNotificationActivatedEventArgs {
    const IID: ::windows::core::GUID = <IToastNotificationActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ToastNotificationActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ToastNotificationActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(ToastNotificationActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for ToastNotificationActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for ToastNotificationActivatedEventArgs {}
impl ::windows::core::CanTryInto<IApplicationViewActivatedEventArgs> for ToastNotificationActivatedEventArgs {}
impl ::windows::core::CanTryInto<IToastNotificationActivatedEventArgs> for ToastNotificationActivatedEventArgs {}
unsafe impl ::core::marker::Send for ToastNotificationActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ToastNotificationActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct UserDataAccountProviderActivatedEventArgs(::windows::core::IUnknown);
impl UserDataAccountProviderActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    pub fn Operation(&self) -> ::windows::core::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation>();
            (::windows::core::Interface::vtable(this).Operation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for UserDataAccountProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccountProviderActivatedEventArgs {}
impl ::core::fmt::Debug for UserDataAccountProviderActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountProviderActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for UserDataAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.UserDataAccountProviderActivatedEventArgs;{1bc9f723-8ef1-4a51-a63a-fe711eeab607})");
}
impl ::core::clone::Clone for UserDataAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for UserDataAccountProviderActivatedEventArgs {
    type Vtable = IUserDataAccountProviderActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for UserDataAccountProviderActivatedEventArgs {
    const IID: ::windows::core::GUID = <IUserDataAccountProviderActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for UserDataAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.UserDataAccountProviderActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(UserDataAccountProviderActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for UserDataAccountProviderActivatedEventArgs {}
impl ::windows::core::CanTryInto<IUserDataAccountProviderActivatedEventArgs> for UserDataAccountProviderActivatedEventArgs {}
unsafe impl ::core::marker::Send for UserDataAccountProviderActivatedEventArgs {}
unsafe impl ::core::marker::Sync for UserDataAccountProviderActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct VoiceCommandActivatedEventArgs(::windows::core::IUnknown);
impl VoiceCommandActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
    #[cfg(feature = "Media_SpeechRecognition")]
    pub fn Result(&self) -> ::windows::core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Media::SpeechRecognition::SpeechRecognitionResult>();
            (::windows::core::Interface::vtable(this).Result)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VoiceCommandActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandActivatedEventArgs {}
impl ::core::fmt::Debug for VoiceCommandActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for VoiceCommandActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.VoiceCommandActivatedEventArgs;{ab92dcfd-8d43-4de6-9775-20704b581b00})");
}
impl ::core::clone::Clone for VoiceCommandActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for VoiceCommandActivatedEventArgs {
    type Vtable = IVoiceCommandActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for VoiceCommandActivatedEventArgs {
    const IID: ::windows::core::GUID = <IVoiceCommandActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for VoiceCommandActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.VoiceCommandActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(VoiceCommandActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for VoiceCommandActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for VoiceCommandActivatedEventArgs {}
impl ::windows::core::CanTryInto<IVoiceCommandActivatedEventArgs> for VoiceCommandActivatedEventArgs {}
unsafe impl ::core::marker::Send for VoiceCommandActivatedEventArgs {}
unsafe impl ::core::marker::Sync for VoiceCommandActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct WalletActionActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl WalletActionActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ItemId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ItemId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Wallet\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "ApplicationModel_Wallet", feature = "deprecated"))]
    pub fn ActionKind(&self) -> ::windows::core::Result<super::Wallet::WalletActionKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Wallet::WalletActionKind>();
            (::windows::core::Interface::vtable(this).ActionKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn ActionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ActionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for WalletActionActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for WalletActionActivatedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for WalletActionActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WalletActionActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for WalletActionActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.WalletActionActivatedEventArgs;{fcfc027b-1a1a-4d22-923f-ae6f45fa52d9})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for WalletActionActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for WalletActionActivatedEventArgs {
    type Vtable = IWalletActionActivatedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for WalletActionActivatedEventArgs {
    const IID: ::windows::core::GUID = <IWalletActionActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for WalletActionActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WalletActionActivatedEventArgs";
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(WalletActionActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows::core::CanTryInto<IActivatedEventArgs> for WalletActionActivatedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows::core::CanTryInto<IWalletActionActivatedEventArgs> for WalletActionActivatedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for WalletActionActivatedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for WalletActionActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct WebAccountProviderActivatedEventArgs(::windows::core::IUnknown);
impl WebAccountProviderActivatedEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web_Provider\"`*"]
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    pub fn Operation(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation>();
            (::windows::core::Interface::vtable(this).Operation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WebAccountProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAccountProviderActivatedEventArgs {}
impl ::core::fmt::Debug for WebAccountProviderActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAccountProviderActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WebAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.WebAccountProviderActivatedEventArgs;{72b71774-98ea-4ccf-9752-46d9051004f1})");
}
impl ::core::clone::Clone for WebAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WebAccountProviderActivatedEventArgs {
    type Vtable = IWebAccountProviderActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WebAccountProviderActivatedEventArgs {
    const IID: ::windows::core::GUID = <IWebAccountProviderActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WebAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WebAccountProviderActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(WebAccountProviderActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for WebAccountProviderActivatedEventArgs {}
impl ::windows::core::CanTryInto<IActivatedEventArgsWithUser> for WebAccountProviderActivatedEventArgs {}
impl ::windows::core::CanTryInto<IWebAccountProviderActivatedEventArgs> for WebAccountProviderActivatedEventArgs {}
unsafe impl ::core::marker::Send for WebAccountProviderActivatedEventArgs {}
unsafe impl ::core::marker::Sync for WebAccountProviderActivatedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
pub struct WebAuthenticationBrokerContinuationEventArgs(::windows::core::IUnknown);
impl WebAuthenticationBrokerContinuationEventArgs {
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::ComInterface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::ValueSet>();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Authentication_Web\"`*"]
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn WebAuthenticationResult(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Security::Authentication::Web::WebAuthenticationResult>();
            (::windows::core::Interface::vtable(this).WebAuthenticationResult)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WebAuthenticationBrokerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAuthenticationBrokerContinuationEventArgs {}
impl ::core::fmt::Debug for WebAuthenticationBrokerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAuthenticationBrokerContinuationEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WebAuthenticationBrokerContinuationEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.WebAuthenticationBrokerContinuationEventArgs;{75dda3d4-7714-453d-b7ff-b95e3a1709da})");
}
impl ::core::clone::Clone for WebAuthenticationBrokerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WebAuthenticationBrokerContinuationEventArgs {
    type Vtable = IWebAuthenticationBrokerContinuationEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WebAuthenticationBrokerContinuationEventArgs {
    const IID: ::windows::core::GUID = <IWebAuthenticationBrokerContinuationEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WebAuthenticationBrokerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WebAuthenticationBrokerContinuationEventArgs";
}
::windows::imp::interface_hierarchy!(WebAuthenticationBrokerContinuationEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IActivatedEventArgs> for WebAuthenticationBrokerContinuationEventArgs {}
impl ::windows::core::CanTryInto<IContinuationActivatedEventArgs> for WebAuthenticationBrokerContinuationEventArgs {}
impl ::windows::core::CanTryInto<IWebAuthenticationBrokerContinuationEventArgs> for WebAuthenticationBrokerContinuationEventArgs {}
unsafe impl ::core::marker::Send for WebAuthenticationBrokerContinuationEventArgs {}
unsafe impl ::core::marker::Sync for WebAuthenticationBrokerContinuationEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ActivationKind(pub i32);
impl ActivationKind {
    pub const Launch: Self = Self(0i32);
    pub const Search: Self = Self(1i32);
    pub const ShareTarget: Self = Self(2i32);
    pub const File: Self = Self(3i32);
    pub const Protocol: Self = Self(4i32);
    pub const FileOpenPicker: Self = Self(5i32);
    pub const FileSavePicker: Self = Self(6i32);
    pub const CachedFileUpdater: Self = Self(7i32);
    pub const ContactPicker: Self = Self(8i32);
    pub const Device: Self = Self(9i32);
    pub const PrintTaskSettings: Self = Self(10i32);
    pub const CameraSettings: Self = Self(11i32);
    pub const RestrictedLaunch: Self = Self(12i32);
    pub const AppointmentsProvider: Self = Self(13i32);
    pub const Contact: Self = Self(14i32);
    pub const LockScreenCall: Self = Self(15i32);
    pub const VoiceCommand: Self = Self(16i32);
    pub const LockScreen: Self = Self(17i32);
    pub const PickerReturned: Self = Self(1000i32);
    pub const WalletAction: Self = Self(1001i32);
    pub const PickFileContinuation: Self = Self(1002i32);
    pub const PickSaveFileContinuation: Self = Self(1003i32);
    pub const PickFolderContinuation: Self = Self(1004i32);
    pub const WebAuthenticationBrokerContinuation: Self = Self(1005i32);
    pub const WebAccountProvider: Self = Self(1006i32);
    pub const ComponentUI: Self = Self(1007i32);
    pub const ProtocolForResults: Self = Self(1009i32);
    pub const ToastNotification: Self = Self(1010i32);
    pub const Print3DWorkflow: Self = Self(1011i32);
    pub const DialReceiver: Self = Self(1012i32);
    pub const DevicePairing: Self = Self(1013i32);
    pub const UserDataAccountsProvider: Self = Self(1014i32);
    pub const FilePickerExperience: Self = Self(1015i32);
    pub const LockScreenComponent: Self = Self(1016i32);
    pub const ContactPanel: Self = Self(1017i32);
    pub const PrintWorkflowForegroundTask: Self = Self(1018i32);
    pub const GameUIProvider: Self = Self(1019i32);
    pub const StartupTask: Self = Self(1020i32);
    pub const CommandLineLaunch: Self = Self(1021i32);
    pub const BarcodeScannerProvider: Self = Self(1022i32);
    pub const PrintSupportJobUI: Self = Self(1023i32);
    pub const PrintSupportSettingsUI: Self = Self(1024i32);
    pub const PhoneCallActivation: Self = Self(1025i32);
    pub const VpnForeground: Self = Self(1026i32);
}
impl ::core::marker::Copy for ActivationKind {}
impl ::core::clone::Clone for ActivationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ActivationKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ActivationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ActivationKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Activation.ActivationKind;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ApplicationExecutionState(pub i32);
impl ApplicationExecutionState {
    pub const NotRunning: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Suspended: Self = Self(2i32);
    pub const Terminated: Self = Self(3i32);
    pub const ClosedByUser: Self = Self(4i32);
}
impl ::core::marker::Copy for ApplicationExecutionState {}
impl ::core::clone::Clone for ApplicationExecutionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ApplicationExecutionState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ApplicationExecutionState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ApplicationExecutionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationExecutionState").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ApplicationExecutionState {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Activation.ApplicationExecutionState;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
