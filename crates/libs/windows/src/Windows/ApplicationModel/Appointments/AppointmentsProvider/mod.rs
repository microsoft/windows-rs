#[doc(hidden)]
#[repr(transparent)]
pub struct IAddAppointmentOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAddAppointmentOperation {
    type Vtable = IAddAppointmentOperation_Vtbl;
}
impl ::core::clone::Clone for IAddAppointmentOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAddAppointmentOperation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec4a9af3_620d_4c69_add7_9794e918081f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddAppointmentOperation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppointmentInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SourcePackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemid: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ReportCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReportError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DismissUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentsProviderLaunchActionVerbsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentsProviderLaunchActionVerbsStatics {
    type Vtable = IAppointmentsProviderLaunchActionVerbsStatics_Vtbl;
}
impl ::core::clone::Clone for IAppointmentsProviderLaunchActionVerbsStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppointmentsProviderLaunchActionVerbsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36dbba28_9e2e_49c6_8ef7_3ab7a5dcc8b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderLaunchActionVerbsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AddAppointment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ReplaceAppointment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RemoveAppointment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ShowTimeFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentsProviderLaunchActionVerbsStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentsProviderLaunchActionVerbsStatics2 {
    type Vtable = IAppointmentsProviderLaunchActionVerbsStatics2_Vtbl;
}
impl ::core::clone::Clone for IAppointmentsProviderLaunchActionVerbsStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppointmentsProviderLaunchActionVerbsStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef9049a4_af21_473c_88dc_76cd89f60ca5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentsProviderLaunchActionVerbsStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ShowAppointmentDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoveAppointmentOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRemoveAppointmentOperation {
    type Vtable = IRemoveAppointmentOperation_Vtbl;
}
impl ::core::clone::Clone for IRemoveAppointmentOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRemoveAppointmentOperation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08b66aba_fe33_46cd_a50c_a8ffb3260537);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoveAppointmentOperation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppointmentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InstanceStartDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InstanceStartDate: usize,
    pub SourcePackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReportCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReportError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DismissUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReplaceAppointmentOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IReplaceAppointmentOperation {
    type Vtable = IReplaceAppointmentOperation_Vtbl;
}
impl ::core::clone::Clone for IReplaceAppointmentOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IReplaceAppointmentOperation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4903d9b_9e61_4de2_a732_2687c07d1de8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReplaceAppointmentOperation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppointmentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AppointmentInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InstanceStartDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InstanceStartDate: usize,
    pub SourcePackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ReportCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemid: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ReportCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReportError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DismissUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
#[repr(transparent)]
pub struct AddAppointmentOperation(::windows::core::IUnknown);
impl AddAppointmentOperation {
    pub fn AppointmentInformation(&self) -> ::windows::core::Result<super::Appointment> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Appointment>();
            (::windows::core::Interface::vtable(this).AppointmentInformation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SourcePackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).SourcePackageFamilyName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportCompleted(&self, itemid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCompleted)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(itemid)).ok() }
    }
    pub fn ReportCanceled(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCanceled)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportError(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportError)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DismissUI(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DismissUI)(::windows::core::Interface::as_raw(this)).ok() }
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
impl ::windows::core::RuntimeType for AddAppointmentOperation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentsProvider.AddAppointmentOperation;{ec4a9af3-620d-4c69-add7-9794e918081f})");
}
impl ::core::clone::Clone for AddAppointmentOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AddAppointmentOperation {
    type Vtable = IAddAppointmentOperation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AddAppointmentOperation {
    const IID: ::windows::core::GUID = <IAddAppointmentOperation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AddAppointmentOperation {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentsProvider.AddAppointmentOperation";
}
::windows::imp::interface_hierarchy!(AddAppointmentOperation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AddAppointmentOperation {}
unsafe impl ::core::marker::Sync for AddAppointmentOperation {}
#[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
pub struct AppointmentsProviderLaunchActionVerbs;
impl AppointmentsProviderLaunchActionVerbs {
    pub fn AddAppointment() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentsProviderLaunchActionVerbsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AddAppointment)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ReplaceAppointment() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentsProviderLaunchActionVerbsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ReplaceAppointment)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn RemoveAppointment() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentsProviderLaunchActionVerbsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).RemoveAppointment)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ShowTimeFrame() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentsProviderLaunchActionVerbsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ShowTimeFrame)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ShowAppointmentDetails() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IAppointmentsProviderLaunchActionVerbsStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ShowAppointmentDetails)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppointmentsProviderLaunchActionVerbsStatics<R, F: FnOnce(&IAppointmentsProviderLaunchActionVerbsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AppointmentsProviderLaunchActionVerbs, IAppointmentsProviderLaunchActionVerbsStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAppointmentsProviderLaunchActionVerbsStatics2<R, F: FnOnce(&IAppointmentsProviderLaunchActionVerbsStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AppointmentsProviderLaunchActionVerbs, IAppointmentsProviderLaunchActionVerbsStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for AppointmentsProviderLaunchActionVerbs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentsProvider.AppointmentsProviderLaunchActionVerbs";
}
#[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
#[repr(transparent)]
pub struct RemoveAppointmentOperation(::windows::core::IUnknown);
impl RemoveAppointmentOperation {
    pub fn AppointmentId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AppointmentId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InstanceStartDate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>();
            (::windows::core::Interface::vtable(this).InstanceStartDate)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SourcePackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).SourcePackageFamilyName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportCompleted(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCompleted)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportCanceled(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCanceled)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportError(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportError)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DismissUI(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DismissUI)(::windows::core::Interface::as_raw(this)).ok() }
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
impl ::windows::core::RuntimeType for RemoveAppointmentOperation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentsProvider.RemoveAppointmentOperation;{08b66aba-fe33-46cd-a50c-a8ffb3260537})");
}
impl ::core::clone::Clone for RemoveAppointmentOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for RemoveAppointmentOperation {
    type Vtable = IRemoveAppointmentOperation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for RemoveAppointmentOperation {
    const IID: ::windows::core::GUID = <IRemoveAppointmentOperation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for RemoveAppointmentOperation {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentsProvider.RemoveAppointmentOperation";
}
::windows::imp::interface_hierarchy!(RemoveAppointmentOperation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoveAppointmentOperation {}
unsafe impl ::core::marker::Sync for RemoveAppointmentOperation {}
#[doc = "*Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
#[repr(transparent)]
pub struct ReplaceAppointmentOperation(::windows::core::IUnknown);
impl ReplaceAppointmentOperation {
    pub fn AppointmentId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AppointmentId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppointmentInformation(&self) -> ::windows::core::Result<super::Appointment> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Appointment>();
            (::windows::core::Interface::vtable(this).AppointmentInformation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InstanceStartDate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>();
            (::windows::core::Interface::vtable(this).InstanceStartDate)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SourcePackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).SourcePackageFamilyName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportCompleted(&self, itemid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCompleted)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(itemid)).ok() }
    }
    pub fn ReportCanceled(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportCanceled)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn ReportError(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportError)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DismissUI(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DismissUI)(::windows::core::Interface::as_raw(this)).ok() }
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
impl ::windows::core::RuntimeType for ReplaceAppointmentOperation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentsProvider.ReplaceAppointmentOperation;{f4903d9b-9e61-4de2-a732-2687c07d1de8})");
}
impl ::core::clone::Clone for ReplaceAppointmentOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ReplaceAppointmentOperation {
    type Vtable = IReplaceAppointmentOperation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ReplaceAppointmentOperation {
    const IID: ::windows::core::GUID = <IReplaceAppointmentOperation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ReplaceAppointmentOperation {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentsProvider.ReplaceAppointmentOperation";
}
::windows::imp::interface_hierarchy!(ReplaceAppointmentOperation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ReplaceAppointmentOperation {}
unsafe impl ::core::marker::Sync for ReplaceAppointmentOperation {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
