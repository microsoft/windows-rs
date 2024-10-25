windows_core::imp::define_interface!(IContactPartnerProvisioningManagerStatics, IContactPartnerProvisioningManagerStatics_Vtbl, 0xc0d79a21_01af_4fd3_98cd_b3d656de15f4);
impl windows_core::RuntimeType for IContactPartnerProvisioningManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContactPartnerProvisioningManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AssociateNetworkAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ImportVcardToSystemAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImportVcardToSystemAsync: usize,
}
windows_core::imp::define_interface!(IContactPartnerProvisioningManagerStatics2, IContactPartnerProvisioningManagerStatics2_Vtbl, 0xc26155f7_55ed_475d_9334_c5d484c30f1a);
impl windows_core::RuntimeType for IContactPartnerProvisioningManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IContactPartnerProvisioningManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AssociateSocialNetworkAccountAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMessagePartnerProvisioningManagerStatics, IMessagePartnerProvisioningManagerStatics_Vtbl, 0x8a1b0850_73c5_457c_bc59_ed7d615c05a4);
impl windows_core::RuntimeType for IMessagePartnerProvisioningManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMessagePartnerProvisioningManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ImportSmsToSystemAsync: unsafe extern "system" fn(*mut core::ffi::c_void, bool, bool, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, super::super::super::Foundation::DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImportSmsToSystemAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ImportMmsToSystemAsync: unsafe extern "system" fn(*mut core::ffi::c_void, bool, bool, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, super::super::super::Foundation::DateTime, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImportMmsToSystemAsync: usize,
}
pub struct ContactPartnerProvisioningManager;
impl ContactPartnerProvisioningManager {}
impl windows_core::RuntimeName for ContactPartnerProvisioningManager {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.Provisioning.ContactPartnerProvisioningManager";
}
pub struct MessagePartnerProvisioningManager;
impl MessagePartnerProvisioningManager {}
impl windows_core::RuntimeName for MessagePartnerProvisioningManager {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.Provisioning.MessagePartnerProvisioningManager";
}
