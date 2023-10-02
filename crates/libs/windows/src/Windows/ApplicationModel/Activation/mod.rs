#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IActivatedEventArgs(::windows_core::IUnknown);
impl IActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{cf651713-cd08-4fd8-b697-a281b6544e2e}");
}
unsafe impl ::windows_core::Interface for IActivatedEventArgs {
    type Vtable = IActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcf651713_cd08_4fd8_b697_a281b6544e2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ActivationKind) -> ::windows_core::HRESULT,
    pub PreviousExecutionState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ApplicationExecutionState) -> ::windows_core::HRESULT,
    pub SplashScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IActivatedEventArgsWithUser(::windows_core::IUnknown);
impl IActivatedEventArgsWithUser {
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IActivatedEventArgsWithUser, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IActivatedEventArgsWithUser {}
impl ::windows_core::RuntimeType for IActivatedEventArgsWithUser {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{1cf09b9e-9962-4936-80ff-afc8e8ae5c8c}");
}
unsafe impl ::windows_core::Interface for IActivatedEventArgsWithUser {
    type Vtable = IActivatedEventArgsWithUser_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IActivatedEventArgsWithUser {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1cf09b9e_9962_4936_80ff_afc8e8ae5c8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedEventArgsWithUser_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IApplicationViewActivatedEventArgs(::windows_core::IUnknown);
impl IApplicationViewActivatedEventArgs {
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IApplicationViewActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IApplicationViewActivatedEventArgs {}
impl ::windows_core::RuntimeType for IApplicationViewActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{930cef4b-b829-40fc-88f4-8513e8a64738}");
}
unsafe impl ::windows_core::Interface for IApplicationViewActivatedEventArgs {
    type Vtable = IApplicationViewActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IApplicationViewActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x930cef4b_b829_40fc_88f4_8513e8a64738);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CurrentlyShownApplicationViewId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentsProviderActivatedEventArgs(::windows_core::IUnknown);
impl IAppointmentsProviderActivatedEventArgs {
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IAppointmentsProviderActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {}
impl ::windows_core::RuntimeType for IAppointmentsProviderActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{3364c405-933c-4e7d-a034-500fb8dcd9f3}");
}
unsafe impl ::windows_core::Interface for IAppointmentsProviderActivatedEventArgs {
    type Vtable = IAppointmentsProviderActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentsProviderActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3364c405_933c_4e7d_a034_500fb8dcd9f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Verb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentsProviderAddAppointmentActivatedEventArgs(::windows_core::IUnknown);
impl IAppointmentsProviderAddAppointmentActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`"]
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn AddAppointmentOperation(&self) -> ::windows_core::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddAppointmentOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IAppointmentsProviderAddAppointmentActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IAppointmentsProviderAddAppointmentActivatedEventArgs {}
impl ::windows_core::CanTryInto<IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderAddAppointmentActivatedEventArgs {}
impl ::windows_core::RuntimeType for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{a2861367-cee5-4e4d-9ed7-41c34ec18b02}");
}
unsafe impl ::windows_core::Interface for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderAddAppointmentActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa2861367_cee5_4e4d_9ed7_41c34ec18b02);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderAddAppointmentActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub AddAppointmentOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    AddAppointmentOperation: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentsProviderRemoveAppointmentActivatedEventArgs(::windows_core::IUnknown);
impl IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`"]
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn RemoveAppointmentOperation(&self) -> ::windows_core::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveAppointmentOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IAppointmentsProviderRemoveAppointmentActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {}
impl ::windows_core::CanTryInto<IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {}
impl ::windows_core::RuntimeType for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{751f3ab8-0b8e-451c-9f15-966e699bac25}");
}
unsafe impl ::windows_core::Interface for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x751f3ab8_0b8e_451c_9f15_966e699bac25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub RemoveAppointmentOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    RemoveAppointmentOperation: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentsProviderReplaceAppointmentActivatedEventArgs(::windows_core::IUnknown);
impl IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`"]
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn ReplaceAppointmentOperation(&self) -> ::windows_core::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceAppointmentOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IAppointmentsProviderReplaceAppointmentActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {}
impl ::windows_core::CanTryInto<IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {}
impl ::windows_core::RuntimeType for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{1551b7d4-a981-4067-8a62-0524e4ade121}");
}
unsafe impl ::windows_core::Interface for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1551b7d4_a981_4067_8a62_0524e4ade121);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub ReplaceAppointmentOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    ReplaceAppointmentOperation: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs(::windows_core::IUnknown);
impl IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn InstanceStartDate(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstanceStartDate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RoamingId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoamingId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
impl ::windows_core::CanTryInto<IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
impl ::windows_core::RuntimeType for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{3958f065-9841-4ca5-999b-885198b9ef2a}");
}
unsafe impl ::windows_core::Interface for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3958f065_9841_4ca5_999b_885198b9ef2a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub InstanceStartDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InstanceStartDate: usize,
    pub LocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RoamingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentsProviderShowTimeFrameActivatedEventArgs(::windows_core::IUnknown);
impl IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TimeToShow(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimeToShow)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IAppointmentsProviderShowTimeFrameActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {}
impl ::windows_core::CanTryInto<IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {}
impl ::windows_core::RuntimeType for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{9baeaba6-0e0b-49aa-babc-12b1dc774986}");
}
unsafe impl ::windows_core::Interface for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowTimeFrameActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9baeaba6_0e0b_49aa_babc_12b1dc774986);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderShowTimeFrameActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub TimeToShow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimeToShow: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundActivatedEventArgs(::windows_core::IUnknown);
impl IBackgroundActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Background\"`"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn TaskInstance(&self) -> ::windows_core::Result<super::Background::IBackgroundTaskInstance> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaskInstance)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IBackgroundActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IBackgroundActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{ab14bee0-e760-440e-a91c-44796de3a92d}");
}
unsafe impl ::windows_core::Interface for IBackgroundActivatedEventArgs {
    type Vtable = IBackgroundActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab14bee0_e760_440e_a91c_44796de3a92d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Background")]
    pub TaskInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))]
    TaskInstance: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBarcodeScannerPreviewActivatedEventArgs(::windows_core::IUnknown);
impl IBarcodeScannerPreviewActivatedEventArgs {
    pub fn ConnectionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IBarcodeScannerPreviewActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IBarcodeScannerPreviewActivatedEventArgs {}
impl ::windows_core::RuntimeType for IBarcodeScannerPreviewActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{6772797c-99bf-4349-af22-e4123560371c}");
}
unsafe impl ::windows_core::Interface for IBarcodeScannerPreviewActivatedEventArgs {
    type Vtable = IBarcodeScannerPreviewActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBarcodeScannerPreviewActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6772797c_99bf_4349_af22_e4123560371c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerPreviewActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ConnectionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICachedFileUpdaterActivatedEventArgs(::windows_core::IUnknown);
impl ICachedFileUpdaterActivatedEventArgs {
    #[doc = "Required features: `\"Storage_Provider\"`"]
    #[cfg(feature = "Storage_Provider")]
    pub fn CachedFileUpdaterUI(&self) -> ::windows_core::Result<super::super::Storage::Provider::CachedFileUpdaterUI> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CachedFileUpdaterUI)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(ICachedFileUpdaterActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for ICachedFileUpdaterActivatedEventArgs {}
impl ::windows_core::RuntimeType for ICachedFileUpdaterActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{d06eb1c7-3805-4ecb-b757-6cf15e26fef3}");
}
unsafe impl ::windows_core::Interface for ICachedFileUpdaterActivatedEventArgs {
    type Vtable = ICachedFileUpdaterActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICachedFileUpdaterActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd06eb1c7_3805_4ecb_b757_6cf15e26fef3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Provider")]
    pub CachedFileUpdaterUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Provider"))]
    CachedFileUpdaterUI: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICameraSettingsActivatedEventArgs(::windows_core::IUnknown);
impl ICameraSettingsActivatedEventArgs {
    pub fn VideoDeviceController(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceController)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VideoDeviceExtension(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceExtension)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(ICameraSettingsActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for ICameraSettingsActivatedEventArgs {}
impl ::windows_core::RuntimeType for ICameraSettingsActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{fb67a508-2dad-490a-9170-dca036eb114b}");
}
unsafe impl ::windows_core::Interface for ICameraSettingsActivatedEventArgs {
    type Vtable = ICameraSettingsActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICameraSettingsActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb67a508_2dad_490a_9170_dca036eb114b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraSettingsActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub VideoDeviceController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub VideoDeviceExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICommandLineActivatedEventArgs(::windows_core::IUnknown);
impl ICommandLineActivatedEventArgs {
    pub fn Operation(&self) -> ::windows_core::Result<CommandLineActivationOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(ICommandLineActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for ICommandLineActivatedEventArgs {}
impl ::windows_core::RuntimeType for ICommandLineActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{4506472c-006a-48eb-8afb-d07ab25e3366}");
}
unsafe impl ::windows_core::Interface for ICommandLineActivatedEventArgs {
    type Vtable = ICommandLineActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICommandLineActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4506472c_006a_48eb_8afb_d07ab25e3366);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandLineActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICommandLineActivationOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICommandLineActivationOperation {
    type Vtable = ICommandLineActivationOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICommandLineActivationOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x994b2841_c59e_4f69_bcfd_b61ed4e622eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandLineActivationOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CurrentDirectoryPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetExitCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub ExitCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactActivatedEventArgs(::windows_core::IUnknown);
impl IContactActivatedEventArgs {
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IContactActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IContactActivatedEventArgs {}
impl ::windows_core::RuntimeType for IContactActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{d627a1c4-c025-4c41-9def-f1eafad075e7}");
}
unsafe impl ::windows_core::Interface for IContactActivatedEventArgs {
    type Vtable = IContactActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd627a1c4_c025_4c41_9def_f1eafad075e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Verb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactCallActivatedEventArgs(::windows_core::IUnknown);
impl IContactCallActivatedEventArgs {
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Contacts\"`"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IContactCallActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IContactCallActivatedEventArgs {}
impl ::windows_core::CanTryInto<IContactActivatedEventArgs> for IContactCallActivatedEventArgs {}
impl ::windows_core::RuntimeType for IContactCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{c2df14c7-30eb-41c6-b3bc-5b1694f9dab3}");
}
unsafe impl ::windows_core::Interface for IContactCallActivatedEventArgs {
    type Vtable = IContactCallActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactCallActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc2df14c7_30eb_41c6_b3bc_5b1694f9dab3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactCallActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactMapActivatedEventArgs(::windows_core::IUnknown);
impl IContactMapActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Contacts\"`"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Address(&self) -> ::windows_core::Result<super::Contacts::ContactAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Address)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Contacts\"`"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IContactMapActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IContactMapActivatedEventArgs {}
impl ::windows_core::CanTryInto<IContactActivatedEventArgs> for IContactMapActivatedEventArgs {}
impl ::windows_core::RuntimeType for IContactMapActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{b32bf870-eee7-4ad2-aaf1-a87effcf00a4}");
}
unsafe impl ::windows_core::Interface for IContactMapActivatedEventArgs {
    type Vtable = IContactMapActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactMapActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb32bf870_eee7_4ad2_aaf1_a87effcf00a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactMapActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Address: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactMessageActivatedEventArgs(::windows_core::IUnknown);
impl IContactMessageActivatedEventArgs {
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Contacts\"`"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IContactMessageActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IContactMessageActivatedEventArgs {}
impl ::windows_core::CanTryInto<IContactActivatedEventArgs> for IContactMessageActivatedEventArgs {}
impl ::windows_core::RuntimeType for IContactMessageActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{de598db2-0e03-43b0-bf56-bcc40b3162df}");
}
unsafe impl ::windows_core::Interface for IContactMessageActivatedEventArgs {
    type Vtable = IContactMessageActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactMessageActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xde598db2_0e03_43b0_bf56_bcc40b3162df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactMessageActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactPanelActivatedEventArgs(::windows_core::IUnknown);
impl IContactPanelActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Contacts\"`"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn ContactPanel(&self) -> ::windows_core::Result<super::Contacts::ContactPanel> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContactPanel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Contacts\"`"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IContactPanelActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IContactPanelActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{52bb63e4-d3d4-4b63-8051-4af2082cab80}");
}
unsafe impl ::windows_core::Interface for IContactPanelActivatedEventArgs {
    type Vtable = IContactPanelActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactPanelActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x52bb63e4_d3d4_4b63_8051_4af2082cab80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPanelActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub ContactPanel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    ContactPanel: usize,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactPickerActivatedEventArgs(::windows_core::IUnknown);
impl IContactPickerActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Contacts_Provider\"`"]
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    pub fn ContactPickerUI(&self) -> ::windows_core::Result<super::Contacts::Provider::ContactPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContactPickerUI)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IContactPickerActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IContactPickerActivatedEventArgs {}
impl ::windows_core::RuntimeType for IContactPickerActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{ce57aae7-6449-45a7-971f-d113be7a8936}");
}
unsafe impl ::windows_core::Interface for IContactPickerActivatedEventArgs {
    type Vtable = IContactPickerActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactPickerActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xce57aae7_6449_45a7_971f_d113be7a8936);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPickerActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    pub ContactPickerUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts_Provider"))]
    ContactPickerUI: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactPostActivatedEventArgs(::windows_core::IUnknown);
impl IContactPostActivatedEventArgs {
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Contacts\"`"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IContactPostActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IContactPostActivatedEventArgs {}
impl ::windows_core::CanTryInto<IContactActivatedEventArgs> for IContactPostActivatedEventArgs {}
impl ::windows_core::RuntimeType for IContactPostActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{b35a3c67-f1e7-4655-ad6e-4857588f552f}");
}
unsafe impl ::windows_core::Interface for IContactPostActivatedEventArgs {
    type Vtable = IContactPostActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactPostActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb35a3c67_f1e7_4655_ad6e_4857588f552f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPostActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactVideoCallActivatedEventArgs(::windows_core::IUnknown);
impl IContactVideoCallActivatedEventArgs {
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Contacts\"`"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IContactVideoCallActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IContactVideoCallActivatedEventArgs {}
impl ::windows_core::CanTryInto<IContactActivatedEventArgs> for IContactVideoCallActivatedEventArgs {}
impl ::windows_core::RuntimeType for IContactVideoCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{61079db8-e3e7-4b4f-858d-5c63a96ef684}");
}
unsafe impl ::windows_core::Interface for IContactVideoCallActivatedEventArgs {
    type Vtable = IContactVideoCallActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactVideoCallActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x61079db8_e3e7_4b4f_858d_5c63a96ef684);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactVideoCallActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ServiceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ServiceUserId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub Contact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))]
    Contact: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactsProviderActivatedEventArgs(::windows_core::IUnknown);
impl IContactsProviderActivatedEventArgs {
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IContactsProviderActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IContactsProviderActivatedEventArgs {}
impl ::windows_core::RuntimeType for IContactsProviderActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{4580dca8-5750-4916-aa52-c0829521eb94}");
}
unsafe impl ::windows_core::Interface for IContactsProviderActivatedEventArgs {
    type Vtable = IContactsProviderActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactsProviderActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4580dca8_5750_4916_aa52_c0829521eb94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactsProviderActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Verb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContinuationActivatedEventArgs(::windows_core::IUnknown);
impl IContinuationActivatedEventArgs {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IContinuationActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IContinuationActivatedEventArgs {}
impl ::windows_core::RuntimeType for IContinuationActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{e58106b5-155f-4a94-a742-c7e08f4e188c}");
}
unsafe impl ::windows_core::Interface for IContinuationActivatedEventArgs {
    type Vtable = IContinuationActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContinuationActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe58106b5_155f_4a94_a742_c7e08f4e188c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContinuationActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ContinuationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ContinuationData: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDeviceActivatedEventArgs(::windows_core::IUnknown);
impl IDeviceActivatedEventArgs {
    pub fn DeviceInformationId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInformationId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IDeviceActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IDeviceActivatedEventArgs {}
impl ::windows_core::RuntimeType for IDeviceActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{cd50b9a9-ce10-44d2-8234-c355a073ef33}");
}
unsafe impl ::windows_core::Interface for IDeviceActivatedEventArgs {
    type Vtable = IDeviceActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDeviceActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd50b9a9_ce10_44d2_8234_c355a073ef33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DeviceInformationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Verb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDevicePairingActivatedEventArgs(::windows_core::IUnknown);
impl IDevicePairingActivatedEventArgs {
    #[doc = "Required features: `\"Devices_Enumeration\"`"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> ::windows_core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInformation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IDevicePairingActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IDevicePairingActivatedEventArgs {}
impl ::windows_core::RuntimeType for IDevicePairingActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{eba0d1e4-ecc6-4148-94ed-f4b37ec05b3e}");
}
unsafe impl ::windows_core::Interface for IDevicePairingActivatedEventArgs {
    type Vtable = IDevicePairingActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDevicePairingActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeba0d1e4_ecc6_4148_94ed_f4b37ec05b3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Enumeration")]
    pub DeviceInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    DeviceInformation: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDialReceiverActivatedEventArgs(::windows_core::IUnknown);
impl IDialReceiverActivatedEventArgs {
    pub fn AppName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TileId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IDialReceiverActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IDialReceiverActivatedEventArgs {}
impl ::windows_core::CanTryInto<ILaunchActivatedEventArgs> for IDialReceiverActivatedEventArgs {}
impl ::windows_core::RuntimeType for IDialReceiverActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{fb777ed7-85ee-456e-a44d-85d730e70aed}");
}
unsafe impl ::windows_core::Interface for IDialReceiverActivatedEventArgs {
    type Vtable = IDialReceiverActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDialReceiverActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb777ed7_85ee_456e_a44d_85d730e70aed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialReceiverActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AppName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileActivatedEventArgs(::windows_core::IUnknown);
impl IFileActivatedEventArgs {
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Files(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Files)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IFileActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IFileActivatedEventArgs {}
impl ::windows_core::RuntimeType for IFileActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{bb2afc33-93b1-42ed-8b26-236dd9c78496}");
}
unsafe impl ::windows_core::Interface for IFileActivatedEventArgs {
    type Vtable = IFileActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb2afc33_93b1_42ed_8b26_236dd9c78496);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub Files: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    Files: usize,
    pub Verb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileActivatedEventArgsWithCallerPackageFamilyName(::windows_core::IUnknown);
impl IFileActivatedEventArgsWithCallerPackageFamilyName {
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IFileActivatedEventArgsWithCallerPackageFamilyName, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IFileActivatedEventArgsWithCallerPackageFamilyName {}
impl ::windows_core::RuntimeType for IFileActivatedEventArgsWithCallerPackageFamilyName {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{2d60f06b-d25f-4d25-8653-e1c5e1108309}");
}
unsafe impl ::windows_core::Interface for IFileActivatedEventArgsWithCallerPackageFamilyName {
    type Vtable = IFileActivatedEventArgsWithCallerPackageFamilyName_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileActivatedEventArgsWithCallerPackageFamilyName {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2d60f06b_d25f_4d25_8653_e1c5e1108309);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgsWithCallerPackageFamilyName_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileActivatedEventArgsWithNeighboringFiles(::windows_core::IUnknown);
impl IFileActivatedEventArgsWithNeighboringFiles {
    #[doc = "Required features: `\"Storage_Search\"`"]
    #[cfg(feature = "Storage_Search")]
    pub fn NeighboringFilesQuery(&self) -> ::windows_core::Result<super::super::Storage::Search::StorageFileQueryResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NeighboringFilesQuery)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Files(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = &::windows_core::ComInterface::cast::<IFileActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Files)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IFileActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IFileActivatedEventArgsWithNeighboringFiles, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IFileActivatedEventArgsWithNeighboringFiles {}
impl ::windows_core::CanTryInto<IFileActivatedEventArgs> for IFileActivatedEventArgsWithNeighboringFiles {}
impl ::windows_core::RuntimeType for IFileActivatedEventArgsWithNeighboringFiles {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{433ba1a4-e1e2-48fd-b7fc-b5d6eee65033}");
}
unsafe impl ::windows_core::Interface for IFileActivatedEventArgsWithNeighboringFiles {
    type Vtable = IFileActivatedEventArgsWithNeighboringFiles_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileActivatedEventArgsWithNeighboringFiles {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x433ba1a4_e1e2_48fd_b7fc_b5d6eee65033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgsWithNeighboringFiles_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Search")]
    pub NeighboringFilesQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    NeighboringFilesQuery: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileOpenPickerActivatedEventArgs(::windows_core::IUnknown);
impl IFileOpenPickerActivatedEventArgs {
    #[doc = "Required features: `\"Storage_Pickers_Provider\"`"]
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileOpenPickerUI(&self) -> ::windows_core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FileOpenPickerUI)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IFileOpenPickerActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IFileOpenPickerActivatedEventArgs {}
impl ::windows_core::RuntimeType for IFileOpenPickerActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{72827082-5525-4bf2-bc09-1f5095d4964d}");
}
unsafe impl ::windows_core::Interface for IFileOpenPickerActivatedEventArgs {
    type Vtable = IFileOpenPickerActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileOpenPickerActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72827082_5525_4bf2_bc09_1f5095d4964d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub FileOpenPickerUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Pickers_Provider"))]
    FileOpenPickerUI: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileOpenPickerActivatedEventArgs2(::windows_core::IUnknown);
impl IFileOpenPickerActivatedEventArgs2 {
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IFileOpenPickerActivatedEventArgs2, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IFileOpenPickerActivatedEventArgs2 {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{5e731f66-8d1f-45fb-af1d-73205c8fc7a1}");
}
unsafe impl ::windows_core::Interface for IFileOpenPickerActivatedEventArgs2 {
    type Vtable = IFileOpenPickerActivatedEventArgs2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileOpenPickerActivatedEventArgs2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e731f66_8d1f_45fb_af1d_73205c8fc7a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerActivatedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileOpenPickerContinuationEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl IFileOpenPickerContinuationEventArgs {
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub fn Files(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Files)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::ComInterface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(IFileOpenPickerContinuationEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IFileOpenPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<IContinuationActivatedEventArgs> for IFileOpenPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for IFileOpenPickerContinuationEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{f0fa3f3a-d4e8-4ad3-9c34-2308f32fcec9}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IFileOpenPickerContinuationEventArgs {
    type Vtable = IFileOpenPickerContinuationEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IFileOpenPickerContinuationEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0fa3f3a_d4e8_4ad3_9c34_2308f32fcec9);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerContinuationEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub Files: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated")))]
    Files: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileSavePickerActivatedEventArgs(::windows_core::IUnknown);
impl IFileSavePickerActivatedEventArgs {
    #[doc = "Required features: `\"Storage_Pickers_Provider\"`"]
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileSavePickerUI(&self) -> ::windows_core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FileSavePickerUI)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IFileSavePickerActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IFileSavePickerActivatedEventArgs {}
impl ::windows_core::RuntimeType for IFileSavePickerActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{81c19cf1-74e6-4387-82eb-bb8fd64b4346}");
}
unsafe impl ::windows_core::Interface for IFileSavePickerActivatedEventArgs {
    type Vtable = IFileSavePickerActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileSavePickerActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81c19cf1_74e6_4387_82eb_bb8fd64b4346);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub FileSavePickerUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Pickers_Provider"))]
    FileSavePickerUI: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileSavePickerActivatedEventArgs2(::windows_core::IUnknown);
impl IFileSavePickerActivatedEventArgs2 {
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EnterpriseId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnterpriseId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IFileSavePickerActivatedEventArgs2, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IFileSavePickerActivatedEventArgs2 {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{6b73fe13-2cf2-4d48-8cbc-af67d23f1ce7}");
}
unsafe impl ::windows_core::Interface for IFileSavePickerActivatedEventArgs2 {
    type Vtable = IFileSavePickerActivatedEventArgs2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileSavePickerActivatedEventArgs2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b73fe13_2cf2_4d48_8cbc_af67d23f1ce7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerActivatedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileSavePickerContinuationEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl IFileSavePickerContinuationEventArgs {
    #[doc = "Required features: `\"Storage\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub fn File(&self) -> ::windows_core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::ComInterface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(IFileSavePickerContinuationEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IFileSavePickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<IContinuationActivatedEventArgs> for IFileSavePickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for IFileSavePickerContinuationEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{2c846fe1-3bad-4f33-8c8b-e46fae824b4b}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IFileSavePickerContinuationEventArgs {
    type Vtable = IFileSavePickerContinuationEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IFileSavePickerContinuationEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c846fe1_3bad_4f33_8c8b_e46fae824b4b);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerContinuationEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage", feature = "deprecated")))]
    File: usize,
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFolderPickerContinuationEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl IFolderPickerContinuationEventArgs {
    #[doc = "Required features: `\"Storage\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub fn Folder(&self) -> ::windows_core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Folder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::ComInterface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(IFolderPickerContinuationEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IFolderPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<IContinuationActivatedEventArgs> for IFolderPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for IFolderPickerContinuationEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{51882366-9f4b-498f-beb0-42684f6e1c29}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IFolderPickerContinuationEventArgs {
    type Vtable = IFolderPickerContinuationEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IFolderPickerContinuationEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x51882366_9f4b_498f_beb0_42684f6e1c29);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IFolderPickerContinuationEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub Folder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage", feature = "deprecated")))]
    Folder: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILaunchActivatedEventArgs(::windows_core::IUnknown);
impl ILaunchActivatedEventArgs {
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TileId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(ILaunchActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for ILaunchActivatedEventArgs {}
impl ::windows_core::RuntimeType for ILaunchActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{fbc93e26-a14a-4b4f-82b0-33bed920af52}");
}
unsafe impl ::windows_core::Interface for ILaunchActivatedEventArgs {
    type Vtable = ILaunchActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILaunchActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfbc93e26_a14a_4b4f_82b0_33bed920af52);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILaunchActivatedEventArgs2(::windows_core::IUnknown);
impl ILaunchActivatedEventArgs2 {
    pub fn TileActivatedInfo(&self) -> ::windows_core::Result<TileActivatedInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TileActivatedInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TileId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(ILaunchActivatedEventArgs2, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for ILaunchActivatedEventArgs2 {}
impl ::windows_core::CanTryInto<ILaunchActivatedEventArgs> for ILaunchActivatedEventArgs2 {}
impl ::windows_core::RuntimeType for ILaunchActivatedEventArgs2 {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{0fd37ebc-9dc9-46b5-9ace-bd95d4565345}");
}
unsafe impl ::windows_core::Interface for ILaunchActivatedEventArgs2 {
    type Vtable = ILaunchActivatedEventArgs2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILaunchActivatedEventArgs2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0fd37ebc_9dc9_46b5_9ace_bd95d4565345);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchActivatedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TileActivatedInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILockScreenActivatedEventArgs(::windows_core::IUnknown);
impl ILockScreenActivatedEventArgs {
    pub fn Info(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Info)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(ILockScreenActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for ILockScreenActivatedEventArgs {}
impl ::windows_core::RuntimeType for ILockScreenActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{3ca77966-6108-4a41-8220-ee7d133c8532}");
}
unsafe impl ::windows_core::Interface for ILockScreenActivatedEventArgs {
    type Vtable = ILockScreenActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILockScreenActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ca77966_6108_4a41_8220_ee7d133c8532);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Info: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILockScreenCallActivatedEventArgs(::windows_core::IUnknown);
impl ILockScreenCallActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Calls\"`"]
    #[cfg(feature = "ApplicationModel_Calls")]
    pub fn CallUI(&self) -> ::windows_core::Result<super::Calls::LockScreenCallUI> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallUI)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TileId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(ILockScreenCallActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for ILockScreenCallActivatedEventArgs {}
impl ::windows_core::CanTryInto<ILaunchActivatedEventArgs> for ILockScreenCallActivatedEventArgs {}
impl ::windows_core::RuntimeType for ILockScreenCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{06f37fbe-b5f2-448b-b13e-e328ac1c516a}");
}
unsafe impl ::windows_core::Interface for ILockScreenCallActivatedEventArgs {
    type Vtable = ILockScreenCallActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILockScreenCallActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x06f37fbe_b5f2_448b_b13e_e328ac1c516a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenCallActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Calls")]
    pub CallUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Calls"))]
    CallUI: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPhoneCallActivatedEventArgs(::windows_core::IUnknown);
impl IPhoneCallActivatedEventArgs {
    pub fn LineId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IPhoneCallActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IPhoneCallActivatedEventArgs {}
impl ::windows_core::RuntimeType for IPhoneCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{54615221-a3c1-4ced-b62f-8c60523619ad}");
}
unsafe impl ::windows_core::Interface for IPhoneCallActivatedEventArgs {
    type Vtable = IPhoneCallActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPhoneCallActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x54615221_a3c1_4ced_b62f_8c60523619ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LineId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPickerReturnedActivatedEventArgs(::windows_core::IUnknown);
impl IPickerReturnedActivatedEventArgs {
    pub fn PickerOperationId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PickerOperationId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IPickerReturnedActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IPickerReturnedActivatedEventArgs {}
impl ::windows_core::RuntimeType for IPickerReturnedActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{360defb9-a9d3-4984-a4ed-9ec734604921}");
}
unsafe impl ::windows_core::Interface for IPickerReturnedActivatedEventArgs {
    type Vtable = IPickerReturnedActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPickerReturnedActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x360defb9_a9d3_4984_a4ed_9ec734604921);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerReturnedActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PickerOperationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrelaunchActivatedEventArgs(::windows_core::IUnknown);
impl IPrelaunchActivatedEventArgs {
    pub fn PrelaunchActivated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrelaunchActivated)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IPrelaunchActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IPrelaunchActivatedEventArgs {}
impl ::windows_core::RuntimeType for IPrelaunchActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{0c44717b-19f7-48d6-b046-cf22826eaa74}");
}
unsafe impl ::windows_core::Interface for IPrelaunchActivatedEventArgs {
    type Vtable = IPrelaunchActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrelaunchActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c44717b_19f7_48d6_b046_cf22826eaa74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrelaunchActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PrelaunchActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrint3DWorkflowActivatedEventArgs(::windows_core::IUnknown);
impl IPrint3DWorkflowActivatedEventArgs {
    #[doc = "Required features: `\"Devices_Printers_Extensions\"`"]
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Workflow(&self) -> ::windows_core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Workflow)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IPrint3DWorkflowActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IPrint3DWorkflowActivatedEventArgs {}
impl ::windows_core::RuntimeType for IPrint3DWorkflowActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{3f57e78b-f2ac-4619-8302-ef855e1c9b90}");
}
unsafe impl ::windows_core::Interface for IPrint3DWorkflowActivatedEventArgs {
    type Vtable = IPrint3DWorkflowActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrint3DWorkflowActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3f57e78b_f2ac_4619_8302_ef855e1c9b90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DWorkflowActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub Workflow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Printers_Extensions"))]
    Workflow: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPrintTaskSettingsActivatedEventArgs(::windows_core::IUnknown);
impl IPrintTaskSettingsActivatedEventArgs {
    #[doc = "Required features: `\"Devices_Printers_Extensions\"`"]
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Configuration(&self) -> ::windows_core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IPrintTaskSettingsActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IPrintTaskSettingsActivatedEventArgs {}
impl ::windows_core::RuntimeType for IPrintTaskSettingsActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{ee30a0c9-ce56-4865-ba8e-8954ac271107}");
}
unsafe impl ::windows_core::Interface for IPrintTaskSettingsActivatedEventArgs {
    type Vtable = IPrintTaskSettingsActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPrintTaskSettingsActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee30a0c9_ce56_4865_ba8e_8954ac271107);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskSettingsActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Printers_Extensions"))]
    Configuration: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProtocolActivatedEventArgs(::windows_core::IUnknown);
impl IProtocolActivatedEventArgs {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IProtocolActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IProtocolActivatedEventArgs {}
impl ::windows_core::RuntimeType for IProtocolActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{6095f4dd-b7c0-46ab-81fe-d90f36d00d24}");
}
unsafe impl ::windows_core::Interface for IProtocolActivatedEventArgs {
    type Vtable = IProtocolActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProtocolActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6095f4dd_b7c0_46ab_81fe_d90f36d00d24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData(::windows_core::IUnknown);
impl IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Data(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {}
impl ::windows_core::RuntimeType for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{d84a0c12-5c8f-438c-83cb-c28fcc0b2fdb}");
}
unsafe impl ::windows_core::Interface for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Vtable = IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd84a0c12_5c8f_438c_83cb_c28fcc0b2fdb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CallerPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Data: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProtocolForResultsActivatedEventArgs(::windows_core::IUnknown);
impl IProtocolForResultsActivatedEventArgs {
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn ProtocolForResultsOperation(&self) -> ::windows_core::Result<super::super::System::ProtocolForResultsOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtocolForResultsOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IProtocolForResultsActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IProtocolForResultsActivatedEventArgs {}
impl ::windows_core::RuntimeType for IProtocolForResultsActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{e75132c2-7ae7-4517-80ac-dbe8d7cc5b9c}");
}
unsafe impl ::windows_core::Interface for IProtocolForResultsActivatedEventArgs {
    type Vtable = IProtocolForResultsActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProtocolForResultsActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe75132c2_7ae7_4517_80ac_dbe8d7cc5b9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolForResultsActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub ProtocolForResultsOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    ProtocolForResultsOperation: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRestrictedLaunchActivatedEventArgs(::windows_core::IUnknown);
impl IRestrictedLaunchActivatedEventArgs {
    pub fn SharedContext(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SharedContext)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IRestrictedLaunchActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IRestrictedLaunchActivatedEventArgs {}
impl ::windows_core::RuntimeType for IRestrictedLaunchActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{e0b7ac81-bfc3-4344-a5da-19fd5a27baae}");
}
unsafe impl ::windows_core::Interface for IRestrictedLaunchActivatedEventArgs {
    type Vtable = IRestrictedLaunchActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRestrictedLaunchActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe0b7ac81_bfc3_4344_a5da_19fd5a27baae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRestrictedLaunchActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SharedContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISearchActivatedEventArgs(::windows_core::IUnknown);
impl ISearchActivatedEventArgs {
    pub fn QueryText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QueryText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(ISearchActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for ISearchActivatedEventArgs {}
impl ::windows_core::RuntimeType for ISearchActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{8cb36951-58c8-43e3-94bc-41d33f8b630e}");
}
unsafe impl ::windows_core::Interface for ISearchActivatedEventArgs {
    type Vtable = ISearchActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISearchActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8cb36951_58c8_43e3_94bc_41d33f8b630e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub QueryText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISearchActivatedEventArgsWithLinguisticDetails(::windows_core::IUnknown);
impl ISearchActivatedEventArgsWithLinguisticDetails {
    #[doc = "Required features: `\"ApplicationModel_Search\"`"]
    #[cfg(feature = "ApplicationModel_Search")]
    pub fn LinguisticDetails(&self) -> ::windows_core::Result<super::Search::SearchPaneQueryLinguisticDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LinguisticDetails)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(ISearchActivatedEventArgsWithLinguisticDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for ISearchActivatedEventArgsWithLinguisticDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{c09f33da-08ab-4931-9b7c-451025f21f81}");
}
unsafe impl ::windows_core::Interface for ISearchActivatedEventArgsWithLinguisticDetails {
    type Vtable = ISearchActivatedEventArgsWithLinguisticDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISearchActivatedEventArgsWithLinguisticDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc09f33da_08ab_4931_9b7c_451025f21f81);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchActivatedEventArgsWithLinguisticDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Search")]
    pub LinguisticDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Search"))]
    LinguisticDetails: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IShareTargetActivatedEventArgs(::windows_core::IUnknown);
impl IShareTargetActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_DataTransfer_ShareTarget\"`"]
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    pub fn ShareOperation(&self) -> ::windows_core::Result<super::DataTransfer::ShareTarget::ShareOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShareOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IShareTargetActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IShareTargetActivatedEventArgs {}
impl ::windows_core::RuntimeType for IShareTargetActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{4bdaf9c8-cdb2-4acb-bfc3-6648563378ec}");
}
unsafe impl ::windows_core::Interface for IShareTargetActivatedEventArgs {
    type Vtable = IShareTargetActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IShareTargetActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4bdaf9c8_cdb2_4acb_bfc3_6648563378ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareTargetActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    pub ShareOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer_ShareTarget"))]
    ShareOperation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISplashScreen(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISplashScreen {
    type Vtable = ISplashScreen_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISplashScreen {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca4d975c_d4d6_43f0_97c0_0833c6391c24);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplashScreen_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ImageLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImageLocation: usize,
    #[cfg(feature = "Foundation")]
    pub Dismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Dismissed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDismissed: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStartupTaskActivatedEventArgs(::windows_core::IUnknown);
impl IStartupTaskActivatedEventArgs {
    pub fn TaskId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaskId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IStartupTaskActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IStartupTaskActivatedEventArgs {}
impl ::windows_core::RuntimeType for IStartupTaskActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{03b11a58-5276-4d91-8621-54611864d5fa}");
}
unsafe impl ::windows_core::Interface for IStartupTaskActivatedEventArgs {
    type Vtable = IStartupTaskActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStartupTaskActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x03b11a58_5276_4d91_8621_54611864d5fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartupTaskActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TaskId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITileActivatedInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITileActivatedInfo {
    type Vtable = ITileActivatedInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITileActivatedInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80e4a3b1_3980_4f17_b738_89194e0b8f65);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileActivatedInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Notifications"))]
    pub RecentlyShownNotifications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Notifications")))]
    RecentlyShownNotifications: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IToastNotificationActivatedEventArgs(::windows_core::IUnknown);
impl IToastNotificationActivatedEventArgs {
    pub fn Argument(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Argument)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserInput(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserInput)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IToastNotificationActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IToastNotificationActivatedEventArgs {}
impl ::windows_core::RuntimeType for IToastNotificationActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{92a86f82-5290-431d-be85-c4aaeeb8685f}");
}
unsafe impl ::windows_core::Interface for IToastNotificationActivatedEventArgs {
    type Vtable = IToastNotificationActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IToastNotificationActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92a86f82_5290_431d_be85_c4aaeeb8685f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Argument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub UserInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UserInput: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUserDataAccountProviderActivatedEventArgs(::windows_core::IUnknown);
impl IUserDataAccountProviderActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`"]
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    pub fn Operation(&self) -> ::windows_core::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IUserDataAccountProviderActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IUserDataAccountProviderActivatedEventArgs {}
impl ::windows_core::RuntimeType for IUserDataAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{1bc9f723-8ef1-4a51-a63a-fe711eeab607}");
}
unsafe impl ::windows_core::Interface for IUserDataAccountProviderActivatedEventArgs {
    type Vtable = IUserDataAccountProviderActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUserDataAccountProviderActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1bc9f723_8ef1_4a51_a63a_fe711eeab607);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_UserDataAccounts_Provider"))]
    Operation: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IViewSwitcherProvider(::windows_core::IUnknown);
impl IViewSwitcherProvider {
    #[doc = "Required features: `\"UI_ViewManagement\"`"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ViewSwitcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IViewSwitcherProvider, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IViewSwitcherProvider {}
impl ::windows_core::RuntimeType for IViewSwitcherProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{33f288a6-5c2c-4d27-bac7-7536088f1219}");
}
unsafe impl ::windows_core::Interface for IViewSwitcherProvider {
    type Vtable = IViewSwitcherProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IViewSwitcherProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33f288a6_5c2c_4d27_bac7_7536088f1219);
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewSwitcherProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_ViewManagement")]
    pub ViewSwitcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))]
    ViewSwitcher: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IVoiceCommandActivatedEventArgs(::windows_core::IUnknown);
impl IVoiceCommandActivatedEventArgs {
    #[doc = "Required features: `\"Media_SpeechRecognition\"`"]
    #[cfg(feature = "Media_SpeechRecognition")]
    pub fn Result(&self) -> ::windows_core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Result)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IVoiceCommandActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IVoiceCommandActivatedEventArgs {}
impl ::windows_core::RuntimeType for IVoiceCommandActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{ab92dcfd-8d43-4de6-9775-20704b581b00}");
}
unsafe impl ::windows_core::Interface for IVoiceCommandActivatedEventArgs {
    type Vtable = IVoiceCommandActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IVoiceCommandActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab92dcfd_8d43_4de6_9775_20704b581b00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_SpeechRecognition")]
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_SpeechRecognition"))]
    Result: usize,
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWalletActionActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl IWalletActionActivatedEventArgs {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn ItemId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ItemId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Wallet\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "ApplicationModel_Wallet", feature = "deprecated"))]
    pub fn ActionKind(&self) -> ::windows_core::Result<super::Wallet::WalletActionKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActionKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn ActionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(IWalletActionActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IWalletActionActivatedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for IWalletActionActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{fcfc027b-1a1a-4d22-923f-ae6f45fa52d9}");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IWalletActionActivatedEventArgs {
    type Vtable = IWalletActionActivatedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IWalletActionActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfcfc027b_1a1a_4d22_923f_ae6f45fa52d9);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IWalletActionActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub ItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ItemId: usize,
    #[cfg(all(feature = "ApplicationModel_Wallet", feature = "deprecated"))]
    pub ActionKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Wallet::WalletActionKind) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Wallet", feature = "deprecated")))]
    ActionKind: usize,
    #[cfg(feature = "deprecated")]
    pub ActionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    ActionId: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWebAccountProviderActivatedEventArgs(::windows_core::IUnknown);
impl IWebAccountProviderActivatedEventArgs {
    #[doc = "Required features: `\"Security_Authentication_Web_Provider\"`"]
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    pub fn Operation(&self) -> ::windows_core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IWebAccountProviderActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IWebAccountProviderActivatedEventArgs {}
impl ::windows_core::RuntimeType for IWebAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{72b71774-98ea-4ccf-9752-46d9051004f1}");
}
unsafe impl ::windows_core::Interface for IWebAccountProviderActivatedEventArgs {
    type Vtable = IWebAccountProviderActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWebAccountProviderActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72b71774_98ea_4ccf_9752_46d9051004f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Provider"))]
    Operation: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWebAuthenticationBrokerContinuationEventArgs(::windows_core::IUnknown);
impl IWebAuthenticationBrokerContinuationEventArgs {
    #[doc = "Required features: `\"Security_Authentication_Web\"`"]
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn WebAuthenticationResult(&self) -> ::windows_core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WebAuthenticationResult)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::ComInterface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IWebAuthenticationBrokerContinuationEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for IWebAuthenticationBrokerContinuationEventArgs {}
impl ::windows_core::CanTryInto<IContinuationActivatedEventArgs> for IWebAuthenticationBrokerContinuationEventArgs {}
impl ::windows_core::RuntimeType for IWebAuthenticationBrokerContinuationEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{75dda3d4-7714-453d-b7ff-b95e3a1709da}");
}
unsafe impl ::windows_core::Interface for IWebAuthenticationBrokerContinuationEventArgs {
    type Vtable = IWebAuthenticationBrokerContinuationEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWebAuthenticationBrokerContinuationEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75dda3d4_7714_453d_b7ff_b95e3a1709da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationBrokerContinuationEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Security_Authentication_Web")]
    pub WebAuthenticationResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))]
    WebAuthenticationResult: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppointmentsProviderAddAppointmentActivatedEventArgs(::windows_core::IUnknown);
impl AppointmentsProviderAddAppointmentActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`"]
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn AddAppointmentOperation(&self) -> ::windows_core::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddAppointmentOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppointmentsProviderAddAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderAddAppointmentActivatedEventArgs;{a2861367-cee5-4e4d-9ed7-41c34ec18b02})");
}
unsafe impl ::windows_core::Interface for AppointmentsProviderAddAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderAddAppointmentActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppointmentsProviderAddAppointmentActivatedEventArgs {
    const IID: ::windows_core::GUID = <IAppointmentsProviderAddAppointmentActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentsProviderAddAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderAddAppointmentActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppointmentsProviderAddAppointmentActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for AppointmentsProviderAddAppointmentActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for AppointmentsProviderAddAppointmentActivatedEventArgs {}
impl ::windows_core::CanTryInto<IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderAddAppointmentActivatedEventArgs {}
impl ::windows_core::CanTryInto<IAppointmentsProviderAddAppointmentActivatedEventArgs> for AppointmentsProviderAddAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Send for AppointmentsProviderAddAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderAddAppointmentActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppointmentsProviderRemoveAppointmentActivatedEventArgs(::windows_core::IUnknown);
impl AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`"]
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn RemoveAppointmentOperation(&self) -> ::windows_core::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveAppointmentOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderRemoveAppointmentActivatedEventArgs;{751f3ab8-0b8e-451c-9f15-966e699bac25})");
}
unsafe impl ::windows_core::Interface for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const IID: ::windows_core::GUID = <IAppointmentsProviderRemoveAppointmentActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderRemoveAppointmentActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppointmentsProviderRemoveAppointmentActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
impl ::windows_core::CanTryInto<IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
impl ::windows_core::CanTryInto<IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Send for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppointmentsProviderReplaceAppointmentActivatedEventArgs(::windows_core::IUnknown);
impl AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`"]
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    pub fn ReplaceAppointmentOperation(&self) -> ::windows_core::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceAppointmentOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderReplaceAppointmentActivatedEventArgs;{1551b7d4-a981-4067-8a62-0524e4ade121})");
}
unsafe impl ::windows_core::Interface for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const IID: ::windows_core::GUID = <IAppointmentsProviderReplaceAppointmentActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderReplaceAppointmentActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppointmentsProviderReplaceAppointmentActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
impl ::windows_core::CanTryInto<IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
impl ::windows_core::CanTryInto<IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Send for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppointmentsProviderShowAppointmentDetailsActivatedEventArgs(::windows_core::IUnknown);
impl AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn InstanceStartDate(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstanceStartDate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LocalId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LocalId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RoamingId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RoamingId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderShowAppointmentDetailsActivatedEventArgs;{3958f065-9841-4ca5-999b-885198b9ef2a})");
}
unsafe impl ::windows_core::Interface for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const IID: ::windows_core::GUID = <IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppointmentsProviderShowAppointmentDetailsActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
impl ::windows_core::CanTryInto<IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
impl ::windows_core::CanTryInto<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
unsafe impl ::core::marker::Send for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppointmentsProviderShowTimeFrameActivatedEventArgs(::windows_core::IUnknown);
impl AppointmentsProviderShowTimeFrameActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TimeToShow(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimeToShow)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderShowTimeFrameActivatedEventArgs;{9baeaba6-0e0b-49aa-babc-12b1dc774986})");
}
unsafe impl ::windows_core::Interface for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowTimeFrameActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    const IID: ::windows_core::GUID = <IAppointmentsProviderShowTimeFrameActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderShowTimeFrameActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppointmentsProviderShowTimeFrameActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
impl ::windows_core::CanTryInto<IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
impl ::windows_core::CanTryInto<IAppointmentsProviderShowTimeFrameActivatedEventArgs> for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
unsafe impl ::core::marker::Send for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BackgroundActivatedEventArgs(::windows_core::IUnknown);
impl BackgroundActivatedEventArgs {
    #[doc = "Required features: `\"ApplicationModel_Background\"`"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn TaskInstance(&self) -> ::windows_core::Result<super::Background::IBackgroundTaskInstance> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaskInstance)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for BackgroundActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.BackgroundActivatedEventArgs;{ab14bee0-e760-440e-a91c-44796de3a92d})");
}
unsafe impl ::windows_core::Interface for BackgroundActivatedEventArgs {
    type Vtable = IBackgroundActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BackgroundActivatedEventArgs {
    const IID: ::windows_core::GUID = <IBackgroundActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BackgroundActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.BackgroundActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(BackgroundActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundActivatedEventArgs> for BackgroundActivatedEventArgs {}
unsafe impl ::core::marker::Send for BackgroundActivatedEventArgs {}
unsafe impl ::core::marker::Sync for BackgroundActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BarcodeScannerPreviewActivatedEventArgs(::windows_core::IUnknown);
impl BarcodeScannerPreviewActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConnectionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConnectionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for BarcodeScannerPreviewActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.BarcodeScannerPreviewActivatedEventArgs;{6772797c-99bf-4349-af22-e4123560371c})");
}
unsafe impl ::windows_core::Interface for BarcodeScannerPreviewActivatedEventArgs {
    type Vtable = IBarcodeScannerPreviewActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BarcodeScannerPreviewActivatedEventArgs {
    const IID: ::windows_core::GUID = <IBarcodeScannerPreviewActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BarcodeScannerPreviewActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.BarcodeScannerPreviewActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(BarcodeScannerPreviewActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for BarcodeScannerPreviewActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for BarcodeScannerPreviewActivatedEventArgs {}
impl ::windows_core::CanTryInto<IBarcodeScannerPreviewActivatedEventArgs> for BarcodeScannerPreviewActivatedEventArgs {}
unsafe impl ::core::marker::Send for BarcodeScannerPreviewActivatedEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerPreviewActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CachedFileUpdaterActivatedEventArgs(::windows_core::IUnknown);
impl CachedFileUpdaterActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Provider\"`"]
    #[cfg(feature = "Storage_Provider")]
    pub fn CachedFileUpdaterUI(&self) -> ::windows_core::Result<super::super::Storage::Provider::CachedFileUpdaterUI> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CachedFileUpdaterUI)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CachedFileUpdaterActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CachedFileUpdaterActivatedEventArgs;{d06eb1c7-3805-4ecb-b757-6cf15e26fef3})");
}
unsafe impl ::windows_core::Interface for CachedFileUpdaterActivatedEventArgs {
    type Vtable = ICachedFileUpdaterActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CachedFileUpdaterActivatedEventArgs {
    const IID: ::windows_core::GUID = <ICachedFileUpdaterActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CachedFileUpdaterActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CachedFileUpdaterActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(CachedFileUpdaterActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for CachedFileUpdaterActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for CachedFileUpdaterActivatedEventArgs {}
impl ::windows_core::CanTryInto<ICachedFileUpdaterActivatedEventArgs> for CachedFileUpdaterActivatedEventArgs {}
unsafe impl ::core::marker::Send for CachedFileUpdaterActivatedEventArgs {}
unsafe impl ::core::marker::Sync for CachedFileUpdaterActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CameraSettingsActivatedEventArgs(::windows_core::IUnknown);
impl CameraSettingsActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VideoDeviceController(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceController)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn VideoDeviceExtension(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoDeviceExtension)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CameraSettingsActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CameraSettingsActivatedEventArgs;{fb67a508-2dad-490a-9170-dca036eb114b})");
}
unsafe impl ::windows_core::Interface for CameraSettingsActivatedEventArgs {
    type Vtable = ICameraSettingsActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CameraSettingsActivatedEventArgs {
    const IID: ::windows_core::GUID = <ICameraSettingsActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CameraSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CameraSettingsActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(CameraSettingsActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for CameraSettingsActivatedEventArgs {}
impl ::windows_core::CanTryInto<ICameraSettingsActivatedEventArgs> for CameraSettingsActivatedEventArgs {}
unsafe impl ::core::marker::Send for CameraSettingsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for CameraSettingsActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CommandLineActivatedEventArgs(::windows_core::IUnknown);
impl CommandLineActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Operation(&self) -> ::windows_core::Result<CommandLineActivationOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CommandLineActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CommandLineActivatedEventArgs;{4506472c-006a-48eb-8afb-d07ab25e3366})");
}
unsafe impl ::windows_core::Interface for CommandLineActivatedEventArgs {
    type Vtable = ICommandLineActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CommandLineActivatedEventArgs {
    const IID: ::windows_core::GUID = <ICommandLineActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CommandLineActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CommandLineActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(CommandLineActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for CommandLineActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for CommandLineActivatedEventArgs {}
impl ::windows_core::CanTryInto<ICommandLineActivatedEventArgs> for CommandLineActivatedEventArgs {}
unsafe impl ::core::marker::Send for CommandLineActivatedEventArgs {}
unsafe impl ::core::marker::Sync for CommandLineActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CommandLineActivationOperation(::windows_core::IUnknown);
impl CommandLineActivationOperation {
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentDirectoryPath(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentDirectoryPath)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetExitCode(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExitCode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExitCode(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExitCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CommandLineActivationOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CommandLineActivationOperation;{994b2841-c59e-4f69-bcfd-b61ed4e622eb})");
}
unsafe impl ::windows_core::Interface for CommandLineActivationOperation {
    type Vtable = ICommandLineActivationOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CommandLineActivationOperation {
    const IID: ::windows_core::GUID = <ICommandLineActivationOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CommandLineActivationOperation {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CommandLineActivationOperation";
}
::windows_core::imp::interface_hierarchy!(CommandLineActivationOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CommandLineActivationOperation {}
unsafe impl ::core::marker::Sync for CommandLineActivationOperation {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContactCallActivatedEventArgs(::windows_core::IUnknown);
impl ContactCallActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Contacts\"`"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ContactCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactCallActivatedEventArgs;{c2df14c7-30eb-41c6-b3bc-5b1694f9dab3})");
}
unsafe impl ::windows_core::Interface for ContactCallActivatedEventArgs {
    type Vtable = IContactCallActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactCallActivatedEventArgs {
    const IID: ::windows_core::GUID = <IContactCallActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactCallActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ContactCallActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for ContactCallActivatedEventArgs {}
impl ::windows_core::CanTryInto<IContactActivatedEventArgs> for ContactCallActivatedEventArgs {}
impl ::windows_core::CanTryInto<IContactCallActivatedEventArgs> for ContactCallActivatedEventArgs {}
unsafe impl ::core::marker::Send for ContactCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactCallActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContactMapActivatedEventArgs(::windows_core::IUnknown);
impl ContactMapActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Contacts\"`"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Address(&self) -> ::windows_core::Result<super::Contacts::ContactAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Address)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Contacts\"`"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ContactMapActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactMapActivatedEventArgs;{b32bf870-eee7-4ad2-aaf1-a87effcf00a4})");
}
unsafe impl ::windows_core::Interface for ContactMapActivatedEventArgs {
    type Vtable = IContactMapActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactMapActivatedEventArgs {
    const IID: ::windows_core::GUID = <IContactMapActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactMapActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactMapActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ContactMapActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for ContactMapActivatedEventArgs {}
impl ::windows_core::CanTryInto<IContactActivatedEventArgs> for ContactMapActivatedEventArgs {}
impl ::windows_core::CanTryInto<IContactMapActivatedEventArgs> for ContactMapActivatedEventArgs {}
unsafe impl ::core::marker::Send for ContactMapActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactMapActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContactMessageActivatedEventArgs(::windows_core::IUnknown);
impl ContactMessageActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Contacts\"`"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ContactMessageActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactMessageActivatedEventArgs;{de598db2-0e03-43b0-bf56-bcc40b3162df})");
}
unsafe impl ::windows_core::Interface for ContactMessageActivatedEventArgs {
    type Vtable = IContactMessageActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactMessageActivatedEventArgs {
    const IID: ::windows_core::GUID = <IContactMessageActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactMessageActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactMessageActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ContactMessageActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for ContactMessageActivatedEventArgs {}
impl ::windows_core::CanTryInto<IContactActivatedEventArgs> for ContactMessageActivatedEventArgs {}
impl ::windows_core::CanTryInto<IContactMessageActivatedEventArgs> for ContactMessageActivatedEventArgs {}
unsafe impl ::core::marker::Send for ContactMessageActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactMessageActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContactPanelActivatedEventArgs(::windows_core::IUnknown);
impl ContactPanelActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Contacts\"`"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn ContactPanel(&self) -> ::windows_core::Result<super::Contacts::ContactPanel> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContactPanel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Contacts\"`"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ContactPanelActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactPanelActivatedEventArgs;{52bb63e4-d3d4-4b63-8051-4af2082cab80})");
}
unsafe impl ::windows_core::Interface for ContactPanelActivatedEventArgs {
    type Vtable = IContactPanelActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactPanelActivatedEventArgs {
    const IID: ::windows_core::GUID = <IContactPanelActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactPanelActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPanelActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ContactPanelActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for ContactPanelActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for ContactPanelActivatedEventArgs {}
impl ::windows_core::CanTryInto<IContactPanelActivatedEventArgs> for ContactPanelActivatedEventArgs {}
unsafe impl ::core::marker::Send for ContactPanelActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactPanelActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContactPickerActivatedEventArgs(::windows_core::IUnknown);
impl ContactPickerActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Contacts_Provider\"`"]
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    pub fn ContactPickerUI(&self) -> ::windows_core::Result<super::Contacts::Provider::ContactPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContactPickerUI)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ContactPickerActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactPickerActivatedEventArgs;{ce57aae7-6449-45a7-971f-d113be7a8936})");
}
unsafe impl ::windows_core::Interface for ContactPickerActivatedEventArgs {
    type Vtable = IContactPickerActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactPickerActivatedEventArgs {
    const IID: ::windows_core::GUID = <IContactPickerActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPickerActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ContactPickerActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for ContactPickerActivatedEventArgs {}
impl ::windows_core::CanTryInto<IContactPickerActivatedEventArgs> for ContactPickerActivatedEventArgs {}
unsafe impl ::core::marker::Send for ContactPickerActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactPickerActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContactPostActivatedEventArgs(::windows_core::IUnknown);
impl ContactPostActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Contacts\"`"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ContactPostActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactPostActivatedEventArgs;{b35a3c67-f1e7-4655-ad6e-4857588f552f})");
}
unsafe impl ::windows_core::Interface for ContactPostActivatedEventArgs {
    type Vtable = IContactPostActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactPostActivatedEventArgs {
    const IID: ::windows_core::GUID = <IContactPostActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactPostActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPostActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ContactPostActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for ContactPostActivatedEventArgs {}
impl ::windows_core::CanTryInto<IContactActivatedEventArgs> for ContactPostActivatedEventArgs {}
impl ::windows_core::CanTryInto<IContactPostActivatedEventArgs> for ContactPostActivatedEventArgs {}
unsafe impl ::core::marker::Send for ContactPostActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactPostActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContactVideoCallActivatedEventArgs(::windows_core::IUnknown);
impl ContactVideoCallActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ServiceUserId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ServiceUserId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Contacts\"`"]
    #[cfg(feature = "ApplicationModel_Contacts")]
    pub fn Contact(&self) -> ::windows_core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Contact)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ContactVideoCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactVideoCallActivatedEventArgs;{61079db8-e3e7-4b4f-858d-5c63a96ef684})");
}
unsafe impl ::windows_core::Interface for ContactVideoCallActivatedEventArgs {
    type Vtable = IContactVideoCallActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactVideoCallActivatedEventArgs {
    const IID: ::windows_core::GUID = <IContactVideoCallActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactVideoCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactVideoCallActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ContactVideoCallActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for ContactVideoCallActivatedEventArgs {}
impl ::windows_core::CanTryInto<IContactActivatedEventArgs> for ContactVideoCallActivatedEventArgs {}
impl ::windows_core::CanTryInto<IContactVideoCallActivatedEventArgs> for ContactVideoCallActivatedEventArgs {}
unsafe impl ::core::marker::Send for ContactVideoCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactVideoCallActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DeviceActivatedEventArgs(::windows_core::IUnknown);
impl DeviceActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DeviceInformationId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInformationId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"UI_ViewManagement\"`"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows_core::ComInterface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ViewSwitcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for DeviceActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.DeviceActivatedEventArgs;{cd50b9a9-ce10-44d2-8234-c355a073ef33})");
}
unsafe impl ::windows_core::Interface for DeviceActivatedEventArgs {
    type Vtable = IDeviceActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DeviceActivatedEventArgs {
    const IID: ::windows_core::GUID = <IDeviceActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DeviceActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DeviceActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(DeviceActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for DeviceActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for DeviceActivatedEventArgs {}
impl ::windows_core::CanTryInto<IApplicationViewActivatedEventArgs> for DeviceActivatedEventArgs {}
impl ::windows_core::CanTryInto<IDeviceActivatedEventArgs> for DeviceActivatedEventArgs {}
impl ::windows_core::CanTryInto<IViewSwitcherProvider> for DeviceActivatedEventArgs {}
unsafe impl ::core::marker::Send for DeviceActivatedEventArgs {}
unsafe impl ::core::marker::Sync for DeviceActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DevicePairingActivatedEventArgs(::windows_core::IUnknown);
impl DevicePairingActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_Enumeration\"`"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn DeviceInformation(&self) -> ::windows_core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceInformation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for DevicePairingActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.DevicePairingActivatedEventArgs;{eba0d1e4-ecc6-4148-94ed-f4b37ec05b3e})");
}
unsafe impl ::windows_core::Interface for DevicePairingActivatedEventArgs {
    type Vtable = IDevicePairingActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DevicePairingActivatedEventArgs {
    const IID: ::windows_core::GUID = <IDevicePairingActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DevicePairingActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DevicePairingActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(DevicePairingActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for DevicePairingActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for DevicePairingActivatedEventArgs {}
impl ::windows_core::CanTryInto<IDevicePairingActivatedEventArgs> for DevicePairingActivatedEventArgs {}
unsafe impl ::core::marker::Send for DevicePairingActivatedEventArgs {}
unsafe impl ::core::marker::Sync for DevicePairingActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DialReceiverActivatedEventArgs(::windows_core::IUnknown);
impl DialReceiverActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TileId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"UI_ViewManagement\"`"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows_core::ComInterface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ViewSwitcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for DialReceiverActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.DialReceiverActivatedEventArgs;{fb777ed7-85ee-456e-a44d-85d730e70aed})");
}
unsafe impl ::windows_core::Interface for DialReceiverActivatedEventArgs {
    type Vtable = IDialReceiverActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DialReceiverActivatedEventArgs {
    const IID: ::windows_core::GUID = <IDialReceiverActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DialReceiverActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DialReceiverActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(DialReceiverActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for DialReceiverActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for DialReceiverActivatedEventArgs {}
impl ::windows_core::CanTryInto<IApplicationViewActivatedEventArgs> for DialReceiverActivatedEventArgs {}
impl ::windows_core::CanTryInto<IDialReceiverActivatedEventArgs> for DialReceiverActivatedEventArgs {}
impl ::windows_core::CanTryInto<ILaunchActivatedEventArgs> for DialReceiverActivatedEventArgs {}
impl ::windows_core::CanTryInto<IViewSwitcherProvider> for DialReceiverActivatedEventArgs {}
unsafe impl ::core::marker::Send for DialReceiverActivatedEventArgs {}
unsafe impl ::core::marker::Sync for DialReceiverActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FileActivatedEventArgs(::windows_core::IUnknown);
impl FileActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Files(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Files)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Verb(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Verb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IFileActivatedEventArgsWithCallerPackageFamilyName>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Search\"`"]
    #[cfg(feature = "Storage_Search")]
    pub fn NeighboringFilesQuery(&self) -> ::windows_core::Result<super::super::Storage::Search::StorageFileQueryResult> {
        let this = &::windows_core::ComInterface::cast::<IFileActivatedEventArgsWithNeighboringFiles>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NeighboringFilesQuery)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"UI_ViewManagement\"`"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows_core::ComInterface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ViewSwitcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for FileActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileActivatedEventArgs;{bb2afc33-93b1-42ed-8b26-236dd9c78496})");
}
unsafe impl ::windows_core::Interface for FileActivatedEventArgs {
    type Vtable = IFileActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FileActivatedEventArgs {
    const IID: ::windows_core::GUID = <IFileActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FileActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(FileActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for FileActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for FileActivatedEventArgs {}
impl ::windows_core::CanTryInto<IApplicationViewActivatedEventArgs> for FileActivatedEventArgs {}
impl ::windows_core::CanTryInto<IFileActivatedEventArgs> for FileActivatedEventArgs {}
impl ::windows_core::CanTryInto<IFileActivatedEventArgsWithCallerPackageFamilyName> for FileActivatedEventArgs {}
impl ::windows_core::CanTryInto<IFileActivatedEventArgsWithNeighboringFiles> for FileActivatedEventArgs {}
impl ::windows_core::CanTryInto<IViewSwitcherProvider> for FileActivatedEventArgs {}
unsafe impl ::core::marker::Send for FileActivatedEventArgs {}
unsafe impl ::core::marker::Sync for FileActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FileOpenPickerActivatedEventArgs(::windows_core::IUnknown);
impl FileOpenPickerActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Pickers_Provider\"`"]
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileOpenPickerUI(&self) -> ::windows_core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FileOpenPickerUI)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IFileOpenPickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for FileOpenPickerActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileOpenPickerActivatedEventArgs;{72827082-5525-4bf2-bc09-1f5095d4964d})");
}
unsafe impl ::windows_core::Interface for FileOpenPickerActivatedEventArgs {
    type Vtable = IFileOpenPickerActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FileOpenPickerActivatedEventArgs {
    const IID: ::windows_core::GUID = <IFileOpenPickerActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FileOpenPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileOpenPickerActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(FileOpenPickerActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for FileOpenPickerActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for FileOpenPickerActivatedEventArgs {}
impl ::windows_core::CanTryInto<IFileOpenPickerActivatedEventArgs> for FileOpenPickerActivatedEventArgs {}
impl ::windows_core::CanTryInto<IFileOpenPickerActivatedEventArgs2> for FileOpenPickerActivatedEventArgs {}
unsafe impl ::core::marker::Send for FileOpenPickerActivatedEventArgs {}
unsafe impl ::core::marker::Sync for FileOpenPickerActivatedEventArgs {}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FileOpenPickerContinuationEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl FileOpenPickerContinuationEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::ComInterface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub fn Files(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Files)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for FileOpenPickerContinuationEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileOpenPickerContinuationEventArgs;{f0fa3f3a-d4e8-4ad3-9c34-2308f32fcec9})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for FileOpenPickerContinuationEventArgs {
    type Vtable = IFileOpenPickerContinuationEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for FileOpenPickerContinuationEventArgs {
    const IID: ::windows_core::GUID = <IFileOpenPickerContinuationEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for FileOpenPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileOpenPickerContinuationEventArgs";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(FileOpenPickerContinuationEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<IActivatedEventArgs> for FileOpenPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for FileOpenPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<IContinuationActivatedEventArgs> for FileOpenPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<IFileOpenPickerContinuationEventArgs> for FileOpenPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for FileOpenPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for FileOpenPickerContinuationEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FileSavePickerActivatedEventArgs(::windows_core::IUnknown);
impl FileSavePickerActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Pickers_Provider\"`"]
    #[cfg(feature = "Storage_Pickers_Provider")]
    pub fn FileSavePickerUI(&self) -> ::windows_core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FileSavePickerUI)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn EnterpriseId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnterpriseId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for FileSavePickerActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileSavePickerActivatedEventArgs;{81c19cf1-74e6-4387-82eb-bb8fd64b4346})");
}
unsafe impl ::windows_core::Interface for FileSavePickerActivatedEventArgs {
    type Vtable = IFileSavePickerActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FileSavePickerActivatedEventArgs {
    const IID: ::windows_core::GUID = <IFileSavePickerActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FileSavePickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileSavePickerActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(FileSavePickerActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for FileSavePickerActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for FileSavePickerActivatedEventArgs {}
impl ::windows_core::CanTryInto<IFileSavePickerActivatedEventArgs> for FileSavePickerActivatedEventArgs {}
impl ::windows_core::CanTryInto<IFileSavePickerActivatedEventArgs2> for FileSavePickerActivatedEventArgs {}
unsafe impl ::core::marker::Send for FileSavePickerActivatedEventArgs {}
unsafe impl ::core::marker::Sync for FileSavePickerActivatedEventArgs {}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FileSavePickerContinuationEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl FileSavePickerContinuationEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::ComInterface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub fn File(&self) -> ::windows_core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for FileSavePickerContinuationEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileSavePickerContinuationEventArgs;{2c846fe1-3bad-4f33-8c8b-e46fae824b4b})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for FileSavePickerContinuationEventArgs {
    type Vtable = IFileSavePickerContinuationEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for FileSavePickerContinuationEventArgs {
    const IID: ::windows_core::GUID = <IFileSavePickerContinuationEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for FileSavePickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileSavePickerContinuationEventArgs";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(FileSavePickerContinuationEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<IActivatedEventArgs> for FileSavePickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for FileSavePickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<IContinuationActivatedEventArgs> for FileSavePickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<IFileSavePickerContinuationEventArgs> for FileSavePickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for FileSavePickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for FileSavePickerContinuationEventArgs {}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FolderPickerContinuationEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl FolderPickerContinuationEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::ComInterface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Storage", feature = "deprecated"))]
    pub fn Folder(&self) -> ::windows_core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Folder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for FolderPickerContinuationEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FolderPickerContinuationEventArgs;{51882366-9f4b-498f-beb0-42684f6e1c29})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for FolderPickerContinuationEventArgs {
    type Vtable = IFolderPickerContinuationEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for FolderPickerContinuationEventArgs {
    const IID: ::windows_core::GUID = <IFolderPickerContinuationEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for FolderPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FolderPickerContinuationEventArgs";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(FolderPickerContinuationEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<IActivatedEventArgs> for FolderPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for FolderPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<IContinuationActivatedEventArgs> for FolderPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<IFolderPickerContinuationEventArgs> for FolderPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for FolderPickerContinuationEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for FolderPickerContinuationEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LaunchActivatedEventArgs(::windows_core::IUnknown);
impl LaunchActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TileId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TileActivatedInfo(&self) -> ::windows_core::Result<TileActivatedInfo> {
        let this = &::windows_core::ComInterface::cast::<ILaunchActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TileActivatedInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PrelaunchActivated(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IPrelaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrelaunchActivated)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"UI_ViewManagement\"`"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows_core::ComInterface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ViewSwitcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for LaunchActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LaunchActivatedEventArgs;{fbc93e26-a14a-4b4f-82b0-33bed920af52})");
}
unsafe impl ::windows_core::Interface for LaunchActivatedEventArgs {
    type Vtable = ILaunchActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LaunchActivatedEventArgs {
    const IID: ::windows_core::GUID = <ILaunchActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LaunchActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(LaunchActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for LaunchActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for LaunchActivatedEventArgs {}
impl ::windows_core::CanTryInto<IApplicationViewActivatedEventArgs> for LaunchActivatedEventArgs {}
impl ::windows_core::CanTryInto<ILaunchActivatedEventArgs> for LaunchActivatedEventArgs {}
impl ::windows_core::CanTryInto<ILaunchActivatedEventArgs2> for LaunchActivatedEventArgs {}
impl ::windows_core::CanTryInto<IPrelaunchActivatedEventArgs> for LaunchActivatedEventArgs {}
impl ::windows_core::CanTryInto<IViewSwitcherProvider> for LaunchActivatedEventArgs {}
unsafe impl ::core::marker::Send for LaunchActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LaunchActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LockScreenActivatedEventArgs(::windows_core::IUnknown);
impl LockScreenActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Info(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Info)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for LockScreenActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LockScreenActivatedEventArgs;{3ca77966-6108-4a41-8220-ee7d133c8532})");
}
unsafe impl ::windows_core::Interface for LockScreenActivatedEventArgs {
    type Vtable = ILockScreenActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LockScreenActivatedEventArgs {
    const IID: ::windows_core::GUID = <ILockScreenActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LockScreenActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(LockScreenActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for LockScreenActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for LockScreenActivatedEventArgs {}
impl ::windows_core::CanTryInto<ILockScreenActivatedEventArgs> for LockScreenActivatedEventArgs {}
unsafe impl ::core::marker::Send for LockScreenActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LockScreenCallActivatedEventArgs(::windows_core::IUnknown);
impl LockScreenCallActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Arguments(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TileId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TileId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Calls\"`"]
    #[cfg(feature = "ApplicationModel_Calls")]
    pub fn CallUI(&self) -> ::windows_core::Result<super::Calls::LockScreenCallUI> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallUI)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"UI_ViewManagement\"`"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows_core::ComInterface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ViewSwitcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for LockScreenCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LockScreenCallActivatedEventArgs;{06f37fbe-b5f2-448b-b13e-e328ac1c516a})");
}
unsafe impl ::windows_core::Interface for LockScreenCallActivatedEventArgs {
    type Vtable = ILockScreenCallActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LockScreenCallActivatedEventArgs {
    const IID: ::windows_core::GUID = <ILockScreenCallActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LockScreenCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenCallActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(LockScreenCallActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for LockScreenCallActivatedEventArgs {}
impl ::windows_core::CanTryInto<IApplicationViewActivatedEventArgs> for LockScreenCallActivatedEventArgs {}
impl ::windows_core::CanTryInto<ILaunchActivatedEventArgs> for LockScreenCallActivatedEventArgs {}
impl ::windows_core::CanTryInto<ILockScreenCallActivatedEventArgs> for LockScreenCallActivatedEventArgs {}
impl ::windows_core::CanTryInto<IViewSwitcherProvider> for LockScreenCallActivatedEventArgs {}
unsafe impl ::core::marker::Send for LockScreenCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenCallActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LockScreenComponentActivatedEventArgs(::windows_core::IUnknown);
impl LockScreenComponentActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for LockScreenComponentActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LockScreenComponentActivatedEventArgs;{cf651713-cd08-4fd8-b697-a281b6544e2e})");
}
unsafe impl ::windows_core::Interface for LockScreenComponentActivatedEventArgs {
    type Vtable = IActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LockScreenComponentActivatedEventArgs {
    const IID: ::windows_core::GUID = <IActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LockScreenComponentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenComponentActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(LockScreenComponentActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for LockScreenComponentActivatedEventArgs {}
unsafe impl ::core::marker::Send for LockScreenComponentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenComponentActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PhoneCallActivatedEventArgs(::windows_core::IUnknown);
impl PhoneCallActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LineId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PhoneCallActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.PhoneCallActivatedEventArgs;{54615221-a3c1-4ced-b62f-8c60523619ad})");
}
unsafe impl ::windows_core::Interface for PhoneCallActivatedEventArgs {
    type Vtable = IPhoneCallActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneCallActivatedEventArgs {
    const IID: ::windows_core::GUID = <IPhoneCallActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PhoneCallActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PhoneCallActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for PhoneCallActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for PhoneCallActivatedEventArgs {}
impl ::windows_core::CanTryInto<IPhoneCallActivatedEventArgs> for PhoneCallActivatedEventArgs {}
unsafe impl ::core::marker::Send for PhoneCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PhoneCallActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PickerReturnedActivatedEventArgs(::windows_core::IUnknown);
impl PickerReturnedActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PickerOperationId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PickerOperationId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PickerReturnedActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.PickerReturnedActivatedEventArgs;{360defb9-a9d3-4984-a4ed-9ec734604921})");
}
unsafe impl ::windows_core::Interface for PickerReturnedActivatedEventArgs {
    type Vtable = IPickerReturnedActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PickerReturnedActivatedEventArgs {
    const IID: ::windows_core::GUID = <IPickerReturnedActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PickerReturnedActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PickerReturnedActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PickerReturnedActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for PickerReturnedActivatedEventArgs {}
impl ::windows_core::CanTryInto<IPickerReturnedActivatedEventArgs> for PickerReturnedActivatedEventArgs {}
unsafe impl ::core::marker::Send for PickerReturnedActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PickerReturnedActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Print3DWorkflowActivatedEventArgs(::windows_core::IUnknown);
impl Print3DWorkflowActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_Printers_Extensions\"`"]
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Workflow(&self) -> ::windows_core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Workflow)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for Print3DWorkflowActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.Print3DWorkflowActivatedEventArgs;{3f57e78b-f2ac-4619-8302-ef855e1c9b90})");
}
unsafe impl ::windows_core::Interface for Print3DWorkflowActivatedEventArgs {
    type Vtable = IPrint3DWorkflowActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Print3DWorkflowActivatedEventArgs {
    const IID: ::windows_core::GUID = <IPrint3DWorkflowActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Print3DWorkflowActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.Print3DWorkflowActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(Print3DWorkflowActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for Print3DWorkflowActivatedEventArgs {}
impl ::windows_core::CanTryInto<IPrint3DWorkflowActivatedEventArgs> for Print3DWorkflowActivatedEventArgs {}
unsafe impl ::core::marker::Send for Print3DWorkflowActivatedEventArgs {}
unsafe impl ::core::marker::Sync for Print3DWorkflowActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PrintTaskSettingsActivatedEventArgs(::windows_core::IUnknown);
impl PrintTaskSettingsActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_Printers_Extensions\"`"]
    #[cfg(feature = "Devices_Printers_Extensions")]
    pub fn Configuration(&self) -> ::windows_core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for PrintTaskSettingsActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.PrintTaskSettingsActivatedEventArgs;{ee30a0c9-ce56-4865-ba8e-8954ac271107})");
}
unsafe impl ::windows_core::Interface for PrintTaskSettingsActivatedEventArgs {
    type Vtable = IPrintTaskSettingsActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintTaskSettingsActivatedEventArgs {
    const IID: ::windows_core::GUID = <IPrintTaskSettingsActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintTaskSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PrintTaskSettingsActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PrintTaskSettingsActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for PrintTaskSettingsActivatedEventArgs {}
impl ::windows_core::CanTryInto<IPrintTaskSettingsActivatedEventArgs> for PrintTaskSettingsActivatedEventArgs {}
unsafe impl ::core::marker::Send for PrintTaskSettingsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PrintTaskSettingsActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ProtocolActivatedEventArgs(::windows_core::IUnknown);
impl ProtocolActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Data(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::ComInterface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"UI_ViewManagement\"`"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows_core::ComInterface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ViewSwitcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ProtocolActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ProtocolActivatedEventArgs;{6095f4dd-b7c0-46ab-81fe-d90f36d00d24})");
}
unsafe impl ::windows_core::Interface for ProtocolActivatedEventArgs {
    type Vtable = IProtocolActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProtocolActivatedEventArgs {
    const IID: ::windows_core::GUID = <IProtocolActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProtocolActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ProtocolActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ProtocolActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for ProtocolActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for ProtocolActivatedEventArgs {}
impl ::windows_core::CanTryInto<IApplicationViewActivatedEventArgs> for ProtocolActivatedEventArgs {}
impl ::windows_core::CanTryInto<IProtocolActivatedEventArgs> for ProtocolActivatedEventArgs {}
impl ::windows_core::CanTryInto<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ProtocolActivatedEventArgs {}
impl ::windows_core::CanTryInto<IViewSwitcherProvider> for ProtocolActivatedEventArgs {}
unsafe impl ::core::marker::Send for ProtocolActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ProtocolActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ProtocolForResultsActivatedEventArgs(::windows_core::IUnknown);
impl ProtocolForResultsActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<IProtocolActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CallerPackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CallerPackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Data(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::ComInterface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn ProtocolForResultsOperation(&self) -> ::windows_core::Result<super::super::System::ProtocolForResultsOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtocolForResultsOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"UI_ViewManagement\"`"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows_core::ComInterface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ViewSwitcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ProtocolForResultsActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ProtocolForResultsActivatedEventArgs;{e75132c2-7ae7-4517-80ac-dbe8d7cc5b9c})");
}
unsafe impl ::windows_core::Interface for ProtocolForResultsActivatedEventArgs {
    type Vtable = IProtocolForResultsActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProtocolForResultsActivatedEventArgs {
    const IID: ::windows_core::GUID = <IProtocolForResultsActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProtocolForResultsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ProtocolForResultsActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ProtocolForResultsActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for ProtocolForResultsActivatedEventArgs {}
impl ::windows_core::CanTryInto<IApplicationViewActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {}
impl ::windows_core::CanTryInto<IProtocolActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {}
impl ::windows_core::CanTryInto<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ProtocolForResultsActivatedEventArgs {}
impl ::windows_core::CanTryInto<IProtocolForResultsActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {}
impl ::windows_core::CanTryInto<IViewSwitcherProvider> for ProtocolForResultsActivatedEventArgs {}
unsafe impl ::core::marker::Send for ProtocolForResultsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ProtocolForResultsActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RestrictedLaunchActivatedEventArgs(::windows_core::IUnknown);
impl RestrictedLaunchActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SharedContext(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SharedContext)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for RestrictedLaunchActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.RestrictedLaunchActivatedEventArgs;{e0b7ac81-bfc3-4344-a5da-19fd5a27baae})");
}
unsafe impl ::windows_core::Interface for RestrictedLaunchActivatedEventArgs {
    type Vtable = IRestrictedLaunchActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RestrictedLaunchActivatedEventArgs {
    const IID: ::windows_core::GUID = <IRestrictedLaunchActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RestrictedLaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.RestrictedLaunchActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(RestrictedLaunchActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for RestrictedLaunchActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for RestrictedLaunchActivatedEventArgs {}
impl ::windows_core::CanTryInto<IRestrictedLaunchActivatedEventArgs> for RestrictedLaunchActivatedEventArgs {}
unsafe impl ::core::marker::Send for RestrictedLaunchActivatedEventArgs {}
unsafe impl ::core::marker::Sync for RestrictedLaunchActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SearchActivatedEventArgs(::windows_core::IUnknown);
impl SearchActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn QueryText(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QueryText)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Search\"`"]
    #[cfg(feature = "ApplicationModel_Search")]
    pub fn LinguisticDetails(&self) -> ::windows_core::Result<super::Search::SearchPaneQueryLinguisticDetails> {
        let this = &::windows_core::ComInterface::cast::<ISearchActivatedEventArgsWithLinguisticDetails>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LinguisticDetails)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"UI_ViewManagement\"`"]
    #[cfg(feature = "UI_ViewManagement")]
    pub fn ViewSwitcher(&self) -> ::windows_core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows_core::ComInterface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ViewSwitcher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for SearchActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.SearchActivatedEventArgs;{8cb36951-58c8-43e3-94bc-41d33f8b630e})");
}
unsafe impl ::windows_core::Interface for SearchActivatedEventArgs {
    type Vtable = ISearchActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SearchActivatedEventArgs {
    const IID: ::windows_core::GUID = <ISearchActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SearchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.SearchActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(SearchActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for SearchActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for SearchActivatedEventArgs {}
impl ::windows_core::CanTryInto<IApplicationViewActivatedEventArgs> for SearchActivatedEventArgs {}
impl ::windows_core::CanTryInto<ISearchActivatedEventArgs> for SearchActivatedEventArgs {}
impl ::windows_core::CanTryInto<ISearchActivatedEventArgsWithLinguisticDetails> for SearchActivatedEventArgs {}
impl ::windows_core::CanTryInto<IViewSwitcherProvider> for SearchActivatedEventArgs {}
unsafe impl ::core::marker::Send for SearchActivatedEventArgs {}
unsafe impl ::core::marker::Sync for SearchActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ShareTargetActivatedEventArgs(::windows_core::IUnknown);
impl ShareTargetActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_DataTransfer_ShareTarget\"`"]
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    pub fn ShareOperation(&self) -> ::windows_core::Result<super::DataTransfer::ShareTarget::ShareOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShareOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ShareTargetActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ShareTargetActivatedEventArgs;{4bdaf9c8-cdb2-4acb-bfc3-6648563378ec})");
}
unsafe impl ::windows_core::Interface for ShareTargetActivatedEventArgs {
    type Vtable = IShareTargetActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ShareTargetActivatedEventArgs {
    const IID: ::windows_core::GUID = <IShareTargetActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ShareTargetActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ShareTargetActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ShareTargetActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for ShareTargetActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for ShareTargetActivatedEventArgs {}
impl ::windows_core::CanTryInto<IShareTargetActivatedEventArgs> for ShareTargetActivatedEventArgs {}
unsafe impl ::core::marker::Send for ShareTargetActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ShareTargetActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SplashScreen(::windows_core::IUnknown);
impl SplashScreen {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ImageLocation(&self) -> ::windows_core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImageLocation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Dismissed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SplashScreen, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Dismissed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDismissed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDismissed)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
}
impl ::windows_core::RuntimeType for SplashScreen {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.SplashScreen;{ca4d975c-d4d6-43f0-97c0-0833c6391c24})");
}
unsafe impl ::windows_core::Interface for SplashScreen {
    type Vtable = ISplashScreen_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SplashScreen {
    const IID: ::windows_core::GUID = <ISplashScreen as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SplashScreen {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.SplashScreen";
}
::windows_core::imp::interface_hierarchy!(SplashScreen, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct StartupTaskActivatedEventArgs(::windows_core::IUnknown);
impl StartupTaskActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TaskId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaskId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for StartupTaskActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.StartupTaskActivatedEventArgs;{03b11a58-5276-4d91-8621-54611864d5fa})");
}
unsafe impl ::windows_core::Interface for StartupTaskActivatedEventArgs {
    type Vtable = IStartupTaskActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for StartupTaskActivatedEventArgs {
    const IID: ::windows_core::GUID = <IStartupTaskActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for StartupTaskActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.StartupTaskActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(StartupTaskActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for StartupTaskActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for StartupTaskActivatedEventArgs {}
impl ::windows_core::CanTryInto<IStartupTaskActivatedEventArgs> for StartupTaskActivatedEventArgs {}
unsafe impl ::core::marker::Send for StartupTaskActivatedEventArgs {}
unsafe impl ::core::marker::Sync for StartupTaskActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TileActivatedInfo(::windows_core::IUnknown);
impl TileActivatedInfo {
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"UI_Notifications\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Notifications"))]
    pub fn RecentlyShownNotifications(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::UI::Notifications::ShownTileNotification>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RecentlyShownNotifications)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for TileActivatedInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.TileActivatedInfo;{80e4a3b1-3980-4f17-b738-89194e0b8f65})");
}
unsafe impl ::windows_core::Interface for TileActivatedInfo {
    type Vtable = ITileActivatedInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TileActivatedInfo {
    const IID: ::windows_core::GUID = <ITileActivatedInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TileActivatedInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.TileActivatedInfo";
}
::windows_core::imp::interface_hierarchy!(TileActivatedInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TileActivatedInfo {}
unsafe impl ::core::marker::Sync for TileActivatedInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ToastNotificationActivatedEventArgs(::windows_core::IUnknown);
impl ToastNotificationActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Argument(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Argument)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UserInput(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UserInput)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ToastNotificationActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ToastNotificationActivatedEventArgs;{92a86f82-5290-431d-be85-c4aaeeb8685f})");
}
unsafe impl ::windows_core::Interface for ToastNotificationActivatedEventArgs {
    type Vtable = IToastNotificationActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ToastNotificationActivatedEventArgs {
    const IID: ::windows_core::GUID = <IToastNotificationActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ToastNotificationActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ToastNotificationActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ToastNotificationActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for ToastNotificationActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for ToastNotificationActivatedEventArgs {}
impl ::windows_core::CanTryInto<IApplicationViewActivatedEventArgs> for ToastNotificationActivatedEventArgs {}
impl ::windows_core::CanTryInto<IToastNotificationActivatedEventArgs> for ToastNotificationActivatedEventArgs {}
unsafe impl ::core::marker::Send for ToastNotificationActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ToastNotificationActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UserDataAccountProviderActivatedEventArgs(::windows_core::IUnknown);
impl UserDataAccountProviderActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_UserDataAccounts_Provider\"`"]
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    pub fn Operation(&self) -> ::windows_core::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for UserDataAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.UserDataAccountProviderActivatedEventArgs;{1bc9f723-8ef1-4a51-a63a-fe711eeab607})");
}
unsafe impl ::windows_core::Interface for UserDataAccountProviderActivatedEventArgs {
    type Vtable = IUserDataAccountProviderActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UserDataAccountProviderActivatedEventArgs {
    const IID: ::windows_core::GUID = <IUserDataAccountProviderActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UserDataAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.UserDataAccountProviderActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(UserDataAccountProviderActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for UserDataAccountProviderActivatedEventArgs {}
impl ::windows_core::CanTryInto<IUserDataAccountProviderActivatedEventArgs> for UserDataAccountProviderActivatedEventArgs {}
unsafe impl ::core::marker::Send for UserDataAccountProviderActivatedEventArgs {}
unsafe impl ::core::marker::Sync for UserDataAccountProviderActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct VoiceCommandActivatedEventArgs(::windows_core::IUnknown);
impl VoiceCommandActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Media_SpeechRecognition\"`"]
    #[cfg(feature = "Media_SpeechRecognition")]
    pub fn Result(&self) -> ::windows_core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Result)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for VoiceCommandActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.VoiceCommandActivatedEventArgs;{ab92dcfd-8d43-4de6-9775-20704b581b00})");
}
unsafe impl ::windows_core::Interface for VoiceCommandActivatedEventArgs {
    type Vtable = IVoiceCommandActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VoiceCommandActivatedEventArgs {
    const IID: ::windows_core::GUID = <IVoiceCommandActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VoiceCommandActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.VoiceCommandActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(VoiceCommandActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for VoiceCommandActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for VoiceCommandActivatedEventArgs {}
impl ::windows_core::CanTryInto<IVoiceCommandActivatedEventArgs> for VoiceCommandActivatedEventArgs {}
unsafe impl ::core::marker::Send for VoiceCommandActivatedEventArgs {}
unsafe impl ::core::marker::Sync for VoiceCommandActivatedEventArgs {}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WalletActionActivatedEventArgs(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl WalletActionActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn ItemId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ItemId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Wallet\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "ApplicationModel_Wallet", feature = "deprecated"))]
    pub fn ActionKind(&self) -> ::windows_core::Result<super::Wallet::WalletActionKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActionKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn ActionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for WalletActionActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.WalletActionActivatedEventArgs;{fcfc027b-1a1a-4d22-923f-ae6f45fa52d9})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for WalletActionActivatedEventArgs {
    type Vtable = IWalletActionActivatedEventArgs_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for WalletActionActivatedEventArgs {
    const IID: ::windows_core::GUID = <IWalletActionActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for WalletActionActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WalletActionActivatedEventArgs";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(WalletActionActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<IActivatedEventArgs> for WalletActionActivatedEventArgs {}
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<IWalletActionActivatedEventArgs> for WalletActionActivatedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for WalletActionActivatedEventArgs {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for WalletActionActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebAccountProviderActivatedEventArgs(::windows_core::IUnknown);
impl WebAccountProviderActivatedEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Security_Authentication_Web_Provider\"`"]
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    pub fn Operation(&self) -> ::windows_core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WebAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.WebAccountProviderActivatedEventArgs;{72b71774-98ea-4ccf-9752-46d9051004f1})");
}
unsafe impl ::windows_core::Interface for WebAccountProviderActivatedEventArgs {
    type Vtable = IWebAccountProviderActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebAccountProviderActivatedEventArgs {
    const IID: ::windows_core::GUID = <IWebAccountProviderActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WebAccountProviderActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(WebAccountProviderActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for WebAccountProviderActivatedEventArgs {}
impl ::windows_core::CanTryInto<IActivatedEventArgsWithUser> for WebAccountProviderActivatedEventArgs {}
impl ::windows_core::CanTryInto<IWebAccountProviderActivatedEventArgs> for WebAccountProviderActivatedEventArgs {}
unsafe impl ::core::marker::Send for WebAccountProviderActivatedEventArgs {}
unsafe impl ::core::marker::Sync for WebAccountProviderActivatedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebAuthenticationBrokerContinuationEventArgs(::windows_core::IUnknown);
impl WebAuthenticationBrokerContinuationEventArgs {
    pub fn Kind(&self) -> ::windows_core::Result<ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SplashScreen(&self) -> ::windows_core::Result<SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ContinuationData(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows_core::ComInterface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContinuationData)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Security_Authentication_Web\"`"]
    #[cfg(feature = "Security_Authentication_Web")]
    pub fn WebAuthenticationResult(&self) -> ::windows_core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WebAuthenticationResult)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WebAuthenticationBrokerContinuationEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.WebAuthenticationBrokerContinuationEventArgs;{75dda3d4-7714-453d-b7ff-b95e3a1709da})");
}
unsafe impl ::windows_core::Interface for WebAuthenticationBrokerContinuationEventArgs {
    type Vtable = IWebAuthenticationBrokerContinuationEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebAuthenticationBrokerContinuationEventArgs {
    const IID: ::windows_core::GUID = <IWebAuthenticationBrokerContinuationEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebAuthenticationBrokerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WebAuthenticationBrokerContinuationEventArgs";
}
::windows_core::imp::interface_hierarchy!(WebAuthenticationBrokerContinuationEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IActivatedEventArgs> for WebAuthenticationBrokerContinuationEventArgs {}
impl ::windows_core::CanTryInto<IContinuationActivatedEventArgs> for WebAuthenticationBrokerContinuationEventArgs {}
impl ::windows_core::CanTryInto<IWebAuthenticationBrokerContinuationEventArgs> for WebAuthenticationBrokerContinuationEventArgs {}
unsafe impl ::core::marker::Send for WebAuthenticationBrokerContinuationEventArgs {}
unsafe impl ::core::marker::Sync for WebAuthenticationBrokerContinuationEventArgs {}
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
impl ::windows_core::TypeKind for ActivationKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ActivationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ActivationKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Activation.ActivationKind;i4)");
}
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
impl ::windows_core::TypeKind for ApplicationExecutionState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ApplicationExecutionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationExecutionState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ApplicationExecutionState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Activation.ApplicationExecutionState;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
