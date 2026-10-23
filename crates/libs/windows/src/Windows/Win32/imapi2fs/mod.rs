pub const BlockRange: windows_core::GUID = windows_core::GUID::from_u128(0xb507ca27_2204_11dd_966a_001aa01bbc58);
pub const BlockRangeList: windows_core::GUID = windows_core::GUID::from_u128(0xb507ca28_2204_11dd_966a_001aa01bbc58);
pub const BootOptions: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fce_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(DFileSystemImageEvents, DFileSystemImageEvents_Vtbl, 0x2c941fdf_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for DFileSystemImageEvents {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(DFileSystemImageEvents, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl DFileSystemImageEvents {
    #[cfg(feature = "shared")]
    pub unsafe fn Update<P0>(&self, object: P0, currentfile: &windows_core::BSTR, copiedsectors: super::LONG, totalsectors: super::LONG) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), core::mem::transmute_copy(currentfile), copiedsectors, totalsectors) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct DFileSystemImageEvents_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Update: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait DFileSystemImageEvents_Impl: super::IDispatch_Impl {
    fn Update(&self, object: windows_core::Ref<super::IDispatch>, currentfile: &windows_core::BSTR, copiedsectors: super::LONG, totalsectors: super::LONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl DFileSystemImageEvents_Vtbl {
    pub const fn new<Identity: DFileSystemImageEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Update<Identity: DFileSystemImageEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, currentfile: *mut core::ffi::c_void, copiedsectors: super::LONG, totalsectors: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DFileSystemImageEvents_Impl::Update(this, core::mem::transmute_copy(&object), core::mem::transmute(&currentfile), core::mem::transmute_copy(&copiedsectors), core::mem::transmute_copy(&totalsectors)).into()
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DFileSystemImageEvents as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for DFileSystemImageEvents {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(DFileSystemImageImportEvents, DFileSystemImageImportEvents_Vtbl, 0xd25c30f9_4087_4366_9e24_e55be286424b);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for DFileSystemImageImportEvents {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(DFileSystemImageImportEvents, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl DFileSystemImageImportEvents {
    #[cfg(feature = "shared")]
    pub unsafe fn UpdateImport<P0>(&self, object: P0, filesystem: FsiFileSystems, currentitem: &windows_core::BSTR, importeddirectoryitems: super::LONG, totaldirectoryitems: super::LONG, importedfileitems: super::LONG, totalfileitems: super::LONG) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateImport)(windows_core::Interface::as_raw(self), object.param().abi(), filesystem, core::mem::transmute_copy(currentitem), importeddirectoryitems, totaldirectoryitems, importedfileitems, totalfileitems) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct DFileSystemImageImportEvents_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub UpdateImport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, FsiFileSystems, *mut core::ffi::c_void, super::LONG, super::LONG, super::LONG, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    UpdateImport: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait DFileSystemImageImportEvents_Impl: super::IDispatch_Impl {
    fn UpdateImport(&self, object: windows_core::Ref<super::IDispatch>, filesystem: FsiFileSystems, currentitem: &windows_core::BSTR, importeddirectoryitems: super::LONG, totaldirectoryitems: super::LONG, importedfileitems: super::LONG, totalfileitems: super::LONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl DFileSystemImageImportEvents_Vtbl {
    pub const fn new<Identity: DFileSystemImageImportEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UpdateImport<Identity: DFileSystemImageImportEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, filesystem: FsiFileSystems, currentitem: *mut core::ffi::c_void, importeddirectoryitems: super::LONG, totaldirectoryitems: super::LONG, importedfileitems: super::LONG, totalfileitems: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DFileSystemImageImportEvents_Impl::UpdateImport(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&filesystem), core::mem::transmute(&currentitem), core::mem::transmute_copy(&importeddirectoryitems), core::mem::transmute_copy(&totaldirectoryitems), core::mem::transmute_copy(&importedfileitems), core::mem::transmute_copy(&totalfileitems)).into()
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), UpdateImport: UpdateImport::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DFileSystemImageImportEvents as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for DFileSystemImageImportEvents {}
pub const DISPID_DFILESYSTEMIMAGEEVENTS_UPDATE: i32 = 256;
pub const DISPID_DFILESYSTEMIMAGEIMPORTEVENTS_UPDATEIMPORT: i32 = 257;
pub const Emulation12MFloppy: EmulationType = 1;
pub const Emulation144MFloppy: EmulationType = 2;
pub const Emulation288MFloppy: EmulationType = 3;
pub const EmulationHardDisk: EmulationType = 4;
pub const EmulationNone: EmulationType = 0;
pub type EmulationType = i32;
pub const EnumFsiItems: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fc6_975b_59be_a960_9a2a262853a5);
pub const EnumProgressItems: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fca_975b_59be_a960_9a2a262853a5);
pub const FileSystemImageResult: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fcc_975b_59be_a960_9a2a262853a5);
pub const FsiDirectoryItem: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fc8_975b_59be_a960_9a2a262853a5);
pub const FsiFileItem: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fc7_975b_59be_a960_9a2a262853a5);
pub const FsiFileSystemISO9660: FsiFileSystems = 1;
pub const FsiFileSystemJoliet: FsiFileSystems = 2;
pub const FsiFileSystemNone: FsiFileSystems = 0;
pub const FsiFileSystemUDF: FsiFileSystems = 4;
pub const FsiFileSystemUnknown: FsiFileSystems = 1073741824;
pub type FsiFileSystems = i32;
pub const FsiItemDirectory: FsiItemType = 1;
pub const FsiItemFile: FsiItemType = 2;
pub const FsiItemNotFound: FsiItemType = 0;
pub type FsiItemType = i32;
pub const FsiNamedStreams: windows_core::GUID = windows_core::GUID::from_u128(0xc6b6f8ed_6d19_44b4_b539_b159b793a32d);
pub const FsiStream: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fcd_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IBootOptions, IBootOptions_Vtbl, 0x2c941fd4_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IBootOptions {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IBootOptions, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IBootOptions {
    #[cfg(feature = "objidlbase")]
    pub unsafe fn get_BootImage(&self) -> windows_core::Result<super::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BootImage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Manufacturer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Manufacturer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_Manufacturer(&self, newval: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Manufacturer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)) }
    }
    pub unsafe fn get_PlatformId(&self) -> windows_core::Result<PlatformId> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PlatformId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_PlatformId(&self, newval: PlatformId) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PlatformId)(windows_core::Interface::as_raw(self), newval) }
    }
    pub unsafe fn get_Emulation(&self) -> windows_core::Result<EmulationType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Emulation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_Emulation(&self, newval: EmulationType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Emulation)(windows_core::Interface::as_raw(self), newval) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_ImageSize(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ImageSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn AssignBootImage<P0>(&self, newval: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AssignBootImage)(windows_core::Interface::as_raw(self), newval.param().abi()) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IBootOptions_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "objidlbase")]
    pub get_BootImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    get_BootImage: usize,
    pub get_Manufacturer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Manufacturer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_PlatformId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PlatformId) -> windows_core::HRESULT,
    pub put_PlatformId: unsafe extern "system" fn(*mut core::ffi::c_void, PlatformId) -> windows_core::HRESULT,
    pub get_Emulation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut EmulationType) -> windows_core::HRESULT,
    pub put_Emulation: unsafe extern "system" fn(*mut core::ffi::c_void, EmulationType) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_ImageSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_ImageSize: usize,
    #[cfg(feature = "objidlbase")]
    pub AssignBootImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    AssignBootImage: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IBootOptions_Impl: super::IDispatch_Impl {
    fn get_BootImage(&self) -> windows_core::Result<super::IStream>;
    fn get_Manufacturer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_Manufacturer(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_PlatformId(&self) -> windows_core::Result<PlatformId>;
    fn put_PlatformId(&self, newval: PlatformId) -> windows_core::Result<()>;
    fn get_Emulation(&self) -> windows_core::Result<EmulationType>;
    fn put_Emulation(&self, newval: EmulationType) -> windows_core::Result<()>;
    fn get_ImageSize(&self) -> windows_core::Result<super::ULONG>;
    fn AssignBootImage(&self, newval: windows_core::Ref<super::IStream>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IBootOptions_Vtbl {
    pub const fn new<Identity: IBootOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_BootImage<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBootOptions_Impl::get_BootImage(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Manufacturer<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBootOptions_Impl::get_Manufacturer(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Manufacturer<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBootOptions_Impl::put_Manufacturer(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn get_PlatformId<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut PlatformId) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBootOptions_Impl::get_PlatformId(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PlatformId<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: PlatformId) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBootOptions_Impl::put_PlatformId(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn get_Emulation<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut EmulationType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBootOptions_Impl::get_Emulation(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Emulation<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: EmulationType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBootOptions_Impl::put_Emulation(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn get_ImageSize<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBootOptions_Impl::get_ImageSize(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AssignBootImage<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBootOptions_Impl::AssignBootImage(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_BootImage: get_BootImage::<Identity, OFFSET>,
            get_Manufacturer: get_Manufacturer::<Identity, OFFSET>,
            put_Manufacturer: put_Manufacturer::<Identity, OFFSET>,
            get_PlatformId: get_PlatformId::<Identity, OFFSET>,
            put_PlatformId: put_PlatformId::<Identity, OFFSET>,
            get_Emulation: get_Emulation::<Identity, OFFSET>,
            put_Emulation: put_Emulation::<Identity, OFFSET>,
            get_ImageSize: get_ImageSize::<Identity, OFFSET>,
            AssignBootImage: AssignBootImage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBootOptions as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IBootOptions {}
windows_core::imp::define_interface!(IEnumFsiItems, IEnumFsiItems_Vtbl, 0x2c941fda_975b_59be_a960_9a2a262853a5);
windows_core::imp::interface_hierarchy!(IEnumFsiItems, windows_core::IUnknown);
impl IEnumFsiItems {
    #[cfg(all(feature = "oaidl", feature = "shared"))]
    pub unsafe fn Next(&self, celt: super::ULONG, rgelt: *mut Option<IFsiItem>, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgelt), pceltfetched as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Skip(&self, celt: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumFsiItems_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "shared"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut *mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared")))]
    Next: usize,
    #[cfg(feature = "shared")]
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Skip: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
pub trait IEnumFsiItems_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: super::ULONG, rgelt: windows_core::OutRef<IFsiItem>, pceltfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn Skip(&self, celt: super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumFsiItems>;
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
impl IEnumFsiItems_Vtbl {
    pub const fn new<Identity: IEnumFsiItems_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumFsiItems_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumFsiItems_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumFsiItems_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumFsiItems_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
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
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumFsiItems as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
impl windows_core::RuntimeName for IEnumFsiItems {}
windows_core::imp::define_interface!(IEnumProgressItems, IEnumProgressItems_Vtbl, 0x2c941fd6_975b_59be_a960_9a2a262853a5);
windows_core::imp::interface_hierarchy!(IEnumProgressItems, windows_core::IUnknown);
impl IEnumProgressItems {
    #[cfg(all(feature = "oaidl", feature = "shared"))]
    pub unsafe fn Next(&self, celt: super::ULONG, rgelt: *mut Option<IProgressItem>, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgelt), pceltfetched as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Skip(&self, celt: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumProgressItems_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "shared"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut *mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared")))]
    Next: usize,
    #[cfg(feature = "shared")]
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Skip: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
pub trait IEnumProgressItems_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: super::ULONG, rgelt: windows_core::OutRef<IProgressItem>, pceltfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn Skip(&self, celt: super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumProgressItems>;
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
impl IEnumProgressItems_Vtbl {
    pub const fn new<Identity: IEnumProgressItems_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumProgressItems_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumProgressItems_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumProgressItems_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumProgressItems_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
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
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumProgressItems as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shared"))]
impl windows_core::RuntimeName for IEnumProgressItems {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFileSystemImage, IFileSystemImage_Vtbl, 0x2c941fe1_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFileSystemImage {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFileSystemImage, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IFileSystemImage {
    pub unsafe fn get_Root(&self) -> windows_core::Result<IFsiDirectoryItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Root)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_SessionStartBlock(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SessionStartBlock)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_SessionStartBlock(&self, newval: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_SessionStartBlock)(windows_core::Interface::as_raw(self), newval) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_FreeMediaBlocks(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FreeMediaBlocks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_FreeMediaBlocks(&self, newval: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FreeMediaBlocks)(windows_core::Interface::as_raw(self), newval) }
    }
    #[cfg(feature = "imapi2")]
    pub unsafe fn SetMaxMediaBlocksFromDevice<P0>(&self, discrecorder: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDiscRecorder2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMaxMediaBlocksFromDevice)(windows_core::Interface::as_raw(self), discrecorder.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_UsedBlocks(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UsedBlocks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_VolumeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_VolumeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_VolumeName(&self, newval: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_VolumeName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)) }
    }
    pub unsafe fn get_ImportedVolumeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ImportedVolumeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_BootImageOptions(&self) -> windows_core::Result<IBootOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BootImageOptions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_BootImageOptions<P0>(&self, newval: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IBootOptions>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_BootImageOptions)(windows_core::Interface::as_raw(self), newval.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_FileCount(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FileCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_DirectoryCount(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DirectoryCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_WorkingDirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WorkingDirectory)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_WorkingDirectory(&self, newval: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_WorkingDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_ChangePoint(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ChangePoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_StrictFileSystemCompliance(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StrictFileSystemCompliance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_StrictFileSystemCompliance(&self, newval: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_StrictFileSystemCompliance)(windows_core::Interface::as_raw(self), newval) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_UseRestrictedCharacterSet(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UseRestrictedCharacterSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_UseRestrictedCharacterSet(&self, newval: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_UseRestrictedCharacterSet)(windows_core::Interface::as_raw(self), newval) }
    }
    pub unsafe fn get_FileSystemsToCreate(&self) -> windows_core::Result<FsiFileSystems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FileSystemsToCreate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_FileSystemsToCreate(&self, newval: FsiFileSystems) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FileSystemsToCreate)(windows_core::Interface::as_raw(self), newval) }
    }
    pub unsafe fn get_FileSystemsSupported(&self) -> windows_core::Result<FsiFileSystems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FileSystemsSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_UDFRevision(&self, newval: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_UDFRevision)(windows_core::Interface::as_raw(self), newval) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_UDFRevision(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UDFRevision)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_UDFRevisionsSupported(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UDFRevisionsSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "imapi2")]
    pub unsafe fn ChooseImageDefaults<P0>(&self, discrecorder: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDiscRecorder2>,
    {
        unsafe { (windows_core::Interface::vtable(self).ChooseImageDefaults)(windows_core::Interface::as_raw(self), discrecorder.param().abi()) }
    }
    #[cfg(feature = "imapi2")]
    pub unsafe fn ChooseImageDefaultsForMediaType(&self, value: super::IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ChooseImageDefaultsForMediaType)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_ISO9660InterchangeLevel(&self, newval: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ISO9660InterchangeLevel)(windows_core::Interface::as_raw(self), newval) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_ISO9660InterchangeLevel(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ISO9660InterchangeLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_ISO9660InterchangeLevelsSupported(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ISO9660InterchangeLevelsSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateResultImage(&self) -> windows_core::Result<IFileSystemImageResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateResultImage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Exists(&self, fullpath: &windows_core::BSTR) -> windows_core::Result<FsiItemType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Exists)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(fullpath), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CalculateDiscIdentifier(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CalculateDiscIdentifier)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "imapi2")]
    pub unsafe fn IdentifyFileSystemsOnDisc<P0>(&self, discrecorder: P0) -> windows_core::Result<FsiFileSystems>
    where
        P0: windows_core::Param<super::IDiscRecorder2>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IdentifyFileSystemsOnDisc)(windows_core::Interface::as_raw(self), discrecorder.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDefaultFileSystemForImport(&self, filesystems: FsiFileSystems) -> windows_core::Result<FsiFileSystems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefaultFileSystemForImport)(windows_core::Interface::as_raw(self), filesystems, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ImportFileSystem(&self) -> windows_core::Result<FsiFileSystems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImportFileSystem)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ImportSpecificFileSystem(&self, filesystemtouse: FsiFileSystems) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ImportSpecificFileSystem)(windows_core::Interface::as_raw(self), filesystemtouse) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RollbackToChangePoint(&self, changepoint: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RollbackToChangePoint)(windows_core::Interface::as_raw(self), changepoint) }
    }
    pub unsafe fn LockInChangePoint(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockInChangePoint)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CreateDirectoryItem(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsiDirectoryItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDirectoryItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateFileItem(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsiFileItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFileItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_VolumeNameUDF(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_VolumeNameUDF)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_VolumeNameJoliet(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_VolumeNameJoliet)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_VolumeNameISO9660(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_VolumeNameISO9660)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_StageFiles(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StageFiles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_StageFiles(&self, newval: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_StageFiles)(windows_core::Interface::as_raw(self), newval) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_MultisessionInterfaces(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MultisessionInterfaces)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_MultisessionInterfaces(&self, newval: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MultisessionInterfaces)(windows_core::Interface::as_raw(self), newval) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImage_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Root: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_SessionStartBlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_SessionStartBlock: usize,
    #[cfg(feature = "shared")]
    pub put_SessionStartBlock: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_SessionStartBlock: usize,
    #[cfg(feature = "shared")]
    pub get_FreeMediaBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_FreeMediaBlocks: usize,
    #[cfg(feature = "shared")]
    pub put_FreeMediaBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_FreeMediaBlocks: usize,
    #[cfg(feature = "imapi2")]
    pub SetMaxMediaBlocksFromDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "imapi2"))]
    SetMaxMediaBlocksFromDevice: usize,
    #[cfg(feature = "shared")]
    pub get_UsedBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_UsedBlocks: usize,
    pub get_VolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_VolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ImportedVolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_BootImageOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_BootImageOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_FileCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_FileCount: usize,
    #[cfg(feature = "shared")]
    pub get_DirectoryCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_DirectoryCount: usize,
    pub get_WorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_WorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_ChangePoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_ChangePoint: usize,
    #[cfg(feature = "wtypes")]
    pub get_StrictFileSystemCompliance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_StrictFileSystemCompliance: usize,
    #[cfg(feature = "wtypes")]
    pub put_StrictFileSystemCompliance: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_StrictFileSystemCompliance: usize,
    #[cfg(feature = "wtypes")]
    pub get_UseRestrictedCharacterSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_UseRestrictedCharacterSet: usize,
    #[cfg(feature = "wtypes")]
    pub put_UseRestrictedCharacterSet: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_UseRestrictedCharacterSet: usize,
    pub get_FileSystemsToCreate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsiFileSystems) -> windows_core::HRESULT,
    pub put_FileSystemsToCreate: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems) -> windows_core::HRESULT,
    pub get_FileSystemsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsiFileSystems) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub put_UDFRevision: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_UDFRevision: usize,
    #[cfg(feature = "shared")]
    pub get_UDFRevision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_UDFRevision: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_UDFRevisionsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_UDFRevisionsSupported: usize,
    #[cfg(feature = "imapi2")]
    pub ChooseImageDefaults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "imapi2"))]
    ChooseImageDefaults: usize,
    #[cfg(feature = "imapi2")]
    pub ChooseImageDefaultsForMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, super::IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "imapi2"))]
    ChooseImageDefaultsForMediaType: usize,
    #[cfg(feature = "shared")]
    pub put_ISO9660InterchangeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_ISO9660InterchangeLevel: usize,
    #[cfg(feature = "shared")]
    pub get_ISO9660InterchangeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_ISO9660InterchangeLevel: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_ISO9660InterchangeLevelsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_ISO9660InterchangeLevelsSupported: usize,
    pub CreateResultImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Exists: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut FsiItemType) -> windows_core::HRESULT,
    pub CalculateDiscIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "imapi2")]
    pub IdentifyFileSystemsOnDisc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut FsiFileSystems) -> windows_core::HRESULT,
    #[cfg(not(feature = "imapi2"))]
    IdentifyFileSystemsOnDisc: usize,
    pub GetDefaultFileSystemForImport: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems, *mut FsiFileSystems) -> windows_core::HRESULT,
    pub ImportFileSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsiFileSystems) -> windows_core::HRESULT,
    pub ImportSpecificFileSystem: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub RollbackToChangePoint: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RollbackToChangePoint: usize,
    pub LockInChangePoint: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDirectoryItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFileItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_VolumeNameUDF: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_VolumeNameJoliet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_VolumeNameISO9660: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_StageFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_StageFiles: usize,
    #[cfg(feature = "wtypes")]
    pub put_StageFiles: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_StageFiles: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_MultisessionInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_MultisessionInterfaces: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_MultisessionInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_MultisessionInterfaces: usize,
}
#[cfg(all(feature = "guiddef", feature = "imapi2", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFileSystemImage_Impl: super::IDispatch_Impl {
    fn get_Root(&self) -> windows_core::Result<IFsiDirectoryItem>;
    fn get_SessionStartBlock(&self) -> windows_core::Result<super::LONG>;
    fn put_SessionStartBlock(&self, newval: super::LONG) -> windows_core::Result<()>;
    fn get_FreeMediaBlocks(&self) -> windows_core::Result<super::LONG>;
    fn put_FreeMediaBlocks(&self, newval: super::LONG) -> windows_core::Result<()>;
    fn SetMaxMediaBlocksFromDevice(&self, discrecorder: windows_core::Ref<super::IDiscRecorder2>) -> windows_core::Result<()>;
    fn get_UsedBlocks(&self) -> windows_core::Result<super::LONG>;
    fn get_VolumeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_VolumeName(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ImportedVolumeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_BootImageOptions(&self) -> windows_core::Result<IBootOptions>;
    fn put_BootImageOptions(&self, newval: windows_core::Ref<IBootOptions>) -> windows_core::Result<()>;
    fn get_FileCount(&self) -> windows_core::Result<super::LONG>;
    fn get_DirectoryCount(&self) -> windows_core::Result<super::LONG>;
    fn get_WorkingDirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_WorkingDirectory(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ChangePoint(&self) -> windows_core::Result<super::LONG>;
    fn get_StrictFileSystemCompliance(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_StrictFileSystemCompliance(&self, newval: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_UseRestrictedCharacterSet(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_UseRestrictedCharacterSet(&self, newval: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_FileSystemsToCreate(&self) -> windows_core::Result<FsiFileSystems>;
    fn put_FileSystemsToCreate(&self, newval: FsiFileSystems) -> windows_core::Result<()>;
    fn get_FileSystemsSupported(&self) -> windows_core::Result<FsiFileSystems>;
    fn put_UDFRevision(&self, newval: super::LONG) -> windows_core::Result<()>;
    fn get_UDFRevision(&self) -> windows_core::Result<super::LONG>;
    fn get_UDFRevisionsSupported(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn ChooseImageDefaults(&self, discrecorder: windows_core::Ref<super::IDiscRecorder2>) -> windows_core::Result<()>;
    fn ChooseImageDefaultsForMediaType(&self, value: super::IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::Result<()>;
    fn put_ISO9660InterchangeLevel(&self, newval: super::LONG) -> windows_core::Result<()>;
    fn get_ISO9660InterchangeLevel(&self) -> windows_core::Result<super::LONG>;
    fn get_ISO9660InterchangeLevelsSupported(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn CreateResultImage(&self) -> windows_core::Result<IFileSystemImageResult>;
    fn Exists(&self, fullpath: &windows_core::BSTR) -> windows_core::Result<FsiItemType>;
    fn CalculateDiscIdentifier(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IdentifyFileSystemsOnDisc(&self, discrecorder: windows_core::Ref<super::IDiscRecorder2>) -> windows_core::Result<FsiFileSystems>;
    fn GetDefaultFileSystemForImport(&self, filesystems: FsiFileSystems) -> windows_core::Result<FsiFileSystems>;
    fn ImportFileSystem(&self) -> windows_core::Result<FsiFileSystems>;
    fn ImportSpecificFileSystem(&self, filesystemtouse: FsiFileSystems) -> windows_core::Result<()>;
    fn RollbackToChangePoint(&self, changepoint: super::LONG) -> windows_core::Result<()>;
    fn LockInChangePoint(&self) -> windows_core::Result<()>;
    fn CreateDirectoryItem(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsiDirectoryItem>;
    fn CreateFileItem(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsiFileItem>;
    fn get_VolumeNameUDF(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_VolumeNameJoliet(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_VolumeNameISO9660(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_StageFiles(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_StageFiles(&self, newval: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_MultisessionInterfaces(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_MultisessionInterfaces(&self, newval: *const super::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "imapi2", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFileSystemImage_Vtbl {
    pub const fn new<Identity: IFileSystemImage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Root<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_Root(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SessionStartBlock<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_SessionStartBlock(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_SessionStartBlock<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::put_SessionStartBlock(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn get_FreeMediaBlocks<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_FreeMediaBlocks(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FreeMediaBlocks<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::put_FreeMediaBlocks(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn SetMaxMediaBlocksFromDevice<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discrecorder: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetMaxMediaBlocksFromDevice(this, core::mem::transmute_copy(&discrecorder)).into()
            }
        }
        unsafe extern "system" fn get_UsedBlocks<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_UsedBlocks(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_VolumeName<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_VolumeName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_VolumeName<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::put_VolumeName(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn get_ImportedVolumeName<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_ImportedVolumeName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_BootImageOptions<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_BootImageOptions(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_BootImageOptions<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::put_BootImageOptions(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn get_FileCount<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_FileCount(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DirectoryCount<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_DirectoryCount(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_WorkingDirectory<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_WorkingDirectory(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_WorkingDirectory<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::put_WorkingDirectory(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn get_ChangePoint<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_ChangePoint(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_StrictFileSystemCompliance<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_StrictFileSystemCompliance(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StrictFileSystemCompliance<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::put_StrictFileSystemCompliance(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn get_UseRestrictedCharacterSet<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_UseRestrictedCharacterSet(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_UseRestrictedCharacterSet<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::put_UseRestrictedCharacterSet(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn get_FileSystemsToCreate<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut FsiFileSystems) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_FileSystemsToCreate(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FileSystemsToCreate<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: FsiFileSystems) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::put_FileSystemsToCreate(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn get_FileSystemsSupported<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut FsiFileSystems) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_FileSystemsSupported(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_UDFRevision<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::put_UDFRevision(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn get_UDFRevision<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_UDFRevision(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_UDFRevisionsSupported<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_UDFRevisionsSupported(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ChooseImageDefaults<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discrecorder: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::ChooseImageDefaults(this, core::mem::transmute_copy(&discrecorder)).into()
            }
        }
        unsafe extern "system" fn ChooseImageDefaultsForMediaType<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::ChooseImageDefaultsForMediaType(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn put_ISO9660InterchangeLevel<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::put_ISO9660InterchangeLevel(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn get_ISO9660InterchangeLevel<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_ISO9660InterchangeLevel(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ISO9660InterchangeLevelsSupported<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_ISO9660InterchangeLevelsSupported(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateResultImage<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resultstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::CreateResultImage(this) {
                    Ok(ok__) => {
                        resultstream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Exists<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fullpath: *mut core::ffi::c_void, itemtype: *mut FsiItemType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::Exists(this, core::mem::transmute(&fullpath)) {
                    Ok(ok__) => {
                        itemtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CalculateDiscIdentifier<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discidentifier: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::CalculateDiscIdentifier(this) {
                    Ok(ok__) => {
                        discidentifier.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IdentifyFileSystemsOnDisc<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discrecorder: *mut core::ffi::c_void, filesystems: *mut FsiFileSystems) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::IdentifyFileSystemsOnDisc(this, core::mem::transmute_copy(&discrecorder)) {
                    Ok(ok__) => {
                        filesystems.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDefaultFileSystemForImport<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesystems: FsiFileSystems, importdefault: *mut FsiFileSystems) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::GetDefaultFileSystemForImport(this, core::mem::transmute_copy(&filesystems)) {
                    Ok(ok__) => {
                        importdefault.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ImportFileSystem<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, importedfilesystem: *mut FsiFileSystems) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::ImportFileSystem(this) {
                    Ok(ok__) => {
                        importedfilesystem.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ImportSpecificFileSystem<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesystemtouse: FsiFileSystems) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::ImportSpecificFileSystem(this, core::mem::transmute_copy(&filesystemtouse)).into()
            }
        }
        unsafe extern "system" fn RollbackToChangePoint<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, changepoint: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::RollbackToChangePoint(this, core::mem::transmute_copy(&changepoint)).into()
            }
        }
        unsafe extern "system" fn LockInChangePoint<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::LockInChangePoint(this).into()
            }
        }
        unsafe extern "system" fn CreateDirectoryItem<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, newitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::CreateDirectoryItem(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        newitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFileItem<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, newitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::CreateFileItem(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        newitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_VolumeNameUDF<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_VolumeNameUDF(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_VolumeNameJoliet<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_VolumeNameJoliet(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_VolumeNameISO9660<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_VolumeNameISO9660(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_StageFiles<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_StageFiles(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StageFiles<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::put_StageFiles(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn get_MultisessionInterfaces<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::get_MultisessionInterfaces(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MultisessionInterfaces<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::put_MultisessionInterfaces(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Root: get_Root::<Identity, OFFSET>,
            get_SessionStartBlock: get_SessionStartBlock::<Identity, OFFSET>,
            put_SessionStartBlock: put_SessionStartBlock::<Identity, OFFSET>,
            get_FreeMediaBlocks: get_FreeMediaBlocks::<Identity, OFFSET>,
            put_FreeMediaBlocks: put_FreeMediaBlocks::<Identity, OFFSET>,
            SetMaxMediaBlocksFromDevice: SetMaxMediaBlocksFromDevice::<Identity, OFFSET>,
            get_UsedBlocks: get_UsedBlocks::<Identity, OFFSET>,
            get_VolumeName: get_VolumeName::<Identity, OFFSET>,
            put_VolumeName: put_VolumeName::<Identity, OFFSET>,
            get_ImportedVolumeName: get_ImportedVolumeName::<Identity, OFFSET>,
            get_BootImageOptions: get_BootImageOptions::<Identity, OFFSET>,
            put_BootImageOptions: put_BootImageOptions::<Identity, OFFSET>,
            get_FileCount: get_FileCount::<Identity, OFFSET>,
            get_DirectoryCount: get_DirectoryCount::<Identity, OFFSET>,
            get_WorkingDirectory: get_WorkingDirectory::<Identity, OFFSET>,
            put_WorkingDirectory: put_WorkingDirectory::<Identity, OFFSET>,
            get_ChangePoint: get_ChangePoint::<Identity, OFFSET>,
            get_StrictFileSystemCompliance: get_StrictFileSystemCompliance::<Identity, OFFSET>,
            put_StrictFileSystemCompliance: put_StrictFileSystemCompliance::<Identity, OFFSET>,
            get_UseRestrictedCharacterSet: get_UseRestrictedCharacterSet::<Identity, OFFSET>,
            put_UseRestrictedCharacterSet: put_UseRestrictedCharacterSet::<Identity, OFFSET>,
            get_FileSystemsToCreate: get_FileSystemsToCreate::<Identity, OFFSET>,
            put_FileSystemsToCreate: put_FileSystemsToCreate::<Identity, OFFSET>,
            get_FileSystemsSupported: get_FileSystemsSupported::<Identity, OFFSET>,
            put_UDFRevision: put_UDFRevision::<Identity, OFFSET>,
            get_UDFRevision: get_UDFRevision::<Identity, OFFSET>,
            get_UDFRevisionsSupported: get_UDFRevisionsSupported::<Identity, OFFSET>,
            ChooseImageDefaults: ChooseImageDefaults::<Identity, OFFSET>,
            ChooseImageDefaultsForMediaType: ChooseImageDefaultsForMediaType::<Identity, OFFSET>,
            put_ISO9660InterchangeLevel: put_ISO9660InterchangeLevel::<Identity, OFFSET>,
            get_ISO9660InterchangeLevel: get_ISO9660InterchangeLevel::<Identity, OFFSET>,
            get_ISO9660InterchangeLevelsSupported: get_ISO9660InterchangeLevelsSupported::<Identity, OFFSET>,
            CreateResultImage: CreateResultImage::<Identity, OFFSET>,
            Exists: Exists::<Identity, OFFSET>,
            CalculateDiscIdentifier: CalculateDiscIdentifier::<Identity, OFFSET>,
            IdentifyFileSystemsOnDisc: IdentifyFileSystemsOnDisc::<Identity, OFFSET>,
            GetDefaultFileSystemForImport: GetDefaultFileSystemForImport::<Identity, OFFSET>,
            ImportFileSystem: ImportFileSystem::<Identity, OFFSET>,
            ImportSpecificFileSystem: ImportSpecificFileSystem::<Identity, OFFSET>,
            RollbackToChangePoint: RollbackToChangePoint::<Identity, OFFSET>,
            LockInChangePoint: LockInChangePoint::<Identity, OFFSET>,
            CreateDirectoryItem: CreateDirectoryItem::<Identity, OFFSET>,
            CreateFileItem: CreateFileItem::<Identity, OFFSET>,
            get_VolumeNameUDF: get_VolumeNameUDF::<Identity, OFFSET>,
            get_VolumeNameJoliet: get_VolumeNameJoliet::<Identity, OFFSET>,
            get_VolumeNameISO9660: get_VolumeNameISO9660::<Identity, OFFSET>,
            get_StageFiles: get_StageFiles::<Identity, OFFSET>,
            put_StageFiles: put_StageFiles::<Identity, OFFSET>,
            get_MultisessionInterfaces: get_MultisessionInterfaces::<Identity, OFFSET>,
            put_MultisessionInterfaces: put_MultisessionInterfaces::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSystemImage as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "imapi2", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFileSystemImage {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFileSystemImage2, IFileSystemImage2_Vtbl, 0xd7644b2c_1537_4767_b62f_f1387b02ddfd);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFileSystemImage2 {
    type Target = IFileSystemImage;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFileSystemImage2, windows_core::IUnknown, super::IDispatch, IFileSystemImage);
#[cfg(feature = "oaidl")]
impl IFileSystemImage2 {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_BootImageOptionsArray(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BootImageOptionsArray)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn put_BootImageOptionsArray(&self, newval: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_BootImageOptionsArray)(windows_core::Interface::as_raw(self), newval) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImage2_Vtbl {
    pub base__: IFileSystemImage_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_BootImageOptionsArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_BootImageOptionsArray: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub put_BootImageOptionsArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    put_BootImageOptionsArray: usize,
}
#[cfg(all(feature = "guiddef", feature = "imapi2", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFileSystemImage2_Impl: IFileSystemImage_Impl {
    fn get_BootImageOptionsArray(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_BootImageOptionsArray(&self, newval: *const super::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "imapi2", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFileSystemImage2_Vtbl {
    pub const fn new<Identity: IFileSystemImage2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_BootImageOptionsArray<Identity: IFileSystemImage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage2_Impl::get_BootImageOptionsArray(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_BootImageOptionsArray<Identity: IFileSystemImage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage2_Impl::put_BootImageOptionsArray(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        Self {
            base__: IFileSystemImage_Vtbl::new::<Identity, OFFSET>(),
            get_BootImageOptionsArray: get_BootImageOptionsArray::<Identity, OFFSET>,
            put_BootImageOptionsArray: put_BootImageOptionsArray::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSystemImage2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IFileSystemImage as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "imapi2", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFileSystemImage2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFileSystemImage3, IFileSystemImage3_Vtbl, 0x7cff842c_7e97_4807_8304_910dd8f7c051);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFileSystemImage3 {
    type Target = IFileSystemImage2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFileSystemImage3, windows_core::IUnknown, super::IDispatch, IFileSystemImage, IFileSystemImage2);
#[cfg(feature = "oaidl")]
impl IFileSystemImage3 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_CreateRedundantUdfMetadataFiles(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CreateRedundantUdfMetadataFiles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_CreateRedundantUdfMetadataFiles(&self, newval: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_CreateRedundantUdfMetadataFiles)(windows_core::Interface::as_raw(self), newval) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn ProbeSpecificFileSystem(&self, filesystemtoprobe: FsiFileSystems) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProbeSpecificFileSystem)(windows_core::Interface::as_raw(self), filesystemtoprobe, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImage3_Vtbl {
    pub base__: IFileSystemImage2_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_CreateRedundantUdfMetadataFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_CreateRedundantUdfMetadataFiles: usize,
    #[cfg(feature = "wtypes")]
    pub put_CreateRedundantUdfMetadataFiles: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_CreateRedundantUdfMetadataFiles: usize,
    #[cfg(feature = "wtypes")]
    pub ProbeSpecificFileSystem: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    ProbeSpecificFileSystem: usize,
}
#[cfg(all(feature = "guiddef", feature = "imapi2", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFileSystemImage3_Impl: IFileSystemImage2_Impl {
    fn get_CreateRedundantUdfMetadataFiles(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_CreateRedundantUdfMetadataFiles(&self, newval: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ProbeSpecificFileSystem(&self, filesystemtoprobe: FsiFileSystems) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "imapi2", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFileSystemImage3_Vtbl {
    pub const fn new<Identity: IFileSystemImage3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CreateRedundantUdfMetadataFiles<Identity: IFileSystemImage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage3_Impl::get_CreateRedundantUdfMetadataFiles(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_CreateRedundantUdfMetadataFiles<Identity: IFileSystemImage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage3_Impl::put_CreateRedundantUdfMetadataFiles(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn ProbeSpecificFileSystem<Identity: IFileSystemImage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesystemtoprobe: FsiFileSystems, isappendable: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage3_Impl::ProbeSpecificFileSystem(this, core::mem::transmute_copy(&filesystemtoprobe)) {
                    Ok(ok__) => {
                        isappendable.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IFileSystemImage2_Vtbl::new::<Identity, OFFSET>(),
            get_CreateRedundantUdfMetadataFiles: get_CreateRedundantUdfMetadataFiles::<Identity, OFFSET>,
            put_CreateRedundantUdfMetadataFiles: put_CreateRedundantUdfMetadataFiles::<Identity, OFFSET>,
            ProbeSpecificFileSystem: ProbeSpecificFileSystem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSystemImage3 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IFileSystemImage as windows_core::Interface>::IID || iid == &<IFileSystemImage2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "imapi2", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFileSystemImage3 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFileSystemImageResult, IFileSystemImageResult_Vtbl, 0x2c941fd8_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFileSystemImageResult {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFileSystemImageResult, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IFileSystemImageResult {
    #[cfg(feature = "objidlbase")]
    pub unsafe fn get_ImageStream(&self) -> windows_core::Result<super::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ImageStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_ProgressItems(&self) -> windows_core::Result<IProgressItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ProgressItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_TotalBlocks(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TotalBlocks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_BlockSize(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BlockSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_DiscId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DiscId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImageResult_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "objidlbase")]
    pub get_ImageStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    get_ImageStream: usize,
    pub get_ProgressItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_TotalBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_TotalBlocks: usize,
    #[cfg(feature = "shared")]
    pub get_BlockSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_BlockSize: usize,
    pub get_DiscId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFileSystemImageResult_Impl: super::IDispatch_Impl {
    fn get_ImageStream(&self) -> windows_core::Result<super::IStream>;
    fn get_ProgressItems(&self) -> windows_core::Result<IProgressItems>;
    fn get_TotalBlocks(&self) -> windows_core::Result<super::LONG>;
    fn get_BlockSize(&self) -> windows_core::Result<super::LONG>;
    fn get_DiscId(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFileSystemImageResult_Vtbl {
    pub const fn new<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ImageStream<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImageResult_Impl::get_ImageStream(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ProgressItems<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImageResult_Impl::get_ProgressItems(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_TotalBlocks<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImageResult_Impl::get_TotalBlocks(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_BlockSize<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImageResult_Impl::get_BlockSize(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DiscId<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImageResult_Impl::get_DiscId(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ImageStream: get_ImageStream::<Identity, OFFSET>,
            get_ProgressItems: get_ProgressItems::<Identity, OFFSET>,
            get_TotalBlocks: get_TotalBlocks::<Identity, OFFSET>,
            get_BlockSize: get_BlockSize::<Identity, OFFSET>,
            get_DiscId: get_DiscId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSystemImageResult as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFileSystemImageResult {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFileSystemImageResult2, IFileSystemImageResult2_Vtbl, 0xb507ca29_2204_11dd_966a_001aa01bbc58);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFileSystemImageResult2 {
    type Target = IFileSystemImageResult;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFileSystemImageResult2, windows_core::IUnknown, super::IDispatch, IFileSystemImageResult);
#[cfg(feature = "oaidl")]
impl IFileSystemImageResult2 {
    #[cfg(feature = "imapi2")]
    pub unsafe fn get_ModifiedBlocks(&self) -> windows_core::Result<super::IBlockRangeList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ModifiedBlocks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImageResult2_Vtbl {
    pub base__: IFileSystemImageResult_Vtbl,
    #[cfg(feature = "imapi2")]
    pub get_ModifiedBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "imapi2"))]
    get_ModifiedBlocks: usize,
}
#[cfg(all(feature = "guiddef", feature = "imapi2", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFileSystemImageResult2_Impl: IFileSystemImageResult_Impl {
    fn get_ModifiedBlocks(&self) -> windows_core::Result<super::IBlockRangeList>;
}
#[cfg(all(feature = "guiddef", feature = "imapi2", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFileSystemImageResult2_Vtbl {
    pub const fn new<Identity: IFileSystemImageResult2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ModifiedBlocks<Identity: IFileSystemImageResult2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImageResult2_Impl::get_ModifiedBlocks(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IFileSystemImageResult_Vtbl::new::<Identity, OFFSET>(), get_ModifiedBlocks: get_ModifiedBlocks::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSystemImageResult2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IFileSystemImageResult as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "imapi2", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFileSystemImageResult2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsiDirectoryItem, IFsiDirectoryItem_Vtbl, 0x2c941fdc_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsiDirectoryItem {
    type Target = IFsiItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsiDirectoryItem, windows_core::IUnknown, super::IDispatch, IFsiItem);
#[cfg(feature = "oaidl")]
impl IFsiDirectoryItem {
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<super::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Item(&self, path: &windows_core::BSTR) -> windows_core::Result<IFsiItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Count(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_EnumFsiItems(&self) -> windows_core::Result<IEnumFsiItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EnumFsiItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddDirectory(&self, path: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path)) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn AddFile<P1>(&self, path: &windows_core::BSTR, filedata: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddFile)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), filedata.param().abi()) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn AddTree(&self, sourcedirectory: &windows_core::BSTR, includebasedirectory: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddTree)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sourcedirectory), includebasedirectory) }
    }
    pub unsafe fn Add<P0>(&self, item: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IFsiItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), item.param().abi()) }
    }
    pub unsafe fn Remove(&self, path: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path)) }
    }
    pub unsafe fn RemoveTree(&self, path: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveTree)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiDirectoryItem_Vtbl {
    pub base__: IFsiItem_Vtbl,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Count: usize,
    pub get_EnumFsiItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub AddFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    AddFile: usize,
    #[cfg(feature = "wtypes")]
    pub AddTree: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AddTree: usize,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveTree: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsiDirectoryItem_Impl: IFsiItem_Impl {
    fn get__NewEnum(&self) -> windows_core::Result<super::IEnumVARIANT>;
    fn get_Item(&self, path: &windows_core::BSTR) -> windows_core::Result<IFsiItem>;
    fn get_Count(&self) -> windows_core::Result<super::LONG>;
    fn get_EnumFsiItems(&self) -> windows_core::Result<IEnumFsiItems>;
    fn AddDirectory(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddFile(&self, path: &windows_core::BSTR, filedata: windows_core::Ref<super::IStream>) -> windows_core::Result<()>;
    fn AddTree(&self, sourcedirectory: &windows_core::BSTR, includebasedirectory: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Add(&self, item: windows_core::Ref<IFsiItem>) -> windows_core::Result<()>;
    fn Remove(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoveTree(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsiDirectoryItem_Vtbl {
    pub const fn new<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get__NewEnum<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiDirectoryItem_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiDirectoryItem_Impl::get_Item(this, core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Count<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiDirectoryItem_Impl::get_Count(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_EnumFsiItems<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiDirectoryItem_Impl::get_EnumFsiItems(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddDirectory<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiDirectoryItem_Impl::AddDirectory(this, core::mem::transmute(&path)).into()
            }
        }
        unsafe extern "system" fn AddFile<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, filedata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiDirectoryItem_Impl::AddFile(this, core::mem::transmute(&path), core::mem::transmute_copy(&filedata)).into()
            }
        }
        unsafe extern "system" fn AddTree<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcedirectory: *mut core::ffi::c_void, includebasedirectory: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiDirectoryItem_Impl::AddTree(this, core::mem::transmute(&sourcedirectory), core::mem::transmute_copy(&includebasedirectory)).into()
            }
        }
        unsafe extern "system" fn Add<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiDirectoryItem_Impl::Add(this, core::mem::transmute_copy(&item)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiDirectoryItem_Impl::Remove(this, core::mem::transmute(&path)).into()
            }
        }
        unsafe extern "system" fn RemoveTree<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiDirectoryItem_Impl::RemoveTree(this, core::mem::transmute(&path)).into()
            }
        }
        Self {
            base__: IFsiItem_Vtbl::new::<Identity, OFFSET>(),
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            get_Count: get_Count::<Identity, OFFSET>,
            get_EnumFsiItems: get_EnumFsiItems::<Identity, OFFSET>,
            AddDirectory: AddDirectory::<Identity, OFFSET>,
            AddFile: AddFile::<Identity, OFFSET>,
            AddTree: AddTree::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            RemoveTree: RemoveTree::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiDirectoryItem as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IFsiItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsiDirectoryItem {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsiDirectoryItem2, IFsiDirectoryItem2_Vtbl, 0xf7fb4b9b_6d96_4d7b_9115_201b144811ef);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsiDirectoryItem2 {
    type Target = IFsiDirectoryItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsiDirectoryItem2, windows_core::IUnknown, super::IDispatch, IFsiItem, IFsiDirectoryItem);
#[cfg(feature = "oaidl")]
impl IFsiDirectoryItem2 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn AddTreeWithNamedStreams(&self, sourcedirectory: &windows_core::BSTR, includebasedirectory: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddTreeWithNamedStreams)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sourcedirectory), includebasedirectory) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiDirectoryItem2_Vtbl {
    pub base__: IFsiDirectoryItem_Vtbl,
    #[cfg(feature = "wtypes")]
    pub AddTreeWithNamedStreams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AddTreeWithNamedStreams: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsiDirectoryItem2_Impl: IFsiDirectoryItem_Impl {
    fn AddTreeWithNamedStreams(&self, sourcedirectory: &windows_core::BSTR, includebasedirectory: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsiDirectoryItem2_Vtbl {
    pub const fn new<Identity: IFsiDirectoryItem2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddTreeWithNamedStreams<Identity: IFsiDirectoryItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcedirectory: *mut core::ffi::c_void, includebasedirectory: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiDirectoryItem2_Impl::AddTreeWithNamedStreams(this, core::mem::transmute(&sourcedirectory), core::mem::transmute_copy(&includebasedirectory)).into()
            }
        }
        Self { base__: IFsiDirectoryItem_Vtbl::new::<Identity, OFFSET>(), AddTreeWithNamedStreams: AddTreeWithNamedStreams::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiDirectoryItem2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IFsiItem as windows_core::Interface>::IID || iid == &<IFsiDirectoryItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsiDirectoryItem2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsiFileItem, IFsiFileItem_Vtbl, 0x2c941fdb_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsiFileItem {
    type Target = IFsiItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsiFileItem, windows_core::IUnknown, super::IDispatch, IFsiItem);
#[cfg(feature = "oaidl")]
impl IFsiFileItem {
    #[cfg(feature = "shared")]
    pub unsafe fn get_DataSize(&self) -> windows_core::Result<super::LONGLONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DataSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_DataSize32BitLow(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DataSize32BitLow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_DataSize32BitHigh(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DataSize32BitHigh)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn get_Data(&self) -> windows_core::Result<super::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Data)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn put_Data<P0>(&self, newval: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_Data)(windows_core::Interface::as_raw(self), newval.param().abi()) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiFileItem_Vtbl {
    pub base__: IFsiItem_Vtbl,
    #[cfg(feature = "shared")]
    pub get_DataSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONGLONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_DataSize: usize,
    #[cfg(feature = "shared")]
    pub get_DataSize32BitLow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_DataSize32BitLow: usize,
    #[cfg(feature = "shared")]
    pub get_DataSize32BitHigh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_DataSize32BitHigh: usize,
    #[cfg(feature = "objidlbase")]
    pub get_Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    get_Data: usize,
    #[cfg(feature = "objidlbase")]
    pub put_Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    put_Data: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsiFileItem_Impl: IFsiItem_Impl {
    fn get_DataSize(&self) -> windows_core::Result<super::LONGLONG>;
    fn get_DataSize32BitLow(&self) -> windows_core::Result<super::LONG>;
    fn get_DataSize32BitHigh(&self) -> windows_core::Result<super::LONG>;
    fn get_Data(&self) -> windows_core::Result<super::IStream>;
    fn put_Data(&self, newval: windows_core::Ref<super::IStream>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsiFileItem_Vtbl {
    pub const fn new<Identity: IFsiFileItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_DataSize<Identity: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::LONGLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiFileItem_Impl::get_DataSize(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DataSize32BitLow<Identity: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiFileItem_Impl::get_DataSize32BitLow(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DataSize32BitHigh<Identity: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiFileItem_Impl::get_DataSize32BitHigh(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Data<Identity: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiFileItem_Impl::get_Data(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_Data<Identity: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiFileItem_Impl::put_Data(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        Self {
            base__: IFsiItem_Vtbl::new::<Identity, OFFSET>(),
            get_DataSize: get_DataSize::<Identity, OFFSET>,
            get_DataSize32BitLow: get_DataSize32BitLow::<Identity, OFFSET>,
            get_DataSize32BitHigh: get_DataSize32BitHigh::<Identity, OFFSET>,
            get_Data: get_Data::<Identity, OFFSET>,
            put_Data: put_Data::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiFileItem as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IFsiItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsiFileItem {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsiFileItem2, IFsiFileItem2_Vtbl, 0x199d0c19_11e1_40eb_8ec2_c8c822a07792);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsiFileItem2 {
    type Target = IFsiFileItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsiFileItem2, windows_core::IUnknown, super::IDispatch, IFsiItem, IFsiFileItem);
#[cfg(feature = "oaidl")]
impl IFsiFileItem2 {
    pub unsafe fn get_FsiNamedStreams(&self) -> windows_core::Result<IFsiNamedStreams> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FsiNamedStreams)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsNamedStream(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsNamedStream)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn AddStream<P1>(&self, name: &windows_core::BSTR, streamdata: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddStream)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), streamdata.param().abi()) }
    }
    pub unsafe fn RemoveStream(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveStream)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsRealTime(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsRealTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_IsRealTime(&self, newval: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_IsRealTime)(windows_core::Interface::as_raw(self), newval) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiFileItem2_Vtbl {
    pub base__: IFsiFileItem_Vtbl,
    pub get_FsiNamedStreams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_IsNamedStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsNamedStream: usize,
    #[cfg(feature = "objidlbase")]
    pub AddStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    AddStream: usize,
    pub RemoveStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_IsRealTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsRealTime: usize,
    #[cfg(feature = "wtypes")]
    pub put_IsRealTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_IsRealTime: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsiFileItem2_Impl: IFsiFileItem_Impl {
    fn get_FsiNamedStreams(&self) -> windows_core::Result<IFsiNamedStreams>;
    fn get_IsNamedStream(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn AddStream(&self, name: &windows_core::BSTR, streamdata: windows_core::Ref<super::IStream>) -> windows_core::Result<()>;
    fn RemoveStream(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_IsRealTime(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_IsRealTime(&self, newval: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsiFileItem2_Vtbl {
    pub const fn new<Identity: IFsiFileItem2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_FsiNamedStreams<Identity: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streams: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiFileItem2_Impl::get_FsiNamedStreams(this) {
                    Ok(ok__) => {
                        streams.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsNamedStream<Identity: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiFileItem2_Impl::get_IsNamedStream(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddStream<Identity: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, streamdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiFileItem2_Impl::AddStream(this, core::mem::transmute(&name), core::mem::transmute_copy(&streamdata)).into()
            }
        }
        unsafe extern "system" fn RemoveStream<Identity: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiFileItem2_Impl::RemoveStream(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn get_IsRealTime<Identity: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiFileItem2_Impl::get_IsRealTime(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_IsRealTime<Identity: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiFileItem2_Impl::put_IsRealTime(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        Self {
            base__: IFsiFileItem_Vtbl::new::<Identity, OFFSET>(),
            get_FsiNamedStreams: get_FsiNamedStreams::<Identity, OFFSET>,
            get_IsNamedStream: get_IsNamedStream::<Identity, OFFSET>,
            AddStream: AddStream::<Identity, OFFSET>,
            RemoveStream: RemoveStream::<Identity, OFFSET>,
            get_IsRealTime: get_IsRealTime::<Identity, OFFSET>,
            put_IsRealTime: put_IsRealTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiFileItem2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IFsiItem as windows_core::Interface>::IID || iid == &<IFsiFileItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsiFileItem2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsiItem, IFsiItem_Vtbl, 0x2c941fd9_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsiItem {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsiItem, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsiItem {
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_FullPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FullPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_CreationTime(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CreationTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_CreationTime(&self, newval: super::DATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_CreationTime)(windows_core::Interface::as_raw(self), newval) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_LastAccessedTime(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastAccessedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_LastAccessedTime(&self, newval: super::DATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LastAccessedTime)(windows_core::Interface::as_raw(self), newval) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_LastModifiedTime(&self) -> windows_core::Result<super::DATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastModifiedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_LastModifiedTime(&self, newval: super::DATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_LastModifiedTime)(windows_core::Interface::as_raw(self), newval) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsHidden(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsHidden)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_IsHidden(&self, newval: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_IsHidden)(windows_core::Interface::as_raw(self), newval) }
    }
    pub unsafe fn FileSystemName(&self, filesystem: FsiFileSystems) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileSystemName)(windows_core::Interface::as_raw(self), filesystem, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn FileSystemPath(&self, filesystem: FsiFileSystems) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileSystemPath)(windows_core::Interface::as_raw(self), filesystem, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiItem_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_FullPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_CreationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_CreationTime: usize,
    #[cfg(feature = "wtypes")]
    pub put_CreationTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_CreationTime: usize,
    #[cfg(feature = "wtypes")]
    pub get_LastAccessedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_LastAccessedTime: usize,
    #[cfg(feature = "wtypes")]
    pub put_LastAccessedTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_LastAccessedTime: usize,
    #[cfg(feature = "wtypes")]
    pub get_LastModifiedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_LastModifiedTime: usize,
    #[cfg(feature = "wtypes")]
    pub put_LastModifiedTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::DATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_LastModifiedTime: usize,
    #[cfg(feature = "wtypes")]
    pub get_IsHidden: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsHidden: usize,
    #[cfg(feature = "wtypes")]
    pub put_IsHidden: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_IsHidden: usize,
    pub FileSystemName: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FileSystemPath: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsiItem_Impl: super::IDispatch_Impl {
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_FullPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CreationTime(&self) -> windows_core::Result<super::DATE>;
    fn put_CreationTime(&self, newval: super::DATE) -> windows_core::Result<()>;
    fn get_LastAccessedTime(&self) -> windows_core::Result<super::DATE>;
    fn put_LastAccessedTime(&self, newval: super::DATE) -> windows_core::Result<()>;
    fn get_LastModifiedTime(&self) -> windows_core::Result<super::DATE>;
    fn put_LastModifiedTime(&self, newval: super::DATE) -> windows_core::Result<()>;
    fn get_IsHidden(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_IsHidden(&self, newval: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn FileSystemName(&self, filesystem: FsiFileSystems) -> windows_core::Result<windows_core::BSTR>;
    fn FileSystemPath(&self, filesystem: FsiFileSystems) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsiItem_Vtbl {
    pub const fn new<Identity: IFsiItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Name<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiItem_Impl::get_Name(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_FullPath<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiItem_Impl::get_FullPath(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CreationTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiItem_Impl::get_CreationTime(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_CreationTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiItem_Impl::put_CreationTime(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn get_LastAccessedTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiItem_Impl::get_LastAccessedTime(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LastAccessedTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiItem_Impl::put_LastAccessedTime(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn get_LastModifiedTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiItem_Impl::get_LastModifiedTime(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_LastModifiedTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::DATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiItem_Impl::put_LastModifiedTime(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn get_IsHidden<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiItem_Impl::get_IsHidden(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_IsHidden<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiItem_Impl::put_IsHidden(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn FileSystemName<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesystem: FsiFileSystems, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiItem_Impl::FileSystemName(this, core::mem::transmute_copy(&filesystem)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FileSystemPath<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesystem: FsiFileSystems, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiItem_Impl::FileSystemPath(this, core::mem::transmute_copy(&filesystem)) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Name: get_Name::<Identity, OFFSET>,
            get_FullPath: get_FullPath::<Identity, OFFSET>,
            get_CreationTime: get_CreationTime::<Identity, OFFSET>,
            put_CreationTime: put_CreationTime::<Identity, OFFSET>,
            get_LastAccessedTime: get_LastAccessedTime::<Identity, OFFSET>,
            put_LastAccessedTime: put_LastAccessedTime::<Identity, OFFSET>,
            get_LastModifiedTime: get_LastModifiedTime::<Identity, OFFSET>,
            put_LastModifiedTime: put_LastModifiedTime::<Identity, OFFSET>,
            get_IsHidden: get_IsHidden::<Identity, OFFSET>,
            put_IsHidden: put_IsHidden::<Identity, OFFSET>,
            FileSystemName: FileSystemName::<Identity, OFFSET>,
            FileSystemPath: FileSystemPath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiItem as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsiItem {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IFsiNamedStreams, IFsiNamedStreams_Vtbl, 0xed79ba56_5294_4250_8d46_f9aecee23459);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IFsiNamedStreams {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IFsiNamedStreams, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IFsiNamedStreams {
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<super::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Item(&self, index: super::LONG) -> windows_core::Result<IFsiFileItem2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Count(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_EnumNamedStreams(&self) -> windows_core::Result<IEnumFsiItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EnumNamedStreams)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiNamedStreams_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Item: usize,
    #[cfg(feature = "shared")]
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Count: usize,
    pub get_EnumNamedStreams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFsiNamedStreams_Impl: super::IDispatch_Impl {
    fn get__NewEnum(&self) -> windows_core::Result<super::IEnumVARIANT>;
    fn get_Item(&self, index: super::LONG) -> windows_core::Result<IFsiFileItem2>;
    fn get_Count(&self) -> windows_core::Result<super::LONG>;
    fn get_EnumNamedStreams(&self) -> windows_core::Result<IEnumFsiItems>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFsiNamedStreams_Vtbl {
    pub const fn new<Identity: IFsiNamedStreams_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get__NewEnum<Identity: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiNamedStreams_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::LONG, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiNamedStreams_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Count<Identity: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiNamedStreams_Impl::get_Count(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_EnumNamedStreams<Identity: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiNamedStreams_Impl::get_EnumNamedStreams(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            get_Count: get_Count::<Identity, OFFSET>,
            get_EnumNamedStreams: get_EnumNamedStreams::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiNamedStreams as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFsiNamedStreams {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IIsoImageManager, IIsoImageManager_Vtbl, 0x6ca38be5_fbbb_4800_95a1_a438865eb0d4);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IIsoImageManager {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IIsoImageManager, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IIsoImageManager {
    pub unsafe fn get_Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn get_Stream(&self) -> windows_core::Result<super::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Stream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPath(&self, val: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(val)) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn SetStream<P0>(&self, data: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStream)(windows_core::Interface::as_raw(self), data.param().abi()) }
    }
    pub unsafe fn Validate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Validate)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IIsoImageManager_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub get_Stream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    get_Stream: usize,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub SetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    SetStream: usize,
    pub Validate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IIsoImageManager_Impl: super::IDispatch_Impl {
    fn get_Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Stream(&self) -> windows_core::Result<super::IStream>;
    fn SetPath(&self, val: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetStream(&self, data: windows_core::Ref<super::IStream>) -> windows_core::Result<()>;
    fn Validate(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IIsoImageManager_Vtbl {
    pub const fn new<Identity: IIsoImageManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Path<Identity: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIsoImageManager_Impl::get_Path(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Stream<Identity: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIsoImageManager_Impl::get_Stream(this) {
                    Ok(ok__) => {
                        data.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPath<Identity: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIsoImageManager_Impl::SetPath(this, core::mem::transmute(&val)).into()
            }
        }
        unsafe extern "system" fn SetStream<Identity: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIsoImageManager_Impl::SetStream(this, core::mem::transmute_copy(&data)).into()
            }
        }
        unsafe extern "system" fn Validate<Identity: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IIsoImageManager_Impl::Validate(this).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Path: get_Path::<Identity, OFFSET>,
            get_Stream: get_Stream::<Identity, OFFSET>,
            SetPath: SetPath::<Identity, OFFSET>,
            SetStream: SetStream::<Identity, OFFSET>,
            Validate: Validate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIsoImageManager as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IIsoImageManager {}
pub const IMAPI2FS_BOOT_ENTRY_COUNT_MAX: i32 = 32;
pub const IMAPI2FS_FullVersion_STR: windows_core::PCSTR = windows_core::s!("1.0");
pub const IMAPI2FS_FullVersion_WSTR: windows_core::PCWSTR = windows_core::w!("1.0");
pub const IMAPI2FS_MajorVersion: i32 = 1;
pub const IMAPI2FS_MinorVersion: i32 = 0;
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IProgressItem, IProgressItem_Vtbl, 0x2c941fd5_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IProgressItem {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IProgressItem, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IProgressItem {
    pub unsafe fn get_Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_FirstBlock(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FirstBlock)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_LastBlock(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastBlock)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_BlockCount(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BlockCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IProgressItem_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_FirstBlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_FirstBlock: usize,
    #[cfg(feature = "shared")]
    pub get_LastBlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_LastBlock: usize,
    #[cfg(feature = "shared")]
    pub get_BlockCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_BlockCount: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IProgressItem_Impl: super::IDispatch_Impl {
    fn get_Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_FirstBlock(&self) -> windows_core::Result<super::ULONG>;
    fn get_LastBlock(&self) -> windows_core::Result<super::ULONG>;
    fn get_BlockCount(&self) -> windows_core::Result<super::ULONG>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IProgressItem_Vtbl {
    pub const fn new<Identity: IProgressItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Description<Identity: IProgressItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItem_Impl::get_Description(this) {
                    Ok(ok__) => {
                        desc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_FirstBlock<Identity: IProgressItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, block: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItem_Impl::get_FirstBlock(this) {
                    Ok(ok__) => {
                        block.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LastBlock<Identity: IProgressItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, block: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItem_Impl::get_LastBlock(this) {
                    Ok(ok__) => {
                        block.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_BlockCount<Identity: IProgressItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blocks: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItem_Impl::get_BlockCount(this) {
                    Ok(ok__) => {
                        blocks.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_Description: get_Description::<Identity, OFFSET>,
            get_FirstBlock: get_FirstBlock::<Identity, OFFSET>,
            get_LastBlock: get_LastBlock::<Identity, OFFSET>,
            get_BlockCount: get_BlockCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProgressItem as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IProgressItem {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IProgressItems, IProgressItems_Vtbl, 0x2c941fd7_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IProgressItems {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IProgressItems, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IProgressItems {
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<super::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Item(&self, index: i32) -> windows_core::Result<IProgressItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ProgressItemFromBlock(&self, block: super::ULONG) -> windows_core::Result<IProgressItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProgressItemFromBlock)(windows_core::Interface::as_raw(self), block, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn ProgressItemFromDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<IProgressItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProgressItemFromDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(description), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_EnumProgressItems(&self) -> windows_core::Result<IEnumProgressItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EnumProgressItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IProgressItems_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get__NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub ProgressItemFromBlock: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ProgressItemFromBlock: usize,
    pub ProgressItemFromDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_EnumProgressItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IProgressItems_Impl: super::IDispatch_Impl {
    fn get__NewEnum(&self) -> windows_core::Result<super::IEnumVARIANT>;
    fn get_Item(&self, index: i32) -> windows_core::Result<IProgressItem>;
    fn get_Count(&self) -> windows_core::Result<i32>;
    fn ProgressItemFromBlock(&self, block: super::ULONG) -> windows_core::Result<IProgressItem>;
    fn ProgressItemFromDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<IProgressItem>;
    fn get_EnumProgressItems(&self) -> windows_core::Result<IEnumProgressItems>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IProgressItems_Vtbl {
    pub const fn new<Identity: IProgressItems_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get__NewEnum<Identity: IProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItems_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItems_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Count<Identity: IProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItems_Impl::get_Count(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProgressItemFromBlock<Identity: IProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, block: super::ULONG, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItems_Impl::ProgressItemFromBlock(this, core::mem::transmute_copy(&block)) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProgressItemFromDescription<Identity: IProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut core::ffi::c_void, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItems_Impl::ProgressItemFromDescription(this, core::mem::transmute(&description)) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_EnumProgressItems<Identity: IProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItems_Impl::get_EnumProgressItems(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get__NewEnum: get__NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            get_Count: get_Count::<Identity, OFFSET>,
            ProgressItemFromBlock: ProgressItemFromBlock::<Identity, OFFSET>,
            ProgressItemFromDescription: ProgressItemFromDescription::<Identity, OFFSET>,
            get_EnumProgressItems: get_EnumProgressItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProgressItems as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IProgressItems {}
pub const MsftFileSystemImage: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fc5_975b_59be_a960_9a2a262853a5);
pub const MsftIsoImageManager: windows_core::GUID = windows_core::GUID::from_u128(0xceee3b62_8f56_4056_869b_ef16917e3efc);
pub const PlatformEFI: PlatformId = 239;
pub type PlatformId = i32;
pub const PlatformMac: PlatformId = 2;
pub const PlatformPowerPC: PlatformId = 1;
pub const PlatformX86: PlatformId = 0;
pub const ProgressItem: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fcb_975b_59be_a960_9a2a262853a5);
pub const ProgressItems: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fc9_975b_59be_a960_9a2a262853a5);
