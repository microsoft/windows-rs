pub const DBGPROP_ATTRIB_ACCESS_FINAL: i32 = 32768;
pub const DBGPROP_ATTRIB_ACCESS_PRIVATE: i32 = 8192;
pub const DBGPROP_ATTRIB_ACCESS_PROTECTED: i32 = 16384;
pub const DBGPROP_ATTRIB_ACCESS_PUBLIC: i32 = 4096;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBGPROP_ATTRIB_FLAGS(pub u32);
pub const DBGPROP_ATTRIB_FRAME_INCATCHBLOCK: i32 = 33554432;
pub const DBGPROP_ATTRIB_FRAME_INFINALLYBLOCK: i32 = 67108864;
pub const DBGPROP_ATTRIB_FRAME_INTRYBLOCK: i32 = 16777216;
pub const DBGPROP_ATTRIB_HAS_EXTENDED_ATTRIBS: i32 = 8388608;
pub const DBGPROP_ATTRIB_NO_ATTRIB: i32 = 0;
pub const DBGPROP_ATTRIB_STORAGE_FIELD: i32 = 262144;
pub const DBGPROP_ATTRIB_STORAGE_GLOBAL: i32 = 65536;
pub const DBGPROP_ATTRIB_STORAGE_STATIC: i32 = 131072;
pub const DBGPROP_ATTRIB_STORAGE_VIRTUAL: i32 = 524288;
pub const DBGPROP_ATTRIB_TYPE_IS_CONSTANT: i32 = 1048576;
pub const DBGPROP_ATTRIB_TYPE_IS_SYNCHRONIZED: i32 = 2097152;
pub const DBGPROP_ATTRIB_TYPE_IS_VOLATILE: i32 = 4194304;
pub const DBGPROP_ATTRIB_VALUE_IS_EVENT: i32 = 512;
pub const DBGPROP_ATTRIB_VALUE_IS_EXPANDABLE: i32 = 16;
pub const DBGPROP_ATTRIB_VALUE_IS_FAKE: i32 = 32;
pub const DBGPROP_ATTRIB_VALUE_IS_INVALID: i32 = 8;
pub const DBGPROP_ATTRIB_VALUE_IS_METHOD: i32 = 256;
pub const DBGPROP_ATTRIB_VALUE_IS_RAW_STRING: i32 = 1024;
pub const DBGPROP_ATTRIB_VALUE_IS_RETURN_VALUE: i32 = 134217728;
pub const DBGPROP_ATTRIB_VALUE_PENDING_MUTATION: i32 = 268435456;
pub const DBGPROP_ATTRIB_VALUE_READONLY: i32 = 2048;
pub const DBGPROP_INFO_ALL: u32 = 63;
pub const DBGPROP_INFO_ATTRIBUTES: i32 = 8;
pub const DBGPROP_INFO_AUTOEXPAND: i32 = 134217728;
pub const DBGPROP_INFO_BEAUTIFY: i32 = 33554432;
pub const DBGPROP_INFO_CALLTOSTRING: i32 = 67108864;
pub const DBGPROP_INFO_DEBUGPROP: i32 = 16;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DBGPROP_INFO_FLAGS(pub u32);
pub const DBGPROP_INFO_FULLNAME: i32 = 32;
pub const DBGPROP_INFO_NAME: i32 = 1;
pub const DBGPROP_INFO_STANDARD: u32 = 15;
pub const DBGPROP_INFO_TYPE: i32 = 2;
pub const DBGPROP_INFO_VALUE: i32 = 4;
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DebugPropertyInfo {
    pub m_dwValidFields: u32,
    pub m_bstrName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub m_bstrType: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub m_bstrValue: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub m_bstrFullName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub m_dwAttrib: u32,
    pub m_pDebugProp: core::mem::ManuallyDrop<Option<IDebugProperty>>,
}
pub const EX_PROP_INFO_DEBUGEXTPROP: EX_PROP_INFO_FLAGS = 4096;
pub type EX_PROP_INFO_FLAGS = i32;
pub const EX_PROP_INFO_ID: EX_PROP_INFO_FLAGS = 256;
pub const EX_PROP_INFO_LOCKBYTES: EX_PROP_INFO_FLAGS = 2048;
pub const EX_PROP_INFO_NTYPE: EX_PROP_INFO_FLAGS = 512;
pub const EX_PROP_INFO_NVALUE: EX_PROP_INFO_FLAGS = 1024;
#[repr(C)]
#[cfg(all(feature = "oaidl", feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
pub struct ExtendedDebugPropertyInfo {
    pub dwValidFields: u32,
    pub pszName: windows_core::PWSTR,
    pub pszType: windows_core::PWSTR,
    pub pszValue: windows_core::PWSTR,
    pub pszFullName: windows_core::PWSTR,
    pub dwAttrib: u32,
    pub pDebugProp: core::mem::ManuallyDrop<Option<IDebugProperty>>,
    pub nDISPID: u32,
    pub nType: u32,
    pub varValue: super::oaidl::VARIANT,
    pub plbValue: core::mem::ManuallyDrop<Option<super::objidl::ILockBytes>>,
    pub pDebugExtProp: core::mem::ManuallyDrop<Option<IDebugExtendedProperty>>,
}
#[cfg(all(feature = "oaidl", feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for ExtendedDebugPropertyInfo {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "oaidl", feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for ExtendedDebugPropertyInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
windows_core::imp::define_interface!(IDebugExtendedProperty, IDebugExtendedProperty_Vtbl, 0x51973c52_cb0c_11d0_b5c9_00a0244a0e7a);
impl core::ops::Deref for IDebugExtendedProperty {
    type Target = IDebugProperty;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDebugExtendedProperty, windows_core::IUnknown, IDebugProperty);
impl IDebugExtendedProperty {
    #[cfg(all(feature = "oaidl", feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetExtendedPropertyInfo(&self, dwfieldspec: u32, nradix: u32, pextendedpropertyinfo: *mut ExtendedDebugPropertyInfo) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetExtendedPropertyInfo)(windows_core::Interface::as_raw(self), dwfieldspec, nradix, pextendedpropertyinfo) }
    }
    pub unsafe fn EnumExtendedMembers(&self, dwfieldspec: u32, nradix: u32) -> windows_core::Result<IEnumDebugExtendedPropertyInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumExtendedMembers)(windows_core::Interface::as_raw(self), dwfieldspec, nradix, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugExtendedProperty_Vtbl {
    pub base__: IDebugProperty_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetExtendedPropertyInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut ExtendedDebugPropertyInfo) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "objidl", feature = "wtypes", feature = "wtypesbase")))]
    GetExtendedPropertyInfo: usize,
    pub EnumExtendedMembers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDebugExtendedProperty_Impl: IDebugProperty_Impl {
    fn GetExtendedPropertyInfo(&self, dwfieldspec: u32, nradix: u32, pextendedpropertyinfo: *mut ExtendedDebugPropertyInfo) -> windows_core::Result<()>;
    fn EnumExtendedMembers(&self, dwfieldspec: u32, nradix: u32) -> windows_core::Result<IEnumDebugExtendedPropertyInfo>;
}
#[cfg(all(feature = "oaidl", feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
impl IDebugExtendedProperty_Vtbl {
    pub const fn new<Identity: IDebugExtendedProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetExtendedPropertyInfo<Identity: IDebugExtendedProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfieldspec: u32, nradix: u32, pextendedpropertyinfo: *mut ExtendedDebugPropertyInfo) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugExtendedProperty_Impl::GetExtendedPropertyInfo(this, core::mem::transmute_copy(&dwfieldspec), core::mem::transmute_copy(&nradix), core::mem::transmute_copy(&pextendedpropertyinfo)).into()
            }
        }
        unsafe extern "system" fn EnumExtendedMembers<Identity: IDebugExtendedProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfieldspec: u32, nradix: u32, ppeepi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugExtendedProperty_Impl::EnumExtendedMembers(this, core::mem::transmute_copy(&dwfieldspec), core::mem::transmute_copy(&nradix)) {
                    Ok(ok__) => {
                        ppeepi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDebugProperty_Vtbl::new::<Identity, OFFSET>(),
            GetExtendedPropertyInfo: GetExtendedPropertyInfo::<Identity, OFFSET>,
            EnumExtendedMembers: EnumExtendedMembers::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugExtendedProperty as windows_core::Interface>::IID || iid == &<IDebugProperty as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDebugExtendedProperty {}
windows_core::imp::define_interface!(IDebugProperty, IDebugProperty_Vtbl, 0x51973c50_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IDebugProperty, windows_core::IUnknown);
impl IDebugProperty {
    pub unsafe fn GetPropertyInfo(&self, dwfieldspec: u32, nradix: u32, ppropertyinfo: *mut DebugPropertyInfo) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyInfo)(windows_core::Interface::as_raw(self), dwfieldspec, nradix, ppropertyinfo) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetExtendedInfo(&self, cinfos: u32, rgguidextendedinfo: *const windows_core::GUID) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExtendedInfo)(windows_core::Interface::as_raw(self), cinfos, rgguidextendedinfo, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetValueAsString<P0>(&self, pszvalue: P0, nradix: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetValueAsString)(windows_core::Interface::as_raw(self), pszvalue.param().abi(), nradix) }
    }
    pub unsafe fn EnumMembers(&self, dwfieldspec: u32, nradix: u32, refiid: *const windows_core::GUID) -> windows_core::Result<IEnumDebugPropertyInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumMembers)(windows_core::Interface::as_raw(self), dwfieldspec, nradix, refiid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetParent(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugProperty_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPropertyInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut DebugPropertyInfo) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetExtendedInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetExtendedInfo: usize,
    pub SetValueAsString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub EnumMembers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDebugProperty_Impl: windows_core::IUnknownImpl {
    fn GetPropertyInfo(&self, dwfieldspec: u32, nradix: u32, ppropertyinfo: *mut DebugPropertyInfo) -> windows_core::Result<()>;
    fn GetExtendedInfo(&self, cinfos: u32, rgguidextendedinfo: *const windows_core::GUID) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetValueAsString(&self, pszvalue: &windows_core::PCWSTR, nradix: u32) -> windows_core::Result<()>;
    fn EnumMembers(&self, dwfieldspec: u32, nradix: u32, refiid: *const windows_core::GUID) -> windows_core::Result<IEnumDebugPropertyInfo>;
    fn GetParent(&self) -> windows_core::Result<IDebugProperty>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IDebugProperty_Vtbl {
    pub const fn new<Identity: IDebugProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPropertyInfo<Identity: IDebugProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfieldspec: u32, nradix: u32, ppropertyinfo: *mut DebugPropertyInfo) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugProperty_Impl::GetPropertyInfo(this, core::mem::transmute_copy(&dwfieldspec), core::mem::transmute_copy(&nradix), core::mem::transmute_copy(&ppropertyinfo)).into()
            }
        }
        unsafe extern "system" fn GetExtendedInfo<Identity: IDebugProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cinfos: u32, rgguidextendedinfo: *const windows_core::GUID, rgvar: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugProperty_Impl::GetExtendedInfo(this, core::mem::transmute_copy(&cinfos), core::mem::transmute_copy(&rgguidextendedinfo)) {
                    Ok(ok__) => {
                        rgvar.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValueAsString<Identity: IDebugProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszvalue: windows_core::PCWSTR, nradix: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDebugProperty_Impl::SetValueAsString(this, core::mem::transmute(&pszvalue), core::mem::transmute_copy(&nradix)).into()
            }
        }
        unsafe extern "system" fn EnumMembers<Identity: IDebugProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwfieldspec: u32, nradix: u32, refiid: *const windows_core::GUID, ppepi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugProperty_Impl::EnumMembers(this, core::mem::transmute_copy(&dwfieldspec), core::mem::transmute_copy(&nradix), core::mem::transmute_copy(&refiid)) {
                    Ok(ok__) => {
                        ppepi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetParent<Identity: IDebugProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdebugprop: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugProperty_Impl::GetParent(this) {
                    Ok(ok__) => {
                        ppdebugprop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyInfo: GetPropertyInfo::<Identity, OFFSET>,
            GetExtendedInfo: GetExtendedInfo::<Identity, OFFSET>,
            SetValueAsString: SetValueAsString::<Identity, OFFSET>,
            EnumMembers: EnumMembers::<Identity, OFFSET>,
            GetParent: GetParent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugProperty as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDebugProperty {}
windows_core::imp::define_interface!(IDebugPropertyEnumType_All, IDebugPropertyEnumType_All_Vtbl, 0x51973c55_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IDebugPropertyEnumType_All, windows_core::IUnknown);
impl IDebugPropertyEnumType_All {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDebugPropertyEnumType_All_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDebugPropertyEnumType_All_Impl: windows_core::IUnknownImpl {
    fn GetName(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl IDebugPropertyEnumType_All_Vtbl {
    pub const fn new<Identity: IDebugPropertyEnumType_All_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IDebugPropertyEnumType_All_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDebugPropertyEnumType_All_Impl::GetName(this) {
                    Ok(ok__) => {
                        param0.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetName: GetName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugPropertyEnumType_All as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugPropertyEnumType_All {}
windows_core::imp::define_interface!(IDebugPropertyEnumType_Arguments, IDebugPropertyEnumType_Arguments_Vtbl, 0x51973c57_cb0c_11d0_b5c9_00a0244a0e7a);
impl core::ops::Deref for IDebugPropertyEnumType_Arguments {
    type Target = IDebugPropertyEnumType_All;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDebugPropertyEnumType_Arguments, windows_core::IUnknown, IDebugPropertyEnumType_All);
#[repr(C)]
#[doc(hidden)]
pub struct IDebugPropertyEnumType_Arguments_Vtbl {
    pub base__: IDebugPropertyEnumType_All_Vtbl,
}
pub trait IDebugPropertyEnumType_Arguments_Impl: IDebugPropertyEnumType_All_Impl {}
impl IDebugPropertyEnumType_Arguments_Vtbl {
    pub const fn new<Identity: IDebugPropertyEnumType_Arguments_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IDebugPropertyEnumType_All_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugPropertyEnumType_Arguments as windows_core::Interface>::IID || iid == &<IDebugPropertyEnumType_All as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugPropertyEnumType_Arguments {}
windows_core::imp::define_interface!(IDebugPropertyEnumType_Locals, IDebugPropertyEnumType_Locals_Vtbl, 0x51973c56_cb0c_11d0_b5c9_00a0244a0e7a);
impl core::ops::Deref for IDebugPropertyEnumType_Locals {
    type Target = IDebugPropertyEnumType_All;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDebugPropertyEnumType_Locals, windows_core::IUnknown, IDebugPropertyEnumType_All);
#[repr(C)]
#[doc(hidden)]
pub struct IDebugPropertyEnumType_Locals_Vtbl {
    pub base__: IDebugPropertyEnumType_All_Vtbl,
}
pub trait IDebugPropertyEnumType_Locals_Impl: IDebugPropertyEnumType_All_Impl {}
impl IDebugPropertyEnumType_Locals_Vtbl {
    pub const fn new<Identity: IDebugPropertyEnumType_Locals_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IDebugPropertyEnumType_All_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugPropertyEnumType_Locals as windows_core::Interface>::IID || iid == &<IDebugPropertyEnumType_All as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugPropertyEnumType_Locals {}
windows_core::imp::define_interface!(IDebugPropertyEnumType_LocalsPlusArgs, IDebugPropertyEnumType_LocalsPlusArgs_Vtbl, 0x51973c58_cb0c_11d0_b5c9_00a0244a0e7a);
impl core::ops::Deref for IDebugPropertyEnumType_LocalsPlusArgs {
    type Target = IDebugPropertyEnumType_All;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDebugPropertyEnumType_LocalsPlusArgs, windows_core::IUnknown, IDebugPropertyEnumType_All);
#[repr(C)]
#[doc(hidden)]
pub struct IDebugPropertyEnumType_LocalsPlusArgs_Vtbl {
    pub base__: IDebugPropertyEnumType_All_Vtbl,
}
pub trait IDebugPropertyEnumType_LocalsPlusArgs_Impl: IDebugPropertyEnumType_All_Impl {}
impl IDebugPropertyEnumType_LocalsPlusArgs_Vtbl {
    pub const fn new<Identity: IDebugPropertyEnumType_LocalsPlusArgs_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IDebugPropertyEnumType_All_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugPropertyEnumType_LocalsPlusArgs as windows_core::Interface>::IID || iid == &<IDebugPropertyEnumType_All as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugPropertyEnumType_LocalsPlusArgs {}
windows_core::imp::define_interface!(IDebugPropertyEnumType_Registers, IDebugPropertyEnumType_Registers_Vtbl, 0x51973c59_cb0c_11d0_b5c9_00a0244a0e7a);
impl core::ops::Deref for IDebugPropertyEnumType_Registers {
    type Target = IDebugPropertyEnumType_All;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDebugPropertyEnumType_Registers, windows_core::IUnknown, IDebugPropertyEnumType_All);
#[repr(C)]
#[doc(hidden)]
pub struct IDebugPropertyEnumType_Registers_Vtbl {
    pub base__: IDebugPropertyEnumType_All_Vtbl,
}
pub trait IDebugPropertyEnumType_Registers_Impl: IDebugPropertyEnumType_All_Impl {}
impl IDebugPropertyEnumType_Registers_Vtbl {
    pub const fn new<Identity: IDebugPropertyEnumType_Registers_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IDebugPropertyEnumType_All_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDebugPropertyEnumType_Registers as windows_core::Interface>::IID || iid == &<IDebugPropertyEnumType_All as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDebugPropertyEnumType_Registers {}
windows_core::imp::define_interface!(IEnumDebugExtendedPropertyInfo, IEnumDebugExtendedPropertyInfo_Vtbl, 0x51973c53_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IEnumDebugExtendedPropertyInfo, windows_core::IUnknown);
impl IEnumDebugExtendedPropertyInfo {
    #[cfg(all(feature = "oaidl", feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Next(&self, celt: u32, rgextendedpropertyinfo: *mut ExtendedDebugPropertyInfo, pceltfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgextendedpropertyinfo, pceltfetched as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDebugExtendedPropertyInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut ExtendedDebugPropertyInfo, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "objidl", feature = "wtypes", feature = "wtypesbase")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IEnumDebugExtendedPropertyInfo_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgextendedpropertyinfo: *mut ExtendedDebugPropertyInfo, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumDebugExtendedPropertyInfo>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "oaidl", feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
impl IEnumDebugExtendedPropertyInfo_Vtbl {
    pub const fn new<Identity: IEnumDebugExtendedPropertyInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumDebugExtendedPropertyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgextendedpropertyinfo: *mut ExtendedDebugPropertyInfo, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDebugExtendedPropertyInfo_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgextendedpropertyinfo), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumDebugExtendedPropertyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDebugExtendedPropertyInfo_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumDebugExtendedPropertyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDebugExtendedPropertyInfo_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumDebugExtendedPropertyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pedpe: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumDebugExtendedPropertyInfo_Impl::Clone(this) {
                    Ok(ok__) => {
                        pedpe.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumDebugExtendedPropertyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumDebugExtendedPropertyInfo_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcelt.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumDebugExtendedPropertyInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "objidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IEnumDebugExtendedPropertyInfo {}
windows_core::imp::define_interface!(IEnumDebugPropertyInfo, IEnumDebugPropertyInfo_Vtbl, 0x51973c51_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IEnumDebugPropertyInfo, windows_core::IUnknown);
impl IEnumDebugPropertyInfo {
    pub unsafe fn Next(&self, celt: u32, pi: *mut DebugPropertyInfo, pceltsfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, pi, pceltsfetched as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDebugPropertyInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DebugPropertyInfo, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IEnumDebugPropertyInfo_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, pi: *mut DebugPropertyInfo, pceltsfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumDebugPropertyInfo>;
    fn GetCount(&self) -> windows_core::Result<u32>;
}
impl IEnumDebugPropertyInfo_Vtbl {
    pub const fn new<Identity: IEnumDebugPropertyInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumDebugPropertyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, pi: *mut DebugPropertyInfo, pceltsfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDebugPropertyInfo_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&pi), core::mem::transmute_copy(&pceltsfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumDebugPropertyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDebugPropertyInfo_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumDebugPropertyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumDebugPropertyInfo_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumDebugPropertyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppepi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumDebugPropertyInfo_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppepi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: IEnumDebugPropertyInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelt: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumDebugPropertyInfo_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcelt.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumDebugPropertyInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumDebugPropertyInfo {}
windows_core::imp::define_interface!(IPerPropertyBrowsing2, IPerPropertyBrowsing2_Vtbl, 0x51973c54_cb0c_11d0_b5c9_00a0244a0e7a);
windows_core::imp::interface_hierarchy!(IPerPropertyBrowsing2, windows_core::IUnknown);
impl IPerPropertyBrowsing2 {
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetDisplayString(&self, dispid: super::oaidl::DISPID) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayString)(windows_core::Interface::as_raw(self), dispid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn MapPropertyToPage(&self, dispid: super::oaidl::DISPID) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MapPropertyToPage)(windows_core::Interface::as_raw(self), dispid, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oaidl", feature = "ocidl"))]
    pub unsafe fn GetPredefinedStrings(&self, dispid: super::oaidl::DISPID, pcastrings: *mut super::ocidl::CALPOLESTR, pcacookies: *mut super::ocidl::CADWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPredefinedStrings)(windows_core::Interface::as_raw(self), dispid, pcastrings as _, pcacookies as _) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn SetPredefinedValue(&self, dispid: super::oaidl::DISPID, dwcookie: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPredefinedValue)(windows_core::Interface::as_raw(self), dispid, dwcookie) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerPropertyBrowsing2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "oaidl")]
    pub GetDisplayString: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::DISPID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetDisplayString: usize,
    #[cfg(feature = "oaidl")]
    pub MapPropertyToPage: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::DISPID, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    MapPropertyToPage: usize,
    #[cfg(all(feature = "oaidl", feature = "ocidl"))]
    pub GetPredefinedStrings: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::DISPID, *mut super::ocidl::CALPOLESTR, *mut super::ocidl::CADWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "ocidl")))]
    GetPredefinedStrings: usize,
    #[cfg(feature = "oaidl")]
    pub SetPredefinedValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::DISPID, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    SetPredefinedValue: usize,
}
#[cfg(all(feature = "oaidl", feature = "ocidl"))]
pub trait IPerPropertyBrowsing2_Impl: windows_core::IUnknownImpl {
    fn GetDisplayString(&self, dispid: super::oaidl::DISPID) -> windows_core::Result<windows_core::BSTR>;
    fn MapPropertyToPage(&self, dispid: super::oaidl::DISPID) -> windows_core::Result<windows_core::GUID>;
    fn GetPredefinedStrings(&self, dispid: super::oaidl::DISPID, pcastrings: *mut super::ocidl::CALPOLESTR, pcacookies: *mut super::ocidl::CADWORD) -> windows_core::Result<()>;
    fn SetPredefinedValue(&self, dispid: super::oaidl::DISPID, dwcookie: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "ocidl"))]
impl IPerPropertyBrowsing2_Vtbl {
    pub const fn new<Identity: IPerPropertyBrowsing2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDisplayString<Identity: IPerPropertyBrowsing2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: super::oaidl::DISPID, pbstr: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerPropertyBrowsing2_Impl::GetDisplayString(this, core::mem::transmute_copy(&dispid)) {
                    Ok(ok__) => {
                        pbstr.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MapPropertyToPage<Identity: IPerPropertyBrowsing2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: super::oaidl::DISPID, pclsidproppage: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerPropertyBrowsing2_Impl::MapPropertyToPage(this, core::mem::transmute_copy(&dispid)) {
                    Ok(ok__) => {
                        pclsidproppage.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPredefinedStrings<Identity: IPerPropertyBrowsing2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: super::oaidl::DISPID, pcastrings: *mut super::ocidl::CALPOLESTR, pcacookies: *mut super::ocidl::CADWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerPropertyBrowsing2_Impl::GetPredefinedStrings(this, core::mem::transmute_copy(&dispid), core::mem::transmute_copy(&pcastrings), core::mem::transmute_copy(&pcacookies)).into()
            }
        }
        unsafe extern "system" fn SetPredefinedValue<Identity: IPerPropertyBrowsing2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispid: super::oaidl::DISPID, dwcookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerPropertyBrowsing2_Impl::SetPredefinedValue(this, core::mem::transmute_copy(&dispid), core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDisplayString: GetDisplayString::<Identity, OFFSET>,
            MapPropertyToPage: MapPropertyToPage::<Identity, OFFSET>,
            GetPredefinedStrings: GetPredefinedStrings::<Identity, OFFSET>,
            SetPredefinedValue: SetPredefinedValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPerPropertyBrowsing2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "ocidl"))]
impl windows_core::RuntimeName for IPerPropertyBrowsing2 {}
pub const OBJECT_ATTRIB_ACCESS_FINAL: OBJECT_ATTRIB_FLAGS = 32768;
pub const OBJECT_ATTRIB_ACCESS_PRIVATE: OBJECT_ATTRIB_FLAGS = 8192;
pub const OBJECT_ATTRIB_ACCESS_PROTECTED: OBJECT_ATTRIB_FLAGS = 16384;
pub const OBJECT_ATTRIB_ACCESS_PUBLIC: OBJECT_ATTRIB_FLAGS = 4096;
pub type OBJECT_ATTRIB_FLAGS = i32;
pub const OBJECT_ATTRIB_HAS_EXTENDED_ATTRIBS: OBJECT_ATTRIB_FLAGS = 8388608;
pub const OBJECT_ATTRIB_IS_CLASS: OBJECT_ATTRIB_FLAGS = 16777216;
pub const OBJECT_ATTRIB_IS_FUNCTION: OBJECT_ATTRIB_FLAGS = 33554432;
pub const OBJECT_ATTRIB_IS_INHERITED: OBJECT_ATTRIB_FLAGS = 1073741824;
pub const OBJECT_ATTRIB_IS_INTERFACE: OBJECT_ATTRIB_FLAGS = -2147483648;
pub const OBJECT_ATTRIB_IS_MACRO: OBJECT_ATTRIB_FLAGS = 268435456;
pub const OBJECT_ATTRIB_IS_PROPERTY: OBJECT_ATTRIB_FLAGS = 134217728;
pub const OBJECT_ATTRIB_IS_TYPE: OBJECT_ATTRIB_FLAGS = 536870912;
pub const OBJECT_ATTRIB_IS_VARIABLE: OBJECT_ATTRIB_FLAGS = 67108864;
pub const OBJECT_ATTRIB_NO_ATTRIB: OBJECT_ATTRIB_FLAGS = 0;
pub const OBJECT_ATTRIB_NO_NAME: OBJECT_ATTRIB_FLAGS = 1;
pub const OBJECT_ATTRIB_NO_TYPE: OBJECT_ATTRIB_FLAGS = 2;
pub const OBJECT_ATTRIB_NO_VALUE: OBJECT_ATTRIB_FLAGS = 4;
pub const OBJECT_ATTRIB_OBJECT_IS_EXPANDABLE: OBJECT_ATTRIB_FLAGS = 112;
pub const OBJECT_ATTRIB_SLOT_IS_CATEGORY: OBJECT_ATTRIB_FLAGS = 1024;
pub const OBJECT_ATTRIB_STORAGE_FIELD: OBJECT_ATTRIB_FLAGS = 262144;
pub const OBJECT_ATTRIB_STORAGE_GLOBAL: OBJECT_ATTRIB_FLAGS = 65536;
pub const OBJECT_ATTRIB_STORAGE_STATIC: OBJECT_ATTRIB_FLAGS = 131072;
pub const OBJECT_ATTRIB_STORAGE_VIRTUAL: OBJECT_ATTRIB_FLAGS = 524288;
pub const OBJECT_ATTRIB_TYPE_HAS_CODE: OBJECT_ATTRIB_FLAGS = 512;
pub const OBJECT_ATTRIB_TYPE_IS_CONSTANT: OBJECT_ATTRIB_FLAGS = 1048576;
pub const OBJECT_ATTRIB_TYPE_IS_EXPANDABLE: OBJECT_ATTRIB_FLAGS = 256;
pub const OBJECT_ATTRIB_TYPE_IS_OBJECT: OBJECT_ATTRIB_FLAGS = 256;
pub const OBJECT_ATTRIB_TYPE_IS_SYNCHRONIZED: OBJECT_ATTRIB_FLAGS = 2097152;
pub const OBJECT_ATTRIB_TYPE_IS_VOLATILE: OBJECT_ATTRIB_FLAGS = 4194304;
pub const OBJECT_ATTRIB_VALUE_HAS_CODE: OBJECT_ATTRIB_FLAGS = 128;
pub const OBJECT_ATTRIB_VALUE_IS_CUSTOM: OBJECT_ATTRIB_FLAGS = 64;
pub const OBJECT_ATTRIB_VALUE_IS_ENUM: OBJECT_ATTRIB_FLAGS = 32;
pub const OBJECT_ATTRIB_VALUE_IS_INVALID: OBJECT_ATTRIB_FLAGS = 8;
pub const OBJECT_ATTRIB_VALUE_IS_OBJECT: OBJECT_ATTRIB_FLAGS = 16;
pub const OBJECT_ATTRIB_VALUE_READONLY: OBJECT_ATTRIB_FLAGS = 2048;
pub const PROP_INFO_ALL: u32 = 63;
pub const PROP_INFO_ATTRIBUTES: PROP_INFO_FLAGS = 8;
pub const PROP_INFO_AUTOEXPAND: PROP_INFO_FLAGS = 134217728;
pub const PROP_INFO_DEBUGPROP: PROP_INFO_FLAGS = 16;
pub type PROP_INFO_FLAGS = i32;
pub const PROP_INFO_FULLNAME: PROP_INFO_FLAGS = 32;
pub const PROP_INFO_NAME: PROP_INFO_FLAGS = 1;
pub const PROP_INFO_STANDARD: u32 = 15;
pub const PROP_INFO_TYPE: PROP_INFO_FLAGS = 2;
pub const PROP_INFO_VALUE: PROP_INFO_FLAGS = 4;
