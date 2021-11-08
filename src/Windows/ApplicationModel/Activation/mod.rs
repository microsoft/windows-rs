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
unsafe impl ::windows::runtime::Abi for ActivationKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ActivationKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Activation.ActivationKind;i4)");
}
impl ::windows::runtime::DefaultType for ActivationKind {
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
unsafe impl ::windows::runtime::Abi for ApplicationExecutionState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ApplicationExecutionState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Activation.ApplicationExecutionState;i4)");
}
impl ::windows::runtime::DefaultType for ApplicationExecutionState {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentsProviderAddAppointmentActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl AppointmentsProviderAddAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Appointments_AppointmentsProvider`*"]
    pub fn AddAppointmentOperation(&self) -> ::windows::runtime::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Appointments::AppointmentsProvider::AddAppointmentOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppointmentsProviderAddAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderAddAppointmentActivatedEventArgs;{a2861367-cee5-4e4d-9ed7-41c34ec18b02})");
}
unsafe impl ::windows::runtime::Interface for AppointmentsProviderAddAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderAddAppointmentActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2726695783, 52965, 20045, [158, 215, 65, 195, 78, 193, 139, 2]);
}
impl ::windows::runtime::RuntimeName for AppointmentsProviderAddAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderAddAppointmentActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderAddAppointmentActivatedEventArgs> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderAddAppointmentActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderAddAppointmentActivatedEventArgs> for &AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderAddAppointmentActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &AppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderAddAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderAddAppointmentActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentsProviderRemoveAppointmentActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Appointments_AppointmentsProvider`*"]
    pub fn RemoveAppointmentOperation(&self) -> ::windows::runtime::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderRemoveAppointmentActivatedEventArgs;{751f3ab8-0b8e-451c-9f15-966e699bac25})");
}
unsafe impl ::windows::runtime::Interface for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderRemoveAppointmentActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1964980920, 2958, 17692, [159, 21, 150, 110, 105, 155, 172, 37]);
}
impl ::windows::runtime::RuntimeName for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderRemoveAppointmentActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderRemoveAppointmentActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for &AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderRemoveAppointmentActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &AppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderRemoveAppointmentActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentsProviderReplaceAppointmentActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Appointments_AppointmentsProvider`*"]
    pub fn ReplaceAppointmentOperation(&self) -> ::windows::runtime::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderReplaceAppointmentActivatedEventArgs;{1551b7d4-a981-4067-8a62-0524e4ade121})");
}
unsafe impl ::windows::runtime::Interface for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderReplaceAppointmentActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(357677012, 43393, 16487, [138, 98, 5, 36, 228, 173, 225, 33]);
}
impl ::windows::runtime::RuntimeName for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderReplaceAppointmentActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderReplaceAppointmentActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for &AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderReplaceAppointmentActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &AppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderReplaceAppointmentActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentsProviderShowAppointmentDetailsActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn InstanceStartDate(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn LocalId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn RoamingId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderShowAppointmentDetailsActivatedEventArgs;{3958f065-9841-4ca5-999b-885198b9ef2a})");
}
unsafe impl ::windows::runtime::Interface for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(962130021, 38977, 19621, [153, 155, 136, 81, 152, 185, 239, 42]);
}
impl ::windows::runtime::RuntimeName for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AppointmentsProviderShowTimeFrameActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl AppointmentsProviderShowTimeFrameActivatedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn TimeToShow(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.AppointmentsProviderShowTimeFrameActivatedEventArgs;{9baeaba6-0e0b-49aa-babc-12b1dc774986})");
}
unsafe impl ::windows::runtime::Interface for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowTimeFrameActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2611915686, 3595, 18858, [186, 188, 18, 177, 220, 119, 73, 134]);
}
impl ::windows::runtime::RuntimeName for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.AppointmentsProviderShowTimeFrameActivatedEventArgs";
}
impl ::core::convert::From<AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderShowTimeFrameActivatedEventArgs> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderShowTimeFrameActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderShowTimeFrameActivatedEventArgs> for &AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderShowTimeFrameActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<AppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&AppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &AppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &AppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentsProviderShowTimeFrameActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BackgroundActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl BackgroundActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Background")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Background`*"]
    pub fn TaskInstance(&self) -> ::windows::runtime::Result<super::Background::IBackgroundTaskInstance> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Background::IBackgroundTaskInstance>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BackgroundActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.BackgroundActivatedEventArgs;{ab14bee0-e760-440e-a91c-44796de3a92d})");
}
unsafe impl ::windows::runtime::Interface for BackgroundActivatedEventArgs {
    type Vtable = IBackgroundActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2870263520, 59232, 17422, [169, 28, 68, 121, 109, 227, 169, 45]);
}
impl ::windows::runtime::RuntimeName for BackgroundActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.BackgroundActivatedEventArgs";
}
impl ::core::convert::From<BackgroundActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: BackgroundActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BackgroundActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &BackgroundActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BackgroundActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: BackgroundActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BackgroundActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &BackgroundActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundActivatedEventArgs> for BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBackgroundActivatedEventArgs> for &BackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBackgroundActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BackgroundActivatedEventArgs {}
unsafe impl ::core::marker::Sync for BackgroundActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BarcodeScannerPreviewActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl BarcodeScannerPreviewActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ConnectionId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerPreviewActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.BarcodeScannerPreviewActivatedEventArgs;{6772797c-99bf-4349-af22-e4123560371c})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerPreviewActivatedEventArgs {
    type Vtable = IBarcodeScannerPreviewActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1735555452, 39359, 17225, [175, 34, 228, 18, 53, 96, 55, 28]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerPreviewActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.BarcodeScannerPreviewActivatedEventArgs";
}
impl ::core::convert::From<BarcodeScannerPreviewActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: BarcodeScannerPreviewActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BarcodeScannerPreviewActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &BarcodeScannerPreviewActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BarcodeScannerPreviewActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: BarcodeScannerPreviewActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BarcodeScannerPreviewActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &BarcodeScannerPreviewActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<BarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: BarcodeScannerPreviewActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &BarcodeScannerPreviewActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<BarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: BarcodeScannerPreviewActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &BarcodeScannerPreviewActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
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
impl<'a> ::windows::runtime::IntoParam<'a, IBarcodeScannerPreviewActivatedEventArgs> for BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBarcodeScannerPreviewActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IBarcodeScannerPreviewActivatedEventArgs> for &BarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IBarcodeScannerPreviewActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerPreviewActivatedEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerPreviewActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CachedFileUpdaterActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl CachedFileUpdaterActivatedEventArgs {
    #[cfg(feature = "Storage_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage_Provider`*"]
    pub fn CachedFileUpdaterUI(&self) -> ::windows::runtime::Result<super::super::Storage::Provider::CachedFileUpdaterUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Provider::CachedFileUpdaterUI>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CachedFileUpdaterActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CachedFileUpdaterActivatedEventArgs;{d06eb1c7-3805-4ecb-b757-6cf15e26fef3})");
}
unsafe impl ::windows::runtime::Interface for CachedFileUpdaterActivatedEventArgs {
    type Vtable = ICachedFileUpdaterActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3496915399, 14341, 20171, [183, 87, 108, 241, 94, 38, 254, 243]);
}
impl ::windows::runtime::RuntimeName for CachedFileUpdaterActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CachedFileUpdaterActivatedEventArgs";
}
impl ::core::convert::From<CachedFileUpdaterActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CachedFileUpdaterActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CachedFileUpdaterActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CachedFileUpdaterActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CachedFileUpdaterActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CachedFileUpdaterActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CachedFileUpdaterActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CachedFileUpdaterActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ICachedFileUpdaterActivatedEventArgs> for CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICachedFileUpdaterActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICachedFileUpdaterActivatedEventArgs> for &CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICachedFileUpdaterActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CachedFileUpdaterActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CachedFileUpdaterActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CachedFileUpdaterActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CachedFileUpdaterActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<CachedFileUpdaterActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CachedFileUpdaterActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CachedFileUpdaterActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CachedFileUpdaterActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &CachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for CachedFileUpdaterActivatedEventArgs {}
unsafe impl ::core::marker::Sync for CachedFileUpdaterActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CameraSettingsActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl CameraSettingsActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn VideoDeviceController(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn VideoDeviceExtension(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CameraSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CameraSettingsActivatedEventArgs;{fb67a508-2dad-490a-9170-dca036eb114b})");
}
unsafe impl ::windows::runtime::Interface for CameraSettingsActivatedEventArgs {
    type Vtable = ICameraSettingsActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4217873672, 11693, 18698, [145, 112, 220, 160, 54, 235, 17, 75]);
}
impl ::windows::runtime::RuntimeName for CameraSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CameraSettingsActivatedEventArgs";
}
impl ::core::convert::From<CameraSettingsActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CameraSettingsActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CameraSettingsActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CameraSettingsActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CameraSettingsActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CameraSettingsActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CameraSettingsActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CameraSettingsActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ICameraSettingsActivatedEventArgs> for CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICameraSettingsActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICameraSettingsActivatedEventArgs> for &CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICameraSettingsActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CameraSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CameraSettingsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CameraSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CameraSettingsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &CameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for CameraSettingsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for CameraSettingsActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CommandLineActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl CommandLineActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Operation(&self) -> ::windows::runtime::Result<CommandLineActivationOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CommandLineActivationOperation>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CommandLineActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CommandLineActivatedEventArgs;{4506472c-006a-48eb-8afb-d07ab25e3366})");
}
unsafe impl ::windows::runtime::Interface for CommandLineActivatedEventArgs {
    type Vtable = ICommandLineActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1158039340, 106, 18667, [138, 251, 208, 122, 178, 94, 51, 102]);
}
impl ::windows::runtime::RuntimeName for CommandLineActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CommandLineActivatedEventArgs";
}
impl ::core::convert::From<CommandLineActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CommandLineActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CommandLineActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CommandLineActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CommandLineActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CommandLineActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CommandLineActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CommandLineActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<CommandLineActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CommandLineActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CommandLineActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CommandLineActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<CommandLineActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CommandLineActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CommandLineActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CommandLineActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
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
impl<'a> ::windows::runtime::IntoParam<'a, ICommandLineActivatedEventArgs> for CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommandLineActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICommandLineActivatedEventArgs> for &CommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommandLineActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CommandLineActivatedEventArgs {}
unsafe impl ::core::marker::Sync for CommandLineActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CommandLineActivationOperation(pub ::windows::runtime::IInspectable);
impl CommandLineActivationOperation {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Arguments(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CurrentDirectoryPath(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SetExitCode(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ExitCode(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CommandLineActivationOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.CommandLineActivationOperation;{994b2841-c59e-4f69-bcfd-b61ed4e622eb})");
}
unsafe impl ::windows::runtime::Interface for CommandLineActivationOperation {
    type Vtable = ICommandLineActivationOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2571839553, 50590, 20329, [188, 253, 182, 30, 212, 230, 34, 235]);
}
impl ::windows::runtime::RuntimeName for CommandLineActivationOperation {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.CommandLineActivationOperation";
}
impl ::core::convert::From<CommandLineActivationOperation> for ::windows::runtime::IUnknown {
    fn from(value: CommandLineActivationOperation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CommandLineActivationOperation> for ::windows::runtime::IUnknown {
    fn from(value: &CommandLineActivationOperation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CommandLineActivationOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a CommandLineActivationOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CommandLineActivationOperation> for ::windows::runtime::IInspectable {
    fn from(value: CommandLineActivationOperation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CommandLineActivationOperation> for ::windows::runtime::IInspectable {
    fn from(value: &CommandLineActivationOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CommandLineActivationOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CommandLineActivationOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
pub struct ContactCallActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl ContactCallActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceUserId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::runtime::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactCallActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactCallActivatedEventArgs;{c2df14c7-30eb-41c6-b3bc-5b1694f9dab3})");
}
unsafe impl ::windows::runtime::Interface for ContactCallActivatedEventArgs {
    type Vtable = IContactCallActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3269399751, 12523, 16838, [179, 188, 91, 22, 148, 249, 218, 179]);
}
impl ::windows::runtime::RuntimeName for ContactCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactCallActivatedEventArgs";
}
impl ::core::convert::From<ContactCallActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ContactCallActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactCallActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ContactCallActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactCallActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ContactCallActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactCallActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ContactCallActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IContactCallActivatedEventArgs> for ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactCallActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactCallActivatedEventArgs> for &ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactCallActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ContactCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ContactCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ContactCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ContactCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactActivatedEventArgs> for ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactActivatedEventArgs> for &ContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactCallActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactMapActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl ContactMapActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Address(&self) -> ::windows::runtime::Result<super::Contacts::ContactAddress> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::ContactAddress>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::runtime::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactMapActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactMapActivatedEventArgs;{b32bf870-eee7-4ad2-aaf1-a87effcf00a4})");
}
unsafe impl ::windows::runtime::Interface for ContactMapActivatedEventArgs {
    type Vtable = IContactMapActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3006003312, 61159, 19154, [170, 241, 168, 126, 255, 207, 0, 164]);
}
impl ::windows::runtime::RuntimeName for ContactMapActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactMapActivatedEventArgs";
}
impl ::core::convert::From<ContactMapActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ContactMapActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactMapActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ContactMapActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactMapActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ContactMapActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactMapActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ContactMapActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IContactMapActivatedEventArgs> for ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactMapActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactMapActivatedEventArgs> for &ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactMapActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactMapActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ContactMapActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMapActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ContactMapActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactMapActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ContactMapActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMapActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ContactMapActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactActivatedEventArgs> for ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactActivatedEventArgs> for &ContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactMapActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactMapActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactMessageActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl ContactMessageActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceUserId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::runtime::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactMessageActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactMessageActivatedEventArgs;{de598db2-0e03-43b0-bf56-bcc40b3162df})");
}
unsafe impl ::windows::runtime::Interface for ContactMessageActivatedEventArgs {
    type Vtable = IContactMessageActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3730410930, 3587, 17328, [191, 86, 188, 196, 11, 49, 98, 223]);
}
impl ::windows::runtime::RuntimeName for ContactMessageActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactMessageActivatedEventArgs";
}
impl ::core::convert::From<ContactMessageActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ContactMessageActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactMessageActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ContactMessageActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactMessageActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ContactMessageActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactMessageActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ContactMessageActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IContactMessageActivatedEventArgs> for ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactMessageActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactMessageActivatedEventArgs> for &ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactMessageActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactMessageActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ContactMessageActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMessageActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ContactMessageActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactMessageActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ContactMessageActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactMessageActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ContactMessageActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactActivatedEventArgs> for ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactActivatedEventArgs> for &ContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactMessageActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactMessageActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactPanelActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl ContactPanelActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn ContactPanel(&self) -> ::windows::runtime::Result<super::Contacts::ContactPanel> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::ContactPanel>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::runtime::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactPanelActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactPanelActivatedEventArgs;{52bb63e4-d3d4-4b63-8051-4af2082cab80})");
}
unsafe impl ::windows::runtime::Interface for ContactPanelActivatedEventArgs {
    type Vtable = IContactPanelActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1388012516, 54228, 19299, [128, 81, 74, 242, 8, 44, 171, 128]);
}
impl ::windows::runtime::RuntimeName for ContactPanelActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPanelActivatedEventArgs";
}
impl ::core::convert::From<ContactPanelActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ContactPanelActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactPanelActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ContactPanelActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactPanelActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ContactPanelActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactPanelActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ContactPanelActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ContactPanelActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ContactPanelActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPanelActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ContactPanelActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactPanelActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ContactPanelActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPanelActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ContactPanelActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
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
impl<'a> ::windows::runtime::IntoParam<'a, IContactPanelActivatedEventArgs> for ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactPanelActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactPanelActivatedEventArgs> for &ContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactPanelActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContactPanelActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactPanelActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactPickerActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl ContactPickerActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts_Provider`*"]
    pub fn ContactPickerUI(&self) -> ::windows::runtime::Result<super::Contacts::Provider::ContactPickerUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Provider::ContactPickerUI>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactPickerActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactPickerActivatedEventArgs;{ce57aae7-6449-45a7-971f-d113be7a8936})");
}
unsafe impl ::windows::runtime::Interface for ContactPickerActivatedEventArgs {
    type Vtable = IContactPickerActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3461851879, 25673, 17831, [151, 31, 209, 19, 190, 122, 137, 54]);
}
impl ::windows::runtime::RuntimeName for ContactPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPickerActivatedEventArgs";
}
impl ::core::convert::From<ContactPickerActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ContactPickerActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactPickerActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ContactPickerActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactPickerActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ContactPickerActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactPickerActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ContactPickerActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IContactPickerActivatedEventArgs> for ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactPickerActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactPickerActivatedEventArgs> for &ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactPickerActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ContactPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ContactPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &ContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactPickerActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactPickerActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactPostActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl ContactPostActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceUserId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::runtime::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactPostActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactPostActivatedEventArgs;{b35a3c67-f1e7-4655-ad6e-4857588f552f})");
}
unsafe impl ::windows::runtime::Interface for ContactPostActivatedEventArgs {
    type Vtable = IContactPostActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3009035367, 61927, 18005, [173, 110, 72, 87, 88, 143, 85, 47]);
}
impl ::windows::runtime::RuntimeName for ContactPostActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactPostActivatedEventArgs";
}
impl ::core::convert::From<ContactPostActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ContactPostActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactPostActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ContactPostActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactPostActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ContactPostActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactPostActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ContactPostActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IContactPostActivatedEventArgs> for ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactPostActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactPostActivatedEventArgs> for &ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactPostActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactPostActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ContactPostActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPostActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ContactPostActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactPostActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ContactPostActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactPostActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ContactPostActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactActivatedEventArgs> for ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactActivatedEventArgs> for &ContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactPostActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactPostActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactVideoCallActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl ContactVideoCallActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceUserId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::runtime::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactVideoCallActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ContactVideoCallActivatedEventArgs;{61079db8-e3e7-4b4f-858d-5c63a96ef684})");
}
unsafe impl ::windows::runtime::Interface for ContactVideoCallActivatedEventArgs {
    type Vtable = IContactVideoCallActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1627889080, 58343, 19279, [133, 141, 92, 99, 169, 110, 246, 132]);
}
impl ::windows::runtime::RuntimeName for ContactVideoCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ContactVideoCallActivatedEventArgs";
}
impl ::core::convert::From<ContactVideoCallActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ContactVideoCallActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactVideoCallActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ContactVideoCallActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactVideoCallActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ContactVideoCallActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactVideoCallActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ContactVideoCallActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IContactVideoCallActivatedEventArgs> for ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactVideoCallActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactVideoCallActivatedEventArgs> for &ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactVideoCallActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ContactVideoCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ContactVideoCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactVideoCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ContactVideoCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<ContactVideoCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ContactVideoCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ContactVideoCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ContactVideoCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactActivatedEventArgs> for ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactActivatedEventArgs> for &ContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for ContactVideoCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ContactVideoCallActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeviceActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl DeviceActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn DeviceInformationId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `UI_ViewManagement`*"]
    pub fn ViewSwitcher(&self) -> ::windows::runtime::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::runtime::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeviceActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.DeviceActivatedEventArgs;{cd50b9a9-ce10-44d2-8234-c355a073ef33})");
}
unsafe impl ::windows::runtime::Interface for DeviceActivatedEventArgs {
    type Vtable = IDeviceActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3444619689, 52752, 17618, [130, 52, 195, 85, 160, 115, 239, 51]);
}
impl ::windows::runtime::RuntimeName for DeviceActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DeviceActivatedEventArgs";
}
impl ::core::convert::From<DeviceActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: DeviceActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DeviceActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &DeviceActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DeviceActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: DeviceActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DeviceActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &DeviceActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IDeviceActivatedEventArgs> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDeviceActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDeviceActivatedEventArgs> for &DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDeviceActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DeviceActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DeviceActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<DeviceActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DeviceActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IApplicationViewActivatedEventArgs> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IApplicationViewActivatedEventArgs> for &DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<DeviceActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DeviceActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IViewSwitcherProvider> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IViewSwitcherProvider> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IViewSwitcherProvider> for &DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<DeviceActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DeviceActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DeviceActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DeviceActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &DeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for DeviceActivatedEventArgs {}
unsafe impl ::core::marker::Sync for DeviceActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DevicePairingActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl DevicePairingActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Devices_Enumeration`*"]
    pub fn DeviceInformation(&self) -> ::windows::runtime::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DevicePairingActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.DevicePairingActivatedEventArgs;{eba0d1e4-ecc6-4148-94ed-f4b37ec05b3e})");
}
unsafe impl ::windows::runtime::Interface for DevicePairingActivatedEventArgs {
    type Vtable = IDevicePairingActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3953185252, 60614, 16712, [148, 237, 244, 179, 126, 192, 91, 62]);
}
impl ::windows::runtime::RuntimeName for DevicePairingActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DevicePairingActivatedEventArgs";
}
impl ::core::convert::From<DevicePairingActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: DevicePairingActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DevicePairingActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &DevicePairingActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DevicePairingActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: DevicePairingActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DevicePairingActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &DevicePairingActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<DevicePairingActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DevicePairingActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DevicePairingActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DevicePairingActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
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
impl<'a> ::windows::runtime::IntoParam<'a, IDevicePairingActivatedEventArgs> for DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDevicePairingActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDevicePairingActivatedEventArgs> for &DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDevicePairingActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DevicePairingActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DevicePairingActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DevicePairingActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DevicePairingActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &DevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for DevicePairingActivatedEventArgs {}
unsafe impl ::core::marker::Sync for DevicePairingActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DialReceiverActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl DialReceiverActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn AppName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Arguments(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn TileId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `UI_ViewManagement`*"]
    pub fn ViewSwitcher(&self) -> ::windows::runtime::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::runtime::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DialReceiverActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.DialReceiverActivatedEventArgs;{fb777ed7-85ee-456e-a44d-85d730e70aed})");
}
unsafe impl ::windows::runtime::Interface for DialReceiverActivatedEventArgs {
    type Vtable = IDialReceiverActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4218912471, 34286, 17774, [164, 77, 133, 215, 48, 231, 10, 237]);
}
impl ::windows::runtime::RuntimeName for DialReceiverActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.DialReceiverActivatedEventArgs";
}
impl ::core::convert::From<DialReceiverActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: DialReceiverActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DialReceiverActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &DialReceiverActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DialReceiverActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: DialReceiverActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DialReceiverActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &DialReceiverActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IDialReceiverActivatedEventArgs> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDialReceiverActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDialReceiverActivatedEventArgs> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDialReceiverActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IApplicationViewActivatedEventArgs> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IApplicationViewActivatedEventArgs> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILaunchActivatedEventArgs> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILaunchActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILaunchActivatedEventArgs> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IViewSwitcherProvider> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IViewSwitcherProvider> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IViewSwitcherProvider> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<DialReceiverActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&DialReceiverActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &DialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for DialReceiverActivatedEventArgs {}
unsafe impl ::core::marker::Sync for DialReceiverActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FileActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl FileActivatedEventArgs {
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`, `Storage`*"]
    pub fn Files(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CallerPackageFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IFileActivatedEventArgsWithCallerPackageFamilyName>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage_Search`*"]
    pub fn NeighboringFilesQuery(&self) -> ::windows::runtime::Result<super::super::Storage::Search::StorageFileQueryResult> {
        let this = &::windows::runtime::Interface::cast::<IFileActivatedEventArgsWithNeighboringFiles>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Search::StorageFileQueryResult>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `UI_ViewManagement`*"]
    pub fn ViewSwitcher(&self) -> ::windows::runtime::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::runtime::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FileActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileActivatedEventArgs;{bb2afc33-93b1-42ed-8b26-236dd9c78496})");
}
unsafe impl ::windows::runtime::Interface for FileActivatedEventArgs {
    type Vtable = IFileActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3140156467, 37809, 17133, [139, 38, 35, 109, 217, 199, 132, 150]);
}
impl ::windows::runtime::RuntimeName for FileActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileActivatedEventArgs";
}
impl ::core::convert::From<FileActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: FileActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FileActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &FileActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a FileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FileActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: FileActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FileActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &FileActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IFileActivatedEventArgs> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileActivatedEventArgs> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IApplicationViewActivatedEventArgs> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IApplicationViewActivatedEventArgs> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IFileActivatedEventArgsWithCallerPackageFamilyName {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IFileActivatedEventArgsWithCallerPackageFamilyName {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileActivatedEventArgsWithCallerPackageFamilyName> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileActivatedEventArgsWithCallerPackageFamilyName> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileActivatedEventArgsWithCallerPackageFamilyName> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileActivatedEventArgsWithCallerPackageFamilyName> {
        ::core::convert::TryInto::<IFileActivatedEventArgsWithCallerPackageFamilyName>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IFileActivatedEventArgsWithNeighboringFiles {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IFileActivatedEventArgsWithNeighboringFiles {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileActivatedEventArgsWithNeighboringFiles> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileActivatedEventArgsWithNeighboringFiles> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileActivatedEventArgsWithNeighboringFiles> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileActivatedEventArgsWithNeighboringFiles> {
        ::core::convert::TryInto::<IFileActivatedEventArgsWithNeighboringFiles>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IViewSwitcherProvider> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IViewSwitcherProvider> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IViewSwitcherProvider> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<FileActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for FileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &FileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for FileActivatedEventArgs {}
unsafe impl ::core::marker::Sync for FileActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FileOpenPickerActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl FileOpenPickerActivatedEventArgs {
    #[cfg(feature = "Storage_Pickers_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage_Pickers_Provider`*"]
    pub fn FileOpenPickerUI(&self) -> ::windows::runtime::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Pickers::Provider::FileOpenPickerUI>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CallerPackageFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IFileOpenPickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FileOpenPickerActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileOpenPickerActivatedEventArgs;{72827082-5525-4bf2-bc09-1f5095d4964d})");
}
unsafe impl ::windows::runtime::Interface for FileOpenPickerActivatedEventArgs {
    type Vtable = IFileOpenPickerActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1921151106, 21797, 19442, [188, 9, 31, 80, 149, 212, 150, 77]);
}
impl ::windows::runtime::RuntimeName for FileOpenPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileOpenPickerActivatedEventArgs";
}
impl ::core::convert::From<FileOpenPickerActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: FileOpenPickerActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FileOpenPickerActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &FileOpenPickerActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FileOpenPickerActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: FileOpenPickerActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FileOpenPickerActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &FileOpenPickerActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IFileOpenPickerActivatedEventArgs> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileOpenPickerActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileOpenPickerActivatedEventArgs> for &FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileOpenPickerActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FileOpenPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileOpenPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileOpenPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<FileOpenPickerActivatedEventArgs> for IFileOpenPickerActivatedEventArgs2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileOpenPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerActivatedEventArgs> for IFileOpenPickerActivatedEventArgs2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileOpenPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileOpenPickerActivatedEventArgs2> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileOpenPickerActivatedEventArgs2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileOpenPickerActivatedEventArgs2> for &FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileOpenPickerActivatedEventArgs2> {
        ::core::convert::TryInto::<IFileOpenPickerActivatedEventArgs2>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<FileOpenPickerActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileOpenPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileOpenPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &FileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for FileOpenPickerActivatedEventArgs {}
unsafe impl ::core::marker::Sync for FileOpenPickerActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FileOpenPickerContinuationEventArgs(pub ::windows::runtime::IInspectable);
impl FileOpenPickerContinuationEventArgs {
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`, `Storage`*"]
    pub fn Files(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn ContinuationData(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::runtime::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FileOpenPickerContinuationEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileOpenPickerContinuationEventArgs;{f0fa3f3a-d4e8-4ad3-9c34-2308f32fcec9})");
}
unsafe impl ::windows::runtime::Interface for FileOpenPickerContinuationEventArgs {
    type Vtable = IFileOpenPickerContinuationEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4042932026, 54504, 19155, [156, 52, 35, 8, 243, 47, 206, 201]);
}
impl ::windows::runtime::RuntimeName for FileOpenPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileOpenPickerContinuationEventArgs";
}
impl ::core::convert::From<FileOpenPickerContinuationEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: FileOpenPickerContinuationEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FileOpenPickerContinuationEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &FileOpenPickerContinuationEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FileOpenPickerContinuationEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: FileOpenPickerContinuationEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FileOpenPickerContinuationEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &FileOpenPickerContinuationEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IFileOpenPickerContinuationEventArgs> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileOpenPickerContinuationEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileOpenPickerContinuationEventArgs> for &FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileOpenPickerContinuationEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FileOpenPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileOpenPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileOpenPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<FileOpenPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileOpenPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileOpenPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContinuationActivatedEventArgs> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContinuationActivatedEventArgs> for &FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<FileOpenPickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileOpenPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileOpenPickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileOpenPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &FileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for FileOpenPickerContinuationEventArgs {}
unsafe impl ::core::marker::Sync for FileOpenPickerContinuationEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FileSavePickerActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl FileSavePickerActivatedEventArgs {
    #[cfg(feature = "Storage_Pickers_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage_Pickers_Provider`*"]
    pub fn FileSavePickerUI(&self) -> ::windows::runtime::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Pickers::Provider::FileSavePickerUI>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CallerPackageFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn EnterpriseId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FileSavePickerActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileSavePickerActivatedEventArgs;{81c19cf1-74e6-4387-82eb-bb8fd64b4346})");
}
unsafe impl ::windows::runtime::Interface for FileSavePickerActivatedEventArgs {
    type Vtable = IFileSavePickerActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2176949489, 29926, 17287, [130, 235, 187, 143, 214, 75, 67, 70]);
}
impl ::windows::runtime::RuntimeName for FileSavePickerActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileSavePickerActivatedEventArgs";
}
impl ::core::convert::From<FileSavePickerActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: FileSavePickerActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FileSavePickerActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &FileSavePickerActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FileSavePickerActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: FileSavePickerActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FileSavePickerActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &FileSavePickerActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IFileSavePickerActivatedEventArgs> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileSavePickerActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileSavePickerActivatedEventArgs> for &FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileSavePickerActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FileSavePickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileSavePickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileSavePickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<FileSavePickerActivatedEventArgs> for IFileSavePickerActivatedEventArgs2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileSavePickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerActivatedEventArgs> for IFileSavePickerActivatedEventArgs2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileSavePickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileSavePickerActivatedEventArgs2> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileSavePickerActivatedEventArgs2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileSavePickerActivatedEventArgs2> for &FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileSavePickerActivatedEventArgs2> {
        ::core::convert::TryInto::<IFileSavePickerActivatedEventArgs2>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<FileSavePickerActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileSavePickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileSavePickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &FileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for FileSavePickerActivatedEventArgs {}
unsafe impl ::core::marker::Sync for FileSavePickerActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FileSavePickerContinuationEventArgs(pub ::windows::runtime::IInspectable);
impl FileSavePickerContinuationEventArgs {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage`*"]
    pub fn File(&self) -> ::windows::runtime::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn ContinuationData(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::runtime::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FileSavePickerContinuationEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FileSavePickerContinuationEventArgs;{2c846fe1-3bad-4f33-8c8b-e46fae824b4b})");
}
unsafe impl ::windows::runtime::Interface for FileSavePickerContinuationEventArgs {
    type Vtable = IFileSavePickerContinuationEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(746876897, 15277, 20275, [140, 139, 228, 111, 174, 130, 75, 75]);
}
impl ::windows::runtime::RuntimeName for FileSavePickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FileSavePickerContinuationEventArgs";
}
impl ::core::convert::From<FileSavePickerContinuationEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: FileSavePickerContinuationEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FileSavePickerContinuationEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &FileSavePickerContinuationEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FileSavePickerContinuationEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: FileSavePickerContinuationEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FileSavePickerContinuationEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &FileSavePickerContinuationEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IFileSavePickerContinuationEventArgs> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileSavePickerContinuationEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileSavePickerContinuationEventArgs> for &FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileSavePickerContinuationEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FileSavePickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileSavePickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileSavePickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<FileSavePickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileSavePickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileSavePickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContinuationActivatedEventArgs> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContinuationActivatedEventArgs> for &FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<FileSavePickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileSavePickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileSavePickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileSavePickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &FileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for FileSavePickerContinuationEventArgs {}
unsafe impl ::core::marker::Sync for FileSavePickerContinuationEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct FolderPickerContinuationEventArgs(pub ::windows::runtime::IInspectable);
impl FolderPickerContinuationEventArgs {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage`*"]
    pub fn Folder(&self) -> ::windows::runtime::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn ContinuationData(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::runtime::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FolderPickerContinuationEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.FolderPickerContinuationEventArgs;{51882366-9f4b-498f-beb0-42684f6e1c29})");
}
unsafe impl ::windows::runtime::Interface for FolderPickerContinuationEventArgs {
    type Vtable = IFolderPickerContinuationEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1367876454, 40779, 18831, [190, 176, 66, 104, 79, 110, 28, 41]);
}
impl ::windows::runtime::RuntimeName for FolderPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.FolderPickerContinuationEventArgs";
}
impl ::core::convert::From<FolderPickerContinuationEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: FolderPickerContinuationEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FolderPickerContinuationEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &FolderPickerContinuationEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FolderPickerContinuationEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: FolderPickerContinuationEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FolderPickerContinuationEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &FolderPickerContinuationEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IFolderPickerContinuationEventArgs> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFolderPickerContinuationEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFolderPickerContinuationEventArgs> for &FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFolderPickerContinuationEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FolderPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FolderPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FolderPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<FolderPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FolderPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FolderPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContinuationActivatedEventArgs> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContinuationActivatedEventArgs> for &FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<FolderPickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FolderPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderPickerContinuationEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FolderPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &FolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for FolderPickerContinuationEventArgs {}
unsafe impl ::core::marker::Sync for FolderPickerContinuationEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IActivatedEventArgs {
    type Vtable = IActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3479508755, 52488, 20440, [182, 151, 162, 129, 182, 84, 78, 46]);
}
impl IActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = self;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{cf651713-cd08-4fd8-b697-a281b6544e2e}");
}
impl ::core::convert::From<IActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ActivationKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ApplicationExecutionState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IActivatedEventArgsWithUser(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IActivatedEventArgsWithUser {
    type Vtable = IActivatedEventArgsWithUser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(485530526, 39266, 18742, [128, 255, 175, 200, 232, 174, 92, 140]);
}
impl IActivatedEventArgsWithUser {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IActivatedEventArgsWithUser {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{1cf09b9e-9962-4936-80ff-afc8e8ae5c8c}");
}
impl ::core::convert::From<IActivatedEventArgsWithUser> for ::windows::runtime::IUnknown {
    fn from(value: IActivatedEventArgsWithUser) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IActivatedEventArgsWithUser> for ::windows::runtime::IUnknown {
    fn from(value: &IActivatedEventArgsWithUser) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IActivatedEventArgsWithUser> for ::windows::runtime::IInspectable {
    fn from(value: IActivatedEventArgsWithUser) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IActivatedEventArgsWithUser> for ::windows::runtime::IInspectable {
    fn from(value: &IActivatedEventArgsWithUser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IActivatedEventArgsWithUser> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IActivatedEventArgsWithUser) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IActivatedEventArgsWithUser> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IActivatedEventArgsWithUser) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IActivatedEventArgsWithUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedEventArgsWithUser_abi(
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IApplicationViewActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IApplicationViewActivatedEventArgs {
    type Vtable = IApplicationViewActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2467098443, 47145, 16636, [136, 244, 133, 19, 232, 166, 71, 56]);
}
impl IApplicationViewActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IApplicationViewActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{930cef4b-b829-40fc-88f4-8513e8a64738}");
}
impl ::core::convert::From<IApplicationViewActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IApplicationViewActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IApplicationViewActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IApplicationViewActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IApplicationViewActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IApplicationViewActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IApplicationViewActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IApplicationViewActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IApplicationViewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IApplicationViewActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IApplicationViewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IApplicationViewActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IApplicationViewActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IAppointmentsProviderActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppointmentsProviderActivatedEventArgs {
    type Vtable = IAppointmentsProviderActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(862241797, 37692, 20093, [160, 52, 80, 15, 184, 220, 217, 243]);
}
impl IAppointmentsProviderActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IAppointmentsProviderActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{3364c405-933c-4e7d-a034-500fb8dcd9f3}");
}
impl ::core::convert::From<IAppointmentsProviderActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IAppointmentsProviderActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAppointmentsProviderActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IAppointmentsProviderActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAppointmentsProviderActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IAppointmentsProviderActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppointmentsProviderActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IAppointmentsProviderActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IAppointmentsProviderActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IAppointmentsProviderActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IAppointmentsProviderAddAppointmentActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderAddAppointmentActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2726695783, 52965, 20045, [158, 215, 65, 195, 78, 193, 139, 2]);
}
impl IAppointmentsProviderAddAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Appointments_AppointmentsProvider`*"]
    pub fn AddAppointmentOperation(&self) -> ::windows::runtime::Result<super::Appointments::AppointmentsProvider::AddAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Appointments::AppointmentsProvider::AddAppointmentOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{a2861367-cee5-4e4d-9ed7-41c34ec18b02}");
}
impl ::core::convert::From<IAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderAddAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &IAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderAddAppointmentActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IAppointmentsProviderRemoveAppointmentActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderRemoveAppointmentActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1964980920, 2958, 17692, [159, 21, 150, 110, 105, 155, 172, 37]);
}
impl IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Appointments_AppointmentsProvider`*"]
    pub fn RemoveAppointmentOperation(&self) -> ::windows::runtime::Result<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Appointments::AppointmentsProvider::RemoveAppointmentOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{751f3ab8-0b8e-451c-9f15-966e699bac25}");
}
impl ::core::convert::From<IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderRemoveAppointmentActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IAppointmentsProviderReplaceAppointmentActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Vtable = IAppointmentsProviderReplaceAppointmentActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(357677012, 43393, 16487, [138, 98, 5, 36, 228, 173, 225, 33]);
}
impl IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Appointments_AppointmentsProvider`*"]
    pub fn ReplaceAppointmentOperation(&self) -> ::windows::runtime::Result<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Appointments::AppointmentsProvider::ReplaceAppointmentOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{1551b7d4-a981-4067-8a62-0524e4ade121}");
}
impl ::core::convert::From<IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderReplaceAppointmentActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments_AppointmentsProvider"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(962130021, 38977, 19621, [153, 155, 136, 81, 152, 185, 239, 42]);
}
impl IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn InstanceStartDate(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn LocalId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn RoamingId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{3958f065-9841-4ca5-999b-885198b9ef2a}");
}
impl ::core::convert::From<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IAppointmentsProviderShowTimeFrameActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Vtable = IAppointmentsProviderShowTimeFrameActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2611915686, 3595, 18858, [186, 188, 18, 177, 220, 119, 73, 134]);
}
impl IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn TimeToShow(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{9baeaba6-0e0b-49aa-babc-12b1dc774986}");
}
impl ::core::convert::From<IAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<IAppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAppointmentsProviderShowTimeFrameActivatedEventArgs> for IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAppointmentsProviderActivatedEventArgs> for &IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAppointmentsProviderActivatedEventArgs> {
        ::core::convert::TryInto::<IAppointmentsProviderActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderShowTimeFrameActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IBackgroundActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBackgroundActivatedEventArgs {
    type Vtable = IBackgroundActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2870263520, 59232, 17422, [169, 28, 68, 121, 109, 227, 169, 45]);
}
impl IBackgroundActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Background")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Background`*"]
    pub fn TaskInstance(&self) -> ::windows::runtime::Result<super::Background::IBackgroundTaskInstance> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Background::IBackgroundTaskInstance>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IBackgroundActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{ab14bee0-e760-440e-a91c-44796de3a92d}");
}
impl ::core::convert::From<IBackgroundActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IBackgroundActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IBackgroundActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IBackgroundActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IBackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IBackgroundActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IBackgroundActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBackgroundActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IBackgroundActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IBackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IBackgroundActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_Background")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Background"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IBarcodeScannerPreviewActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerPreviewActivatedEventArgs {
    type Vtable = IBarcodeScannerPreviewActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1735555452, 39359, 17225, [175, 34, 228, 18, 53, 96, 55, 28]);
}
impl IBarcodeScannerPreviewActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ConnectionId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IBarcodeScannerPreviewActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{6772797c-99bf-4349-af22-e4123560371c}");
}
impl ::core::convert::From<IBarcodeScannerPreviewActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IBarcodeScannerPreviewActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IBarcodeScannerPreviewActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IBarcodeScannerPreviewActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IBarcodeScannerPreviewActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IBarcodeScannerPreviewActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBarcodeScannerPreviewActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IBarcodeScannerPreviewActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IBarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IBarcodeScannerPreviewActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IBarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IBarcodeScannerPreviewActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IBarcodeScannerPreviewActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerPreviewActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct ICachedFileUpdaterActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICachedFileUpdaterActivatedEventArgs {
    type Vtable = ICachedFileUpdaterActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3496915399, 14341, 20171, [183, 87, 108, 241, 94, 38, 254, 243]);
}
impl ICachedFileUpdaterActivatedEventArgs {
    #[cfg(feature = "Storage_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage_Provider`*"]
    pub fn CachedFileUpdaterUI(&self) -> ::windows::runtime::Result<super::super::Storage::Provider::CachedFileUpdaterUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Provider::CachedFileUpdaterUI>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ICachedFileUpdaterActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{d06eb1c7-3805-4ecb-b757-6cf15e26fef3}");
}
impl ::core::convert::From<ICachedFileUpdaterActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ICachedFileUpdaterActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICachedFileUpdaterActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ICachedFileUpdaterActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICachedFileUpdaterActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ICachedFileUpdaterActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICachedFileUpdaterActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ICachedFileUpdaterActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ICachedFileUpdaterActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ICachedFileUpdaterActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ICachedFileUpdaterActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ICachedFileUpdaterActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &ICachedFileUpdaterActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Provider")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Provider"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct ICameraSettingsActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICameraSettingsActivatedEventArgs {
    type Vtable = ICameraSettingsActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4217873672, 11693, 18698, [145, 112, 220, 160, 54, 235, 17, 75]);
}
impl ICameraSettingsActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn VideoDeviceController(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn VideoDeviceExtension(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ICameraSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{fb67a508-2dad-490a-9170-dca036eb114b}");
}
impl ::core::convert::From<ICameraSettingsActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ICameraSettingsActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICameraSettingsActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ICameraSettingsActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICameraSettingsActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ICameraSettingsActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICameraSettingsActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ICameraSettingsActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ICameraSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ICameraSettingsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ICameraSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ICameraSettingsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &ICameraSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraSettingsActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct ICommandLineActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICommandLineActivatedEventArgs {
    type Vtable = ICommandLineActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1158039340, 106, 18667, [138, 251, 208, 122, 178, 94, 51, 102]);
}
impl ICommandLineActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Operation(&self) -> ::windows::runtime::Result<CommandLineActivationOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CommandLineActivationOperation>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ICommandLineActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{4506472c-006a-48eb-8afb-d07ab25e3366}");
}
impl ::core::convert::From<ICommandLineActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ICommandLineActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICommandLineActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ICommandLineActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICommandLineActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ICommandLineActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICommandLineActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ICommandLineActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ICommandLineActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ICommandLineActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ICommandLineActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ICommandLineActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &ICommandLineActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandLineActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICommandLineActivationOperation(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICommandLineActivationOperation {
    type Vtable = ICommandLineActivationOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2571839553, 50590, 20329, [188, 253, 182, 30, 212, 230, 34, 235]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommandLineActivationOperation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IContactActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactActivatedEventArgs {
    type Vtable = IContactActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3592921540, 49189, 19521, [157, 239, 241, 234, 250, 208, 117, 231]);
}
impl IContactActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IContactActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{d627a1c4-c025-4c41-9def-f1eafad075e7}");
}
impl ::core::convert::From<IContactActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IContactActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IContactActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IContactActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IContactActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IContactActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IContactActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IContactActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IContactActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IContactActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IContactActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IContactActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IContactActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IContactActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IContactCallActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactCallActivatedEventArgs {
    type Vtable = IContactCallActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3269399751, 12523, 16838, [179, 188, 91, 22, 148, 249, 218, 179]);
}
impl IContactCallActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceUserId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::runtime::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IContactCallActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{c2df14c7-30eb-41c6-b3bc-5b1694f9dab3}");
}
impl ::core::convert::From<IContactCallActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IContactCallActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IContactCallActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IContactCallActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IContactCallActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IContactCallActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactCallActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IContactCallActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IContactCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IContactCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IContactCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<IContactCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IContactCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IContactCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactActivatedEventArgs> for IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactActivatedEventArgs> for &IContactCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactCallActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IContactMapActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactMapActivatedEventArgs {
    type Vtable = IContactMapActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3006003312, 61159, 19154, [170, 241, 168, 126, 255, 207, 0, 164]);
}
impl IContactMapActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Address(&self) -> ::windows::runtime::Result<super::Contacts::ContactAddress> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::ContactAddress>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::runtime::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IContactMapActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{b32bf870-eee7-4ad2-aaf1-a87effcf00a4}");
}
impl ::core::convert::From<IContactMapActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IContactMapActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IContactMapActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IContactMapActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IContactMapActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IContactMapActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactMapActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IContactMapActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IContactMapActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IContactMapActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactMapActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IContactMapActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<IContactMapActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IContactMapActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactMapActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IContactMapActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactActivatedEventArgs> for IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactActivatedEventArgs> for &IContactMapActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactMapActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IContactMessageActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactMessageActivatedEventArgs {
    type Vtable = IContactMessageActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3730410930, 3587, 17328, [191, 86, 188, 196, 11, 49, 98, 223]);
}
impl IContactMessageActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceUserId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::runtime::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IContactMessageActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{de598db2-0e03-43b0-bf56-bcc40b3162df}");
}
impl ::core::convert::From<IContactMessageActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IContactMessageActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IContactMessageActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IContactMessageActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IContactMessageActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IContactMessageActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactMessageActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IContactMessageActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IContactMessageActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IContactMessageActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactMessageActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IContactMessageActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<IContactMessageActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IContactMessageActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactMessageActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IContactMessageActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactActivatedEventArgs> for IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactActivatedEventArgs> for &IContactMessageActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactMessageActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IContactPanelActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactPanelActivatedEventArgs {
    type Vtable = IContactPanelActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1388012516, 54228, 19299, [128, 81, 74, 242, 8, 44, 171, 128]);
}
impl IContactPanelActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn ContactPanel(&self) -> ::windows::runtime::Result<super::Contacts::ContactPanel> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::ContactPanel>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::runtime::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IContactPanelActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{52bb63e4-d3d4-4b63-8051-4af2082cab80}");
}
impl ::core::convert::From<IContactPanelActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IContactPanelActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IContactPanelActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IContactPanelActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IContactPanelActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IContactPanelActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactPanelActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IContactPanelActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IContactPanelActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPanelActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IContactPickerActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactPickerActivatedEventArgs {
    type Vtable = IContactPickerActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3461851879, 25673, 17831, [151, 31, 209, 19, 190, 122, 137, 54]);
}
impl IContactPickerActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Contacts_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts_Provider`*"]
    pub fn ContactPickerUI(&self) -> ::windows::runtime::Result<super::Contacts::Provider::ContactPickerUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Provider::ContactPickerUI>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IContactPickerActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{ce57aae7-6449-45a7-971f-d113be7a8936}");
}
impl ::core::convert::From<IContactPickerActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IContactPickerActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IContactPickerActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IContactPickerActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IContactPickerActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IContactPickerActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactPickerActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IContactPickerActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IContactPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IContactPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IContactPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IContactPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPickerActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts_Provider")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts_Provider"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IContactPostActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactPostActivatedEventArgs {
    type Vtable = IContactPostActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3009035367, 61927, 18005, [173, 110, 72, 87, 88, 143, 85, 47]);
}
impl IContactPostActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceUserId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::runtime::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IContactPostActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{b35a3c67-f1e7-4655-ad6e-4857588f552f}");
}
impl ::core::convert::From<IContactPostActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IContactPostActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IContactPostActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IContactPostActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IContactPostActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IContactPostActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactPostActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IContactPostActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IContactPostActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IContactPostActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactPostActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IContactPostActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<IContactPostActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IContactPostActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactPostActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IContactPostActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactActivatedEventArgs> for IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactActivatedEventArgs> for &IContactPostActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPostActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IContactVideoCallActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactVideoCallActivatedEventArgs {
    type Vtable = IContactVideoCallActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1627889080, 58343, 19279, [133, 141, 92, 99, 169, 110, 246, 132]);
}
impl IContactVideoCallActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ServiceUserId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Contacts")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::runtime::Result<super::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contacts::Contact>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IContactVideoCallActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{61079db8-e3e7-4b4f-858d-5c63a96ef684}");
}
impl ::core::convert::From<IContactVideoCallActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IContactVideoCallActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IContactVideoCallActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IContactVideoCallActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IContactVideoCallActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IContactVideoCallActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactVideoCallActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IContactVideoCallActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IContactVideoCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IContactVideoCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactVideoCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IContactVideoCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<IContactVideoCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IContactVideoCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactVideoCallActivatedEventArgs> for IContactActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IContactVideoCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactActivatedEventArgs> for IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactActivatedEventArgs> for &IContactVideoCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactActivatedEventArgs> {
        ::core::convert::TryInto::<IContactActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactVideoCallActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_Contacts")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Contacts"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IContactsProviderActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactsProviderActivatedEventArgs {
    type Vtable = IContactsProviderActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1166073000, 22352, 18710, [170, 82, 192, 130, 149, 33, 235, 148]);
}
impl IContactsProviderActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IContactsProviderActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{4580dca8-5750-4916-aa52-c0829521eb94}");
}
impl ::core::convert::From<IContactsProviderActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IContactsProviderActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IContactsProviderActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IContactsProviderActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IContactsProviderActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IContactsProviderActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactsProviderActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IContactsProviderActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IContactsProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IContactsProviderActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContactsProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IContactsProviderActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IContactsProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactsProviderActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IContinuationActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContinuationActivatedEventArgs {
    type Vtable = IContinuationActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3850438325, 5471, 19092, [167, 66, 199, 224, 143, 78, 24, 140]);
}
impl IContinuationActivatedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn ContinuationData(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IContinuationActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{e58106b5-155f-4a94-a742-c7e08f4e188c}");
}
impl ::core::convert::From<IContinuationActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IContinuationActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IContinuationActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IContinuationActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IContinuationActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IContinuationActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContinuationActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IContinuationActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IContinuationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IContinuationActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IContinuationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IContinuationActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IContinuationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContinuationActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IDeviceActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceActivatedEventArgs {
    type Vtable = IDeviceActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3444619689, 52752, 17618, [130, 52, 195, 85, 160, 115, 239, 51]);
}
impl IDeviceActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn DeviceInformationId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IDeviceActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{cd50b9a9-ce10-44d2-8234-c355a073ef33}");
}
impl ::core::convert::From<IDeviceActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IDeviceActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IDeviceActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IDeviceActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IDeviceActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IDeviceActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDeviceActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IDeviceActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IDeviceActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IDeviceActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IDeviceActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IDeviceActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IDeviceActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IDevicePairingActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDevicePairingActivatedEventArgs {
    type Vtable = IDevicePairingActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3953185252, 60614, 16712, [148, 237, 244, 179, 126, 192, 91, 62]);
}
impl IDevicePairingActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Devices_Enumeration")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Devices_Enumeration`*"]
    pub fn DeviceInformation(&self) -> ::windows::runtime::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IDevicePairingActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{eba0d1e4-ecc6-4148-94ed-f4b37ec05b3e}");
}
impl ::core::convert::From<IDevicePairingActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IDevicePairingActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IDevicePairingActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IDevicePairingActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IDevicePairingActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IDevicePairingActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDevicePairingActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IDevicePairingActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IDevicePairingActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IDevicePairingActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IDevicePairingActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IDevicePairingActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IDevicePairingActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDevicePairingActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Enumeration")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IDialReceiverActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDialReceiverActivatedEventArgs {
    type Vtable = IDialReceiverActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4218912471, 34286, 17774, [164, 77, 133, 215, 48, 231, 10, 237]);
}
impl IDialReceiverActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn AppName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Arguments(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn TileId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IDialReceiverActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{fb777ed7-85ee-456e-a44d-85d730e70aed}");
}
impl ::core::convert::From<IDialReceiverActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IDialReceiverActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IDialReceiverActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IDialReceiverActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IDialReceiverActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IDialReceiverActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDialReceiverActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IDialReceiverActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IDialReceiverActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IDialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IDialReceiverActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IDialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<IDialReceiverActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IDialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IDialReceiverActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IDialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILaunchActivatedEventArgs> for IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILaunchActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILaunchActivatedEventArgs> for &IDialReceiverActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDialReceiverActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IFileActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileActivatedEventArgs {
    type Vtable = IFileActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3140156467, 37809, 17133, [139, 38, 35, 109, 217, 199, 132, 150]);
}
impl IFileActivatedEventArgs {
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`, `Storage`*"]
    pub fn Files(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IFileActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{bb2afc33-93b1-42ed-8b26-236dd9c78496}");
}
impl ::core::convert::From<IFileActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IFileActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFileActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IFileActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFileActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IFileActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IFileActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IFileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IFileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IFileActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IFileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IFileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IFileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IFileActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IFileActivatedEventArgsWithCallerPackageFamilyName(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileActivatedEventArgsWithCallerPackageFamilyName {
    type Vtable = IFileActivatedEventArgsWithCallerPackageFamilyName_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(761327723, 53855, 19749, [134, 83, 225, 197, 225, 16, 131, 9]);
}
impl IFileActivatedEventArgsWithCallerPackageFamilyName {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CallerPackageFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IFileActivatedEventArgsWithCallerPackageFamilyName {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{2d60f06b-d25f-4d25-8653-e1c5e1108309}");
}
impl ::core::convert::From<IFileActivatedEventArgsWithCallerPackageFamilyName> for ::windows::runtime::IUnknown {
    fn from(value: IFileActivatedEventArgsWithCallerPackageFamilyName) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFileActivatedEventArgsWithCallerPackageFamilyName> for ::windows::runtime::IUnknown {
    fn from(value: &IFileActivatedEventArgsWithCallerPackageFamilyName) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFileActivatedEventArgsWithCallerPackageFamilyName> for ::windows::runtime::IInspectable {
    fn from(value: IFileActivatedEventArgsWithCallerPackageFamilyName) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileActivatedEventArgsWithCallerPackageFamilyName> for ::windows::runtime::IInspectable {
    fn from(value: &IFileActivatedEventArgsWithCallerPackageFamilyName) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IFileActivatedEventArgsWithCallerPackageFamilyName> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IFileActivatedEventArgsWithCallerPackageFamilyName) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileActivatedEventArgsWithCallerPackageFamilyName> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IFileActivatedEventArgsWithCallerPackageFamilyName) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IFileActivatedEventArgsWithCallerPackageFamilyName {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgsWithCallerPackageFamilyName_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IFileActivatedEventArgsWithNeighboringFiles(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileActivatedEventArgsWithNeighboringFiles {
    type Vtable = IFileActivatedEventArgsWithNeighboringFiles_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1127981476, 57826, 18685, [183, 252, 181, 214, 238, 230, 80, 51]);
}
impl IFileActivatedEventArgsWithNeighboringFiles {
    #[cfg(feature = "Storage_Search")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage_Search`*"]
    pub fn NeighboringFilesQuery(&self) -> ::windows::runtime::Result<super::super::Storage::Search::StorageFileQueryResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Search::StorageFileQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`, `Storage`*"]
    pub fn Files(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = &::windows::runtime::Interface::cast::<IFileActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IFileActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IFileActivatedEventArgsWithNeighboringFiles {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{433ba1a4-e1e2-48fd-b7fc-b5d6eee65033}");
}
impl ::core::convert::From<IFileActivatedEventArgsWithNeighboringFiles> for ::windows::runtime::IUnknown {
    fn from(value: IFileActivatedEventArgsWithNeighboringFiles) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFileActivatedEventArgsWithNeighboringFiles> for ::windows::runtime::IUnknown {
    fn from(value: &IFileActivatedEventArgsWithNeighboringFiles) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFileActivatedEventArgsWithNeighboringFiles> for ::windows::runtime::IInspectable {
    fn from(value: IFileActivatedEventArgsWithNeighboringFiles) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileActivatedEventArgsWithNeighboringFiles> for ::windows::runtime::IInspectable {
    fn from(value: &IFileActivatedEventArgsWithNeighboringFiles) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IFileActivatedEventArgsWithNeighboringFiles> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IFileActivatedEventArgsWithNeighboringFiles) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileActivatedEventArgsWithNeighboringFiles> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IFileActivatedEventArgsWithNeighboringFiles) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<IFileActivatedEventArgsWithNeighboringFiles> for IFileActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IFileActivatedEventArgsWithNeighboringFiles) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileActivatedEventArgsWithNeighboringFiles> for IFileActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IFileActivatedEventArgsWithNeighboringFiles) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileActivatedEventArgs> for IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileActivatedEventArgs> for &IFileActivatedEventArgsWithNeighboringFiles {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileActivatedEventArgs> {
        ::core::convert::TryInto::<IFileActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileActivatedEventArgsWithNeighboringFiles_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Search")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Search"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IFileOpenPickerActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileOpenPickerActivatedEventArgs {
    type Vtable = IFileOpenPickerActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1921151106, 21797, 19442, [188, 9, 31, 80, 149, 212, 150, 77]);
}
impl IFileOpenPickerActivatedEventArgs {
    #[cfg(feature = "Storage_Pickers_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage_Pickers_Provider`*"]
    pub fn FileOpenPickerUI(&self) -> ::windows::runtime::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Pickers::Provider::FileOpenPickerUI>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IFileOpenPickerActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{72827082-5525-4bf2-bc09-1f5095d4964d}");
}
impl ::core::convert::From<IFileOpenPickerActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IFileOpenPickerActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFileOpenPickerActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IFileOpenPickerActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFileOpenPickerActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IFileOpenPickerActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileOpenPickerActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IFileOpenPickerActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IFileOpenPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IFileOpenPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileOpenPickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IFileOpenPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IFileOpenPickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Pickers_Provider")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Pickers_Provider"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IFileOpenPickerActivatedEventArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileOpenPickerActivatedEventArgs2 {
    type Vtable = IFileOpenPickerActivatedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1584602982, 36127, 17915, [175, 29, 115, 32, 92, 143, 199, 161]);
}
impl IFileOpenPickerActivatedEventArgs2 {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CallerPackageFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IFileOpenPickerActivatedEventArgs2 {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{5e731f66-8d1f-45fb-af1d-73205c8fc7a1}");
}
impl ::core::convert::From<IFileOpenPickerActivatedEventArgs2> for ::windows::runtime::IUnknown {
    fn from(value: IFileOpenPickerActivatedEventArgs2) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFileOpenPickerActivatedEventArgs2> for ::windows::runtime::IUnknown {
    fn from(value: &IFileOpenPickerActivatedEventArgs2) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFileOpenPickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFileOpenPickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFileOpenPickerActivatedEventArgs2> for ::windows::runtime::IInspectable {
    fn from(value: IFileOpenPickerActivatedEventArgs2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileOpenPickerActivatedEventArgs2> for ::windows::runtime::IInspectable {
    fn from(value: &IFileOpenPickerActivatedEventArgs2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IFileOpenPickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IFileOpenPickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerActivatedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IFileOpenPickerContinuationEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileOpenPickerContinuationEventArgs {
    type Vtable = IFileOpenPickerContinuationEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4042932026, 54504, 19155, [156, 52, 35, 8, 243, 47, 206, 201]);
}
impl IFileOpenPickerContinuationEventArgs {
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`, `Storage`*"]
    pub fn Files(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn ContinuationData(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::runtime::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IFileOpenPickerContinuationEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{f0fa3f3a-d4e8-4ad3-9c34-2308f32fcec9}");
}
impl ::core::convert::From<IFileOpenPickerContinuationEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IFileOpenPickerContinuationEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFileOpenPickerContinuationEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IFileOpenPickerContinuationEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFileOpenPickerContinuationEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IFileOpenPickerContinuationEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileOpenPickerContinuationEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IFileOpenPickerContinuationEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IFileOpenPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IFileOpenPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileOpenPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IFileOpenPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<IFileOpenPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IFileOpenPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileOpenPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IFileOpenPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContinuationActivatedEventArgs> for IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContinuationActivatedEventArgs> for &IFileOpenPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileOpenPickerContinuationEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IFileSavePickerActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileSavePickerActivatedEventArgs {
    type Vtable = IFileSavePickerActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2176949489, 29926, 17287, [130, 235, 187, 143, 214, 75, 67, 70]);
}
impl IFileSavePickerActivatedEventArgs {
    #[cfg(feature = "Storage_Pickers_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage_Pickers_Provider`*"]
    pub fn FileSavePickerUI(&self) -> ::windows::runtime::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Pickers::Provider::FileSavePickerUI>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IFileSavePickerActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{81c19cf1-74e6-4387-82eb-bb8fd64b4346}");
}
impl ::core::convert::From<IFileSavePickerActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IFileSavePickerActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFileSavePickerActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IFileSavePickerActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFileSavePickerActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IFileSavePickerActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileSavePickerActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IFileSavePickerActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IFileSavePickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IFileSavePickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileSavePickerActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IFileSavePickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IFileSavePickerActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Pickers_Provider")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Pickers_Provider"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IFileSavePickerActivatedEventArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileSavePickerActivatedEventArgs2 {
    type Vtable = IFileSavePickerActivatedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1802763795, 11506, 19784, [140, 188, 175, 103, 210, 63, 28, 231]);
}
impl IFileSavePickerActivatedEventArgs2 {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CallerPackageFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn EnterpriseId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IFileSavePickerActivatedEventArgs2 {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{6b73fe13-2cf2-4d48-8cbc-af67d23f1ce7}");
}
impl ::core::convert::From<IFileSavePickerActivatedEventArgs2> for ::windows::runtime::IUnknown {
    fn from(value: IFileSavePickerActivatedEventArgs2) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFileSavePickerActivatedEventArgs2> for ::windows::runtime::IUnknown {
    fn from(value: &IFileSavePickerActivatedEventArgs2) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFileSavePickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFileSavePickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFileSavePickerActivatedEventArgs2> for ::windows::runtime::IInspectable {
    fn from(value: IFileSavePickerActivatedEventArgs2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileSavePickerActivatedEventArgs2> for ::windows::runtime::IInspectable {
    fn from(value: &IFileSavePickerActivatedEventArgs2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IFileSavePickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IFileSavePickerActivatedEventArgs2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerActivatedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IFileSavePickerContinuationEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileSavePickerContinuationEventArgs {
    type Vtable = IFileSavePickerContinuationEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(746876897, 15277, 20275, [140, 139, 228, 111, 174, 130, 75, 75]);
}
impl IFileSavePickerContinuationEventArgs {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage`*"]
    pub fn File(&self) -> ::windows::runtime::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn ContinuationData(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::runtime::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IFileSavePickerContinuationEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{2c846fe1-3bad-4f33-8c8b-e46fae824b4b}");
}
impl ::core::convert::From<IFileSavePickerContinuationEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IFileSavePickerContinuationEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFileSavePickerContinuationEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IFileSavePickerContinuationEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFileSavePickerContinuationEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IFileSavePickerContinuationEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileSavePickerContinuationEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IFileSavePickerContinuationEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IFileSavePickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IFileSavePickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileSavePickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IFileSavePickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<IFileSavePickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IFileSavePickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileSavePickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IFileSavePickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContinuationActivatedEventArgs> for IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContinuationActivatedEventArgs> for &IFileSavePickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileSavePickerContinuationEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IFolderPickerContinuationEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFolderPickerContinuationEventArgs {
    type Vtable = IFolderPickerContinuationEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1367876454, 40779, 18831, [190, 176, 66, 104, 79, 110, 28, 41]);
}
impl IFolderPickerContinuationEventArgs {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Storage`*"]
    pub fn Folder(&self) -> ::windows::runtime::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn ContinuationData(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::runtime::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IFolderPickerContinuationEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{51882366-9f4b-498f-beb0-42684f6e1c29}");
}
impl ::core::convert::From<IFolderPickerContinuationEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IFolderPickerContinuationEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFolderPickerContinuationEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IFolderPickerContinuationEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFolderPickerContinuationEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IFolderPickerContinuationEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFolderPickerContinuationEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IFolderPickerContinuationEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IFolderPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IFolderPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFolderPickerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IFolderPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<IFolderPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IFolderPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFolderPickerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IFolderPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContinuationActivatedEventArgs> for IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContinuationActivatedEventArgs> for &IFolderPickerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFolderPickerContinuationEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct ILaunchActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILaunchActivatedEventArgs {
    type Vtable = ILaunchActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4224269862, 41290, 19279, [130, 176, 51, 190, 217, 32, 175, 82]);
}
impl ILaunchActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Arguments(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn TileId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ILaunchActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{fbc93e26-a14a-4b4f-82b0-33bed920af52}");
}
impl ::core::convert::From<ILaunchActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ILaunchActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ILaunchActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ILaunchActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ILaunchActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ILaunchActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILaunchActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ILaunchActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ILaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ILaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ILaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &ILaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct ILaunchActivatedEventArgs2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILaunchActivatedEventArgs2 {
    type Vtable = ILaunchActivatedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(265518780, 40393, 18101, [154, 206, 189, 149, 212, 86, 83, 69]);
}
impl ILaunchActivatedEventArgs2 {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Arguments(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn TileId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn TileActivatedInfo(&self) -> ::windows::runtime::Result<TileActivatedInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TileActivatedInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ILaunchActivatedEventArgs2 {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{0fd37ebc-9dc9-46b5-9ace-bd95d4565345}");
}
impl ::core::convert::From<ILaunchActivatedEventArgs2> for ::windows::runtime::IUnknown {
    fn from(value: ILaunchActivatedEventArgs2) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ILaunchActivatedEventArgs2> for ::windows::runtime::IUnknown {
    fn from(value: &ILaunchActivatedEventArgs2) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ILaunchActivatedEventArgs2> for ::windows::runtime::IInspectable {
    fn from(value: ILaunchActivatedEventArgs2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILaunchActivatedEventArgs2> for ::windows::runtime::IInspectable {
    fn from(value: &ILaunchActivatedEventArgs2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ILaunchActivatedEventArgs2> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ILaunchActivatedEventArgs2) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILaunchActivatedEventArgs2> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ILaunchActivatedEventArgs2) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<ILaunchActivatedEventArgs2> for ILaunchActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ILaunchActivatedEventArgs2) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILaunchActivatedEventArgs2> for ILaunchActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ILaunchActivatedEventArgs2) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILaunchActivatedEventArgs> for ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILaunchActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILaunchActivatedEventArgs> for &ILaunchActivatedEventArgs2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchActivatedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct ILockScreenActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILockScreenActivatedEventArgs {
    type Vtable = ILockScreenActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1017608550, 24840, 19009, [130, 32, 238, 125, 19, 60, 133, 50]);
}
impl ILockScreenActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Info(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ILockScreenActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{3ca77966-6108-4a41-8220-ee7d133c8532}");
}
impl ::core::convert::From<ILockScreenActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ILockScreenActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ILockScreenActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ILockScreenActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ILockScreenActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ILockScreenActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILockScreenActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ILockScreenActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ILockScreenActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ILockScreenActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILockScreenActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ILockScreenActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &ILockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct ILockScreenCallActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILockScreenCallActivatedEventArgs {
    type Vtable = ILockScreenCallActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(116621246, 46578, 17547, [177, 62, 227, 40, 172, 28, 81, 106]);
}
impl ILockScreenCallActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Calls")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Calls`*"]
    pub fn CallUI(&self) -> ::windows::runtime::Result<super::Calls::LockScreenCallUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Calls::LockScreenCallUI>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Arguments(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn TileId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ILockScreenCallActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{06f37fbe-b5f2-448b-b13e-e328ac1c516a}");
}
impl ::core::convert::From<ILockScreenCallActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ILockScreenCallActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ILockScreenCallActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ILockScreenCallActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ILockScreenCallActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ILockScreenCallActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILockScreenCallActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ILockScreenCallActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ILockScreenCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ILockScreenCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILockScreenCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ILockScreenCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<ILockScreenCallActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ILockScreenCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILockScreenCallActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ILockScreenCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILaunchActivatedEventArgs> for ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILaunchActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILaunchActivatedEventArgs> for &ILockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockScreenCallActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_Calls")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Calls"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IPhoneCallActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPhoneCallActivatedEventArgs {
    type Vtable = IPhoneCallActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1415664161, 41921, 19693, [182, 47, 140, 96, 82, 54, 25, 173]);
}
impl IPhoneCallActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn LineId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPhoneCallActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{54615221-a3c1-4ced-b62f-8c60523619ad}");
}
impl ::core::convert::From<IPhoneCallActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IPhoneCallActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPhoneCallActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IPhoneCallActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPhoneCallActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IPhoneCallActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPhoneCallActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IPhoneCallActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IPhoneCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IPhoneCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPhoneCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IPhoneCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IPhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneCallActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IPickerReturnedActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPickerReturnedActivatedEventArgs {
    type Vtable = IPickerReturnedActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(906883001, 43475, 18820, [164, 237, 158, 199, 52, 96, 73, 33]);
}
impl IPickerReturnedActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PickerOperationId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPickerReturnedActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{360defb9-a9d3-4984-a4ed-9ec734604921}");
}
impl ::core::convert::From<IPickerReturnedActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IPickerReturnedActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPickerReturnedActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IPickerReturnedActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPickerReturnedActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IPickerReturnedActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPickerReturnedActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IPickerReturnedActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IPickerReturnedActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IPickerReturnedActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPickerReturnedActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IPickerReturnedActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IPickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPickerReturnedActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IPrelaunchActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrelaunchActivatedEventArgs {
    type Vtable = IPrelaunchActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(205812091, 6647, 18646, [176, 70, 207, 34, 130, 110, 170, 116]);
}
impl IPrelaunchActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PrelaunchActivated(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPrelaunchActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{0c44717b-19f7-48d6-b046-cf22826eaa74}");
}
impl ::core::convert::From<IPrelaunchActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IPrelaunchActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPrelaunchActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IPrelaunchActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPrelaunchActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IPrelaunchActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrelaunchActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IPrelaunchActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IPrelaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IPrelaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPrelaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IPrelaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IPrelaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrelaunchActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IPrint3DWorkflowActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrint3DWorkflowActivatedEventArgs {
    type Vtable = IPrint3DWorkflowActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1062725515, 62124, 17945, [131, 2, 239, 133, 94, 28, 155, 144]);
}
impl IPrint3DWorkflowActivatedEventArgs {
    #[cfg(feature = "Devices_Printers_Extensions")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Devices_Printers_Extensions`*"]
    pub fn Workflow(&self) -> ::windows::runtime::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Printers::Extensions::Print3DWorkflow>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPrint3DWorkflowActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{3f57e78b-f2ac-4619-8302-ef855e1c9b90}");
}
impl ::core::convert::From<IPrint3DWorkflowActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IPrint3DWorkflowActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPrint3DWorkflowActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IPrint3DWorkflowActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPrint3DWorkflowActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IPrint3DWorkflowActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrint3DWorkflowActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IPrint3DWorkflowActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IPrint3DWorkflowActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IPrint3DWorkflowActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPrint3DWorkflowActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IPrint3DWorkflowActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IPrint3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DWorkflowActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Printers_Extensions")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Printers_Extensions"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IPrintTaskSettingsActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintTaskSettingsActivatedEventArgs {
    type Vtable = IPrintTaskSettingsActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3996164297, 52822, 18533, [186, 142, 137, 84, 172, 39, 17, 7]);
}
impl IPrintTaskSettingsActivatedEventArgs {
    #[cfg(feature = "Devices_Printers_Extensions")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Devices_Printers_Extensions`*"]
    pub fn Configuration(&self) -> ::windows::runtime::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Printers::Extensions::PrintTaskConfiguration>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPrintTaskSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{ee30a0c9-ce56-4865-ba8e-8954ac271107}");
}
impl ::core::convert::From<IPrintTaskSettingsActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IPrintTaskSettingsActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPrintTaskSettingsActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IPrintTaskSettingsActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPrintTaskSettingsActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IPrintTaskSettingsActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintTaskSettingsActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IPrintTaskSettingsActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IPrintTaskSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IPrintTaskSettingsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPrintTaskSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IPrintTaskSettingsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IPrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskSettingsActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Printers_Extensions")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Printers_Extensions"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IProtocolActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtocolActivatedEventArgs {
    type Vtable = IProtocolActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1620440285, 47040, 18091, [129, 254, 217, 15, 54, 208, 13, 36]);
}
impl IProtocolActivatedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IProtocolActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{6095f4dd-b7c0-46ab-81fe-d90f36d00d24}");
}
impl ::core::convert::From<IProtocolActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IProtocolActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IProtocolActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IProtocolActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IProtocolActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IProtocolActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProtocolActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IProtocolActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IProtocolActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IProtocolActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IProtocolActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IProtocolActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Vtable = IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3628731410, 23695, 17292, [131, 203, 194, 143, 204, 11, 47, 219]);
}
impl IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CallerPackageFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn Data(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{d84a0c12-5c8f-438c-83cb-c28fcc0b2fdb}");
}
impl ::core::convert::From<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ::windows::runtime::IUnknown {
    fn from(value: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ::windows::runtime::IUnknown {
    fn from(value: &IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ::windows::runtime::IInspectable {
    fn from(value: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ::windows::runtime::IInspectable {
    fn from(value: &IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IProtocolForResultsActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtocolForResultsActivatedEventArgs {
    type Vtable = IProtocolForResultsActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3880858306, 31463, 17687, [128, 172, 219, 232, 215, 204, 91, 156]);
}
impl IProtocolForResultsActivatedEventArgs {
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn ProtocolForResultsOperation(&self) -> ::windows::runtime::Result<super::super::System::ProtocolForResultsOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::ProtocolForResultsOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IProtocolForResultsActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{e75132c2-7ae7-4517-80ac-dbe8d7cc5b9c}");
}
impl ::core::convert::From<IProtocolForResultsActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IProtocolForResultsActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IProtocolForResultsActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IProtocolForResultsActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IProtocolForResultsActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IProtocolForResultsActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IProtocolForResultsActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IProtocolForResultsActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IProtocolForResultsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IProtocolForResultsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IProtocolForResultsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IProtocolForResultsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtocolForResultsActivatedEventArgs_abi(
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IRestrictedLaunchActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRestrictedLaunchActivatedEventArgs {
    type Vtable = IRestrictedLaunchActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3770133633, 49091, 17220, [165, 218, 25, 253, 90, 39, 186, 174]);
}
impl IRestrictedLaunchActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SharedContext(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IRestrictedLaunchActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{e0b7ac81-bfc3-4344-a5da-19fd5a27baae}");
}
impl ::core::convert::From<IRestrictedLaunchActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IRestrictedLaunchActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IRestrictedLaunchActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IRestrictedLaunchActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IRestrictedLaunchActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IRestrictedLaunchActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRestrictedLaunchActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IRestrictedLaunchActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IRestrictedLaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IRestrictedLaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IRestrictedLaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IRestrictedLaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IRestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRestrictedLaunchActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct ISearchActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISearchActivatedEventArgs {
    type Vtable = ISearchActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2360568145, 22728, 17379, [148, 188, 65, 211, 63, 139, 99, 14]);
}
impl ISearchActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn QueryText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ISearchActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{8cb36951-58c8-43e3-94bc-41d33f8b630e}");
}
impl ::core::convert::From<ISearchActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ISearchActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ISearchActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ISearchActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ISearchActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ISearchActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISearchActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ISearchActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ISearchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ISearchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ISearchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ISearchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &ISearchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct ISearchActivatedEventArgsWithLinguisticDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISearchActivatedEventArgsWithLinguisticDetails {
    type Vtable = ISearchActivatedEventArgsWithLinguisticDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3231658970, 2219, 18737, [155, 124, 69, 16, 37, 242, 31, 129]);
}
impl ISearchActivatedEventArgsWithLinguisticDetails {
    #[cfg(feature = "ApplicationModel_Search")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Search`*"]
    pub fn LinguisticDetails(&self) -> ::windows::runtime::Result<super::Search::SearchPaneQueryLinguisticDetails> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Search::SearchPaneQueryLinguisticDetails>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ISearchActivatedEventArgsWithLinguisticDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{c09f33da-08ab-4931-9b7c-451025f21f81}");
}
impl ::core::convert::From<ISearchActivatedEventArgsWithLinguisticDetails> for ::windows::runtime::IUnknown {
    fn from(value: ISearchActivatedEventArgsWithLinguisticDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ISearchActivatedEventArgsWithLinguisticDetails> for ::windows::runtime::IUnknown {
    fn from(value: &ISearchActivatedEventArgsWithLinguisticDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISearchActivatedEventArgsWithLinguisticDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISearchActivatedEventArgsWithLinguisticDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ISearchActivatedEventArgsWithLinguisticDetails> for ::windows::runtime::IInspectable {
    fn from(value: ISearchActivatedEventArgsWithLinguisticDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ISearchActivatedEventArgsWithLinguisticDetails> for ::windows::runtime::IInspectable {
    fn from(value: &ISearchActivatedEventArgsWithLinguisticDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ISearchActivatedEventArgsWithLinguisticDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ISearchActivatedEventArgsWithLinguisticDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISearchActivatedEventArgsWithLinguisticDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_Search")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Search"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IShareTargetActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IShareTargetActivatedEventArgs {
    type Vtable = IShareTargetActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1272641992, 52658, 19147, [191, 195, 102, 72, 86, 51, 120, 236]);
}
impl IShareTargetActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_DataTransfer_ShareTarget`*"]
    pub fn ShareOperation(&self) -> ::windows::runtime::Result<super::DataTransfer::ShareTarget::ShareOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DataTransfer::ShareTarget::ShareOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IShareTargetActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{4bdaf9c8-cdb2-4acb-bfc3-6648563378ec}");
}
impl ::core::convert::From<IShareTargetActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IShareTargetActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IShareTargetActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IShareTargetActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IShareTargetActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IShareTargetActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IShareTargetActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IShareTargetActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IShareTargetActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IShareTargetActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IShareTargetActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IShareTargetActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareTargetActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_DataTransfer_ShareTarget"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISplashScreen(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISplashScreen {
    type Vtable = ISplashScreen_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3394082652, 54486, 17392, [151, 192, 8, 51, 198, 57, 28, 36]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISplashScreen_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IStartupTaskActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStartupTaskActivatedEventArgs {
    type Vtable = IStartupTaskActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(61938264, 21110, 19857, [134, 33, 84, 97, 24, 100, 213, 250]);
}
impl IStartupTaskActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn TaskId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IStartupTaskActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{03b11a58-5276-4d91-8621-54611864d5fa}");
}
impl ::core::convert::From<IStartupTaskActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IStartupTaskActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IStartupTaskActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IStartupTaskActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IStartupTaskActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IStartupTaskActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStartupTaskActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IStartupTaskActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IStartupTaskActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IStartupTaskActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IStartupTaskActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IStartupTaskActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IStartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartupTaskActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITileActivatedInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITileActivatedInfo {
    type Vtable = ITileActivatedInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2162467761, 14720, 20247, [183, 56, 137, 25, 78, 11, 143, 101]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITileActivatedInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Notifications"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Notifications")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IToastNotificationActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotificationActivatedEventArgs {
    type Vtable = IToastNotificationActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2460512130, 21136, 17181, [190, 133, 196, 170, 238, 184, 104, 95]);
}
impl IToastNotificationActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Argument(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn UserInput(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IToastNotificationActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{92a86f82-5290-431d-be85-c4aaeeb8685f}");
}
impl ::core::convert::From<IToastNotificationActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IToastNotificationActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IToastNotificationActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IToastNotificationActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IToastNotificationActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IToastNotificationActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IToastNotificationActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IToastNotificationActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IToastNotificationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IToastNotificationActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IToastNotificationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IToastNotificationActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IUserDataAccountProviderActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccountProviderActivatedEventArgs {
    type Vtable = IUserDataAccountProviderActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(466220835, 36593, 19025, [166, 58, 254, 113, 30, 234, 182, 7]);
}
impl IUserDataAccountProviderActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_UserDataAccounts_Provider`*"]
    pub fn Operation(&self) -> ::windows::runtime::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IUserDataAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{1bc9f723-8ef1-4a51-a63a-fe711eeab607}");
}
impl ::core::convert::From<IUserDataAccountProviderActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IUserDataAccountProviderActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IUserDataAccountProviderActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IUserDataAccountProviderActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IUserDataAccountProviderActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IUserDataAccountProviderActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUserDataAccountProviderActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IUserDataAccountProviderActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IUserDataAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IUserDataAccountProviderActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IUserDataAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IUserDataAccountProviderActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IUserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountProviderActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_UserDataAccounts_Provider"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IViewSwitcherProvider(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IViewSwitcherProvider {
    type Vtable = IViewSwitcherProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(871532710, 23596, 19751, [186, 199, 117, 54, 8, 143, 18, 25]);
}
impl IViewSwitcherProvider {
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `UI_ViewManagement`*"]
    pub fn ViewSwitcher(&self) -> ::windows::runtime::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IViewSwitcherProvider {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{33f288a6-5c2c-4d27-bac7-7536088f1219}");
}
impl ::core::convert::From<IViewSwitcherProvider> for ::windows::runtime::IUnknown {
    fn from(value: IViewSwitcherProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IViewSwitcherProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IViewSwitcherProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IViewSwitcherProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IViewSwitcherProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IViewSwitcherProvider> for ::windows::runtime::IInspectable {
    fn from(value: IViewSwitcherProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IViewSwitcherProvider> for ::windows::runtime::IInspectable {
    fn from(value: &IViewSwitcherProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IViewSwitcherProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IViewSwitcherProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IViewSwitcherProvider> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IViewSwitcherProvider) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IViewSwitcherProvider> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IViewSwitcherProvider) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IViewSwitcherProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IViewSwitcherProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewSwitcherProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_ViewManagement")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IVoiceCommandActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVoiceCommandActivatedEventArgs {
    type Vtable = IVoiceCommandActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2878528765, 36163, 19942, [151, 117, 32, 112, 75, 88, 27, 0]);
}
impl IVoiceCommandActivatedEventArgs {
    #[cfg(feature = "Media_SpeechRecognition")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Media_SpeechRecognition`*"]
    pub fn Result(&self) -> ::windows::runtime::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::SpeechRecognition::SpeechRecognitionResult>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IVoiceCommandActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{ab92dcfd-8d43-4de6-9775-20704b581b00}");
}
impl ::core::convert::From<IVoiceCommandActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IVoiceCommandActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IVoiceCommandActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IVoiceCommandActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IVoiceCommandActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IVoiceCommandActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVoiceCommandActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IVoiceCommandActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IVoiceCommandActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IVoiceCommandActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IVoiceCommandActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IVoiceCommandActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IVoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_SpeechRecognition")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_SpeechRecognition"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IWalletActionActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWalletActionActivatedEventArgs {
    type Vtable = IWalletActionActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4244374139, 6682, 19746, [146, 63, 174, 111, 69, 250, 82, 217]);
}
impl IWalletActionActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ItemId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Wallet")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Wallet`*"]
    pub fn ActionKind(&self) -> ::windows::runtime::Result<super::Wallet::WalletActionKind> {
        let this = self;
        unsafe {
            let mut result__: super::Wallet::WalletActionKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Wallet::WalletActionKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ActionId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IWalletActionActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{fcfc027b-1a1a-4d22-923f-ae6f45fa52d9}");
}
impl ::core::convert::From<IWalletActionActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IWalletActionActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IWalletActionActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IWalletActionActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IWalletActionActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IWalletActionActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWalletActionActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IWalletActionActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IWalletActionActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IWalletActionActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWalletActionActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IWalletActionActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IWalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWalletActionActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_Wallet")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Wallet::WalletActionKind) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Wallet"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IWebAccountProviderActivatedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAccountProviderActivatedEventArgs {
    type Vtable = IWebAccountProviderActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1924601716, 39146, 19663, [151, 82, 70, 217, 5, 16, 4, 241]);
}
impl IWebAccountProviderActivatedEventArgs {
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Security_Authentication_Web_Provider`*"]
    pub fn Operation(&self) -> ::windows::runtime::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IWebAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{72b71774-98ea-4ccf-9752-46d9051004f1}");
}
impl ::core::convert::From<IWebAccountProviderActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IWebAccountProviderActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IWebAccountProviderActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IWebAccountProviderActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IWebAccountProviderActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IWebAccountProviderActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWebAccountProviderActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IWebAccountProviderActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IWebAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IWebAccountProviderActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWebAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IWebAccountProviderActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IWebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountProviderActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Security_Authentication_Web_Provider")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web_Provider"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Activation`*"]
pub struct IWebAuthenticationBrokerContinuationEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAuthenticationBrokerContinuationEventArgs {
    type Vtable = IWebAuthenticationBrokerContinuationEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1977459668, 30484, 17725, [183, 255, 185, 94, 58, 23, 9, 218]);
}
impl IWebAuthenticationBrokerContinuationEventArgs {
    #[cfg(feature = "Security_Authentication_Web")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Security_Authentication_Web`*"]
    pub fn WebAuthenticationResult(&self) -> ::windows::runtime::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Authentication::Web::WebAuthenticationResult>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn ContinuationData(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::runtime::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IWebAuthenticationBrokerContinuationEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{75dda3d4-7714-453d-b7ff-b95e3a1709da}");
}
impl ::core::convert::From<IWebAuthenticationBrokerContinuationEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IWebAuthenticationBrokerContinuationEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IWebAuthenticationBrokerContinuationEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IWebAuthenticationBrokerContinuationEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IWebAuthenticationBrokerContinuationEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IWebAuthenticationBrokerContinuationEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWebAuthenticationBrokerContinuationEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IWebAuthenticationBrokerContinuationEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IWebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IWebAuthenticationBrokerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IWebAuthenticationBrokerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<IWebAuthenticationBrokerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IWebAuthenticationBrokerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IWebAuthenticationBrokerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IWebAuthenticationBrokerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContinuationActivatedEventArgs> for IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContinuationActivatedEventArgs> for &IWebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationBrokerContinuationEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Security_Authentication_Web")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Authentication_Web"))] usize,
);
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LaunchActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl LaunchActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Arguments(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn TileId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PrelaunchActivated(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IPrelaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `UI_ViewManagement`*"]
    pub fn ViewSwitcher(&self) -> ::windows::runtime::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::runtime::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn TileActivatedInfo(&self) -> ::windows::runtime::Result<TileActivatedInfo> {
        let this = &::windows::runtime::Interface::cast::<ILaunchActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TileActivatedInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LaunchActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LaunchActivatedEventArgs;{fbc93e26-a14a-4b4f-82b0-33bed920af52})");
}
unsafe impl ::windows::runtime::Interface for LaunchActivatedEventArgs {
    type Vtable = ILaunchActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4224269862, 41290, 19279, [130, 176, 51, 190, 217, 32, 175, 82]);
}
impl ::windows::runtime::RuntimeName for LaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LaunchActivatedEventArgs";
}
impl ::core::convert::From<LaunchActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: LaunchActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LaunchActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &LaunchActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LaunchActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: LaunchActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LaunchActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &LaunchActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ILaunchActivatedEventArgs> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILaunchActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILaunchActivatedEventArgs> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILaunchActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IApplicationViewActivatedEventArgs> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IApplicationViewActivatedEventArgs> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IPrelaunchActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IPrelaunchActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPrelaunchActivatedEventArgs> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPrelaunchActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPrelaunchActivatedEventArgs> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPrelaunchActivatedEventArgs> {
        ::core::convert::TryInto::<IPrelaunchActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IViewSwitcherProvider> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IViewSwitcherProvider> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IViewSwitcherProvider> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<LaunchActivatedEventArgs> for ILaunchActivatedEventArgs2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LaunchActivatedEventArgs> for ILaunchActivatedEventArgs2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILaunchActivatedEventArgs2> for LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILaunchActivatedEventArgs2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILaunchActivatedEventArgs2> for &LaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILaunchActivatedEventArgs2> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs2>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for LaunchActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LaunchActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LockScreenActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl LockScreenActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Info(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LockScreenActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LockScreenActivatedEventArgs;{3ca77966-6108-4a41-8220-ee7d133c8532})");
}
unsafe impl ::windows::runtime::Interface for LockScreenActivatedEventArgs {
    type Vtable = ILockScreenActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1017608550, 24840, 19009, [130, 32, 238, 125, 19, 60, 133, 50]);
}
impl ::windows::runtime::RuntimeName for LockScreenActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenActivatedEventArgs";
}
impl ::core::convert::From<LockScreenActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: LockScreenActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LockScreenActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &LockScreenActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LockScreenActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: LockScreenActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LockScreenActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &LockScreenActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ILockScreenActivatedEventArgs> for LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILockScreenActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILockScreenActivatedEventArgs> for &LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILockScreenActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LockScreenActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LockScreenActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LockScreenActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<LockScreenActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LockScreenActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LockScreenActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &LockScreenActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for LockScreenActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LockScreenCallActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl LockScreenCallActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Calls")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Calls`*"]
    pub fn CallUI(&self) -> ::windows::runtime::Result<super::Calls::LockScreenCallUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Calls::LockScreenCallUI>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Arguments(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn TileId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `UI_ViewManagement`*"]
    pub fn ViewSwitcher(&self) -> ::windows::runtime::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::runtime::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LockScreenCallActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LockScreenCallActivatedEventArgs;{06f37fbe-b5f2-448b-b13e-e328ac1c516a})");
}
unsafe impl ::windows::runtime::Interface for LockScreenCallActivatedEventArgs {
    type Vtable = ILockScreenCallActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(116621246, 46578, 17547, [177, 62, 227, 40, 172, 28, 81, 106]);
}
impl ::windows::runtime::RuntimeName for LockScreenCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenCallActivatedEventArgs";
}
impl ::core::convert::From<LockScreenCallActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: LockScreenCallActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LockScreenCallActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &LockScreenCallActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LockScreenCallActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: LockScreenCallActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LockScreenCallActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &LockScreenCallActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ILockScreenCallActivatedEventArgs> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILockScreenCallActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILockScreenCallActivatedEventArgs> for &LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILockScreenCallActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LockScreenCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LockScreenCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<LockScreenCallActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LockScreenCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IApplicationViewActivatedEventArgs> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IApplicationViewActivatedEventArgs> for &LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<LockScreenCallActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LockScreenCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for ILaunchActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILaunchActivatedEventArgs> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILaunchActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILaunchActivatedEventArgs> for &LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILaunchActivatedEventArgs> {
        ::core::convert::TryInto::<ILaunchActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<LockScreenCallActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LockScreenCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LockScreenCallActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LockScreenCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IViewSwitcherProvider> for LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IViewSwitcherProvider> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IViewSwitcherProvider> for &LockScreenCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for LockScreenCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenCallActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LockScreenComponentActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl LockScreenComponentActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = self;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LockScreenComponentActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.LockScreenComponentActivatedEventArgs;{cf651713-cd08-4fd8-b697-a281b6544e2e})");
}
unsafe impl ::windows::runtime::Interface for LockScreenComponentActivatedEventArgs {
    type Vtable = IActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3479508755, 52488, 20440, [182, 151, 162, 129, 182, 84, 78, 46]);
}
impl ::windows::runtime::RuntimeName for LockScreenComponentActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LockScreenComponentActivatedEventArgs";
}
impl ::core::convert::From<LockScreenComponentActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: LockScreenComponentActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LockScreenComponentActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &LockScreenComponentActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LockScreenComponentActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: LockScreenComponentActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LockScreenComponentActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &LockScreenComponentActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &LockScreenComponentActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LockScreenComponentActivatedEventArgs {}
unsafe impl ::core::marker::Sync for LockScreenComponentActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PhoneCallActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl PhoneCallActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn LineId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PhoneCallActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.PhoneCallActivatedEventArgs;{54615221-a3c1-4ced-b62f-8c60523619ad})");
}
unsafe impl ::windows::runtime::Interface for PhoneCallActivatedEventArgs {
    type Vtable = IPhoneCallActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1415664161, 41921, 19693, [182, 47, 140, 96, 82, 54, 25, 173]);
}
impl ::windows::runtime::RuntimeName for PhoneCallActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PhoneCallActivatedEventArgs";
}
impl ::core::convert::From<PhoneCallActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: PhoneCallActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PhoneCallActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &PhoneCallActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PhoneCallActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: PhoneCallActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PhoneCallActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &PhoneCallActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<PhoneCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PhoneCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PhoneCallActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PhoneCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<PhoneCallActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PhoneCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PhoneCallActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PhoneCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
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
impl<'a> ::windows::runtime::IntoParam<'a, IPhoneCallActivatedEventArgs> for PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPhoneCallActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPhoneCallActivatedEventArgs> for &PhoneCallActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPhoneCallActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PhoneCallActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PhoneCallActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PickerReturnedActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl PickerReturnedActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PickerOperationId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PickerReturnedActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.PickerReturnedActivatedEventArgs;{360defb9-a9d3-4984-a4ed-9ec734604921})");
}
unsafe impl ::windows::runtime::Interface for PickerReturnedActivatedEventArgs {
    type Vtable = IPickerReturnedActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(906883001, 43475, 18820, [164, 237, 158, 199, 52, 96, 73, 33]);
}
impl ::windows::runtime::RuntimeName for PickerReturnedActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PickerReturnedActivatedEventArgs";
}
impl ::core::convert::From<PickerReturnedActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: PickerReturnedActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PickerReturnedActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &PickerReturnedActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PickerReturnedActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: PickerReturnedActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PickerReturnedActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &PickerReturnedActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IPickerReturnedActivatedEventArgs> for PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPickerReturnedActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPickerReturnedActivatedEventArgs> for &PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPickerReturnedActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PickerReturnedActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PickerReturnedActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PickerReturnedActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PickerReturnedActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &PickerReturnedActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for PickerReturnedActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PickerReturnedActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Print3DWorkflowActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl Print3DWorkflowActivatedEventArgs {
    #[cfg(feature = "Devices_Printers_Extensions")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Devices_Printers_Extensions`*"]
    pub fn Workflow(&self) -> ::windows::runtime::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Printers::Extensions::Print3DWorkflow>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Print3DWorkflowActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.Print3DWorkflowActivatedEventArgs;{3f57e78b-f2ac-4619-8302-ef855e1c9b90})");
}
unsafe impl ::windows::runtime::Interface for Print3DWorkflowActivatedEventArgs {
    type Vtable = IPrint3DWorkflowActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1062725515, 62124, 17945, [131, 2, 239, 133, 94, 28, 155, 144]);
}
impl ::windows::runtime::RuntimeName for Print3DWorkflowActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.Print3DWorkflowActivatedEventArgs";
}
impl ::core::convert::From<Print3DWorkflowActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: Print3DWorkflowActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Print3DWorkflowActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &Print3DWorkflowActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Print3DWorkflowActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: Print3DWorkflowActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Print3DWorkflowActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &Print3DWorkflowActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IPrint3DWorkflowActivatedEventArgs> for Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPrint3DWorkflowActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPrint3DWorkflowActivatedEventArgs> for &Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPrint3DWorkflowActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<Print3DWorkflowActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: Print3DWorkflowActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Print3DWorkflowActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &Print3DWorkflowActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &Print3DWorkflowActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for Print3DWorkflowActivatedEventArgs {}
unsafe impl ::core::marker::Sync for Print3DWorkflowActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTaskSettingsActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl PrintTaskSettingsActivatedEventArgs {
    #[cfg(feature = "Devices_Printers_Extensions")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Devices_Printers_Extensions`*"]
    pub fn Configuration(&self) -> ::windows::runtime::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Printers::Extensions::PrintTaskConfiguration>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrintTaskSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.PrintTaskSettingsActivatedEventArgs;{ee30a0c9-ce56-4865-ba8e-8954ac271107})");
}
unsafe impl ::windows::runtime::Interface for PrintTaskSettingsActivatedEventArgs {
    type Vtable = IPrintTaskSettingsActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3996164297, 52822, 18533, [186, 142, 137, 84, 172, 39, 17, 7]);
}
impl ::windows::runtime::RuntimeName for PrintTaskSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.PrintTaskSettingsActivatedEventArgs";
}
impl ::core::convert::From<PrintTaskSettingsActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: PrintTaskSettingsActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintTaskSettingsActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &PrintTaskSettingsActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintTaskSettingsActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: PrintTaskSettingsActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintTaskSettingsActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &PrintTaskSettingsActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IPrintTaskSettingsActivatedEventArgs> for PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPrintTaskSettingsActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPrintTaskSettingsActivatedEventArgs> for &PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPrintTaskSettingsActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PrintTaskSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PrintTaskSettingsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintTaskSettingsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PrintTaskSettingsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &PrintTaskSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintTaskSettingsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PrintTaskSettingsActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProtocolActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl ProtocolActivatedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CallerPackageFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn Data(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::runtime::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `UI_ViewManagement`*"]
    pub fn ViewSwitcher(&self) -> ::windows::runtime::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::runtime::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProtocolActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ProtocolActivatedEventArgs;{6095f4dd-b7c0-46ab-81fe-d90f36d00d24})");
}
unsafe impl ::windows::runtime::Interface for ProtocolActivatedEventArgs {
    type Vtable = IProtocolActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1620440285, 47040, 18091, [129, 254, 217, 15, 54, 208, 13, 36]);
}
impl ::windows::runtime::RuntimeName for ProtocolActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ProtocolActivatedEventArgs";
}
impl ::core::convert::From<ProtocolActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ProtocolActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProtocolActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ProtocolActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProtocolActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ProtocolActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProtocolActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ProtocolActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IProtocolActivatedEventArgs> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IProtocolActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IProtocolActivatedEventArgs> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IProtocolActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IApplicationViewActivatedEventArgs> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IApplicationViewActivatedEventArgs> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
        ::core::convert::TryInto::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IViewSwitcherProvider> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IViewSwitcherProvider> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IViewSwitcherProvider> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ProtocolActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ProtocolActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &ProtocolActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for ProtocolActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ProtocolActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ProtocolForResultsActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl ProtocolForResultsActivatedEventArgs {
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn ProtocolForResultsOperation(&self) -> ::windows::runtime::Result<super::super::System::ProtocolForResultsOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::ProtocolForResultsOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<IProtocolActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CallerPackageFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn Data(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::runtime::Interface::cast::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `UI_ViewManagement`*"]
    pub fn ViewSwitcher(&self) -> ::windows::runtime::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::runtime::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProtocolForResultsActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ProtocolForResultsActivatedEventArgs;{e75132c2-7ae7-4517-80ac-dbe8d7cc5b9c})");
}
unsafe impl ::windows::runtime::Interface for ProtocolForResultsActivatedEventArgs {
    type Vtable = IProtocolForResultsActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3880858306, 31463, 17687, [128, 172, 219, 232, 215, 204, 91, 156]);
}
impl ::windows::runtime::RuntimeName for ProtocolForResultsActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ProtocolForResultsActivatedEventArgs";
}
impl ::core::convert::From<ProtocolForResultsActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ProtocolForResultsActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ProtocolForResultsActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ProtocolForResultsActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ProtocolForResultsActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ProtocolForResultsActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ProtocolForResultsActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ProtocolForResultsActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IProtocolForResultsActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IProtocolForResultsActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IProtocolForResultsActivatedEventArgs> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IProtocolForResultsActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IApplicationViewActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IApplicationViewActivatedEventArgs> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IProtocolActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IProtocolActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IProtocolActivatedEventArgs> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IProtocolActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IProtocolActivatedEventArgs> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IProtocolActivatedEventArgs> {
        ::core::convert::TryInto::<IProtocolActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
        ::core::convert::TryInto::<IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IViewSwitcherProvider> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IViewSwitcherProvider> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IViewSwitcherProvider> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<ProtocolForResultsActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ProtocolForResultsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ProtocolForResultsActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ProtocolForResultsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &ProtocolForResultsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for ProtocolForResultsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ProtocolForResultsActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RestrictedLaunchActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl RestrictedLaunchActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SharedContext(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RestrictedLaunchActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.RestrictedLaunchActivatedEventArgs;{e0b7ac81-bfc3-4344-a5da-19fd5a27baae})");
}
unsafe impl ::windows::runtime::Interface for RestrictedLaunchActivatedEventArgs {
    type Vtable = IRestrictedLaunchActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3770133633, 49091, 17220, [165, 218, 25, 253, 90, 39, 186, 174]);
}
impl ::windows::runtime::RuntimeName for RestrictedLaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.RestrictedLaunchActivatedEventArgs";
}
impl ::core::convert::From<RestrictedLaunchActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RestrictedLaunchActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RestrictedLaunchActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RestrictedLaunchActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RestrictedLaunchActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: RestrictedLaunchActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RestrictedLaunchActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &RestrictedLaunchActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IRestrictedLaunchActivatedEventArgs> for RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRestrictedLaunchActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRestrictedLaunchActivatedEventArgs> for &RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRestrictedLaunchActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<RestrictedLaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RestrictedLaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RestrictedLaunchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RestrictedLaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<RestrictedLaunchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RestrictedLaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&RestrictedLaunchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RestrictedLaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &RestrictedLaunchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for RestrictedLaunchActivatedEventArgs {}
unsafe impl ::core::marker::Sync for RestrictedLaunchActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SearchActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl SearchActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn QueryText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Search")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Search`*"]
    pub fn LinguisticDetails(&self) -> ::windows::runtime::Result<super::Search::SearchPaneQueryLinguisticDetails> {
        let this = &::windows::runtime::Interface::cast::<ISearchActivatedEventArgsWithLinguisticDetails>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Search::SearchPaneQueryLinguisticDetails>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `UI_ViewManagement`*"]
    pub fn ViewSwitcher(&self) -> ::windows::runtime::Result<super::super::UI::ViewManagement::ActivationViewSwitcher> {
        let this = &::windows::runtime::Interface::cast::<IViewSwitcherProvider>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ActivationViewSwitcher>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SearchActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.SearchActivatedEventArgs;{8cb36951-58c8-43e3-94bc-41d33f8b630e})");
}
unsafe impl ::windows::runtime::Interface for SearchActivatedEventArgs {
    type Vtable = ISearchActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2360568145, 22728, 17379, [148, 188, 65, 211, 63, 139, 99, 14]);
}
impl ::windows::runtime::RuntimeName for SearchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.SearchActivatedEventArgs";
}
impl ::core::convert::From<SearchActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SearchActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SearchActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SearchActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SearchActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SearchActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SearchActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SearchActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ISearchActivatedEventArgs> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISearchActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISearchActivatedEventArgs> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISearchActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IApplicationViewActivatedEventArgs> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IApplicationViewActivatedEventArgs> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for ISearchActivatedEventArgsWithLinguisticDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for ISearchActivatedEventArgsWithLinguisticDetails {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISearchActivatedEventArgsWithLinguisticDetails> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISearchActivatedEventArgsWithLinguisticDetails> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISearchActivatedEventArgsWithLinguisticDetails> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISearchActivatedEventArgsWithLinguisticDetails> {
        ::core::convert::TryInto::<ISearchActivatedEventArgsWithLinguisticDetails>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for IViewSwitcherProvider {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IViewSwitcherProvider> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IViewSwitcherProvider> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IViewSwitcherProvider> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IViewSwitcherProvider> {
        ::core::convert::TryInto::<IViewSwitcherProvider>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<SearchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SearchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SearchActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SearchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &SearchActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for SearchActivatedEventArgs {}
unsafe impl ::core::marker::Sync for SearchActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ShareTargetActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl ShareTargetActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_DataTransfer_ShareTarget`*"]
    pub fn ShareOperation(&self) -> ::windows::runtime::Result<super::DataTransfer::ShareTarget::ShareOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DataTransfer::ShareTarget::ShareOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ShareTargetActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ShareTargetActivatedEventArgs;{4bdaf9c8-cdb2-4acb-bfc3-6648563378ec})");
}
unsafe impl ::windows::runtime::Interface for ShareTargetActivatedEventArgs {
    type Vtable = IShareTargetActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1272641992, 52658, 19147, [191, 195, 102, 72, 86, 51, 120, 236]);
}
impl ::windows::runtime::RuntimeName for ShareTargetActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ShareTargetActivatedEventArgs";
}
impl ::core::convert::From<ShareTargetActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ShareTargetActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ShareTargetActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ShareTargetActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ShareTargetActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ShareTargetActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ShareTargetActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ShareTargetActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IShareTargetActivatedEventArgs> for ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShareTargetActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShareTargetActivatedEventArgs> for &ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShareTargetActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ShareTargetActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ShareTargetActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ShareTargetActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ShareTargetActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<ShareTargetActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ShareTargetActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ShareTargetActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ShareTargetActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &ShareTargetActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for ShareTargetActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ShareTargetActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SplashScreen(pub ::windows::runtime::IInspectable);
impl SplashScreen {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn ImageLocation(&self) -> ::windows::runtime::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn Dismissed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<SplashScreen, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation`*"]
    pub fn RemoveDismissed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SplashScreen {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.SplashScreen;{ca4d975c-d4d6-43f0-97c0-0833c6391c24})");
}
unsafe impl ::windows::runtime::Interface for SplashScreen {
    type Vtable = ISplashScreen_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3394082652, 54486, 17392, [151, 192, 8, 51, 198, 57, 28, 36]);
}
impl ::windows::runtime::RuntimeName for SplashScreen {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.SplashScreen";
}
impl ::core::convert::From<SplashScreen> for ::windows::runtime::IUnknown {
    fn from(value: SplashScreen) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SplashScreen> for ::windows::runtime::IUnknown {
    fn from(value: &SplashScreen) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SplashScreen {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a SplashScreen {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SplashScreen> for ::windows::runtime::IInspectable {
    fn from(value: SplashScreen) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SplashScreen> for ::windows::runtime::IInspectable {
    fn from(value: &SplashScreen) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SplashScreen {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SplashScreen {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct StartupTaskActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl StartupTaskActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn TaskId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for StartupTaskActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.StartupTaskActivatedEventArgs;{03b11a58-5276-4d91-8621-54611864d5fa})");
}
unsafe impl ::windows::runtime::Interface for StartupTaskActivatedEventArgs {
    type Vtable = IStartupTaskActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(61938264, 21110, 19857, [134, 33, 84, 97, 24, 100, 213, 250]);
}
impl ::windows::runtime::RuntimeName for StartupTaskActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.StartupTaskActivatedEventArgs";
}
impl ::core::convert::From<StartupTaskActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: StartupTaskActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&StartupTaskActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &StartupTaskActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<StartupTaskActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: StartupTaskActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&StartupTaskActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &StartupTaskActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<StartupTaskActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: StartupTaskActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StartupTaskActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &StartupTaskActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<StartupTaskActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: StartupTaskActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StartupTaskActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &StartupTaskActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
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
impl<'a> ::windows::runtime::IntoParam<'a, IStartupTaskActivatedEventArgs> for StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStartupTaskActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IStartupTaskActivatedEventArgs> for &StartupTaskActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStartupTaskActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StartupTaskActivatedEventArgs {}
unsafe impl ::core::marker::Sync for StartupTaskActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TileActivatedInfo(pub ::windows::runtime::IInspectable);
impl TileActivatedInfo {
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Notifications"))]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`, `UI_Notifications`*"]
    pub fn RecentlyShownNotifications(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::super::UI::Notifications::ShownTileNotification>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::UI::Notifications::ShownTileNotification>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TileActivatedInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.TileActivatedInfo;{80e4a3b1-3980-4f17-b738-89194e0b8f65})");
}
unsafe impl ::windows::runtime::Interface for TileActivatedInfo {
    type Vtable = ITileActivatedInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2162467761, 14720, 20247, [183, 56, 137, 25, 78, 11, 143, 101]);
}
impl ::windows::runtime::RuntimeName for TileActivatedInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.TileActivatedInfo";
}
impl ::core::convert::From<TileActivatedInfo> for ::windows::runtime::IUnknown {
    fn from(value: TileActivatedInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TileActivatedInfo> for ::windows::runtime::IUnknown {
    fn from(value: &TileActivatedInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TileActivatedInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a TileActivatedInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TileActivatedInfo> for ::windows::runtime::IInspectable {
    fn from(value: TileActivatedInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TileActivatedInfo> for ::windows::runtime::IInspectable {
    fn from(value: &TileActivatedInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TileActivatedInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TileActivatedInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TileActivatedInfo {}
unsafe impl ::core::marker::Sync for TileActivatedInfo {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ToastNotificationActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl ToastNotificationActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Argument(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn UserInput(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ToastNotificationActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.ToastNotificationActivatedEventArgs;{92a86f82-5290-431d-be85-c4aaeeb8685f})");
}
unsafe impl ::windows::runtime::Interface for ToastNotificationActivatedEventArgs {
    type Vtable = IToastNotificationActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2460512130, 21136, 17181, [190, 133, 196, 170, 238, 184, 104, 95]);
}
impl ::windows::runtime::RuntimeName for ToastNotificationActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ToastNotificationActivatedEventArgs";
}
impl ::core::convert::From<ToastNotificationActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ToastNotificationActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ToastNotificationActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ToastNotificationActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ToastNotificationActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ToastNotificationActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ToastNotificationActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ToastNotificationActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IToastNotificationActivatedEventArgs> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IToastNotificationActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IToastNotificationActivatedEventArgs> for &ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IToastNotificationActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ToastNotificationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ToastNotificationActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ToastNotificationActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ToastNotificationActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<ToastNotificationActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ToastNotificationActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ToastNotificationActivatedEventArgs> for IApplicationViewActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ToastNotificationActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IApplicationViewActivatedEventArgs> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IApplicationViewActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IApplicationViewActivatedEventArgs> for &ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IApplicationViewActivatedEventArgs> {
        ::core::convert::TryInto::<IApplicationViewActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<ToastNotificationActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ToastNotificationActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ToastNotificationActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ToastNotificationActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &ToastNotificationActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for ToastNotificationActivatedEventArgs {}
unsafe impl ::core::marker::Sync for ToastNotificationActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserDataAccountProviderActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl UserDataAccountProviderActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_UserDataAccounts_Provider`*"]
    pub fn Operation(&self) -> ::windows::runtime::Result<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UserDataAccounts::Provider::IUserDataAccountProviderOperation>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.UserDataAccountProviderActivatedEventArgs;{1bc9f723-8ef1-4a51-a63a-fe711eeab607})");
}
unsafe impl ::windows::runtime::Interface for UserDataAccountProviderActivatedEventArgs {
    type Vtable = IUserDataAccountProviderActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(466220835, 36593, 19025, [166, 58, 254, 113, 30, 234, 182, 7]);
}
impl ::windows::runtime::RuntimeName for UserDataAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.UserDataAccountProviderActivatedEventArgs";
}
impl ::core::convert::From<UserDataAccountProviderActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: UserDataAccountProviderActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserDataAccountProviderActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &UserDataAccountProviderActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserDataAccountProviderActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: UserDataAccountProviderActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserDataAccountProviderActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &UserDataAccountProviderActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<UserDataAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: UserDataAccountProviderActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UserDataAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &UserDataAccountProviderActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
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
impl<'a> ::windows::runtime::IntoParam<'a, IUserDataAccountProviderActivatedEventArgs> for UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IUserDataAccountProviderActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IUserDataAccountProviderActivatedEventArgs> for &UserDataAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IUserDataAccountProviderActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataAccountProviderActivatedEventArgs {}
unsafe impl ::core::marker::Sync for UserDataAccountProviderActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VoiceCommandActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl VoiceCommandActivatedEventArgs {
    #[cfg(feature = "Media_SpeechRecognition")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Media_SpeechRecognition`*"]
    pub fn Result(&self) -> ::windows::runtime::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::SpeechRecognition::SpeechRecognitionResult>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VoiceCommandActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.VoiceCommandActivatedEventArgs;{ab92dcfd-8d43-4de6-9775-20704b581b00})");
}
unsafe impl ::windows::runtime::Interface for VoiceCommandActivatedEventArgs {
    type Vtable = IVoiceCommandActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2878528765, 36163, 19942, [151, 117, 32, 112, 75, 88, 27, 0]);
}
impl ::windows::runtime::RuntimeName for VoiceCommandActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.VoiceCommandActivatedEventArgs";
}
impl ::core::convert::From<VoiceCommandActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: VoiceCommandActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VoiceCommandActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &VoiceCommandActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VoiceCommandActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: VoiceCommandActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VoiceCommandActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &VoiceCommandActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IVoiceCommandActivatedEventArgs> for VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVoiceCommandActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVoiceCommandActivatedEventArgs> for &VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVoiceCommandActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<VoiceCommandActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: VoiceCommandActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VoiceCommandActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &VoiceCommandActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<VoiceCommandActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: VoiceCommandActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&VoiceCommandActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &VoiceCommandActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &VoiceCommandActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for VoiceCommandActivatedEventArgs {}
unsafe impl ::core::marker::Sync for VoiceCommandActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WalletActionActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl WalletActionActivatedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ItemId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Wallet")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `ApplicationModel_Wallet`*"]
    pub fn ActionKind(&self) -> ::windows::runtime::Result<super::Wallet::WalletActionKind> {
        let this = self;
        unsafe {
            let mut result__: super::Wallet::WalletActionKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Wallet::WalletActionKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn ActionId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WalletActionActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.WalletActionActivatedEventArgs;{fcfc027b-1a1a-4d22-923f-ae6f45fa52d9})");
}
unsafe impl ::windows::runtime::Interface for WalletActionActivatedEventArgs {
    type Vtable = IWalletActionActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4244374139, 6682, 19746, [146, 63, 174, 111, 69, 250, 82, 217]);
}
impl ::windows::runtime::RuntimeName for WalletActionActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WalletActionActivatedEventArgs";
}
impl ::core::convert::From<WalletActionActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WalletActionActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WalletActionActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WalletActionActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WalletActionActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: WalletActionActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WalletActionActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &WalletActionActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IWalletActionActivatedEventArgs> for WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWalletActionActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWalletActionActivatedEventArgs> for &WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWalletActionActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WalletActionActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WalletActionActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WalletActionActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WalletActionActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &WalletActionActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for WalletActionActivatedEventArgs {}
unsafe impl ::core::marker::Sync for WalletActionActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WebAccountProviderActivatedEventArgs(pub ::windows::runtime::IInspectable);
impl WebAccountProviderActivatedEventArgs {
    #[cfg(feature = "Security_Authentication_Web_Provider")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Security_Authentication_Web_Provider`*"]
    pub fn Operation(&self) -> ::windows::runtime::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.WebAccountProviderActivatedEventArgs;{72b71774-98ea-4ccf-9752-46d9051004f1})");
}
unsafe impl ::windows::runtime::Interface for WebAccountProviderActivatedEventArgs {
    type Vtable = IWebAccountProviderActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1924601716, 39146, 19663, [151, 82, 70, 217, 5, 16, 4, 241]);
}
impl ::windows::runtime::RuntimeName for WebAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WebAccountProviderActivatedEventArgs";
}
impl ::core::convert::From<WebAccountProviderActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebAccountProviderActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WebAccountProviderActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebAccountProviderActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WebAccountProviderActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: WebAccountProviderActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WebAccountProviderActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &WebAccountProviderActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IWebAccountProviderActivatedEventArgs> for WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebAccountProviderActivatedEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWebAccountProviderActivatedEventArgs> for &WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebAccountProviderActivatedEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WebAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebAccountProviderActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderActivatedEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebAccountProviderActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<WebAccountProviderActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebAccountProviderActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAccountProviderActivatedEventArgs> for IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebAccountProviderActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsWithUser> for &WebAccountProviderActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebAccountProviderActivatedEventArgs {}
unsafe impl ::core::marker::Sync for WebAccountProviderActivatedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Activation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WebAuthenticationBrokerContinuationEventArgs(pub ::windows::runtime::IInspectable);
impl WebAuthenticationBrokerContinuationEventArgs {
    #[cfg(feature = "Security_Authentication_Web")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Security_Authentication_Web`*"]
    pub fn WebAuthenticationResult(&self) -> ::windows::runtime::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Security::Authentication::Web::WebAuthenticationResult>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ActivationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SplashScreen>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Activation`, `Foundation_Collections`*"]
    pub fn ContinuationData(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::runtime::Interface::cast::<IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebAuthenticationBrokerContinuationEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Activation.WebAuthenticationBrokerContinuationEventArgs;{75dda3d4-7714-453d-b7ff-b95e3a1709da})");
}
unsafe impl ::windows::runtime::Interface for WebAuthenticationBrokerContinuationEventArgs {
    type Vtable = IWebAuthenticationBrokerContinuationEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1977459668, 30484, 17725, [183, 255, 185, 94, 58, 23, 9, 218]);
}
impl ::windows::runtime::RuntimeName for WebAuthenticationBrokerContinuationEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.WebAuthenticationBrokerContinuationEventArgs";
}
impl ::core::convert::From<WebAuthenticationBrokerContinuationEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebAuthenticationBrokerContinuationEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WebAuthenticationBrokerContinuationEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebAuthenticationBrokerContinuationEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WebAuthenticationBrokerContinuationEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: WebAuthenticationBrokerContinuationEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WebAuthenticationBrokerContinuationEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &WebAuthenticationBrokerContinuationEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IWebAuthenticationBrokerContinuationEventArgs> for WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebAuthenticationBrokerContinuationEventArgs> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWebAuthenticationBrokerContinuationEventArgs> for &WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebAuthenticationBrokerContinuationEventArgs> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<WebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebAuthenticationBrokerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebAuthenticationBrokerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgs> for &WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgs> {
        ::core::convert::TryInto::<IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<WebAuthenticationBrokerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebAuthenticationBrokerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebAuthenticationBrokerContinuationEventArgs> for IContinuationActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebAuthenticationBrokerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContinuationActivatedEventArgs> for WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContinuationActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContinuationActivatedEventArgs> for &WebAuthenticationBrokerContinuationEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContinuationActivatedEventArgs> {
        ::core::convert::TryInto::<IContinuationActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebAuthenticationBrokerContinuationEventArgs {}
unsafe impl ::core::marker::Sync for WebAuthenticationBrokerContinuationEventArgs {}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct WebUISearchActivatedEventsContract(pub u8);
