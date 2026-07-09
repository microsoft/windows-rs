pub const CPAO_EMPTY_CONNECTED: CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS = 2;
pub const CPAO_EMPTY_LOCAL: CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS = 1;
pub const CPAO_NONE: CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS = 0;
pub const CPCFO_ENABLE_PASSWORD_REVEAL: CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS = 1;
pub const CPCFO_ENABLE_TOUCH_KEYBOARD_AUTO_INVOKE: CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS = 4;
pub const CPCFO_IS_EMAIL_ADDRESS: CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS = 2;
pub const CPCFO_NONE: CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS = 0;
pub const CPCFO_NUMBERS_ONLY: CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS = 8;
pub const CPCFO_SHOW_ENGLISH_KEYBOARD: CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS = 16;
pub const CPFIS_DISABLED: CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE = 2;
pub const CPFIS_FOCUSED: CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE = 3;
pub const CPFIS_NONE: CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE = 0;
pub const CPFIS_READONLY: CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE = 1;
pub const CPFS_DISPLAY_IN_BOTH: CREDENTIAL_PROVIDER_FIELD_STATE = 3;
pub const CPFS_DISPLAY_IN_DESELECTED_TILE: CREDENTIAL_PROVIDER_FIELD_STATE = 2;
pub const CPFS_DISPLAY_IN_SELECTED_TILE: CREDENTIAL_PROVIDER_FIELD_STATE = 1;
pub const CPFS_HIDDEN: CREDENTIAL_PROVIDER_FIELD_STATE = 0;
pub const CPFT_CHECKBOX: CREDENTIAL_PROVIDER_FIELD_TYPE = 7;
pub const CPFT_COMBOBOX: CREDENTIAL_PROVIDER_FIELD_TYPE = 8;
pub const CPFT_COMMAND_LINK: CREDENTIAL_PROVIDER_FIELD_TYPE = 3;
pub const CPFT_EDIT_TEXT: CREDENTIAL_PROVIDER_FIELD_TYPE = 4;
pub const CPFT_INVALID: CREDENTIAL_PROVIDER_FIELD_TYPE = 0;
pub const CPFT_LARGE_TEXT: CREDENTIAL_PROVIDER_FIELD_TYPE = 1;
pub const CPFT_PASSWORD_TEXT: CREDENTIAL_PROVIDER_FIELD_TYPE = 5;
pub const CPFT_SMALL_TEXT: CREDENTIAL_PROVIDER_FIELD_TYPE = 2;
pub const CPFT_SUBMIT_BUTTON: CREDENTIAL_PROVIDER_FIELD_TYPE = 9;
pub const CPFT_TILE_IMAGE: CREDENTIAL_PROVIDER_FIELD_TYPE = 6;
pub const CPGSR_NO_CREDENTIAL_FINISHED: CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE = 1;
pub const CPGSR_NO_CREDENTIAL_NOT_FINISHED: CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE = 0;
pub const CPGSR_RETURN_CREDENTIAL_FINISHED: CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE = 2;
pub const CPGSR_RETURN_NO_CREDENTIAL_FINISHED: CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE = 3;
pub const CPSI_ERROR: CREDENTIAL_PROVIDER_STATUS_ICON = 1;
pub const CPSI_NONE: CREDENTIAL_PROVIDER_STATUS_ICON = 0;
pub const CPSI_SUCCESS: CREDENTIAL_PROVIDER_STATUS_ICON = 3;
pub const CPSI_WARNING: CREDENTIAL_PROVIDER_STATUS_ICON = 2;
pub const CPUS_CHANGE_PASSWORD: CREDENTIAL_PROVIDER_USAGE_SCENARIO = 3;
pub const CPUS_CREDUI: CREDENTIAL_PROVIDER_USAGE_SCENARIO = 4;
pub const CPUS_INVALID: CREDENTIAL_PROVIDER_USAGE_SCENARIO = 0;
pub const CPUS_LOGON: CREDENTIAL_PROVIDER_USAGE_SCENARIO = 1;
pub const CPUS_PLAP: CREDENTIAL_PROVIDER_USAGE_SCENARIO = 5;
pub const CPUS_UNLOCK_WORKSTATION: CREDENTIAL_PROVIDER_USAGE_SCENARIO = 2;
pub type CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS = u32;
pub type CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS = u32;
#[repr(C)]
#[cfg(feature = "Win32_rpcndr")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION {
    pub ulAuthenticationPackage: u32,
    pub clsidCredentialProvider: windows_core::GUID,
    pub cbSerialization: u32,
    pub rgbSerialization: *mut super::rpcndr::byte,
}
#[cfg(feature = "Win32_rpcndr")]
impl Default for CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR {
    pub dwFieldID: u32,
    pub cpft: CREDENTIAL_PROVIDER_FIELD_TYPE,
    pub pszLabel: windows_core::PWSTR,
    pub guidFieldType: windows_core::GUID,
}
pub type CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE = i32;
pub type CREDENTIAL_PROVIDER_FIELD_STATE = i32;
pub type CREDENTIAL_PROVIDER_FIELD_TYPE = i32;
pub type CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE = i32;
pub const CREDENTIAL_PROVIDER_NO_DEFAULT: u32 = 4294967295;
pub type CREDENTIAL_PROVIDER_STATUS_ICON = i32;
pub type CREDENTIAL_PROVIDER_USAGE_SCENARIO = i32;
pub const GenericCredentialProvider: windows_core::GUID = windows_core::GUID::from_u128(0x25cbb996_92ed_457e_b28c_4774084bd562);
windows_core::imp::define_interface!(IConnectableCredentialProviderCredential, IConnectableCredentialProviderCredential_Vtbl, 0x9387928b_ac75_4bf9_8ab2_2b93c4a55290);
impl core::ops::Deref for IConnectableCredentialProviderCredential {
    type Target = ICredentialProviderCredential;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IConnectableCredentialProviderCredential, windows_core::IUnknown, ICredentialProviderCredential);
impl IConnectableCredentialProviderCredential {
    #[cfg(feature = "Win32_shobjidl_core")]
    pub unsafe fn Connect<P0>(&self, pqcws: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IQueryContinueWithStatus>,
    {
        unsafe { (windows_core::Interface::vtable(self).Connect)(windows_core::Interface::as_raw(self), pqcws.param().abi()) }
    }
    pub unsafe fn Disconnect(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectableCredentialProviderCredential_Vtbl {
    pub base__: ICredentialProviderCredential_Vtbl,
    #[cfg(feature = "Win32_shobjidl_core")]
    pub Connect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_shobjidl_core"))]
    Connect: usize,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_rpcndr", feature = "Win32_shobjidl_core", feature = "Win32_windef"))]
pub trait IConnectableCredentialProviderCredential_Impl: ICredentialProviderCredential_Impl {
    fn Connect(&self, pqcws: windows_core::Ref<IQueryContinueWithStatus>) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_rpcndr", feature = "Win32_shobjidl_core", feature = "Win32_windef"))]
impl IConnectableCredentialProviderCredential_Vtbl {
    pub const fn new<Identity: IConnectableCredentialProviderCredential_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Connect<Identity: IConnectableCredentialProviderCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqcws: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConnectableCredentialProviderCredential_Impl::Connect(this, core::mem::transmute_copy(&pqcws)).into()
            }
        }
        unsafe extern "system" fn Disconnect<Identity: IConnectableCredentialProviderCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConnectableCredentialProviderCredential_Impl::Disconnect(this).into()
            }
        }
        Self {
            base__: ICredentialProviderCredential_Vtbl::new::<Identity, OFFSET>(),
            Connect: Connect::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConnectableCredentialProviderCredential as windows_core::Interface>::IID || iid == &<ICredentialProviderCredential as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_rpcndr", feature = "Win32_shobjidl_core", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IConnectableCredentialProviderCredential {}
windows_core::imp::define_interface!(ICredentialProvider, ICredentialProvider_Vtbl, 0xd27c3481_5a1c_45b2_8aaa_c20ebbe8229e);
windows_core::imp::interface_hierarchy!(ICredentialProvider, windows_core::IUnknown);
impl ICredentialProvider {
    pub unsafe fn SetUsageScenario(&self, cpus: CREDENTIAL_PROVIDER_USAGE_SCENARIO, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUsageScenario)(windows_core::Interface::as_raw(self), cpus, dwflags) }
    }
    #[cfg(feature = "Win32_rpcndr")]
    pub unsafe fn SetSerialization(&self, pcpcs: *const CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSerialization)(windows_core::Interface::as_raw(self), pcpcs) }
    }
    pub unsafe fn Advise<P0>(&self, pcpe: P0, upadvisecontext: usize) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ICredentialProviderEvents>,
    {
        unsafe { (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), pcpe.param().abi(), upadvisecontext) }
    }
    pub unsafe fn UnAdvise(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnAdvise)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFieldDescriptorCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFieldDescriptorCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFieldDescriptorAt(&self, dwindex: u32) -> windows_core::Result<*mut CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFieldDescriptorAt)(windows_core::Interface::as_raw(self), dwindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCredentialCount(&self, pdwcount: *mut u32, pdwdefault: *mut u32, pbautologonwithdefault: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCredentialCount)(windows_core::Interface::as_raw(self), pdwcount as _, pdwdefault as _, pbautologonwithdefault as _) }
    }
    pub unsafe fn GetCredentialAt(&self, dwindex: u32) -> windows_core::Result<ICredentialProviderCredential> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCredentialAt)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetUsageScenario: unsafe extern "system" fn(*mut core::ffi::c_void, CREDENTIAL_PROVIDER_USAGE_SCENARIO, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_rpcndr")]
    pub SetSerialization: unsafe extern "system" fn(*mut core::ffi::c_void, *const CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_rpcndr"))]
    SetSerialization: usize,
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    pub UnAdvise: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFieldDescriptorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFieldDescriptorAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR) -> windows_core::HRESULT,
    pub GetCredentialCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetCredentialAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_rpcndr")]
pub trait ICredentialProvider_Impl: windows_core::IUnknownImpl {
    fn SetUsageScenario(&self, cpus: CREDENTIAL_PROVIDER_USAGE_SCENARIO, dwflags: u32) -> windows_core::Result<()>;
    fn SetSerialization(&self, pcpcs: *const CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION) -> windows_core::Result<()>;
    fn Advise(&self, pcpe: windows_core::Ref<ICredentialProviderEvents>, upadvisecontext: usize) -> windows_core::Result<()>;
    fn UnAdvise(&self) -> windows_core::Result<()>;
    fn GetFieldDescriptorCount(&self) -> windows_core::Result<u32>;
    fn GetFieldDescriptorAt(&self, dwindex: u32) -> windows_core::Result<*mut CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR>;
    fn GetCredentialCount(&self, pdwcount: *mut u32, pdwdefault: *mut u32, pbautologonwithdefault: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetCredentialAt(&self, dwindex: u32) -> windows_core::Result<ICredentialProviderCredential>;
}
#[cfg(feature = "Win32_rpcndr")]
impl ICredentialProvider_Vtbl {
    pub const fn new<Identity: ICredentialProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetUsageScenario<Identity: ICredentialProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpus: CREDENTIAL_PROVIDER_USAGE_SCENARIO, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProvider_Impl::SetUsageScenario(this, core::mem::transmute_copy(&cpus), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn SetSerialization<Identity: ICredentialProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcpcs: *const CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProvider_Impl::SetSerialization(this, core::mem::transmute_copy(&pcpcs)).into()
            }
        }
        unsafe extern "system" fn Advise<Identity: ICredentialProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcpe: *mut core::ffi::c_void, upadvisecontext: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProvider_Impl::Advise(this, core::mem::transmute_copy(&pcpe), core::mem::transmute_copy(&upadvisecontext)).into()
            }
        }
        unsafe extern "system" fn UnAdvise<Identity: ICredentialProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProvider_Impl::UnAdvise(this).into()
            }
        }
        unsafe extern "system" fn GetFieldDescriptorCount<Identity: ICredentialProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICredentialProvider_Impl::GetFieldDescriptorCount(this) {
                    Ok(ok__) => {
                        pdwcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFieldDescriptorAt<Identity: ICredentialProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, ppcpfd: *mut *mut CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICredentialProvider_Impl::GetFieldDescriptorAt(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        ppcpfd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCredentialCount<Identity: ICredentialProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32, pdwdefault: *mut u32, pbautologonwithdefault: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProvider_Impl::GetCredentialCount(this, core::mem::transmute_copy(&pdwcount), core::mem::transmute_copy(&pdwdefault), core::mem::transmute_copy(&pbautologonwithdefault)).into()
            }
        }
        unsafe extern "system" fn GetCredentialAt<Identity: ICredentialProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, ppcpc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICredentialProvider_Impl::GetCredentialAt(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        ppcpc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetUsageScenario: SetUsageScenario::<Identity, OFFSET>,
            SetSerialization: SetSerialization::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            UnAdvise: UnAdvise::<Identity, OFFSET>,
            GetFieldDescriptorCount: GetFieldDescriptorCount::<Identity, OFFSET>,
            GetFieldDescriptorAt: GetFieldDescriptorAt::<Identity, OFFSET>,
            GetCredentialCount: GetCredentialCount::<Identity, OFFSET>,
            GetCredentialAt: GetCredentialAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICredentialProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_rpcndr")]
impl windows_core::RuntimeName for ICredentialProvider {}
windows_core::imp::define_interface!(ICredentialProviderCredential, ICredentialProviderCredential_Vtbl, 0x63913a93_40c1_481a_818d_4072ff8c70cc);
windows_core::imp::interface_hierarchy!(ICredentialProviderCredential, windows_core::IUnknown);
impl ICredentialProviderCredential {
    pub unsafe fn Advise<P0>(&self, pcpce: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ICredentialProviderCredentialEvents>,
    {
        unsafe { (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), pcpce.param().abi()) }
    }
    pub unsafe fn UnAdvise(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnAdvise)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetSelected(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetSelected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDeselected(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDeselected)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFieldState(&self, dwfieldid: u32, pcpfs: *mut CREDENTIAL_PROVIDER_FIELD_STATE, pcpfis: *mut CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFieldState)(windows_core::Interface::as_raw(self), dwfieldid, pcpfs as _, pcpfis as _) }
    }
    pub unsafe fn GetStringValue(&self, dwfieldid: u32) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStringValue)(windows_core::Interface::as_raw(self), dwfieldid, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetBitmapValue(&self, dwfieldid: u32) -> windows_core::Result<super::windef::HBITMAP> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBitmapValue)(windows_core::Interface::as_raw(self), dwfieldid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCheckboxValue(&self, dwfieldid: u32, pbchecked: *mut windows_core::BOOL, ppszlabel: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCheckboxValue)(windows_core::Interface::as_raw(self), dwfieldid, pbchecked as _, ppszlabel as _) }
    }
    pub unsafe fn GetSubmitButtonValue(&self, dwfieldid: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSubmitButtonValue)(windows_core::Interface::as_raw(self), dwfieldid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetComboBoxValueCount(&self, dwfieldid: u32, pcitems: *mut u32, pdwselecteditem: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetComboBoxValueCount)(windows_core::Interface::as_raw(self), dwfieldid, pcitems as _, pdwselecteditem as _) }
    }
    pub unsafe fn GetComboBoxValueAt(&self, dwfieldid: u32, dwitem: u32) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetComboBoxValueAt)(windows_core::Interface::as_raw(self), dwfieldid, dwitem, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStringValue<P1>(&self, dwfieldid: u32, psz: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStringValue)(windows_core::Interface::as_raw(self), dwfieldid, psz.param().abi()) }
    }
    pub unsafe fn SetCheckboxValue(&self, dwfieldid: u32, bchecked: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCheckboxValue)(windows_core::Interface::as_raw(self), dwfieldid, bchecked.into()) }
    }
    pub unsafe fn SetComboBoxSelectedValue(&self, dwfieldid: u32, dwselecteditem: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetComboBoxSelectedValue)(windows_core::Interface::as_raw(self), dwfieldid, dwselecteditem) }
    }
    pub unsafe fn CommandLinkClicked(&self, dwfieldid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CommandLinkClicked)(windows_core::Interface::as_raw(self), dwfieldid) }
    }
    #[cfg(feature = "Win32_rpcndr")]
    pub unsafe fn GetSerialization(&self, pcpgsr: *mut CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE, pcpcs: *mut CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION, ppszoptionalstatustext: *mut windows_core::PWSTR, pcpsioptionalstatusicon: *mut CREDENTIAL_PROVIDER_STATUS_ICON) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSerialization)(windows_core::Interface::as_raw(self), pcpgsr as _, pcpcs as _, ppszoptionalstatustext as _, pcpsioptionalstatusicon as _) }
    }
    #[cfg(feature = "Win32_bcrypt")]
    pub unsafe fn ReportResult(&self, ntsstatus: super::bcrypt::NTSTATUS, ntssubstatus: super::bcrypt::NTSTATUS, ppszoptionalstatustext: *mut windows_core::PWSTR, pcpsioptionalstatusicon: *mut CREDENTIAL_PROVIDER_STATUS_ICON) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReportResult)(windows_core::Interface::as_raw(self), ntsstatus, ntssubstatus, ppszoptionalstatustext as _, pcpsioptionalstatusicon as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialProviderCredential_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnAdvise: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetDeselected: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFieldState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CREDENTIAL_PROVIDER_FIELD_STATE, *mut CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE) -> windows_core::HRESULT,
    pub GetStringValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub GetBitmapValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::windef::HBITMAP) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetBitmapValue: usize,
    pub GetCheckboxValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetSubmitButtonValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub GetComboBoxValueCount: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetComboBoxValueAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetStringValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetCheckboxValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetComboBoxSelectedValue: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub CommandLinkClicked: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_rpcndr")]
    pub GetSerialization: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE, *mut CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION, *mut windows_core::PWSTR, *mut CREDENTIAL_PROVIDER_STATUS_ICON) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_rpcndr"))]
    GetSerialization: usize,
    #[cfg(feature = "Win32_bcrypt")]
    pub ReportResult: unsafe extern "system" fn(*mut core::ffi::c_void, super::bcrypt::NTSTATUS, super::bcrypt::NTSTATUS, *mut windows_core::PWSTR, *mut CREDENTIAL_PROVIDER_STATUS_ICON) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bcrypt"))]
    ReportResult: usize,
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_rpcndr", feature = "Win32_windef"))]
pub trait ICredentialProviderCredential_Impl: windows_core::IUnknownImpl {
    fn Advise(&self, pcpce: windows_core::Ref<ICredentialProviderCredentialEvents>) -> windows_core::Result<()>;
    fn UnAdvise(&self) -> windows_core::Result<()>;
    fn SetSelected(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetDeselected(&self) -> windows_core::Result<()>;
    fn GetFieldState(&self, dwfieldid: u32, pcpfs: *mut CREDENTIAL_PROVIDER_FIELD_STATE, pcpfis: *mut CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE) -> windows_core::Result<()>;
    fn GetStringValue(&self, dwfieldid: u32) -> windows_core::Result<windows_core::PWSTR>;
    fn GetBitmapValue(&self, dwfieldid: u32) -> windows_core::Result<super::windef::HBITMAP>;
    fn GetCheckboxValue(&self, dwfieldid: u32, pbchecked: *mut windows_core::BOOL, ppszlabel: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetSubmitButtonValue(&self, dwfieldid: u32) -> windows_core::Result<u32>;
    fn GetComboBoxValueCount(&self, dwfieldid: u32, pcitems: *mut u32, pdwselecteditem: *mut u32) -> windows_core::Result<()>;
    fn GetComboBoxValueAt(&self, dwfieldid: u32, dwitem: u32) -> windows_core::Result<windows_core::PWSTR>;
    fn SetStringValue(&self, dwfieldid: u32, psz: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetCheckboxValue(&self, dwfieldid: u32, bchecked: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetComboBoxSelectedValue(&self, dwfieldid: u32, dwselecteditem: u32) -> windows_core::Result<()>;
    fn CommandLinkClicked(&self, dwfieldid: u32) -> windows_core::Result<()>;
    fn GetSerialization(&self, pcpgsr: *mut CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE, pcpcs: *mut CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION, ppszoptionalstatustext: *mut windows_core::PWSTR, pcpsioptionalstatusicon: *mut CREDENTIAL_PROVIDER_STATUS_ICON) -> windows_core::Result<()>;
    fn ReportResult(&self, ntsstatus: super::bcrypt::NTSTATUS, ntssubstatus: super::bcrypt::NTSTATUS, ppszoptionalstatustext: *mut windows_core::PWSTR, pcpsioptionalstatusicon: *mut CREDENTIAL_PROVIDER_STATUS_ICON) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_rpcndr", feature = "Win32_windef"))]
impl ICredentialProviderCredential_Vtbl {
    pub const fn new<Identity: ICredentialProviderCredential_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Advise<Identity: ICredentialProviderCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcpce: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredential_Impl::Advise(this, core::mem::transmute_copy(&pcpce)).into()
            }
        }
        unsafe extern "system" fn UnAdvise<Identity: ICredentialProviderCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredential_Impl::UnAdvise(this).into()
            }
        }
        unsafe extern "system" fn SetSelected<Identity: ICredentialProviderCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbautologon: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICredentialProviderCredential_Impl::SetSelected(this) {
                    Ok(ok__) => {
                        pbautologon.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDeselected<Identity: ICredentialProviderCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredential_Impl::SetDeselected(this).into()
            }
        }
        unsafe extern "system" fn GetFieldState<Identity: ICredentialProviderCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfieldid: u32, pcpfs: *mut CREDENTIAL_PROVIDER_FIELD_STATE, pcpfis: *mut CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredential_Impl::GetFieldState(this, core::mem::transmute_copy(&dwfieldid), core::mem::transmute_copy(&pcpfs), core::mem::transmute_copy(&pcpfis)).into()
            }
        }
        unsafe extern "system" fn GetStringValue<Identity: ICredentialProviderCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfieldid: u32, ppsz: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICredentialProviderCredential_Impl::GetStringValue(this, core::mem::transmute_copy(&dwfieldid)) {
                    Ok(ok__) => {
                        ppsz.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBitmapValue<Identity: ICredentialProviderCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfieldid: u32, phbmp: *mut super::windef::HBITMAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICredentialProviderCredential_Impl::GetBitmapValue(this, core::mem::transmute_copy(&dwfieldid)) {
                    Ok(ok__) => {
                        phbmp.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCheckboxValue<Identity: ICredentialProviderCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfieldid: u32, pbchecked: *mut windows_core::BOOL, ppszlabel: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredential_Impl::GetCheckboxValue(this, core::mem::transmute_copy(&dwfieldid), core::mem::transmute_copy(&pbchecked), core::mem::transmute_copy(&ppszlabel)).into()
            }
        }
        unsafe extern "system" fn GetSubmitButtonValue<Identity: ICredentialProviderCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfieldid: u32, pdwadjacentto: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICredentialProviderCredential_Impl::GetSubmitButtonValue(this, core::mem::transmute_copy(&dwfieldid)) {
                    Ok(ok__) => {
                        pdwadjacentto.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetComboBoxValueCount<Identity: ICredentialProviderCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfieldid: u32, pcitems: *mut u32, pdwselecteditem: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredential_Impl::GetComboBoxValueCount(this, core::mem::transmute_copy(&dwfieldid), core::mem::transmute_copy(&pcitems), core::mem::transmute_copy(&pdwselecteditem)).into()
            }
        }
        unsafe extern "system" fn GetComboBoxValueAt<Identity: ICredentialProviderCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfieldid: u32, dwitem: u32, ppszitem: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICredentialProviderCredential_Impl::GetComboBoxValueAt(this, core::mem::transmute_copy(&dwfieldid), core::mem::transmute_copy(&dwitem)) {
                    Ok(ok__) => {
                        ppszitem.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStringValue<Identity: ICredentialProviderCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfieldid: u32, psz: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredential_Impl::SetStringValue(this, core::mem::transmute_copy(&dwfieldid), core::mem::transmute(&psz)).into()
            }
        }
        unsafe extern "system" fn SetCheckboxValue<Identity: ICredentialProviderCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfieldid: u32, bchecked: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredential_Impl::SetCheckboxValue(this, core::mem::transmute_copy(&dwfieldid), core::mem::transmute_copy(&bchecked)).into()
            }
        }
        unsafe extern "system" fn SetComboBoxSelectedValue<Identity: ICredentialProviderCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfieldid: u32, dwselecteditem: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredential_Impl::SetComboBoxSelectedValue(this, core::mem::transmute_copy(&dwfieldid), core::mem::transmute_copy(&dwselecteditem)).into()
            }
        }
        unsafe extern "system" fn CommandLinkClicked<Identity: ICredentialProviderCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfieldid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredential_Impl::CommandLinkClicked(this, core::mem::transmute_copy(&dwfieldid)).into()
            }
        }
        unsafe extern "system" fn GetSerialization<Identity: ICredentialProviderCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcpgsr: *mut CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE, pcpcs: *mut CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION, ppszoptionalstatustext: *mut windows_core::PWSTR, pcpsioptionalstatusicon: *mut CREDENTIAL_PROVIDER_STATUS_ICON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredential_Impl::GetSerialization(this, core::mem::transmute_copy(&pcpgsr), core::mem::transmute_copy(&pcpcs), core::mem::transmute_copy(&ppszoptionalstatustext), core::mem::transmute_copy(&pcpsioptionalstatusicon)).into()
            }
        }
        unsafe extern "system" fn ReportResult<Identity: ICredentialProviderCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ntsstatus: super::bcrypt::NTSTATUS, ntssubstatus: super::bcrypt::NTSTATUS, ppszoptionalstatustext: *mut windows_core::PWSTR, pcpsioptionalstatusicon: *mut CREDENTIAL_PROVIDER_STATUS_ICON) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredential_Impl::ReportResult(this, core::mem::transmute_copy(&ntsstatus), core::mem::transmute_copy(&ntssubstatus), core::mem::transmute_copy(&ppszoptionalstatustext), core::mem::transmute_copy(&pcpsioptionalstatusicon)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Advise: Advise::<Identity, OFFSET>,
            UnAdvise: UnAdvise::<Identity, OFFSET>,
            SetSelected: SetSelected::<Identity, OFFSET>,
            SetDeselected: SetDeselected::<Identity, OFFSET>,
            GetFieldState: GetFieldState::<Identity, OFFSET>,
            GetStringValue: GetStringValue::<Identity, OFFSET>,
            GetBitmapValue: GetBitmapValue::<Identity, OFFSET>,
            GetCheckboxValue: GetCheckboxValue::<Identity, OFFSET>,
            GetSubmitButtonValue: GetSubmitButtonValue::<Identity, OFFSET>,
            GetComboBoxValueCount: GetComboBoxValueCount::<Identity, OFFSET>,
            GetComboBoxValueAt: GetComboBoxValueAt::<Identity, OFFSET>,
            SetStringValue: SetStringValue::<Identity, OFFSET>,
            SetCheckboxValue: SetCheckboxValue::<Identity, OFFSET>,
            SetComboBoxSelectedValue: SetComboBoxSelectedValue::<Identity, OFFSET>,
            CommandLinkClicked: CommandLinkClicked::<Identity, OFFSET>,
            GetSerialization: GetSerialization::<Identity, OFFSET>,
            ReportResult: ReportResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICredentialProviderCredential as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_rpcndr", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ICredentialProviderCredential {}
windows_core::imp::define_interface!(ICredentialProviderCredential2, ICredentialProviderCredential2_Vtbl, 0xfd672c54_40ea_4d6e_9b49_cfb1a7507bd7);
impl core::ops::Deref for ICredentialProviderCredential2 {
    type Target = ICredentialProviderCredential;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICredentialProviderCredential2, windows_core::IUnknown, ICredentialProviderCredential);
impl ICredentialProviderCredential2 {
    pub unsafe fn GetUserSid(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUserSid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialProviderCredential2_Vtbl {
    pub base__: ICredentialProviderCredential_Vtbl,
    pub GetUserSid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_rpcndr", feature = "Win32_windef"))]
pub trait ICredentialProviderCredential2_Impl: ICredentialProviderCredential_Impl {
    fn GetUserSid(&self) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_rpcndr", feature = "Win32_windef"))]
impl ICredentialProviderCredential2_Vtbl {
    pub const fn new<Identity: ICredentialProviderCredential2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetUserSid<Identity: ICredentialProviderCredential2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICredentialProviderCredential2_Impl::GetUserSid(this) {
                    Ok(ok__) => {
                        sid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: ICredentialProviderCredential_Vtbl::new::<Identity, OFFSET>(), GetUserSid: GetUserSid::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICredentialProviderCredential2 as windows_core::Interface>::IID || iid == &<ICredentialProviderCredential as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_rpcndr", feature = "Win32_windef"))]
impl windows_core::RuntimeName for ICredentialProviderCredential2 {}
windows_core::imp::define_interface!(ICredentialProviderCredentialEvents, ICredentialProviderCredentialEvents_Vtbl, 0xfa6fa76b_66b7_4b11_95f1_86171118e816);
windows_core::imp::interface_hierarchy!(ICredentialProviderCredentialEvents, windows_core::IUnknown);
impl ICredentialProviderCredentialEvents {
    pub unsafe fn SetFieldState<P0>(&self, pcpc: P0, dwfieldid: u32, cpfs: CREDENTIAL_PROVIDER_FIELD_STATE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ICredentialProviderCredential>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFieldState)(windows_core::Interface::as_raw(self), pcpc.param().abi(), dwfieldid, cpfs) }
    }
    pub unsafe fn SetFieldInteractiveState<P0>(&self, pcpc: P0, dwfieldid: u32, cpfis: CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ICredentialProviderCredential>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFieldInteractiveState)(windows_core::Interface::as_raw(self), pcpc.param().abi(), dwfieldid, cpfis) }
    }
    pub unsafe fn SetFieldString<P0, P2>(&self, pcpc: P0, dwfieldid: u32, psz: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ICredentialProviderCredential>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFieldString)(windows_core::Interface::as_raw(self), pcpc.param().abi(), dwfieldid, psz.param().abi()) }
    }
    pub unsafe fn SetFieldCheckbox<P0, P3>(&self, pcpc: P0, dwfieldid: u32, bchecked: bool, pszlabel: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ICredentialProviderCredential>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFieldCheckbox)(windows_core::Interface::as_raw(self), pcpc.param().abi(), dwfieldid, bchecked.into(), pszlabel.param().abi()) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn SetFieldBitmap<P0>(&self, pcpc: P0, dwfieldid: u32, hbmp: super::windef::HBITMAP) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ICredentialProviderCredential>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFieldBitmap)(windows_core::Interface::as_raw(self), pcpc.param().abi(), dwfieldid, hbmp) }
    }
    pub unsafe fn SetFieldComboBoxSelectedItem<P0>(&self, pcpc: P0, dwfieldid: u32, dwselecteditem: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ICredentialProviderCredential>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFieldComboBoxSelectedItem)(windows_core::Interface::as_raw(self), pcpc.param().abi(), dwfieldid, dwselecteditem) }
    }
    pub unsafe fn DeleteFieldComboBoxItem<P0>(&self, pcpc: P0, dwfieldid: u32, dwitem: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ICredentialProviderCredential>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteFieldComboBoxItem)(windows_core::Interface::as_raw(self), pcpc.param().abi(), dwfieldid, dwitem) }
    }
    pub unsafe fn AppendFieldComboBoxItem<P0, P2>(&self, pcpc: P0, dwfieldid: u32, pszitem: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ICredentialProviderCredential>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AppendFieldComboBoxItem)(windows_core::Interface::as_raw(self), pcpc.param().abi(), dwfieldid, pszitem.param().abi()) }
    }
    pub unsafe fn SetFieldSubmitButton<P0>(&self, pcpc: P0, dwfieldid: u32, dwadjacentto: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ICredentialProviderCredential>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFieldSubmitButton)(windows_core::Interface::as_raw(self), pcpc.param().abi(), dwfieldid, dwadjacentto) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn OnCreatingWindow(&self) -> windows_core::Result<super::windef::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnCreatingWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialProviderCredentialEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetFieldState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, CREDENTIAL_PROVIDER_FIELD_STATE) -> windows_core::HRESULT,
    pub SetFieldInteractiveState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE) -> windows_core::HRESULT,
    pub SetFieldString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetFieldCheckbox: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::BOOL, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub SetFieldBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, super::windef::HBITMAP) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    SetFieldBitmap: usize,
    pub SetFieldComboBoxSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub DeleteFieldComboBoxItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub AppendFieldComboBoxItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetFieldSubmitButton: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub OnCreatingWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    OnCreatingWindow: usize,
}
#[cfg(feature = "Win32_windef")]
pub trait ICredentialProviderCredentialEvents_Impl: windows_core::IUnknownImpl {
    fn SetFieldState(&self, pcpc: windows_core::Ref<ICredentialProviderCredential>, dwfieldid: u32, cpfs: CREDENTIAL_PROVIDER_FIELD_STATE) -> windows_core::Result<()>;
    fn SetFieldInteractiveState(&self, pcpc: windows_core::Ref<ICredentialProviderCredential>, dwfieldid: u32, cpfis: CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE) -> windows_core::Result<()>;
    fn SetFieldString(&self, pcpc: windows_core::Ref<ICredentialProviderCredential>, dwfieldid: u32, psz: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetFieldCheckbox(&self, pcpc: windows_core::Ref<ICredentialProviderCredential>, dwfieldid: u32, bchecked: windows_core::BOOL, pszlabel: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetFieldBitmap(&self, pcpc: windows_core::Ref<ICredentialProviderCredential>, dwfieldid: u32, hbmp: super::windef::HBITMAP) -> windows_core::Result<()>;
    fn SetFieldComboBoxSelectedItem(&self, pcpc: windows_core::Ref<ICredentialProviderCredential>, dwfieldid: u32, dwselecteditem: u32) -> windows_core::Result<()>;
    fn DeleteFieldComboBoxItem(&self, pcpc: windows_core::Ref<ICredentialProviderCredential>, dwfieldid: u32, dwitem: u32) -> windows_core::Result<()>;
    fn AppendFieldComboBoxItem(&self, pcpc: windows_core::Ref<ICredentialProviderCredential>, dwfieldid: u32, pszitem: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetFieldSubmitButton(&self, pcpc: windows_core::Ref<ICredentialProviderCredential>, dwfieldid: u32, dwadjacentto: u32) -> windows_core::Result<()>;
    fn OnCreatingWindow(&self) -> windows_core::Result<super::windef::HWND>;
}
#[cfg(feature = "Win32_windef")]
impl ICredentialProviderCredentialEvents_Vtbl {
    pub const fn new<Identity: ICredentialProviderCredentialEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFieldState<Identity: ICredentialProviderCredentialEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcpc: *mut core::ffi::c_void, dwfieldid: u32, cpfs: CREDENTIAL_PROVIDER_FIELD_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredentialEvents_Impl::SetFieldState(this, core::mem::transmute_copy(&pcpc), core::mem::transmute_copy(&dwfieldid), core::mem::transmute_copy(&cpfs)).into()
            }
        }
        unsafe extern "system" fn SetFieldInteractiveState<Identity: ICredentialProviderCredentialEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcpc: *mut core::ffi::c_void, dwfieldid: u32, cpfis: CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredentialEvents_Impl::SetFieldInteractiveState(this, core::mem::transmute_copy(&pcpc), core::mem::transmute_copy(&dwfieldid), core::mem::transmute_copy(&cpfis)).into()
            }
        }
        unsafe extern "system" fn SetFieldString<Identity: ICredentialProviderCredentialEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcpc: *mut core::ffi::c_void, dwfieldid: u32, psz: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredentialEvents_Impl::SetFieldString(this, core::mem::transmute_copy(&pcpc), core::mem::transmute_copy(&dwfieldid), core::mem::transmute(&psz)).into()
            }
        }
        unsafe extern "system" fn SetFieldCheckbox<Identity: ICredentialProviderCredentialEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcpc: *mut core::ffi::c_void, dwfieldid: u32, bchecked: windows_core::BOOL, pszlabel: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredentialEvents_Impl::SetFieldCheckbox(this, core::mem::transmute_copy(&pcpc), core::mem::transmute_copy(&dwfieldid), core::mem::transmute_copy(&bchecked), core::mem::transmute(&pszlabel)).into()
            }
        }
        unsafe extern "system" fn SetFieldBitmap<Identity: ICredentialProviderCredentialEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcpc: *mut core::ffi::c_void, dwfieldid: u32, hbmp: super::windef::HBITMAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredentialEvents_Impl::SetFieldBitmap(this, core::mem::transmute_copy(&pcpc), core::mem::transmute_copy(&dwfieldid), core::mem::transmute_copy(&hbmp)).into()
            }
        }
        unsafe extern "system" fn SetFieldComboBoxSelectedItem<Identity: ICredentialProviderCredentialEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcpc: *mut core::ffi::c_void, dwfieldid: u32, dwselecteditem: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredentialEvents_Impl::SetFieldComboBoxSelectedItem(this, core::mem::transmute_copy(&pcpc), core::mem::transmute_copy(&dwfieldid), core::mem::transmute_copy(&dwselecteditem)).into()
            }
        }
        unsafe extern "system" fn DeleteFieldComboBoxItem<Identity: ICredentialProviderCredentialEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcpc: *mut core::ffi::c_void, dwfieldid: u32, dwitem: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredentialEvents_Impl::DeleteFieldComboBoxItem(this, core::mem::transmute_copy(&pcpc), core::mem::transmute_copy(&dwfieldid), core::mem::transmute_copy(&dwitem)).into()
            }
        }
        unsafe extern "system" fn AppendFieldComboBoxItem<Identity: ICredentialProviderCredentialEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcpc: *mut core::ffi::c_void, dwfieldid: u32, pszitem: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredentialEvents_Impl::AppendFieldComboBoxItem(this, core::mem::transmute_copy(&pcpc), core::mem::transmute_copy(&dwfieldid), core::mem::transmute(&pszitem)).into()
            }
        }
        unsafe extern "system" fn SetFieldSubmitButton<Identity: ICredentialProviderCredentialEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcpc: *mut core::ffi::c_void, dwfieldid: u32, dwadjacentto: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredentialEvents_Impl::SetFieldSubmitButton(this, core::mem::transmute_copy(&pcpc), core::mem::transmute_copy(&dwfieldid), core::mem::transmute_copy(&dwadjacentto)).into()
            }
        }
        unsafe extern "system" fn OnCreatingWindow<Identity: ICredentialProviderCredentialEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwndowner: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICredentialProviderCredentialEvents_Impl::OnCreatingWindow(this) {
                    Ok(ok__) => {
                        phwndowner.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFieldState: SetFieldState::<Identity, OFFSET>,
            SetFieldInteractiveState: SetFieldInteractiveState::<Identity, OFFSET>,
            SetFieldString: SetFieldString::<Identity, OFFSET>,
            SetFieldCheckbox: SetFieldCheckbox::<Identity, OFFSET>,
            SetFieldBitmap: SetFieldBitmap::<Identity, OFFSET>,
            SetFieldComboBoxSelectedItem: SetFieldComboBoxSelectedItem::<Identity, OFFSET>,
            DeleteFieldComboBoxItem: DeleteFieldComboBoxItem::<Identity, OFFSET>,
            AppendFieldComboBoxItem: AppendFieldComboBoxItem::<Identity, OFFSET>,
            SetFieldSubmitButton: SetFieldSubmitButton::<Identity, OFFSET>,
            OnCreatingWindow: OnCreatingWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICredentialProviderCredentialEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for ICredentialProviderCredentialEvents {}
windows_core::imp::define_interface!(ICredentialProviderCredentialEvents2, ICredentialProviderCredentialEvents2_Vtbl, 0xb53c00b6_9922_4b78_b1f4_ddfe774dc39b);
impl core::ops::Deref for ICredentialProviderCredentialEvents2 {
    type Target = ICredentialProviderCredentialEvents;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ICredentialProviderCredentialEvents2, windows_core::IUnknown, ICredentialProviderCredentialEvents);
impl ICredentialProviderCredentialEvents2 {
    pub unsafe fn BeginFieldUpdates(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BeginFieldUpdates)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EndFieldUpdates(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndFieldUpdates)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetFieldOptions<P0>(&self, credential: P0, fieldid: u32, options: CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ICredentialProviderCredential>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFieldOptions)(windows_core::Interface::as_raw(self), credential.param().abi(), fieldid, options) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialProviderCredentialEvents2_Vtbl {
    pub base__: ICredentialProviderCredentialEvents_Vtbl,
    pub BeginFieldUpdates: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndFieldUpdates: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFieldOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_windef")]
pub trait ICredentialProviderCredentialEvents2_Impl: ICredentialProviderCredentialEvents_Impl {
    fn BeginFieldUpdates(&self) -> windows_core::Result<()>;
    fn EndFieldUpdates(&self) -> windows_core::Result<()>;
    fn SetFieldOptions(&self, credential: windows_core::Ref<ICredentialProviderCredential>, fieldid: u32, options: CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_windef")]
impl ICredentialProviderCredentialEvents2_Vtbl {
    pub const fn new<Identity: ICredentialProviderCredentialEvents2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginFieldUpdates<Identity: ICredentialProviderCredentialEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredentialEvents2_Impl::BeginFieldUpdates(this).into()
            }
        }
        unsafe extern "system" fn EndFieldUpdates<Identity: ICredentialProviderCredentialEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredentialEvents2_Impl::EndFieldUpdates(this).into()
            }
        }
        unsafe extern "system" fn SetFieldOptions<Identity: ICredentialProviderCredentialEvents2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, credential: *mut core::ffi::c_void, fieldid: u32, options: CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderCredentialEvents2_Impl::SetFieldOptions(this, core::mem::transmute_copy(&credential), core::mem::transmute_copy(&fieldid), core::mem::transmute_copy(&options)).into()
            }
        }
        Self {
            base__: ICredentialProviderCredentialEvents_Vtbl::new::<Identity, OFFSET>(),
            BeginFieldUpdates: BeginFieldUpdates::<Identity, OFFSET>,
            EndFieldUpdates: EndFieldUpdates::<Identity, OFFSET>,
            SetFieldOptions: SetFieldOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICredentialProviderCredentialEvents2 as windows_core::Interface>::IID || iid == &<ICredentialProviderCredentialEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for ICredentialProviderCredentialEvents2 {}
windows_core::imp::define_interface!(ICredentialProviderCredentialWithFieldOptions, ICredentialProviderCredentialWithFieldOptions_Vtbl, 0xdbc6fb30_c843_49e3_a645_573e6f39446a);
windows_core::imp::interface_hierarchy!(ICredentialProviderCredentialWithFieldOptions, windows_core::IUnknown);
impl ICredentialProviderCredentialWithFieldOptions {
    pub unsafe fn GetFieldOptions(&self, fieldid: u32) -> windows_core::Result<CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFieldOptions)(windows_core::Interface::as_raw(self), fieldid, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialProviderCredentialWithFieldOptions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFieldOptions: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS) -> windows_core::HRESULT,
}
pub trait ICredentialProviderCredentialWithFieldOptions_Impl: windows_core::IUnknownImpl {
    fn GetFieldOptions(&self, fieldid: u32) -> windows_core::Result<CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS>;
}
impl ICredentialProviderCredentialWithFieldOptions_Vtbl {
    pub const fn new<Identity: ICredentialProviderCredentialWithFieldOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFieldOptions<Identity: ICredentialProviderCredentialWithFieldOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fieldid: u32, options: *mut CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICredentialProviderCredentialWithFieldOptions_Impl::GetFieldOptions(this, core::mem::transmute_copy(&fieldid)) {
                    Ok(ok__) => {
                        options.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetFieldOptions: GetFieldOptions::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICredentialProviderCredentialWithFieldOptions as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICredentialProviderCredentialWithFieldOptions {}
windows_core::imp::define_interface!(ICredentialProviderEvents, ICredentialProviderEvents_Vtbl, 0x34201e5a_a787_41a3_a5a4_bd6dcf2a854e);
windows_core::imp::interface_hierarchy!(ICredentialProviderEvents, windows_core::IUnknown);
impl ICredentialProviderEvents {
    pub unsafe fn CredentialsChanged(&self, upadvisecontext: usize) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CredentialsChanged)(windows_core::Interface::as_raw(self), upadvisecontext) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialProviderEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CredentialsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
pub trait ICredentialProviderEvents_Impl: windows_core::IUnknownImpl {
    fn CredentialsChanged(&self, upadvisecontext: usize) -> windows_core::Result<()>;
}
impl ICredentialProviderEvents_Vtbl {
    pub const fn new<Identity: ICredentialProviderEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CredentialsChanged<Identity: ICredentialProviderEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, upadvisecontext: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderEvents_Impl::CredentialsChanged(this, core::mem::transmute_copy(&upadvisecontext)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CredentialsChanged: CredentialsChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICredentialProviderEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICredentialProviderEvents {}
windows_core::imp::define_interface!(ICredentialProviderFilter, ICredentialProviderFilter_Vtbl, 0xa5da53f9_d475_4080_a120_910c4a739880);
windows_core::imp::interface_hierarchy!(ICredentialProviderFilter, windows_core::IUnknown);
impl ICredentialProviderFilter {
    pub unsafe fn Filter(&self, cpus: CREDENTIAL_PROVIDER_USAGE_SCENARIO, dwflags: u32, rgclsidproviders: *const windows_core::GUID, rgballow: *mut windows_core::BOOL, cproviders: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Filter)(windows_core::Interface::as_raw(self), cpus, dwflags, rgclsidproviders, rgballow as _, cproviders) }
    }
    #[cfg(feature = "Win32_rpcndr")]
    pub unsafe fn UpdateRemoteCredential(&self, pcpcsin: *const CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION, pcpcsout: *mut CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateRemoteCredential)(windows_core::Interface::as_raw(self), pcpcsin, pcpcsout as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialProviderFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Filter: unsafe extern "system" fn(*mut core::ffi::c_void, CREDENTIAL_PROVIDER_USAGE_SCENARIO, u32, *const windows_core::GUID, *mut windows_core::BOOL, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_rpcndr")]
    pub UpdateRemoteCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *const CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION, *mut CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_rpcndr"))]
    UpdateRemoteCredential: usize,
}
#[cfg(feature = "Win32_rpcndr")]
pub trait ICredentialProviderFilter_Impl: windows_core::IUnknownImpl {
    fn Filter(&self, cpus: CREDENTIAL_PROVIDER_USAGE_SCENARIO, dwflags: u32, rgclsidproviders: *const windows_core::GUID, rgballow: *mut windows_core::BOOL, cproviders: u32) -> windows_core::Result<()>;
    fn UpdateRemoteCredential(&self, pcpcsin: *const CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION, pcpcsout: *mut CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_rpcndr")]
impl ICredentialProviderFilter_Vtbl {
    pub const fn new<Identity: ICredentialProviderFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Filter<Identity: ICredentialProviderFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cpus: CREDENTIAL_PROVIDER_USAGE_SCENARIO, dwflags: u32, rgclsidproviders: *const windows_core::GUID, rgballow: *mut windows_core::BOOL, cproviders: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderFilter_Impl::Filter(this, core::mem::transmute_copy(&cpus), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&rgclsidproviders), core::mem::transmute_copy(&rgballow), core::mem::transmute_copy(&cproviders)).into()
            }
        }
        unsafe extern "system" fn UpdateRemoteCredential<Identity: ICredentialProviderFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcpcsin: *const CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION, pcpcsout: *mut CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderFilter_Impl::UpdateRemoteCredential(this, core::mem::transmute_copy(&pcpcsin), core::mem::transmute_copy(&pcpcsout)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Filter: Filter::<Identity, OFFSET>,
            UpdateRemoteCredential: UpdateRemoteCredential::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICredentialProviderFilter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_rpcndr")]
impl windows_core::RuntimeName for ICredentialProviderFilter {}
windows_core::imp::define_interface!(ICredentialProviderSetUserArray, ICredentialProviderSetUserArray_Vtbl, 0x095c1484_1c0c_4388_9c6d_500e61bf84bd);
windows_core::imp::interface_hierarchy!(ICredentialProviderSetUserArray, windows_core::IUnknown);
impl ICredentialProviderSetUserArray {
    pub unsafe fn SetUserArray<P0>(&self, users: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ICredentialProviderUserArray>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetUserArray)(windows_core::Interface::as_raw(self), users.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialProviderSetUserArray_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetUserArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICredentialProviderSetUserArray_Impl: windows_core::IUnknownImpl {
    fn SetUserArray(&self, users: windows_core::Ref<ICredentialProviderUserArray>) -> windows_core::Result<()>;
}
impl ICredentialProviderSetUserArray_Vtbl {
    pub const fn new<Identity: ICredentialProviderSetUserArray_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetUserArray<Identity: ICredentialProviderSetUserArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, users: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderSetUserArray_Impl::SetUserArray(this, core::mem::transmute_copy(&users)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetUserArray: SetUserArray::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICredentialProviderSetUserArray as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICredentialProviderSetUserArray {}
windows_core::imp::define_interface!(ICredentialProviderUser, ICredentialProviderUser_Vtbl, 0x13793285_3ea6_40fd_b420_15f47da41fbb);
windows_core::imp::interface_hierarchy!(ICredentialProviderUser, windows_core::IUnknown);
impl ICredentialProviderUser {
    pub unsafe fn GetSid(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProviderID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProviderID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn GetStringValue(&self, key: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStringValue)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetValue(&self, key: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::propidlbase::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialProviderUser_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetProviderID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub GetStringValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    GetStringValue: usize,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetValue: usize,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ICredentialProviderUser_Impl: windows_core::IUnknownImpl {
    fn GetSid(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetProviderID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetStringValue(&self, key: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<windows_core::PWSTR>;
    fn GetValue(&self, key: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::propidlbase::PROPVARIANT>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ICredentialProviderUser_Vtbl {
    pub const fn new<Identity: ICredentialProviderUser_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSid<Identity: ICredentialProviderUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICredentialProviderUser_Impl::GetSid(this) {
                    Ok(ok__) => {
                        sid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProviderID<Identity: ICredentialProviderUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICredentialProviderUser_Impl::GetProviderID(this) {
                    Ok(ok__) => {
                        providerid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStringValue<Identity: ICredentialProviderUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::wtypes::PROPERTYKEY, stringvalue: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICredentialProviderUser_Impl::GetStringValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        stringvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValue<Identity: ICredentialProviderUser_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::wtypes::PROPERTYKEY, value: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICredentialProviderUser_Impl::GetValue(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSid: GetSid::<Identity, OFFSET>,
            GetProviderID: GetProviderID::<Identity, OFFSET>,
            GetStringValue: GetStringValue::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICredentialProviderUser as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ICredentialProviderUser {}
windows_core::imp::define_interface!(ICredentialProviderUserArray, ICredentialProviderUserArray_Vtbl, 0x90c119ae_0f18_4520_a1f1_114366a40fe8);
windows_core::imp::interface_hierarchy!(ICredentialProviderUserArray, windows_core::IUnknown);
impl ICredentialProviderUserArray {
    pub unsafe fn SetProviderFilter(&self, guidprovidertofilterto: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProviderFilter)(windows_core::Interface::as_raw(self), guidprovidertofilterto) }
    }
    pub unsafe fn GetAccountOptions(&self) -> windows_core::Result<CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAccountOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAt(&self, userindex: u32) -> windows_core::Result<ICredentialProviderUser> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), userindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICredentialProviderUserArray_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetProviderFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetAccountOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICredentialProviderUserArray_Impl: windows_core::IUnknownImpl {
    fn SetProviderFilter(&self, guidprovidertofilterto: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetAccountOptions(&self) -> windows_core::Result<CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, userindex: u32) -> windows_core::Result<ICredentialProviderUser>;
}
impl ICredentialProviderUserArray_Vtbl {
    pub const fn new<Identity: ICredentialProviderUserArray_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetProviderFilter<Identity: ICredentialProviderUserArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidprovidertofilterto: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICredentialProviderUserArray_Impl::SetProviderFilter(this, core::mem::transmute_copy(&guidprovidertofilterto)).into()
            }
        }
        unsafe extern "system" fn GetAccountOptions<Identity: ICredentialProviderUserArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, credentialprovideraccountoptions: *mut CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICredentialProviderUserArray_Impl::GetAccountOptions(this) {
                    Ok(ok__) => {
                        credentialprovideraccountoptions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: ICredentialProviderUserArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usercount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICredentialProviderUserArray_Impl::GetCount(this) {
                    Ok(ok__) => {
                        usercount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAt<Identity: ICredentialProviderUserArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, userindex: u32, user: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICredentialProviderUserArray_Impl::GetAt(this, core::mem::transmute_copy(&userindex)) {
                    Ok(ok__) => {
                        user.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetProviderFilter: SetProviderFilter::<Identity, OFFSET>,
            GetAccountOptions: GetAccountOptions::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICredentialProviderUserArray as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICredentialProviderUserArray {}
#[cfg(feature = "Win32_shobjidl_core")]
windows_core::imp::define_interface!(IQueryContinueWithStatus, IQueryContinueWithStatus_Vtbl, 0x9090be5b_502b_41fb_bccc_0049a6c7254b);
#[cfg(feature = "Win32_shobjidl_core")]
impl core::ops::Deref for IQueryContinueWithStatus {
    type Target = super::shobjidl_core::IQueryContinue;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_shobjidl_core")]
windows_core::imp::interface_hierarchy!(IQueryContinueWithStatus, windows_core::IUnknown, super::shobjidl_core::IQueryContinue);
#[cfg(feature = "Win32_shobjidl_core")]
impl IQueryContinueWithStatus {
    pub unsafe fn SetStatusMessage<P0>(&self, psz: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStatusMessage)(windows_core::Interface::as_raw(self), psz.param().abi()) }
    }
}
#[cfg(feature = "Win32_shobjidl_core")]
#[repr(C)]
#[doc(hidden)]
pub struct IQueryContinueWithStatus_Vtbl {
    pub base__: super::shobjidl_core::IQueryContinue_Vtbl,
    pub SetStatusMessage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_shobjidl_core")]
pub trait IQueryContinueWithStatus_Impl: super::shobjidl_core::IQueryContinue_Impl {
    fn SetStatusMessage(&self, psz: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_shobjidl_core")]
impl IQueryContinueWithStatus_Vtbl {
    pub const fn new<Identity: IQueryContinueWithStatus_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetStatusMessage<Identity: IQueryContinueWithStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psz: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IQueryContinueWithStatus_Impl::SetStatusMessage(this, core::mem::transmute(&psz)).into()
            }
        }
        Self { base__: super::shobjidl_core::IQueryContinue_Vtbl::new::<Identity, OFFSET>(), SetStatusMessage: SetStatusMessage::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IQueryContinueWithStatus as windows_core::Interface>::IID || iid == &<super::shobjidl_core::IQueryContinue as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_shobjidl_core")]
impl windows_core::RuntimeName for IQueryContinueWithStatus {}
pub const Identity_LocalUserProvider: windows_core::GUID = windows_core::GUID::from_u128(0xa198529b_730f_4089_b646_a12557f5665e);
pub const NPCredentialProvider: windows_core::GUID = windows_core::GUID::from_u128(0x3dd6bec0_8193_4ffe_ae25_e08e39ea4063);
pub const OnexCredentialProvider: windows_core::GUID = windows_core::GUID::from_u128(0x07aa0886_cc8d_4e19_a410_1c75af686e62);
pub const OnexPlapSmartcardCredentialProvider: windows_core::GUID = windows_core::GUID::from_u128(0x33c86cd6_705f_4ba1_9adb_67070b837775);
pub const PINLogonCredentialProvider: windows_core::GUID = windows_core::GUID::from_u128(0xcb82ea12_9f71_446d_89e1_8d0924e1256e);
pub const PasswordCredentialProvider: windows_core::GUID = windows_core::GUID::from_u128(0x60b78e88_ead8_445c_9cfd_0b87f74ea6cd);
pub const RASProvider: windows_core::GUID = windows_core::GUID::from_u128(0x5537e283_b1e7_4ef8_9c6e_7ab0afe5056d);
pub const SmartcardCredentialProvider: windows_core::GUID = windows_core::GUID::from_u128(0x8fd7e19c_3bf7_489b_a72c_846ab3678c96);
pub const SmartcardPinProvider: windows_core::GUID = windows_core::GUID::from_u128(0x94596c7e_3744_41ce_893e_bbf09122f76a);
pub const SmartcardReaderSelectionProvider: windows_core::GUID = windows_core::GUID::from_u128(0x1b283861_754f_4022_ad47_a5eaaa618894);
pub const SmartcardWinRTProvider: windows_core::GUID = windows_core::GUID::from_u128(0x1ee7337f_85ac_45e2_a23c_37c753209769);
pub const V1PasswordCredentialProvider: windows_core::GUID = windows_core::GUID::from_u128(0x6f45dc1e_5384_457a_bc13_2cd81b0d28ed);
pub const V1SmartcardCredentialProvider: windows_core::GUID = windows_core::GUID::from_u128(0x8bf9a910_a8ff_457f_999f_a5ca10b4a885);
pub const V1WinBioCredentialProvider: windows_core::GUID = windows_core::GUID::from_u128(0xac3ac249_e820_4343_a65b_377ac634dc09);
pub const VaultProvider: windows_core::GUID = windows_core::GUID::from_u128(0x503739d0_4c5e_4cfd_b3ba_d881334f0df2);
pub const WinBioCredentialProvider: windows_core::GUID = windows_core::GUID::from_u128(0xbec09223_b018_416d_a0ac_523971b639f5);
