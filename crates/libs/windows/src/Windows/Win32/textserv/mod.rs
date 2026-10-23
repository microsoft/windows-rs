pub const CARET_CUSTOM: CARET_FLAGS = 1;
pub type CARET_FLAGS = i32;
pub const CARET_ITALIC: CARET_FLAGS = 32;
pub const CARET_NONE: CARET_FLAGS = 0;
pub const CARET_NULL: CARET_FLAGS = 64;
pub const CARET_ROTATE90: CARET_FLAGS = 128;
pub const CARET_RTL: CARET_FLAGS = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CHANGENOTIFY {
    pub dwChangeType: super::DWORD,
    pub pvCookieData: *mut core::ffi::c_void,
}
pub type CHANGETYPE = i32;
pub const CN_GENERIC: CHANGETYPE = 0;
pub const CN_NEWREDO: CHANGETYPE = 4;
pub const CN_NEWUNDO: CHANGETYPE = 2;
pub const CN_TEXTCHANGED: CHANGETYPE = 1;
windows_core::imp::define_interface!(IRichEditUiaInformation, IRichEditUiaInformation_Vtbl, 0x23969a9d_8546_4032_a1bb_73750cbf3333);
windows_core::imp::interface_hierarchy!(IRichEditUiaInformation, windows_core::IUnknown);
impl IRichEditUiaInformation {
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn GetBoundaryRectangle(&self, puiarect: *mut super::UiaRect) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBoundaryRectangle)(windows_core::Interface::as_raw(self), puiarect as _) }
    }
    pub unsafe fn IsVisible(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsVisible)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRichEditUiaInformation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "uiautomationcore")]
    pub GetBoundaryRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UiaRect) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    GetBoundaryRectangle: usize,
    pub IsVisible: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "uiautomationcore")]
pub trait IRichEditUiaInformation_Impl: windows_core::IUnknownImpl {
    fn GetBoundaryRectangle(&self, puiarect: *mut super::UiaRect) -> windows_core::Result<()>;
    fn IsVisible(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "uiautomationcore")]
impl IRichEditUiaInformation_Vtbl {
    pub const fn new<Identity: IRichEditUiaInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBoundaryRectangle<Identity: IRichEditUiaInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puiarect: *mut super::UiaRect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditUiaInformation_Impl::GetBoundaryRectangle(this, core::mem::transmute_copy(&puiarect)).into()
            }
        }
        unsafe extern "system" fn IsVisible<Identity: IRichEditUiaInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRichEditUiaInformation_Impl::IsVisible(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBoundaryRectangle: GetBoundaryRectangle::<Identity, OFFSET>,
            IsVisible: IsVisible::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRichEditUiaInformation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "uiautomationcore")]
impl windows_core::RuntimeName for IRichEditUiaInformation {}
windows_core::imp::define_interface!(IRicheditUiaOverrides, IRicheditUiaOverrides_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IRicheditUiaOverrides, windows_core::IUnknown);
impl IRicheditUiaOverrides {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetPropertyOverrideValue(&self, propertyid: super::PROPERTYID, pretvalue: *mut super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyOverrideValue)(windows_core::Interface::as_raw(self), propertyid, pretvalue as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRicheditUiaOverrides_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetPropertyOverrideValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::PROPERTYID, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetPropertyOverrideValue: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRicheditUiaOverrides_Impl: windows_core::IUnknownImpl {
    fn GetPropertyOverrideValue(&self, propertyid: super::PROPERTYID, pretvalue: *mut super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IRicheditUiaOverrides_Vtbl {
    pub const fn new<Identity: IRicheditUiaOverrides_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPropertyOverrideValue<Identity: IRicheditUiaOverrides_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: super::PROPERTYID, pretvalue: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRicheditUiaOverrides_Impl::GetPropertyOverrideValue(this, core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&pretvalue)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetPropertyOverrideValue: GetPropertyOverrideValue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRicheditUiaOverrides as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRicheditUiaOverrides {}
windows_core::imp::define_interface!(IRicheditWindowlessAccessibility, IRicheditWindowlessAccessibility_Vtbl, 0x983e572d_20cd_460b_9104_83111592dd10);
windows_core::imp::interface_hierarchy!(IRicheditWindowlessAccessibility, windows_core::IUnknown);
impl IRicheditWindowlessAccessibility {
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn CreateProvider(&self, psite: &Option<super::IRawElementProviderWindowlessSite>, ppprovider: *mut Option<super::IRawElementProviderSimple>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateProvider)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(psite), ppprovider as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRicheditWindowlessAccessibility_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "uiautomationcore")]
    pub CreateProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CreateProvider: usize,
}
#[cfg(feature = "uiautomationcore")]
pub trait IRicheditWindowlessAccessibility_Impl: windows_core::IUnknownImpl {
    fn CreateProvider(&self, psite: Option<super::IRawElementProviderWindowlessSite>, ppprovider: *mut Option<super::IRawElementProviderSimple>) -> windows_core::Result<()>;
}
#[cfg(feature = "uiautomationcore")]
impl IRicheditWindowlessAccessibility_Vtbl {
    pub const fn new<Identity: IRicheditWindowlessAccessibility_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateProvider<Identity: IRicheditWindowlessAccessibility_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psite: *mut core::ffi::c_void, ppprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRicheditWindowlessAccessibility_Impl::CreateProvider(this, core::mem::transmute(&psite), core::mem::transmute_copy(&ppprovider)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateProvider: CreateProvider::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRicheditWindowlessAccessibility as windows_core::Interface>::IID
    }
}
#[cfg(feature = "uiautomationcore")]
impl windows_core::RuntimeName for IRicheditWindowlessAccessibility {}
pub type PShutdownTextServices = Option<unsafe extern "system" fn(ptextservices: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::HRESULT>;
pub const S_MSG_KEY_IGNORED: windows_core::HRESULT = windows_core::HRESULT(0x40201_u32 as _);
pub const TXES_ISDIALOG: i32 = 1;
pub type TXTBACKSTYLE = i32;
pub const TXTBACK_OPAQUE: TXTBACKSTYLE = 1;
pub const TXTBACK_TRANSPARENT: TXTBACKSTYLE = 0;
pub const TXTBIT_ADVANCEDINPUT: i32 = 536870912;
pub const TXTBIT_ALLOWBEEP: i32 = 2048;
pub const TXTBIT_AUTOWORDSEL: i32 = 128;
pub const TXTBIT_BACKSTYLECHANGE: i32 = 16384;
pub const TXTBIT_CHARFORMATCHANGE: i32 = 131072;
pub const TXTBIT_CLIENTRECTCHANGE: i32 = 1048576;
pub const TXTBIT_D2DDWRITE: i32 = 16777216;
pub const TXTBIT_D2DPIXELSNAPPED: i32 = 67108864;
pub const TXTBIT_D2DSIMPLETYPOGRAPHY: i32 = 33554432;
pub const TXTBIT_D2DSUBPIXELLINES: i32 = 134217728;
pub const TXTBIT_DISABLEDRAG: i32 = 4096;
pub const TXTBIT_EXTENTCHANGE: i32 = 524288;
pub const TXTBIT_FLASHLASTPASSWORDCHAR: i32 = 268435456;
pub const TXTBIT_HIDESELECTION: i32 = 32;
pub const TXTBIT_MAXLENGTHCHANGE: i32 = 32768;
pub const TXTBIT_MULTILINE: i32 = 2;
pub const TXTBIT_NOTHREADREFCOUNT: i32 = 4194304;
pub const TXTBIT_PARAFORMATCHANGE: i32 = 262144;
pub const TXTBIT_READONLY: i32 = 4;
pub const TXTBIT_RICHTEXT: i32 = 1;
pub const TXTBIT_SAVESELECTION: i32 = 64;
pub const TXTBIT_SCROLLBARCHANGE: i32 = 65536;
pub const TXTBIT_SELBARCHANGE: i32 = 512;
pub const TXTBIT_SHOWACCELERATOR: i32 = 8;
pub const TXTBIT_SHOWPASSWORD: i32 = 8388608;
pub const TXTBIT_USECURRENTBKG: i32 = 2097152;
pub const TXTBIT_USEPASSWORD: i32 = 16;
pub const TXTBIT_VERTICAL: i32 = 256;
pub const TXTBIT_VIEWINSETCHANGE: i32 = 8192;
pub const TXTBIT_WORDWRAP: i32 = 1024;
pub type TXTHITRESULT = i32;
pub const TXTHITRESULT_CLOSE: TXTHITRESULT = 2;
pub const TXTHITRESULT_HIT: TXTHITRESULT = 3;
pub const TXTHITRESULT_NOHIT: TXTHITRESULT = 0;
pub const TXTHITRESULT_TRANSPARENT: TXTHITRESULT = 1;
pub type TXTNATURALSIZE = i32;
pub const TXTNS_EMU: TXTNATURALSIZE = -2147483648;
pub const TXTNS_FITTOCONTENT: TXTNATURALSIZE = 1;
pub const TXTNS_FITTOCONTENT2: TXTNATURALSIZE = 0;
pub const TXTNS_FITTOCONTENT3: TXTNATURALSIZE = 3;
pub const TXTNS_FITTOCONTENTWSP: TXTNATURALSIZE = 4;
pub const TXTNS_INCLUDELASTLINE: TXTNATURALSIZE = 1073741824;
pub const TXTNS_ROUNDTOLINE: TXTNATURALSIZE = 2;
pub type TXTVIEW = i32;
pub const TXTVIEW_ACTIVE: TXTVIEW = 0;
pub const TXTVIEW_INACTIVE: TXTVIEW = -1;
