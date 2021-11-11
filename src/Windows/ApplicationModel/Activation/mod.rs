#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct ActivatedEventsContract(pub u8);
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct ActivationCameraSettingsContract(pub u8);
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ActivationKind(pub i32);
impl ActivationKind {
    pub const Launch: ActivationKind = ActivationKind(0i32);
    pub const Search: ActivationKind = ActivationKind(1i32);
    pub const ShareTarget: ActivationKind = ActivationKind(2i32);
    pub const File: ActivationKind = ActivationKind(3i32);
    pub const Protocol: ActivationKind = ActivationKind(4i32);
    pub const FileOpenPicker: ActivationKind = ActivationKind(5i32);
    pub const FileSavePicker: ActivationKind = ActivationKind(6i32);
    pub const CachedFileUpdater: ActivationKind = ActivationKind(7i32);
    pub const ContactPicker: ActivationKind = ActivationKind(8i32);
    pub const Device: ActivationKind = ActivationKind(9i32);
    pub const PrintTaskSettings: ActivationKind = ActivationKind(10i32);
    pub const CameraSettings: ActivationKind = ActivationKind(11i32);
    pub const RestrictedLaunch: ActivationKind = ActivationKind(12i32);
    pub const AppointmentsProvider: ActivationKind = ActivationKind(13i32);
    pub const Contact: ActivationKind = ActivationKind(14i32);
    pub const LockScreenCall: ActivationKind = ActivationKind(15i32);
    pub const VoiceCommand: ActivationKind = ActivationKind(16i32);
    pub const LockScreen: ActivationKind = ActivationKind(17i32);
    pub const PickerReturned: ActivationKind = ActivationKind(1000i32);
    pub const WalletAction: ActivationKind = ActivationKind(1001i32);
    pub const PickFileContinuation: ActivationKind = ActivationKind(1002i32);
    pub const PickSaveFileContinuation: ActivationKind = ActivationKind(1003i32);
    pub const PickFolderContinuation: ActivationKind = ActivationKind(1004i32);
    pub const WebAuthenticationBrokerContinuation: ActivationKind = ActivationKind(1005i32);
    pub const WebAccountProvider: ActivationKind = ActivationKind(1006i32);
    pub const ComponentUI: ActivationKind = ActivationKind(1007i32);
    pub const ProtocolForResults: ActivationKind = ActivationKind(1009i32);
    pub const ToastNotification: ActivationKind = ActivationKind(1010i32);
    pub const Print3DWorkflow: ActivationKind = ActivationKind(1011i32);
    pub const DialReceiver: ActivationKind = ActivationKind(1012i32);
    pub const DevicePairing: ActivationKind = ActivationKind(1013i32);
    pub const UserDataAccountsProvider: ActivationKind = ActivationKind(1014i32);
    pub const FilePickerExperience: ActivationKind = ActivationKind(1015i32);
    pub const LockScreenComponent: ActivationKind = ActivationKind(1016i32);
    pub const ContactPanel: ActivationKind = ActivationKind(1017i32);
    pub const PrintWorkflowForegroundTask: ActivationKind = ActivationKind(1018i32);
    pub const GameUIProvider: ActivationKind = ActivationKind(1019i32);
    pub const StartupTask: ActivationKind = ActivationKind(1020i32);
    pub const CommandLineLaunch: ActivationKind = ActivationKind(1021i32);
    pub const BarcodeScannerProvider: ActivationKind = ActivationKind(1022i32);
    pub const PrintSupportJobUI: ActivationKind = ActivationKind(1023i32);
    pub const PrintSupportSettingsUI: ActivationKind = ActivationKind(1024i32);
    pub const PhoneCallActivation: ActivationKind = ActivationKind(1025i32);
    pub const VpnForeground: ActivationKind = ActivationKind(1026i32);
}
impl ::core::convert::From<i32> for ActivationKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ActivationKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ActivationKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Activation.ActivationKind;i4)");
}
impl ::windows::core::DefaultType for ActivationKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ApplicationExecutionState(pub i32);
impl ApplicationExecutionState {
    pub const NotRunning: ApplicationExecutionState = ApplicationExecutionState(0i32);
    pub const Running: ApplicationExecutionState = ApplicationExecutionState(1i32);
    pub const Suspended: ApplicationExecutionState = ApplicationExecutionState(2i32);
    pub const Terminated: ApplicationExecutionState = ApplicationExecutionState(3i32);
    pub const ClosedByUser: ApplicationExecutionState = ApplicationExecutionState(4i32);
}
impl ::core::convert::From<i32> for ApplicationExecutionState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ApplicationExecutionState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ApplicationExecutionState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Activation.ApplicationExecutionState;i4)");
}
impl ::windows::core::DefaultType for ApplicationExecutionState {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentsProviderAddAppointmentActivatedEventArgs(pub ::windows::core::IInspectable);
impl AppointmentsProviderAddAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Appointments_AppointmentsProvider`*"]
    pub fn AddAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Appointments::AppointmentsProvider::AddAppointmentOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentsProviderAddAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderAddAppointmentActivatedEventArgs;{a2861367-cee5-4e4d-9ed7-41c34ec18b02})");
}
unsafe impl ::windows::core::Interface for AppointmentsProviderAddAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderAddAppointmentActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2861367_cee5_4e4d_9ed7_41c34ec18b02);
}
impl ::windows::core::RuntimeName for AppointmentsProviderAddAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderAddAppointmentActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<AppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderAddAppointmentActivatedEventArgs> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderAddAppointmentActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderAddAppointmentActivatedEventArgs> for &AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderAddAppointmentActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderAddAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderAddAppointmentActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentsProviderRemoveAppointmentActivatedEventArgs(pub ::windows::core::IInspectable);
impl AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Appointments_AppointmentsProvider`*"]
    pub fn RemoveAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderRemoveAppointmentActivatedEventArgs;{751f3ab8-0b8e-451c-9f15-966e699bac25})");
}
unsafe impl ::windows::core::Interface for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderRemoveAppointmentActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x751f3ab8_0b8e_451c_9f15_966e699bac25);
}
impl ::windows::core::RuntimeName for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderRemoveAppointmentActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderRemoveAppointmentActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for &AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderRemoveAppointmentActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentsProviderReplaceAppointmentActivatedEventArgs(pub ::windows::core::IInspectable);
impl AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Appointments_AppointmentsProvider`*"]
    pub fn ReplaceAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderReplaceAppointmentActivatedEventArgs;{1551b7d4-a981-4067-8a62-0524e4ade121})");
}
unsafe impl ::windows::core::Interface for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderReplaceAppointmentActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1551b7d4_a981_4067_8a62_0524e4ade121);
}
impl ::windows::core::RuntimeName for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderReplaceAppointmentActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderReplaceAppointmentActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for &AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderReplaceAppointmentActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentsProviderShowAppointmentDetailsActivatedEventArgs(pub ::windows::core::IInspectable);
impl AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn InstanceStartDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn LocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn RoamingId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderShowAppointmentDetailsActivatedEventArgs;{3958f065-9841-4ca5-999b-885198b9ef2a})");
}
unsafe impl ::windows::core::Interface for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3958f065_9841_4ca5_999b_885198b9ef2a);
}
impl ::windows::core::RuntimeName for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentsProviderShowTimeFrameActivatedEventArgs(pub ::windows::core::IInspectable);
impl AppointmentsProviderShowTimeFrameActivatedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn TimeToShow(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderShowTimeFrameActivatedEventArgs;{9baeaba6-0e0b-49aa-babc-12b1dc774986})");
}
unsafe impl ::windows::core::Interface for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowTimeFrameActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9baeaba6_0e0b_49aa_babc_12b1dc774986);
}
impl ::windows::core::RuntimeName for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderShowTimeFrameActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<AppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderShowTimeFrameActivatedEventArgs> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderShowTimeFrameActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderShowTimeFrameActivatedEventArgs> for &AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderShowTimeFrameActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BackgroundActivatedEventArgs(pub ::windows::core::IInspectable);
impl BackgroundActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Background")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Background`*"]
    pub fn TaskInstance(&self) -> ::windows::core::Result<super::Background::IBackgroundTaskInstance> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Background::IBackgroundTaskInstance>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BackgroundActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.BackgroundActivatedEventArgs;{ab14bee0-e760-440e-a91c-44796de3a92d})");
}
unsafe impl ::windows::core::Interface for BackgroundActivatedEventArgs {
    type Vtable = IBackgroundActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab14bee0_e760_440e_a91c_44796de3a92d);
}
impl ::windows::core::RuntimeName for BackgroundActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.BackgroundActivatedEventArgs";
}
impl ::core::convert::From<BackgroundActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: BackgroundActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BackgroundActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BackgroundActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BackgroundActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: BackgroundActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BackgroundActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BackgroundActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<BackgroundActivatedEventArgs> for IBackgroundActivatedEventArgs {
    fn from(value: BackgroundActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BackgroundActivatedEventArgs> for IBackgroundActivatedEventArgs {
    fn from(value: &BackgroundActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundActivatedEventArgs> for BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBackgroundActivatedEventArgs> for &BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IBackgroundActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BackgroundActivatedEventArgs {}
unsafe impl ::core::marker::Sync for BackgroundActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BarcodeScannerPreviewActivatedEventArgs(pub ::windows::core::IInspectable);
impl BarcodeScannerPreviewActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ConnectionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerPreviewActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.BarcodeScannerPreviewActivatedEventArgs;{6772797c-99bf-4349-af22-e4123560371c})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerPreviewActivatedEventArgs {
    type Vtable = IBarcodeScannerPreviewActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6772797c_99bf_4349_af22_e4123560371c);
}
impl ::windows::core::RuntimeName for BarcodeScannerPreviewActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.BarcodeScannerPreviewActivatedEventArgs";
}
impl ::core::convert::From<BarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerPreviewActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerPreviewActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerPreviewActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerPreviewActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<BarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: BarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &BarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<BarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: BarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &BarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<BarcodeScannerPreviewActivatedEventArgs> for IBarcodeScannerPreviewActivatedEventArgs {
    fn from(value: BarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerPreviewActivatedEventArgs> for IBarcodeScannerPreviewActivatedEventArgs {
    fn from(value: &BarcodeScannerPreviewActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBarcodeScannerPreviewActivatedEventArgs> for BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IBarcodeScannerPreviewActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBarcodeScannerPreviewActivatedEventArgs> for &BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IBarcodeScannerPreviewActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerPreviewActivatedEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerPreviewActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CachedFileUpdaterActivatedEventArgs(pub ::windows::core::IInspectable);
impl CachedFileUpdaterActivatedEventArgs {
    #[cfg(feature = "Storage_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage_Provider`*"]
    pub fn CachedFileUpdaterUI(&self) -> ::windows::core::Result<super::super::Storage::Provider::CachedFileUpdaterUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Provider::CachedFileUpdaterUI>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CachedFileUpdaterActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CachedFileUpdaterActivatedEventArgs;{d06eb1c7-3805-4ecb-b757-6cf15e26fef3})");
}
unsafe impl ::windows::core::Interface for CachedFileUpdaterActivatedEventArgs {
    type Vtable = ICachedFileUpdaterActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd06eb1c7_3805_4ecb_b757_6cf15e26fef3);
}
impl ::windows::core::RuntimeName for CachedFileUpdaterActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CachedFileUpdaterActivatedEventArgs";
}
impl ::core::convert::From<CachedFileUpdaterActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CachedFileUpdaterActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CachedFileUpdaterActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CachedFileUpdaterActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CachedFileUpdaterActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CachedFileUpdaterActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CachedFileUpdaterActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CachedFileUpdaterActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<CachedFileUpdaterActivatedEventArgs> for ICachedFileUpdaterActivatedEventArgs {
    fn from(value: CachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CachedFileUpdaterActivatedEventArgs> for ICachedFileUpdaterActivatedEventArgs {
    fn from(value: &CachedFileUpdaterActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICachedFileUpdaterActivatedEventArgs> for CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICachedFileUpdaterActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICachedFileUpdaterActivatedEventArgs> for &CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICachedFileUpdaterActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CachedFileUpdaterActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: CachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CachedFileUpdaterActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &CachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<CachedFileUpdaterActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: CachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CachedFileUpdaterActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &CachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CachedFileUpdaterActivatedEventArgs {}
unsafe impl ::core::marker::Sync for CachedFileUpdaterActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CameraSettingsActivatedEventArgs(pub ::windows::core::IInspectable);
impl CameraSettingsActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn VideoDeviceController(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn VideoDeviceExtension(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CameraSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CameraSettingsActivatedEventArgs;{fb67a508-2dad-490a-9170-dca036eb114b})");
}
unsafe impl ::windows::core::Interface for CameraSettingsActivatedEventArgs {
    type Vtable = ICameraSettingsActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb67a508_2dad_490a_9170_dca036eb114b);
}
impl ::windows::core::RuntimeName for CameraSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CameraSettingsActivatedEventArgs";
}
impl ::core::convert::From<CameraSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CameraSettingsActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CameraSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CameraSettingsActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CameraSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CameraSettingsActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CameraSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CameraSettingsActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<CameraSettingsActivatedEventArgs> for ICameraSettingsActivatedEventArgs {
    fn from(value: CameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraSettingsActivatedEventArgs> for ICameraSettingsActivatedEventArgs {
    fn from(value: &CameraSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICameraSettingsActivatedEventArgs> for CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICameraSettingsActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICameraSettingsActivatedEventArgs> for &CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICameraSettingsActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CameraSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: CameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CameraSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &CameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CameraSettingsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for CameraSettingsActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CommandLineActivatedEventArgs(pub ::windows::core::IInspectable);
impl CommandLineActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Operation(&self) -> ::windows::core::Result<CommandLineActivationOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CommandLineActivationOperation>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CommandLineActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CommandLineActivatedEventArgs;{4506472c-006a-48eb-8afb-d07ab25e3366})");
}
unsafe impl ::windows::core::Interface for CommandLineActivatedEventArgs {
    type Vtable = ICommandLineActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4506472c_006a_48eb_8afb_d07ab25e3366);
}
impl ::windows::core::RuntimeName for CommandLineActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CommandLineActivatedEventArgs";
}
impl ::core::convert::From<CommandLineActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CommandLineActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CommandLineActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CommandLineActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CommandLineActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CommandLineActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CommandLineActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CommandLineActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<CommandLineActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: CommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CommandLineActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &CommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<CommandLineActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: CommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CommandLineActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &CommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<CommandLineActivatedEventArgs> for ICommandLineActivatedEventArgs {
    fn from(value: CommandLineActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CommandLineActivatedEventArgs> for ICommandLineActivatedEventArgs {
    fn from(value: &CommandLineActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommandLineActivatedEventArgs> for CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICommandLineActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommandLineActivatedEventArgs> for &CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICommandLineActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CommandLineActivatedEventArgs {}
unsafe impl ::core::marker::Sync for CommandLineActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CommandLineActivationOperation(pub ::windows::core::IInspectable);
impl CommandLineActivationOperation {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CurrentDirectoryPath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SetExitCode(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ExitCode(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CommandLineActivationOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CommandLineActivationOperation;{994b2841-c59e-4f69-bcfd-b61ed4e622eb})");
}
unsafe impl ::windows::core::Interface for CommandLineActivationOperation {
    type Vtable = ICommandLineActivationOperation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x994b2841_c59e_4f69_bcfd_b61ed4e622eb);
}
impl ::windows::core::RuntimeName for CommandLineActivationOperation {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CommandLineActivationOperation";
}
impl ::core::convert::From<CommandLineActivationOperation> for ::windows::core::IUnknown {
    fn from(value: CommandLineActivationOperation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CommandLineActivationOperation> for ::windows::core::IUnknown {
    fn from(value: &CommandLineActivationOperation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CommandLineActivationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CommandLineActivationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CommandLineActivationOperation> for ::windows::core::IInspectable {
    fn from(value: CommandLineActivationOperation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CommandLineActivationOperation> for ::windows::core::IInspectable {
    fn from(value: &CommandLineActivationOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CommandLineActivationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CommandLineActivationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CommandLineActivationOperation {}
unsafe impl ::core::marker::Sync for CommandLineActivationOperation {}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct ContactActivatedEventsContract(pub u8);
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactCallActivatedEventArgs(pub ::windows::core::IInspectable);
impl ContactCallActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ContactCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactCallActivatedEventArgs;{c2df14c7-30eb-41c6-b3bc-5b1694f9dab3})");
}
unsafe impl ::windows::core::Interface for ContactCallActivatedEventArgs {
    type Vtable = IContactCallActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2df14c7_30eb_41c6_b3bc_5b1694f9dab3);
}
impl ::windows::core::RuntimeName for ContactCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactCallActivatedEventArgs";
}
impl ::core::convert::From<ContactCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactCallActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactCallActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactCallActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactCallActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ContactCallActivatedEventArgs> for IContactCallActivatedEventArgs {
    fn from(value: ContactCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactCallActivatedEventArgs> for IContactCallActivatedEventArgs {
    fn from(value: &ContactCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactCallActivatedEventArgs> for ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactCallActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactCallActivatedEventArgs> for &ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactCallActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for &ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactCallActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactMapActivatedEventArgs(pub ::windows::core::IInspectable);
impl ContactMapActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Address(&self) -> ::windows::core::Result<super::Contacts::ContactAddress> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::ContactAddress>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ContactMapActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactMapActivatedEventArgs;{b32bf870-eee7-4ad2-aaf1-a87effcf00a4})");
}
unsafe impl ::windows::core::Interface for ContactMapActivatedEventArgs {
    type Vtable = IContactMapActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb32bf870_eee7_4ad2_aaf1_a87effcf00a4);
}
impl ::windows::core::RuntimeName for ContactMapActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactMapActivatedEventArgs";
}
impl ::core::convert::From<ContactMapActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactMapActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactMapActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactMapActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactMapActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactMapActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactMapActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactMapActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ContactMapActivatedEventArgs> for IContactMapActivatedEventArgs {
    fn from(value: ContactMapActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactMapActivatedEventArgs> for IContactMapActivatedEventArgs {
    fn from(value: &ContactMapActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactMapActivatedEventArgs> for ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactMapActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactMapActivatedEventArgs> for &ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactMapActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactMapActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMapActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactMapActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMapActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for &ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactMapActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactMapActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactMessageActivatedEventArgs(pub ::windows::core::IInspectable);
impl ContactMessageActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ContactMessageActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactMessageActivatedEventArgs;{de598db2-0e03-43b0-bf56-bcc40b3162df})");
}
unsafe impl ::windows::core::Interface for ContactMessageActivatedEventArgs {
    type Vtable = IContactMessageActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde598db2_0e03_43b0_bf56_bcc40b3162df);
}
impl ::windows::core::RuntimeName for ContactMessageActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactMessageActivatedEventArgs";
}
impl ::core::convert::From<ContactMessageActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactMessageActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactMessageActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactMessageActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactMessageActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactMessageActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactMessageActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactMessageActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ContactMessageActivatedEventArgs> for IContactMessageActivatedEventArgs {
    fn from(value: ContactMessageActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactMessageActivatedEventArgs> for IContactMessageActivatedEventArgs {
    fn from(value: &ContactMessageActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactMessageActivatedEventArgs> for ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactMessageActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactMessageActivatedEventArgs> for &ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactMessageActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactMessageActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMessageActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactMessageActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMessageActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for &ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactMessageActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactMessageActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactPanelActivatedEventArgs(pub ::windows::core::IInspectable);
impl ContactPanelActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn ContactPanel(&self) -> ::windows::core::Result<super::Contacts::ContactPanel> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::ContactPanel>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ContactPanelActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactPanelActivatedEventArgs;{52bb63e4-d3d4-4b63-8051-4af2082cab80})");
}
unsafe impl ::windows::core::Interface for ContactPanelActivatedEventArgs {
    type Vtable = IContactPanelActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52bb63e4_d3d4_4b63_8051_4af2082cab80);
}
impl ::windows::core::RuntimeName for ContactPanelActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPanelActivatedEventArgs";
}
impl ::core::convert::From<ContactPanelActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactPanelActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactPanelActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactPanelActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactPanelActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactPanelActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactPanelActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactPanelActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ContactPanelActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPanelActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactPanelActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPanelActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<ContactPanelActivatedEventArgs> for IContactPanelActivatedEventArgs {
    fn from(value: ContactPanelActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPanelActivatedEventArgs> for IContactPanelActivatedEventArgs {
    fn from(value: &ContactPanelActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactPanelActivatedEventArgs> for ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactPanelActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactPanelActivatedEventArgs> for &ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactPanelActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContactPanelActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactPanelActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactPickerActivatedEventArgs(pub ::windows::core::IInspectable);
impl ContactPickerActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts_Provider`*"]
    pub fn ContactPickerUI(&self) -> ::windows::core::Result<super::Contacts::Provider::ContactPickerUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Provider::ContactPickerUI>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ContactPickerActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactPickerActivatedEventArgs;{ce57aae7-6449-45a7-971f-d113be7a8936})");
}
unsafe impl ::windows::core::Interface for ContactPickerActivatedEventArgs {
    type Vtable = IContactPickerActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce57aae7_6449_45a7_971f_d113be7a8936);
}
impl ::windows::core::RuntimeName for ContactPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPickerActivatedEventArgs";
}
impl ::core::convert::From<ContactPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactPickerActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactPickerActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactPickerActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactPickerActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ContactPickerActivatedEventArgs> for IContactPickerActivatedEventArgs {
    fn from(value: ContactPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPickerActivatedEventArgs> for IContactPickerActivatedEventArgs {
    fn from(value: &ContactPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactPickerActivatedEventArgs> for ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactPickerActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactPickerActivatedEventArgs> for &ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactPickerActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactPickerActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactPickerActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactPostActivatedEventArgs(pub ::windows::core::IInspectable);
impl ContactPostActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ContactPostActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactPostActivatedEventArgs;{b35a3c67-f1e7-4655-ad6e-4857588f552f})");
}
unsafe impl ::windows::core::Interface for ContactPostActivatedEventArgs {
    type Vtable = IContactPostActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb35a3c67_f1e7_4655_ad6e_4857588f552f);
}
impl ::windows::core::RuntimeName for ContactPostActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPostActivatedEventArgs";
}
impl ::core::convert::From<ContactPostActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactPostActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactPostActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactPostActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactPostActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactPostActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactPostActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactPostActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ContactPostActivatedEventArgs> for IContactPostActivatedEventArgs {
    fn from(value: ContactPostActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPostActivatedEventArgs> for IContactPostActivatedEventArgs {
    fn from(value: &ContactPostActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactPostActivatedEventArgs> for ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactPostActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactPostActivatedEventArgs> for &ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactPostActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactPostActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPostActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactPostActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPostActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for &ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactPostActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactPostActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactVideoCallActivatedEventArgs(pub ::windows::core::IInspectable);
impl ContactVideoCallActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ContactVideoCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactVideoCallActivatedEventArgs;{61079db8-e3e7-4b4f-858d-5c63a96ef684})");
}
unsafe impl ::windows::core::Interface for ContactVideoCallActivatedEventArgs {
    type Vtable = IContactVideoCallActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61079db8_e3e7_4b4f_858d_5c63a96ef684);
}
impl ::windows::core::RuntimeName for ContactVideoCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactVideoCallActivatedEventArgs";
}
impl ::core::convert::From<ContactVideoCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactVideoCallActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactVideoCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactVideoCallActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactVideoCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactVideoCallActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactVideoCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactVideoCallActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ContactVideoCallActivatedEventArgs> for IContactVideoCallActivatedEventArgs {
    fn from(value: ContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactVideoCallActivatedEventArgs> for IContactVideoCallActivatedEventArgs {
    fn from(value: &ContactVideoCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactVideoCallActivatedEventArgs> for ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactVideoCallActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactVideoCallActivatedEventArgs> for &ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactVideoCallActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactVideoCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactVideoCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactVideoCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactVideoCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for &ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactVideoCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactVideoCallActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceActivatedEventArgs(pub ::windows::core::IInspectable);
impl DeviceActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn DeviceInformationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `UI_ViewManagement`*"]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.DeviceActivatedEventArgs;{cd50b9a9-ce10-44d2-8234-c355a073ef33})");
}
unsafe impl ::windows::core::Interface for DeviceActivatedEventArgs {
    type Vtable = IDeviceActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd50b9a9_ce10_44d2_8234_c355a073ef33);
}
impl ::windows::core::RuntimeName for DeviceActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DeviceActivatedEventArgs";
}
impl ::core::convert::From<DeviceActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DeviceActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DeviceActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DeviceActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DeviceActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DeviceActivatedEventArgs> for IDeviceActivatedEventArgs {
    fn from(value: DeviceActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceActivatedEventArgs> for IDeviceActivatedEventArgs {
    fn from(value: &DeviceActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDeviceActivatedEventArgs> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IDeviceActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDeviceActivatedEventArgs> for &DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IDeviceActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DeviceActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DeviceActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DeviceActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for &DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DeviceActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DeviceActivatedEventArgs {}
unsafe impl ::core::marker::Sync for DeviceActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DevicePairingActivatedEventArgs(pub ::windows::core::IInspectable);
impl DevicePairingActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Devices_Enumeration`*"]
    pub fn DeviceInformation(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DevicePairingActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.DevicePairingActivatedEventArgs;{eba0d1e4-ecc6-4148-94ed-f4b37ec05b3e})");
}
unsafe impl ::windows::core::Interface for DevicePairingActivatedEventArgs {
    type Vtable = IDevicePairingActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba0d1e4_ecc6_4148_94ed_f4b37ec05b3e);
}
impl ::windows::core::RuntimeName for DevicePairingActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DevicePairingActivatedEventArgs";
}
impl ::core::convert::From<DevicePairingActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DevicePairingActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DevicePairingActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DevicePairingActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DevicePairingActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DevicePairingActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DevicePairingActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DevicePairingActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<DevicePairingActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DevicePairingActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<DevicePairingActivatedEventArgs> for IDevicePairingActivatedEventArgs {
    fn from(value: DevicePairingActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DevicePairingActivatedEventArgs> for IDevicePairingActivatedEventArgs {
    fn from(value: &DevicePairingActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDevicePairingActivatedEventArgs> for DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IDevicePairingActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDevicePairingActivatedEventArgs> for &DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IDevicePairingActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DevicePairingActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: DevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DevicePairingActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &DevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DevicePairingActivatedEventArgs {}
unsafe impl ::core::marker::Sync for DevicePairingActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DialReceiverActivatedEventArgs(pub ::windows::core::IInspectable);
impl DialReceiverActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn AppName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `UI_ViewManagement`*"]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DialReceiverActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.DialReceiverActivatedEventArgs;{fb777ed7-85ee-456e-a44d-85d730e70aed})");
}
unsafe impl ::windows::core::Interface for DialReceiverActivatedEventArgs {
    type Vtable = IDialReceiverActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb777ed7_85ee_456e_a44d_85d730e70aed);
}
impl ::windows::core::RuntimeName for DialReceiverActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DialReceiverActivatedEventArgs";
}
impl ::core::convert::From<DialReceiverActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DialReceiverActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DialReceiverActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DialReceiverActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DialReceiverActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DialReceiverActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DialReceiverActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DialReceiverActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<DialReceiverActivatedEventArgs> for IDialReceiverActivatedEventArgs {
    fn from(value: DialReceiverActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DialReceiverActivatedEventArgs> for IDialReceiverActivatedEventArgs {
    fn from(value: &DialReceiverActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDialReceiverActivatedEventArgs> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IDialReceiverActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDialReceiverActivatedEventArgs> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IDialReceiverActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DialReceiverActivatedEventArgs {}
unsafe impl ::core::marker::Sync for DialReceiverActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FileActivatedEventArgs(pub ::windows::core::IInspectable);
impl FileActivatedEventArgs {
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`, `Storage`*"]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IFileActivatedEventArgsWithCallerPackageFamilyName>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage_Search`*"]
    pub fn NeighboringFilesQuery(&self) -> ::windows::core::Result<super::super::Storage::Search::StorageFileQueryResult> {
        let this = &::windows::core::Interface::cast::<IFileActivatedEventArgsWithNeighboringFiles>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Search::StorageFileQueryResult>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `UI_ViewManagement`*"]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for FileActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileActivatedEventArgs;{bb2afc33-93b1-42ed-8b26-236dd9c78496})");
}
unsafe impl ::windows::core::Interface for FileActivatedEventArgs {
    type Vtable = IFileActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb2afc33_93b1_42ed_8b26_236dd9c78496);
}
impl ::windows::core::RuntimeName for FileActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileActivatedEventArgs";
}
impl ::core::convert::From<FileActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: FileActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FileActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FileActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FileActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: FileActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FileActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FileActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<FileActivatedEventArgs> for IFileActivatedEventArgs {
    fn from(value: FileActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileActivatedEventArgs> for IFileActivatedEventArgs {
    fn from(value: &FileActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileActivatedEventArgs> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileActivatedEventArgs> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IFileActivatedEventArgsWithCallerPackageFamilyName {
    type Error = ::windows::core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IFileActivatedEventArgsWithCallerPackageFamilyName {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileActivatedEventArgsWithCallerPackageFamilyName> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileActivatedEventArgsWithCallerPackageFamilyName> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileActivatedEventArgsWithCallerPackageFamilyName> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileActivatedEventArgsWithCallerPackageFamilyName> {
        ::core::convert::TryInto::<IFileActivatedEventArgsWithCallerPackageFamilyName>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IFileActivatedEventArgsWithNeighboringFiles {
    type Error = ::windows::core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IFileActivatedEventArgsWithNeighboringFiles {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileActivatedEventArgsWithNeighboringFiles> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileActivatedEventArgsWithNeighboringFiles> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileActivatedEventArgsWithNeighboringFiles> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileActivatedEventArgsWithNeighboringFiles> {
        ::core::convert::TryInto::<IFileActivatedEventArgsWithNeighboringFiles>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FileActivatedEventArgs {}
unsafe impl ::core::marker::Sync for FileActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FileOpenPickerActivatedEventArgs(pub ::windows::core::IInspectable);
impl FileOpenPickerActivatedEventArgs {
    #[cfg(feature = "Storage_Pickers_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage_Pickers_Provider`*"]
    pub fn FileOpenPickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Pickers::Provider::FileOpenPickerUI>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IFileOpenPickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for FileOpenPickerActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileOpenPickerActivatedEventArgs;{72827082-5525-4bf2-bc09-1f5095d4964d})");
}
unsafe impl ::windows::core::Interface for FileOpenPickerActivatedEventArgs {
    type Vtable = IFileOpenPickerActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72827082_5525_4bf2_bc09_1f5095d4964d);
}
impl ::windows::core::RuntimeName for FileOpenPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileOpenPickerActivatedEventArgs";
}
impl ::core::convert::From<FileOpenPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: FileOpenPickerActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FileOpenPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FileOpenPickerActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FileOpenPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: FileOpenPickerActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FileOpenPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FileOpenPickerActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<FileOpenPickerActivatedEventArgs> for IFileOpenPickerActivatedEventArgs {
    fn from(value: FileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileOpenPickerActivatedEventArgs> for IFileOpenPickerActivatedEventArgs {
    fn from(value: &FileOpenPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileOpenPickerActivatedEventArgs> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileOpenPickerActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileOpenPickerActivatedEventArgs> for &FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileOpenPickerActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FileOpenPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileOpenPickerActivatedEventArgs> for IFileOpenPickerActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerActivatedEventArgs> for IFileOpenPickerActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileOpenPickerActivatedEventArgs2> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileOpenPickerActivatedEventArgs2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileOpenPickerActivatedEventArgs2> for &FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileOpenPickerActivatedEventArgs2> {
        ::core::convert::TryInto::<IFileOpenPickerActivatedEventArgs2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileOpenPickerActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FileOpenPickerActivatedEventArgs {}
unsafe impl ::core::marker::Sync for FileOpenPickerActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FileOpenPickerContinuationEventArgs(pub ::windows::core::IInspectable);
impl FileOpenPickerContinuationEventArgs {
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`, `Storage`*"]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for FileOpenPickerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileOpenPickerContinuationEventArgs;{f0fa3f3a-d4e8-4ad3-9c34-2308f32fcec9})");
}
unsafe impl ::windows::core::Interface for FileOpenPickerContinuationEventArgs {
    type Vtable = IFileOpenPickerContinuationEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0fa3f3a_d4e8_4ad3_9c34_2308f32fcec9);
}
impl ::windows::core::RuntimeName for FileOpenPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileOpenPickerContinuationEventArgs";
}
impl ::core::convert::From<FileOpenPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: FileOpenPickerContinuationEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FileOpenPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FileOpenPickerContinuationEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FileOpenPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: FileOpenPickerContinuationEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FileOpenPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FileOpenPickerContinuationEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<FileOpenPickerContinuationEventArgs> for IFileOpenPickerContinuationEventArgs {
    fn from(value: FileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileOpenPickerContinuationEventArgs> for IFileOpenPickerContinuationEventArgs {
    fn from(value: &FileOpenPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileOpenPickerContinuationEventArgs> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileOpenPickerContinuationEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileOpenPickerContinuationEventArgs> for &FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileOpenPickerContinuationEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FileOpenPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileOpenPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for &FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileOpenPickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FileOpenPickerContinuationEventArgs {}
unsafe impl ::core::marker::Sync for FileOpenPickerContinuationEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FileSavePickerActivatedEventArgs(pub ::windows::core::IInspectable);
impl FileSavePickerActivatedEventArgs {
    #[cfg(feature = "Storage_Pickers_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage_Pickers_Provider`*"]
    pub fn FileSavePickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Pickers::Provider::FileSavePickerUI>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for FileSavePickerActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileSavePickerActivatedEventArgs;{81c19cf1-74e6-4387-82eb-bb8fd64b4346})");
}
unsafe impl ::windows::core::Interface for FileSavePickerActivatedEventArgs {
    type Vtable = IFileSavePickerActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81c19cf1_74e6_4387_82eb_bb8fd64b4346);
}
impl ::windows::core::RuntimeName for FileSavePickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileSavePickerActivatedEventArgs";
}
impl ::core::convert::From<FileSavePickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: FileSavePickerActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FileSavePickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FileSavePickerActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FileSavePickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: FileSavePickerActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FileSavePickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FileSavePickerActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<FileSavePickerActivatedEventArgs> for IFileSavePickerActivatedEventArgs {
    fn from(value: FileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileSavePickerActivatedEventArgs> for IFileSavePickerActivatedEventArgs {
    fn from(value: &FileSavePickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileSavePickerActivatedEventArgs> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileSavePickerActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileSavePickerActivatedEventArgs> for &FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileSavePickerActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FileSavePickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileSavePickerActivatedEventArgs> for IFileSavePickerActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerActivatedEventArgs> for IFileSavePickerActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileSavePickerActivatedEventArgs2> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileSavePickerActivatedEventArgs2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileSavePickerActivatedEventArgs2> for &FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileSavePickerActivatedEventArgs2> {
        ::core::convert::TryInto::<IFileSavePickerActivatedEventArgs2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileSavePickerActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FileSavePickerActivatedEventArgs {}
unsafe impl ::core::marker::Sync for FileSavePickerActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FileSavePickerContinuationEventArgs(pub ::windows::core::IInspectable);
impl FileSavePickerContinuationEventArgs {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage`*"]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for FileSavePickerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileSavePickerContinuationEventArgs;{2c846fe1-3bad-4f33-8c8b-e46fae824b4b})");
}
unsafe impl ::windows::core::Interface for FileSavePickerContinuationEventArgs {
    type Vtable = IFileSavePickerContinuationEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c846fe1_3bad_4f33_8c8b_e46fae824b4b);
}
impl ::windows::core::RuntimeName for FileSavePickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileSavePickerContinuationEventArgs";
}
impl ::core::convert::From<FileSavePickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: FileSavePickerContinuationEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FileSavePickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FileSavePickerContinuationEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FileSavePickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: FileSavePickerContinuationEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FileSavePickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FileSavePickerContinuationEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<FileSavePickerContinuationEventArgs> for IFileSavePickerContinuationEventArgs {
    fn from(value: FileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileSavePickerContinuationEventArgs> for IFileSavePickerContinuationEventArgs {
    fn from(value: &FileSavePickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileSavePickerContinuationEventArgs> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileSavePickerContinuationEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileSavePickerContinuationEventArgs> for &FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFileSavePickerContinuationEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FileSavePickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileSavePickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for &FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FileSavePickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FileSavePickerContinuationEventArgs {}
unsafe impl ::core::marker::Sync for FileSavePickerContinuationEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FolderPickerContinuationEventArgs(pub ::windows::core::IInspectable);
impl FolderPickerContinuationEventArgs {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage`*"]
    pub fn Folder(&self) -> ::windows::core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for FolderPickerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FolderPickerContinuationEventArgs;{51882366-9f4b-498f-beb0-42684f6e1c29})");
}
unsafe impl ::windows::core::Interface for FolderPickerContinuationEventArgs {
    type Vtable = IFolderPickerContinuationEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51882366_9f4b_498f_beb0_42684f6e1c29);
}
impl ::windows::core::RuntimeName for FolderPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FolderPickerContinuationEventArgs";
}
impl ::core::convert::From<FolderPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: FolderPickerContinuationEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FolderPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FolderPickerContinuationEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FolderPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: FolderPickerContinuationEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FolderPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FolderPickerContinuationEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<FolderPickerContinuationEventArgs> for IFolderPickerContinuationEventArgs {
    fn from(value: FolderPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FolderPickerContinuationEventArgs> for IFolderPickerContinuationEventArgs {
    fn from(value: &FolderPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFolderPickerContinuationEventArgs> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFolderPickerContinuationEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFolderPickerContinuationEventArgs> for &FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IFolderPickerContinuationEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FolderPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FolderPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for &FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<FolderPickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderPickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for FolderPickerContinuationEventArgs {}
unsafe impl ::core::marker::Sync for FolderPickerContinuationEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IActivatedEventArgs {
    type Vtable = IActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf651713_cd08_4fd8_b697_a281b6544e2e);
}
impl IActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = self;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{cf651713-cd08-4fd8-b697-a281b6544e2e}");
}
impl ::core::convert::From<IActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ActivationKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ApplicationExecutionState) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IActivatedEventArgsWithUser(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IActivatedEventArgsWithUser {
    type Vtable = IActivatedEventArgsWithUser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cf09b9e_9962_4936_80ff_afc8e8ae5c8c);
}
impl IActivatedEventArgsWithUser {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IActivatedEventArgsWithUser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1cf09b9e-9962-4936-80ff-afc8e8ae5c8c}");
}
impl ::core::convert::From<IActivatedEventArgsWithUser> for ::windows::core::IUnknown {
    fn from(value: IActivatedEventArgsWithUser) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IActivatedEventArgsWithUser> for ::windows::core::IUnknown {
    fn from(value: &IActivatedEventArgsWithUser) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IActivatedEventArgsWithUser> for ::windows::core::IInspectable {
    fn from(value: IActivatedEventArgsWithUser) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IActivatedEventArgsWithUser> for ::windows::core::IInspectable {
    fn from(value: &IActivatedEventArgsWithUser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IActivatedEventArgsWithUser> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IActivatedEventArgsWithUser) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IActivatedEventArgsWithUser> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IActivatedEventArgsWithUser) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedEventArgsWithUser_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IApplicationViewActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IApplicationViewActivatedEventArgs {
    type Vtable = IApplicationViewActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x930cef4b_b829_40fc_88f4_8513e8a64738);
}
impl IApplicationViewActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IApplicationViewActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{930cef4b-b829-40fc-88f4-8513e8a64738}");
}
impl ::core::convert::From<IApplicationViewActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IApplicationViewActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IApplicationViewActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IApplicationViewActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IApplicationViewActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IApplicationViewActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IApplicationViewActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IApplicationViewActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IApplicationViewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IApplicationViewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IApplicationViewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IApplicationViewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IAppointmentsProviderActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentsProviderActivatedEventArgs {
    type Vtable = IAppointmentsProviderActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3364c405_933c_4e7d_a034_500fb8dcd9f3);
}
impl IAppointmentsProviderActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IAppointmentsProviderActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3364c405-933c-4e7d-a034-500fb8dcd9f3}");
}
impl ::core::convert::From<IAppointmentsProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IAppointmentsProviderActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAppointmentsProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IAppointmentsProviderActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAppointmentsProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IAppointmentsProviderActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppointmentsProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IAppointmentsProviderActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IAppointmentsProviderAddAppointmentActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderAddAppointmentActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2861367_cee5_4e4d_9ed7_41c34ec18b02);
}
impl IAppointmentsProviderAddAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Appointments_AppointmentsProvider`*"]
    pub fn AddAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Appointments::AppointmentsProvider::AddAppointmentOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a2861367-cee5-4e4d-9ed7-41c34ec18b02}");
}
impl ::core::convert::From<IAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderAddAppointmentActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IAppointmentsProviderRemoveAppointmentActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderRemoveAppointmentActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x751f3ab8_0b8e_451c_9f15_966e699bac25);
}
impl IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Appointments_AppointmentsProvider`*"]
    pub fn RemoveAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{751f3ab8-0b8e-451c-9f15-966e699bac25}");
}
impl ::core::convert::From<IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderRemoveAppointmentActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IAppointmentsProviderReplaceAppointmentActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderReplaceAppointmentActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1551b7d4_a981_4067_8a62_0524e4ade121);
}
impl IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Appointments_AppointmentsProvider`*"]
    pub fn ReplaceAppointmentOperation(&self) -> ::windows::core::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1551b7d4-a981-4067-8a62-0524e4ade121}");
}
impl ::core::convert::From<IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderReplaceAppointmentActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3958f065_9841_4ca5_999b_885198b9ef2a);
}
impl IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn InstanceStartDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn LocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn RoamingId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3958f065-9841-4ca5-999b-885198b9ef2a}");
}
impl ::core::convert::From<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IAppointmentsProviderShowTimeFrameActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowTimeFrameActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9baeaba6_0e0b_49aa_babc_12b1dc774986);
}
impl IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn TimeToShow(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{9baeaba6-0e0b-49aa-babc-12b1dc774986}");
}
impl ::core::convert::From<IAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderShowTimeFrameActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IBackgroundActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBackgroundActivatedEventArgs {
    type Vtable = IBackgroundActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab14bee0_e760_440e_a91c_44796de3a92d);
}
impl IBackgroundActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Background")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Background`*"]
    pub fn TaskInstance(&self) -> ::windows::core::Result<super::Background::IBackgroundTaskInstance> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Background::IBackgroundTaskInstance>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IBackgroundActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ab14bee0-e760-440e-a91c-44796de3a92d}");
}
impl ::core::convert::From<IBackgroundActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IBackgroundActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IBackgroundActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IBackgroundActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IBackgroundActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IBackgroundActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IBackgroundActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IBackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Background")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IBarcodeScannerPreviewActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarcodeScannerPreviewActivatedEventArgs {
    type Vtable = IBarcodeScannerPreviewActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6772797c_99bf_4349_af22_e4123560371c);
}
impl IBarcodeScannerPreviewActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ConnectionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IBarcodeScannerPreviewActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6772797c-99bf-4349-af22-e4123560371c}");
}
impl ::core::convert::From<IBarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IBarcodeScannerPreviewActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IBarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IBarcodeScannerPreviewActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IBarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IBarcodeScannerPreviewActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IBarcodeScannerPreviewActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IBarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IBarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IBarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IBarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerPreviewActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct ICachedFileUpdaterActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICachedFileUpdaterActivatedEventArgs {
    type Vtable = ICachedFileUpdaterActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd06eb1c7_3805_4ecb_b757_6cf15e26fef3);
}
impl ICachedFileUpdaterActivatedEventArgs {
    #[cfg(feature = "Storage_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage_Provider`*"]
    pub fn CachedFileUpdaterUI(&self) -> ::windows::core::Result<super::super::Storage::Provider::CachedFileUpdaterUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Provider::CachedFileUpdaterUI>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ICachedFileUpdaterActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d06eb1c7-3805-4ecb-b757-6cf15e26fef3}");
}
impl ::core::convert::From<ICachedFileUpdaterActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ICachedFileUpdaterActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICachedFileUpdaterActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ICachedFileUpdaterActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICachedFileUpdaterActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ICachedFileUpdaterActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICachedFileUpdaterActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ICachedFileUpdaterActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ICachedFileUpdaterActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ICachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ICachedFileUpdaterActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Provider")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Provider"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct ICameraSettingsActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICameraSettingsActivatedEventArgs {
    type Vtable = ICameraSettingsActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb67a508_2dad_490a_9170_dca036eb114b);
}
impl ICameraSettingsActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn VideoDeviceController(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn VideoDeviceExtension(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ICameraSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fb67a508-2dad-490a-9170-dca036eb114b}");
}
impl ::core::convert::From<ICameraSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ICameraSettingsActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICameraSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ICameraSettingsActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICameraSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ICameraSettingsActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICameraSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ICameraSettingsActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ICameraSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ICameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ICameraSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraSettingsActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct ICommandLineActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICommandLineActivatedEventArgs {
    type Vtable = ICommandLineActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4506472c_006a_48eb_8afb_d07ab25e3366);
}
impl ICommandLineActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Operation(&self) -> ::windows::core::Result<CommandLineActivationOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CommandLineActivationOperation>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ICommandLineActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4506472c-006a-48eb-8afb-d07ab25e3366}");
}
impl ::core::convert::From<ICommandLineActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ICommandLineActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICommandLineActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ICommandLineActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICommandLineActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ICommandLineActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICommandLineActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ICommandLineActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ICommandLineActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ICommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ICommandLineActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandLineActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICommandLineActivationOperation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICommandLineActivationOperation {
    type Vtable = ICommandLineActivationOperation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x994b2841_c59e_4f69_bcfd_b61ed4e622eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandLineActivationOperation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IContactActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactActivatedEventArgs {
    type Vtable = IContactActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd627a1c4_c025_4c41_9def_f1eafad075e7);
}
impl IContactActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IContactActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d627a1c4-c025-4c41-9def-f1eafad075e7}");
}
impl ::core::convert::From<IContactActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IContactActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContactActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IContactActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IContactActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContactActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IContactActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IContactActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IContactActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IContactActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IContactCallActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactCallActivatedEventArgs {
    type Vtable = IContactCallActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2df14c7_30eb_41c6_b3bc_5b1694f9dab3);
}
impl IContactCallActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IContactCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{c2df14c7-30eb-41c6-b3bc-5b1694f9dab3}");
}
impl ::core::convert::From<IContactCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactCallActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IContactCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactCallActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IContactCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactCallActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactCallActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IContactCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IContactCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for &IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactCallActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IContactMapActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactMapActivatedEventArgs {
    type Vtable = IContactMapActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb32bf870_eee7_4ad2_aaf1_a87effcf00a4);
}
impl IContactMapActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Address(&self) -> ::windows::core::Result<super::Contacts::ContactAddress> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::ContactAddress>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IContactMapActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b32bf870-eee7-4ad2-aaf1-a87effcf00a4}");
}
impl ::core::convert::From<IContactMapActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactMapActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IContactMapActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactMapActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IContactMapActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactMapActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactMapActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactMapActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IContactMapActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactMapActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IContactMapActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactMapActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for &IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactMapActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IContactMessageActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactMessageActivatedEventArgs {
    type Vtable = IContactMessageActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde598db2_0e03_43b0_bf56_bcc40b3162df);
}
impl IContactMessageActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IContactMessageActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{de598db2-0e03-43b0-bf56-bcc40b3162df}");
}
impl ::core::convert::From<IContactMessageActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactMessageActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IContactMessageActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactMessageActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IContactMessageActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactMessageActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactMessageActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactMessageActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IContactMessageActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactMessageActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IContactMessageActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactMessageActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for &IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactMessageActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IContactPanelActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactPanelActivatedEventArgs {
    type Vtable = IContactPanelActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52bb63e4_d3d4_4b63_8051_4af2082cab80);
}
impl IContactPanelActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn ContactPanel(&self) -> ::windows::core::Result<super::Contacts::ContactPanel> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::ContactPanel>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IContactPanelActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{52bb63e4-d3d4-4b63-8051-4af2082cab80}");
}
impl ::core::convert::From<IContactPanelActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactPanelActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IContactPanelActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactPanelActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IContactPanelActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactPanelActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactPanelActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactPanelActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPanelActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IContactPickerActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactPickerActivatedEventArgs {
    type Vtable = IContactPickerActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce57aae7_6449_45a7_971f_d113be7a8936);
}
impl IContactPickerActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts_Provider`*"]
    pub fn ContactPickerUI(&self) -> ::windows::core::Result<super::Contacts::Provider::ContactPickerUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Provider::ContactPickerUI>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IContactPickerActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ce57aae7-6449-45a7-971f-d113be7a8936}");
}
impl ::core::convert::From<IContactPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactPickerActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IContactPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactPickerActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IContactPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactPickerActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactPickerActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IContactPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPickerActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts_Provider")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts_Provider"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IContactPostActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactPostActivatedEventArgs {
    type Vtable = IContactPostActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb35a3c67_f1e7_4655_ad6e_4857588f552f);
}
impl IContactPostActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IContactPostActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b35a3c67-f1e7-4655-ad6e-4857588f552f}");
}
impl ::core::convert::From<IContactPostActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactPostActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IContactPostActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactPostActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IContactPostActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactPostActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactPostActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactPostActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IContactPostActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactPostActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IContactPostActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactPostActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for &IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPostActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IContactVideoCallActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactVideoCallActivatedEventArgs {
    type Vtable = IContactVideoCallActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61079db8_e3e7_4b4f_858d_5c63a96ef684);
}
impl IContactVideoCallActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::core::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IContactVideoCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{61079db8-e3e7-4b4f-858d-5c63a96ef684}");
}
impl ::core::convert::From<IContactVideoCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactVideoCallActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IContactVideoCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactVideoCallActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IContactVideoCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactVideoCallActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactVideoCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactVideoCallActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IContactVideoCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactVideoCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IContactVideoCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactVideoCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContactActivatedEventArgs> for &IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactVideoCallActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IContactsProviderActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactsProviderActivatedEventArgs {
    type Vtable = IContactsProviderActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4580dca8_5750_4916_aa52_c0829521eb94);
}
impl IContactsProviderActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IContactsProviderActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4580dca8-5750-4916-aa52-c0829521eb94}");
}
impl ::core::convert::From<IContactsProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContactsProviderActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IContactsProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContactsProviderActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IContactsProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContactsProviderActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactsProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContactsProviderActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IContactsProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContactsProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactsProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContactsProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactsProviderActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IContinuationActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContinuationActivatedEventArgs {
    type Vtable = IContinuationActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe58106b5_155f_4a94_a742_c7e08f4e188c);
}
impl IContinuationActivatedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IContinuationActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e58106b5-155f-4a94-a742-c7e08f4e188c}");
}
impl ::core::convert::From<IContinuationActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IContinuationActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IContinuationActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IContinuationActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IContinuationActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IContinuationActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContinuationActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IContinuationActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IContinuationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IContinuationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContinuationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IContinuationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContinuationActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IDeviceActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeviceActivatedEventArgs {
    type Vtable = IDeviceActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd50b9a9_ce10_44d2_8234_c355a073ef33);
}
impl IDeviceActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn DeviceInformationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IDeviceActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{cd50b9a9-ce10-44d2-8234-c355a073ef33}");
}
impl ::core::convert::From<IDeviceActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IDeviceActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IDeviceActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IDeviceActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IDeviceActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IDeviceActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDeviceActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IDeviceActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IDeviceActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IDeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IDeviceActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IDeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IDevicePairingActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDevicePairingActivatedEventArgs {
    type Vtable = IDevicePairingActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeba0d1e4_ecc6_4148_94ed_f4b37ec05b3e);
}
impl IDevicePairingActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Devices_Enumeration`*"]
    pub fn DeviceInformation(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IDevicePairingActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{eba0d1e4-ecc6-4148-94ed-f4b37ec05b3e}");
}
impl ::core::convert::From<IDevicePairingActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IDevicePairingActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IDevicePairingActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IDevicePairingActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IDevicePairingActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IDevicePairingActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDevicePairingActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IDevicePairingActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IDevicePairingActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IDevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IDevicePairingActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IDevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IDialReceiverActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDialReceiverActivatedEventArgs {
    type Vtable = IDialReceiverActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb777ed7_85ee_456e_a44d_85d730e70aed);
}
impl IDialReceiverActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn AppName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IDialReceiverActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fb777ed7-85ee-456e-a44d-85d730e70aed}");
}
impl ::core::convert::From<IDialReceiverActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IDialReceiverActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IDialReceiverActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IDialReceiverActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IDialReceiverActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IDialReceiverActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDialReceiverActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IDialReceiverActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IDialReceiverActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IDialReceiverActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IDialReceiverActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IDialReceiverActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for &IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialReceiverActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IFileActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFileActivatedEventArgs {
    type Vtable = IFileActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb2afc33_93b1_42ed_8b26_236dd9c78496);
}
impl IFileActivatedEventArgs {
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`, `Storage`*"]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IFileActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{bb2afc33-93b1-42ed-8b26-236dd9c78496}");
}
impl ::core::convert::From<IFileActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IFileActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFileActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IFileActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFileActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IFileActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IFileActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IFileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IFileActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IFileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IFileActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IFileActivatedEventArgsWithCallerPackageFamilyName(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFileActivatedEventArgsWithCallerPackageFamilyName {
    type Vtable = IFileActivatedEventArgsWithCallerPackageFamilyName_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d60f06b_d25f_4d25_8653_e1c5e1108309);
}
impl IFileActivatedEventArgsWithCallerPackageFamilyName {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IFileActivatedEventArgsWithCallerPackageFamilyName {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2d60f06b-d25f-4d25-8653-e1c5e1108309}");
}
impl ::core::convert::From<IFileActivatedEventArgsWithCallerPackageFamilyName> for ::windows::core::IUnknown {
    fn from(value: IFileActivatedEventArgsWithCallerPackageFamilyName) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFileActivatedEventArgsWithCallerPackageFamilyName> for ::windows::core::IUnknown {
    fn from(value: &IFileActivatedEventArgsWithCallerPackageFamilyName) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFileActivatedEventArgsWithCallerPackageFamilyName> for ::windows::core::IInspectable {
    fn from(value: IFileActivatedEventArgsWithCallerPackageFamilyName) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileActivatedEventArgsWithCallerPackageFamilyName> for ::windows::core::IInspectable {
    fn from(value: &IFileActivatedEventArgsWithCallerPackageFamilyName) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IFileActivatedEventArgsWithCallerPackageFamilyName> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileActivatedEventArgsWithCallerPackageFamilyName) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileActivatedEventArgsWithCallerPackageFamilyName> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileActivatedEventArgsWithCallerPackageFamilyName) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgsWithCallerPackageFamilyName_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IFileActivatedEventArgsWithNeighboringFiles(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFileActivatedEventArgsWithNeighboringFiles {
    type Vtable = IFileActivatedEventArgsWithNeighboringFiles_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x433ba1a4_e1e2_48fd_b7fc_b5d6eee65033);
}
impl IFileActivatedEventArgsWithNeighboringFiles {
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage_Search`*"]
    pub fn NeighboringFilesQuery(&self) -> ::windows::core::Result<super::super::Storage::Search::StorageFileQueryResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Search::StorageFileQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`, `Storage`*"]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = &::windows::core::Interface::cast::<IFileActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IFileActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IFileActivatedEventArgsWithNeighboringFiles {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{433ba1a4-e1e2-48fd-b7fc-b5d6eee65033}");
}
impl ::core::convert::From<IFileActivatedEventArgsWithNeighboringFiles> for ::windows::core::IUnknown {
    fn from(value: IFileActivatedEventArgsWithNeighboringFiles) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFileActivatedEventArgsWithNeighboringFiles> for ::windows::core::IUnknown {
    fn from(value: &IFileActivatedEventArgsWithNeighboringFiles) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFileActivatedEventArgsWithNeighboringFiles> for ::windows::core::IInspectable {
    fn from(value: IFileActivatedEventArgsWithNeighboringFiles) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileActivatedEventArgsWithNeighboringFiles> for ::windows::core::IInspectable {
    fn from(value: &IFileActivatedEventArgsWithNeighboringFiles) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IFileActivatedEventArgsWithNeighboringFiles> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileActivatedEventArgsWithNeighboringFiles) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileActivatedEventArgsWithNeighboringFiles> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileActivatedEventArgsWithNeighboringFiles) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IFileActivatedEventArgsWithNeighboringFiles> for IFileActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileActivatedEventArgsWithNeighboringFiles) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileActivatedEventArgsWithNeighboringFiles> for IFileActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileActivatedEventArgsWithNeighboringFiles) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileActivatedEventArgs> for IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::core::Param<'a, IFileActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IFileActivatedEventArgs> for &IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::core::Param<'a, IFileActivatedEventArgs> {
        ::core::convert::TryInto::<IFileActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgsWithNeighboringFiles_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Search")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IFileOpenPickerActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFileOpenPickerActivatedEventArgs {
    type Vtable = IFileOpenPickerActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72827082_5525_4bf2_bc09_1f5095d4964d);
}
impl IFileOpenPickerActivatedEventArgs {
    #[cfg(feature = "Storage_Pickers_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage_Pickers_Provider`*"]
    pub fn FileOpenPickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Pickers::Provider::FileOpenPickerUI>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IFileOpenPickerActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{72827082-5525-4bf2-bc09-1f5095d4964d}");
}
impl ::core::convert::From<IFileOpenPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IFileOpenPickerActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFileOpenPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IFileOpenPickerActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFileOpenPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IFileOpenPickerActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileOpenPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IFileOpenPickerActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IFileOpenPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileOpenPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Pickers_Provider")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Pickers_Provider"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IFileOpenPickerActivatedEventArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFileOpenPickerActivatedEventArgs2 {
    type Vtable = IFileOpenPickerActivatedEventArgs2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e731f66_8d1f_45fb_af1d_73205c8fc7a1);
}
impl IFileOpenPickerActivatedEventArgs2 {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IFileOpenPickerActivatedEventArgs2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{5e731f66-8d1f-45fb-af1d-73205c8fc7a1}");
}
impl ::core::convert::From<IFileOpenPickerActivatedEventArgs2> for ::windows::core::IUnknown {
    fn from(value: IFileOpenPickerActivatedEventArgs2) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFileOpenPickerActivatedEventArgs2> for ::windows::core::IUnknown {
    fn from(value: &IFileOpenPickerActivatedEventArgs2) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFileOpenPickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFileOpenPickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFileOpenPickerActivatedEventArgs2> for ::windows::core::IInspectable {
    fn from(value: IFileOpenPickerActivatedEventArgs2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileOpenPickerActivatedEventArgs2> for ::windows::core::IInspectable {
    fn from(value: &IFileOpenPickerActivatedEventArgs2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFileOpenPickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IFileOpenPickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerActivatedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IFileOpenPickerContinuationEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFileOpenPickerContinuationEventArgs {
    type Vtable = IFileOpenPickerContinuationEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0fa3f3a_d4e8_4ad3_9c34_2308f32fcec9);
}
impl IFileOpenPickerContinuationEventArgs {
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`, `Storage`*"]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IFileOpenPickerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f0fa3f3a-d4e8-4ad3-9c34-2308f32fcec9}");
}
impl ::core::convert::From<IFileOpenPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: IFileOpenPickerContinuationEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFileOpenPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IFileOpenPickerContinuationEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFileOpenPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: IFileOpenPickerContinuationEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileOpenPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IFileOpenPickerContinuationEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IFileOpenPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileOpenPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IFileOpenPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileOpenPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for &IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerContinuationEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IFileSavePickerActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFileSavePickerActivatedEventArgs {
    type Vtable = IFileSavePickerActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81c19cf1_74e6_4387_82eb_bb8fd64b4346);
}
impl IFileSavePickerActivatedEventArgs {
    #[cfg(feature = "Storage_Pickers_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage_Pickers_Provider`*"]
    pub fn FileSavePickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Pickers::Provider::FileSavePickerUI>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IFileSavePickerActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{81c19cf1-74e6-4387-82eb-bb8fd64b4346}");
}
impl ::core::convert::From<IFileSavePickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IFileSavePickerActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFileSavePickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IFileSavePickerActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFileSavePickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IFileSavePickerActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileSavePickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IFileSavePickerActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IFileSavePickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileSavePickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Pickers_Provider")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Pickers_Provider"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IFileSavePickerActivatedEventArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFileSavePickerActivatedEventArgs2 {
    type Vtable = IFileSavePickerActivatedEventArgs2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b73fe13_2cf2_4d48_8cbc_af67d23f1ce7);
}
impl IFileSavePickerActivatedEventArgs2 {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IFileSavePickerActivatedEventArgs2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6b73fe13-2cf2-4d48-8cbc-af67d23f1ce7}");
}
impl ::core::convert::From<IFileSavePickerActivatedEventArgs2> for ::windows::core::IUnknown {
    fn from(value: IFileSavePickerActivatedEventArgs2) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFileSavePickerActivatedEventArgs2> for ::windows::core::IUnknown {
    fn from(value: &IFileSavePickerActivatedEventArgs2) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFileSavePickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFileSavePickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFileSavePickerActivatedEventArgs2> for ::windows::core::IInspectable {
    fn from(value: IFileSavePickerActivatedEventArgs2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileSavePickerActivatedEventArgs2> for ::windows::core::IInspectable {
    fn from(value: &IFileSavePickerActivatedEventArgs2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFileSavePickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IFileSavePickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerActivatedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IFileSavePickerContinuationEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFileSavePickerContinuationEventArgs {
    type Vtable = IFileSavePickerContinuationEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c846fe1_3bad_4f33_8c8b_e46fae824b4b);
}
impl IFileSavePickerContinuationEventArgs {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage`*"]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IFileSavePickerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2c846fe1-3bad-4f33-8c8b-e46fae824b4b}");
}
impl ::core::convert::From<IFileSavePickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: IFileSavePickerContinuationEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFileSavePickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IFileSavePickerContinuationEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFileSavePickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: IFileSavePickerContinuationEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileSavePickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IFileSavePickerContinuationEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IFileSavePickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileSavePickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IFileSavePickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileSavePickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for &IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerContinuationEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IFolderPickerContinuationEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IFolderPickerContinuationEventArgs {
    type Vtable = IFolderPickerContinuationEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51882366_9f4b_498f_beb0_42684f6e1c29);
}
impl IFolderPickerContinuationEventArgs {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage`*"]
    pub fn Folder(&self) -> ::windows::core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IFolderPickerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{51882366-9f4b-498f-beb0-42684f6e1c29}");
}
impl ::core::convert::From<IFolderPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: IFolderPickerContinuationEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFolderPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IFolderPickerContinuationEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFolderPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: IFolderPickerContinuationEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFolderPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IFolderPickerContinuationEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IFolderPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFolderPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IFolderPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFolderPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for &IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderPickerContinuationEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct ILaunchActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILaunchActivatedEventArgs {
    type Vtable = ILaunchActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbc93e26_a14a_4b4f_82b0_33bed920af52);
}
impl ILaunchActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ILaunchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fbc93e26-a14a-4b4f-82b0-33bed920af52}");
}
impl ::core::convert::From<ILaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ILaunchActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ILaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ILaunchActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ILaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ILaunchActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ILaunchActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ILaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct ILaunchActivatedEventArgs2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILaunchActivatedEventArgs2 {
    type Vtable = ILaunchActivatedEventArgs2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fd37ebc_9dc9_46b5_9ace_bd95d4565345);
}
impl ILaunchActivatedEventArgs2 {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn TileActivatedInfo(&self) -> ::windows::core::Result<TileActivatedInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TileActivatedInfo>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ILaunchActivatedEventArgs2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{0fd37ebc-9dc9-46b5-9ace-bd95d4565345}");
}
impl ::core::convert::From<ILaunchActivatedEventArgs2> for ::windows::core::IUnknown {
    fn from(value: ILaunchActivatedEventArgs2) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ILaunchActivatedEventArgs2> for ::windows::core::IUnknown {
    fn from(value: &ILaunchActivatedEventArgs2) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ILaunchActivatedEventArgs2> for ::windows::core::IInspectable {
    fn from(value: ILaunchActivatedEventArgs2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILaunchActivatedEventArgs2> for ::windows::core::IInspectable {
    fn from(value: &ILaunchActivatedEventArgs2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ILaunchActivatedEventArgs2> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ILaunchActivatedEventArgs2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILaunchActivatedEventArgs2> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILaunchActivatedEventArgs2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ILaunchActivatedEventArgs2> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ILaunchActivatedEventArgs2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILaunchActivatedEventArgs2> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILaunchActivatedEventArgs2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for &ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchActivatedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct ILockScreenActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILockScreenActivatedEventArgs {
    type Vtable = ILockScreenActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ca77966_6108_4a41_8220_ee7d133c8532);
}
impl ILockScreenActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Info(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ILockScreenActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3ca77966-6108-4a41-8220-ee7d133c8532}");
}
impl ::core::convert::From<ILockScreenActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ILockScreenActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ILockScreenActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ILockScreenActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ILockScreenActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ILockScreenActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILockScreenActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ILockScreenActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ILockScreenActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ILockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILockScreenActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct ILockScreenCallActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILockScreenCallActivatedEventArgs {
    type Vtable = ILockScreenCallActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06f37fbe_b5f2_448b_b13e_e328ac1c516a);
}
impl ILockScreenCallActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Calls")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Calls`*"]
    pub fn CallUI(&self) -> ::windows::core::Result<super::Calls::LockScreenCallUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Calls::LockScreenCallUI>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ILockScreenCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{06f37fbe-b5f2-448b-b13e-e328ac1c516a}");
}
impl ::core::convert::From<ILockScreenCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ILockScreenCallActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ILockScreenCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ILockScreenCallActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ILockScreenCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ILockScreenCallActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILockScreenCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ILockScreenCallActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ILockScreenCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILockScreenCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ILockScreenCallActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILockScreenCallActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for &ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenCallActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Calls")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Calls"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IPhoneCallActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPhoneCallActivatedEventArgs {
    type Vtable = IPhoneCallActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54615221_a3c1_4ced_b62f_8c60523619ad);
}
impl IPhoneCallActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IPhoneCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{54615221-a3c1-4ced-b62f-8c60523619ad}");
}
impl ::core::convert::From<IPhoneCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IPhoneCallActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPhoneCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IPhoneCallActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPhoneCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IPhoneCallActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPhoneCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IPhoneCallActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IPhoneCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IPhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPhoneCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IPickerReturnedActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPickerReturnedActivatedEventArgs {
    type Vtable = IPickerReturnedActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x360defb9_a9d3_4984_a4ed_9ec734604921);
}
impl IPickerReturnedActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PickerOperationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IPickerReturnedActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{360defb9-a9d3-4984-a4ed-9ec734604921}");
}
impl ::core::convert::From<IPickerReturnedActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IPickerReturnedActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPickerReturnedActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IPickerReturnedActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPickerReturnedActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IPickerReturnedActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPickerReturnedActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IPickerReturnedActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IPickerReturnedActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IPickerReturnedActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPickerReturnedActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPickerReturnedActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerReturnedActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IPrelaunchActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrelaunchActivatedEventArgs {
    type Vtable = IPrelaunchActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c44717b_19f7_48d6_b046_cf22826eaa74);
}
impl IPrelaunchActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PrelaunchActivated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IPrelaunchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{0c44717b-19f7-48d6-b046-cf22826eaa74}");
}
impl ::core::convert::From<IPrelaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IPrelaunchActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPrelaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IPrelaunchActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPrelaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IPrelaunchActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrelaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IPrelaunchActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IPrelaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IPrelaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPrelaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPrelaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrelaunchActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IPrint3DWorkflowActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrint3DWorkflowActivatedEventArgs {
    type Vtable = IPrint3DWorkflowActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f57e78b_f2ac_4619_8302_ef855e1c9b90);
}
impl IPrint3DWorkflowActivatedEventArgs {
    #[cfg(feature = "Devices_Printers_Extensions")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Devices_Printers_Extensions`*"]
    pub fn Workflow(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Printers::Extensions::Print3DWorkflow>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IPrint3DWorkflowActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3f57e78b-f2ac-4619-8302-ef855e1c9b90}");
}
impl ::core::convert::From<IPrint3DWorkflowActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IPrint3DWorkflowActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPrint3DWorkflowActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IPrint3DWorkflowActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPrint3DWorkflowActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IPrint3DWorkflowActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrint3DWorkflowActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IPrint3DWorkflowActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IPrint3DWorkflowActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IPrint3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPrint3DWorkflowActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPrint3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DWorkflowActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Printers_Extensions")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Printers_Extensions"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IPrintTaskSettingsActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintTaskSettingsActivatedEventArgs {
    type Vtable = IPrintTaskSettingsActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee30a0c9_ce56_4865_ba8e_8954ac271107);
}
impl IPrintTaskSettingsActivatedEventArgs {
    #[cfg(feature = "Devices_Printers_Extensions")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Devices_Printers_Extensions`*"]
    pub fn Configuration(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Printers::Extensions::PrintTaskConfiguration>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IPrintTaskSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ee30a0c9-ce56-4865-ba8e-8954ac271107}");
}
impl ::core::convert::From<IPrintTaskSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IPrintTaskSettingsActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPrintTaskSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IPrintTaskSettingsActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPrintTaskSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IPrintTaskSettingsActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintTaskSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IPrintTaskSettingsActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IPrintTaskSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IPrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPrintTaskSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskSettingsActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Printers_Extensions")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Printers_Extensions"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IProtocolActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProtocolActivatedEventArgs {
    type Vtable = IProtocolActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6095f4dd_b7c0_46ab_81fe_d90f36d00d24);
}
impl IProtocolActivatedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IProtocolActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6095f4dd-b7c0-46ab-81fe-d90f36d00d24}");
}
impl ::core::convert::From<IProtocolActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IProtocolActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IProtocolActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IProtocolActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IProtocolActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IProtocolActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProtocolActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IProtocolActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IProtocolActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IProtocolActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Vtable = IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd84a0c12_5c8f_438c_83cb_c28fcc0b2fdb);
}
impl IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d84a0c12-5c8f-438c-83cb-c28fcc0b2fdb}");
}
impl ::core::convert::From<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ::windows::core::IUnknown {
    fn from(value: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ::windows::core::IUnknown {
    fn from(value: &IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ::windows::core::IInspectable {
    fn from(value: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ::windows::core::IInspectable {
    fn from(value: &IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IProtocolForResultsActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IProtocolForResultsActivatedEventArgs {
    type Vtable = IProtocolForResultsActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe75132c2_7ae7_4517_80ac_dbe8d7cc5b9c);
}
impl IProtocolForResultsActivatedEventArgs {
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn ProtocolForResultsOperation(&self) -> ::windows::core::Result<super::super::System::ProtocolForResultsOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::ProtocolForResultsOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IProtocolForResultsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e75132c2-7ae7-4517-80ac-dbe8d7cc5b9c}");
}
impl ::core::convert::From<IProtocolForResultsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IProtocolForResultsActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IProtocolForResultsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IProtocolForResultsActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IProtocolForResultsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IProtocolForResultsActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProtocolForResultsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IProtocolForResultsActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IProtocolForResultsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IProtocolForResultsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolForResultsActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IRestrictedLaunchActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IRestrictedLaunchActivatedEventArgs {
    type Vtable = IRestrictedLaunchActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0b7ac81_bfc3_4344_a5da_19fd5a27baae);
}
impl IRestrictedLaunchActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SharedContext(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IRestrictedLaunchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e0b7ac81-bfc3-4344-a5da-19fd5a27baae}");
}
impl ::core::convert::From<IRestrictedLaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IRestrictedLaunchActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IRestrictedLaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IRestrictedLaunchActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IRestrictedLaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IRestrictedLaunchActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRestrictedLaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IRestrictedLaunchActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IRestrictedLaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IRestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IRestrictedLaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IRestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRestrictedLaunchActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct ISearchActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISearchActivatedEventArgs {
    type Vtable = ISearchActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cb36951_58c8_43e3_94bc_41d33f8b630e);
}
impl ISearchActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ISearchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{8cb36951-58c8-43e3-94bc-41d33f8b630e}");
}
impl ::core::convert::From<ISearchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ISearchActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ISearchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ISearchActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ISearchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ISearchActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISearchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ISearchActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ISearchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ISearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ISearchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ISearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct ISearchActivatedEventArgsWithLinguisticDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISearchActivatedEventArgsWithLinguisticDetails {
    type Vtable = ISearchActivatedEventArgsWithLinguisticDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc09f33da_08ab_4931_9b7c_451025f21f81);
}
impl ISearchActivatedEventArgsWithLinguisticDetails {
    #[cfg(feature = "ApplicationModel_Search")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Search`*"]
    pub fn LinguisticDetails(&self) -> ::windows::core::Result<super::Search::SearchPaneQueryLinguisticDetails> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Search::SearchPaneQueryLinguisticDetails>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ISearchActivatedEventArgsWithLinguisticDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{c09f33da-08ab-4931-9b7c-451025f21f81}");
}
impl ::core::convert::From<ISearchActivatedEventArgsWithLinguisticDetails> for ::windows::core::IUnknown {
    fn from(value: ISearchActivatedEventArgsWithLinguisticDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ISearchActivatedEventArgsWithLinguisticDetails> for ::windows::core::IUnknown {
    fn from(value: &ISearchActivatedEventArgsWithLinguisticDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISearchActivatedEventArgsWithLinguisticDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISearchActivatedEventArgsWithLinguisticDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ISearchActivatedEventArgsWithLinguisticDetails> for ::windows::core::IInspectable {
    fn from(value: ISearchActivatedEventArgsWithLinguisticDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISearchActivatedEventArgsWithLinguisticDetails> for ::windows::core::IInspectable {
    fn from(value: &ISearchActivatedEventArgsWithLinguisticDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISearchActivatedEventArgsWithLinguisticDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ISearchActivatedEventArgsWithLinguisticDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchActivatedEventArgsWithLinguisticDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Search")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Search"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IShareTargetActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IShareTargetActivatedEventArgs {
    type Vtable = IShareTargetActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bdaf9c8_cdb2_4acb_bfc3_6648563378ec);
}
impl IShareTargetActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_DataTransfer_ShareTarget`*"]
    pub fn ShareOperation(&self) -> ::windows::core::Result<super::DataTransfer::ShareTarget::ShareOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DataTransfer::ShareTarget::ShareOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IShareTargetActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4bdaf9c8-cdb2-4acb-bfc3-6648563378ec}");
}
impl ::core::convert::From<IShareTargetActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IShareTargetActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IShareTargetActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IShareTargetActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IShareTargetActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IShareTargetActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IShareTargetActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IShareTargetActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IShareTargetActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IShareTargetActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareTargetActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer_ShareTarget"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISplashScreen(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISplashScreen {
    type Vtable = ISplashScreen_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca4d975c_d4d6_43f0_97c0_0833c6391c24);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplashScreen_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IStartupTaskActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStartupTaskActivatedEventArgs {
    type Vtable = IStartupTaskActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03b11a58_5276_4d91_8621_54611864d5fa);
}
impl IStartupTaskActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IStartupTaskActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{03b11a58-5276-4d91-8621-54611864d5fa}");
}
impl ::core::convert::From<IStartupTaskActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IStartupTaskActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IStartupTaskActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IStartupTaskActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IStartupTaskActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IStartupTaskActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStartupTaskActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IStartupTaskActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IStartupTaskActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IStartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IStartupTaskActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IStartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartupTaskActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITileActivatedInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ITileActivatedInfo {
    type Vtable = ITileActivatedInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80e4a3b1_3980_4f17_b738_89194e0b8f65);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileActivatedInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Notifications"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Notifications")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IToastNotificationActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IToastNotificationActivatedEventArgs {
    type Vtable = IToastNotificationActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92a86f82_5290_431d_be85_c4aaeeb8685f);
}
impl IToastNotificationActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Argument(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn UserInput(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IToastNotificationActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{92a86f82-5290-431d-be85-c4aaeeb8685f}");
}
impl ::core::convert::From<IToastNotificationActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IToastNotificationActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IToastNotificationActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IToastNotificationActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IToastNotificationActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IToastNotificationActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IToastNotificationActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IToastNotificationActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IToastNotificationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IToastNotificationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IUserDataAccountProviderActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserDataAccountProviderActivatedEventArgs {
    type Vtable = IUserDataAccountProviderActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bc9f723_8ef1_4a51_a63a_fe711eeab607);
}
impl IUserDataAccountProviderActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_UserDataAccounts_Provider`*"]
    pub fn Operation(&self) -> ::windows::core::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IUserDataAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1bc9f723-8ef1-4a51-a63a-fe711eeab607}");
}
impl ::core::convert::From<IUserDataAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IUserDataAccountProviderActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IUserDataAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IUserDataAccountProviderActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IUserDataAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IUserDataAccountProviderActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUserDataAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IUserDataAccountProviderActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IUserDataAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IUserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IUserDataAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IUserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_UserDataAccounts_Provider"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IViewSwitcherProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IViewSwitcherProvider {
    type Vtable = IViewSwitcherProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33f288a6_5c2c_4d27_bac7_7536088f1219);
}
impl IViewSwitcherProvider {
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `UI_ViewManagement`*"]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IViewSwitcherProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{33f288a6-5c2c-4d27-bac7-7536088f1219}");
}
impl ::core::convert::From<IViewSwitcherProvider> for ::windows::core::IUnknown {
    fn from(value: IViewSwitcherProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IViewSwitcherProvider> for ::windows::core::IUnknown {
    fn from(value: &IViewSwitcherProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IViewSwitcherProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IViewSwitcherProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IViewSwitcherProvider> for ::windows::core::IInspectable {
    fn from(value: IViewSwitcherProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IViewSwitcherProvider> for ::windows::core::IInspectable {
    fn from(value: &IViewSwitcherProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IViewSwitcherProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IViewSwitcherProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IViewSwitcherProvider> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IViewSwitcherProvider) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IViewSwitcherProvider> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IViewSwitcherProvider) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IViewSwitcherProvider {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IViewSwitcherProvider {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewSwitcherProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_ViewManagement")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IVoiceCommandActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVoiceCommandActivatedEventArgs {
    type Vtable = IVoiceCommandActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab92dcfd_8d43_4de6_9775_20704b581b00);
}
impl IVoiceCommandActivatedEventArgs {
    #[cfg(feature = "Media_SpeechRecognition")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Media_SpeechRecognition`*"]
    pub fn Result(&self) -> ::windows::core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::SpeechRecognition::SpeechRecognitionResult>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IVoiceCommandActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ab92dcfd-8d43-4de6-9775-20704b581b00}");
}
impl ::core::convert::From<IVoiceCommandActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IVoiceCommandActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IVoiceCommandActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IVoiceCommandActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IVoiceCommandActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IVoiceCommandActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVoiceCommandActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IVoiceCommandActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IVoiceCommandActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IVoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IVoiceCommandActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IVoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_SpeechRecognition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_SpeechRecognition"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IWalletActionActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWalletActionActivatedEventArgs {
    type Vtable = IWalletActionActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcfc027b_1a1a_4d22_923f_ae6f45fa52d9);
}
impl IWalletActionActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ItemId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Wallet")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Wallet`*"]
    pub fn ActionKind(&self) -> ::windows::core::Result<super::Wallet::WalletActionKind> {
        let this = self;
        unsafe {
            let mut result__: super::Wallet::WalletActionKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Wallet::WalletActionKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ActionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IWalletActionActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fcfc027b-1a1a-4d22-923f-ae6f45fa52d9}");
}
impl ::core::convert::From<IWalletActionActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IWalletActionActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IWalletActionActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IWalletActionActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IWalletActionActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IWalletActionActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWalletActionActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IWalletActionActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IWalletActionActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IWalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWalletActionActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IWalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletActionActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Wallet")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::Wallet::WalletActionKind) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Wallet"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IWebAccountProviderActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWebAccountProviderActivatedEventArgs {
    type Vtable = IWebAccountProviderActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72b71774_98ea_4ccf_9752_46d9051004f1);
}
impl IWebAccountProviderActivatedEventArgs {
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Security_Authentication_Web_Provider`*"]
    pub fn Operation(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IWebAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{72b71774-98ea-4ccf-9752-46d9051004f1}");
}
impl ::core::convert::From<IWebAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IWebAccountProviderActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IWebAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IWebAccountProviderActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IWebAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IWebAccountProviderActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWebAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IWebAccountProviderActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IWebAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IWebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWebAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IWebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Authentication_Web_Provider")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Provider"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IWebAuthenticationBrokerContinuationEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWebAuthenticationBrokerContinuationEventArgs {
    type Vtable = IWebAuthenticationBrokerContinuationEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75dda3d4_7714_453d_b7ff_b95e3a1709da);
}
impl IWebAuthenticationBrokerContinuationEventArgs {
    #[cfg(feature = "Security_Authentication_Web")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Security_Authentication_Web`*"]
    pub fn WebAuthenticationResult(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Authentication::Web::WebAuthenticationResult>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IWebAuthenticationBrokerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{75dda3d4-7714-453d-b7ff-b95e3a1709da}");
}
impl ::core::convert::From<IWebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: IWebAuthenticationBrokerContinuationEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IWebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IWebAuthenticationBrokerContinuationEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IWebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: IWebAuthenticationBrokerContinuationEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IWebAuthenticationBrokerContinuationEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IWebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<IWebAuthenticationBrokerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: IWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWebAuthenticationBrokerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &IWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for &IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationBrokerContinuationEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Security_Authentication_Web")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))] usize,
);
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LaunchActivatedEventArgs(pub ::windows::core::IInspectable);
impl LaunchActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PrelaunchActivated(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPrelaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `UI_ViewManagement`*"]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn TileActivatedInfo(&self) -> ::windows::core::Result<TileActivatedInfo> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TileActivatedInfo>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LaunchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LaunchActivatedEventArgs;{fbc93e26-a14a-4b4f-82b0-33bed920af52})");
}
unsafe impl ::windows::core::Interface for LaunchActivatedEventArgs {
    type Vtable = ILaunchActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbc93e26_a14a_4b4f_82b0_33bed920af52);
}
impl ::windows::core::RuntimeName for LaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LaunchActivatedEventArgs";
}
impl ::core::convert::From<LaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: LaunchActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LaunchActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: LaunchActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LaunchActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<LaunchActivatedEventArgs> for ILaunchActivatedEventArgs {
    fn from(value: LaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LaunchActivatedEventArgs> for ILaunchActivatedEventArgs {
    fn from(value: &LaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IPrelaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IPrelaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrelaunchActivatedEventArgs> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IPrelaunchActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrelaunchActivatedEventArgs> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IPrelaunchActivatedEventArgs> {
        ::core::convert::TryInto::<IPrelaunchActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for ILaunchActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for ILaunchActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs2> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs2> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs2> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LaunchActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LaunchActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LockScreenActivatedEventArgs(pub ::windows::core::IInspectable);
impl LockScreenActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Info(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LockScreenActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LockScreenActivatedEventArgs;{3ca77966-6108-4a41-8220-ee7d133c8532})");
}
unsafe impl ::windows::core::Interface for LockScreenActivatedEventArgs {
    type Vtable = ILockScreenActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ca77966_6108_4a41_8220_ee7d133c8532);
}
impl ::windows::core::RuntimeName for LockScreenActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenActivatedEventArgs";
}
impl ::core::convert::From<LockScreenActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: LockScreenActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LockScreenActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LockScreenActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LockScreenActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: LockScreenActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LockScreenActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LockScreenActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<LockScreenActivatedEventArgs> for ILockScreenActivatedEventArgs {
    fn from(value: LockScreenActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenActivatedEventArgs> for ILockScreenActivatedEventArgs {
    fn from(value: &LockScreenActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILockScreenActivatedEventArgs> for LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILockScreenActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILockScreenActivatedEventArgs> for &LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILockScreenActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LockScreenActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LockScreenActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LockScreenActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LockScreenCallActivatedEventArgs(pub ::windows::core::IInspectable);
impl LockScreenCallActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Calls")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Calls`*"]
    pub fn CallUI(&self) -> ::windows::core::Result<super::Calls::LockScreenCallUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Calls::LockScreenCallUI>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `UI_ViewManagement`*"]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LockScreenCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LockScreenCallActivatedEventArgs;{06f37fbe-b5f2-448b-b13e-e328ac1c516a})");
}
unsafe impl ::windows::core::Interface for LockScreenCallActivatedEventArgs {
    type Vtable = ILockScreenCallActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06f37fbe_b5f2_448b_b13e_e328ac1c516a);
}
impl ::windows::core::RuntimeName for LockScreenCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenCallActivatedEventArgs";
}
impl ::core::convert::From<LockScreenCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: LockScreenCallActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LockScreenCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LockScreenCallActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LockScreenCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: LockScreenCallActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LockScreenCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LockScreenCallActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<LockScreenCallActivatedEventArgs> for ILockScreenCallActivatedEventArgs {
    fn from(value: LockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenCallActivatedEventArgs> for ILockScreenCallActivatedEventArgs {
    fn from(value: &LockScreenCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILockScreenCallActivatedEventArgs> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILockScreenCallActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILockScreenCallActivatedEventArgs> for &LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILockScreenCallActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LockScreenCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LockScreenCallActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LockScreenCallActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ILaunchActivatedEventArgs> for &LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<LockScreenCallActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for &LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LockScreenCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenCallActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LockScreenComponentActivatedEventArgs(pub ::windows::core::IInspectable);
impl LockScreenComponentActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = self;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LockScreenComponentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LockScreenComponentActivatedEventArgs;{cf651713-cd08-4fd8-b697-a281b6544e2e})");
}
unsafe impl ::windows::core::Interface for LockScreenComponentActivatedEventArgs {
    type Vtable = IActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf651713_cd08_4fd8_b697_a281b6544e2e);
}
impl ::windows::core::RuntimeName for LockScreenComponentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenComponentActivatedEventArgs";
}
impl ::core::convert::From<LockScreenComponentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: LockScreenComponentActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LockScreenComponentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LockScreenComponentActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LockScreenComponentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: LockScreenComponentActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LockScreenComponentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LockScreenComponentActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<LockScreenComponentActivatedEventArgs> for IActivatedEventArgs {
    fn from(value: LockScreenComponentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LockScreenComponentActivatedEventArgs> for IActivatedEventArgs {
    fn from(value: &LockScreenComponentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LockScreenComponentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenComponentActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneCallActivatedEventArgs(pub ::windows::core::IInspectable);
impl PhoneCallActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PhoneCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.PhoneCallActivatedEventArgs;{54615221-a3c1-4ced-b62f-8c60523619ad})");
}
unsafe impl ::windows::core::Interface for PhoneCallActivatedEventArgs {
    type Vtable = IPhoneCallActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54615221_a3c1_4ced_b62f_8c60523619ad);
}
impl ::windows::core::RuntimeName for PhoneCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PhoneCallActivatedEventArgs";
}
impl ::core::convert::From<PhoneCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PhoneCallActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PhoneCallActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PhoneCallActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PhoneCallActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<PhoneCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: PhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PhoneCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &PhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PhoneCallActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: PhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PhoneCallActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &PhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<PhoneCallActivatedEventArgs> for IPhoneCallActivatedEventArgs {
    fn from(value: PhoneCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhoneCallActivatedEventArgs> for IPhoneCallActivatedEventArgs {
    fn from(value: &PhoneCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPhoneCallActivatedEventArgs> for PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IPhoneCallActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPhoneCallActivatedEventArgs> for &PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IPhoneCallActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PhoneCallActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PickerReturnedActivatedEventArgs(pub ::windows::core::IInspectable);
impl PickerReturnedActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PickerOperationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PickerReturnedActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.PickerReturnedActivatedEventArgs;{360defb9-a9d3-4984-a4ed-9ec734604921})");
}
unsafe impl ::windows::core::Interface for PickerReturnedActivatedEventArgs {
    type Vtable = IPickerReturnedActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x360defb9_a9d3_4984_a4ed_9ec734604921);
}
impl ::windows::core::RuntimeName for PickerReturnedActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PickerReturnedActivatedEventArgs";
}
impl ::core::convert::From<PickerReturnedActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PickerReturnedActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PickerReturnedActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PickerReturnedActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PickerReturnedActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PickerReturnedActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PickerReturnedActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PickerReturnedActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PickerReturnedActivatedEventArgs> for IPickerReturnedActivatedEventArgs {
    fn from(value: PickerReturnedActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PickerReturnedActivatedEventArgs> for IPickerReturnedActivatedEventArgs {
    fn from(value: &PickerReturnedActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPickerReturnedActivatedEventArgs> for PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IPickerReturnedActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPickerReturnedActivatedEventArgs> for &PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IPickerReturnedActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PickerReturnedActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: PickerReturnedActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PickerReturnedActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &PickerReturnedActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PickerReturnedActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PickerReturnedActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Print3DWorkflowActivatedEventArgs(pub ::windows::core::IInspectable);
impl Print3DWorkflowActivatedEventArgs {
    #[cfg(feature = "Devices_Printers_Extensions")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Devices_Printers_Extensions`*"]
    pub fn Workflow(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Printers::Extensions::Print3DWorkflow>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for Print3DWorkflowActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.Print3DWorkflowActivatedEventArgs;{3f57e78b-f2ac-4619-8302-ef855e1c9b90})");
}
unsafe impl ::windows::core::Interface for Print3DWorkflowActivatedEventArgs {
    type Vtable = IPrint3DWorkflowActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f57e78b_f2ac_4619_8302_ef855e1c9b90);
}
impl ::windows::core::RuntimeName for Print3DWorkflowActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.Print3DWorkflowActivatedEventArgs";
}
impl ::core::convert::From<Print3DWorkflowActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: Print3DWorkflowActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Print3DWorkflowActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &Print3DWorkflowActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Print3DWorkflowActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: Print3DWorkflowActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Print3DWorkflowActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &Print3DWorkflowActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<Print3DWorkflowActivatedEventArgs> for IPrint3DWorkflowActivatedEventArgs {
    fn from(value: Print3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Print3DWorkflowActivatedEventArgs> for IPrint3DWorkflowActivatedEventArgs {
    fn from(value: &Print3DWorkflowActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrint3DWorkflowActivatedEventArgs> for Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IPrint3DWorkflowActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrint3DWorkflowActivatedEventArgs> for &Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IPrint3DWorkflowActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<Print3DWorkflowActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: Print3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Print3DWorkflowActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &Print3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for Print3DWorkflowActivatedEventArgs {}
unsafe impl ::core::marker::Sync for Print3DWorkflowActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTaskSettingsActivatedEventArgs(pub ::windows::core::IInspectable);
impl PrintTaskSettingsActivatedEventArgs {
    #[cfg(feature = "Devices_Printers_Extensions")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Devices_Printers_Extensions`*"]
    pub fn Configuration(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Printers::Extensions::PrintTaskConfiguration>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.PrintTaskSettingsActivatedEventArgs;{ee30a0c9-ce56-4865-ba8e-8954ac271107})");
}
unsafe impl ::windows::core::Interface for PrintTaskSettingsActivatedEventArgs {
    type Vtable = IPrintTaskSettingsActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee30a0c9_ce56_4865_ba8e_8954ac271107);
}
impl ::windows::core::RuntimeName for PrintTaskSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PrintTaskSettingsActivatedEventArgs";
}
impl ::core::convert::From<PrintTaskSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintTaskSettingsActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintTaskSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskSettingsActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintTaskSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintTaskSettingsActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintTaskSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskSettingsActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PrintTaskSettingsActivatedEventArgs> for IPrintTaskSettingsActivatedEventArgs {
    fn from(value: PrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTaskSettingsActivatedEventArgs> for IPrintTaskSettingsActivatedEventArgs {
    fn from(value: &PrintTaskSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintTaskSettingsActivatedEventArgs> for PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintTaskSettingsActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintTaskSettingsActivatedEventArgs> for &PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintTaskSettingsActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PrintTaskSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintTaskSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintTaskSettingsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PrintTaskSettingsActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProtocolActivatedEventArgs(pub ::windows::core::IInspectable);
impl ProtocolActivatedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `UI_ViewManagement`*"]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ProtocolActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ProtocolActivatedEventArgs;{6095f4dd-b7c0-46ab-81fe-d90f36d00d24})");
}
unsafe impl ::windows::core::Interface for ProtocolActivatedEventArgs {
    type Vtable = IProtocolActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6095f4dd_b7c0_46ab_81fe_d90f36d00d24);
}
impl ::windows::core::RuntimeName for ProtocolActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ProtocolActivatedEventArgs";
}
impl ::core::convert::From<ProtocolActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ProtocolActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProtocolActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ProtocolActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProtocolActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ProtocolActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProtocolActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ProtocolActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ProtocolActivatedEventArgs> for IProtocolActivatedEventArgs {
    fn from(value: ProtocolActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtocolActivatedEventArgs> for IProtocolActivatedEventArgs {
    fn from(value: &ProtocolActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IProtocolActivatedEventArgs> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IProtocolActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IProtocolActivatedEventArgs> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IProtocolActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
        ::core::convert::TryInto::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ProtocolActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ProtocolActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProtocolForResultsActivatedEventArgs(pub ::windows::core::IInspectable);
impl ProtocolForResultsActivatedEventArgs {
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn ProtocolForResultsOperation(&self) -> ::windows::core::Result<super::super::System::ProtocolForResultsOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::ProtocolForResultsOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IProtocolActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `UI_ViewManagement`*"]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ProtocolForResultsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ProtocolForResultsActivatedEventArgs;{e75132c2-7ae7-4517-80ac-dbe8d7cc5b9c})");
}
unsafe impl ::windows::core::Interface for ProtocolForResultsActivatedEventArgs {
    type Vtable = IProtocolForResultsActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe75132c2_7ae7_4517_80ac_dbe8d7cc5b9c);
}
impl ::windows::core::RuntimeName for ProtocolForResultsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ProtocolForResultsActivatedEventArgs";
}
impl ::core::convert::From<ProtocolForResultsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ProtocolForResultsActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProtocolForResultsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ProtocolForResultsActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProtocolForResultsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ProtocolForResultsActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProtocolForResultsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ProtocolForResultsActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ProtocolForResultsActivatedEventArgs> for IProtocolForResultsActivatedEventArgs {
    fn from(value: ProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ProtocolForResultsActivatedEventArgs> for IProtocolForResultsActivatedEventArgs {
    fn from(value: &ProtocolForResultsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IProtocolForResultsActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IProtocolForResultsActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IProtocolForResultsActivatedEventArgs> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IProtocolForResultsActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IProtocolActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IProtocolActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IProtocolActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IProtocolActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IProtocolActivatedEventArgs> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IProtocolActivatedEventArgs> {
        ::core::convert::TryInto::<IProtocolActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
        ::core::convert::TryInto::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ProtocolForResultsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ProtocolForResultsActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RestrictedLaunchActivatedEventArgs(pub ::windows::core::IInspectable);
impl RestrictedLaunchActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SharedContext(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for RestrictedLaunchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.RestrictedLaunchActivatedEventArgs;{e0b7ac81-bfc3-4344-a5da-19fd5a27baae})");
}
unsafe impl ::windows::core::Interface for RestrictedLaunchActivatedEventArgs {
    type Vtable = IRestrictedLaunchActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0b7ac81_bfc3_4344_a5da_19fd5a27baae);
}
impl ::windows::core::RuntimeName for RestrictedLaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.RestrictedLaunchActivatedEventArgs";
}
impl ::core::convert::From<RestrictedLaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RestrictedLaunchActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RestrictedLaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RestrictedLaunchActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RestrictedLaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RestrictedLaunchActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RestrictedLaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RestrictedLaunchActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<RestrictedLaunchActivatedEventArgs> for IRestrictedLaunchActivatedEventArgs {
    fn from(value: RestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RestrictedLaunchActivatedEventArgs> for IRestrictedLaunchActivatedEventArgs {
    fn from(value: &RestrictedLaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRestrictedLaunchActivatedEventArgs> for RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IRestrictedLaunchActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IRestrictedLaunchActivatedEventArgs> for &RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IRestrictedLaunchActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<RestrictedLaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: RestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RestrictedLaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &RestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<RestrictedLaunchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: RestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RestrictedLaunchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &RestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for RestrictedLaunchActivatedEventArgs {}
unsafe impl ::core::marker::Sync for RestrictedLaunchActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SearchActivatedEventArgs(pub ::windows::core::IInspectable);
impl SearchActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Search")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Search`*"]
    pub fn LinguisticDetails(&self) -> ::windows::core::Result<super::Search::SearchPaneQueryLinguisticDetails> {
        let this = &::windows::core::Interface::cast::<ISearchActivatedEventArgsWithLinguisticDetails>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Search::SearchPaneQueryLinguisticDetails>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `UI_ViewManagement`*"]
    pub fn ViewSwitcher(&self) -> ::windows::core::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::core::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SearchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.SearchActivatedEventArgs;{8cb36951-58c8-43e3-94bc-41d33f8b630e})");
}
unsafe impl ::windows::core::Interface for SearchActivatedEventArgs {
    type Vtable = ISearchActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cb36951_58c8_43e3_94bc_41d33f8b630e);
}
impl ::windows::core::RuntimeName for SearchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.SearchActivatedEventArgs";
}
impl ::core::convert::From<SearchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: SearchActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SearchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SearchActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SearchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: SearchActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SearchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SearchActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<SearchActivatedEventArgs> for ISearchActivatedEventArgs {
    fn from(value: SearchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SearchActivatedEventArgs> for ISearchActivatedEventArgs {
    fn from(value: &SearchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISearchActivatedEventArgs> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ISearchActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISearchActivatedEventArgs> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ISearchActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for ISearchActivatedEventArgsWithLinguisticDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for ISearchActivatedEventArgsWithLinguisticDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISearchActivatedEventArgsWithLinguisticDetails> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ISearchActivatedEventArgsWithLinguisticDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ISearchActivatedEventArgsWithLinguisticDetails> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ISearchActivatedEventArgsWithLinguisticDetails> {
        ::core::convert::TryInto::<ISearchActivatedEventArgsWithLinguisticDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IViewSwitcherProvider> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SearchActivatedEventArgs {}
unsafe impl ::core::marker::Sync for SearchActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ShareTargetActivatedEventArgs(pub ::windows::core::IInspectable);
impl ShareTargetActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_DataTransfer_ShareTarget`*"]
    pub fn ShareOperation(&self) -> ::windows::core::Result<super::DataTransfer::ShareTarget::ShareOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DataTransfer::ShareTarget::ShareOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ShareTargetActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ShareTargetActivatedEventArgs;{4bdaf9c8-cdb2-4acb-bfc3-6648563378ec})");
}
unsafe impl ::windows::core::Interface for ShareTargetActivatedEventArgs {
    type Vtable = IShareTargetActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bdaf9c8_cdb2_4acb_bfc3_6648563378ec);
}
impl ::windows::core::RuntimeName for ShareTargetActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ShareTargetActivatedEventArgs";
}
impl ::core::convert::From<ShareTargetActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ShareTargetActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ShareTargetActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ShareTargetActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ShareTargetActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ShareTargetActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ShareTargetActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ShareTargetActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ShareTargetActivatedEventArgs> for IShareTargetActivatedEventArgs {
    fn from(value: ShareTargetActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ShareTargetActivatedEventArgs> for IShareTargetActivatedEventArgs {
    fn from(value: &ShareTargetActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IShareTargetActivatedEventArgs> for ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IShareTargetActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IShareTargetActivatedEventArgs> for &ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IShareTargetActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ShareTargetActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ShareTargetActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ShareTargetActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: ShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ShareTargetActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &ShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ShareTargetActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ShareTargetActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SplashScreen(pub ::windows::core::IInspectable);
impl SplashScreen {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn ImageLocation(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn Dismissed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<SplashScreen, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn RemoveDismissed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for SplashScreen {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.SplashScreen;{ca4d975c-d4d6-43f0-97c0-0833c6391c24})");
}
unsafe impl ::windows::core::Interface for SplashScreen {
    type Vtable = ISplashScreen_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca4d975c_d4d6_43f0_97c0_0833c6391c24);
}
impl ::windows::core::RuntimeName for SplashScreen {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.SplashScreen";
}
impl ::core::convert::From<SplashScreen> for ::windows::core::IUnknown {
    fn from(value: SplashScreen) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SplashScreen> for ::windows::core::IUnknown {
    fn from(value: &SplashScreen) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SplashScreen {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SplashScreen {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SplashScreen> for ::windows::core::IInspectable {
    fn from(value: SplashScreen) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SplashScreen> for ::windows::core::IInspectable {
    fn from(value: &SplashScreen) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SplashScreen {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SplashScreen {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StartupTaskActivatedEventArgs(pub ::windows::core::IInspectable);
impl StartupTaskActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for StartupTaskActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.StartupTaskActivatedEventArgs;{03b11a58-5276-4d91-8621-54611864d5fa})");
}
unsafe impl ::windows::core::Interface for StartupTaskActivatedEventArgs {
    type Vtable = IStartupTaskActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03b11a58_5276_4d91_8621_54611864d5fa);
}
impl ::windows::core::RuntimeName for StartupTaskActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.StartupTaskActivatedEventArgs";
}
impl ::core::convert::From<StartupTaskActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: StartupTaskActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StartupTaskActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &StartupTaskActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StartupTaskActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: StartupTaskActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StartupTaskActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &StartupTaskActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<StartupTaskActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: StartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StartupTaskActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &StartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<StartupTaskActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: StartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StartupTaskActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &StartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<StartupTaskActivatedEventArgs> for IStartupTaskActivatedEventArgs {
    fn from(value: StartupTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StartupTaskActivatedEventArgs> for IStartupTaskActivatedEventArgs {
    fn from(value: &StartupTaskActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStartupTaskActivatedEventArgs> for StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IStartupTaskActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStartupTaskActivatedEventArgs> for &StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IStartupTaskActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StartupTaskActivatedEventArgs {}
unsafe impl ::core::marker::Sync for StartupTaskActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TileActivatedInfo(pub ::windows::core::IInspectable);
impl TileActivatedInfo {
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Notifications"))]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`, `UI_Notifications`*"]
    pub fn RecentlyShownNotifications(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::UI::Notifications::ShownTileNotification>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::UI::Notifications::ShownTileNotification>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for TileActivatedInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.TileActivatedInfo;{80e4a3b1-3980-4f17-b738-89194e0b8f65})");
}
unsafe impl ::windows::core::Interface for TileActivatedInfo {
    type Vtable = ITileActivatedInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80e4a3b1_3980_4f17_b738_89194e0b8f65);
}
impl ::windows::core::RuntimeName for TileActivatedInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.TileActivatedInfo";
}
impl ::core::convert::From<TileActivatedInfo> for ::windows::core::IUnknown {
    fn from(value: TileActivatedInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TileActivatedInfo> for ::windows::core::IUnknown {
    fn from(value: &TileActivatedInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for TileActivatedInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a TileActivatedInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TileActivatedInfo> for ::windows::core::IInspectable {
    fn from(value: TileActivatedInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TileActivatedInfo> for ::windows::core::IInspectable {
    fn from(value: &TileActivatedInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for TileActivatedInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a TileActivatedInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TileActivatedInfo {}
unsafe impl ::core::marker::Sync for TileActivatedInfo {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ToastNotificationActivatedEventArgs(pub ::windows::core::IInspectable);
impl ToastNotificationActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Argument(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn UserInput(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ToastNotificationActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ToastNotificationActivatedEventArgs;{92a86f82-5290-431d-be85-c4aaeeb8685f})");
}
unsafe impl ::windows::core::Interface for ToastNotificationActivatedEventArgs {
    type Vtable = IToastNotificationActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92a86f82_5290_431d_be85_c4aaeeb8685f);
}
impl ::windows::core::RuntimeName for ToastNotificationActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ToastNotificationActivatedEventArgs";
}
impl ::core::convert::From<ToastNotificationActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ToastNotificationActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ToastNotificationActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ToastNotificationActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ToastNotificationActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ToastNotificationActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ToastNotificationActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ToastNotificationActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ToastNotificationActivatedEventArgs> for IToastNotificationActivatedEventArgs {
    fn from(value: ToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ToastNotificationActivatedEventArgs> for IToastNotificationActivatedEventArgs {
    fn from(value: &ToastNotificationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IToastNotificationActivatedEventArgs> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IToastNotificationActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IToastNotificationActivatedEventArgs> for &ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IToastNotificationActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ToastNotificationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ToastNotificationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ToastNotificationActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ToastNotificationActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IApplicationViewActivatedEventArgs> for &ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ToastNotificationActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ToastNotificationActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &ToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ToastNotificationActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ToastNotificationActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserDataAccountProviderActivatedEventArgs(pub ::windows::core::IInspectable);
impl UserDataAccountProviderActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_UserDataAccounts_Provider`*"]
    pub fn Operation(&self) -> ::windows::core::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.UserDataAccountProviderActivatedEventArgs;{1bc9f723-8ef1-4a51-a63a-fe711eeab607})");
}
unsafe impl ::windows::core::Interface for UserDataAccountProviderActivatedEventArgs {
    type Vtable = IUserDataAccountProviderActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1bc9f723_8ef1_4a51_a63a_fe711eeab607);
}
impl ::windows::core::RuntimeName for UserDataAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.UserDataAccountProviderActivatedEventArgs";
}
impl ::core::convert::From<UserDataAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: UserDataAccountProviderActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserDataAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &UserDataAccountProviderActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserDataAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: UserDataAccountProviderActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserDataAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &UserDataAccountProviderActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<UserDataAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: UserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserDataAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &UserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::From<UserDataAccountProviderActivatedEventArgs> for IUserDataAccountProviderActivatedEventArgs {
    fn from(value: UserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountProviderActivatedEventArgs> for IUserDataAccountProviderActivatedEventArgs {
    fn from(value: &UserDataAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IUserDataAccountProviderActivatedEventArgs> for UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IUserDataAccountProviderActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IUserDataAccountProviderActivatedEventArgs> for &UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IUserDataAccountProviderActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataAccountProviderActivatedEventArgs {}
unsafe impl ::core::marker::Sync for UserDataAccountProviderActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VoiceCommandActivatedEventArgs(pub ::windows::core::IInspectable);
impl VoiceCommandActivatedEventArgs {
    #[cfg(feature = "Media_SpeechRecognition")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Media_SpeechRecognition`*"]
    pub fn Result(&self) -> ::windows::core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::SpeechRecognition::SpeechRecognitionResult>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceCommandActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.VoiceCommandActivatedEventArgs;{ab92dcfd-8d43-4de6-9775-20704b581b00})");
}
unsafe impl ::windows::core::Interface for VoiceCommandActivatedEventArgs {
    type Vtable = IVoiceCommandActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab92dcfd_8d43_4de6_9775_20704b581b00);
}
impl ::windows::core::RuntimeName for VoiceCommandActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.VoiceCommandActivatedEventArgs";
}
impl ::core::convert::From<VoiceCommandActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: VoiceCommandActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VoiceCommandActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &VoiceCommandActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VoiceCommandActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: VoiceCommandActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VoiceCommandActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &VoiceCommandActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<VoiceCommandActivatedEventArgs> for IVoiceCommandActivatedEventArgs {
    fn from(value: VoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VoiceCommandActivatedEventArgs> for IVoiceCommandActivatedEventArgs {
    fn from(value: &VoiceCommandActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVoiceCommandActivatedEventArgs> for VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IVoiceCommandActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVoiceCommandActivatedEventArgs> for &VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IVoiceCommandActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<VoiceCommandActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: VoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VoiceCommandActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &VoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<VoiceCommandActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: VoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VoiceCommandActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &VoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for VoiceCommandActivatedEventArgs {}
unsafe impl ::core::marker::Sync for VoiceCommandActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WalletActionActivatedEventArgs(pub ::windows::core::IInspectable);
impl WalletActionActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ItemId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Wallet")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Wallet`*"]
    pub fn ActionKind(&self) -> ::windows::core::Result<super::Wallet::WalletActionKind> {
        let this = self;
        unsafe {
            let mut result__: super::Wallet::WalletActionKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Wallet::WalletActionKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ActionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for WalletActionActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.WalletActionActivatedEventArgs;{fcfc027b-1a1a-4d22-923f-ae6f45fa52d9})");
}
unsafe impl ::windows::core::Interface for WalletActionActivatedEventArgs {
    type Vtable = IWalletActionActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcfc027b_1a1a_4d22_923f_ae6f45fa52d9);
}
impl ::windows::core::RuntimeName for WalletActionActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WalletActionActivatedEventArgs";
}
impl ::core::convert::From<WalletActionActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WalletActionActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WalletActionActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WalletActionActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WalletActionActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WalletActionActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WalletActionActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WalletActionActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<WalletActionActivatedEventArgs> for IWalletActionActivatedEventArgs {
    fn from(value: WalletActionActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WalletActionActivatedEventArgs> for IWalletActionActivatedEventArgs {
    fn from(value: &WalletActionActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWalletActionActivatedEventArgs> for WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IWalletActionActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWalletActionActivatedEventArgs> for &WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IWalletActionActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WalletActionActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WalletActionActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WalletActionActivatedEventArgs {}
unsafe impl ::core::marker::Sync for WalletActionActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WebAccountProviderActivatedEventArgs(pub ::windows::core::IInspectable);
impl WebAccountProviderActivatedEventArgs {
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Security_Authentication_Web_Provider`*"]
    pub fn Operation(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for WebAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.WebAccountProviderActivatedEventArgs;{72b71774-98ea-4ccf-9752-46d9051004f1})");
}
unsafe impl ::windows::core::Interface for WebAccountProviderActivatedEventArgs {
    type Vtable = IWebAccountProviderActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72b71774_98ea_4ccf_9752_46d9051004f1);
}
impl ::windows::core::RuntimeName for WebAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WebAccountProviderActivatedEventArgs";
}
impl ::core::convert::From<WebAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebAccountProviderActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WebAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebAccountProviderActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WebAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebAccountProviderActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WebAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebAccountProviderActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<WebAccountProviderActivatedEventArgs> for IWebAccountProviderActivatedEventArgs {
    fn from(value: WebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAccountProviderActivatedEventArgs> for IWebAccountProviderActivatedEventArgs {
    fn from(value: &WebAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderActivatedEventArgs> for WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderActivatedEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAccountProviderActivatedEventArgs> for &WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAccountProviderActivatedEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WebAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<WebAccountProviderActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgsWithUser> for &WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebAccountProviderActivatedEventArgs {}
unsafe impl ::core::marker::Sync for WebAccountProviderActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WebAuthenticationBrokerContinuationEventArgs(pub ::windows::core::IInspectable);
impl WebAuthenticationBrokerContinuationEventArgs {
    #[cfg(feature = "Security_Authentication_Web")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Security_Authentication_Web`*"]
    pub fn WebAuthenticationResult(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Authentication::Web::WebAuthenticationResult>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationKind> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::core::Result<SplashScreen> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for WebAuthenticationBrokerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.WebAuthenticationBrokerContinuationEventArgs;{75dda3d4-7714-453d-b7ff-b95e3a1709da})");
}
unsafe impl ::windows::core::Interface for WebAuthenticationBrokerContinuationEventArgs {
    type Vtable = IWebAuthenticationBrokerContinuationEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75dda3d4_7714_453d_b7ff_b95e3a1709da);
}
impl ::windows::core::RuntimeName for WebAuthenticationBrokerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WebAuthenticationBrokerContinuationEventArgs";
}
impl ::core::convert::From<WebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebAuthenticationBrokerContinuationEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebAuthenticationBrokerContinuationEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebAuthenticationBrokerContinuationEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebAuthenticationBrokerContinuationEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<WebAuthenticationBrokerContinuationEventArgs> for IWebAuthenticationBrokerContinuationEventArgs {
    fn from(value: WebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAuthenticationBrokerContinuationEventArgs> for IWebAuthenticationBrokerContinuationEventArgs {
    fn from(value: &WebAuthenticationBrokerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAuthenticationBrokerContinuationEventArgs> for WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAuthenticationBrokerContinuationEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebAuthenticationBrokerContinuationEventArgs> for &WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IWebAuthenticationBrokerContinuationEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IActivatedEventArgs> for &WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<WebAuthenticationBrokerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAuthenticationBrokerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IContinuationActivatedEventArgs> for &WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebAuthenticationBrokerContinuationEventArgs {}
unsafe impl ::core::marker::Sync for WebAuthenticationBrokerContinuationEventArgs {}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct WebUISearchActivatedEventsContract(pub u8);
