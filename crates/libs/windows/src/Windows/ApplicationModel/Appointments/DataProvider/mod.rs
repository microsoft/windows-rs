#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
#[repr(transparent)]
pub struct AppointmentCalendarCancelMeetingRequest(::windows::core::IUnknown);
impl AppointmentCalendarCancelMeetingRequest {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn AppointmentCalendarLocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppointmentCalendarLocalId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn AppointmentLocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppointmentLocalId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AppointmentOriginalStartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppointmentOriginalStartTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Subject)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Comment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn NotifyInvitees(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NotifyInvitees)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportCompletedAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportFailedAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentCalendarCancelMeetingRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentCalendarCancelMeetingRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentCalendarCancelMeetingRequest {}
impl ::core::fmt::Debug for AppointmentCalendarCancelMeetingRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarCancelMeetingRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentCalendarCancelMeetingRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCancelMeetingRequest;{49460f8d-6434-40d7-ad46-6297419314d1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentCalendarCancelMeetingRequest {
    type Vtable = IAppointmentCalendarCancelMeetingRequest_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentCalendarCancelMeetingRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentCalendarCancelMeetingRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCancelMeetingRequest";
}
impl ::core::convert::From<AppointmentCalendarCancelMeetingRequest> for ::windows::core::IUnknown {
    fn from(value: AppointmentCalendarCancelMeetingRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarCancelMeetingRequest> for ::windows::core::IUnknown {
    fn from(value: &AppointmentCalendarCancelMeetingRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentCalendarCancelMeetingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentCalendarCancelMeetingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentCalendarCancelMeetingRequest> for ::windows::core::IInspectable {
    fn from(value: AppointmentCalendarCancelMeetingRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarCancelMeetingRequest> for ::windows::core::IInspectable {
    fn from(value: &AppointmentCalendarCancelMeetingRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentCalendarCancelMeetingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentCalendarCancelMeetingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentCalendarCancelMeetingRequest {}
unsafe impl ::core::marker::Sync for AppointmentCalendarCancelMeetingRequest {}
#[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
#[repr(transparent)]
pub struct AppointmentCalendarCancelMeetingRequestEventArgs(::windows::core::IUnknown);
impl AppointmentCalendarCancelMeetingRequestEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn Request(&self) -> ::windows::core::Result<AppointmentCalendarCancelMeetingRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Request)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentCalendarCancelMeetingRequest>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentCalendarCancelMeetingRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentCalendarCancelMeetingRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentCalendarCancelMeetingRequestEventArgs {}
impl ::core::fmt::Debug for AppointmentCalendarCancelMeetingRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarCancelMeetingRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentCalendarCancelMeetingRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCancelMeetingRequestEventArgs;{1a79be16-7f30-4e35-beef-9d2c7b6dcae1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentCalendarCancelMeetingRequestEventArgs {
    type Vtable = IAppointmentCalendarCancelMeetingRequestEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentCalendarCancelMeetingRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentCalendarCancelMeetingRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCancelMeetingRequestEventArgs";
}
impl ::core::convert::From<AppointmentCalendarCancelMeetingRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppointmentCalendarCancelMeetingRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarCancelMeetingRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppointmentCalendarCancelMeetingRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentCalendarCancelMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentCalendarCancelMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentCalendarCancelMeetingRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppointmentCalendarCancelMeetingRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarCancelMeetingRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppointmentCalendarCancelMeetingRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentCalendarCancelMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentCalendarCancelMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentCalendarCancelMeetingRequestEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentCalendarCancelMeetingRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
#[repr(transparent)]
pub struct AppointmentCalendarCreateOrUpdateAppointmentRequest(::windows::core::IUnknown);
impl AppointmentCalendarCreateOrUpdateAppointmentRequest {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn AppointmentCalendarLocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppointmentCalendarLocalId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn Appointment(&self) -> ::windows::core::Result<super::Appointment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Appointment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Appointment>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn NotifyInvitees(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NotifyInvitees)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ChangedProperties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChangedProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Appointment>>(&self, createdorupdatedappointment: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportCompletedAsync)(::core::mem::transmute_copy(this), createdorupdatedappointment.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportFailedAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentCalendarCreateOrUpdateAppointmentRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentCalendarCreateOrUpdateAppointmentRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentCalendarCreateOrUpdateAppointmentRequest {}
impl ::core::fmt::Debug for AppointmentCalendarCreateOrUpdateAppointmentRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarCreateOrUpdateAppointmentRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentCalendarCreateOrUpdateAppointmentRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCreateOrUpdateAppointmentRequest;{2e62f2b2-ca96-48ac-9124-406b19fefa70})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentCalendarCreateOrUpdateAppointmentRequest {
    type Vtable = IAppointmentCalendarCreateOrUpdateAppointmentRequest_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentCalendarCreateOrUpdateAppointmentRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentCalendarCreateOrUpdateAppointmentRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCreateOrUpdateAppointmentRequest";
}
impl ::core::convert::From<AppointmentCalendarCreateOrUpdateAppointmentRequest> for ::windows::core::IUnknown {
    fn from(value: AppointmentCalendarCreateOrUpdateAppointmentRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarCreateOrUpdateAppointmentRequest> for ::windows::core::IUnknown {
    fn from(value: &AppointmentCalendarCreateOrUpdateAppointmentRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentCalendarCreateOrUpdateAppointmentRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentCalendarCreateOrUpdateAppointmentRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentCalendarCreateOrUpdateAppointmentRequest> for ::windows::core::IInspectable {
    fn from(value: AppointmentCalendarCreateOrUpdateAppointmentRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarCreateOrUpdateAppointmentRequest> for ::windows::core::IInspectable {
    fn from(value: &AppointmentCalendarCreateOrUpdateAppointmentRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentCalendarCreateOrUpdateAppointmentRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentCalendarCreateOrUpdateAppointmentRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentCalendarCreateOrUpdateAppointmentRequest {}
unsafe impl ::core::marker::Sync for AppointmentCalendarCreateOrUpdateAppointmentRequest {}
#[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
#[repr(transparent)]
pub struct AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs(::windows::core::IUnknown);
impl AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn Request(&self) -> ::windows::core::Result<AppointmentCalendarCreateOrUpdateAppointmentRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Request)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentCalendarCreateOrUpdateAppointmentRequest>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {}
impl ::core::fmt::Debug for AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs;{cf8ded28-002e-4bf7-8e9d-5e20d49aa3ba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    type Vtable = IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs";
}
impl ::core::convert::From<AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
#[repr(transparent)]
pub struct AppointmentCalendarForwardMeetingRequest(::windows::core::IUnknown);
impl AppointmentCalendarForwardMeetingRequest {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn AppointmentCalendarLocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppointmentCalendarLocalId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn AppointmentLocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppointmentLocalId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AppointmentOriginalStartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppointmentOriginalStartTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Invitees(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::AppointmentInvitee>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Invitees)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::AppointmentInvitee>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Subject)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn ForwardHeader(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ForwardHeader)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Comment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportCompletedAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportFailedAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentCalendarForwardMeetingRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentCalendarForwardMeetingRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentCalendarForwardMeetingRequest {}
impl ::core::fmt::Debug for AppointmentCalendarForwardMeetingRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarForwardMeetingRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentCalendarForwardMeetingRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarForwardMeetingRequest;{82e5ee56-26b6-4253-8a8f-6cf5f2ff7884})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentCalendarForwardMeetingRequest {
    type Vtable = IAppointmentCalendarForwardMeetingRequest_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentCalendarForwardMeetingRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentCalendarForwardMeetingRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarForwardMeetingRequest";
}
impl ::core::convert::From<AppointmentCalendarForwardMeetingRequest> for ::windows::core::IUnknown {
    fn from(value: AppointmentCalendarForwardMeetingRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarForwardMeetingRequest> for ::windows::core::IUnknown {
    fn from(value: &AppointmentCalendarForwardMeetingRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentCalendarForwardMeetingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentCalendarForwardMeetingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentCalendarForwardMeetingRequest> for ::windows::core::IInspectable {
    fn from(value: AppointmentCalendarForwardMeetingRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarForwardMeetingRequest> for ::windows::core::IInspectable {
    fn from(value: &AppointmentCalendarForwardMeetingRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentCalendarForwardMeetingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentCalendarForwardMeetingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentCalendarForwardMeetingRequest {}
unsafe impl ::core::marker::Sync for AppointmentCalendarForwardMeetingRequest {}
#[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
#[repr(transparent)]
pub struct AppointmentCalendarForwardMeetingRequestEventArgs(::windows::core::IUnknown);
impl AppointmentCalendarForwardMeetingRequestEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn Request(&self) -> ::windows::core::Result<AppointmentCalendarForwardMeetingRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Request)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentCalendarForwardMeetingRequest>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentCalendarForwardMeetingRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentCalendarForwardMeetingRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentCalendarForwardMeetingRequestEventArgs {}
impl ::core::fmt::Debug for AppointmentCalendarForwardMeetingRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarForwardMeetingRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentCalendarForwardMeetingRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarForwardMeetingRequestEventArgs;{3109151a-23a2-42fd-9c82-c9a60d59f8a8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentCalendarForwardMeetingRequestEventArgs {
    type Vtable = IAppointmentCalendarForwardMeetingRequestEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentCalendarForwardMeetingRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentCalendarForwardMeetingRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarForwardMeetingRequestEventArgs";
}
impl ::core::convert::From<AppointmentCalendarForwardMeetingRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppointmentCalendarForwardMeetingRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarForwardMeetingRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppointmentCalendarForwardMeetingRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentCalendarForwardMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentCalendarForwardMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentCalendarForwardMeetingRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppointmentCalendarForwardMeetingRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarForwardMeetingRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppointmentCalendarForwardMeetingRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentCalendarForwardMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentCalendarForwardMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentCalendarForwardMeetingRequestEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentCalendarForwardMeetingRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
#[repr(transparent)]
pub struct AppointmentCalendarProposeNewTimeForMeetingRequest(::windows::core::IUnknown);
impl AppointmentCalendarProposeNewTimeForMeetingRequest {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn AppointmentCalendarLocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppointmentCalendarLocalId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn AppointmentLocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppointmentLocalId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AppointmentOriginalStartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppointmentOriginalStartTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NewStartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NewStartTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NewDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NewDuration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Subject)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Comment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportCompletedAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportFailedAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentCalendarProposeNewTimeForMeetingRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentCalendarProposeNewTimeForMeetingRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentCalendarProposeNewTimeForMeetingRequest {}
impl ::core::fmt::Debug for AppointmentCalendarProposeNewTimeForMeetingRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarProposeNewTimeForMeetingRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentCalendarProposeNewTimeForMeetingRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarProposeNewTimeForMeetingRequest;{ce1c63f5-edf6-43c3-82b7-be6b368c6900})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentCalendarProposeNewTimeForMeetingRequest {
    type Vtable = IAppointmentCalendarProposeNewTimeForMeetingRequest_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentCalendarProposeNewTimeForMeetingRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentCalendarProposeNewTimeForMeetingRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarProposeNewTimeForMeetingRequest";
}
impl ::core::convert::From<AppointmentCalendarProposeNewTimeForMeetingRequest> for ::windows::core::IUnknown {
    fn from(value: AppointmentCalendarProposeNewTimeForMeetingRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarProposeNewTimeForMeetingRequest> for ::windows::core::IUnknown {
    fn from(value: &AppointmentCalendarProposeNewTimeForMeetingRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentCalendarProposeNewTimeForMeetingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentCalendarProposeNewTimeForMeetingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentCalendarProposeNewTimeForMeetingRequest> for ::windows::core::IInspectable {
    fn from(value: AppointmentCalendarProposeNewTimeForMeetingRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarProposeNewTimeForMeetingRequest> for ::windows::core::IInspectable {
    fn from(value: &AppointmentCalendarProposeNewTimeForMeetingRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentCalendarProposeNewTimeForMeetingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentCalendarProposeNewTimeForMeetingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentCalendarProposeNewTimeForMeetingRequest {}
unsafe impl ::core::marker::Sync for AppointmentCalendarProposeNewTimeForMeetingRequest {}
#[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
#[repr(transparent)]
pub struct AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs(::windows::core::IUnknown);
impl AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn Request(&self) -> ::windows::core::Result<AppointmentCalendarProposeNewTimeForMeetingRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Request)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentCalendarProposeNewTimeForMeetingRequest>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {}
impl ::core::fmt::Debug for AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs;{d2d777d8-fed1-4280-a3ba-2e1f47609aa2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    type Vtable = IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs";
}
impl ::core::convert::From<AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
#[repr(transparent)]
pub struct AppointmentCalendarSyncManagerSyncRequest(::windows::core::IUnknown);
impl AppointmentCalendarSyncManagerSyncRequest {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn AppointmentCalendarLocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppointmentCalendarLocalId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportCompletedAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportFailedAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentCalendarSyncManagerSyncRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentCalendarSyncManagerSyncRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentCalendarSyncManagerSyncRequest {}
impl ::core::fmt::Debug for AppointmentCalendarSyncManagerSyncRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarSyncManagerSyncRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentCalendarSyncManagerSyncRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarSyncManagerSyncRequest;{12ab382b-7163-4a56-9a4e-7223a84adf46})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentCalendarSyncManagerSyncRequest {
    type Vtable = IAppointmentCalendarSyncManagerSyncRequest_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentCalendarSyncManagerSyncRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentCalendarSyncManagerSyncRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarSyncManagerSyncRequest";
}
impl ::core::convert::From<AppointmentCalendarSyncManagerSyncRequest> for ::windows::core::IUnknown {
    fn from(value: AppointmentCalendarSyncManagerSyncRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarSyncManagerSyncRequest> for ::windows::core::IUnknown {
    fn from(value: &AppointmentCalendarSyncManagerSyncRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentCalendarSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentCalendarSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentCalendarSyncManagerSyncRequest> for ::windows::core::IInspectable {
    fn from(value: AppointmentCalendarSyncManagerSyncRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarSyncManagerSyncRequest> for ::windows::core::IInspectable {
    fn from(value: &AppointmentCalendarSyncManagerSyncRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentCalendarSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentCalendarSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentCalendarSyncManagerSyncRequest {}
unsafe impl ::core::marker::Sync for AppointmentCalendarSyncManagerSyncRequest {}
#[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
#[repr(transparent)]
pub struct AppointmentCalendarSyncManagerSyncRequestEventArgs(::windows::core::IUnknown);
impl AppointmentCalendarSyncManagerSyncRequestEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn Request(&self) -> ::windows::core::Result<AppointmentCalendarSyncManagerSyncRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Request)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentCalendarSyncManagerSyncRequest>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentCalendarSyncManagerSyncRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentCalendarSyncManagerSyncRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentCalendarSyncManagerSyncRequestEventArgs {}
impl ::core::fmt::Debug for AppointmentCalendarSyncManagerSyncRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarSyncManagerSyncRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentCalendarSyncManagerSyncRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarSyncManagerSyncRequestEventArgs;{ca17c6f7-0284-4edd-87ba-4d8f69dcf5c0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentCalendarSyncManagerSyncRequestEventArgs {
    type Vtable = IAppointmentCalendarSyncManagerSyncRequestEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentCalendarSyncManagerSyncRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentCalendarSyncManagerSyncRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarSyncManagerSyncRequestEventArgs";
}
impl ::core::convert::From<AppointmentCalendarSyncManagerSyncRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppointmentCalendarSyncManagerSyncRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarSyncManagerSyncRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppointmentCalendarSyncManagerSyncRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentCalendarSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentCalendarSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentCalendarSyncManagerSyncRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppointmentCalendarSyncManagerSyncRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarSyncManagerSyncRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppointmentCalendarSyncManagerSyncRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentCalendarSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentCalendarSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentCalendarSyncManagerSyncRequestEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentCalendarSyncManagerSyncRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
#[repr(transparent)]
pub struct AppointmentCalendarUpdateMeetingResponseRequest(::windows::core::IUnknown);
impl AppointmentCalendarUpdateMeetingResponseRequest {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn AppointmentCalendarLocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppointmentCalendarLocalId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn AppointmentLocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppointmentLocalId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AppointmentOriginalStartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppointmentOriginalStartTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn Response(&self) -> ::windows::core::Result<super::AppointmentParticipantResponse> {
        let this = self;
        unsafe {
            let mut result__: super::AppointmentParticipantResponse = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Response)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::AppointmentParticipantResponse>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Subject)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Comment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn SendUpdate(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SendUpdate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportCompletedAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReportFailedAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentCalendarUpdateMeetingResponseRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentCalendarUpdateMeetingResponseRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentCalendarUpdateMeetingResponseRequest {}
impl ::core::fmt::Debug for AppointmentCalendarUpdateMeetingResponseRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarUpdateMeetingResponseRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentCalendarUpdateMeetingResponseRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarUpdateMeetingResponseRequest;{a36d608c-c29d-4b94-b086-7e9ff7bd84a0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentCalendarUpdateMeetingResponseRequest {
    type Vtable = IAppointmentCalendarUpdateMeetingResponseRequest_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentCalendarUpdateMeetingResponseRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentCalendarUpdateMeetingResponseRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarUpdateMeetingResponseRequest";
}
impl ::core::convert::From<AppointmentCalendarUpdateMeetingResponseRequest> for ::windows::core::IUnknown {
    fn from(value: AppointmentCalendarUpdateMeetingResponseRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarUpdateMeetingResponseRequest> for ::windows::core::IUnknown {
    fn from(value: &AppointmentCalendarUpdateMeetingResponseRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentCalendarUpdateMeetingResponseRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentCalendarUpdateMeetingResponseRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentCalendarUpdateMeetingResponseRequest> for ::windows::core::IInspectable {
    fn from(value: AppointmentCalendarUpdateMeetingResponseRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarUpdateMeetingResponseRequest> for ::windows::core::IInspectable {
    fn from(value: &AppointmentCalendarUpdateMeetingResponseRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentCalendarUpdateMeetingResponseRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentCalendarUpdateMeetingResponseRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentCalendarUpdateMeetingResponseRequest {}
unsafe impl ::core::marker::Sync for AppointmentCalendarUpdateMeetingResponseRequest {}
#[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
#[repr(transparent)]
pub struct AppointmentCalendarUpdateMeetingResponseRequestEventArgs(::windows::core::IUnknown);
impl AppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn Request(&self) -> ::windows::core::Result<AppointmentCalendarUpdateMeetingResponseRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Request)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentCalendarUpdateMeetingResponseRequest>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentCalendarUpdateMeetingResponseRequestEventArgs {}
impl ::core::fmt::Debug for AppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarUpdateMeetingResponseRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarUpdateMeetingResponseRequestEventArgs;{88759883-97bf-479d-aed5-0be8ce567d1e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    type Vtable = IAppointmentCalendarUpdateMeetingResponseRequestEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentCalendarUpdateMeetingResponseRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarUpdateMeetingResponseRequestEventArgs";
}
impl ::core::convert::From<AppointmentCalendarUpdateMeetingResponseRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppointmentCalendarUpdateMeetingResponseRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarUpdateMeetingResponseRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppointmentCalendarUpdateMeetingResponseRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentCalendarUpdateMeetingResponseRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppointmentCalendarUpdateMeetingResponseRequestEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentCalendarUpdateMeetingResponseRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppointmentCalendarUpdateMeetingResponseRequestEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentCalendarUpdateMeetingResponseRequestEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentCalendarUpdateMeetingResponseRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
#[repr(transparent)]
pub struct AppointmentDataProviderConnection(::windows::core::IUnknown);
impl AppointmentDataProviderConnection {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SyncRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarSyncManagerSyncRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SyncRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSyncRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSyncRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateOrUpdateAppointmentRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateOrUpdateAppointmentRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCreateOrUpdateAppointmentRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCreateOrUpdateAppointmentRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CancelMeetingRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarCancelMeetingRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CancelMeetingRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCancelMeetingRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCancelMeetingRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ForwardMeetingRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarForwardMeetingRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ForwardMeetingRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveForwardMeetingRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveForwardMeetingRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProposeNewTimeForMeetingRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProposeNewTimeForMeetingRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProposeNewTimeForMeetingRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveProposeNewTimeForMeetingRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateMeetingResponseRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<AppointmentDataProviderConnection, AppointmentCalendarUpdateMeetingResponseRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateMeetingResponseRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdateMeetingResponseRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUpdateMeetingResponseRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for AppointmentDataProviderConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentDataProviderConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentDataProviderConnection {}
impl ::core::fmt::Debug for AppointmentDataProviderConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentDataProviderConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentDataProviderConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderConnection;{f3dd9d83-3254-465f-abdb-928046552cf4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentDataProviderConnection {
    type Vtable = IAppointmentDataProviderConnection_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentDataProviderConnection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentDataProviderConnection {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderConnection";
}
impl ::core::convert::From<AppointmentDataProviderConnection> for ::windows::core::IUnknown {
    fn from(value: AppointmentDataProviderConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentDataProviderConnection> for ::windows::core::IUnknown {
    fn from(value: &AppointmentDataProviderConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentDataProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentDataProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentDataProviderConnection> for ::windows::core::IInspectable {
    fn from(value: AppointmentDataProviderConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentDataProviderConnection> for ::windows::core::IInspectable {
    fn from(value: &AppointmentDataProviderConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentDataProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentDataProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentDataProviderConnection {}
unsafe impl ::core::marker::Sync for AppointmentDataProviderConnection {}
#[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
#[repr(transparent)]
pub struct AppointmentDataProviderTriggerDetails(::windows::core::IUnknown);
impl AppointmentDataProviderTriggerDetails {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_DataProvider\"`*"]
    pub fn Connection(&self) -> ::windows::core::Result<AppointmentDataProviderConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Connection)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppointmentDataProviderConnection>(result__)
        }
    }
}
impl ::core::clone::Clone for AppointmentDataProviderTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppointmentDataProviderTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentDataProviderTriggerDetails {}
impl ::core::fmt::Debug for AppointmentDataProviderTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentDataProviderTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppointmentDataProviderTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderTriggerDetails;{b3283c01-7e12-4e5e-b1ef-74fb68ac6f2a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppointmentDataProviderTriggerDetails {
    type Vtable = IAppointmentDataProviderTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IAppointmentDataProviderTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentDataProviderTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderTriggerDetails";
}
impl ::core::convert::From<AppointmentDataProviderTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: AppointmentDataProviderTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentDataProviderTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &AppointmentDataProviderTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppointmentDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppointmentDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppointmentDataProviderTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: AppointmentDataProviderTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppointmentDataProviderTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &AppointmentDataProviderTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppointmentDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppointmentDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppointmentDataProviderTriggerDetails {}
unsafe impl ::core::marker::Sync for AppointmentDataProviderTriggerDetails {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentCalendarCancelMeetingRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentCalendarCancelMeetingRequest {
    type Vtable = IAppointmentCalendarCancelMeetingRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49460f8d_6434_40d7_ad46_6297419314d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarCancelMeetingRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AppointmentCalendarLocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AppointmentLocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AppointmentOriginalStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppointmentOriginalStartTime: usize,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Comment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NotifyInvitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentCalendarCancelMeetingRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentCalendarCancelMeetingRequestEventArgs {
    type Vtable = IAppointmentCalendarCancelMeetingRequestEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a79be16_7f30_4e35_beef_9d2c7b6dcae1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarCancelMeetingRequestEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentCalendarCreateOrUpdateAppointmentRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentCalendarCreateOrUpdateAppointmentRequest {
    type Vtable = IAppointmentCalendarCreateOrUpdateAppointmentRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e62f2b2_ca96_48ac_9124_406b19fefa70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarCreateOrUpdateAppointmentRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AppointmentCalendarLocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Appointment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub NotifyInvitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ChangedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ChangedProperties: usize,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, createdorupdatedappointment: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs {
    type Vtable = IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf8ded28_002e_4bf7_8e9d_5e20d49aa3ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentCalendarForwardMeetingRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentCalendarForwardMeetingRequest {
    type Vtable = IAppointmentCalendarForwardMeetingRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82e5ee56_26b6_4253_8a8f_6cf5f2ff7884);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarForwardMeetingRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AppointmentCalendarLocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AppointmentLocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AppointmentOriginalStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppointmentOriginalStartTime: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Invitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Invitees: usize,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ForwardHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Comment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentCalendarForwardMeetingRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentCalendarForwardMeetingRequestEventArgs {
    type Vtable = IAppointmentCalendarForwardMeetingRequestEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3109151a_23a2_42fd_9c82_c9a60d59f8a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarForwardMeetingRequestEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentCalendarProposeNewTimeForMeetingRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentCalendarProposeNewTimeForMeetingRequest {
    type Vtable = IAppointmentCalendarProposeNewTimeForMeetingRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce1c63f5_edf6_43c3_82b7_be6b368c6900);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarProposeNewTimeForMeetingRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AppointmentCalendarLocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AppointmentLocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AppointmentOriginalStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppointmentOriginalStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub NewStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NewStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub NewDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NewDuration: usize,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Comment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs {
    type Vtable = IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2d777d8_fed1_4280_a3ba_2e1f47609aa2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentCalendarSyncManagerSyncRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentCalendarSyncManagerSyncRequest {
    type Vtable = IAppointmentCalendarSyncManagerSyncRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12ab382b_7163_4a56_9a4e_7223a84adf46);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarSyncManagerSyncRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AppointmentCalendarLocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentCalendarSyncManagerSyncRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentCalendarSyncManagerSyncRequestEventArgs {
    type Vtable = IAppointmentCalendarSyncManagerSyncRequestEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca17c6f7_0284_4edd_87ba_4d8f69dcf5c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarSyncManagerSyncRequestEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentCalendarUpdateMeetingResponseRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentCalendarUpdateMeetingResponseRequest {
    type Vtable = IAppointmentCalendarUpdateMeetingResponseRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa36d608c_c29d_4b94_b086_7e9ff7bd84a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarUpdateMeetingResponseRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AppointmentCalendarLocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AppointmentLocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AppointmentOriginalStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppointmentOriginalStartTime: usize,
    pub Response: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::AppointmentParticipantResponse) -> ::windows::core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Comment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SendUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentCalendarUpdateMeetingResponseRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentCalendarUpdateMeetingResponseRequestEventArgs {
    type Vtable = IAppointmentCalendarUpdateMeetingResponseRequestEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88759883_97bf_479d_aed5_0be8ce567d1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarUpdateMeetingResponseRequestEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentDataProviderConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentDataProviderConnection {
    type Vtable = IAppointmentDataProviderConnection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3dd9d83_3254_465f_abdb_928046552cf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentDataProviderConnection_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SyncRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSyncRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSyncRequested: usize,
    #[cfg(feature = "Foundation")]
    pub CreateOrUpdateAppointmentRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateOrUpdateAppointmentRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCreateOrUpdateAppointmentRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCreateOrUpdateAppointmentRequested: usize,
    #[cfg(feature = "Foundation")]
    pub CancelMeetingRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CancelMeetingRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCancelMeetingRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCancelMeetingRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ForwardMeetingRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ForwardMeetingRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveForwardMeetingRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveForwardMeetingRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ProposeNewTimeForMeetingRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProposeNewTimeForMeetingRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProposeNewTimeForMeetingRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProposeNewTimeForMeetingRequested: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateMeetingResponseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateMeetingResponseRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdateMeetingResponseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdateMeetingResponseRequested: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentDataProviderTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentDataProviderTriggerDetails {
    type Vtable = IAppointmentDataProviderTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3283c01_7e12_4e5e_b1ef_74fb68ac6f2a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentDataProviderTriggerDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
