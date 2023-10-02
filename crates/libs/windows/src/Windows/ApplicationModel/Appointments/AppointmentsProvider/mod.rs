#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAddAppointmentOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAddAppointmentOperation {
    type Vtable = IAddAppointmentOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAddAppointmentOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xec4a9af3_620d_4c69_add7_9794e918081f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddAppointmentOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AppointmentInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SourcePackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemid: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ReportCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DismissUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentsProviderLaunchActionVerbsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentsProviderLaunchActionVerbsStatics {
    type Vtable = IAppointmentsProviderLaunchActionVerbsStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentsProviderLaunchActionVerbsStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x36dbba28_9e2e_49c6_8ef7_3ab7a5dcc8b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderLaunchActionVerbsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AddAppointment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ReplaceAppointment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RemoveAppointment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ShowTimeFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentsProviderLaunchActionVerbsStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentsProviderLaunchActionVerbsStatics2 {
    type Vtable = IAppointmentsProviderLaunchActionVerbsStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentsProviderLaunchActionVerbsStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef9049a4_af21_473c_88dc_76cd89f60ca5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderLaunchActionVerbsStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ShowAppointmentDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRemoveAppointmentOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoveAppointmentOperation {
    type Vtable = IRemoveAppointmentOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRemoveAppointmentOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08b66aba_fe33_46cd_a50c_a8ffb3260537);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoveAppointmentOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AppointmentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InstanceStartDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InstanceStartDate: usize,
    pub SourcePackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DismissUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IReplaceAppointmentOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IReplaceAppointmentOperation {
    type Vtable = IReplaceAppointmentOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IReplaceAppointmentOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf4903d9b_9e61_4de2_a732_2687c07d1de8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReplaceAppointmentOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AppointmentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AppointmentInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InstanceStartDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InstanceStartDate: usize,
    pub SourcePackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemid: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ReportCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DismissUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AddAppointmentOperation(::windows_core::IUnknown);
impl AddAppointmentOperation {
    pub fn AppointmentInformation(&self) -> ::windows_core::Result<super::Appointment> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppointmentInformation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SourcePackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourcePackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportCompleted(&self, itemid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCompleted)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(itemid)).ok() }
    }
    pub fn ReportCanceled(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCanceled)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportError(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportError)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DismissUI(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).DismissUI)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::windows_core::RuntimeType for AddAppointmentOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentsProvider.AddAppointmentOperation;{ec4a9af3-620d-4c69-add7-9794e918081f})");
}
unsafe impl ::windows_core::Interface for AddAppointmentOperation {
    type Vtable = IAddAppointmentOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AddAppointmentOperation {
    const IID: ::windows_core::GUID = <IAddAppointmentOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AddAppointmentOperation {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentsProvider.AddAppointmentOperation";
}
::windows_core::imp::interface_hierarchy!(AddAppointmentOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AddAppointmentOperation {}
unsafe impl ::core::marker::Sync for AddAppointmentOperation {}
pub struct AppointmentsProviderLaunchActionVerbs;
impl AppointmentsProviderLaunchActionVerbs {
    pub fn AddAppointment() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentsProviderLaunchActionVerbsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddAppointment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ReplaceAppointment() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentsProviderLaunchActionVerbsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReplaceAppointment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn RemoveAppointment() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentsProviderLaunchActionVerbsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoveAppointment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ShowTimeFrame() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentsProviderLaunchActionVerbsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShowTimeFrame)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ShowAppointmentDetails() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentsProviderLaunchActionVerbsStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShowAppointmentDetails)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppointmentsProviderLaunchActionVerbsStatics<R, F: FnOnce(&IAppointmentsProviderLaunchActionVerbsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppointmentsProviderLaunchActionVerbs, IAppointmentsProviderLaunchActionVerbsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAppointmentsProviderLaunchActionVerbsStatics2<R, F: FnOnce(&IAppointmentsProviderLaunchActionVerbsStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppointmentsProviderLaunchActionVerbs, IAppointmentsProviderLaunchActionVerbsStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for AppointmentsProviderLaunchActionVerbs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentsProvider.AppointmentsProviderLaunchActionVerbs";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RemoveAppointmentOperation(::windows_core::IUnknown);
impl RemoveAppointmentOperation {
    pub fn AppointmentId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppointmentId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn InstanceStartDate(&self) -> ::windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstanceStartDate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SourcePackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourcePackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportCompleted(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCompleted)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportCanceled(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCanceled)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportError(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportError)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DismissUI(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).DismissUI)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::windows_core::RuntimeType for RemoveAppointmentOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentsProvider.RemoveAppointmentOperation;{08b66aba-fe33-46cd-a50c-a8ffb3260537})");
}
unsafe impl ::windows_core::Interface for RemoveAppointmentOperation {
    type Vtable = IRemoveAppointmentOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RemoveAppointmentOperation {
    const IID: ::windows_core::GUID = <IRemoveAppointmentOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RemoveAppointmentOperation {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentsProvider.RemoveAppointmentOperation";
}
::windows_core::imp::interface_hierarchy!(RemoveAppointmentOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for RemoveAppointmentOperation {}
unsafe impl ::core::marker::Sync for RemoveAppointmentOperation {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ReplaceAppointmentOperation(::windows_core::IUnknown);
impl ReplaceAppointmentOperation {
    pub fn AppointmentId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppointmentId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppointmentInformation(&self) -> ::windows_core::Result<super::Appointment> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppointmentInformation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn InstanceStartDate(&self) -> ::windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstanceStartDate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SourcePackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourcePackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportCompleted(&self, itemid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCompleted)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(itemid)).ok() }
    }
    pub fn ReportCanceled(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportCanceled)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportError(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportError)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DismissUI(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).DismissUI)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::windows_core::RuntimeType for ReplaceAppointmentOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentsProvider.ReplaceAppointmentOperation;{f4903d9b-9e61-4de2-a732-2687c07d1de8})");
}
unsafe impl ::windows_core::Interface for ReplaceAppointmentOperation {
    type Vtable = IReplaceAppointmentOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ReplaceAppointmentOperation {
    const IID: ::windows_core::GUID = <IReplaceAppointmentOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ReplaceAppointmentOperation {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentsProvider.ReplaceAppointmentOperation";
}
::windows_core::imp::interface_hierarchy!(ReplaceAppointmentOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ReplaceAppointmentOperation {}
unsafe impl ::core::marker::Sync for ReplaceAppointmentOperation {}
