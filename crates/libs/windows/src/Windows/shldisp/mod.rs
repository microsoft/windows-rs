pub type ACENUMOPTION = i32;
pub const ACEO_FIRSTUNUSED: ACENUMOPTION = 65536;
pub const ACEO_MOSTRECENTFIRST: ACENUMOPTION = 1;
pub const ACEO_NONE: ACENUMOPTION = 0;
pub const ACO_AUTOAPPEND: AUTOCOMPLETEOPTIONS = 2;
pub const ACO_AUTOSUGGEST: AUTOCOMPLETEOPTIONS = 1;
pub const ACO_FILTERPREFIXES: AUTOCOMPLETEOPTIONS = 8;
pub const ACO_NONE: AUTOCOMPLETEOPTIONS = 0;
pub const ACO_NOPREFIXFILTERING: AUTOCOMPLETEOPTIONS = 256;
pub const ACO_RTLREADING: AUTOCOMPLETEOPTIONS = 64;
pub const ACO_SEARCH: AUTOCOMPLETEOPTIONS = 4;
pub const ACO_UPDOWNKEYDROPSLIST: AUTOCOMPLETEOPTIONS = 32;
pub const ACO_USETAB: AUTOCOMPLETEOPTIONS = 16;
pub const ACO_WORD_FILTER: AUTOCOMPLETEOPTIONS = 128;
pub type AUTOCOMPLETEOPTIONS = i32;
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(DFConstraint, DFConstraint_Vtbl, 0x4a3df050_23bd_11d2_939f_00a0c91eedba);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for DFConstraint {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(DFConstraint, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl DFConstraint {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Value(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct DFConstraint_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Value: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait DFConstraint_Impl: super::oaidl::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Value(&self) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl DFConstraint_Vtbl {
    pub const fn new<Identity: DFConstraint_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: DFConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match DFConstraint_Impl::Name(this) {
                    Ok(ok__) => {
                        pbs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Value<Identity: DFConstraint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match DFConstraint_Impl::Value(this) {
                    Ok(ok__) => {
                        pv.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(), Name: Name::<Identity, OFFSET>, Value: Value::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DFConstraint as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for DFConstraint {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(DShellFolderViewEvents, DShellFolderViewEvents_Vtbl, 0x62112aa2_ebe4_11cf_a5fb_0020afe7292d);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for DShellFolderViewEvents {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(DShellFolderViewEvents, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct DShellFolderViewEvents_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait DShellFolderViewEvents_Impl: super::oaidl::IDispatch_Impl {}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl DShellFolderViewEvents_Vtbl {
    pub const fn new<Identity: DShellFolderViewEvents_Impl, const OFFSET: isize>() -> Self {
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DShellFolderViewEvents as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for DShellFolderViewEvents {}
pub const FileSearchBand: windows_core::GUID = windows_core::GUID::from_u128(0xc4ee31f3_4768_11d2_be5c_00a0c9a83da1);
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(Folder, Folder_Vtbl, 0xbbcbde60_c3ff_11ce_8350_444553540000);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for Folder {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(Folder, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl Folder {
    pub unsafe fn Title(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Title)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Application(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Application)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Parent(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ParentFolder(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ParentFolder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Items(&self) -> windows_core::Result<FolderItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Items)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ParseName(&self, bname: &windows_core::BSTR) -> windows_core::Result<FolderItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ParseName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn NewFolder(&self, bname: &windows_core::BSTR, voptions: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NewFolder)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bname), core::mem::transmute_copy(voptions)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn MoveHere(&self, vitem: &super::oaidl::VARIANT, voptions: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MoveHere)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vitem), core::mem::transmute_copy(voptions)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CopyHere(&self, vitem: &super::oaidl::VARIANT, voptions: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CopyHere)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vitem), core::mem::transmute_copy(voptions)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetDetailsOf(&self, vitem: &super::oaidl::VARIANT, icolumn: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDetailsOf)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vitem), icolumn, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct Folder_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Application: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ParentFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Items: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ParseName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub NewFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    NewFolder: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub MoveHere: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    MoveHere: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub CopyHere: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    CopyHere: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub GetDetailsOf: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    GetDetailsOf: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait Folder_Impl: super::oaidl::IDispatch_Impl {
    fn Title(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Application(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Parent(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn ParentFolder(&self) -> windows_core::Result<Folder>;
    fn Items(&self) -> windows_core::Result<FolderItems>;
    fn ParseName(&self, bname: &windows_core::BSTR) -> windows_core::Result<FolderItem>;
    fn NewFolder(&self, bname: &windows_core::BSTR, voptions: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn MoveHere(&self, vitem: &super::oaidl::VARIANT, voptions: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn CopyHere(&self, vitem: &super::oaidl::VARIANT, voptions: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn GetDetailsOf(&self, vitem: &super::oaidl::VARIANT, icolumn: i32) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Folder_Vtbl {
    pub const fn new<Identity: Folder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Title<Identity: Folder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Folder_Impl::Title(this) {
                    Ok(ok__) => {
                        pbs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Application<Identity: Folder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Folder_Impl::Application(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Parent<Identity: Folder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Folder_Impl::Parent(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ParentFolder<Identity: Folder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsf: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Folder_Impl::ParentFolder(this) {
                    Ok(ok__) => {
                        ppsf.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Items<Identity: Folder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Folder_Impl::Items(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ParseName<Identity: Folder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bname: *mut core::ffi::c_void, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Folder_Impl::ParseName(this, core::mem::transmute(&bname)) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NewFolder<Identity: Folder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bname: *mut core::ffi::c_void, voptions: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Folder_Impl::NewFolder(this, core::mem::transmute(&bname), core::mem::transmute(&voptions)).into()
            }
        }
        unsafe extern "system" fn MoveHere<Identity: Folder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vitem: super::oaidl::VARIANT, voptions: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Folder_Impl::MoveHere(this, core::mem::transmute(&vitem), core::mem::transmute(&voptions)).into()
            }
        }
        unsafe extern "system" fn CopyHere<Identity: Folder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vitem: super::oaidl::VARIANT, voptions: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Folder_Impl::CopyHere(this, core::mem::transmute(&vitem), core::mem::transmute(&voptions)).into()
            }
        }
        unsafe extern "system" fn GetDetailsOf<Identity: Folder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vitem: super::oaidl::VARIANT, icolumn: i32, pbs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Folder_Impl::GetDetailsOf(this, core::mem::transmute(&vitem), core::mem::transmute_copy(&icolumn)) {
                    Ok(ok__) => {
                        pbs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Title: Title::<Identity, OFFSET>,
            Application: Application::<Identity, OFFSET>,
            Parent: Parent::<Identity, OFFSET>,
            ParentFolder: ParentFolder::<Identity, OFFSET>,
            Items: Items::<Identity, OFFSET>,
            ParseName: ParseName::<Identity, OFFSET>,
            NewFolder: NewFolder::<Identity, OFFSET>,
            MoveHere: MoveHere::<Identity, OFFSET>,
            CopyHere: CopyHere::<Identity, OFFSET>,
            GetDetailsOf: GetDetailsOf::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Folder as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for Folder {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(Folder2, Folder2_Vtbl, 0xf0d2d8ef_3890_11d2_bf8b_00c04fb93661);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for Folder2 {
    type Target = Folder;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(Folder2, windows_core::IUnknown, super::oaidl::IDispatch, Folder);
#[cfg(feature = "oaidl")]
impl Folder2 {
    pub unsafe fn Self_(&self) -> windows_core::Result<FolderItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Self_)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OfflineStatus(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OfflineStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Synchronize(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Synchronize)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn HaveToShowWebViewBarricade(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HaveToShowWebViewBarricade)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DismissedWebViewBarricade(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DismissedWebViewBarricade)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct Folder2_Vtbl {
    pub base__: Folder_Vtbl,
    pub Self_: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OfflineStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Synchronize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub HaveToShowWebViewBarricade: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    HaveToShowWebViewBarricade: usize,
    pub DismissedWebViewBarricade: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait Folder2_Impl: Folder_Impl {
    fn Self_(&self) -> windows_core::Result<FolderItem>;
    fn OfflineStatus(&self) -> windows_core::Result<i32>;
    fn Synchronize(&self) -> windows_core::Result<()>;
    fn HaveToShowWebViewBarricade(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn DismissedWebViewBarricade(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Folder2_Vtbl {
    pub const fn new<Identity: Folder2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Self_<Identity: Folder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Folder2_Impl::Self_(this) {
                    Ok(ok__) => {
                        ppfi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OfflineStatus<Identity: Folder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pul: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Folder2_Impl::OfflineStatus(this) {
                    Ok(ok__) => {
                        pul.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Synchronize<Identity: Folder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Folder2_Impl::Synchronize(this).into()
            }
        }
        unsafe extern "system" fn HaveToShowWebViewBarricade<Identity: Folder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbhavetoshowwebviewbarricade: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Folder2_Impl::HaveToShowWebViewBarricade(this) {
                    Ok(ok__) => {
                        pbhavetoshowwebviewbarricade.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DismissedWebViewBarricade<Identity: Folder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Folder2_Impl::DismissedWebViewBarricade(this).into()
            }
        }
        Self {
            base__: Folder_Vtbl::new::<Identity, OFFSET>(),
            Self_: Self_::<Identity, OFFSET>,
            OfflineStatus: OfflineStatus::<Identity, OFFSET>,
            Synchronize: Synchronize::<Identity, OFFSET>,
            HaveToShowWebViewBarricade: HaveToShowWebViewBarricade::<Identity, OFFSET>,
            DismissedWebViewBarricade: DismissedWebViewBarricade::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Folder2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<Folder as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for Folder2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(Folder3, Folder3_Vtbl, 0xa7ae5f64_c4d7_4d7f_9307_4d24ee54b841);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for Folder3 {
    type Target = Folder2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(Folder3, windows_core::IUnknown, super::oaidl::IDispatch, Folder, Folder2);
#[cfg(feature = "oaidl")]
impl Folder3 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn ShowWebViewBarricade(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowWebViewBarricade)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetShowWebViewBarricade(&self, bshowwebviewbarricade: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetShowWebViewBarricade)(windows_core::Interface::as_raw(self), bshowwebviewbarricade) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct Folder3_Vtbl {
    pub base__: Folder2_Vtbl,
    #[cfg(feature = "wtypes")]
    pub ShowWebViewBarricade: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    ShowWebViewBarricade: usize,
    #[cfg(feature = "wtypes")]
    pub SetShowWebViewBarricade: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetShowWebViewBarricade: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait Folder3_Impl: Folder2_Impl {
    fn ShowWebViewBarricade(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetShowWebViewBarricade(&self, bshowwebviewbarricade: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Folder3_Vtbl {
    pub const fn new<Identity: Folder3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ShowWebViewBarricade<Identity: Folder3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbshowwebviewbarricade: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Folder3_Impl::ShowWebViewBarricade(this) {
                    Ok(ok__) => {
                        pbshowwebviewbarricade.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetShowWebViewBarricade<Identity: Folder3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bshowwebviewbarricade: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                Folder3_Impl::SetShowWebViewBarricade(this, core::mem::transmute_copy(&bshowwebviewbarricade)).into()
            }
        }
        Self {
            base__: Folder2_Vtbl::new::<Identity, OFFSET>(),
            ShowWebViewBarricade: ShowWebViewBarricade::<Identity, OFFSET>,
            SetShowWebViewBarricade: SetShowWebViewBarricade::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Folder3 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<Folder as windows_core::Interface>::IID || iid == &<Folder2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for Folder3 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(FolderItem, FolderItem_Vtbl, 0xfac32c80_cbe4_11ce_8350_444553540000);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for FolderItem {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(FolderItem, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl FolderItem {
    pub unsafe fn Application(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Application)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Parent(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetName(&self, bs: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bs)) }
    }
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetLink(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLink)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFolder(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFolder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsLink(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLink)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsFolder(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsFolder)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsFileSystem(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsFileSystem)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsBrowsable(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsBrowsable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ModifyDate(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModifyDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetModifyDate(&self, dt: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetModifyDate)(windows_core::Interface::as_raw(self), dt) }
    }
    pub unsafe fn Size(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Type(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Type)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Verbs(&self) -> windows_core::Result<FolderItemVerbs> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Verbs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn InvokeVerb(&self, vverb: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InvokeVerb)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vverb)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct FolderItem_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Application: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub IsLink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsLink: usize,
    #[cfg(feature = "wtypes")]
    pub IsFolder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsFolder: usize,
    #[cfg(feature = "wtypes")]
    pub IsFileSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsFileSystem: usize,
    #[cfg(feature = "wtypes")]
    pub IsBrowsable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsBrowsable: usize,
    pub ModifyDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetModifyDate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Verbs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub InvokeVerb: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    InvokeVerb: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait FolderItem_Impl: super::oaidl::IDispatch_Impl {
    fn Application(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Parent(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, bs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetLink(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn GetFolder(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn IsLink(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn IsFolder(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn IsFileSystem(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn IsBrowsable(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn ModifyDate(&self) -> windows_core::Result<f64>;
    fn SetModifyDate(&self, dt: f64) -> windows_core::Result<()>;
    fn Size(&self) -> windows_core::Result<i32>;
    fn Type(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Verbs(&self) -> windows_core::Result<FolderItemVerbs>;
    fn InvokeVerb(&self, vverb: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl FolderItem_Vtbl {
    pub const fn new<Identity: FolderItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Application<Identity: FolderItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItem_Impl::Application(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Parent<Identity: FolderItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItem_Impl::Parent(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: FolderItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItem_Impl::Name(this) {
                    Ok(ok__) => {
                        pbs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: FolderItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                FolderItem_Impl::SetName(this, core::mem::transmute(&bs)).into()
            }
        }
        unsafe extern "system" fn Path<Identity: FolderItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItem_Impl::Path(this) {
                    Ok(ok__) => {
                        pbs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLink<Identity: FolderItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItem_Impl::GetLink(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFolder<Identity: FolderItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItem_Impl::GetFolder(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsLink<Identity: FolderItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pb: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItem_Impl::IsLink(this) {
                    Ok(ok__) => {
                        pb.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsFolder<Identity: FolderItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pb: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItem_Impl::IsFolder(this) {
                    Ok(ok__) => {
                        pb.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsFileSystem<Identity: FolderItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pb: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItem_Impl::IsFileSystem(this) {
                    Ok(ok__) => {
                        pb.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsBrowsable<Identity: FolderItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pb: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItem_Impl::IsBrowsable(this) {
                    Ok(ok__) => {
                        pb.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ModifyDate<Identity: FolderItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdt: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItem_Impl::ModifyDate(this) {
                    Ok(ok__) => {
                        pdt.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetModifyDate<Identity: FolderItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dt: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                FolderItem_Impl::SetModifyDate(this, core::mem::transmute_copy(&dt)).into()
            }
        }
        unsafe extern "system" fn Size<Identity: FolderItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pul: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItem_Impl::Size(this) {
                    Ok(ok__) => {
                        pul.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Type<Identity: FolderItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItem_Impl::Type(this) {
                    Ok(ok__) => {
                        pbs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Verbs<Identity: FolderItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfic: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItem_Impl::Verbs(this) {
                    Ok(ok__) => {
                        ppfic.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InvokeVerb<Identity: FolderItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vverb: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                FolderItem_Impl::InvokeVerb(this, core::mem::transmute(&vverb)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Application: Application::<Identity, OFFSET>,
            Parent: Parent::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Path: Path::<Identity, OFFSET>,
            GetLink: GetLink::<Identity, OFFSET>,
            GetFolder: GetFolder::<Identity, OFFSET>,
            IsLink: IsLink::<Identity, OFFSET>,
            IsFolder: IsFolder::<Identity, OFFSET>,
            IsFileSystem: IsFileSystem::<Identity, OFFSET>,
            IsBrowsable: IsBrowsable::<Identity, OFFSET>,
            ModifyDate: ModifyDate::<Identity, OFFSET>,
            SetModifyDate: SetModifyDate::<Identity, OFFSET>,
            Size: Size::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            Verbs: Verbs::<Identity, OFFSET>,
            InvokeVerb: InvokeVerb::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<FolderItem as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for FolderItem {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(FolderItem2, FolderItem2_Vtbl, 0xedc817aa_92b8_11d1_b075_00c04fc33aa5);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for FolderItem2 {
    type Target = FolderItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(FolderItem2, windows_core::IUnknown, super::oaidl::IDispatch, FolderItem);
#[cfg(feature = "oaidl")]
impl FolderItem2 {
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn InvokeVerbEx(&self, vverb: &super::oaidl::VARIANT, vargs: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InvokeVerbEx)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vverb), core::mem::transmute_copy(vargs)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ExtendedProperty(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExtendedProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpropname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct FolderItem2_Vtbl {
    pub base__: FolderItem_Vtbl,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub InvokeVerbEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    InvokeVerbEx: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ExtendedProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ExtendedProperty: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait FolderItem2_Impl: FolderItem_Impl {
    fn InvokeVerbEx(&self, vverb: &super::oaidl::VARIANT, vargs: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn ExtendedProperty(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl FolderItem2_Vtbl {
    pub const fn new<Identity: FolderItem2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InvokeVerbEx<Identity: FolderItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vverb: super::oaidl::VARIANT, vargs: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                FolderItem2_Impl::InvokeVerbEx(this, core::mem::transmute(&vverb), core::mem::transmute(&vargs)).into()
            }
        }
        unsafe extern "system" fn ExtendedProperty<Identity: FolderItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: *mut core::ffi::c_void, pvret: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItem2_Impl::ExtendedProperty(this, core::mem::transmute(&bstrpropname)) {
                    Ok(ok__) => {
                        pvret.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: FolderItem_Vtbl::new::<Identity, OFFSET>(),
            InvokeVerbEx: InvokeVerbEx::<Identity, OFFSET>,
            ExtendedProperty: ExtendedProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<FolderItem2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<FolderItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for FolderItem2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(FolderItemVerb, FolderItemVerb_Vtbl, 0x08ec3e00_50b0_11cf_960c_0080c7f4ee85);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for FolderItemVerb {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(FolderItemVerb, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl FolderItemVerb {
    pub unsafe fn Application(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Application)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Parent(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DoIt(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DoIt)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct FolderItemVerb_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Application: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DoIt: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait FolderItemVerb_Impl: super::oaidl::IDispatch_Impl {
    fn Application(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Parent(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DoIt(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl FolderItemVerb_Vtbl {
    pub const fn new<Identity: FolderItemVerb_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Application<Identity: FolderItemVerb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItemVerb_Impl::Application(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Parent<Identity: FolderItemVerb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItemVerb_Impl::Parent(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: FolderItemVerb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItemVerb_Impl::Name(this) {
                    Ok(ok__) => {
                        pbs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DoIt<Identity: FolderItemVerb_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                FolderItemVerb_Impl::DoIt(this).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Application: Application::<Identity, OFFSET>,
            Parent: Parent::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            DoIt: DoIt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<FolderItemVerb as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for FolderItemVerb {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(FolderItemVerbs, FolderItemVerbs_Vtbl, 0x1f8352c0_50b0_11cf_960c_0080c7f4ee85);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for FolderItemVerbs {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(FolderItemVerbs, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl FolderItemVerbs {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Application(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Application)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Parent(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Item(&self, index: &super::oaidl::VARIANT) -> windows_core::Result<FolderItemVerb> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct FolderItemVerbs_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Application: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait FolderItemVerbs_Impl: super::oaidl::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Application(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Parent(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Item(&self, index: &super::oaidl::VARIANT) -> windows_core::Result<FolderItemVerb>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl FolderItemVerbs_Vtbl {
    pub const fn new<Identity: FolderItemVerbs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: FolderItemVerbs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItemVerbs_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Application<Identity: FolderItemVerbs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItemVerbs_Impl::Application(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Parent<Identity: FolderItemVerbs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItemVerbs_Impl::Parent(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: FolderItemVerbs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::oaidl::VARIANT, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItemVerbs_Impl::Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: FolderItemVerbs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItemVerbs_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Application: Application::<Identity, OFFSET>,
            Parent: Parent::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<FolderItemVerbs as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for FolderItemVerbs {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(FolderItems, FolderItems_Vtbl, 0x744129e0_cbe5_11ce_8350_444553540000);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for FolderItems {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(FolderItems, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl FolderItems {
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Application(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Application)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Parent(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Item(&self, index: &super::oaidl::VARIANT) -> windows_core::Result<FolderItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(index), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct FolderItems_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Application: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait FolderItems_Impl: super::oaidl::IDispatch_Impl {
    fn Count(&self) -> windows_core::Result<i32>;
    fn Application(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Parent(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Item(&self, index: &super::oaidl::VARIANT) -> windows_core::Result<FolderItem>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl FolderItems_Vtbl {
    pub const fn new<Identity: FolderItems_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: FolderItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItems_Impl::Count(this) {
                    Ok(ok__) => {
                        plcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Application<Identity: FolderItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItems_Impl::Application(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Parent<Identity: FolderItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItems_Impl::Parent(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: FolderItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::oaidl::VARIANT, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItems_Impl::Item(this, core::mem::transmute(&index)) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: FolderItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItems_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, OFFSET>,
            Application: Application::<Identity, OFFSET>,
            Parent: Parent::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<FolderItems as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for FolderItems {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(FolderItems2, FolderItems2_Vtbl, 0xc94f0ad0_f363_11d2_a327_00c04f8eec7f);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for FolderItems2 {
    type Target = FolderItems;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(FolderItems2, windows_core::IUnknown, super::oaidl::IDispatch, FolderItems);
#[cfg(feature = "oaidl")]
impl FolderItems2 {
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn InvokeVerbEx(&self, vverb: &super::oaidl::VARIANT, vargs: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InvokeVerbEx)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vverb), core::mem::transmute_copy(vargs)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct FolderItems2_Vtbl {
    pub base__: FolderItems_Vtbl,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub InvokeVerbEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    InvokeVerbEx: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait FolderItems2_Impl: FolderItems_Impl {
    fn InvokeVerbEx(&self, vverb: &super::oaidl::VARIANT, vargs: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl FolderItems2_Vtbl {
    pub const fn new<Identity: FolderItems2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InvokeVerbEx<Identity: FolderItems2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vverb: super::oaidl::VARIANT, vargs: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                FolderItems2_Impl::InvokeVerbEx(this, core::mem::transmute(&vverb), core::mem::transmute(&vargs)).into()
            }
        }
        Self { base__: FolderItems_Vtbl::new::<Identity, OFFSET>(), InvokeVerbEx: InvokeVerbEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<FolderItems2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<FolderItems as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for FolderItems2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(FolderItems3, FolderItems3_Vtbl, 0xeaa7c309_bbec_49d5_821d_64d966cb667f);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for FolderItems3 {
    type Target = FolderItems2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(FolderItems3, windows_core::IUnknown, super::oaidl::IDispatch, FolderItems, FolderItems2);
#[cfg(feature = "oaidl")]
impl FolderItems3 {
    pub unsafe fn Filter(&self, grfflags: i32, bstrfilespec: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Filter)(windows_core::Interface::as_raw(self), grfflags, core::mem::transmute_copy(bstrfilespec)) }
    }
    pub unsafe fn Verbs(&self) -> windows_core::Result<FolderItemVerbs> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Verbs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct FolderItems3_Vtbl {
    pub base__: FolderItems2_Vtbl,
    pub Filter: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Verbs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait FolderItems3_Impl: FolderItems2_Impl {
    fn Filter(&self, grfflags: i32, bstrfilespec: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Verbs(&self) -> windows_core::Result<FolderItemVerbs>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl FolderItems3_Vtbl {
    pub const fn new<Identity: FolderItems3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Filter<Identity: FolderItems3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfflags: i32, bstrfilespec: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                FolderItems3_Impl::Filter(this, core::mem::transmute_copy(&grfflags), core::mem::transmute(&bstrfilespec)).into()
            }
        }
        unsafe extern "system" fn Verbs<Identity: FolderItems3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfic: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match FolderItems3_Impl::Verbs(this) {
                    Ok(ok__) => {
                        ppfic.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: FolderItems2_Vtbl::new::<Identity, OFFSET>(), Filter: Filter::<Identity, OFFSET>, Verbs: Verbs::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<FolderItems3 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<FolderItems as windows_core::Interface>::IID || iid == &<FolderItems2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for FolderItems3 {}
windows_core::imp::define_interface!(IAutoComplete, IAutoComplete_Vtbl, 0x00bb2762_6a77_11d0_a535_00c04fd7d062);
windows_core::imp::interface_hierarchy!(IAutoComplete, windows_core::IUnknown);
impl IAutoComplete {
    #[cfg(feature = "windef")]
    pub unsafe fn Init<P1, P2, P3>(&self, hwndedit: super::windef::HWND, punkacl: P1, pwszregkeypath: P2, pwszquickcomplete: P3) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), hwndedit, punkacl.param().abi(), pwszregkeypath.param().abi(), pwszquickcomplete.param().abi()) }
    }
    pub unsafe fn Enable(&self, fenable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self), fenable.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutoComplete_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Init: usize,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait IAutoComplete_Impl: windows_core::IUnknownImpl {
    fn Init(&self, hwndedit: super::windef::HWND, punkacl: windows_core::Ref<windows_core::IUnknown>, pwszregkeypath: &windows_core::PCWSTR, pwszquickcomplete: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Enable(&self, fenable: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IAutoComplete_Vtbl {
    pub const fn new<Identity: IAutoComplete_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Init<Identity: IAutoComplete_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndedit: super::windef::HWND, punkacl: *mut core::ffi::c_void, pwszregkeypath: windows_core::PCWSTR, pwszquickcomplete: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutoComplete_Impl::Init(this, core::mem::transmute_copy(&hwndedit), core::mem::transmute_copy(&punkacl), core::mem::transmute(&pwszregkeypath), core::mem::transmute(&pwszquickcomplete)).into()
            }
        }
        unsafe extern "system" fn Enable<Identity: IAutoComplete_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutoComplete_Impl::Enable(this, core::mem::transmute_copy(&fenable)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Init: Init::<Identity, OFFSET>, Enable: Enable::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAutoComplete as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IAutoComplete {}
windows_core::imp::define_interface!(IAutoComplete2, IAutoComplete2_Vtbl, 0xeac04bc0_3791_11d2_bb95_0060977b464c);
impl core::ops::Deref for IAutoComplete2 {
    type Target = IAutoComplete;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAutoComplete2, windows_core::IUnknown, IAutoComplete);
impl IAutoComplete2 {
    pub unsafe fn SetOptions(&self, dwflag: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOptions)(windows_core::Interface::as_raw(self), dwflag) }
    }
    pub unsafe fn GetOptions(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutoComplete2_Vtbl {
    pub base__: IAutoComplete_Vtbl,
    pub SetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait IAutoComplete2_Impl: IAutoComplete_Impl {
    fn SetOptions(&self, dwflag: u32) -> windows_core::Result<()>;
    fn GetOptions(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "windef")]
impl IAutoComplete2_Vtbl {
    pub const fn new<Identity: IAutoComplete2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetOptions<Identity: IAutoComplete2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflag: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAutoComplete2_Impl::SetOptions(this, core::mem::transmute_copy(&dwflag)).into()
            }
        }
        unsafe extern "system" fn GetOptions<Identity: IAutoComplete2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflag: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAutoComplete2_Impl::GetOptions(this) {
                    Ok(ok__) => {
                        pdwflag.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IAutoComplete_Vtbl::new::<Identity, OFFSET>(), SetOptions: SetOptions::<Identity, OFFSET>, GetOptions: GetOptions::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAutoComplete2 as windows_core::Interface>::IID || iid == &<IAutoComplete as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IAutoComplete2 {}
windows_core::imp::define_interface!(IDataObjectAsyncCapability, IDataObjectAsyncCapability_Vtbl, 0x3d8b0590_f691_11d2_8ea9_006097df5bd4);
windows_core::imp::interface_hierarchy!(IDataObjectAsyncCapability, windows_core::IUnknown);
impl IDataObjectAsyncCapability {
    pub unsafe fn SetAsyncMode(&self, fdoopasync: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAsyncMode)(windows_core::Interface::as_raw(self), fdoopasync.into()) }
    }
    pub unsafe fn GetAsyncMode(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAsyncMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn StartOperation<P0>(&self, pbcreserved: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidl::IBindCtx>,
    {
        unsafe { (windows_core::Interface::vtable(self).StartOperation)(windows_core::Interface::as_raw(self), pbcreserved.param().abi()) }
    }
    pub unsafe fn InOperation(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InOperation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn EndOperation<P1>(&self, hresult: windows_core::HRESULT, pbcreserved: P1, dweffects: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::objidl::IBindCtx>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndOperation)(windows_core::Interface::as_raw(self), hresult, pbcreserved.param().abi(), dweffects) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataObjectAsyncCapability_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAsyncMode: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetAsyncMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "objidl")]
    pub StartOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    StartOperation: usize,
    pub InOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "objidl")]
    pub EndOperation: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    EndOperation: usize,
}
#[cfg(feature = "objidl")]
pub trait IDataObjectAsyncCapability_Impl: windows_core::IUnknownImpl {
    fn SetAsyncMode(&self, fdoopasync: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetAsyncMode(&self) -> windows_core::Result<windows_core::BOOL>;
    fn StartOperation(&self, pbcreserved: windows_core::Ref<super::objidl::IBindCtx>) -> windows_core::Result<()>;
    fn InOperation(&self) -> windows_core::Result<windows_core::BOOL>;
    fn EndOperation(&self, hresult: windows_core::HRESULT, pbcreserved: windows_core::Ref<super::objidl::IBindCtx>, dweffects: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "objidl")]
impl IDataObjectAsyncCapability_Vtbl {
    pub const fn new<Identity: IDataObjectAsyncCapability_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAsyncMode<Identity: IDataObjectAsyncCapability_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdoopasync: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataObjectAsyncCapability_Impl::SetAsyncMode(this, core::mem::transmute_copy(&fdoopasync)).into()
            }
        }
        unsafe extern "system" fn GetAsyncMode<Identity: IDataObjectAsyncCapability_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisopasync: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataObjectAsyncCapability_Impl::GetAsyncMode(this) {
                    Ok(ok__) => {
                        pfisopasync.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartOperation<Identity: IDataObjectAsyncCapability_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbcreserved: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataObjectAsyncCapability_Impl::StartOperation(this, core::mem::transmute_copy(&pbcreserved)).into()
            }
        }
        unsafe extern "system" fn InOperation<Identity: IDataObjectAsyncCapability_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfinasyncop: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataObjectAsyncCapability_Impl::InOperation(this) {
                    Ok(ok__) => {
                        pfinasyncop.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndOperation<Identity: IDataObjectAsyncCapability_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hresult: windows_core::HRESULT, pbcreserved: *mut core::ffi::c_void, dweffects: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataObjectAsyncCapability_Impl::EndOperation(this, core::mem::transmute_copy(&hresult), core::mem::transmute_copy(&pbcreserved), core::mem::transmute_copy(&dweffects)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAsyncMode: SetAsyncMode::<Identity, OFFSET>,
            GetAsyncMode: GetAsyncMode::<Identity, OFFSET>,
            StartOperation: StartOperation::<Identity, OFFSET>,
            InOperation: InOperation::<Identity, OFFSET>,
            EndOperation: EndOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataObjectAsyncCapability as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidl")]
impl windows_core::RuntimeName for IDataObjectAsyncCapability {}
#[cfg(feature = "objidlbase")]
windows_core::imp::define_interface!(IEnumACString, IEnumACString_Vtbl, 0x8e74c210_cf9d_4eaf_a403_7356428f0a5a);
#[cfg(feature = "objidlbase")]
impl core::ops::Deref for IEnumACString {
    type Target = super::objidlbase::IEnumString;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidlbase")]
windows_core::imp::interface_hierarchy!(IEnumACString, windows_core::IUnknown, super::objidlbase::IEnumString);
#[cfg(feature = "objidlbase")]
impl IEnumACString {
    pub unsafe fn NextItem(&self, pszurl: Option<&mut [u16]>, pulsortindex: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NextItem)(windows_core::Interface::as_raw(self), core::mem::transmute(pszurl.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pszurl.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pulsortindex as _) }
    }
    pub unsafe fn SetEnumOptions(&self, dwoptions: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnumOptions)(windows_core::Interface::as_raw(self), dwoptions) }
    }
    pub unsafe fn GetEnumOptions(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnumOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "objidlbase")]
#[repr(C)]
#[doc(hidden)]
pub struct IEnumACString_Vtbl {
    pub base__: super::objidlbase::IEnumString_Vtbl,
    pub NextItem: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub SetEnumOptions: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetEnumOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "objidlbase")]
pub trait IEnumACString_Impl: super::objidlbase::IEnumString_Impl {
    fn NextItem(&self, pszurl: windows_core::PWSTR, cchmax: u32, pulsortindex: *mut u32) -> windows_core::Result<()>;
    fn SetEnumOptions(&self, dwoptions: u32) -> windows_core::Result<()>;
    fn GetEnumOptions(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "objidlbase")]
impl IEnumACString_Vtbl {
    pub const fn new<Identity: IEnumACString_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NextItem<Identity: IEnumACString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PWSTR, cchmax: u32, pulsortindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumACString_Impl::NextItem(this, core::mem::transmute_copy(&pszurl), core::mem::transmute_copy(&cchmax), core::mem::transmute_copy(&pulsortindex)).into()
            }
        }
        unsafe extern "system" fn SetEnumOptions<Identity: IEnumACString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoptions: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumACString_Impl::SetEnumOptions(this, core::mem::transmute_copy(&dwoptions)).into()
            }
        }
        unsafe extern "system" fn GetEnumOptions<Identity: IEnumACString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwoptions: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumACString_Impl::GetEnumOptions(this) {
                    Ok(ok__) => {
                        pdwoptions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::objidlbase::IEnumString_Vtbl::new::<Identity, OFFSET>(),
            NextItem: NextItem::<Identity, OFFSET>,
            SetEnumOptions: SetEnumOptions::<Identity, OFFSET>,
            GetEnumOptions: GetEnumOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumACString as windows_core::Interface>::IID || iid == &<super::objidlbase::IEnumString as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidlbase")]
impl windows_core::RuntimeName for IEnumACString {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFileSearchBand, IFileSearchBand_Vtbl, 0x2d91eea1_9932_11d2_be86_00a0c9a83da1);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFileSearchBand {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFileSearchBand, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IFileSearchBand {
    pub unsafe fn SetFocus(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFocus)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetSearchParameters(&self, pbstrsearchid: *const windows_core::BSTR, bnavtoresults: super::wtypes::VARIANT_BOOL, pvarscope: Option<*const super::oaidl::VARIANT>, pvarqueryfile: Option<*const super::oaidl::VARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSearchParameters)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrsearchid), bnavtoresults, pvarscope.unwrap_or(core::mem::zeroed()) as _, pvarqueryfile.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SearchID(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchID)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Scope(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Scope)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn QueryFile(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryFile)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSearchBand_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub SetFocus: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub SetSearchParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, super::wtypes::VARIANT_BOOL, *const super::oaidl::VARIANT, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    SetSearchParameters: usize,
    pub SearchID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Scope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Scope: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub QueryFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    QueryFile: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFileSearchBand_Impl: super::oaidl::IDispatch_Impl {
    fn SetFocus(&self) -> windows_core::Result<()>;
    fn SetSearchParameters(&self, pbstrsearchid: *const windows_core::BSTR, bnavtoresults: super::wtypes::VARIANT_BOOL, pvarscope: *const super::oaidl::VARIANT, pvarqueryfile: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn SearchID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Scope(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn QueryFile(&self) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFileSearchBand_Vtbl {
    pub const fn new<Identity: IFileSearchBand_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFocus<Identity: IFileSearchBand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSearchBand_Impl::SetFocus(this).into()
            }
        }
        unsafe extern "system" fn SetSearchParameters<Identity: IFileSearchBand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsearchid: *const *mut core::ffi::c_void, bnavtoresults: super::wtypes::VARIANT_BOOL, pvarscope: *const super::oaidl::VARIANT, pvarqueryfile: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSearchBand_Impl::SetSearchParameters(this, core::mem::transmute_copy(&pbstrsearchid), core::mem::transmute_copy(&bnavtoresults), core::mem::transmute_copy(&pvarscope), core::mem::transmute_copy(&pvarqueryfile)).into()
            }
        }
        unsafe extern "system" fn SearchID<Identity: IFileSearchBand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsearchid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSearchBand_Impl::SearchID(this) {
                    Ok(ok__) => {
                        pbstrsearchid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Scope<Identity: IFileSearchBand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarscope: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSearchBand_Impl::Scope(this) {
                    Ok(ok__) => {
                        pvarscope.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueryFile<Identity: IFileSearchBand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarfile: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSearchBand_Impl::QueryFile(this) {
                    Ok(ok__) => {
                        pvarfile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetFocus: SetFocus::<Identity, OFFSET>,
            SetSearchParameters: SetSearchParameters::<Identity, OFFSET>,
            SearchID: SearchID::<Identity, OFFSET>,
            Scope: Scope::<Identity, OFFSET>,
            QueryFile: QueryFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSearchBand as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFileSearchBand {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFolderViewOC, IFolderViewOC_Vtbl, 0x9ba05970_f6a8_11cf_a442_00a0c90a8f39);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFolderViewOC {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFolderViewOC, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IFolderViewOC {
    pub unsafe fn SetFolderView<P0>(&self, pdisp: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oaidl::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFolderView)(windows_core::Interface::as_raw(self), pdisp.param().abi()) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFolderViewOC_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub SetFolderView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFolderViewOC_Impl: super::oaidl::IDispatch_Impl {
    fn SetFolderView(&self, pdisp: windows_core::Ref<super::oaidl::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFolderViewOC_Vtbl {
    pub const fn new<Identity: IFolderViewOC_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFolderView<Identity: IFolderViewOC_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdisp: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFolderViewOC_Impl::SetFolderView(this, core::mem::transmute_copy(&pdisp)).into()
            }
        }
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(), SetFolderView: SetFolderView::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFolderViewOC as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFolderViewOC {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(INewWDEvents, INewWDEvents_Vtbl, 0x0751c551_7568_41c9_8e5b_e22e38919236);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for INewWDEvents {
    type Target = IWebWizardHost;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(INewWDEvents, windows_core::IUnknown, super::oaidl::IDispatch, IWebWizardHost);
#[cfg(feature = "oaidl")]
impl INewWDEvents {
    #[cfg(feature = "wtypes")]
    pub unsafe fn PassportAuthenticate(&self, bstrsigninurl: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PassportAuthenticate)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsigninurl), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct INewWDEvents_Vtbl {
    pub base__: IWebWizardHost_Vtbl,
    #[cfg(feature = "wtypes")]
    pub PassportAuthenticate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    PassportAuthenticate: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait INewWDEvents_Impl: IWebWizardHost_Impl {
    fn PassportAuthenticate(&self, bstrsigninurl: &windows_core::BSTR) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl INewWDEvents_Vtbl {
    pub const fn new<Identity: INewWDEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PassportAuthenticate<Identity: INewWDEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsigninurl: *mut core::ffi::c_void, pvfauthenitcated: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INewWDEvents_Impl::PassportAuthenticate(this, core::mem::transmute(&bstrsigninurl)) {
                    Ok(ok__) => {
                        pvfauthenitcated.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWebWizardHost_Vtbl::new::<Identity, OFFSET>(), PassportAuthenticate: PassportAuthenticate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INewWDEvents as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IWebWizardHost as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for INewWDEvents {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShellDispatch, IShellDispatch_Vtbl, 0xd8f015c0_c278_11ce_a49e_444553540000);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShellDispatch {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShellDispatch, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IShellDispatch {
    pub unsafe fn Application(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Application)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Parent(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn NameSpace(&self, vdir: &super::oaidl::VARIANT) -> windows_core::Result<Folder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NameSpace)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vdir), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn BrowseForFolder(&self, hwnd: i32, title: &windows_core::BSTR, options: i32, rootfolder: &super::oaidl::VARIANT) -> windows_core::Result<Folder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BrowseForFolder)(windows_core::Interface::as_raw(self), hwnd, core::mem::transmute_copy(title), options, core::mem::transmute_copy(rootfolder), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Windows(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Windows)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Open(&self, vdir: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vdir)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Explore(&self, vdir: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Explore)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vdir)) }
    }
    pub unsafe fn MinimizeAll(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MinimizeAll)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn UndoMinimizeALL(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UndoMinimizeALL)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn FileRun(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FileRun)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CascadeWindows(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CascadeWindows)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn TileVertically(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TileVertically)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn TileHorizontally(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TileHorizontally)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ShutdownWindows(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShutdownWindows)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Suspend(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Suspend)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EjectPC(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EjectPC)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetTime(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTime)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn TrayProperties(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TrayProperties)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Help(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Help)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn FindFiles(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FindFiles)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn FindComputer(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FindComputer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RefreshMenu(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RefreshMenu)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ControlPanelItem(&self, bstrdir: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ControlPanelItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrdir)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShellDispatch_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Application: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub NameSpace: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    NameSpace: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub BrowseForFolder: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, i32, super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    BrowseForFolder: usize,
    pub Windows: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Open: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Explore: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Explore: usize,
    pub MinimizeAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UndoMinimizeALL: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FileRun: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CascadeWindows: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TileVertically: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TileHorizontally: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShutdownWindows: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Suspend: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EjectPC: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTime: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TrayProperties: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Help: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindFiles: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindComputer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RefreshMenu: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ControlPanelItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShellDispatch_Impl: super::oaidl::IDispatch_Impl {
    fn Application(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Parent(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn NameSpace(&self, vdir: &super::oaidl::VARIANT) -> windows_core::Result<Folder>;
    fn BrowseForFolder(&self, hwnd: i32, title: &windows_core::BSTR, options: i32, rootfolder: &super::oaidl::VARIANT) -> windows_core::Result<Folder>;
    fn Windows(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Open(&self, vdir: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Explore(&self, vdir: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn MinimizeAll(&self) -> windows_core::Result<()>;
    fn UndoMinimizeALL(&self) -> windows_core::Result<()>;
    fn FileRun(&self) -> windows_core::Result<()>;
    fn CascadeWindows(&self) -> windows_core::Result<()>;
    fn TileVertically(&self) -> windows_core::Result<()>;
    fn TileHorizontally(&self) -> windows_core::Result<()>;
    fn ShutdownWindows(&self) -> windows_core::Result<()>;
    fn Suspend(&self) -> windows_core::Result<()>;
    fn EjectPC(&self) -> windows_core::Result<()>;
    fn SetTime(&self) -> windows_core::Result<()>;
    fn TrayProperties(&self) -> windows_core::Result<()>;
    fn Help(&self) -> windows_core::Result<()>;
    fn FindFiles(&self) -> windows_core::Result<()>;
    fn FindComputer(&self) -> windows_core::Result<()>;
    fn RefreshMenu(&self) -> windows_core::Result<()>;
    fn ControlPanelItem(&self, bstrdir: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShellDispatch_Vtbl {
    pub const fn new<Identity: IShellDispatch_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Application<Identity: IShellDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellDispatch_Impl::Application(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Parent<Identity: IShellDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellDispatch_Impl::Parent(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NameSpace<Identity: IShellDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vdir: super::oaidl::VARIANT, ppsdf: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellDispatch_Impl::NameSpace(this, core::mem::transmute(&vdir)) {
                    Ok(ok__) => {
                        ppsdf.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BrowseForFolder<Identity: IShellDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: i32, title: *mut core::ffi::c_void, options: i32, rootfolder: super::oaidl::VARIANT, ppsdf: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellDispatch_Impl::BrowseForFolder(this, core::mem::transmute_copy(&hwnd), core::mem::transmute(&title), core::mem::transmute_copy(&options), core::mem::transmute(&rootfolder)) {
                    Ok(ok__) => {
                        ppsdf.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Windows<Identity: IShellDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellDispatch_Impl::Windows(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Open<Identity: IShellDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vdir: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch_Impl::Open(this, core::mem::transmute(&vdir)).into()
            }
        }
        unsafe extern "system" fn Explore<Identity: IShellDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vdir: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch_Impl::Explore(this, core::mem::transmute(&vdir)).into()
            }
        }
        unsafe extern "system" fn MinimizeAll<Identity: IShellDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch_Impl::MinimizeAll(this).into()
            }
        }
        unsafe extern "system" fn UndoMinimizeALL<Identity: IShellDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch_Impl::UndoMinimizeALL(this).into()
            }
        }
        unsafe extern "system" fn FileRun<Identity: IShellDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch_Impl::FileRun(this).into()
            }
        }
        unsafe extern "system" fn CascadeWindows<Identity: IShellDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch_Impl::CascadeWindows(this).into()
            }
        }
        unsafe extern "system" fn TileVertically<Identity: IShellDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch_Impl::TileVertically(this).into()
            }
        }
        unsafe extern "system" fn TileHorizontally<Identity: IShellDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch_Impl::TileHorizontally(this).into()
            }
        }
        unsafe extern "system" fn ShutdownWindows<Identity: IShellDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch_Impl::ShutdownWindows(this).into()
            }
        }
        unsafe extern "system" fn Suspend<Identity: IShellDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch_Impl::Suspend(this).into()
            }
        }
        unsafe extern "system" fn EjectPC<Identity: IShellDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch_Impl::EjectPC(this).into()
            }
        }
        unsafe extern "system" fn SetTime<Identity: IShellDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch_Impl::SetTime(this).into()
            }
        }
        unsafe extern "system" fn TrayProperties<Identity: IShellDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch_Impl::TrayProperties(this).into()
            }
        }
        unsafe extern "system" fn Help<Identity: IShellDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch_Impl::Help(this).into()
            }
        }
        unsafe extern "system" fn FindFiles<Identity: IShellDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch_Impl::FindFiles(this).into()
            }
        }
        unsafe extern "system" fn FindComputer<Identity: IShellDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch_Impl::FindComputer(this).into()
            }
        }
        unsafe extern "system" fn RefreshMenu<Identity: IShellDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch_Impl::RefreshMenu(this).into()
            }
        }
        unsafe extern "system" fn ControlPanelItem<Identity: IShellDispatch_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdir: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch_Impl::ControlPanelItem(this, core::mem::transmute(&bstrdir)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Application: Application::<Identity, OFFSET>,
            Parent: Parent::<Identity, OFFSET>,
            NameSpace: NameSpace::<Identity, OFFSET>,
            BrowseForFolder: BrowseForFolder::<Identity, OFFSET>,
            Windows: Windows::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            Explore: Explore::<Identity, OFFSET>,
            MinimizeAll: MinimizeAll::<Identity, OFFSET>,
            UndoMinimizeALL: UndoMinimizeALL::<Identity, OFFSET>,
            FileRun: FileRun::<Identity, OFFSET>,
            CascadeWindows: CascadeWindows::<Identity, OFFSET>,
            TileVertically: TileVertically::<Identity, OFFSET>,
            TileHorizontally: TileHorizontally::<Identity, OFFSET>,
            ShutdownWindows: ShutdownWindows::<Identity, OFFSET>,
            Suspend: Suspend::<Identity, OFFSET>,
            EjectPC: EjectPC::<Identity, OFFSET>,
            SetTime: SetTime::<Identity, OFFSET>,
            TrayProperties: TrayProperties::<Identity, OFFSET>,
            Help: Help::<Identity, OFFSET>,
            FindFiles: FindFiles::<Identity, OFFSET>,
            FindComputer: FindComputer::<Identity, OFFSET>,
            RefreshMenu: RefreshMenu::<Identity, OFFSET>,
            ControlPanelItem: ControlPanelItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellDispatch as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShellDispatch {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShellDispatch2, IShellDispatch2_Vtbl, 0xa4c6892c_3ba9_11d2_9dea_00c04fb16162);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShellDispatch2 {
    type Target = IShellDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShellDispatch2, windows_core::IUnknown, super::oaidl::IDispatch, IShellDispatch);
#[cfg(feature = "oaidl")]
impl IShellDispatch2 {
    pub unsafe fn IsRestricted(&self, group: &windows_core::BSTR, restriction: &windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRestricted)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(group), core::mem::transmute_copy(restriction), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ShellExecute(&self, file: &windows_core::BSTR, vargs: &super::oaidl::VARIANT, vdir: &super::oaidl::VARIANT, voperation: &super::oaidl::VARIANT, vshow: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShellExecute)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(file), core::mem::transmute_copy(vargs), core::mem::transmute_copy(vdir), core::mem::transmute_copy(voperation), core::mem::transmute_copy(vshow)) }
    }
    pub unsafe fn FindPrinter(&self, name: &windows_core::BSTR, location: &windows_core::BSTR, model: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FindPrinter)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), core::mem::transmute_copy(location), core::mem::transmute_copy(model)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetSystemInformation(&self, name: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSystemInformation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ServiceStart(&self, servicename: &windows_core::BSTR, persistent: &super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceStart)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(servicename), core::mem::transmute_copy(persistent), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ServiceStop(&self, servicename: &windows_core::BSTR, persistent: &super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServiceStop)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(servicename), core::mem::transmute_copy(persistent), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn IsServiceRunning(&self, servicename: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsServiceRunning)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(servicename), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CanStartStopService(&self, servicename: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanStartStopService)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(servicename), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ShowBrowserBar(&self, bstrclsid: &windows_core::BSTR, bshow: &super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowBrowserBar)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrclsid), core::mem::transmute_copy(bshow), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShellDispatch2_Vtbl {
    pub base__: IShellDispatch_Vtbl,
    pub IsRestricted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ShellExecute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT, super::oaidl::VARIANT, super::oaidl::VARIANT, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ShellExecute: usize,
    pub FindPrinter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub GetSystemInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    GetSystemInformation: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ServiceStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ServiceStart: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ServiceStop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ServiceStop: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub IsServiceRunning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    IsServiceRunning: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub CanStartStopService: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    CanStartStopService: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ShowBrowserBar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ShowBrowserBar: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShellDispatch2_Impl: IShellDispatch_Impl {
    fn IsRestricted(&self, group: &windows_core::BSTR, restriction: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn ShellExecute(&self, file: &windows_core::BSTR, vargs: &super::oaidl::VARIANT, vdir: &super::oaidl::VARIANT, voperation: &super::oaidl::VARIANT, vshow: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn FindPrinter(&self, name: &windows_core::BSTR, location: &windows_core::BSTR, model: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetSystemInformation(&self, name: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
    fn ServiceStart(&self, servicename: &windows_core::BSTR, persistent: &super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT>;
    fn ServiceStop(&self, servicename: &windows_core::BSTR, persistent: &super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT>;
    fn IsServiceRunning(&self, servicename: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
    fn CanStartStopService(&self, servicename: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
    fn ShowBrowserBar(&self, bstrclsid: &windows_core::BSTR, bshow: &super::oaidl::VARIANT) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShellDispatch2_Vtbl {
    pub const fn new<Identity: IShellDispatch2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsRestricted<Identity: IShellDispatch2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, group: *mut core::ffi::c_void, restriction: *mut core::ffi::c_void, plrestrictvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellDispatch2_Impl::IsRestricted(this, core::mem::transmute(&group), core::mem::transmute(&restriction)) {
                    Ok(ok__) => {
                        plrestrictvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ShellExecute<Identity: IShellDispatch2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, file: *mut core::ffi::c_void, vargs: super::oaidl::VARIANT, vdir: super::oaidl::VARIANT, voperation: super::oaidl::VARIANT, vshow: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch2_Impl::ShellExecute(this, core::mem::transmute(&file), core::mem::transmute(&vargs), core::mem::transmute(&vdir), core::mem::transmute(&voperation), core::mem::transmute(&vshow)).into()
            }
        }
        unsafe extern "system" fn FindPrinter<Identity: IShellDispatch2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, location: *mut core::ffi::c_void, model: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch2_Impl::FindPrinter(this, core::mem::transmute(&name), core::mem::transmute(&location), core::mem::transmute(&model)).into()
            }
        }
        unsafe extern "system" fn GetSystemInformation<Identity: IShellDispatch2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, pv: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellDispatch2_Impl::GetSystemInformation(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        pv.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ServiceStart<Identity: IShellDispatch2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: *mut core::ffi::c_void, persistent: super::oaidl::VARIANT, psuccess: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellDispatch2_Impl::ServiceStart(this, core::mem::transmute(&servicename), core::mem::transmute(&persistent)) {
                    Ok(ok__) => {
                        psuccess.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ServiceStop<Identity: IShellDispatch2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: *mut core::ffi::c_void, persistent: super::oaidl::VARIANT, psuccess: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellDispatch2_Impl::ServiceStop(this, core::mem::transmute(&servicename), core::mem::transmute(&persistent)) {
                    Ok(ok__) => {
                        psuccess.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsServiceRunning<Identity: IShellDispatch2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: *mut core::ffi::c_void, prunning: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellDispatch2_Impl::IsServiceRunning(this, core::mem::transmute(&servicename)) {
                    Ok(ok__) => {
                        prunning.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanStartStopService<Identity: IShellDispatch2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, servicename: *mut core::ffi::c_void, pcanstartstop: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellDispatch2_Impl::CanStartStopService(this, core::mem::transmute(&servicename)) {
                    Ok(ok__) => {
                        pcanstartstop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ShowBrowserBar<Identity: IShellDispatch2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrclsid: *mut core::ffi::c_void, bshow: super::oaidl::VARIANT, psuccess: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellDispatch2_Impl::ShowBrowserBar(this, core::mem::transmute(&bstrclsid), core::mem::transmute(&bshow)) {
                    Ok(ok__) => {
                        psuccess.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IShellDispatch_Vtbl::new::<Identity, OFFSET>(),
            IsRestricted: IsRestricted::<Identity, OFFSET>,
            ShellExecute: ShellExecute::<Identity, OFFSET>,
            FindPrinter: FindPrinter::<Identity, OFFSET>,
            GetSystemInformation: GetSystemInformation::<Identity, OFFSET>,
            ServiceStart: ServiceStart::<Identity, OFFSET>,
            ServiceStop: ServiceStop::<Identity, OFFSET>,
            IsServiceRunning: IsServiceRunning::<Identity, OFFSET>,
            CanStartStopService: CanStartStopService::<Identity, OFFSET>,
            ShowBrowserBar: ShowBrowserBar::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellDispatch2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IShellDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShellDispatch2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShellDispatch3, IShellDispatch3_Vtbl, 0x177160ca_bb5a_411c_841d_bd38facdeaa0);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShellDispatch3 {
    type Target = IShellDispatch2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShellDispatch3, windows_core::IUnknown, super::oaidl::IDispatch, IShellDispatch, IShellDispatch2);
#[cfg(feature = "oaidl")]
impl IShellDispatch3 {
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AddToRecent(&self, varfile: &super::oaidl::VARIANT, bstrcategory: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddToRecent)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(varfile), core::mem::transmute_copy(bstrcategory)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShellDispatch3_Vtbl {
    pub base__: IShellDispatch2_Vtbl,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub AddToRecent: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    AddToRecent: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShellDispatch3_Impl: IShellDispatch2_Impl {
    fn AddToRecent(&self, varfile: &super::oaidl::VARIANT, bstrcategory: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShellDispatch3_Vtbl {
    pub const fn new<Identity: IShellDispatch3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddToRecent<Identity: IShellDispatch3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, varfile: super::oaidl::VARIANT, bstrcategory: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch3_Impl::AddToRecent(this, core::mem::transmute(&varfile), core::mem::transmute(&bstrcategory)).into()
            }
        }
        Self { base__: IShellDispatch2_Vtbl::new::<Identity, OFFSET>(), AddToRecent: AddToRecent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellDispatch3 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IShellDispatch as windows_core::Interface>::IID || iid == &<IShellDispatch2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShellDispatch3 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShellDispatch4, IShellDispatch4_Vtbl, 0xefd84b2d_4bcf_4298_be25_eb542a59fbda);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShellDispatch4 {
    type Target = IShellDispatch3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShellDispatch4, windows_core::IUnknown, super::oaidl::IDispatch, IShellDispatch, IShellDispatch2, IShellDispatch3);
#[cfg(feature = "oaidl")]
impl IShellDispatch4 {
    pub unsafe fn WindowsSecurity(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WindowsSecurity)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ToggleDesktop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ToggleDesktop)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ExplorerPolicy(&self, bstrpolicyname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExplorerPolicy)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpolicyname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn GetSetting(&self, lsetting: i32) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSetting)(windows_core::Interface::as_raw(self), lsetting, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShellDispatch4_Vtbl {
    pub base__: IShellDispatch3_Vtbl,
    pub WindowsSecurity: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ToggleDesktop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub ExplorerPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    ExplorerPolicy: usize,
    #[cfg(feature = "wtypes")]
    pub GetSetting: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    GetSetting: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShellDispatch4_Impl: IShellDispatch3_Impl {
    fn WindowsSecurity(&self) -> windows_core::Result<()>;
    fn ToggleDesktop(&self) -> windows_core::Result<()>;
    fn ExplorerPolicy(&self, bstrpolicyname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
    fn GetSetting(&self, lsetting: i32) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShellDispatch4_Vtbl {
    pub const fn new<Identity: IShellDispatch4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WindowsSecurity<Identity: IShellDispatch4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch4_Impl::WindowsSecurity(this).into()
            }
        }
        unsafe extern "system" fn ToggleDesktop<Identity: IShellDispatch4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch4_Impl::ToggleDesktop(this).into()
            }
        }
        unsafe extern "system" fn ExplorerPolicy<Identity: IShellDispatch4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpolicyname: *mut core::ffi::c_void, pvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellDispatch4_Impl::ExplorerPolicy(this, core::mem::transmute(&bstrpolicyname)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSetting<Identity: IShellDispatch4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsetting: i32, presult: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellDispatch4_Impl::GetSetting(this, core::mem::transmute_copy(&lsetting)) {
                    Ok(ok__) => {
                        presult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IShellDispatch3_Vtbl::new::<Identity, OFFSET>(),
            WindowsSecurity: WindowsSecurity::<Identity, OFFSET>,
            ToggleDesktop: ToggleDesktop::<Identity, OFFSET>,
            ExplorerPolicy: ExplorerPolicy::<Identity, OFFSET>,
            GetSetting: GetSetting::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellDispatch4 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IShellDispatch as windows_core::Interface>::IID || iid == &<IShellDispatch2 as windows_core::Interface>::IID || iid == &<IShellDispatch3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShellDispatch4 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShellDispatch5, IShellDispatch5_Vtbl, 0x866738b9_6cf2_4de8_8767_f794ebe74f4e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShellDispatch5 {
    type Target = IShellDispatch4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShellDispatch5, windows_core::IUnknown, super::oaidl::IDispatch, IShellDispatch, IShellDispatch2, IShellDispatch3, IShellDispatch4);
#[cfg(feature = "oaidl")]
impl IShellDispatch5 {
    pub unsafe fn WindowSwitcher(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WindowSwitcher)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShellDispatch5_Vtbl {
    pub base__: IShellDispatch4_Vtbl,
    pub WindowSwitcher: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShellDispatch5_Impl: IShellDispatch4_Impl {
    fn WindowSwitcher(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShellDispatch5_Vtbl {
    pub const fn new<Identity: IShellDispatch5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WindowSwitcher<Identity: IShellDispatch5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch5_Impl::WindowSwitcher(this).into()
            }
        }
        Self { base__: IShellDispatch4_Vtbl::new::<Identity, OFFSET>(), WindowSwitcher: WindowSwitcher::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellDispatch5 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IShellDispatch as windows_core::Interface>::IID || iid == &<IShellDispatch2 as windows_core::Interface>::IID || iid == &<IShellDispatch3 as windows_core::Interface>::IID || iid == &<IShellDispatch4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShellDispatch5 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShellDispatch6, IShellDispatch6_Vtbl, 0x286e6f1b_7113_4355_9562_96b7e9d64c54);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShellDispatch6 {
    type Target = IShellDispatch5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShellDispatch6, windows_core::IUnknown, super::oaidl::IDispatch, IShellDispatch, IShellDispatch2, IShellDispatch3, IShellDispatch4, IShellDispatch5);
#[cfg(feature = "oaidl")]
impl IShellDispatch6 {
    pub unsafe fn SearchCommand(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SearchCommand)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShellDispatch6_Vtbl {
    pub base__: IShellDispatch5_Vtbl,
    pub SearchCommand: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShellDispatch6_Impl: IShellDispatch5_Impl {
    fn SearchCommand(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShellDispatch6_Vtbl {
    pub const fn new<Identity: IShellDispatch6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SearchCommand<Identity: IShellDispatch6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellDispatch6_Impl::SearchCommand(this).into()
            }
        }
        Self { base__: IShellDispatch5_Vtbl::new::<Identity, OFFSET>(), SearchCommand: SearchCommand::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellDispatch6 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IShellDispatch as windows_core::Interface>::IID || iid == &<IShellDispatch2 as windows_core::Interface>::IID || iid == &<IShellDispatch3 as windows_core::Interface>::IID || iid == &<IShellDispatch4 as windows_core::Interface>::IID || iid == &<IShellDispatch5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShellDispatch6 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShellFolderViewDual, IShellFolderViewDual_Vtbl, 0xe7a1af80_4d96_11cf_960c_0080c7f4ee85);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShellFolderViewDual {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShellFolderViewDual, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IShellFolderViewDual {
    pub unsafe fn Application(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Application)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Parent(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Folder(&self) -> windows_core::Result<Folder> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Folder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SelectedItems(&self) -> windows_core::Result<FolderItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectedItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FocusedItem(&self) -> windows_core::Result<FolderItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FocusedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SelectItem(&self, pvfi: *const super::oaidl::VARIANT, dwflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SelectItem)(windows_core::Interface::as_raw(self), core::mem::transmute(pvfi), dwflags) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn PopupItemMenu<P0>(&self, pfi: P0, vx: &super::oaidl::VARIANT, vy: &super::oaidl::VARIANT) -> windows_core::Result<windows_core::BSTR>
    where
        P0: windows_core::Param<FolderItem>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PopupItemMenu)(windows_core::Interface::as_raw(self), pfi.param().abi(), core::mem::transmute_copy(vx), core::mem::transmute_copy(vy), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Script(&self) -> windows_core::Result<super::oaidl::IDispatch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Script)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ViewOptions(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ViewOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShellFolderViewDual_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Application: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Folder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectedItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FocusedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub SelectItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::VARIANT, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    SelectItem: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub PopupItemMenu: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::oaidl::VARIANT, super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    PopupItemMenu: usize,
    pub Script: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ViewOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShellFolderViewDual_Impl: super::oaidl::IDispatch_Impl {
    fn Application(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Parent(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn Folder(&self) -> windows_core::Result<Folder>;
    fn SelectedItems(&self) -> windows_core::Result<FolderItems>;
    fn FocusedItem(&self) -> windows_core::Result<FolderItem>;
    fn SelectItem(&self, pvfi: *const super::oaidl::VARIANT, dwflags: i32) -> windows_core::Result<()>;
    fn PopupItemMenu(&self, pfi: windows_core::Ref<FolderItem>, vx: &super::oaidl::VARIANT, vy: &super::oaidl::VARIANT) -> windows_core::Result<windows_core::BSTR>;
    fn Script(&self) -> windows_core::Result<super::oaidl::IDispatch>;
    fn ViewOptions(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShellFolderViewDual_Vtbl {
    pub const fn new<Identity: IShellFolderViewDual_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Application<Identity: IShellFolderViewDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderViewDual_Impl::Application(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Parent<Identity: IShellFolderViewDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderViewDual_Impl::Parent(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Folder<Identity: IShellFolderViewDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderViewDual_Impl::Folder(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SelectedItems<Identity: IShellFolderViewDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderViewDual_Impl::SelectedItems(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FocusedItem<Identity: IShellFolderViewDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderViewDual_Impl::FocusedItem(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SelectItem<Identity: IShellFolderViewDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvfi: *const super::oaidl::VARIANT, dwflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderViewDual_Impl::SelectItem(this, core::mem::transmute_copy(&pvfi), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn PopupItemMenu<Identity: IShellFolderViewDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfi: *mut core::ffi::c_void, vx: super::oaidl::VARIANT, vy: super::oaidl::VARIANT, pbs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderViewDual_Impl::PopupItemMenu(this, core::mem::transmute_copy(&pfi), core::mem::transmute(&vx), core::mem::transmute(&vy)) {
                    Ok(ok__) => {
                        pbs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Script<Identity: IShellFolderViewDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdisp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderViewDual_Impl::Script(this) {
                    Ok(ok__) => {
                        ppdisp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ViewOptions<Identity: IShellFolderViewDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plviewoptions: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderViewDual_Impl::ViewOptions(this) {
                    Ok(ok__) => {
                        plviewoptions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Application: Application::<Identity, OFFSET>,
            Parent: Parent::<Identity, OFFSET>,
            Folder: Folder::<Identity, OFFSET>,
            SelectedItems: SelectedItems::<Identity, OFFSET>,
            FocusedItem: FocusedItem::<Identity, OFFSET>,
            SelectItem: SelectItem::<Identity, OFFSET>,
            PopupItemMenu: PopupItemMenu::<Identity, OFFSET>,
            Script: Script::<Identity, OFFSET>,
            ViewOptions: ViewOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellFolderViewDual as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShellFolderViewDual {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShellFolderViewDual2, IShellFolderViewDual2_Vtbl, 0x31c147b6_0ade_4a3c_b514_ddf932ef6d17);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShellFolderViewDual2 {
    type Target = IShellFolderViewDual;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShellFolderViewDual2, windows_core::IUnknown, super::oaidl::IDispatch, IShellFolderViewDual);
#[cfg(feature = "oaidl")]
impl IShellFolderViewDual2 {
    pub unsafe fn CurrentViewMode(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentViewMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCurrentViewMode(&self, viewmode: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentViewMode)(windows_core::Interface::as_raw(self), viewmode) }
    }
    pub unsafe fn SelectItemRelative(&self, irelative: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SelectItemRelative)(windows_core::Interface::as_raw(self), irelative) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShellFolderViewDual2_Vtbl {
    pub base__: IShellFolderViewDual_Vtbl,
    pub CurrentViewMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetCurrentViewMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SelectItemRelative: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShellFolderViewDual2_Impl: IShellFolderViewDual_Impl {
    fn CurrentViewMode(&self) -> windows_core::Result<u32>;
    fn SetCurrentViewMode(&self, viewmode: u32) -> windows_core::Result<()>;
    fn SelectItemRelative(&self, irelative: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShellFolderViewDual2_Vtbl {
    pub const fn new<Identity: IShellFolderViewDual2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentViewMode<Identity: IShellFolderViewDual2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pviewmode: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderViewDual2_Impl::CurrentViewMode(this) {
                    Ok(ok__) => {
                        pviewmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCurrentViewMode<Identity: IShellFolderViewDual2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewmode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderViewDual2_Impl::SetCurrentViewMode(this, core::mem::transmute_copy(&viewmode)).into()
            }
        }
        unsafe extern "system" fn SelectItemRelative<Identity: IShellFolderViewDual2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, irelative: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderViewDual2_Impl::SelectItemRelative(this, core::mem::transmute_copy(&irelative)).into()
            }
        }
        Self {
            base__: IShellFolderViewDual_Vtbl::new::<Identity, OFFSET>(),
            CurrentViewMode: CurrentViewMode::<Identity, OFFSET>,
            SetCurrentViewMode: SetCurrentViewMode::<Identity, OFFSET>,
            SelectItemRelative: SelectItemRelative::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellFolderViewDual2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IShellFolderViewDual as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShellFolderViewDual2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShellFolderViewDual3, IShellFolderViewDual3_Vtbl, 0x29ec8e6c_46d3_411f_baaa_611a6c9cac66);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShellFolderViewDual3 {
    type Target = IShellFolderViewDual2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShellFolderViewDual3, windows_core::IUnknown, super::oaidl::IDispatch, IShellFolderViewDual, IShellFolderViewDual2);
#[cfg(feature = "oaidl")]
impl IShellFolderViewDual3 {
    pub unsafe fn GroupBy(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GroupBy)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetGroupBy(&self, bstrgroupby: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGroupBy)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrgroupby)) }
    }
    pub unsafe fn FolderFlags(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FolderFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFolderFlags(&self, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFolderFlags)(windows_core::Interface::as_raw(self), dwflags) }
    }
    pub unsafe fn SortColumns(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SortColumns)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSortColumns(&self, bstrsortcolumns: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSortColumns)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsortcolumns)) }
    }
    pub unsafe fn SetIconSize(&self, iiconsize: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIconSize)(windows_core::Interface::as_raw(self), iiconsize) }
    }
    pub unsafe fn IconSize(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IconSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FilterView(&self, bstrfiltertext: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FilterView)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrfiltertext)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShellFolderViewDual3_Vtbl {
    pub base__: IShellFolderViewDual2_Vtbl,
    pub GroupBy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetGroupBy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FolderFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFolderFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SortColumns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSortColumns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIconSize: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IconSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FilterView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShellFolderViewDual3_Impl: IShellFolderViewDual2_Impl {
    fn GroupBy(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetGroupBy(&self, bstrgroupby: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FolderFlags(&self) -> windows_core::Result<u32>;
    fn SetFolderFlags(&self, dwflags: u32) -> windows_core::Result<()>;
    fn SortColumns(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSortColumns(&self, bstrsortcolumns: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetIconSize(&self, iiconsize: i32) -> windows_core::Result<()>;
    fn IconSize(&self) -> windows_core::Result<i32>;
    fn FilterView(&self, bstrfiltertext: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShellFolderViewDual3_Vtbl {
    pub const fn new<Identity: IShellFolderViewDual3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GroupBy<Identity: IShellFolderViewDual3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrgroupby: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderViewDual3_Impl::GroupBy(this) {
                    Ok(ok__) => {
                        pbstrgroupby.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGroupBy<Identity: IShellFolderViewDual3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrgroupby: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderViewDual3_Impl::SetGroupBy(this, core::mem::transmute(&bstrgroupby)).into()
            }
        }
        unsafe extern "system" fn FolderFlags<Identity: IShellFolderViewDual3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderViewDual3_Impl::FolderFlags(this) {
                    Ok(ok__) => {
                        pdwflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFolderFlags<Identity: IShellFolderViewDual3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderViewDual3_Impl::SetFolderFlags(this, core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn SortColumns<Identity: IShellFolderViewDual3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsortcolumns: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderViewDual3_Impl::SortColumns(this) {
                    Ok(ok__) => {
                        pbstrsortcolumns.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSortColumns<Identity: IShellFolderViewDual3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsortcolumns: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderViewDual3_Impl::SetSortColumns(this, core::mem::transmute(&bstrsortcolumns)).into()
            }
        }
        unsafe extern "system" fn SetIconSize<Identity: IShellFolderViewDual3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iiconsize: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderViewDual3_Impl::SetIconSize(this, core::mem::transmute_copy(&iiconsize)).into()
            }
        }
        unsafe extern "system" fn IconSize<Identity: IShellFolderViewDual3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piiconsize: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellFolderViewDual3_Impl::IconSize(this) {
                    Ok(ok__) => {
                        piiconsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FilterView<Identity: IShellFolderViewDual3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfiltertext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellFolderViewDual3_Impl::FilterView(this, core::mem::transmute(&bstrfiltertext)).into()
            }
        }
        Self {
            base__: IShellFolderViewDual2_Vtbl::new::<Identity, OFFSET>(),
            GroupBy: GroupBy::<Identity, OFFSET>,
            SetGroupBy: SetGroupBy::<Identity, OFFSET>,
            FolderFlags: FolderFlags::<Identity, OFFSET>,
            SetFolderFlags: SetFolderFlags::<Identity, OFFSET>,
            SortColumns: SortColumns::<Identity, OFFSET>,
            SetSortColumns: SetSortColumns::<Identity, OFFSET>,
            SetIconSize: SetIconSize::<Identity, OFFSET>,
            IconSize: IconSize::<Identity, OFFSET>,
            FilterView: FilterView::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellFolderViewDual3 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IShellFolderViewDual as windows_core::Interface>::IID || iid == &<IShellFolderViewDual2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShellFolderViewDual3 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShellLinkDual, IShellLinkDual_Vtbl, 0x88a05c00_f000_11ce_8350_444553540000);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShellLinkDual {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShellLinkDual, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IShellLinkDual {
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPath(&self, bs: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bs)) }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDescription(&self, bs: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bs)) }
    }
    pub unsafe fn WorkingDirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WorkingDirectory)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetWorkingDirectory(&self, bs: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWorkingDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bs)) }
    }
    pub unsafe fn Arguments(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Arguments)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetArguments(&self, bs: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetArguments)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bs)) }
    }
    pub unsafe fn Hotkey(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Hotkey)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetHotkey(&self, ihk: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHotkey)(windows_core::Interface::as_raw(self), ihk) }
    }
    pub unsafe fn ShowCommand(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowCommand)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetShowCommand(&self, ishowcommand: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetShowCommand)(windows_core::Interface::as_raw(self), ishowcommand) }
    }
    pub unsafe fn Resolve(&self, fflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Resolve)(windows_core::Interface::as_raw(self), fflags) }
    }
    pub unsafe fn GetIconLocation(&self, pbs: *mut windows_core::BSTR) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIconLocation)(windows_core::Interface::as_raw(self), core::mem::transmute(pbs), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIconLocation(&self, bs: &windows_core::BSTR, iicon: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIconLocation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bs), iicon) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Save(&self, vwhere: &super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(vwhere)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShellLinkDual_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Arguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetArguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Hotkey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetHotkey: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ShowCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetShowCommand: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Resolve: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetIconLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetIconLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Save: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShellLinkDual_Impl: super::oaidl::IDispatch_Impl {
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPath(&self, bs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn WorkingDirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetWorkingDirectory(&self, bs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Arguments(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetArguments(&self, bs: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Hotkey(&self) -> windows_core::Result<i32>;
    fn SetHotkey(&self, ihk: i32) -> windows_core::Result<()>;
    fn ShowCommand(&self) -> windows_core::Result<i32>;
    fn SetShowCommand(&self, ishowcommand: i32) -> windows_core::Result<()>;
    fn Resolve(&self, fflags: i32) -> windows_core::Result<()>;
    fn GetIconLocation(&self, pbs: *mut windows_core::BSTR) -> windows_core::Result<i32>;
    fn SetIconLocation(&self, bs: &windows_core::BSTR, iicon: i32) -> windows_core::Result<()>;
    fn Save(&self, vwhere: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShellLinkDual_Vtbl {
    pub const fn new<Identity: IShellLinkDual_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Path<Identity: IShellLinkDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellLinkDual_Impl::Path(this) {
                    Ok(ok__) => {
                        pbs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPath<Identity: IShellLinkDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellLinkDual_Impl::SetPath(this, core::mem::transmute(&bs)).into()
            }
        }
        unsafe extern "system" fn Description<Identity: IShellLinkDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellLinkDual_Impl::Description(this) {
                    Ok(ok__) => {
                        pbs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IShellLinkDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellLinkDual_Impl::SetDescription(this, core::mem::transmute(&bs)).into()
            }
        }
        unsafe extern "system" fn WorkingDirectory<Identity: IShellLinkDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellLinkDual_Impl::WorkingDirectory(this) {
                    Ok(ok__) => {
                        pbs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWorkingDirectory<Identity: IShellLinkDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellLinkDual_Impl::SetWorkingDirectory(this, core::mem::transmute(&bs)).into()
            }
        }
        unsafe extern "system" fn Arguments<Identity: IShellLinkDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellLinkDual_Impl::Arguments(this) {
                    Ok(ok__) => {
                        pbs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetArguments<Identity: IShellLinkDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellLinkDual_Impl::SetArguments(this, core::mem::transmute(&bs)).into()
            }
        }
        unsafe extern "system" fn Hotkey<Identity: IShellLinkDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pihk: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellLinkDual_Impl::Hotkey(this) {
                    Ok(ok__) => {
                        pihk.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHotkey<Identity: IShellLinkDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ihk: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellLinkDual_Impl::SetHotkey(this, core::mem::transmute_copy(&ihk)).into()
            }
        }
        unsafe extern "system" fn ShowCommand<Identity: IShellLinkDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pishowcommand: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellLinkDual_Impl::ShowCommand(this) {
                    Ok(ok__) => {
                        pishowcommand.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetShowCommand<Identity: IShellLinkDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ishowcommand: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellLinkDual_Impl::SetShowCommand(this, core::mem::transmute_copy(&ishowcommand)).into()
            }
        }
        unsafe extern "system" fn Resolve<Identity: IShellLinkDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellLinkDual_Impl::Resolve(this, core::mem::transmute_copy(&fflags)).into()
            }
        }
        unsafe extern "system" fn GetIconLocation<Identity: IShellLinkDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbs: *mut *mut core::ffi::c_void, piicon: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellLinkDual_Impl::GetIconLocation(this, core::mem::transmute_copy(&pbs)) {
                    Ok(ok__) => {
                        piicon.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIconLocation<Identity: IShellLinkDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bs: *mut core::ffi::c_void, iicon: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellLinkDual_Impl::SetIconLocation(this, core::mem::transmute(&bs), core::mem::transmute_copy(&iicon)).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IShellLinkDual_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vwhere: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellLinkDual_Impl::Save(this, core::mem::transmute(&vwhere)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Path: Path::<Identity, OFFSET>,
            SetPath: SetPath::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            WorkingDirectory: WorkingDirectory::<Identity, OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Identity, OFFSET>,
            Arguments: Arguments::<Identity, OFFSET>,
            SetArguments: SetArguments::<Identity, OFFSET>,
            Hotkey: Hotkey::<Identity, OFFSET>,
            SetHotkey: SetHotkey::<Identity, OFFSET>,
            ShowCommand: ShowCommand::<Identity, OFFSET>,
            SetShowCommand: SetShowCommand::<Identity, OFFSET>,
            Resolve: Resolve::<Identity, OFFSET>,
            GetIconLocation: GetIconLocation::<Identity, OFFSET>,
            SetIconLocation: SetIconLocation::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellLinkDual as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShellLinkDual {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IShellLinkDual2, IShellLinkDual2_Vtbl, 0x317ee249_f12e_11d2_b1e4_00c04f8eeb3e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IShellLinkDual2 {
    type Target = IShellLinkDual;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IShellLinkDual2, windows_core::IUnknown, super::oaidl::IDispatch, IShellLinkDual);
#[cfg(feature = "oaidl")]
impl IShellLinkDual2 {
    pub unsafe fn Target(&self) -> windows_core::Result<FolderItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Target)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IShellLinkDual2_Vtbl {
    pub base__: IShellLinkDual_Vtbl,
    pub Target: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IShellLinkDual2_Impl: IShellLinkDual_Impl {
    fn Target(&self) -> windows_core::Result<FolderItem>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IShellLinkDual2_Vtbl {
    pub const fn new<Identity: IShellLinkDual2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Target<Identity: IShellLinkDual2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfi: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellLinkDual2_Impl::Target(this) {
                    Ok(ok__) => {
                        ppfi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IShellLinkDual_Vtbl::new::<Identity, OFFSET>(), Target: Target::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellLinkDual2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IShellLinkDual as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IShellLinkDual2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWebWizardHost, IWebWizardHost_Vtbl, 0x18bcc359_4990_4bfb_b951_3c83702be5f9);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWebWizardHost {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWebWizardHost, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IWebWizardHost {
    pub unsafe fn FinalBack(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FinalBack)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn FinalNext(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FinalNext)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetCaption(&self, bstrcaption: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCaption)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrcaption)) }
    }
    pub unsafe fn Caption(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Caption)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetProperty(&self, bstrpropertyname: &windows_core::BSTR, pvproperty: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpropertyname), core::mem::transmute(pvproperty)) }
    }
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Property(&self, bstrpropertyname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Property)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrpropertyname), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetWizardButtons(&self, vfenableback: super::wtypes::VARIANT_BOOL, vfenablenext: super::wtypes::VARIANT_BOOL, vflastpage: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWizardButtons)(windows_core::Interface::as_raw(self), vfenableback, vfenablenext, vflastpage) }
    }
    pub unsafe fn SetHeaderText(&self, bstrheadertitle: &windows_core::BSTR, bstrheadersubtitle: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHeaderText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrheadertitle), core::mem::transmute_copy(bstrheadersubtitle)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWebWizardHost_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub FinalBack: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FinalNext: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCaption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Caption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    SetProperty: usize,
    #[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
    pub Property: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "wtypes", feature = "wtypesbase")))]
    Property: usize,
    #[cfg(feature = "wtypes")]
    pub SetWizardButtons: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL, super::wtypes::VARIANT_BOOL, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetWizardButtons: usize,
    pub SetHeaderText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWebWizardHost_Impl: super::oaidl::IDispatch_Impl {
    fn FinalBack(&self) -> windows_core::Result<()>;
    fn FinalNext(&self) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn SetCaption(&self, bstrcaption: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Caption(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetProperty(&self, bstrpropertyname: &windows_core::BSTR, pvproperty: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn Property(&self, bstrpropertyname: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
    fn SetWizardButtons(&self, vfenableback: super::wtypes::VARIANT_BOOL, vfenablenext: super::wtypes::VARIANT_BOOL, vflastpage: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetHeaderText(&self, bstrheadertitle: &windows_core::BSTR, bstrheadersubtitle: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWebWizardHost_Vtbl {
    pub const fn new<Identity: IWebWizardHost_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FinalBack<Identity: IWebWizardHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebWizardHost_Impl::FinalBack(this).into()
            }
        }
        unsafe extern "system" fn FinalNext<Identity: IWebWizardHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebWizardHost_Impl::FinalNext(this).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IWebWizardHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebWizardHost_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn SetCaption<Identity: IWebWizardHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcaption: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebWizardHost_Impl::SetCaption(this, core::mem::transmute(&bstrcaption)).into()
            }
        }
        unsafe extern "system" fn Caption<Identity: IWebWizardHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcaption: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebWizardHost_Impl::Caption(this) {
                    Ok(ok__) => {
                        pbstrcaption.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IWebWizardHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: *mut core::ffi::c_void, pvproperty: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebWizardHost_Impl::SetProperty(this, core::mem::transmute(&bstrpropertyname), core::mem::transmute_copy(&pvproperty)).into()
            }
        }
        unsafe extern "system" fn Property<Identity: IWebWizardHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: *mut core::ffi::c_void, pvproperty: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebWizardHost_Impl::Property(this, core::mem::transmute(&bstrpropertyname)) {
                    Ok(ok__) => {
                        pvproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWizardButtons<Identity: IWebWizardHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vfenableback: super::wtypes::VARIANT_BOOL, vfenablenext: super::wtypes::VARIANT_BOOL, vflastpage: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebWizardHost_Impl::SetWizardButtons(this, core::mem::transmute_copy(&vfenableback), core::mem::transmute_copy(&vfenablenext), core::mem::transmute_copy(&vflastpage)).into()
            }
        }
        unsafe extern "system" fn SetHeaderText<Identity: IWebWizardHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrheadertitle: *mut core::ffi::c_void, bstrheadersubtitle: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWebWizardHost_Impl::SetHeaderText(this, core::mem::transmute(&bstrheadertitle), core::mem::transmute(&bstrheadersubtitle)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            FinalBack: FinalBack::<Identity, OFFSET>,
            FinalNext: FinalNext::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            SetCaption: SetCaption::<Identity, OFFSET>,
            Caption: Caption::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            Property: Property::<Identity, OFFSET>,
            SetWizardButtons: SetWizardButtons::<Identity, OFFSET>,
            SetHeaderText: SetHeaderText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebWizardHost as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWebWizardHost {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWebWizardHost2, IWebWizardHost2_Vtbl, 0xf9c013dc_3c23_4041_8e39_cfb402f7ea59);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWebWizardHost2 {
    type Target = IWebWizardHost;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWebWizardHost2, windows_core::IUnknown, super::oaidl::IDispatch, IWebWizardHost);
#[cfg(feature = "oaidl")]
impl IWebWizardHost2 {
    pub unsafe fn SignString(&self, value: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SignString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWebWizardHost2_Vtbl {
    pub base__: IWebWizardHost_Vtbl,
    pub SignString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWebWizardHost2_Impl: IWebWizardHost_Impl {
    fn SignString(&self, value: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWebWizardHost2_Vtbl {
    pub const fn new<Identity: IWebWizardHost2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SignString<Identity: IWebWizardHost2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, signedvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWebWizardHost2_Impl::SignString(this, core::mem::transmute(&value)) {
                    Ok(ok__) => {
                        signedvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IWebWizardHost_Vtbl::new::<Identity, OFFSET>(), SignString: SignString::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWebWizardHost2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IWebWizardHost as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWebWizardHost2 {}
pub const OFS_DIRTYCACHE: OfflineFolderStatus = 3;
pub const OFS_INACTIVE: OfflineFolderStatus = -1;
pub const OFS_OFFLINE: OfflineFolderStatus = 1;
pub const OFS_ONLINE: OfflineFolderStatus = 0;
pub const OFS_SERVERBACK: OfflineFolderStatus = 2;
pub type OfflineFolderStatus = i32;
pub const SFVVO_DESKTOPHTML: ShellFolderViewOptions = 512;
pub const SFVVO_DOUBLECLICKINWEBVIEW: ShellFolderViewOptions = 128;
pub const SFVVO_SHOWALLOBJECTS: ShellFolderViewOptions = 1;
pub const SFVVO_SHOWCOMPCOLOR: ShellFolderViewOptions = 8;
pub const SFVVO_SHOWEXTENSIONS: ShellFolderViewOptions = 2;
pub const SFVVO_SHOWSYSFILES: ShellFolderViewOptions = 32;
pub const SFVVO_WIN95CLASSIC: ShellFolderViewOptions = 64;
pub const Shell: windows_core::GUID = windows_core::GUID::from_u128(0x13709620_c279_11ce_a49e_444553540000);
pub const ShellDispatchInproc: windows_core::GUID = windows_core::GUID::from_u128(0x0a89a860_d7b1_11ce_8350_444553540000);
pub const ShellFolderItem: windows_core::GUID = windows_core::GUID::from_u128(0x2fe352ea_fd1f_11d2_b1f4_00c04f8eeb3e);
pub const ShellFolderView: windows_core::GUID = windows_core::GUID::from_u128(0x62112aa1_ebe4_11cf_a5fb_0020afe7292d);
pub const ShellFolderViewOC: windows_core::GUID = windows_core::GUID::from_u128(0x9ba05971_f6a8_11cf_a442_00a0c90a8f39);
pub type ShellFolderViewOptions = i32;
pub const ShellLinkObject: windows_core::GUID = windows_core::GUID::from_u128(0x11219420_1768_11d1_95be_00609797ea4f);
pub type ShellSpecialFolderConstants = i32;
pub const ssfALTSTARTUP: ShellSpecialFolderConstants = 29;
pub const ssfAPPDATA: ShellSpecialFolderConstants = 26;
pub const ssfBITBUCKET: ShellSpecialFolderConstants = 10;
pub const ssfCOMMONALTSTARTUP: ShellSpecialFolderConstants = 30;
pub const ssfCOMMONAPPDATA: ShellSpecialFolderConstants = 35;
pub const ssfCOMMONDESKTOPDIR: ShellSpecialFolderConstants = 25;
pub const ssfCOMMONFAVORITES: ShellSpecialFolderConstants = 31;
pub const ssfCOMMONPROGRAMS: ShellSpecialFolderConstants = 23;
pub const ssfCOMMONSTARTMENU: ShellSpecialFolderConstants = 22;
pub const ssfCOMMONSTARTUP: ShellSpecialFolderConstants = 24;
pub const ssfCONTROLS: ShellSpecialFolderConstants = 3;
pub const ssfCOOKIES: ShellSpecialFolderConstants = 33;
pub const ssfDESKTOP: ShellSpecialFolderConstants = 0;
pub const ssfDESKTOPDIRECTORY: ShellSpecialFolderConstants = 16;
pub const ssfDRIVES: ShellSpecialFolderConstants = 17;
pub const ssfFAVORITES: ShellSpecialFolderConstants = 6;
pub const ssfFONTS: ShellSpecialFolderConstants = 20;
pub const ssfHISTORY: ShellSpecialFolderConstants = 34;
pub const ssfINTERNETCACHE: ShellSpecialFolderConstants = 32;
pub const ssfLOCALAPPDATA: ShellSpecialFolderConstants = 28;
pub const ssfMYPICTURES: ShellSpecialFolderConstants = 39;
pub const ssfNETHOOD: ShellSpecialFolderConstants = 19;
pub const ssfNETWORK: ShellSpecialFolderConstants = 18;
pub const ssfPERSONAL: ShellSpecialFolderConstants = 5;
pub const ssfPRINTERS: ShellSpecialFolderConstants = 4;
pub const ssfPRINTHOOD: ShellSpecialFolderConstants = 27;
pub const ssfPROFILE: ShellSpecialFolderConstants = 40;
pub const ssfPROGRAMFILES: ShellSpecialFolderConstants = 38;
pub const ssfPROGRAMFILESx86: ShellSpecialFolderConstants = 48;
pub const ssfPROGRAMS: ShellSpecialFolderConstants = 2;
pub const ssfRECENT: ShellSpecialFolderConstants = 8;
pub const ssfSENDTO: ShellSpecialFolderConstants = 9;
pub const ssfSTARTMENU: ShellSpecialFolderConstants = 11;
pub const ssfSTARTUP: ShellSpecialFolderConstants = 7;
pub const ssfSYSTEM: ShellSpecialFolderConstants = 37;
pub const ssfSYSTEMx86: ShellSpecialFolderConstants = 41;
pub const ssfTEMPLATES: ShellSpecialFolderConstants = 21;
pub const ssfWINDOWS: ShellSpecialFolderConstants = 36;
