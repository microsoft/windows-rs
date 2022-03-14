#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
#[repr(transparent)]
pub struct AddAppointmentOperation(::windows::core::IUnknown);
impl AddAppointmentOperation {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn AppointmentInformation(&self) -> ::windows::core::Result<super::Appointment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppointmentInformation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Appointment>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn SourcePackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourcePackageFamilyName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn ReportCompleted<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, itemid: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCompleted)(::core::mem::transmute_copy(this), itemid.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn ReportCanceled(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCanceled)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn ReportError<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportError)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn DismissUI(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DismissUI)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for AddAppointmentOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AddAppointmentOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AddAppointmentOperation {}
impl ::core::fmt::Debug for AddAppointmentOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddAppointmentOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AddAppointmentOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentsProvider.AddAppointmentOperation;{ec4a9af3-620d-4c69-add7-9794e918081f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AddAppointmentOperation {
    type Vtable = IAddAppointmentOperation_Vtbl;
    const IID: ::windows::core::GUID = <IAddAppointmentOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AddAppointmentOperation {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentsProvider.AddAppointmentOperation";
}
impl ::core::convert::From<AddAppointmentOperation> for ::windows::core::IUnknown {
    fn from(value: AddAppointmentOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AddAppointmentOperation> for ::windows::core::IUnknown {
    fn from(value: &AddAppointmentOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AddAppointmentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AddAppointmentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AddAppointmentOperation> for ::windows::core::IInspectable {
    fn from(value: AddAppointmentOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AddAppointmentOperation> for ::windows::core::IInspectable {
    fn from(value: &AddAppointmentOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AddAppointmentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AddAppointmentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AddAppointmentOperation {}
unsafe impl ::core::marker::Sync for AddAppointmentOperation {}
#[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
pub struct AppointmentsProviderLaunchActionVerbs {}
impl AppointmentsProviderLaunchActionVerbs {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn AddAppointment() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentsProviderLaunchActionVerbsStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddAppointment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn ReplaceAppointment() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentsProviderLaunchActionVerbsStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReplaceAppointment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn RemoveAppointment() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentsProviderLaunchActionVerbsStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoveAppointment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn ShowTimeFrame() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentsProviderLaunchActionVerbsStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowTimeFrame)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn ShowAppointmentDetails() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentsProviderLaunchActionVerbsStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowAppointmentDetails)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppointmentsProviderLaunchActionVerbsStatics<R, F: FnOnce(&IAppointmentsProviderLaunchActionVerbsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppointmentsProviderLaunchActionVerbs, IAppointmentsProviderLaunchActionVerbsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IAppointmentsProviderLaunchActionVerbsStatics2<R, F: FnOnce(&IAppointmentsProviderLaunchActionVerbsStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppointmentsProviderLaunchActionVerbs, IAppointmentsProviderLaunchActionVerbsStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for AppointmentsProviderLaunchActionVerbs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentsProvider.AppointmentsProviderLaunchActionVerbs";
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAddAppointmentOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAddAppointmentOperation {
    type Vtable = IAddAppointmentOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec4a9af3_620d_4c69_add7_9794e918081f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddAppointmentOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AppointmentInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SourcePackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ReportCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReportError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DismissUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentsProviderLaunchActionVerbsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentsProviderLaunchActionVerbsStatics {
    type Vtable = IAppointmentsProviderLaunchActionVerbsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36dbba28_9e2e_49c6_8ef7_3ab7a5dcc8b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderLaunchActionVerbsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AddAppointment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ReplaceAppointment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RemoveAppointment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ShowTimeFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentsProviderLaunchActionVerbsStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentsProviderLaunchActionVerbsStatics2 {
    type Vtable = IAppointmentsProviderLaunchActionVerbsStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef9049a4_af21_473c_88dc_76cd89f60ca5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderLaunchActionVerbsStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ShowAppointmentDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoveAppointmentOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRemoveAppointmentOperation {
    type Vtable = IRemoveAppointmentOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08b66aba_fe33_46cd_a50c_a8ffb3260537);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoveAppointmentOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AppointmentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InstanceStartDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InstanceStartDate: usize,
    pub SourcePackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReportCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReportError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DismissUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReplaceAppointmentOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IReplaceAppointmentOperation {
    type Vtable = IReplaceAppointmentOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4903d9b_9e61_4de2_a732_2687c07d1de8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReplaceAppointmentOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AppointmentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AppointmentInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InstanceStartDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InstanceStartDate: usize,
    pub SourcePackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ReportCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReportError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DismissUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
#[repr(transparent)]
pub struct RemoveAppointmentOperation(::windows::core::IUnknown);
impl RemoveAppointmentOperation {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn AppointmentId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppointmentId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InstanceStartDate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InstanceStartDate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn SourcePackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourcePackageFamilyName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn ReportCompleted(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCompleted)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn ReportCanceled(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCanceled)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn ReportError<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportError)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn DismissUI(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DismissUI)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for RemoveAppointmentOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoveAppointmentOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoveAppointmentOperation {}
impl ::core::fmt::Debug for RemoveAppointmentOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoveAppointmentOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoveAppointmentOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentsProvider.RemoveAppointmentOperation;{08b66aba-fe33-46cd-a50c-a8ffb3260537})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RemoveAppointmentOperation {
    type Vtable = IRemoveAppointmentOperation_Vtbl;
    const IID: ::windows::core::GUID = <IRemoveAppointmentOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoveAppointmentOperation {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentsProvider.RemoveAppointmentOperation";
}
impl ::core::convert::From<RemoveAppointmentOperation> for ::windows::core::IUnknown {
    fn from(value: RemoveAppointmentOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoveAppointmentOperation> for ::windows::core::IUnknown {
    fn from(value: &RemoveAppointmentOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RemoveAppointmentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RemoveAppointmentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoveAppointmentOperation> for ::windows::core::IInspectable {
    fn from(value: RemoveAppointmentOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoveAppointmentOperation> for ::windows::core::IInspectable {
    fn from(value: &RemoveAppointmentOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RemoveAppointmentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RemoveAppointmentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoveAppointmentOperation {}
unsafe impl ::core::marker::Sync for RemoveAppointmentOperation {}
#[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
#[repr(transparent)]
pub struct ReplaceAppointmentOperation(::windows::core::IUnknown);
impl ReplaceAppointmentOperation {
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn AppointmentId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppointmentId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn AppointmentInformation(&self) -> ::windows::core::Result<super::Appointment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AppointmentInformation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Appointment>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InstanceStartDate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InstanceStartDate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn SourcePackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourcePackageFamilyName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn ReportCompleted<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, itemid: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCompleted)(::core::mem::transmute_copy(this), itemid.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn ReportCanceled(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCanceled)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn ReportError<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportError)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    pub fn DismissUI(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DismissUI)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for ReplaceAppointmentOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ReplaceAppointmentOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ReplaceAppointmentOperation {}
impl ::core::fmt::Debug for ReplaceAppointmentOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReplaceAppointmentOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ReplaceAppointmentOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentsProvider.ReplaceAppointmentOperation;{f4903d9b-9e61-4de2-a732-2687c07d1de8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ReplaceAppointmentOperation {
    type Vtable = IReplaceAppointmentOperation_Vtbl;
    const IID: ::windows::core::GUID = <IReplaceAppointmentOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ReplaceAppointmentOperation {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentsProvider.ReplaceAppointmentOperation";
}
impl ::core::convert::From<ReplaceAppointmentOperation> for ::windows::core::IUnknown {
    fn from(value: ReplaceAppointmentOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ReplaceAppointmentOperation> for ::windows::core::IUnknown {
    fn from(value: &ReplaceAppointmentOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ReplaceAppointmentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ReplaceAppointmentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ReplaceAppointmentOperation> for ::windows::core::IInspectable {
    fn from(value: ReplaceAppointmentOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ReplaceAppointmentOperation> for ::windows::core::IInspectable {
    fn from(value: &ReplaceAppointmentOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ReplaceAppointmentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ReplaceAppointmentOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ReplaceAppointmentOperation {}
unsafe impl ::core::marker::Sync for ReplaceAppointmentOperation {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
