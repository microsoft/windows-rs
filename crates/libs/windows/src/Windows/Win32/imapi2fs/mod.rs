pub const BlockRange: windows_core::GUID = windows_core::GUID::from_u128(0xb507ca27_2204_11dd_966a_001aa01bbc58);
pub const BlockRangeList: windows_core::GUID = windows_core::GUID::from_u128(0xb507ca28_2204_11dd_966a_001aa01bbc58);
pub const BootOptions: windows_core::GUID = windows_core::GUID::from_u128(0x2c941fce_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(DFileSystemImageEvents, DFileSystemImageEvents_Vtbl, 0x2c941fdf_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for DFileSystemImageEvents {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(DFileSystemImageEvents, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl DFileSystemImageEvents {
    pub unsafe fn Update<P0>(&self, object: P0, currentfile: &windows_core::BSTR, copiedsectors: i32, totalsectors: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oaidl::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), core::mem::transmute_copy(currentfile), copiedsectors, totalsectors) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct DFileSystemImageEvents_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait DFileSystemImageEvents_Impl: super::oaidl::IDispatch_Impl {
    fn Update(&self, object: windows_core::Ref<super::oaidl::IDispatch>, currentfile: &windows_core::BSTR, copiedsectors: i32, totalsectors: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl DFileSystemImageEvents_Vtbl {
    pub const fn new<Identity: DFileSystemImageEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Update<Identity: DFileSystemImageEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, currentfile: *mut core::ffi::c_void, copiedsectors: i32, totalsectors: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DFileSystemImageEvents_Impl::Update(this, core::mem::transmute_copy(&object), core::mem::transmute(&currentfile), core::mem::transmute_copy(&copiedsectors), core::mem::transmute_copy(&totalsectors)).into()
            }
        }
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DFileSystemImageEvents as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for DFileSystemImageEvents {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(DFileSystemImageImportEvents, DFileSystemImageImportEvents_Vtbl, 0xd25c30f9_4087_4366_9e24_e55be286424b);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for DFileSystemImageImportEvents {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(DFileSystemImageImportEvents, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl DFileSystemImageImportEvents {
    pub unsafe fn UpdateImport<P0>(&self, object: P0, filesystem: FsiFileSystems, currentitem: &windows_core::BSTR, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oaidl::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateImport)(windows_core::Interface::as_raw(self), object.param().abi(), filesystem, core::mem::transmute_copy(currentitem), importeddirectoryitems, totaldirectoryitems, importedfileitems, totalfileitems) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct DFileSystemImageImportEvents_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub UpdateImport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, FsiFileSystems, *mut core::ffi::c_void, i32, i32, i32, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait DFileSystemImageImportEvents_Impl: super::oaidl::IDispatch_Impl {
    fn UpdateImport(&self, object: windows_core::Ref<super::oaidl::IDispatch>, filesystem: FsiFileSystems, currentitem: &windows_core::BSTR, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl DFileSystemImageImportEvents_Vtbl {
    pub const fn new<Identity: DFileSystemImageImportEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UpdateImport<Identity: DFileSystemImageImportEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, filesystem: FsiFileSystems, currentitem: *mut core::ffi::c_void, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DFileSystemImageImportEvents_Impl::UpdateImport(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&filesystem), core::mem::transmute(&currentitem), core::mem::transmute_copy(&importeddirectoryitems), core::mem::transmute_copy(&totaldirectoryitems), core::mem::transmute_copy(&importedfileitems), core::mem::transmute_copy(&totalfileitems)).into()
            }
        }
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(), UpdateImport: UpdateImport::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DFileSystemImageImportEvents as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for DFileSystemImageImportEvents {}
pub const DISPID_DFILESYSTEMIMAGEEVENTS_UPDATE: u32 = 256;
pub const DISPID_DFILESYSTEMIMAGEIMPORTEVENTS_UPDATEIMPORT: u32 = 257;
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
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IBootOptions, IBootOptions_Vtbl, 0x2c941fd4_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IBootOptions {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IBootOptions, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IBootOptions {
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn BootImage(&self) -> windows_core::Result<super::objidlbase::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BootImage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Manufacturer(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Manufacturer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetManufacturer(&self, newval: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetManufacturer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)) }
    }
    pub unsafe fn PlatformId(&self) -> windows_core::Result<PlatformId> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PlatformId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPlatformId(&self, newval: PlatformId) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPlatformId)(windows_core::Interface::as_raw(self), newval) }
    }
    pub unsafe fn Emulation(&self) -> windows_core::Result<EmulationType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Emulation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEmulation(&self, newval: EmulationType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEmulation)(windows_core::Interface::as_raw(self), newval) }
    }
    pub unsafe fn ImageSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImageSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn AssignBootImage<P0>(&self, newval: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AssignBootImage)(windows_core::Interface::as_raw(self), newval.param().abi()) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IBootOptions_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(feature = "Win32_objidlbase")]
    pub BootImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    BootImage: usize,
    pub Manufacturer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetManufacturer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PlatformId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PlatformId) -> windows_core::HRESULT,
    pub SetPlatformId: unsafe extern "system" fn(*mut core::ffi::c_void, PlatformId) -> windows_core::HRESULT,
    pub Emulation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut EmulationType) -> windows_core::HRESULT,
    pub SetEmulation: unsafe extern "system" fn(*mut core::ffi::c_void, EmulationType) -> windows_core::HRESULT,
    pub ImageSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidlbase")]
    pub AssignBootImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    AssignBootImage: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IBootOptions_Impl: super::oaidl::IDispatch_Impl {
    fn BootImage(&self) -> windows_core::Result<super::objidlbase::IStream>;
    fn Manufacturer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetManufacturer(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PlatformId(&self) -> windows_core::Result<PlatformId>;
    fn SetPlatformId(&self, newval: PlatformId) -> windows_core::Result<()>;
    fn Emulation(&self) -> windows_core::Result<EmulationType>;
    fn SetEmulation(&self, newval: EmulationType) -> windows_core::Result<()>;
    fn ImageSize(&self) -> windows_core::Result<u32>;
    fn AssignBootImage(&self, newval: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IBootOptions_Vtbl {
    pub const fn new<Identity: IBootOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BootImage<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBootOptions_Impl::BootImage(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Manufacturer<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBootOptions_Impl::Manufacturer(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetManufacturer<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBootOptions_Impl::SetManufacturer(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn PlatformId<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut PlatformId) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBootOptions_Impl::PlatformId(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPlatformId<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: PlatformId) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBootOptions_Impl::SetPlatformId(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn Emulation<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut EmulationType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBootOptions_Impl::Emulation(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEmulation<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: EmulationType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBootOptions_Impl::SetEmulation(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn ImageSize<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBootOptions_Impl::ImageSize(this) {
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
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            BootImage: BootImage::<Identity, OFFSET>,
            Manufacturer: Manufacturer::<Identity, OFFSET>,
            SetManufacturer: SetManufacturer::<Identity, OFFSET>,
            PlatformId: PlatformId::<Identity, OFFSET>,
            SetPlatformId: SetPlatformId::<Identity, OFFSET>,
            Emulation: Emulation::<Identity, OFFSET>,
            SetEmulation: SetEmulation::<Identity, OFFSET>,
            ImageSize: ImageSize::<Identity, OFFSET>,
            AssignBootImage: AssignBootImage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBootOptions as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IBootOptions {}
windows_core::imp::define_interface!(IEnumFsiItems, IEnumFsiItems_Vtbl, 0x2c941fda_975b_59be_a960_9a2a262853a5);
windows_core::imp::interface_hierarchy!(IEnumFsiItems, windows_core::IUnknown);
impl IEnumFsiItems {
    #[cfg(feature = "Win32_oaidl")]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut Option<IFsiItem>, pceltfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgelt), pceltfetched as _) }
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
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumFsiItems_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_oaidl")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_oaidl"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_oaidl")]
pub trait IEnumFsiItems_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: windows_core::OutRef<IFsiItem>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumFsiItems>;
}
#[cfg(feature = "Win32_oaidl")]
impl IEnumFsiItems_Vtbl {
    pub const fn new<Identity: IEnumFsiItems_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumFsiItems_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
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
#[cfg(feature = "Win32_oaidl")]
impl windows_core::RuntimeName for IEnumFsiItems {}
windows_core::imp::define_interface!(IEnumProgressItems, IEnumProgressItems_Vtbl, 0x2c941fd6_975b_59be_a960_9a2a262853a5);
windows_core::imp::interface_hierarchy!(IEnumProgressItems, windows_core::IUnknown);
impl IEnumProgressItems {
    #[cfg(feature = "Win32_oaidl")]
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut Option<IProgressItem>, pceltfetched: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, core::mem::transmute(rgelt), pceltfetched as _) }
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
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumProgressItems_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_oaidl")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_oaidl"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_oaidl")]
pub trait IEnumProgressItems_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: windows_core::OutRef<IProgressItem>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumProgressItems>;
}
#[cfg(feature = "Win32_oaidl")]
impl IEnumProgressItems_Vtbl {
    pub const fn new<Identity: IEnumProgressItems_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumProgressItems_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
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
#[cfg(feature = "Win32_oaidl")]
impl windows_core::RuntimeName for IEnumProgressItems {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IFileSystemImage, IFileSystemImage_Vtbl, 0x2c941fe1_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IFileSystemImage {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IFileSystemImage, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IFileSystemImage {
    pub unsafe fn Root(&self) -> windows_core::Result<IFsiDirectoryItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Root)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SessionStartBlock(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SessionStartBlock)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSessionStartBlock(&self, newval: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSessionStartBlock)(windows_core::Interface::as_raw(self), newval) }
    }
    pub unsafe fn FreeMediaBlocks(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FreeMediaBlocks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFreeMediaBlocks(&self, newval: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFreeMediaBlocks)(windows_core::Interface::as_raw(self), newval) }
    }
    #[cfg(feature = "Win32_imapi2")]
    pub unsafe fn SetMaxMediaBlocksFromDevice<P0>(&self, discrecorder: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::imapi2::IDiscRecorder2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMaxMediaBlocksFromDevice)(windows_core::Interface::as_raw(self), discrecorder.param().abi()) }
    }
    pub unsafe fn UsedBlocks(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UsedBlocks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn VolumeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VolumeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetVolumeName(&self, newval: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVolumeName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)) }
    }
    pub unsafe fn ImportedVolumeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImportedVolumeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn BootImageOptions(&self) -> windows_core::Result<IBootOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BootImageOptions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetBootImageOptions<P0>(&self, newval: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IBootOptions>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBootImageOptions)(windows_core::Interface::as_raw(self), newval.param().abi()) }
    }
    pub unsafe fn FileCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DirectoryCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DirectoryCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn WorkingDirectory(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WorkingDirectory)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetWorkingDirectory(&self, newval: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWorkingDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(newval)) }
    }
    pub unsafe fn ChangePoint(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ChangePoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn StrictFileSystemCompliance(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StrictFileSystemCompliance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetStrictFileSystemCompliance(&self, newval: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStrictFileSystemCompliance)(windows_core::Interface::as_raw(self), newval) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn UseRestrictedCharacterSet(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UseRestrictedCharacterSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetUseRestrictedCharacterSet(&self, newval: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUseRestrictedCharacterSet)(windows_core::Interface::as_raw(self), newval) }
    }
    pub unsafe fn FileSystemsToCreate(&self) -> windows_core::Result<FsiFileSystems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileSystemsToCreate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFileSystemsToCreate(&self, newval: FsiFileSystems) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFileSystemsToCreate)(windows_core::Interface::as_raw(self), newval) }
    }
    pub unsafe fn FileSystemsSupported(&self) -> windows_core::Result<FsiFileSystems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileSystemsSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUDFRevision(&self, newval: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUDFRevision)(windows_core::Interface::as_raw(self), newval) }
    }
    pub unsafe fn UDFRevision(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UDFRevision)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UDFRevisionsSupported(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UDFRevisionsSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_imapi2")]
    pub unsafe fn ChooseImageDefaults<P0>(&self, discrecorder: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::imapi2::IDiscRecorder2>,
    {
        unsafe { (windows_core::Interface::vtable(self).ChooseImageDefaults)(windows_core::Interface::as_raw(self), discrecorder.param().abi()) }
    }
    #[cfg(feature = "Win32_imapi2")]
    pub unsafe fn ChooseImageDefaultsForMediaType(&self, value: super::imapi2::IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ChooseImageDefaultsForMediaType)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn SetISO9660InterchangeLevel(&self, newval: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetISO9660InterchangeLevel)(windows_core::Interface::as_raw(self), newval) }
    }
    pub unsafe fn ISO9660InterchangeLevel(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ISO9660InterchangeLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ISO9660InterchangeLevelsSupported(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ISO9660InterchangeLevelsSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateResultImage(&self) -> windows_core::Result<IFileSystemImageResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateResultImage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    #[cfg(feature = "Win32_imapi2")]
    pub unsafe fn IdentifyFileSystemsOnDisc<P0>(&self, discrecorder: P0) -> windows_core::Result<FsiFileSystems>
    where
        P0: windows_core::Param<super::imapi2::IDiscRecorder2>,
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
    pub unsafe fn RollbackToChangePoint(&self, changepoint: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RollbackToChangePoint)(windows_core::Interface::as_raw(self), changepoint) }
    }
    pub unsafe fn LockInChangePoint(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockInChangePoint)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CreateDirectoryItem(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsiDirectoryItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDirectoryItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateFileItem(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsiFileItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFileItem)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn VolumeNameUDF(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VolumeNameUDF)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn VolumeNameJoliet(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VolumeNameJoliet)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn VolumeNameISO9660(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VolumeNameISO9660)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn StageFiles(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StageFiles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetStageFiles(&self, newval: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStageFiles)(windows_core::Interface::as_raw(self), newval) }
    }
    pub unsafe fn MultisessionInterfaces(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MultisessionInterfaces)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMultisessionInterfaces(&self, newval: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMultisessionInterfaces)(windows_core::Interface::as_raw(self), newval) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImage_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Root: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SessionStartBlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSessionStartBlock: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub FreeMediaBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetFreeMediaBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_imapi2")]
    pub SetMaxMediaBlocksFromDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_imapi2"))]
    SetMaxMediaBlocksFromDevice: usize,
    pub UsedBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub VolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetVolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImportedVolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BootImageOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBootImageOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FileCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DirectoryCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub WorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWorkingDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ChangePoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub StrictFileSystemCompliance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    StrictFileSystemCompliance: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetStrictFileSystemCompliance: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetStrictFileSystemCompliance: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub UseRestrictedCharacterSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    UseRestrictedCharacterSet: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetUseRestrictedCharacterSet: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetUseRestrictedCharacterSet: usize,
    pub FileSystemsToCreate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsiFileSystems) -> windows_core::HRESULT,
    pub SetFileSystemsToCreate: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems) -> windows_core::HRESULT,
    pub FileSystemsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsiFileSystems) -> windows_core::HRESULT,
    pub SetUDFRevision: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub UDFRevision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub UDFRevisionsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_imapi2")]
    pub ChooseImageDefaults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_imapi2"))]
    ChooseImageDefaults: usize,
    #[cfg(feature = "Win32_imapi2")]
    pub ChooseImageDefaultsForMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, super::imapi2::IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_imapi2"))]
    ChooseImageDefaultsForMediaType: usize,
    pub SetISO9660InterchangeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ISO9660InterchangeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ISO9660InterchangeLevelsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub CreateResultImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Exists: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut FsiItemType) -> windows_core::HRESULT,
    pub CalculateDiscIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_imapi2")]
    pub IdentifyFileSystemsOnDisc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut FsiFileSystems) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_imapi2"))]
    IdentifyFileSystemsOnDisc: usize,
    pub GetDefaultFileSystemForImport: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems, *mut FsiFileSystems) -> windows_core::HRESULT,
    pub ImportFileSystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FsiFileSystems) -> windows_core::HRESULT,
    pub ImportSpecificFileSystem: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems) -> windows_core::HRESULT,
    pub RollbackToChangePoint: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub LockInChangePoint: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDirectoryItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFileItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VolumeNameUDF: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VolumeNameJoliet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VolumeNameISO9660: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub StageFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    StageFiles: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetStageFiles: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetStageFiles: usize,
    pub MultisessionInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub SetMultisessionInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_imapi2", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IFileSystemImage_Impl: super::oaidl::IDispatch_Impl {
    fn Root(&self) -> windows_core::Result<IFsiDirectoryItem>;
    fn SessionStartBlock(&self) -> windows_core::Result<i32>;
    fn SetSessionStartBlock(&self, newval: i32) -> windows_core::Result<()>;
    fn FreeMediaBlocks(&self) -> windows_core::Result<i32>;
    fn SetFreeMediaBlocks(&self, newval: i32) -> windows_core::Result<()>;
    fn SetMaxMediaBlocksFromDevice(&self, discrecorder: windows_core::Ref<super::imapi2::IDiscRecorder2>) -> windows_core::Result<()>;
    fn UsedBlocks(&self) -> windows_core::Result<i32>;
    fn VolumeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetVolumeName(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ImportedVolumeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn BootImageOptions(&self) -> windows_core::Result<IBootOptions>;
    fn SetBootImageOptions(&self, newval: windows_core::Ref<IBootOptions>) -> windows_core::Result<()>;
    fn FileCount(&self) -> windows_core::Result<i32>;
    fn DirectoryCount(&self) -> windows_core::Result<i32>;
    fn WorkingDirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetWorkingDirectory(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ChangePoint(&self) -> windows_core::Result<i32>;
    fn StrictFileSystemCompliance(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetStrictFileSystemCompliance(&self, newval: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UseRestrictedCharacterSet(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetUseRestrictedCharacterSet(&self, newval: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn FileSystemsToCreate(&self) -> windows_core::Result<FsiFileSystems>;
    fn SetFileSystemsToCreate(&self, newval: FsiFileSystems) -> windows_core::Result<()>;
    fn FileSystemsSupported(&self) -> windows_core::Result<FsiFileSystems>;
    fn SetUDFRevision(&self, newval: i32) -> windows_core::Result<()>;
    fn UDFRevision(&self) -> windows_core::Result<i32>;
    fn UDFRevisionsSupported(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn ChooseImageDefaults(&self, discrecorder: windows_core::Ref<super::imapi2::IDiscRecorder2>) -> windows_core::Result<()>;
    fn ChooseImageDefaultsForMediaType(&self, value: super::imapi2::IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::Result<()>;
    fn SetISO9660InterchangeLevel(&self, newval: i32) -> windows_core::Result<()>;
    fn ISO9660InterchangeLevel(&self) -> windows_core::Result<i32>;
    fn ISO9660InterchangeLevelsSupported(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn CreateResultImage(&self) -> windows_core::Result<IFileSystemImageResult>;
    fn Exists(&self, fullpath: &windows_core::BSTR) -> windows_core::Result<FsiItemType>;
    fn CalculateDiscIdentifier(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IdentifyFileSystemsOnDisc(&self, discrecorder: windows_core::Ref<super::imapi2::IDiscRecorder2>) -> windows_core::Result<FsiFileSystems>;
    fn GetDefaultFileSystemForImport(&self, filesystems: FsiFileSystems) -> windows_core::Result<FsiFileSystems>;
    fn ImportFileSystem(&self) -> windows_core::Result<FsiFileSystems>;
    fn ImportSpecificFileSystem(&self, filesystemtouse: FsiFileSystems) -> windows_core::Result<()>;
    fn RollbackToChangePoint(&self, changepoint: i32) -> windows_core::Result<()>;
    fn LockInChangePoint(&self) -> windows_core::Result<()>;
    fn CreateDirectoryItem(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsiDirectoryItem>;
    fn CreateFileItem(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsiFileItem>;
    fn VolumeNameUDF(&self) -> windows_core::Result<windows_core::BSTR>;
    fn VolumeNameJoliet(&self) -> windows_core::Result<windows_core::BSTR>;
    fn VolumeNameISO9660(&self) -> windows_core::Result<windows_core::BSTR>;
    fn StageFiles(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetStageFiles(&self, newval: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn MultisessionInterfaces(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn SetMultisessionInterfaces(&self, newval: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_imapi2", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IFileSystemImage_Vtbl {
    pub const fn new<Identity: IFileSystemImage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Root<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::Root(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SessionStartBlock<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::SessionStartBlock(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSessionStartBlock<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetSessionStartBlock(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn FreeMediaBlocks<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::FreeMediaBlocks(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFreeMediaBlocks<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetFreeMediaBlocks(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn SetMaxMediaBlocksFromDevice<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discrecorder: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetMaxMediaBlocksFromDevice(this, core::mem::transmute_copy(&discrecorder)).into()
            }
        }
        unsafe extern "system" fn UsedBlocks<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::UsedBlocks(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VolumeName<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::VolumeName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVolumeName<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetVolumeName(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn ImportedVolumeName<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::ImportedVolumeName(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BootImageOptions<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::BootImageOptions(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBootImageOptions<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetBootImageOptions(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn FileCount<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::FileCount(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DirectoryCount<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::DirectoryCount(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WorkingDirectory<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::WorkingDirectory(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWorkingDirectory<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetWorkingDirectory(this, core::mem::transmute(&newval)).into()
            }
        }
        unsafe extern "system" fn ChangePoint<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::ChangePoint(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StrictFileSystemCompliance<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::StrictFileSystemCompliance(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStrictFileSystemCompliance<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetStrictFileSystemCompliance(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn UseRestrictedCharacterSet<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::UseRestrictedCharacterSet(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUseRestrictedCharacterSet<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetUseRestrictedCharacterSet(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn FileSystemsToCreate<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut FsiFileSystems) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::FileSystemsToCreate(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFileSystemsToCreate<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: FsiFileSystems) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetFileSystemsToCreate(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn FileSystemsSupported<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut FsiFileSystems) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::FileSystemsSupported(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUDFRevision<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetUDFRevision(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn UDFRevision<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::UDFRevision(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UDFRevisionsSupported<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::UDFRevisionsSupported(this) {
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
        unsafe extern "system" fn ChooseImageDefaultsForMediaType<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::imapi2::IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::ChooseImageDefaultsForMediaType(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SetISO9660InterchangeLevel<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetISO9660InterchangeLevel(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn ISO9660InterchangeLevel<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::ISO9660InterchangeLevel(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ISO9660InterchangeLevelsSupported<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::ISO9660InterchangeLevelsSupported(this) {
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
        unsafe extern "system" fn RollbackToChangePoint<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, changepoint: i32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn VolumeNameUDF<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::VolumeNameUDF(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VolumeNameJoliet<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::VolumeNameJoliet(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VolumeNameISO9660<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::VolumeNameISO9660(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StageFiles<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::StageFiles(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStageFiles<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetStageFiles(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn MultisessionInterfaces<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage_Impl::MultisessionInterfaces(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMultisessionInterfaces<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage_Impl::SetMultisessionInterfaces(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Root: Root::<Identity, OFFSET>,
            SessionStartBlock: SessionStartBlock::<Identity, OFFSET>,
            SetSessionStartBlock: SetSessionStartBlock::<Identity, OFFSET>,
            FreeMediaBlocks: FreeMediaBlocks::<Identity, OFFSET>,
            SetFreeMediaBlocks: SetFreeMediaBlocks::<Identity, OFFSET>,
            SetMaxMediaBlocksFromDevice: SetMaxMediaBlocksFromDevice::<Identity, OFFSET>,
            UsedBlocks: UsedBlocks::<Identity, OFFSET>,
            VolumeName: VolumeName::<Identity, OFFSET>,
            SetVolumeName: SetVolumeName::<Identity, OFFSET>,
            ImportedVolumeName: ImportedVolumeName::<Identity, OFFSET>,
            BootImageOptions: BootImageOptions::<Identity, OFFSET>,
            SetBootImageOptions: SetBootImageOptions::<Identity, OFFSET>,
            FileCount: FileCount::<Identity, OFFSET>,
            DirectoryCount: DirectoryCount::<Identity, OFFSET>,
            WorkingDirectory: WorkingDirectory::<Identity, OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Identity, OFFSET>,
            ChangePoint: ChangePoint::<Identity, OFFSET>,
            StrictFileSystemCompliance: StrictFileSystemCompliance::<Identity, OFFSET>,
            SetStrictFileSystemCompliance: SetStrictFileSystemCompliance::<Identity, OFFSET>,
            UseRestrictedCharacterSet: UseRestrictedCharacterSet::<Identity, OFFSET>,
            SetUseRestrictedCharacterSet: SetUseRestrictedCharacterSet::<Identity, OFFSET>,
            FileSystemsToCreate: FileSystemsToCreate::<Identity, OFFSET>,
            SetFileSystemsToCreate: SetFileSystemsToCreate::<Identity, OFFSET>,
            FileSystemsSupported: FileSystemsSupported::<Identity, OFFSET>,
            SetUDFRevision: SetUDFRevision::<Identity, OFFSET>,
            UDFRevision: UDFRevision::<Identity, OFFSET>,
            UDFRevisionsSupported: UDFRevisionsSupported::<Identity, OFFSET>,
            ChooseImageDefaults: ChooseImageDefaults::<Identity, OFFSET>,
            ChooseImageDefaultsForMediaType: ChooseImageDefaultsForMediaType::<Identity, OFFSET>,
            SetISO9660InterchangeLevel: SetISO9660InterchangeLevel::<Identity, OFFSET>,
            ISO9660InterchangeLevel: ISO9660InterchangeLevel::<Identity, OFFSET>,
            ISO9660InterchangeLevelsSupported: ISO9660InterchangeLevelsSupported::<Identity, OFFSET>,
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
            VolumeNameUDF: VolumeNameUDF::<Identity, OFFSET>,
            VolumeNameJoliet: VolumeNameJoliet::<Identity, OFFSET>,
            VolumeNameISO9660: VolumeNameISO9660::<Identity, OFFSET>,
            StageFiles: StageFiles::<Identity, OFFSET>,
            SetStageFiles: SetStageFiles::<Identity, OFFSET>,
            MultisessionInterfaces: MultisessionInterfaces::<Identity, OFFSET>,
            SetMultisessionInterfaces: SetMultisessionInterfaces::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSystemImage as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_imapi2", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IFileSystemImage {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IFileSystemImage2, IFileSystemImage2_Vtbl, 0xd7644b2c_1537_4767_b62f_f1387b02ddfd);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IFileSystemImage2 {
    type Target = IFileSystemImage;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IFileSystemImage2, windows_core::IUnknown, super::oaidl::IDispatch, IFileSystemImage);
#[cfg(feature = "Win32_oaidl")]
impl IFileSystemImage2 {
    pub unsafe fn BootImageOptionsArray(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BootImageOptionsArray)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBootImageOptionsArray(&self, newval: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBootImageOptionsArray)(windows_core::Interface::as_raw(self), newval) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImage2_Vtbl {
    pub base__: IFileSystemImage_Vtbl,
    pub BootImageOptionsArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub SetBootImageOptionsArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_imapi2", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IFileSystemImage2_Impl: IFileSystemImage_Impl {
    fn BootImageOptionsArray(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn SetBootImageOptionsArray(&self, newval: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_imapi2", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IFileSystemImage2_Vtbl {
    pub const fn new<Identity: IFileSystemImage2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BootImageOptionsArray<Identity: IFileSystemImage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage2_Impl::BootImageOptionsArray(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBootImageOptionsArray<Identity: IFileSystemImage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage2_Impl::SetBootImageOptionsArray(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        Self {
            base__: IFileSystemImage_Vtbl::new::<Identity, OFFSET>(),
            BootImageOptionsArray: BootImageOptionsArray::<Identity, OFFSET>,
            SetBootImageOptionsArray: SetBootImageOptionsArray::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSystemImage2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IFileSystemImage as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_imapi2", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IFileSystemImage2 {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IFileSystemImage3, IFileSystemImage3_Vtbl, 0x7cff842c_7e97_4807_8304_910dd8f7c051);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IFileSystemImage3 {
    type Target = IFileSystemImage2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IFileSystemImage3, windows_core::IUnknown, super::oaidl::IDispatch, IFileSystemImage, IFileSystemImage2);
#[cfg(feature = "Win32_oaidl")]
impl IFileSystemImage3 {
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn CreateRedundantUdfMetadataFiles(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRedundantUdfMetadataFiles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetCreateRedundantUdfMetadataFiles(&self, newval: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCreateRedundantUdfMetadataFiles)(windows_core::Interface::as_raw(self), newval) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn ProbeSpecificFileSystem(&self, filesystemtoprobe: FsiFileSystems) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProbeSpecificFileSystem)(windows_core::Interface::as_raw(self), filesystemtoprobe, &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImage3_Vtbl {
    pub base__: IFileSystemImage2_Vtbl,
    #[cfg(feature = "Win32_wtypes")]
    pub CreateRedundantUdfMetadataFiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    CreateRedundantUdfMetadataFiles: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetCreateRedundantUdfMetadataFiles: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetCreateRedundantUdfMetadataFiles: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub ProbeSpecificFileSystem: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    ProbeSpecificFileSystem: usize,
}
#[cfg(all(feature = "Win32_imapi2", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IFileSystemImage3_Impl: IFileSystemImage2_Impl {
    fn CreateRedundantUdfMetadataFiles(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetCreateRedundantUdfMetadataFiles(&self, newval: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ProbeSpecificFileSystem(&self, filesystemtoprobe: FsiFileSystems) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_imapi2", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IFileSystemImage3_Vtbl {
    pub const fn new<Identity: IFileSystemImage3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateRedundantUdfMetadataFiles<Identity: IFileSystemImage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImage3_Impl::CreateRedundantUdfMetadataFiles(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCreateRedundantUdfMetadataFiles<Identity: IFileSystemImage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileSystemImage3_Impl::SetCreateRedundantUdfMetadataFiles(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn ProbeSpecificFileSystem<Identity: IFileSystemImage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesystemtoprobe: FsiFileSystems, isappendable: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
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
            CreateRedundantUdfMetadataFiles: CreateRedundantUdfMetadataFiles::<Identity, OFFSET>,
            SetCreateRedundantUdfMetadataFiles: SetCreateRedundantUdfMetadataFiles::<Identity, OFFSET>,
            ProbeSpecificFileSystem: ProbeSpecificFileSystem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSystemImage3 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IFileSystemImage as windows_core::Interface>::IID || iid == &<IFileSystemImage2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_imapi2", feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IFileSystemImage3 {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IFileSystemImageResult, IFileSystemImageResult_Vtbl, 0x2c941fd8_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IFileSystemImageResult {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IFileSystemImageResult, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IFileSystemImageResult {
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn ImageStream(&self) -> windows_core::Result<super::objidlbase::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImageStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ProgressItems(&self) -> windows_core::Result<IProgressItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProgressItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn TotalBlocks(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TotalBlocks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BlockSize(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BlockSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DiscId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DiscId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImageResult_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(feature = "Win32_objidlbase")]
    pub ImageStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    ImageStream: usize,
    pub ProgressItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TotalBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub BlockSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DiscId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IFileSystemImageResult_Impl: super::oaidl::IDispatch_Impl {
    fn ImageStream(&self) -> windows_core::Result<super::objidlbase::IStream>;
    fn ProgressItems(&self) -> windows_core::Result<IProgressItems>;
    fn TotalBlocks(&self) -> windows_core::Result<i32>;
    fn BlockSize(&self) -> windows_core::Result<i32>;
    fn DiscId(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IFileSystemImageResult_Vtbl {
    pub const fn new<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ImageStream<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImageResult_Impl::ImageStream(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProgressItems<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImageResult_Impl::ProgressItems(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TotalBlocks<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImageResult_Impl::TotalBlocks(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BlockSize<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImageResult_Impl::BlockSize(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DiscId<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImageResult_Impl::DiscId(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ImageStream: ImageStream::<Identity, OFFSET>,
            ProgressItems: ProgressItems::<Identity, OFFSET>,
            TotalBlocks: TotalBlocks::<Identity, OFFSET>,
            BlockSize: BlockSize::<Identity, OFFSET>,
            DiscId: DiscId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSystemImageResult as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IFileSystemImageResult {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IFileSystemImageResult2, IFileSystemImageResult2_Vtbl, 0xb507ca29_2204_11dd_966a_001aa01bbc58);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IFileSystemImageResult2 {
    type Target = IFileSystemImageResult;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IFileSystemImageResult2, windows_core::IUnknown, super::oaidl::IDispatch, IFileSystemImageResult);
#[cfg(feature = "Win32_oaidl")]
impl IFileSystemImageResult2 {
    #[cfg(feature = "Win32_imapi2")]
    pub unsafe fn ModifiedBlocks(&self) -> windows_core::Result<super::imapi2::IBlockRangeList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModifiedBlocks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileSystemImageResult2_Vtbl {
    pub base__: IFileSystemImageResult_Vtbl,
    #[cfg(feature = "Win32_imapi2")]
    pub ModifiedBlocks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_imapi2"))]
    ModifiedBlocks: usize,
}
#[cfg(all(feature = "Win32_imapi2", feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IFileSystemImageResult2_Impl: IFileSystemImageResult_Impl {
    fn ModifiedBlocks(&self) -> windows_core::Result<super::imapi2::IBlockRangeList>;
}
#[cfg(all(feature = "Win32_imapi2", feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IFileSystemImageResult2_Vtbl {
    pub const fn new<Identity: IFileSystemImageResult2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ModifiedBlocks<Identity: IFileSystemImageResult2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileSystemImageResult2_Impl::ModifiedBlocks(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IFileSystemImageResult_Vtbl::new::<Identity, OFFSET>(), ModifiedBlocks: ModifiedBlocks::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSystemImageResult2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IFileSystemImageResult as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_imapi2", feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IFileSystemImageResult2 {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IFsiDirectoryItem, IFsiDirectoryItem_Vtbl, 0x2c941fdc_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IFsiDirectoryItem {
    type Target = IFsiItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IFsiDirectoryItem, windows_core::IUnknown, super::oaidl::IDispatch, IFsiItem);
#[cfg(feature = "Win32_oaidl")]
impl IFsiDirectoryItem {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::oaidl::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, path: &windows_core::BSTR) -> windows_core::Result<IFsiItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumFsiItems(&self) -> windows_core::Result<IEnumFsiItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumFsiItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddDirectory(&self, path: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddDirectory)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path)) }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn AddFile<P1>(&self, path: &windows_core::BSTR, filedata: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddFile)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(path), filedata.param().abi()) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn AddTree(&self, sourcedirectory: &windows_core::BSTR, includebasedirectory: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
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
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiDirectoryItem_Vtbl {
    pub base__: IFsiItem_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumFsiItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddDirectory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidlbase")]
    pub AddFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    AddFile: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub AddTree: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    AddTree: usize,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveTree: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IFsiDirectoryItem_Impl: IFsiItem_Impl {
    fn _NewEnum(&self) -> windows_core::Result<super::oaidl::IEnumVARIANT>;
    fn Item(&self, path: &windows_core::BSTR) -> windows_core::Result<IFsiItem>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn EnumFsiItems(&self) -> windows_core::Result<IEnumFsiItems>;
    fn AddDirectory(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddFile(&self, path: &windows_core::BSTR, filedata: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<()>;
    fn AddTree(&self, sourcedirectory: &windows_core::BSTR, includebasedirectory: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Add(&self, item: windows_core::Ref<IFsiItem>) -> windows_core::Result<()>;
    fn Remove(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoveTree(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IFsiDirectoryItem_Vtbl {
    pub const fn new<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiDirectoryItem_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiDirectoryItem_Impl::Item(this, core::mem::transmute(&path)) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiDirectoryItem_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumFsiItems<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiDirectoryItem_Impl::EnumFsiItems(this) {
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
        unsafe extern "system" fn AddTree<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcedirectory: *mut core::ffi::c_void, includebasedirectory: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
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
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            EnumFsiItems: EnumFsiItems::<Identity, OFFSET>,
            AddDirectory: AddDirectory::<Identity, OFFSET>,
            AddFile: AddFile::<Identity, OFFSET>,
            AddTree: AddTree::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            RemoveTree: RemoveTree::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiDirectoryItem as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IFsiItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IFsiDirectoryItem {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IFsiDirectoryItem2, IFsiDirectoryItem2_Vtbl, 0xf7fb4b9b_6d96_4d7b_9115_201b144811ef);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IFsiDirectoryItem2 {
    type Target = IFsiDirectoryItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IFsiDirectoryItem2, windows_core::IUnknown, super::oaidl::IDispatch, IFsiItem, IFsiDirectoryItem);
#[cfg(feature = "Win32_oaidl")]
impl IFsiDirectoryItem2 {
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn AddTreeWithNamedStreams(&self, sourcedirectory: &windows_core::BSTR, includebasedirectory: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddTreeWithNamedStreams)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(sourcedirectory), includebasedirectory) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiDirectoryItem2_Vtbl {
    pub base__: IFsiDirectoryItem_Vtbl,
    #[cfg(feature = "Win32_wtypes")]
    pub AddTreeWithNamedStreams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    AddTreeWithNamedStreams: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IFsiDirectoryItem2_Impl: IFsiDirectoryItem_Impl {
    fn AddTreeWithNamedStreams(&self, sourcedirectory: &windows_core::BSTR, includebasedirectory: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IFsiDirectoryItem2_Vtbl {
    pub const fn new<Identity: IFsiDirectoryItem2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddTreeWithNamedStreams<Identity: IFsiDirectoryItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcedirectory: *mut core::ffi::c_void, includebasedirectory: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiDirectoryItem2_Impl::AddTreeWithNamedStreams(this, core::mem::transmute(&sourcedirectory), core::mem::transmute_copy(&includebasedirectory)).into()
            }
        }
        Self { base__: IFsiDirectoryItem_Vtbl::new::<Identity, OFFSET>(), AddTreeWithNamedStreams: AddTreeWithNamedStreams::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiDirectoryItem2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IFsiItem as windows_core::Interface>::IID || iid == &<IFsiDirectoryItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IFsiDirectoryItem2 {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IFsiFileItem, IFsiFileItem_Vtbl, 0x2c941fdb_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IFsiFileItem {
    type Target = IFsiItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IFsiFileItem, windows_core::IUnknown, super::oaidl::IDispatch, IFsiItem);
#[cfg(feature = "Win32_oaidl")]
impl IFsiFileItem {
    pub unsafe fn DataSize(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DataSize32BitLow(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataSize32BitLow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DataSize32BitHigh(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataSize32BitHigh)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn Data(&self) -> windows_core::Result<super::objidlbase::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Data)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn SetData<P0>(&self, newval: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), newval.param().abi()) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiFileItem_Vtbl {
    pub base__: IFsiItem_Vtbl,
    pub DataSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub DataSize32BitLow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub DataSize32BitHigh: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidlbase")]
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    Data: usize,
    #[cfg(feature = "Win32_objidlbase")]
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    SetData: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IFsiFileItem_Impl: IFsiItem_Impl {
    fn DataSize(&self) -> windows_core::Result<i64>;
    fn DataSize32BitLow(&self) -> windows_core::Result<i32>;
    fn DataSize32BitHigh(&self) -> windows_core::Result<i32>;
    fn Data(&self) -> windows_core::Result<super::objidlbase::IStream>;
    fn SetData(&self, newval: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IFsiFileItem_Vtbl {
    pub const fn new<Identity: IFsiFileItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DataSize<Identity: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiFileItem_Impl::DataSize(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DataSize32BitLow<Identity: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiFileItem_Impl::DataSize32BitLow(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DataSize32BitHigh<Identity: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiFileItem_Impl::DataSize32BitHigh(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Data<Identity: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiFileItem_Impl::Data(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetData<Identity: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiFileItem_Impl::SetData(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        Self {
            base__: IFsiItem_Vtbl::new::<Identity, OFFSET>(),
            DataSize: DataSize::<Identity, OFFSET>,
            DataSize32BitLow: DataSize32BitLow::<Identity, OFFSET>,
            DataSize32BitHigh: DataSize32BitHigh::<Identity, OFFSET>,
            Data: Data::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiFileItem as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IFsiItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IFsiFileItem {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IFsiFileItem2, IFsiFileItem2_Vtbl, 0x199d0c19_11e1_40eb_8ec2_c8c822a07792);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IFsiFileItem2 {
    type Target = IFsiFileItem;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IFsiFileItem2, windows_core::IUnknown, super::oaidl::IDispatch, IFsiItem, IFsiFileItem);
#[cfg(feature = "Win32_oaidl")]
impl IFsiFileItem2 {
    pub unsafe fn FsiNamedStreams(&self) -> windows_core::Result<IFsiNamedStreams> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FsiNamedStreams)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn IsNamedStream(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsNamedStream)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn AddStream<P1>(&self, name: &windows_core::BSTR, streamdata: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddStream)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), streamdata.param().abi()) }
    }
    pub unsafe fn RemoveStream(&self, name: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveStream)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name)) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn IsRealTime(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRealTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetIsRealTime(&self, newval: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIsRealTime)(windows_core::Interface::as_raw(self), newval) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiFileItem2_Vtbl {
    pub base__: IFsiFileItem_Vtbl,
    pub FsiNamedStreams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub IsNamedStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    IsNamedStream: usize,
    #[cfg(feature = "Win32_objidlbase")]
    pub AddStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    AddStream: usize,
    pub RemoveStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub IsRealTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    IsRealTime: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetIsRealTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetIsRealTime: usize,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IFsiFileItem2_Impl: IFsiFileItem_Impl {
    fn FsiNamedStreams(&self) -> windows_core::Result<IFsiNamedStreams>;
    fn IsNamedStream(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn AddStream(&self, name: &windows_core::BSTR, streamdata: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<()>;
    fn RemoveStream(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsRealTime(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetIsRealTime(&self, newval: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IFsiFileItem2_Vtbl {
    pub const fn new<Identity: IFsiFileItem2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FsiNamedStreams<Identity: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streams: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiFileItem2_Impl::FsiNamedStreams(this) {
                    Ok(ok__) => {
                        streams.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsNamedStream<Identity: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiFileItem2_Impl::IsNamedStream(this) {
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
        unsafe extern "system" fn IsRealTime<Identity: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiFileItem2_Impl::IsRealTime(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsRealTime<Identity: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiFileItem2_Impl::SetIsRealTime(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        Self {
            base__: IFsiFileItem_Vtbl::new::<Identity, OFFSET>(),
            FsiNamedStreams: FsiNamedStreams::<Identity, OFFSET>,
            IsNamedStream: IsNamedStream::<Identity, OFFSET>,
            AddStream: AddStream::<Identity, OFFSET>,
            RemoveStream: RemoveStream::<Identity, OFFSET>,
            IsRealTime: IsRealTime::<Identity, OFFSET>,
            SetIsRealTime: SetIsRealTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiFileItem2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IFsiItem as windows_core::Interface>::IID || iid == &<IFsiFileItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IFsiFileItem2 {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IFsiItem, IFsiItem_Vtbl, 0x2c941fd9_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IFsiItem {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IFsiItem, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IFsiItem {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn FullPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FullPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CreationTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreationTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCreationTime(&self, newval: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCreationTime)(windows_core::Interface::as_raw(self), newval) }
    }
    pub unsafe fn LastAccessedTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastAccessedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLastAccessedTime(&self, newval: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLastAccessedTime)(windows_core::Interface::as_raw(self), newval) }
    }
    pub unsafe fn LastModifiedTime(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastModifiedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLastModifiedTime(&self, newval: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLastModifiedTime)(windows_core::Interface::as_raw(self), newval) }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn IsHidden(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsHidden)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SetIsHidden(&self, newval: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetIsHidden)(windows_core::Interface::as_raw(self), newval) }
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
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiItem_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FullPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreationTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetCreationTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub LastAccessedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetLastAccessedTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub LastModifiedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetLastModifiedTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub IsHidden: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    IsHidden: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SetIsHidden: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SetIsHidden: usize,
    pub FileSystemName: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FileSystemPath: unsafe extern "system" fn(*mut core::ffi::c_void, FsiFileSystems, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IFsiItem_Impl: super::oaidl::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn FullPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CreationTime(&self) -> windows_core::Result<f64>;
    fn SetCreationTime(&self, newval: f64) -> windows_core::Result<()>;
    fn LastAccessedTime(&self) -> windows_core::Result<f64>;
    fn SetLastAccessedTime(&self, newval: f64) -> windows_core::Result<()>;
    fn LastModifiedTime(&self) -> windows_core::Result<f64>;
    fn SetLastModifiedTime(&self, newval: f64) -> windows_core::Result<()>;
    fn IsHidden(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetIsHidden(&self, newval: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn FileSystemName(&self, filesystem: FsiFileSystems) -> windows_core::Result<windows_core::BSTR>;
    fn FileSystemPath(&self, filesystem: FsiFileSystems) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IFsiItem_Vtbl {
    pub const fn new<Identity: IFsiItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiItem_Impl::Name(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FullPath<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiItem_Impl::FullPath(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreationTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiItem_Impl::CreationTime(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCreationTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiItem_Impl::SetCreationTime(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn LastAccessedTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiItem_Impl::LastAccessedTime(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLastAccessedTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiItem_Impl::SetLastAccessedTime(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn LastModifiedTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiItem_Impl::LastModifiedTime(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLastModifiedTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiItem_Impl::SetLastModifiedTime(this, core::mem::transmute_copy(&newval)).into()
            }
        }
        unsafe extern "system" fn IsHidden<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiItem_Impl::IsHidden(this) {
                    Ok(ok__) => {
                        pval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsHidden<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFsiItem_Impl::SetIsHidden(this, core::mem::transmute_copy(&newval)).into()
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
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            FullPath: FullPath::<Identity, OFFSET>,
            CreationTime: CreationTime::<Identity, OFFSET>,
            SetCreationTime: SetCreationTime::<Identity, OFFSET>,
            LastAccessedTime: LastAccessedTime::<Identity, OFFSET>,
            SetLastAccessedTime: SetLastAccessedTime::<Identity, OFFSET>,
            LastModifiedTime: LastModifiedTime::<Identity, OFFSET>,
            SetLastModifiedTime: SetLastModifiedTime::<Identity, OFFSET>,
            IsHidden: IsHidden::<Identity, OFFSET>,
            SetIsHidden: SetIsHidden::<Identity, OFFSET>,
            FileSystemName: FileSystemName::<Identity, OFFSET>,
            FileSystemPath: FileSystemPath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiItem as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IFsiItem {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IFsiNamedStreams, IFsiNamedStreams_Vtbl, 0xed79ba56_5294_4250_8d46_f9aecee23459);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IFsiNamedStreams {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IFsiNamedStreams, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IFsiNamedStreams {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::oaidl::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<IFsiFileItem2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumNamedStreams(&self) -> windows_core::Result<IEnumFsiItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumNamedStreams)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IFsiNamedStreams_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EnumNamedStreams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IFsiNamedStreams_Impl: super::oaidl::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<super::oaidl::IEnumVARIANT>;
    fn Item(&self, index: i32) -> windows_core::Result<IFsiFileItem2>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn EnumNamedStreams(&self) -> windows_core::Result<IEnumFsiItems>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IFsiNamedStreams_Vtbl {
    pub const fn new<Identity: IFsiNamedStreams_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiNamedStreams_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiNamedStreams_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiNamedStreams_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumNamedStreams<Identity: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFsiNamedStreams_Impl::EnumNamedStreams(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            EnumNamedStreams: EnumNamedStreams::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiNamedStreams as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IFsiNamedStreams {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IIsoImageManager, IIsoImageManager_Vtbl, 0x6ca38be5_fbbb_4800_95a1_a438865eb0d4);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IIsoImageManager {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IIsoImageManager, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IIsoImageManager {
    pub unsafe fn Path(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn Stream(&self) -> windows_core::Result<super::objidlbase::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Stream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPath(&self, val: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(val)) }
    }
    #[cfg(feature = "Win32_objidlbase")]
    pub unsafe fn SetStream<P0>(&self, data: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStream)(windows_core::Interface::as_raw(self), data.param().abi()) }
    }
    pub unsafe fn Validate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Validate)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IIsoImageManager_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidlbase")]
    pub Stream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    Stream: usize,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_objidlbase")]
    pub SetStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_objidlbase"))]
    SetStream: usize,
    pub Validate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IIsoImageManager_Impl: super::oaidl::IDispatch_Impl {
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Stream(&self) -> windows_core::Result<super::objidlbase::IStream>;
    fn SetPath(&self, val: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetStream(&self, data: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<()>;
    fn Validate(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IIsoImageManager_Vtbl {
    pub const fn new<Identity: IIsoImageManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Path<Identity: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIsoImageManager_Impl::Path(this) {
                    Ok(ok__) => {
                        pval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Stream<Identity: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIsoImageManager_Impl::Stream(this) {
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
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Path: Path::<Identity, OFFSET>,
            Stream: Stream::<Identity, OFFSET>,
            SetPath: SetPath::<Identity, OFFSET>,
            SetStream: SetStream::<Identity, OFFSET>,
            Validate: Validate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIsoImageManager as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_objidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IIsoImageManager {}
pub const IMAPI2FS_BOOT_ENTRY_COUNT_MAX: u32 = 32;
pub const IMAPI2FS_FullVersion_STR: windows_core::PCSTR = windows_core::s!("1.0");
pub const IMAPI2FS_FullVersion_WSTR: windows_core::PCWSTR = windows_core::w!("1.0");
pub const IMAPI2FS_MajorVersion: u32 = 1;
pub const IMAPI2FS_MinorVersion: u32 = 0;
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IProgressItem, IProgressItem_Vtbl, 0x2c941fd5_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IProgressItem {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IProgressItem, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IProgressItem {
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn FirstBlock(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FirstBlock)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LastBlock(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastBlock)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn BlockCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BlockCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IProgressItem_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FirstBlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub LastBlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub BlockCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IProgressItem_Impl: super::oaidl::IDispatch_Impl {
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn FirstBlock(&self) -> windows_core::Result<u32>;
    fn LastBlock(&self) -> windows_core::Result<u32>;
    fn BlockCount(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IProgressItem_Vtbl {
    pub const fn new<Identity: IProgressItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Description<Identity: IProgressItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItem_Impl::Description(this) {
                    Ok(ok__) => {
                        desc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FirstBlock<Identity: IProgressItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, block: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItem_Impl::FirstBlock(this) {
                    Ok(ok__) => {
                        block.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastBlock<Identity: IProgressItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, block: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItem_Impl::LastBlock(this) {
                    Ok(ok__) => {
                        block.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BlockCount<Identity: IProgressItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blocks: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItem_Impl::BlockCount(this) {
                    Ok(ok__) => {
                        blocks.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Description: Description::<Identity, OFFSET>,
            FirstBlock: FirstBlock::<Identity, OFFSET>,
            LastBlock: LastBlock::<Identity, OFFSET>,
            BlockCount: BlockCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProgressItem as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IProgressItem {}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::define_interface!(IProgressItems, IProgressItems_Vtbl, 0x2c941fd7_975b_59be_a960_9a2a262853a5);
#[cfg(feature = "Win32_oaidl")]
impl core::ops::Deref for IProgressItems {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_oaidl")]
windows_core::imp::interface_hierarchy!(IProgressItems, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "Win32_oaidl")]
impl IProgressItems {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::oaidl::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<IProgressItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ProgressItemFromBlock(&self, block: u32) -> windows_core::Result<IProgressItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProgressItemFromBlock)(windows_core::Interface::as_raw(self), block, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ProgressItemFromDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<IProgressItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProgressItemFromDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(description), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumProgressItems(&self) -> windows_core::Result<IEnumProgressItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumProgressItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "Win32_oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IProgressItems_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ProgressItemFromBlock: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProgressItemFromDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumProgressItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IProgressItems_Impl: super::oaidl::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<super::oaidl::IEnumVARIANT>;
    fn Item(&self, index: i32) -> windows_core::Result<IProgressItem>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn ProgressItemFromBlock(&self, block: u32) -> windows_core::Result<IProgressItem>;
    fn ProgressItemFromDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<IProgressItem>;
    fn EnumProgressItems(&self) -> windows_core::Result<IEnumProgressItems>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IProgressItems_Vtbl {
    pub const fn new<Identity: IProgressItems_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItems_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItems_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        item.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItems_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProgressItemFromBlock<Identity: IProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, block: u32, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn EnumProgressItems<Identity: IProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProgressItems_Impl::EnumProgressItems(this) {
                    Ok(ok__) => {
                        newenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Item: Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            ProgressItemFromBlock: ProgressItemFromBlock::<Identity, OFFSET>,
            ProgressItemFromDescription: ProgressItemFromDescription::<Identity, OFFSET>,
            EnumProgressItems: EnumProgressItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProgressItems as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
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
