#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppointmentCalendarCancelMeetingRequest(pub ::windows::runtime::IInspectable);
impl AppointmentCalendarCancelMeetingRequest {
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn AppointmentCalendarLocalId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn AppointmentLocalId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn AppointmentOriginalStartTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn Subject(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn NotifyInvitees(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppointmentCalendarCancelMeetingRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCancelMeetingRequest;{49460f8d-6434-40d7-ad46-6297419314d1})");
}
unsafe impl ::windows::runtime::Interface for AppointmentCalendarCancelMeetingRequest {
    type Vtable = IAppointmentCalendarCancelMeetingRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1229328269, 25652, 16599, [173, 70, 98, 151, 65, 147, 20, 209]);
}
impl ::windows::runtime::RuntimeName for AppointmentCalendarCancelMeetingRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCancelMeetingRequest";
}
impl ::std::convert::From<AppointmentCalendarCancelMeetingRequest> for ::windows::runtime::IUnknown {
    fn from(value: AppointmentCalendarCancelMeetingRequest) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AppointmentCalendarCancelMeetingRequest> for ::windows::runtime::IUnknown {
    fn from(value: &AppointmentCalendarCancelMeetingRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppointmentCalendarCancelMeetingRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppointmentCalendarCancelMeetingRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AppointmentCalendarCancelMeetingRequest> for ::windows::runtime::IInspectable {
    fn from(value: AppointmentCalendarCancelMeetingRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppointmentCalendarCancelMeetingRequest> for ::windows::runtime::IInspectable {
    fn from(value: &AppointmentCalendarCancelMeetingRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppointmentCalendarCancelMeetingRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppointmentCalendarCancelMeetingRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppointmentCalendarCancelMeetingRequest {}
unsafe impl ::std::marker::Sync for AppointmentCalendarCancelMeetingRequest {}
#[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppointmentCalendarCancelMeetingRequestEventArgs(pub ::windows::runtime::IInspectable);
impl AppointmentCalendarCancelMeetingRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<AppointmentCalendarCancelMeetingRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentCalendarCancelMeetingRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppointmentCalendarCancelMeetingRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCancelMeetingRequestEventArgs;{1a79be16-7f30-4e35-beef-9d2c7b6dcae1})");
}
unsafe impl ::windows::runtime::Interface for AppointmentCalendarCancelMeetingRequestEventArgs {
    type Vtable = IAppointmentCalendarCancelMeetingRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(444186134, 32560, 20021, [190, 239, 157, 44, 123, 109, 202, 225]);
}
impl ::windows::runtime::RuntimeName for AppointmentCalendarCancelMeetingRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCancelMeetingRequestEventArgs";
}
impl ::std::convert::From<AppointmentCalendarCancelMeetingRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AppointmentCalendarCancelMeetingRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AppointmentCalendarCancelMeetingRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AppointmentCalendarCancelMeetingRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppointmentCalendarCancelMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppointmentCalendarCancelMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AppointmentCalendarCancelMeetingRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AppointmentCalendarCancelMeetingRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppointmentCalendarCancelMeetingRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AppointmentCalendarCancelMeetingRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppointmentCalendarCancelMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppointmentCalendarCancelMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppointmentCalendarCancelMeetingRequestEventArgs {}
unsafe impl ::std::marker::Sync for AppointmentCalendarCancelMeetingRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppointmentCalendarCreateOrUpdateAppointmentRequest(pub ::windows::runtime::IInspectable);
impl AppointmentCalendarCreateOrUpdateAppointmentRequest {
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn AppointmentCalendarLocalId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn Appointment(&self) -> ::windows::runtime::Result<super::Appointment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Appointment>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn NotifyInvitees(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation_Collections`*"]
    pub fn ChangedProperties(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Appointment>>(&self, createdorupdatedappointment: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), createdorupdatedappointment.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppointmentCalendarCreateOrUpdateAppointmentRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCreateOrUpdateAppointmentRequest;{2e62f2b2-ca96-48ac-9124-406b19fefa70})");
}
unsafe impl ::windows::runtime::Interface for AppointmentCalendarCreateOrUpdateAppointmentRequest {
    type Vtable = IAppointmentCalendarCreateOrUpdateAppointmentRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(778236594, 51862, 18604, [145, 36, 64, 107, 25, 254, 250, 112]);
}
impl ::windows::runtime::RuntimeName for AppointmentCalendarCreateOrUpdateAppointmentRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCreateOrUpdateAppointmentRequest";
}
impl ::std::convert::From<AppointmentCalendarCreateOrUpdateAppointmentRequest> for ::windows::runtime::IUnknown {
    fn from(value: AppointmentCalendarCreateOrUpdateAppointmentRequest) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AppointmentCalendarCreateOrUpdateAppointmentRequest> for ::windows::runtime::IUnknown {
    fn from(value: &AppointmentCalendarCreateOrUpdateAppointmentRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppointmentCalendarCreateOrUpdateAppointmentRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppointmentCalendarCreateOrUpdateAppointmentRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AppointmentCalendarCreateOrUpdateAppointmentRequest> for ::windows::runtime::IInspectable {
    fn from(value: AppointmentCalendarCreateOrUpdateAppointmentRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppointmentCalendarCreateOrUpdateAppointmentRequest> for ::windows::runtime::IInspectable {
    fn from(value: &AppointmentCalendarCreateOrUpdateAppointmentRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppointmentCalendarCreateOrUpdateAppointmentRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppointmentCalendarCreateOrUpdateAppointmentRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppointmentCalendarCreateOrUpdateAppointmentRequest {}
unsafe impl ::std::marker::Sync for AppointmentCalendarCreateOrUpdateAppointmentRequest {}
#[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs(pub ::windows::runtime::IInspectable);
impl AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<AppointmentCalendarCreateOrUpdateAppointmentRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentCalendarCreateOrUpdateAppointmentRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs;{cf8ded28-002e-4bf7-8e9d-5e20d49aa3ba})");
}
unsafe impl ::windows::runtime::Interface for AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    type Vtable = IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3482185000, 46, 19447, [142, 157, 94, 32, 212, 154, 163, 186]);
}
impl ::windows::runtime::RuntimeName for AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs";
}
impl ::std::convert::From<AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {}
unsafe impl ::std::marker::Sync for AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppointmentCalendarForwardMeetingRequest(pub ::windows::runtime::IInspectable);
impl AppointmentCalendarForwardMeetingRequest {
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn AppointmentCalendarLocalId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn AppointmentLocalId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn AppointmentOriginalStartTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation_Collections`*"]
    pub fn Invitees(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<super::AppointmentInvitee>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::AppointmentInvitee>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn Subject(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn ForwardHeader(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppointmentCalendarForwardMeetingRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarForwardMeetingRequest;{82e5ee56-26b6-4253-8a8f-6cf5f2ff7884})");
}
unsafe impl ::windows::runtime::Interface for AppointmentCalendarForwardMeetingRequest {
    type Vtable = IAppointmentCalendarForwardMeetingRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2196106838, 9910, 16979, [138, 143, 108, 245, 242, 255, 120, 132]);
}
impl ::windows::runtime::RuntimeName for AppointmentCalendarForwardMeetingRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarForwardMeetingRequest";
}
impl ::std::convert::From<AppointmentCalendarForwardMeetingRequest> for ::windows::runtime::IUnknown {
    fn from(value: AppointmentCalendarForwardMeetingRequest) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AppointmentCalendarForwardMeetingRequest> for ::windows::runtime::IUnknown {
    fn from(value: &AppointmentCalendarForwardMeetingRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppointmentCalendarForwardMeetingRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppointmentCalendarForwardMeetingRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AppointmentCalendarForwardMeetingRequest> for ::windows::runtime::IInspectable {
    fn from(value: AppointmentCalendarForwardMeetingRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppointmentCalendarForwardMeetingRequest> for ::windows::runtime::IInspectable {
    fn from(value: &AppointmentCalendarForwardMeetingRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppointmentCalendarForwardMeetingRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppointmentCalendarForwardMeetingRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppointmentCalendarForwardMeetingRequest {}
unsafe impl ::std::marker::Sync for AppointmentCalendarForwardMeetingRequest {}
#[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppointmentCalendarForwardMeetingRequestEventArgs(pub ::windows::runtime::IInspectable);
impl AppointmentCalendarForwardMeetingRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<AppointmentCalendarForwardMeetingRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentCalendarForwardMeetingRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppointmentCalendarForwardMeetingRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarForwardMeetingRequestEventArgs;{3109151a-23a2-42fd-9c82-c9a60d59f8a8})");
}
unsafe impl ::windows::runtime::Interface for AppointmentCalendarForwardMeetingRequestEventArgs {
    type Vtable = IAppointmentCalendarForwardMeetingRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(822678810, 9122, 17149, [156, 130, 201, 166, 13, 89, 248, 168]);
}
impl ::windows::runtime::RuntimeName for AppointmentCalendarForwardMeetingRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarForwardMeetingRequestEventArgs";
}
impl ::std::convert::From<AppointmentCalendarForwardMeetingRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AppointmentCalendarForwardMeetingRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AppointmentCalendarForwardMeetingRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AppointmentCalendarForwardMeetingRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppointmentCalendarForwardMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppointmentCalendarForwardMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AppointmentCalendarForwardMeetingRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AppointmentCalendarForwardMeetingRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppointmentCalendarForwardMeetingRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AppointmentCalendarForwardMeetingRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppointmentCalendarForwardMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppointmentCalendarForwardMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppointmentCalendarForwardMeetingRequestEventArgs {}
unsafe impl ::std::marker::Sync for AppointmentCalendarForwardMeetingRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppointmentCalendarProposeNewTimeForMeetingRequest(pub ::windows::runtime::IInspectable);
impl AppointmentCalendarProposeNewTimeForMeetingRequest {
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn AppointmentCalendarLocalId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn AppointmentLocalId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn AppointmentOriginalStartTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn NewStartTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn NewDuration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn Subject(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppointmentCalendarProposeNewTimeForMeetingRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarProposeNewTimeForMeetingRequest;{ce1c63f5-edf6-43c3-82b7-be6b368c6900})");
}
unsafe impl ::windows::runtime::Interface for AppointmentCalendarProposeNewTimeForMeetingRequest {
    type Vtable = IAppointmentCalendarProposeNewTimeForMeetingRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3457967093, 60918, 17347, [130, 183, 190, 107, 54, 140, 105, 0]);
}
impl ::windows::runtime::RuntimeName for AppointmentCalendarProposeNewTimeForMeetingRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarProposeNewTimeForMeetingRequest";
}
impl ::std::convert::From<AppointmentCalendarProposeNewTimeForMeetingRequest> for ::windows::runtime::IUnknown {
    fn from(value: AppointmentCalendarProposeNewTimeForMeetingRequest) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AppointmentCalendarProposeNewTimeForMeetingRequest> for ::windows::runtime::IUnknown {
    fn from(value: &AppointmentCalendarProposeNewTimeForMeetingRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppointmentCalendarProposeNewTimeForMeetingRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppointmentCalendarProposeNewTimeForMeetingRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AppointmentCalendarProposeNewTimeForMeetingRequest> for ::windows::runtime::IInspectable {
    fn from(value: AppointmentCalendarProposeNewTimeForMeetingRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppointmentCalendarProposeNewTimeForMeetingRequest> for ::windows::runtime::IInspectable {
    fn from(value: &AppointmentCalendarProposeNewTimeForMeetingRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppointmentCalendarProposeNewTimeForMeetingRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppointmentCalendarProposeNewTimeForMeetingRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppointmentCalendarProposeNewTimeForMeetingRequest {}
unsafe impl ::std::marker::Sync for AppointmentCalendarProposeNewTimeForMeetingRequest {}
#[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs(pub ::windows::runtime::IInspectable);
impl AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<AppointmentCalendarProposeNewTimeForMeetingRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentCalendarProposeNewTimeForMeetingRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs;{d2d777d8-fed1-4280-a3ba-2e1f47609aa2})");
}
unsafe impl ::windows::runtime::Interface for AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    type Vtable = IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3537336280, 65233, 17024, [163, 186, 46, 31, 71, 96, 154, 162]);
}
impl ::windows::runtime::RuntimeName for AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs";
}
impl ::std::convert::From<AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {}
unsafe impl ::std::marker::Sync for AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppointmentCalendarSyncManagerSyncRequest(pub ::windows::runtime::IInspectable);
impl AppointmentCalendarSyncManagerSyncRequest {
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn AppointmentCalendarLocalId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppointmentCalendarSyncManagerSyncRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarSyncManagerSyncRequest;{12ab382b-7163-4a56-9a4e-7223a84adf46})");
}
unsafe impl ::windows::runtime::Interface for AppointmentCalendarSyncManagerSyncRequest {
    type Vtable = IAppointmentCalendarSyncManagerSyncRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(313210923, 29027, 19030, [154, 78, 114, 35, 168, 74, 223, 70]);
}
impl ::windows::runtime::RuntimeName for AppointmentCalendarSyncManagerSyncRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarSyncManagerSyncRequest";
}
impl ::std::convert::From<AppointmentCalendarSyncManagerSyncRequest> for ::windows::runtime::IUnknown {
    fn from(value: AppointmentCalendarSyncManagerSyncRequest) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AppointmentCalendarSyncManagerSyncRequest> for ::windows::runtime::IUnknown {
    fn from(value: &AppointmentCalendarSyncManagerSyncRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppointmentCalendarSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppointmentCalendarSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AppointmentCalendarSyncManagerSyncRequest> for ::windows::runtime::IInspectable {
    fn from(value: AppointmentCalendarSyncManagerSyncRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppointmentCalendarSyncManagerSyncRequest> for ::windows::runtime::IInspectable {
    fn from(value: &AppointmentCalendarSyncManagerSyncRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppointmentCalendarSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppointmentCalendarSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppointmentCalendarSyncManagerSyncRequest {}
unsafe impl ::std::marker::Sync for AppointmentCalendarSyncManagerSyncRequest {}
#[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppointmentCalendarSyncManagerSyncRequestEventArgs(pub ::windows::runtime::IInspectable);
impl AppointmentCalendarSyncManagerSyncRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<AppointmentCalendarSyncManagerSyncRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentCalendarSyncManagerSyncRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppointmentCalendarSyncManagerSyncRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarSyncManagerSyncRequestEventArgs;{ca17c6f7-0284-4edd-87ba-4d8f69dcf5c0})");
}
unsafe impl ::windows::runtime::Interface for AppointmentCalendarSyncManagerSyncRequestEventArgs {
    type Vtable = IAppointmentCalendarSyncManagerSyncRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3390555895, 644, 20189, [135, 186, 77, 143, 105, 220, 245, 192]);
}
impl ::windows::runtime::RuntimeName for AppointmentCalendarSyncManagerSyncRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarSyncManagerSyncRequestEventArgs";
}
impl ::std::convert::From<AppointmentCalendarSyncManagerSyncRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AppointmentCalendarSyncManagerSyncRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AppointmentCalendarSyncManagerSyncRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AppointmentCalendarSyncManagerSyncRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppointmentCalendarSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppointmentCalendarSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AppointmentCalendarSyncManagerSyncRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AppointmentCalendarSyncManagerSyncRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppointmentCalendarSyncManagerSyncRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AppointmentCalendarSyncManagerSyncRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppointmentCalendarSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppointmentCalendarSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppointmentCalendarSyncManagerSyncRequestEventArgs {}
unsafe impl ::std::marker::Sync for AppointmentCalendarSyncManagerSyncRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppointmentCalendarUpdateMeetingResponseRequest(pub ::windows::runtime::IInspectable);
impl AppointmentCalendarUpdateMeetingResponseRequest {
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn AppointmentCalendarLocalId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn AppointmentLocalId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn AppointmentOriginalStartTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn Response(&self) -> ::windows::runtime::Result<super::AppointmentParticipantResponse> {
        let this = self;
        unsafe {
            let mut result__: super::AppointmentParticipantResponse = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::AppointmentParticipantResponse>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn Subject(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn SendUpdate(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppointmentCalendarUpdateMeetingResponseRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarUpdateMeetingResponseRequest;{a36d608c-c29d-4b94-b086-7e9ff7bd84a0})");
}
unsafe impl ::windows::runtime::Interface for AppointmentCalendarUpdateMeetingResponseRequest {
    type Vtable = IAppointmentCalendarUpdateMeetingResponseRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2741854348, 49821, 19348, [176, 134, 126, 159, 247, 189, 132, 160]);
}
impl ::windows::runtime::RuntimeName for AppointmentCalendarUpdateMeetingResponseRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarUpdateMeetingResponseRequest";
}
impl ::std::convert::From<AppointmentCalendarUpdateMeetingResponseRequest> for ::windows::runtime::IUnknown {
    fn from(value: AppointmentCalendarUpdateMeetingResponseRequest) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AppointmentCalendarUpdateMeetingResponseRequest> for ::windows::runtime::IUnknown {
    fn from(value: &AppointmentCalendarUpdateMeetingResponseRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppointmentCalendarUpdateMeetingResponseRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppointmentCalendarUpdateMeetingResponseRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AppointmentCalendarUpdateMeetingResponseRequest> for ::windows::runtime::IInspectable {
    fn from(value: AppointmentCalendarUpdateMeetingResponseRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppointmentCalendarUpdateMeetingResponseRequest> for ::windows::runtime::IInspectable {
    fn from(value: &AppointmentCalendarUpdateMeetingResponseRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppointmentCalendarUpdateMeetingResponseRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppointmentCalendarUpdateMeetingResponseRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppointmentCalendarUpdateMeetingResponseRequest {}
unsafe impl ::std::marker::Sync for AppointmentCalendarUpdateMeetingResponseRequest {}
#[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppointmentCalendarUpdateMeetingResponseRequestEventArgs(pub ::windows::runtime::IInspectable);
impl AppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<AppointmentCalendarUpdateMeetingResponseRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentCalendarUpdateMeetingResponseRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarUpdateMeetingResponseRequestEventArgs;{88759883-97bf-479d-aed5-0be8ce567d1e})");
}
unsafe impl ::windows::runtime::Interface for AppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    type Vtable = IAppointmentCalendarUpdateMeetingResponseRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2289408131, 38847, 18333, [174, 213, 11, 232, 206, 86, 125, 30]);
}
impl ::windows::runtime::RuntimeName for AppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarUpdateMeetingResponseRequestEventArgs";
}
impl ::std::convert::From<AppointmentCalendarUpdateMeetingResponseRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: AppointmentCalendarUpdateMeetingResponseRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AppointmentCalendarUpdateMeetingResponseRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &AppointmentCalendarUpdateMeetingResponseRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AppointmentCalendarUpdateMeetingResponseRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: AppointmentCalendarUpdateMeetingResponseRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppointmentCalendarUpdateMeetingResponseRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &AppointmentCalendarUpdateMeetingResponseRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppointmentCalendarUpdateMeetingResponseRequestEventArgs {}
unsafe impl ::std::marker::Sync for AppointmentCalendarUpdateMeetingResponseRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppointmentDataProviderConnection(pub ::windows::runtime::IInspectable);
impl AppointmentDataProviderConnection {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn SyncRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarSyncManagerSyncRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn RemoveSyncRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn CreateOrUpdateAppointmentRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn RemoveCreateOrUpdateAppointmentRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn CancelMeetingRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarCancelMeetingRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn RemoveCancelMeetingRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn ForwardMeetingRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarForwardMeetingRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn RemoveForwardMeetingRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn ProposeNewTimeForMeetingRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn RemoveProposeNewTimeForMeetingRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn UpdateMeetingResponseRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarUpdateMeetingResponseRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`, `Foundation`*"]
    pub fn RemoveUpdateMeetingResponseRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppointmentDataProviderConnection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderConnection;{f3dd9d83-3254-465f-abdb-928046552cf4})");
}
unsafe impl ::windows::runtime::Interface for AppointmentDataProviderConnection {
    type Vtable = IAppointmentDataProviderConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4091387267, 12884, 18015, [171, 219, 146, 128, 70, 85, 44, 244]);
}
impl ::windows::runtime::RuntimeName for AppointmentDataProviderConnection {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderConnection";
}
impl ::std::convert::From<AppointmentDataProviderConnection> for ::windows::runtime::IUnknown {
    fn from(value: AppointmentDataProviderConnection) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AppointmentDataProviderConnection> for ::windows::runtime::IUnknown {
    fn from(value: &AppointmentDataProviderConnection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppointmentDataProviderConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppointmentDataProviderConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AppointmentDataProviderConnection> for ::windows::runtime::IInspectable {
    fn from(value: AppointmentDataProviderConnection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppointmentDataProviderConnection> for ::windows::runtime::IInspectable {
    fn from(value: &AppointmentDataProviderConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppointmentDataProviderConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppointmentDataProviderConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppointmentDataProviderConnection {}
unsafe impl ::std::marker::Sync for AppointmentDataProviderConnection {}
#[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AppointmentDataProviderTriggerDetails(pub ::windows::runtime::IInspectable);
impl AppointmentDataProviderTriggerDetails {
    #[doc = "*Required features: `ApplicationModel_Appointments_DataProvider`*"]
    pub fn Connection(&self) -> ::windows::runtime::Result<AppointmentDataProviderConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentDataProviderConnection>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppointmentDataProviderTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderTriggerDetails;{b3283c01-7e12-4e5e-b1ef-74fb68ac6f2a})");
}
unsafe impl ::windows::runtime::Interface for AppointmentDataProviderTriggerDetails {
    type Vtable = IAppointmentDataProviderTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3005758465, 32274, 20062, [177, 239, 116, 251, 104, 172, 111, 42]);
}
impl ::windows::runtime::RuntimeName for AppointmentDataProviderTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderTriggerDetails";
}
impl ::std::convert::From<AppointmentDataProviderTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: AppointmentDataProviderTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&AppointmentDataProviderTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &AppointmentDataProviderTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AppointmentDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a AppointmentDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<AppointmentDataProviderTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: AppointmentDataProviderTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AppointmentDataProviderTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &AppointmentDataProviderTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AppointmentDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AppointmentDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AppointmentDataProviderTriggerDetails {}
unsafe impl ::std::marker::Sync for AppointmentDataProviderTriggerDetails {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentCalendarCancelMeetingRequest(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppointmentCalendarCancelMeetingRequest {
    type Vtable = IAppointmentCalendarCancelMeetingRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1229328269, 25652, 16599, [173, 70, 98, 151, 65, 147, 20, 209]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarCancelMeetingRequest_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentCalendarCancelMeetingRequestEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppointmentCalendarCancelMeetingRequestEventArgs {
    type Vtable = IAppointmentCalendarCancelMeetingRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(444186134, 32560, 20021, [190, 239, 157, 44, 123, 109, 202, 225]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarCancelMeetingRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentCalendarCreateOrUpdateAppointmentRequest(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppointmentCalendarCreateOrUpdateAppointmentRequest {
    type Vtable = IAppointmentCalendarCreateOrUpdateAppointmentRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(778236594, 51862, 18604, [145, 36, 64, 107, 25, 254, 250, 112]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarCreateOrUpdateAppointmentRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, createdorupdatedappointment: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    type Vtable = IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3482185000, 46, 19447, [142, 157, 94, 32, 212, 154, 163, 186]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentCalendarForwardMeetingRequest(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppointmentCalendarForwardMeetingRequest {
    type Vtable = IAppointmentCalendarForwardMeetingRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2196106838, 9910, 16979, [138, 143, 108, 245, 242, 255, 120, 132]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarForwardMeetingRequest_abi(
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
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentCalendarForwardMeetingRequestEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppointmentCalendarForwardMeetingRequestEventArgs {
    type Vtable = IAppointmentCalendarForwardMeetingRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(822678810, 9122, 17149, [156, 130, 201, 166, 13, 89, 248, 168]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarForwardMeetingRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentCalendarProposeNewTimeForMeetingRequest(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppointmentCalendarProposeNewTimeForMeetingRequest {
    type Vtable = IAppointmentCalendarProposeNewTimeForMeetingRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3457967093, 60918, 17347, [130, 183, 190, 107, 54, 140, 105, 0]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarProposeNewTimeForMeetingRequest_abi(
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
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    type Vtable = IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3537336280, 65233, 17024, [163, 186, 46, 31, 71, 96, 154, 162]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentCalendarSyncManagerSyncRequest(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppointmentCalendarSyncManagerSyncRequest {
    type Vtable = IAppointmentCalendarSyncManagerSyncRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(313210923, 29027, 19030, [154, 78, 114, 35, 168, 74, 223, 70]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarSyncManagerSyncRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentCalendarSyncManagerSyncRequestEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppointmentCalendarSyncManagerSyncRequestEventArgs {
    type Vtable = IAppointmentCalendarSyncManagerSyncRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3390555895, 644, 20189, [135, 186, 77, 143, 105, 220, 245, 192]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarSyncManagerSyncRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentCalendarUpdateMeetingResponseRequest(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppointmentCalendarUpdateMeetingResponseRequest {
    type Vtable = IAppointmentCalendarUpdateMeetingResponseRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2741854348, 49821, 19348, [176, 134, 126, 159, 247, 189, 132, 160]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarUpdateMeetingResponseRequest_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::AppointmentParticipantResponse) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentCalendarUpdateMeetingResponseRequestEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    type Vtable = IAppointmentCalendarUpdateMeetingResponseRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2289408131, 38847, 18333, [174, 213, 11, 232, 206, 86, 125, 30]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarUpdateMeetingResponseRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentDataProviderConnection(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppointmentDataProviderConnection {
    type Vtable = IAppointmentDataProviderConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4091387267, 12884, 18015, [171, 219, 146, 128, 70, 85, 44, 244]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentDataProviderConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppointmentDataProviderTriggerDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppointmentDataProviderTriggerDetails {
    type Vtable = IAppointmentDataProviderTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3005758465, 32274, 20062, [177, 239, 116, 251, 104, 172, 111, 42]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentDataProviderTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
